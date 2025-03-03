use gtk::glib;
use gtk::glib::translate::*;
use libc::{c_char, c_int};
use log::debug;
use std::cell::RefCell;
use std::env;
use std::process::{Command, Stdio};
use std::ptr;
use std::rc::Rc;
use zbus::{Connection, Message};

use crate::util::{PKmessage, PackageInfo};
use packagekit_rs::prelude::*;
use packagekit_rs::{
    pk_offline_get_prepared_ids, pk_offline_trigger, ClientPk, ProgressPk, ResultsPk,
    PK_OFFLINE_ACTION_REBOOT, PK_PROGRESS_TYPE_PERCENTAGE, PK_STATUS_ENUM_DOWNLOAD,
    PK_STATUS_ENUM_INSTALL, PK_STATUS_ENUM_REFRESH_CACHE, PK_STATUS_ENUM_REMOVE,
};

#[derive(Clone)]
pub struct PackagekitState {
    state: Rc<RefCell<bool>>,
}

impl PackagekitState {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(false)),
        }
    }

    pub fn set_busy(&self, s: bool) {
        *self.state.borrow_mut() = s;
    }

    pub fn busy(&self) -> bool {
        let state = self.state.borrow();
        if *state {
            true
        } else {
            false
        }
    }
}

pub fn get_updates(sender: async_channel::Sender<PKmessage>) {
    debug!("get updates start");
    let client = ClientPk::new();
    let sender1 = sender.clone();
    let sender2 = sender.clone();

    // Send update packages list
    {
        let closure = move |progress: &ProgressPk, progress_type: c_int| {
            if progress_type == PK_PROGRESS_TYPE_PERCENTAGE {
                sender1
                    .send_blocking(PKmessage::Progress((progress.percentage(), None)))
                    .expect("Couldn't send data to channel");
            }
        };

        let result: ResultsPk;
        match client.get_updates(Some(Box::new(closure)), None) {
            Ok(ret) => result = ret,
            Err(e) => {
                sender
                    .send_blocking(PKmessage::Error(e.to_string()))
                    .expect("Couldn't send data to channel");
                sender.close();
                return;
            }
        }

        let vecc = result.package_array();
        if vecc.len() == 0 {
            debug!("get updates success: 0 packages");
            sender
                .send_blocking(PKmessage::PackageListNew(vec![]))
                .expect("Couldn't send data to channel");
            sender.close();
            return;
        }

        let mut name_vec: Vec<PackageInfo> = vec![];
        for pkg in vecc {
            name_vec.push(PackageInfo {
                name: pkg.name(),
                version_new: pkg.version(),
                version_current: "".to_string(),
            });
        }
        debug!("get updates success");
        sender
            .send_blocking(PKmessage::PackageListNew(name_vec))
            .expect("Couldn't send data to channel");
    }

    // Send installed packages list
    {
        let closure = move |progress: &ProgressPk, progress_type: c_int| {
            if progress_type == PK_PROGRESS_TYPE_PERCENTAGE {
                sender2
                    .send_blocking(PKmessage::Progress((progress.percentage(), None)))
                    .expect("Couldn't send data to channel");
            }
        };

        let result: ResultsPk;
        match client.get_packages(Some(Box::new(closure)), None) {
            Ok(ret) => result = ret,
            Err(e) => {
                sender
                    .send_blocking(PKmessage::Error(e.to_string()))
                    .expect("Couldn't send data to channel");
                sender.close();
                return;
            }
        }

        let vecc = result.package_array();
        if vecc.len() == 0 {
            return;
        }
        let mut name_vec: Vec<PackageInfo> = vec![];
        for pkg in vecc {
            name_vec.push(PackageInfo {
                name: pkg.name(),
                version_new: "".to_string(),
                version_current: pkg.version(),
            });
        }
        debug!("get installed success");
        sender
            .send_blocking(PKmessage::PackageListInstalled(name_vec))
            .expect("Couldn't send data to channel");
        sender.close();
    }
}

pub fn download_updates(sender: async_channel::Sender<PKmessage>) {
    debug!("download updates start");
    let client = ClientPk::new();
    let sender1 = sender.clone();
    let closure = move |progress: &ProgressPk, progress_type: c_int| {
        if progress_type == PK_PROGRESS_TYPE_PERCENTAGE
            && progress.status() == PK_STATUS_ENUM_DOWNLOAD
        {
            sender1
                .send_blocking(PKmessage::Progress((
                    progress.percentage(),
                    Some(progress.get_item_package()),
                )))
                .expect("Couldn't send data to channel");
        }
    };

    let result: ResultsPk;
    match client.get_updates(None, None) {
        Ok(ret) => result = ret,
        Err(e) => {
            sender
                .send_blocking(PKmessage::Error(e.to_string()))
                .expect("Couldn't send data to channel");
            sender.close();
            return;
        }
    }

    let ids: *mut *mut c_char;
    match result.package_ids() {
        Some(ret) => ids = ret,
        None => {
            sender
                .send_blocking(PKmessage::Error("Update fail".to_string()))
                .expect("Couldn't send data to channel");
            sender.close();
            return;
        }
    }

    client.set_cache_age(60 * 60 * 24);
    match client.update_packages(ids, Some(Box::new(closure)), true) {
        Ok(_) => {
            debug!("download success");
            sender
                .send_blocking(PKmessage::DownloadFinish)
                .expect("Couldn't send data to channel");
            sender.close();
        }
        Err(e) => {
            sender
                .send_blocking(PKmessage::Error(e.to_string()))
                .expect("Couldn't send data to channel");
            sender.close();
            return;
        }
    }
}

pub fn updates(sender: async_channel::Sender<PKmessage>) {
    debug!("updates start");
    let client = ClientPk::new();
    let sender1 = sender.clone();
    let closure = move |progress: &ProgressPk, progress_type: c_int| {
        if progress_type == PK_PROGRESS_TYPE_PERCENTAGE
            && progress.status() == PK_STATUS_ENUM_INSTALL
        {
            sender1
                .send_blocking(PKmessage::Progress((
                    progress.percentage(),
                    Some(progress.get_item_package()),
                )))
                .expect("Couldn't send data to channel");
        }
    };

    let result: ResultsPk;
    match client.get_updates(None, None) {
        Ok(ret) => result = ret,
        Err(e) => {
            sender
                .send_blocking(PKmessage::Error(e.to_string()))
                .expect("Couldn't send data to channel");
            sender.close();
            return;
        }
    }

    let ids: *mut *mut c_char;
    match result.package_ids() {
        Some(ret) => ids = ret,
        None => {
            sender
                .send_blocking(PKmessage::Error("Update fail".to_string()))
                .expect("Couldn't send data to channel");
            sender.close();
            return;
        }
    }

    match client.update_packages(ids, Some(Box::new(closure)), false) {
        Ok(_) => {
            debug!("update success");
            sender
                .send_blocking(PKmessage::UpdateFinish)
                .expect("Couldn't send data to channel");
        }
        Err(e) => {
            sender
                .send_blocking(PKmessage::Error(e.to_string()))
                .expect("Couldn't send data to channel");
            sender.close();
            return;
        }
    }
}

/*
pub fn search_names(sender: glib::Sender<PKmessage>, text: glib::GString) {
    debug!("search start");
    let client = ClientPk::new();
    let sender1 = sender.clone();
    {
        let closure = move |progress: &ProgressPk, progress_type: c_int| {
            if progress_type == PK_PROGRESS_TYPE_PERCENTAGE
                && progress.status() == PK_STATUS_ENUM_REFRESH_CACHE
            {
                sender1
                    .send(PKmessage::Progress((progress.percentage(), None)))
                    .expect("Couldn't send data to channel");
            }
        };

        let names = [text.as_str()];
        let result: ResultsPk;
        match client.search_names(&names, Some(Box::new(closure))) {
            Ok(ret) => result = ret,
            Err(e) => {
                sender
                    .send(PKmessage::Error(e.to_string()))
                    .expect("Couldn't send data to channel");
                return;
            }
        }
        let vecc = result.package_array();
        if vecc.len() == 0 {
            debug!("get search success: 0 packages");
            sender
                .send(PKmessage::SearchListNew(vec![]))
                .expect("Couldn't send data to channel");
            return;
        }
        let mut name_vec: Vec<SearchInfo> = vec![];
        for pkg in vecc {
            name_vec.push(SearchInfo {
                name: pkg.name(),
                id: pkg.id(),
                summary: pkg.summary(),
                info: pkg.info(),
            });
        }
        debug!("get search success");
        sender
            .send(PKmessage::SearchListNew(name_vec))
            .expect("Couldn't send data to channel");
    }
}
*/

pub fn install_packages(sender: async_channel::Sender<PKmessage>, id: String) {
    debug!("install start");
    let client = ClientPk::new();
    let sender1 = sender.clone();
    let closure = move |progress: &ProgressPk, progress_type: c_int| {
        if progress_type == PK_PROGRESS_TYPE_PERCENTAGE
            && (progress.status() == PK_STATUS_ENUM_INSTALL
                || progress.status() == PK_STATUS_ENUM_DOWNLOAD
                || progress.status() == PK_STATUS_ENUM_REFRESH_CACHE)
        {
            sender1
                .send_blocking(PKmessage::Progress((progress.percentage(), None)))
                .expect("Couldn't send data to channel");
        }
    };

    match client.install_packages(&[id.as_str()], Some(Box::new(closure))) {
        Ok(_) => {
            debug!("install success");
            sender
                .send_blocking(PKmessage::InstallFinish)
                .expect("Couldn't send data to channel");
        }
        Err(e) => {
            sender
                .send_blocking(PKmessage::Error(e.to_string()))
                .expect("Couldn't send data to channel");
            sender.close();
            return;
        }
    }
}

pub fn remove_packages(sender: async_channel::Sender<PKmessage>, id: String) {
    debug!("remove start");
    let client = ClientPk::new();
    let sender1 = sender.clone();
    let closure = move |progress: &ProgressPk, progress_type: c_int| {
        if progress_type == PK_PROGRESS_TYPE_PERCENTAGE
            && progress.status() == PK_STATUS_ENUM_REMOVE
        {
            sender1
                .send_blocking(PKmessage::Progress((progress.percentage(), None)))
                .expect("Couldn't send data to channel");
        }
    };

    match client.remove_packages(&[id.as_str()], Some(Box::new(closure))) {
        Ok(_) => {
            debug!("remove success");
            sender
                .send_blocking(PKmessage::RemoveFinish)
                .expect("Couldn't send data to channel");
        }
        Err(e) => {
            sender
                .send_blocking(PKmessage::Error(e.to_string()))
                .expect("Couldn't send data to channel");
            sender.close();
            return;
        }
    }
}

pub fn offline_update_trigger() -> Result<bool, String> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = pk_offline_trigger(PK_OFFLINE_ACTION_REBOOT, ptr::null_mut(), &mut error);
        if error.is_null() {
            Ok(from_glib(ret))
        } else {
            Err("Offline update trigger failed".to_string())
        }
    }
}

pub fn offline_update_prepared() -> bool {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = pk_offline_get_prepared_ids(&mut error);

        if error.is_null() {
            let ids: *mut *mut c_char = ret;
            let v: Vec<glib::GString> = FromGlibPtrContainer::from_glib_none(ids);
            if v.len() > 0 {
                return true;
            }
            return false;
        } else {
            return false;
        }
    }
}

pub fn do_reboot() {
    let desktop = match env::var("DESKTOP_SESSION") {
        Ok(d) => d,
        Err(_) => "none".to_string(),
    };
    if desktop.contains("gnome") {
        do_reboot_gnome();
    } else if desktop.contains("plasma") {
        do_reboot_kde();
    } else {
        do_reboot_other();
    }
}

fn do_reboot_gnome() {
    let conn = match Connection::new_session() {
        Ok(conn) => conn,
        Err(_) => return,
    };
    let msg = Message::method(
        None,
        Some("org.gnome.SessionManager"),
        "/org/gnome/SessionManager",
        Some("org.gnome.SessionManager"),
        "Reboot",
        &(),
    )
    .unwrap();
    let _ret = match conn.send_message(msg) {
        Ok(ret) => ret,
        Err(_) => return,
    };
}

fn do_reboot_other() {
    let conn = match Connection::new_system() {
        Ok(conn) => conn,
        Err(_) => return,
    };
    let msg = Message::method(
        None,
        Some("org.freedesktop.login1"),
        "/org/freedesktop/login1",
        Some("org.freedesktop.login1.Manager"),
        "Reboot",
        &true,
    )
    .unwrap();
    let _ret = match conn.send_message(msg) {
        Ok(ret) => ret,
        Err(_) => return,
    };
}

fn do_reboot_kde() {
    //qdbus org.kde.ksmserver /KSMServer org.kde.KSMServerInterface.logout -1 1 -1
    let _status = Command::new("qdbus-qt5")
        .arg("org.kde.ksmserver")
        .arg("/KSMServer")
        .arg("org.kde.KSMServerInterface.logout")
        .arg("-1")
        .arg("1")
        .arg("-1")
        .stdout(Stdio::piped())
        .status()
        .expect("failed to execute rpm");
}
