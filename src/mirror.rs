extern crate serde;
extern crate serde_json;

use gtk::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::config;
use crate::mirror_row::MirrorRow;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data {
    asia: Vec<Site>,
    africa: Vec<Site>,
    europe: Vec<Site>,
    north_america: Vec<Site>,
    oceania: Vec<Site>,
    south_america: Vec<Site>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Site {
    pub country: String,
    pub name: String,
    pub http: String,
    pub rsync: String,
    pub repo: Vec<String>,
}

#[derive(Clone)]
pub struct MirrorSettings {
    data: Data,
    mirror_page: gtk::Box,
    main_window: libhandy::ApplicationWindow,
}

impl MirrorSettings {
    pub fn new(builder: &gtk::Builder) -> Self {
        let data = MirrorSettings::read_data();
        let mirror_page: gtk::Box = builder.object("mirror_page").unwrap();
        let main_window: libhandy::ApplicationWindow = builder.object("window").unwrap();

        let mirror = Self {
            data,
            mirror_page,
            main_window,
        };

        mirror.creat_row(builder);
        mirror
    }

    fn read_data() -> Data {
        let path = format!("{}/{}", config::PKGDATADIR, "mirror.json");
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return Data {
                    asia: vec![],
                    africa: vec![],
                    europe: vec![],
                    north_america: vec![],
                    oceania: vec![],
                    south_america: vec![],
                }
            }
        };
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).expect("read file fail");

        let data: Data = serde_json::from_str(&buffer).unwrap();
        data
    }

    fn creat_row(&self, builder: &gtk::Builder) {
        {
            let list_box: gtk::ListBox = builder.object("asia").unwrap();
            for site in self.data.asia.clone() {
                let row = MirrorRow::new(site, self.main_window.clone());
                list_box.add(&row.row().to_owned());
            }
        }
        {
            let list_box: gtk::ListBox = builder.object("africa").unwrap();
            for site in self.data.africa.clone() {
                let row = MirrorRow::new(site, self.main_window.clone());
                list_box.add(&row.row().to_owned());
            }
        }
        {
            let list_box: gtk::ListBox = builder.object("europe").unwrap();
            for site in self.data.europe.clone() {
                let row = MirrorRow::new(site, self.main_window.clone());
                list_box.add(&row.row().to_owned());
            }
        }
        {
            let list_box: gtk::ListBox = builder.object("north_america").unwrap();
            for site in self.data.north_america.clone() {
                let row = MirrorRow::new(site, self.main_window.clone());
                list_box.add(&row.row().to_owned());
            }
        }
        {
            let list_box: gtk::ListBox = builder.object("oceania").unwrap();
            for site in self.data.oceania.clone() {
                let row = MirrorRow::new(site, self.main_window.clone());
                list_box.add(&row.row().to_owned());
            }
        }
        {
            let list_box: gtk::ListBox = builder.object("south_america").unwrap();
            for site in self.data.south_america.clone() {
                let row = MirrorRow::new(site, self.main_window.clone());
                list_box.add(&row.row().to_owned());
            }
        }
    }
}
