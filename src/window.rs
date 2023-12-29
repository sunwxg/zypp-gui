use gettextrs::*;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use log::{debug, info};
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
    window: adw::ApplicationWindow,
    stack_box: gtk::Stack,
    page_settings: page_settings::PageSettings,
    stack_list: gtk::ScrolledWindow,
    progress_bar: gtk::ProgressBar,
    stack_label: adw::Clamp,
    progress: gtk::Box,
    progress_label: gtk::Label,
    list_box: gtk::ListBox,
    download_button: gtk::Button,
    trigger_button: gtk::Button,
    cancel_button: gtk::Button,
    package_list: RefCell<Box<Vec<PackageInfo>>>,
    state: Rc<RefCell<ButtonState>>,
    builder: gtk::Builder,
    search: search::SearchPackage,
    notification: notification::Notification,
    packagekit_state: PackagekitState,
    application: adw::Application,
    sender_connect_id: Rc<RefCell<Option<glib::SignalHandlerId>>>,
}

impl Window {
    pub fn new(packagekit_state: PackagekitState, application: adw::Application) -> Self {
        let builder = gtk::Builder::from_resource("/zypp/gui/ui/window.ui");
        let win: adw::ApplicationWindow = builder.object("window").unwrap();
        win.set_application(Some(&application));

        let button: gtk::Button = builder.object("download_button").unwrap();
        button.set_label(&gettext("Refresh"));
        let trigger_button: gtk::Button = builder.object("offline_update_button").unwrap();
        let cancel_button: gtk::Button = builder.object("cancel_button").unwrap();

        let stack_box: gtk::Stack = builder.object("stack_box").unwrap();
        let stack_list = builder.object("stack_list").unwrap();
        let progress_bar: gtk::ProgressBar = builder.object("progress_bar").unwrap();
        let stack_label: adw::Clamp = builder.object("stack_label").unwrap();
        let progress: gtk::Box = builder.object("progress").unwrap();
        let progress_label: gtk::Label = builder.object("progress_label").unwrap();
        stack_box.set_visible_child(&stack_label);
        let state = Rc::new(RefCell::new(ButtonState::Refresh));
        let sender_connect_id = Rc::new(RefCell::new(Some(
            cancel_button.connect_clicked(|_button| {}),
        )));

        let notification = notification::Notification::new(&builder);
        let search = search::SearchPackage::new(
            &builder,
            progress_bar.clone(),
            progress.clone(),
            progress_label.clone(),
            notification.clone(),
            packagekit_state.clone(),
        );

        let deck: adw::ViewStack = builder.object("viewstack").unwrap();
        let page_settings = page_settings::PageSettings::new(&builder);
        deck.add(&page_settings.widget);

        let window = Self {
            window: win,
            stack_box: stack_box,
            page_settings: page_settings,
            stack_list: stack_list,
            progress_bar: progress_bar,
            stack_label: stack_label,
            list_box: builder.object("list_box").unwrap(),
            progress: progress,
            progress_label: progress_label,
            download_button: button,
            trigger_button: trigger_button,
            cancel_button: cancel_button,
            package_list: RefCell::new(Box::new(vec![])),
            state: state,
            builder: builder,
            search: search,
            notification: notification,
            packagekit_state: packagekit_state,
            application,
            sender_connect_id,
        };

        window.add_actions();
        window.download_button_connect();
        window.button_connect();

        window
    }

    pub fn window(&self) -> &adw::ApplicationWindow {
        &self.window
    }

    fn add_actions(&self) {
        let quit = gio::SimpleAction::new("quit", None);
        let application = self.application.clone();
        let window = self.window.clone();
        quit.connect_activate(move |_, _| {
            info!("Actin quit");
            let flag = application.flags();
            if flag == gio::ApplicationFlags::IS_SERVICE {
                window.hide();
            } else {
                window.close();
            }
        });

        self.application.add_action(&quit);
    }

    fn download_button_connect(&self) {
        let button = self.download_button.clone();
        let trigger_button = self.trigger_button.clone();
        let state = self.state.clone();
        let this = self.clone();
        button.connect_clicked(move |button| {
            if this.packagekit_state.busy() {
                return;
            }
            this.packagekit_state.set_busy(true);
            debug!("get_updates");
            let s: Ref<ButtonState> = state.borrow();
            match *s {
                ButtonState::Refresh => {
                    drop(s);
                    state.replace(ButtonState::Refreshing);
                    button.set_label(&gettext("Refreshing"));
                    button.set_sensitive(false);
                    this.set_search_button_sensitive(false);
                    this.get_updates();
                }
                ButtonState::Download => {
                    drop(s);
                    state.replace(ButtonState::Downloading);
                    button.set_label(&gettext("Downloading"));
                    button.set_sensitive(false);
                    this.set_search_button_sensitive(false);
                    this.download_updates();
                }
                ButtonState::Update => {
                    drop(s);
                    state.replace(ButtonState::Updating);
                    button.set_label(&gettext("Updating"));
                    button.set_sensitive(false);
                    this.set_search_button_sensitive(false);
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
            let button: gtk::Button = builder.object("button_settings").unwrap();
            let deck: adw::ViewStack = builder.object("viewstack").unwrap();
            let page_settings: adw::NavigationSplitView = self.page_settings.widget.clone();
            button.connect_clicked(move |_| {
                deck.set_visible_child(&page_settings);
            });
        }

        {
            let button: gtk::Button = self.page_settings.button_deck_back.clone();
            let deck: adw::ViewStack = builder.object("viewstack").unwrap();
            let page_update: gtk::Box = builder.object("page_update").unwrap();
            button.connect_clicked(move |_| {
                deck.set_visible_child(&page_update);
            });
        }

        {
            let button: gtk::ToggleButton = builder.object("search_button").unwrap();
            let button_stack: gtk::Stack = builder.object("button_stack").unwrap();
            let search_bar: adw::Clamp = builder.object("search_bar").unwrap();
            let update_button: gtk::Box = builder.object("update_button").unwrap();
            let search_box: gtk::ScrolledWindow = builder.object("search_box").unwrap();
            let search_entry: gtk::Entry = builder.object("search_entry").unwrap();
            let this = self.clone();
            button.connect_toggled(move |b| {
                if b.is_active() {
                    button_stack.set_visible_child(&search_bar);
                    this.stack_box.set_visible_child(&search_box);
                    search_entry.grab_focus_without_selecting();
                } else {
                    button_stack.set_visible_child(&update_button);
                    let s: Ref<ButtonState> = this.state.borrow();
                    match *s {
                        ButtonState::Refresh => {
                            this.show_label();
                        }
                        ButtonState::Refreshing
                        | ButtonState::Downloading
                        | ButtonState::Updating => {
                            this.show_progress();
                        }
                        _ => {
                            this.show_package_list();
                        }
                    }
                }
            });
        }
        {
            self.trigger_button
                .connect_clicked(move |b| match offline_update_trigger() {
                    Ok(_) => do_reboot(),
                    Err(_) => b.set_visible(false),
                });
        }
    }

    fn disconnect_cancel_button(&self) {
        if let Some(id) = self.sender_connect_id.borrow_mut().take() {
            self.cancel_button.disconnect(id)
        }
    }

    fn cancel_button_connect(&self, sender: glib::Sender<PKmessage>) {
        self.disconnect_cancel_button();
        self.cancel_button.set_visible(true);
        let this = self.clone();
        let id = self.cancel_button.connect_clicked(move |_| {
            match sender.send(PKmessage::Error("The job was canceled".to_string())) {
                Err(_) => debug!("cancel button sender sending fail"),
                _ => {}
            }
            this.disconnect_cancel_button();
        });
        self.sender_connect_id.replace(Some(id));
    }

    fn cancel_to_refresh(&self) {
        self.disconnect_cancel_button();
        self.cancel_button.set_visible(true);
        let this = self.clone();
        let id = self.cancel_button.connect_clicked(move |_| {
            this.set_state(ButtonState::Refresh);
            this.disconnect_cancel_button();
        });
        self.sender_connect_id.replace(Some(id));
    }

    fn check_offline_state(&self) {
        if offline_update_prepared() {
            self.trigger_button.set_visible(true);
        }
    }

    fn set_state(&self, state: ButtonState) {
        match state.clone() {
            ButtonState::Refresh => {
                self.download_button.set_sensitive(true);
                self.trigger_button.set_visible(false);
                self.cancel_button.set_visible(false);
                self.set_search_button_sensitive(true);
                self.clear_list();
                self.show_label();
                self.update_progress_text(None);
            }
            ButtonState::Download => {
                self.download_button.set_sensitive(true);
                self.set_search_button_sensitive(true);
                self.show_package_list();
                self.cancel_to_refresh();
                self.search.update_package_meta();
            }
            ButtonState::Update => {
                self.download_button.set_sensitive(true);
                self.set_search_button_sensitive(true);
                self.show_package_list();
                self.cancel_to_refresh();
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

        self.packagekit_state.set_busy(false);
    }

    pub fn first_show(&self) {
        let state = self.state.borrow();
        match *state {
            ButtonState::Refreshing | ButtonState::Downloading | ButtonState::Updating => return,
            _ => {}
        }
        drop(state);

        let deck: adw::ViewStack = self.builder.object("viewstack").unwrap();
        let page_update: gtk::Box = self.builder.object("page_update").unwrap();
        deck.set_visible_child(&page_update);

        self.search.update_package_meta();
        let search_button: gtk::ToggleButton = self.builder.object("search_button").unwrap();
        search_button.set_active(false);

        self.set_state(ButtonState::Refresh);
    }

    fn set_search_button_sensitive(&self, state: bool) {
        let button: gtk::ToggleButton = self.builder.object("search_button").unwrap();
        button.set_sensitive(state);
    }

    fn show_package_list(&self) {
        self.stack_box.set_visible_child(&self.stack_list);
    }

    fn show_label(&self) {
        self.stack_box.set_visible_child(&self.stack_label);
    }

    fn show_progress(&self) {
        self.stack_box.set_visible_child(&self.progress);
    }

    fn update_progress(&self, percentage: i32) {
        self.show_progress();
        self.progress_bar.set_fraction(percentage as f64 / 100.0);
        self.progress_bar.show();
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
        self.progress_label.show();
    }

    fn update_list(&self, package_list: Vec<PackageInfo>) {
        self.packagekit_state.set_busy(false);

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
            list_box.append(&row.row().to_owned());
        }
    }

    fn clear_list(&self) {
        let list_box = &self.list_box;
        let mut child = match list_box.first_child() {
            Some(child) => child,
            None => return,
        };

        loop {
            let next_child = child.next_sibling();
            list_box.remove(&child);
            match next_child {
                Some(c) => {
                    child = c;
                }
                None => break,
            };
        }
    }

    fn show_notification(&self, text: String) {
        self.notification.set_label(text);
    }

    fn get_updates(&self) {
        let (tx, rx) = glib::MainContext::channel(glib::Priority::default());
        self.cancel_button_connect(tx.clone());

        thread::spawn(move || {
            packagekit::get_updates(tx);
        });

        let this = self.clone();
        self.update_progress(0);
        let mut package_list: Box<Vec<PackageInfo>> = Box::new(vec![]);
        rx.attach(None, move |message| {
            let mut is_continue = true;
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
                    for p in package_list_slice {
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
                    is_continue = false;
                }
                _ => {}
            }
            if is_continue {
                glib::ControlFlow::Continue
            } else {
                glib::ControlFlow::Break
            }
        });
    }

    fn download_updates(&self) {
        let (tx, rx) = glib::MainContext::channel(glib::Priority::default());
        self.cancel_button_connect(tx.clone());

        thread::spawn(move || {
            packagekit::download_updates(tx);
        });

        let this = self.clone();
        this.update_progress(0);
        rx.attach(None, move |message| {
            let mut is_continue = true;
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
                    is_continue = false;
                }
                _ => {}
            }
            if is_continue {
                glib::ControlFlow::Continue
            } else {
                glib::ControlFlow::Break
            }
        });
    }

    pub fn updates(&self) {
        let (tx, rx) = glib::MainContext::channel(glib::Priority::default());
        self.cancel_button_connect(tx.clone());

        thread::spawn(move || {
            packagekit::updates(tx);
        });

        let this = self.clone();
        self.update_progress(0);
        rx.attach(None, move |message| {
            let mut is_continue = true;
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
                    is_continue = false;
                }
                _ => {}
            }
            if is_continue {
                glib::ControlFlow::Continue
            } else {
                glib::ControlFlow::Break
            }
        });
    }
}
