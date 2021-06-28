mod application;
mod config;
mod list_row;
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

extern crate glib;
extern crate gtk;
use application::Application;
use config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};
use gettextrs::*;

fn main() {
    env_logger::init();

    // Prepare i18n
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name("openSUSE Software");
    glib::set_prgname(Some("openSUSE-software"));

    gtk::init().expect("Unable to start GTK3");
    libhandy::init();

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = Application::new();
    app.run();
}
