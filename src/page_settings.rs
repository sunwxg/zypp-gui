//use gtk::gdk;
use gtk::gio;
use gtk::gio::prelude::*;
use gtk::gio::File;
use gtk::glib;
use gtk::prelude::*;
use libadwaita::NavigationDirection;
use log::debug;
use std::thread;

use crate::additional::AdditionalRepo;
use crate::mirror::MirrorSettings;
use crate::repo_row::RepoRow;
use crate::zypper::{RepoInfo, Settings, Zypper};

#[derive(Clone)]
pub struct PageSettings {
    pub widget: libadwaita::Leaflet,
    pub button_deck_back: gtk::Button,
    list_box: gtk::Box,
    main_window: libadwaita::ApplicationWindow,
}

impl PageSettings {
    pub fn new(main_builder: &gtk::Builder) -> Self {
        let builder = gtk::Builder::from_resource("/zypp/gui/ui/page_settings.ui");
        let widget: libadwaita::Leaflet = builder.object("page_settings").unwrap();
        let button_deck_back: gtk::Button = builder.object("button_deck_back").unwrap();
        let list_box: gtk::Box = builder.object("repo_box").unwrap();
        let repo_add_button: gtk::Button = builder.object("repo_add").unwrap();
        let top_right_box: gtk::Box = builder.object("top_right_box").unwrap();
        top_right_box.append(&repo_add_button);
        let main_window: libadwaita::ApplicationWindow = main_builder.object("window").unwrap();
        MirrorSettings::new(main_builder, &builder);
        AdditionalRepo::new(main_builder, &builder);

        let page_settings = Self {
            widget,
            button_deck_back,
            list_box,
            main_window,
        };
        page_settings.button_connect(&builder);
        page_settings.build_repo_list();
        page_settings.monitor_repo_dir();

        page_settings
    }

    fn build_repo_list(&self) {
        let repo_list = match Zypper::get_repos() {
            Some(list) => list,
            None => vec![],
        };
        let list_box = self.list_box.clone();
        for info in repo_list {
            let row = RepoRow::new(info.clone());
            list_box.prepend(&row.row().to_owned());
            self.row_button_connect(&row, info.clone());
        }
    }

    fn clear_repo_list(&self) {
        let mut child = match self.list_box.first_child() {
            Some(child) => child,
            None => return,
        };

        loop {
            let next_child = child.next_sibling();
            self.list_box.remove(&child);
            match next_child {
                Some(c) => {
                    child = c;
                }
                None => break,
            };
        }
    }

    fn row_button_connect(&self, row: &RepoRow, info: RepoInfo) {
        {
            let id = String::from(info.id.clone());
            row.enable().connect_activate(move |switch| {
                Zypper::change_repo(id.clone(), Settings::Enable(switch.is_active()));
            });
        }
        {
            let id = String::from(info.id.clone());
            row.refresh().connect_toggled(move |b| {
                Zypper::change_repo(id.clone(), Settings::Refresh(b.is_active()));
            });
        }
        {
            let id = String::from(info.id.clone());
            row.priority().connect_value_changed(move |b| {
                Zypper::change_repo(id.clone(), Settings::Priority(b.value() as i32));
            });
        }
        {
            let this = self.clone();
            let id = String::from(info.id.clone());
            row.delete().connect_clicked(move |_| {
                this.create_dialog(id.clone());
            });
        }
    }

    fn button_connect(&self, builder: &gtk::Builder) {
        {
            let button: gtk::Button = builder.object("button_leaflet_back").unwrap();
            let page_settings: libadwaita::Leaflet = builder.object("page_settings").unwrap();
            button.connect_clicked(move |_| {
                page_settings.navigate(NavigationDirection::Back);
            });
        }

        {
            let stack: gtk::Stack = builder.object("setting_stack").unwrap();
            let this: gtk::Stack = builder.object("setting_stack").unwrap();
            let repo_add_button: gtk::Button = builder.object("repo_add").unwrap();
            let top_right_box: gtk::Box = builder.object("top_right_box").unwrap();
            let page_settings: libadwaita::Leaflet = builder.object("page_settings").unwrap();
            stack.connect_local("notify::visible-child", true, move |_| {
                page_settings.navigate(NavigationDirection::Forward);
                if this.visible_child_name().unwrap() == "Repo List" {
                    top_right_box.append(&repo_add_button);
                } else {
                    let mut child = match top_right_box.first_child() {
                        Some(child) => child,
                        None => return None,
                    };
                    loop {
                        top_right_box.remove(&child);
                        if let Some(next_child) = child.next_sibling() {
                            child = next_child;
                        } else {
                            break;
                        }
                    }
                }
                None
            });
        }

        {
            let button: gtk::Button = builder.object("repo_add").unwrap();
            let this = self.clone();
            button.connect_clicked(move |_| {
                this.create_add_repo_window();
            });
        }
    }

    fn create_add_repo_window(&self) {
        let builder = gtk::Builder::from_resource("/zypp/gui/ui/repo_add.ui");
        let window: gtk::Window = builder.object("repo_add_window").unwrap();
        window.set_modal(true);
        //window.set_type_hint(gdk::WindowTypeHint::Dialog);
        window.set_transient_for(Some(&self.main_window));

        let cancel: gtk::Button = builder.object("add_cancel").unwrap();
        let w = window.clone();
        cancel.connect_clicked(move |_| {
            w.close();
        });

        let name: gtk::Entry = builder.object("repo_name").unwrap();
        let url: gtk::Entry = builder.object("repo_url").unwrap();
        let ok: gtk::Button = builder.object("add_ok").unwrap();
        let w = window.clone();
        ok.connect_clicked(move |_| {
            let name = name.text();
            let url = url.text();
            if name.len() == 0 || url.len() == 0 {
                return;
            }
            Zypper::add_repo(name.to_string(), url.to_string());
            w.close();
        });

        window.show();
    }

    fn create_dialog(&self, id: String) {
        let dialog = gtk::MessageDialog::new(
            Some(&self.main_window),
            gtk::DialogFlags::DESTROY_WITH_PARENT | gtk::DialogFlags::MODAL,
            gtk::MessageType::Error,
            gtk::ButtonsType::OkCancel,
            "Do you want to delete this repo?",
        );

        dialog.run_async(move |obj, answer| {
            obj.close();
            if answer == gtk::ResponseType::Ok {
                Zypper::delete_repo(id.to_string());
            }
        });
    }

    fn monitor_repo_dir(&self) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        thread::spawn(move || {
            let mainloop = glib::MainLoop::new(None, true);
            let path = std::path::Path::new("/etc/zypp/repos.d");
            let file = File::for_path(path);
            let monitor = file
                .monitor(
                    gio::FileMonitorFlags::SEND_MOVED,
                    Some(&gio::Cancellable::new()),
                )
                .unwrap();
            monitor.connect_changed(move |_, _, _, event| {
                debug!("/etc/zypp/repos.d folder is changed: {:?}", event);
                if event == gio::FileMonitorEvent::Created
                    || event == gio::FileMonitorEvent::Deleted
                    || event == gio::FileMonitorEvent::ChangesDoneHint
                {
                    tx.send("repo changed")
                        .expect("Couldn't send data to channel");
                }
            });
            mainloop.run();
        });
        let this = self.clone();
        rx.attach(None, move |_| {
            this.clear_repo_list();
            this.build_repo_list();
            glib::Continue(true)
        });
    }
}
