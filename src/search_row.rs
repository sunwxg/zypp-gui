use crate::util::SearchInfo;
use gettextrs::*;
use gtk::prelude::*;
use libadwaita::prelude::*;

pub struct SearchRow {
    row: libadwaita::ActionRow,
    button: gtk::Button,
}

impl SearchRow {
    pub fn new(info: SearchInfo) -> Self {
        let builder = gtk::Builder::from_resource("/zypp/gui/ui/search_row.ui");
        let row: libadwaita::ActionRow = builder.object("row").unwrap();
        let button: gtk::Button = builder.object("operation_button").unwrap();
        if info.info == "installed" {
            button.set_label(&gettext("Remove"));
        } else {
            button.set_label(&gettext("Install"));
        }

        Self {
            row: row,
            button: button,
        }
    }

    pub fn row(&self) -> &libadwaita::ActionRow {
        &self.row
    }

    pub fn button(&self) -> &gtk::Button {
        &self.button
    }

    pub fn set_title(&self, name: String) {
        self.row.set_title(&name.to_string());
    }

    pub fn set_subtitle(&self, subtitle: String) {
        self.row.set_subtitle(&subtitle.to_string());
    }
}
