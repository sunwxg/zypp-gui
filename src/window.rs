use gtk::prelude::*;
use libhandy::prelude::*;
use log::debug;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::thread;

use crate::list_row::ListRow;
use crate::notification;
use crate::packagekit;
use crate::packagekit::{
    do_reboot, offline_update_prepared, offline_update_trigger, PackagekitState,
};
use crate::page_settings;
use crate::search;
use crate::util::{ButtonState, PKmessage, PackageInfo};

#[derive(Clone)]
pub struct Window {
    window: libhandy::ApplicationWindow,
    stack_box: gtk::Stack,
    stack_list: gtk::ScrolledWindow,
    progress_bar: gtk::ProgressBar,
    stack_label: libhandy::Clamp,
    progress: gtk::Box,
    progress_label: gtk::Label,
    list_box: gtk::ListBox,
    download_button: gtk::Button,
    trigger_button: gtk::Button,
    package_list: RefCell<Box<Vec<PackageInfo>>>,
    state: Rc<RefCell<ButtonState>>,
    builder: gtk::Builder,
    search: search::SearchPackage,
    notification: notification::Notification,
    packagekit_state: PackagekitState,
}

impl Window {
    pub fn new(packagekit_state: PackagekitState) -> Self {
        let builder = gtk::Builder::from_resource("/org/openSUSE/software/ui/window.ui");
        let win: libhandy::ApplicationWindow = builder.get_object("window").unwrap();
        let button: gtk::Button = builder.get_object("download_button").unwrap();
        button.set_label("Refresh");
        let trigger_button: gtk::Button = builder.get_object("offline_update_button").unwrap();

        let stack_box: gtk::Stack = builder.get_object("stack_box").unwrap();
        let stack_list = builder.get_object("stack_list").unwrap();
        let progress_bar: gtk::ProgressBar = builder.get_object("progress_bar").unwrap();
        let stack_label: libhandy::Clamp = builder.get_object("stack_label").unwrap();
        let progress: gtk::Box = builder.get_object("progress").unwrap();
        let progress_label: gtk::Label = builder.get_object("progress_label").unwrap();
        stack_box.set_visible_child(&stack_label);
        let state = Rc::new(RefCell::new(ButtonState::Refresh));

        let header_bar_stack: gtk::Stack = builder.get_object("header_bar_stack").unwrap();
        let header_bar: libhandy::HeaderBar = builder.get_object("empty_header_bar").unwrap();
        header_bar_stack.set_visible_child(&header_bar);

        let notification = notification::Notification::new(&builder);
        let search = search::SearchPackage::new(
            &builder,
            progress_bar.clone(),
            progress.clone(),
            progress_label.clone(),
            notification.clone(),
            packagekit_state.clone(),
        );
        let _page_settings = page_settings::PageSettings::new(&builder);

        let window = Self {
            window: win,
            stack_box: stack_box,
            stack_list: stack_list,
            progress_bar: progress_bar,
            stack_label: stack_label,
            list_box: builder.get_object("list_box").unwrap(),
            progress: progress,
            progress_label: progress_label,
            download_button: button,
            trigger_button: trigger_button,
            package_list: RefCell::new(Box::new(vec![])),
            state: state,
            builder: builder,
            search: search,
            notification: notification,
            packagekit_state: packagekit_state,
        };

        window.download_button_connect();
        window.button_connect();

        window
    }

    pub fn window(&self) -> &libhandy::ApplicationWindow {
        &self.window
    }

    fn download_button_connect(&self) {
        let builder = self.builder.clone();
        let button: gtk::Button = builder.get_object("download_button").unwrap();
        let trigger_button: gtk::Button = builder.get_object("offline_update_button").unwrap();
        let state = self.state.clone();
        let this = self.clone();
        button.connect_clicked(move |button| {
            if this.packagekit_state.busy() {
                return;
            }
            this.packagekit_state.set_state(true);
            debug!("get_updates");
            let s: Ref<ButtonState> = state.borrow();
            match *s {
                ButtonState::Refresh => {
                    drop(s);
                    state.replace(ButtonState::Refreshing);
                    button.set_label("Refreshing");
                    button.set_sensitive(false);
                    this.get_updates();
                }
                ButtonState::Download => {
                    drop(s);
                    state.replace(ButtonState::Downloading);
                    button.set_label("Downloading");
                    button.set_sensitive(false);
                    this.download_updates();
                }
                ButtonState::Update => {
                    drop(s);
                    state.replace(ButtonState::Updating);
                    button.set_label("Updating");
                    button.set_sensitive(false);
                    trigger_button.set_visible(false);
                    this.updates();
                }
                ButtonState::RestartUpdate => {
                    do_reboot();
                }
                _ => {}
            }
        });
    }

    fn button_connect(&self) {
        let builder = self.builder.clone();
        {
            let button: gtk::Button = builder.get_object("button_settings").unwrap();
            let deck: libhandy::Deck = builder.get_object("deck").unwrap();
            let page_settings: libhandy::Leaflet = builder.get_object("page_settings").unwrap();
            button.connect_clicked(move |_| {
                deck.set_visible_child(&page_settings);
            });
        }

        {
            let button: gtk::Button = builder.get_object("button_deck_back").unwrap();
            let deck: libhandy::Deck = builder.get_object("deck").unwrap();
            let page_update: gtk::Box = builder.get_object("page_update").unwrap();
            button.connect_clicked(move |_| {
                deck.set_visible_child(&page_update);
            });
        }

        {
            let button: gtk::ToggleButton = builder.get_object("search_button").unwrap();
            let button_stack: gtk::Stack = builder.get_object("button_stack").unwrap();
            let search_bar: libhandy::Clamp = builder.get_object("search_bar").unwrap();
            let update_button: gtk::Box = builder.get_object("update_button").unwrap();
            let stack_box: gtk::Stack = builder.get_object("stack_box").unwrap();
            let stack_list: gtk::ScrolledWindow = builder.get_object("stack_list").unwrap();
            let search_box: gtk::ScrolledWindow = builder.get_object("search_box").unwrap();
            let search_entry: gtk::SearchEntry = builder.get_object("search_entry").unwrap();
            button.connect_toggled(move |b| {
                if b.get_active() {
                    button_stack.set_visible_child(&search_bar);
                    stack_box.set_visible_child(&search_box);
                    search_entry.grab_focus_without_selecting();
                } else {
                    button_stack.set_visible_child(&update_button);
                    stack_box.set_visible_child(&stack_list);
                }
            });
        }

        {
            let button: gtk::Button = self.builder.get_object("offline_update_button").unwrap();
            //let this = self.clone();
            button.connect_clicked(move |b| {
                //b.set_visible(false);
                //this.set_state(ButtonState::RestartUpdate);
                match offline_update_trigger() {
                    //Ok(_) => this.set_state(ButtonState::RestartUpdate),
                    //Err(_) => this.set_state(ButtonState::Update),
                    Ok(_) => do_reboot(),
                    Err(_) => b.set_visible(false),
                }
            });
        }
    }

    fn check_offline_state(&self) {
        if offline_update_prepared() {
            self.trigger_button.set_visible(true);
            //match offline_update_trigger() {
            //Ok(ok) => self.set_state(ButtonState::RestartUpdate),
            //Err(error) => self.set_state(ButtonState::Update),
            //}
            //} else {
            //self.set_state(ButtonState::Update);
        }
    }

    fn set_state(&self, state: ButtonState) {
        match state.clone() {
            ButtonState::Refresh => {
                self.download_button.set_sensitive(true);
                self.clear_list();
                self.show_label();
                self.update_progress_text(None);
            }
            ButtonState::Download => {
                self.download_button.set_sensitive(true);
                self.show_package_list();
            }
            ButtonState::Update => {
                self.download_button.set_sensitive(true);
                self.show_package_list();
                self.check_offline_state();
                self.update_progress_text(None);
            }
            ButtonState::RestartUpdate => {
                self.download_button.set_sensitive(true);
                self.show_package_list();
            }
            _ => {}
        }
        self.download_button
            .set_label(&(format!("{}", state.clone())).to_string());
        self.state.replace(state);

        self.packagekit_state.set_state(false);
    }

    pub fn first_show(&self) {
        let deck: libhandy::Deck = self.builder.get_object("deck").unwrap();
        let page_update: gtk::Box = self.builder.get_object("page_update").unwrap();
        deck.set_visible_child(&page_update);
        let search_button: gtk::ToggleButton = self.builder.get_object("search_button").unwrap();
        search_button.set_active(false);
        self.set_state(ButtonState::Refresh);
    }

    fn show_package_list(&self) {
        self.stack_box.set_visible_child(&self.stack_list);
    }

    fn show_label(&self) {
        self.stack_box.set_visible_child(&self.stack_label);
    }

    fn update_progress(&self, percentage: i32) {
        self.stack_box.set_visible_child(&self.progress);
        self.progress_bar.set_fraction(percentage as f64 / 100.0);
    }

    fn update_progress_text(&self, text: Option<String>) {
        let s = if text.is_some() {
            text.unwrap()
        } else {
            String::new()
        };
        let v: Vec<&str> = s.split(';').collect();
        if v.len() < 2 {
            self.progress_label.set_text(String::from("").as_str());
            return;
        }
        let fmt = format!("<b>{}</b>  {}", v[0], v[1]);
        self.progress_label.set_markup(fmt.as_str());
    }

    fn update_list(&self, package_list: Vec<PackageInfo>) {
        self.packagekit_state.set_state(false);

        self.download_button.set_sensitive(true);
        self.package_list.replace(Box::new(package_list.clone()));

        self.clear_list();
        let list_box = &self.list_box;
        for info in package_list {
            let row = ListRow::new();
            row.set_name(info.name);
            let mut version = info.version_current;
            version.push_str(" → ");
            version.push_str(info.version_new.as_str());
            row.set_version(version);
            list_box.add(&row.row().to_owned());
        }
    }

    fn clear_list(&self) {
        let list_box = &self.list_box;
        let children = list_box.get_children();
        for child in children {
            list_box.remove(&child);
        }
    }

    fn show_notification(&self, text: String) {
        self.notification.set_label(text);
    }

    fn get_updates(&self) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            packagekit::get_updates(tx);
        });

        let this = self.clone();
        self.update_progress(0);
        let mut package_list: Box<Vec<PackageInfo>> = Box::new(vec![]);
        rx.attach(None, move |message| {
            match message {
                PKmessage::PackageListNew(list) => {
                    if list.len() == 0 {
                        this.set_state(ButtonState::Refresh);
                    } else {
                        package_list = Box::new(list.clone());
                    }
                }
                PKmessage::PackageListInstalled(list) => {
                    let list_slice = &list[..];
                    let package_list_slice = &mut package_list[..];
                    for mut p in package_list_slice {
                        for l in list_slice {
                            if p.name == l.name {
                                p.version_current = l.version_current.clone();
                            }
                        }
                    }
                    this.set_state(ButtonState::Download);
                    this.update_list(package_list.to_vec());
                }
                PKmessage::Progress((percentage, _)) => {
                    this.update_progress(percentage);
                }
                PKmessage::Error(text) => {
                    this.show_notification(text);
                    this.set_state(ButtonState::Refresh);
                }
                _ => {}
            }
            glib::Continue(true)
        });
    }

    fn download_updates(&self) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            packagekit::download_updates(tx);
        });

        let this = self.clone();
        this.update_progress(0);
        rx.attach(None, move |message| {
            match message {
                PKmessage::DownloadFinish => {
                    debug!("DownloadFinish");
                    this.set_state(ButtonState::Update);
                }
                PKmessage::Progress((percentage, id)) => {
                    this.update_progress(percentage);
                    this.update_progress_text(id);
                }
                PKmessage::Error(text) => {
                    this.show_notification(text);
                    this.set_state(ButtonState::Refresh);
                }
                _ => {}
            }
            glib::Continue(true)
        });
    }

    pub fn updates(&self) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            packagekit::updates(tx);
        });

        let this = self.clone();
        self.update_progress(0);
        rx.attach(None, move |message| {
            match message {
                PKmessage::UpdateFinish => {
                    debug!("UpdateFinish");
                    this.set_state(ButtonState::Refresh);
                }
                PKmessage::Progress((percentage, id)) => {
                    this.update_progress(percentage);
                    this.update_progress_text(id);
                }
                PKmessage::Error(text) => {
                    this.show_notification(text);
                    this.set_state(ButtonState::Refresh);
                }
                _ => {}
            }
            glib::Continue(true)
        });
    }
}
