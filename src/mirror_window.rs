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
    radio_buttons: Vec<gtk::RadioButton>,
}

impl MirrorWindow {
    pub fn new(site: Site) -> Self {
        let builder = gtk::Builder::from_resource("/org/openSUSE/software/ui/mirror_window.ui");
        let window: gtk::Window = builder.get_object("mirror_window").unwrap();
        window.set_modal(true);

        let country: gtk::Label = builder.get_object("country").unwrap();
        let site_label: gtk::Label = builder.get_object("site").unwrap();
        country.set_text(&site.country);
        site_label.set_text(&site.name);

        let mut this = Self {
            window: window,
            site: site.clone(),
            map: HashMap::new(),
            radio_buttons: vec![],
        };

        {
            let connect_box: gtk::Box = builder.get_object("connect_box").unwrap();
            let mut list: Vec<String> = vec![];
            if site.http.len() != 0 {
                list.push("http".to_string());
                this.map.insert("http".to_string(), site.http);
            };
            if site.rsync.len() != 0 {
                list.push("rsync".to_string());
                this.map.insert("rsync".to_string(), site.rsync);
            };
            this.add_radiobutton(connect_box, list.clone());
        }

        {
            let distro_box: gtk::Box = builder.get_object("distro_box").unwrap();
            let mut list: Vec<String> = vec![];
            for i in site.repo.clone() {
                list.push(i.clone());
                this.map.insert(i.clone(), i);
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

        let first_button: gtk::RadioButton = gtk::RadioButton::with_mnemonic(&list[0]);
        first_button.set_widget_name(&list[0]);
        button_box.add(&first_button.to_owned());
        self.radio_buttons.push(first_button.clone());

        for i in 1..list.len() {
            let button: gtk::RadioButton =
                gtk::RadioButton::with_mnemonic_from_widget(&first_button, &list[i]);
            button.set_widget_name(&list[i]);
            button_box.add(&button.to_owned());
            self.radio_buttons.push(button.clone());
        }
    }

    fn connect_signal(&self, builder: &gtk::Builder) {
        let add_button: gtk::Button = builder.get_object("add").unwrap();
        let this = self.clone();
        let radio_buttons = self.radio_buttons.clone();
        add_button.connect_clicked(move |_| {
            let mut _url = "".to_string();
            let mut _distro = "".to_string();
            for b in radio_buttons.clone() {
                if b.get_active() {
                    if b.get_widget_name().contains("http") || b.get_widget_name().contains("rsync")
                    {
                        let name: String = b.get_widget_name().to_string();
                        _url = this.map.get(&name).unwrap().to_string();
                    } else {
                        let name: String = b.get_widget_name().to_string();
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
        url: String,
        distro: String,
    ) -> Vec<(String, String)> {
        let mut list = vec![];
        if distro == "tw" {
            //repo-debug.repo:baseurl=http://download.opensuse.org/debug/tumbleweed/repo/oss/
            //repo-source.repo:baseurl=http://download.opensuse.org/source/tumbleweed/repo/oss/
            //repo-update.repo:baseurl=http://download.opensuse.org/update/tumbleweed/
            //repo-non-oss.repo:baseurl=http://download.opensuse.org/tumbleweed/repo/non-oss/
            //repo-oss.repo:baseurl=http://download.opensuse.org/tumbleweed/repo/oss/
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
                let link = format!("{}/debug/tumbleweed/repo/oss/", url);
                list.push((link, name));
            }
            {
                let name = format!("{}-source", name);
                let link = format!("{}/source/tumbleweed/repo/oss/", url);
                list.push((link, name));
            }
            {
                let name = format!("{}-update", name);
                let link = format!("{}/update/tumbleweed/", url);
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
                let link = format!(
                    "{}/distribution/leap/{}/repo/non-oss/",
                    url,
                    distro.strip_prefix("leap.").unwrap()
                );
                list.push((link, name));
            }
            {
                let name = format!("{}-oss", name);
                let link = format!(
                    "{}/distribution/leap/{}/repo/oss/",
                    url,
                    distro.strip_prefix("leap.").unwrap()
                );
                list.push((link, name));
            }
            {
                let name = format!("{}-update-non-oss", name);
                let link = format!(
                    "{}/update/leap/{}/non-oss/",
                    url,
                    distro.strip_prefix("leap.").unwrap()
                );
                list.push((link, name));
            }
            {
                let name = format!("{}-update-oss", name);
                let link = format!(
                    "{}/update/leap/{}/oss/",
                    url,
                    distro.strip_prefix("leap.").unwrap()
                );
                list.push((link, name));
            }
        }

        list
    }

    fn create_dialog(&self, list: Vec<(String, String)>) {
        let mut text = format!("These repos will be added:\n");
        for i in list.clone() {
            text = format!("{}{}, {}\n", text, i.1, i.0);
        }
        let dialog = gtk::MessageDialogBuilder::new()
            .transient_for(&self.window)
            .modal(true)
            .buttons(gtk::ButtonsType::OkCancel)
            .text(&text)
            .build();

        let this = self.clone();
        dialog.connect_response(move |dialog, event| {
            let l = list.clone();
            if event == gtk::ResponseType::Ok {
                thread::spawn(move || {
                    for i in l {
                        Zypper::add_repo(i.1, i.0);
                    }
                });
                dialog.close();
                this.window.close()
            } else if event == gtk::ResponseType::Cancel {
                dialog.close()
            }
        });
        dialog.show_all();
    }
}
