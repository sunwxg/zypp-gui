use crate::zypper::RepoInfo;
use gtk::prelude::*;

pub struct RepoRow {
    row: gtk::Box,
    enable: gtk::Switch,
    refresh: gtk::CheckButton,
    priority: gtk::SpinButton,
    delete: gtk::Button,
}

impl RepoRow {
    pub fn new(info: RepoInfo) -> Self {
        let builder = gtk::Builder::from_resource("/zypp/gui/ui/repo_row.ui");
        let row: gtk::Box = builder.object("repo_row").unwrap();
        let name: gtk::Label = builder.object("name").unwrap();
        let enable: gtk::Switch = builder.object("enable_switch").unwrap();
        let refresh: gtk::CheckButton = builder.object("refresh_button").unwrap();
        let priority: gtk::SpinButton = builder.object("priority_button").unwrap();
        let url: gtk::Label = builder.object("url").unwrap();
        let delete: gtk::Button = builder.object("delete_button").unwrap();

        name.set_text(info.name.as_str());
        enable.set_state(info.enable);
        refresh.set_active(info.refresh);
        priority.set_value(info.priority as f64);
        url.set_text(info.url.as_str());

        Self {
            row: row,
            enable: enable,
            refresh: refresh,
            priority: priority,
            delete: delete,
        }
    }

    pub fn row(&self) -> &gtk::Box {
        &self.row
    }

    pub fn enable(&self) -> &gtk::Switch {
        &self.enable
    }

    pub fn refresh(&self) -> &gtk::CheckButton {
        &self.refresh
    }

    pub fn priority(&self) -> &gtk::SpinButton {
        &self.priority
    }

    pub fn delete(&self) -> &gtk::Button {
        &self.delete
    }
}
