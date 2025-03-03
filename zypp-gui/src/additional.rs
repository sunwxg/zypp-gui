use adw::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};

use crate::config;
use crate::zypper::Zypper;

enum SYSTEM {
    TW,
    LEAP,
}

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
pub struct AdditionalRepo {
    data: Data,
    additional_page: gtk::ListBox,
    main_window: adw::ApplicationWindow,
}

impl AdditionalRepo {
    pub fn new(main_builder: &gtk::Builder, builder: &gtk::Builder) -> Self {
        let data = AdditionalRepo::read_data();
        let additional_page: gtk::ListBox = builder.object("additional_page").unwrap();
        let main_window: adw::ApplicationWindow = main_builder.object("window").unwrap();

        let this = Self {
            data,
            additional_page,
            main_window,
        };

        this.creat_row();
        this
    }

    fn read_data() -> Data {
        let path = format!("{}/{}", config::PKGDATADIR, "additional.json");
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
            let builder = gtk::Builder::from_resource("/zypp/gui/ui/additional_row.ui");
            let row: adw::ExpanderRow = builder.object("additional_row").unwrap();
            let sub_row: gtk::Label = builder.object("repo_info").unwrap();
            let button: gtk::Button = builder.object("button_add").unwrap();
            row.set_title(&data.name);
            sub_row.set_markup(&data.info);

            let this = self.clone();
            button.connect_clicked(move |_| {
                let url = match this.check_system() {
                    SYSTEM::TW => {
                        let url = format!("{}openSUSE_Tumbleweed/", data.url.clone());
                        url
                    }
                    SYSTEM::LEAP => {
                        let url = format!("{}openSUSE_Leap_$releasever/", data.url.clone());
                        url
                    }
                };
                if this.check_url(url.clone()) {
                    this.create_dialog("The repo has been added".to_string());
                } else {
                    Zypper::add_repo(data.name.clone(), url);
                }
            });

            self.additional_page.append(&row.to_owned());
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
        let dialog = gtk::MessageDialog::new(
            Some(&self.main_window),
            gtk::DialogFlags::DESTROY_WITH_PARENT | gtk::DialogFlags::MODAL,
            gtk::MessageType::Error,
            gtk::ButtonsType::Cancel,
            &text,
        );

        dialog.run_async(move |obj, _answer| {
            obj.close();
        });
    }

    fn check_system(&self) -> SYSTEM {
        let status = Command::new("grep")
            .arg("Tumbleweed")
            .arg("/etc/os-release")
            .stdout(Stdio::piped())
            .status()
            .expect("failed to execute rpm");

        if status.success() {
            SYSTEM::TW
        } else {
            SYSTEM::LEAP
        }
    }
}
