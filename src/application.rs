use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use log::{debug, info};

use crate::config;
use crate::packagekit::PackagekitState;
use crate::window::Window;

pub struct Application {
    application: gtk::Application,
    window: Window,
}

impl Application {
    pub fn new() -> Self {
        let application = gtk::Application::new(Some(config::APP_ID), Default::default()).unwrap();
        let packagekit_state = PackagekitState::new();
        let window = Window::new(packagekit_state);

        let application = Application {
            application: application,
            window: window,
        };

        application.add_actions();
        application.connect_signals();
        application
    }

    fn connect_signals(&self) {
        {
            let window = self.window.window();
            self.application
                .connect_startup(clone!(@weak window as win => move |application| {
                    info!("startup");
                    win.set_application(Some(application));
                }));
        }
        {
            let window = self.window.clone();
            self.application.connect_activate(move |_| {
                info!("activate");
                window.first_show();
                window.window().show_all();
            });
        }
        {
            let window = self.window.window();
            let flag = self.application.get_flags();
            window.connect_delete_event(move |window, _| {
                debug!("window delete event");
                if flag == gio::ApplicationFlags::IS_SERVICE {
                    window.hide();
                    Inhibit(true)
                } else {
                    Inhibit(false)
                }
            });
        }
    }

    fn add_actions(&self) {
        let window = self.window.window();
        let quit = gio::SimpleAction::new("quit", None);
        quit.connect_activate(glib::clone!(@weak window => move |_, _| {
            info!("Actin quit");
            window.hide();
        }));

        self.application.add_action(&quit);
    }

    pub fn run(&self) {
        info!("openSUSE Software ({})", config::APP_ID);
        info!("Version: {} ({})", config::VERSION, config::PROFILE);
        &self.application.run(&std::env::args().collect::<Vec<_>>());
    }
}
