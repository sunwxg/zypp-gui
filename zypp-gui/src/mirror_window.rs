use gtk::prelude::*;
use log::debug;
use std::collections::HashMap;
use std::thread;

use crate::mirror::Site;
use crate::zypper::Zypper;

#[derive(Clone)]
pub struct MirrorWindow {
    window: gtk::Window,
    site: Site,
    map: HashMap<String, String>,
    radio_buttons: Vec<gtk::ToggleButton>,
}

impl MirrorWindow {
    pub fn new(site: Site) -> Self {
        let builder = gtk::Builder::from_resource("/zypp/gui/ui/mirror_window.ui");
        let window: gtk::Window = builder.object("mirror_window").unwrap();
        window.set_modal(true);

        let country: gtk::Label = builder.object("country").unwrap();
        let site_label: gtk::Label = builder.object("site").unwrap();
        country.set_text(&site.country);
        site_label.set_text(&site.name);

        let mut this = Self {
            window: window,
            site: site.clone(),
            map: HashMap::new(),
            radio_buttons: vec![],
        };

        {
            let connect_box: gtk::Box = builder.object("connect_box").unwrap();
            let mut list: Vec<String> = vec![];
            if site.http.len() != 0 {
                list.push("http".to_string());
                this.map.insert("http".to_string(), site.http);
            };
            if site.rsync.len() != 0 {
                list.push("rsync".to_string());
                this.map.insert("rsync".to_string(), site.rsync);
            };
            if site.ftp.len() != 0 {
                list.push("ftp".to_string());
                this.map.insert("ftp".to_string(), site.ftp);
            };
            this.add_radiobutton(connect_box, list.clone());
        }

        {
            let distro_box: gtk::Box = builder.object("distro_box").unwrap();
            let mut list: Vec<String> = vec![];
            for i in &["tw", "leap"] {
                list.push(i.to_string());
                this.map.insert(i.to_string(), i.to_string());
            }
            this.add_radiobutton(distro_box, list.clone());
        }

        this.connect_signal(&builder);

        this
    }

    pub fn window(&self) -> &gtk::Window {
        &self.window
    }

    fn add_radiobutton(&mut self, button_box: gtk::Box, list: Vec<String>) {
        if list.len() == 0 {
            return;
        }

        let first_button: gtk::ToggleButton = gtk::ToggleButton::with_mnemonic(&list[0]);
        first_button.set_widget_name(&list[0]);
        button_box.append(&first_button.to_owned());
        self.radio_buttons.push(first_button.clone());

        for i in 1..list.len() {
            let button: gtk::ToggleButton = gtk::ToggleButton::with_mnemonic(&list[i]);
            button.set_widget_name(&list[i]);
            button_box.append(&button.to_owned());
            self.radio_buttons.push(button.clone());
        }
    }

    fn connect_signal(&self, builder: &gtk::Builder) {
        let add_button: gtk::Button = builder.object("add").unwrap();
        let this = self.clone();
        let radio_buttons = self.radio_buttons.clone();
        add_button.connect_clicked(move |_| {
            let mut _url = "".to_string();
            let mut _distro = "".to_string();
            for b in radio_buttons.clone() {
                if b.is_active() {
                    if b.widget_name().contains("http")
                        || b.widget_name().contains("ftp")
                        || b.widget_name().contains("rsync")
                    {
                        let name: String = b.widget_name().to_string();
                        _url = this.map.get(&name).unwrap().to_string();
                    } else {
                        let name: String = b.widget_name().to_string();
                        _distro = this.map.get(&name).unwrap().to_string();
                    }
                }
            }
            let list = this.get_repo_address_list(this.site.name.clone(), _url, _distro);
            debug!("These addresses will be added.\n{:?}", list);
            this.create_dialog(list);
        });
    }

    fn get_repo_address_list(
        &self,
        name: String,
        url_in: String,
        distro: String,
    ) -> Vec<(String, String)> {
        let url = match url_in.strip_suffix("/") {
            Some(url) => url,
            _ => url_in.as_str(),
        };
        let mut list = vec![];
        if distro == "tw" {
            //repo-debug.repo:      baseurl=http://download.opensuse.org/tumbleweed/repo/debug/
            //repo-src-oss.repo:    baseurl=http://download.opensuse.org/tumbleweed/repo/src-oss
            //repo-src-non-oss.repo:baseurl=http://download.opensuse.org/tumbleweed/repo/src-non-oss
            //repo-non-oss.repo:    baseurl=http://download.opensuse.org/tumbleweed/repo/non-oss/
            //repo-oss.repo:        baseurl=http://download.opensuse.org/tumbleweed/repo/oss/
            {
                let name = format!("{}-non-oss", name);
                let link = format!("{}/tumbleweed/repo/non-oss/", url);
                list.push((link, name));
            }
            {
                let name = format!("{}-oss", name);
                let link = format!("{}/tumbleweed/repo/oss/", url);
                list.push((link, name));
            }
            {
                let name = format!("{}-debug", name);
                let link = format!("{}/tumbleweed/repo/debug/", url);
                list.push((link, name));
            }
            {
                let name = format!("{}-src-oss", name);
                let link = format!("{}/tumbleweed/repo/src-oss/", url);
                list.push((link, name));
            }
            {
                let name = format!("{}-src-non-oss", name);
                let link = format!("{}/tumbleweed/repo/src-non-oss/", url);
                list.push((link, name));
            }
        } else if distro.contains("leap") {
            //repo-debug-non-oss.repo:baseurl=http://download.opensuse.org/debug/distribution/leap/$releasever/repo/non-oss/
            //repo-debug.repo:baseurl=http://download.opensuse.org/debug/distribution/leap/$releasever/repo/oss/
            //repo-debug-update-non-oss.repo:baseurl=http://download.opensuse.org/debug/update/leap/$releasever/non-oss/
            //repo-debug-update.repo:baseurl=http://download.opensuse.org/debug/update/leap/$releasever/oss/
            //repo-non-oss.repo:baseurl=http://download.opensuse.org/distribution/leap/$releasever/repo/non-oss/
            //repo-oss.repo:baseurl=http://download.opensuse.org/distribution/leap/$releasever/repo/oss/
            //repo-source-non-oss.repo:baseurl=http://download.opensuse.org/source/distribution/leap/$releasever/repo/non-oss/
            //repo-source.repo:baseurl=http://download.opensuse.org/source/distribution/leap/$releasever/repo/oss/
            //repo-update-non-oss.repo:baseurl=http://download.opensuse.org/update/leap/$releasever/non-oss/
            //repo-update.repo:baseurl=http://download.opensuse.org/update/leap/$releasever/oss/

            {
                let name = format!("{}-non-oss", name);
                let link = format!("{}/distribution/leap/$releasever/repo/non-oss/", url);
                list.push((link, name));
            }
            {
                let name = format!("{}-oss", name);
                let link = format!("{}/distribution/leap/$releasever/repo/oss/", url);
                list.push((link, name));
            }
            {
                let name = format!("{}-update-non-oss", name);
                let link = format!("{}/update/leap/$releasever/non-oss/", url);
                list.push((link, name));
            }
            {
                let name = format!("{}-update-oss", name);
                let link = format!("{}/update/leap/$releasever/oss/", url);
                list.push((link, name));
            }
        }

        list
    }

    fn create_dialog(&self, list: Vec<(String, String)>) {
        let mut text = format!("These repos will be added:\n\n");
        for i in list.clone() {
            text = format!("{}{}, {}\n", text, i.1, i.0);
        }

        let dialog = gtk::MessageDialog::new(
            Some(&self.window),
            gtk::DialogFlags::DESTROY_WITH_PARENT | gtk::DialogFlags::MODAL,
            gtk::MessageType::Error,
            gtk::ButtonsType::OkCancel,
            &text,
        );

        let window = self.window.clone();
        dialog.run_async(move |obj, _answer| {
            obj.close();
            if _answer == gtk::ResponseType::Ok {
                let l = list.clone();
                thread::spawn(move || {
                    for i in l {
                        Zypper::add_repo(i.1, i.0);
                    }
                });
                //&self.window.close();
                window.close();
            }
        });
    }
}
