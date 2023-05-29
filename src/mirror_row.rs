use gtk::prelude::*;
use libadwaita::prelude::*;

use crate::mirror::Site;
use crate::mirror_window::MirrorWindow;

#[derive(Clone)]
pub struct MirrorRow {
    row: libadwaita::ActionRow,
    site: Site,
    main_window: libadwaita::ApplicationWindow,
}

impl MirrorRow {
    pub fn new(site: Site, main_window: libadwaita::ApplicationWindow) -> Self {
        let builder = gtk::Builder::from_resource("/zypp/gui/ui/mirror_row.ui");
        let row: libadwaita::ActionRow = builder.object("mirror_row").unwrap();
        let title = format!("{}  {}", site.country.clone(), site.name.clone());
        row.set_title(title.as_str());
        row.set_icon_name(Some(site.country.to_lowercase().as_str()));

        let mirror_row = Self {
            row,
            site,
            main_window,
        };

        mirror_row.row_button_connect();
        mirror_row
    }

    pub fn row(&self) -> &libadwaita::ActionRow {
        &self.row
    }

    fn row_button_connect(&self) {
        {
            let this = self.clone();
            self.row.connect_activated(move |_| {
                let window = MirrorWindow::new(this.site.clone());
                window.window().set_transient_for(Some(&this.main_window));
                window.window().show();
            });
        }
    }
}
