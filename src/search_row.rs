use crate::util::SearchInfo;
use adw::prelude::*;
use gettextrs::*;
use gtk::prelude::*;

pub struct SearchRow {
    row: adw::ActionRow,
    button: gtk::Button,
}

impl SearchRow {
    pub fn new(info: SearchInfo) -> Self {
        let builder = gtk::Builder::from_resource("/zypp/gui/ui/search_row.ui");
        let row: adw::ActionRow = builder.object("row").unwrap();
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

    pub fn row(&self) -> &adw::ActionRow {
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
