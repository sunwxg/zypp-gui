mod additional;
mod config;
mod list_row;
mod mirror;
mod mirror_row;
mod mirror_window;
mod notification;
mod package_meta;
mod packagekit;
mod page_settings;
mod repo_row;
mod search;
mod search_row;
mod util;
mod window;
mod zypper;

extern crate gtk;
use config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};
use gettextrs::*;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use log::info;

use crate::packagekit::PackagekitState;
use crate::window::Window;

fn main() {
    env_logger::init();

    // Prepare i18n
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name("zypp gui");
    glib::set_prgname(Some("zypp-gui"));

    gtk::init().expect("Unable to start GTK4");
    adw::init().expect("Unable to init adw");

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    //setup_css();
    let application = adw::Application::new(Some(config::APP_ID), Default::default());
    application.connect_startup(|app| {
        build_ui(app);
    });

    info!("zypp gui ({})", config::APP_ID);
    info!("Version: {} ({})", config::VERSION, config::PROFILE);
    application.run();
}

fn build_ui(application: &adw::Application) {
    info!("startup");
    let packagekit_state = PackagekitState::new();
    let window = Window::new(packagekit_state, application.clone());
    //let window = adw::ApplicationWindow::new(application);

    application.connect_activate(move |_| {
        info!("activate");
        window.first_show();
        window.window().show();
    });
}

//fn setup_css() {
//let provider = gtk::CssProvider::new();
//provider.load_from_resource("/zypp/gui/style.css");
//if let Some(screen) = gdk::Screen::default() {
//gtk::StyleContext::add_provider_for_screen(
//&screen,
//&provider,
//gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
//);
//}
//}
