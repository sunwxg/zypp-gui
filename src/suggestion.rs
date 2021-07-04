extern crate serde;
extern crate serde_json;

use gtk::prelude::*;
use libhandy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};

use crate::config;
use crate::zypper::Zypper;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data {
    list: Vec<Repo>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Repo {
    name: String,
    url: String,
    info: String,
}

#[derive(Clone)]
pub struct SuggestionRepo {
    data: Data,
    suggestion_page: gtk::ListBox,
    main_window: libhandy::ApplicationWindow,
}

impl SuggestionRepo {
    pub fn new(builder: &gtk::Builder) -> Self {
        let data = SuggestionRepo::read_data();
        let suggestion_page: gtk::ListBox = builder.object("suggestion_page").unwrap();
        let main_window: libhandy::ApplicationWindow = builder.object("window").unwrap();

        let this = Self {
            data,
            suggestion_page,
            main_window,
        };

        this.creat_row();
        this
    }

    fn read_data() -> Data {
        let path = format!("{}/{}", config::PKGDATADIR, "suggestion.json");
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Data { list: vec![] },
        };
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).expect("read file fail");
        let data: Data = match serde_json::from_str(&buffer) {
            Ok(data) => data,
            Err(_) => return Data { list: vec![] },
        };

        data
    }

    fn creat_row(&self) {
        for data in self.data.list.clone() {
            let builder =
                gtk::Builder::from_resource("/org/openSUSE/software/ui/suggestion_row.ui");
            let row: libhandy::ExpanderRow = builder.object("suggestion_row").unwrap();
            let sub_row: libhandy::ActionRow = builder.object("repo_info").unwrap();
            let button: gtk::Button = builder.object("button_add").unwrap();
            row.set_title(Some(&data.name));
            sub_row.set_title(Some(&data.info));

            let this = self.clone();
            button.connect_clicked(move |_| {
                if this.check_url(data.url.clone()) {
                    this.create_dialog("The repo has been installed".to_string());
                } else {
                    Zypper::add_repo(data.name.clone(), data.url.clone());
                }
            });

            self.suggestion_page.add(&row.to_owned());
        }
    }

    fn check_url(&self, url: String) -> bool {
        let status = Command::new("grep")
            .arg(url)
            .arg("-r")
            .arg("/etc/zypp/repos.d/")
            .stdout(Stdio::piped())
            .status()
            .expect("failed to execute grep");

        if status.success() {
            true
        } else {
            false
        }
    }

    fn create_dialog(&self, text: String) {
        let dialog = gtk::MessageDialogBuilder::new()
            .transient_for(&self.main_window)
            .modal(true)
            .buttons(gtk::ButtonsType::Cancel)
            .text(&text)
            .build();

        dialog.connect_response(move |dialog, event| {
            if event == gtk::ResponseType::Cancel {
                dialog.close()
            }
        });
        dialog.show_all();
    }
}
