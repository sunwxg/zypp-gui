use gtk::prelude::*;
use libhandy::prelude::*;

use crate::mirror::Site;
use crate::mirror_window::MirrorWindow;

#[derive(Clone)]
pub struct MirrorRow {
    row: libhandy::ActionRow,
    site: Site,
    main_window: libhandy::ApplicationWindow,
}

impl MirrorRow {
    pub fn new(site: Site, main_window: libhandy::ApplicationWindow) -> Self {
        let builder = gtk::Builder::from_resource("/openSUSE/software/ui/mirror_row.ui");
        let row: libhandy::ActionRow = builder.object("mirror_row").unwrap();
        let title = format!("{}  {}", site.country.clone(), site.name.clone());
        row.set_title(Some(title.as_str()));

        let mirror_row = Self {
            row,
            site,
            main_window,
        };

        mirror_row.row_button_connect();
        mirror_row
    }

    pub fn row(&self) -> &libhandy::ActionRow {
        &self.row
    }

    fn row_button_connect(&self) {
        {
            let this = self.clone();
            self.row.connect_activated(move |_| {
                let window = MirrorWindow::new(this.site.clone());
                window.window().set_transient_for(Some(&this.main_window));
                window.window().show_all();
            });
        }
    }
}
