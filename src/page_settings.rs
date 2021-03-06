use gtk::gdk;
use gtk::gio;
use gtk::gio::prelude::*;
use gtk::gio::File;
use gtk::glib;
use gtk::prelude::*;
use libhandy::prelude::*;
use libhandy::NavigationDirection;
use log::debug;
use std::thread;

use crate::additional::AdditionalRepo;
use crate::mirror::MirrorSettings;
use crate::repo_row::RepoRow;
use crate::zypper::{RepoInfo, Settings, Zypper};

#[derive(Clone)]
pub struct PageSettings {
    list_box: gtk::Box,
    main_window: libhandy::ApplicationWindow,
}

impl PageSettings {
    pub fn new(builder: &gtk::Builder) -> Self {
        let list_box: gtk::Box = builder.object("repo_box").unwrap();
        let repo_add_button: gtk::Button = builder.object("repo_add").unwrap();
        let top_right_box: gtk::Box = builder.object("top_right_box").unwrap();
        top_right_box.add(&repo_add_button);
        let main_window: libhandy::ApplicationWindow = builder.object("window").unwrap();
        MirrorSettings::new(builder);
        AdditionalRepo::new(builder);

        let page_settings = Self {
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
            list_box.pack_start(&row.row().to_owned(), true, true, 0);
            self.row_button_connect(&row, info.clone());
        }
    }

    fn clear_repo_list(&self) {
        let children = self.list_box.children();
        for child in children {
            self.list_box.remove(&child);
        }
    }

    fn row_button_connect(&self, row: &RepoRow, info: RepoInfo) {
        {
            let id = String::from(info.id.clone());
            row.enable().connect_changed_active(move |switch| {
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
            let page_settings: libhandy::Leaflet = builder.object("page_settings").unwrap();
            button.connect_clicked(move |_| {
                page_settings.navigate(NavigationDirection::Back);
            });
        }

        {
            let stack: gtk::Stack = builder.object("setting_stack").unwrap();
            let this: gtk::Stack = builder.object("setting_stack").unwrap();
            let repo_add_button: gtk::Button = builder.object("repo_add").unwrap();
            let top_right_box: gtk::Box = builder.object("top_right_box").unwrap();
            let page_settings: libhandy::Leaflet = builder.object("page_settings").unwrap();
            stack
                .connect_local("notify::visible-child", true, move |_| {
                    page_settings.navigate(NavigationDirection::Forward);
                    if this.visible_child_name().unwrap() == "Repo List" {
                        top_right_box.add(&repo_add_button);
                    } else {
                        for w in top_right_box.children() {
                            top_right_box.remove(&w);
                        }
                    }
                    None
                })
                .expect("connecting to visible-child failed");
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
        window.set_type_hint(gdk::WindowTypeHint::Dialog);
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
        let dialog = gtk::MessageDialogBuilder::new()
            .transient_for(&self.main_window)
            .modal(true)
            .buttons(gtk::ButtonsType::OkCancel)
            .text("Do you want to delete this repo?")
            .build();

        dialog.connect_response(move |dialog, event| {
            if event == gtk::ResponseType::Ok {
                Zypper::delete_repo(id.to_string());
                dialog.close()
            } else if event == gtk::ResponseType::Cancel {
                dialog.close()
            }
        });
        dialog.show_all();
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
