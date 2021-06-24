use log::debug;

thread_local!(pub static PACKAGEKIT: RefCell<Option<Packagekit>> = RefCell::new(None));

pub struct Packagekit {
    window: Option<Rc<Window>>,
}

impl Packagekit {
    pub fn new() -> Self  {
        Self {
            window: None,
        }
    }

    pub fn set_window(&mut self, window: Rc<Window>) {
        self.window = Some(Rc::clone(&window));
    }

    pub fn get_updates(&self) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            packagekit::get_updates(tx);
        });

        let window = Rc::clone(&self.window.as_ref().unwrap());
        window.show_progress();
        let mut package_list: Box<Vec<PackageInfo>> = Box::new(vec![]);
        rx.attach(None, move |message| {
            match message {
                PKmessage::PackageListNew(list) => {
                    if list.len() == 0 {
                        window.set_state(ButtonState::Refresh);
                    } else {
                        package_list = Box::new(list.clone());
                    }
                },
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
                    window.set_state(ButtonState::Download);
                    window.update_list(package_list.to_vec());
                },
                PKmessage::Progress((percentage, _)) => {
                    window.update_progress(percentage);
                },
                PKmessage::Error(text) => {
                    window.show_notification(text);
                    window.set_state(ButtonState::Refresh);
                }
                _ => {},
            }
            glib::Continue(true)
        });
    }

    pub fn download_updates(&self) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            packagekit::download_updates(tx);
        });

        let window = Rc::clone(&self.window.as_ref().unwrap());
        window.show_progress();
        rx.attach(None, move |message| {
            match message {
                PKmessage::DownloadFinish => {
                    debug!("DownloadFinish");
                    window.set_state(ButtonState::Update);
                },
                PKmessage::Progress((percentage, id)) => {
                    window.update_progress(percentage);
                    window.update_progress_text(id);
                },
                PKmessage::Error(text) => {
                    window.show_notification(text);
                    window.set_state(ButtonState::Refresh);
                }
                _ => {},
            }
            glib::Continue(true)
        });
    }

    pub fn updates(&self) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            packagekit::updates(tx);
        });

        let window = Rc::clone(&(self.window.as_ref().unwrap()));
        window.show_progress();
        rx.attach(None, move |message| {
            match message {
                PKmessage::UpdateFinish => {
                    debug!("UpdateFinish");
                    window.set_state(ButtonState::Refresh);
                },
                PKmessage::Progress((percentage, id)) => {
                    window.update_progress(percentage);
                    window.update_progress_text(id);
                },
                PKmessage::Error(text) => {
                    window.show_notification(text);
                    window.set_state(ButtonState::Refresh);
                }
                _ => {},
            }
            glib::Continue(true)
        });
    }

    pub fn search_names(&self, text: glib::GString) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            packagekit::search_names(tx, text);
        });

        let window = Rc::clone(&(self.window.as_ref().unwrap()));
        window.show_progress();
        rx.attach(None, move |message| {
            match message {
                PKmessage::SearchListNew(list) => {
                    debug!("SearchFinish len={}", list.len());
                    window.update_search_list(list);
                },
                PKmessage::Progress((percentage, id)) => {
                    window.update_progress(percentage);
                    window.update_progress_text(id);
                },
                PKmessage::Error(text) => {
                    window.show_notification(text);
                }
                _ => {},
            }
            glib::Continue(true)
        });
    }

    pub fn install_packages(&self, id: String) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            packagekit::install_packages(tx, id);
        });

        let window = Rc::clone(&(self.window.as_ref().unwrap()));
        window.show_progress();
        rx.attach(None, move |message| {
            match message {
                PKmessage::InstallFinish => {
                    debug!("InstallFinish");
                    window.update_search_list(vec![]);
                },
                PKmessage::Progress((percentage, id)) => {
                    window.update_progress(percentage);
                    window.update_progress_text(id);
                },
                PKmessage::Error(text) => {
                    window.show_notification(text);
                    window.update_search_list(vec![]);
                }
                _ => {},
            }
            glib::Continue(true)
        });
    }

    pub fn remove_packages(&self, id: String) {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            packagekit::remove_packages(tx, id);
        });

        let window = Rc::clone(&(self.window.as_ref().unwrap()));
        window.show_progress();
        rx.attach(None, move |message| {
            match message {
                PKmessage::RemoveFinish => {
                    debug!("RemoveFinish");
                    window.update_search_list(vec![]);
                },
                PKmessage::Progress((percentage, id)) => {
                    window.update_progress(percentage);
                    window.update_progress_text(id);
                },
                PKmessage::Error(text) => {
                    window.show_notification(text);
                    window.update_search_list(vec![]);
                }
                _ => {},
            }
            glib::Continue(true)
        });
    }
}
