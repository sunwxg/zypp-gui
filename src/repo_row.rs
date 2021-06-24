use crate::zypper::RepoInfo;
use gtk::prelude::*;

pub struct RepoRow {
    row: gtk::Box,
    enable: gtk::Switch,
    cpg: gtk::CheckButton,
    refresh: gtk::CheckButton,
    priority: gtk::SpinButton,
}

impl RepoRow {
    pub fn new(info: RepoInfo) -> Self {
        let builder = gtk::Builder::from_resource("/org/openSUSE/software/ui/repo_row.ui");
        let row: gtk::Box = builder.get_object("repo_row").unwrap();
        let name: gtk::Label = builder.get_object("name").unwrap();
        let enable: gtk::Switch = builder.get_object("enable_switch").unwrap();
        let cpg: gtk::CheckButton = builder.get_object("cpg_button").unwrap();
        let refresh: gtk::CheckButton = builder.get_object("refresh_button").unwrap();
        let priority: gtk::SpinButton = builder.get_object("priority_button").unwrap();
        let url: gtk::Label = builder.get_object("url").unwrap();

        name.set_text(info.name.as_str());
        enable.set_state(info.enable);
        cpg.set_active(info.cpg);
        refresh.set_active(info.refresh);
        priority.set_value(info.priority as f64);
        url.set_text(info.url.as_str());

        Self {
            row: row,
            enable: enable,
            cpg: cpg,
            refresh: refresh,
            priority: priority,
        }
    }

    pub fn row(&self) -> &gtk::Box {
        &self.row
    }

    pub fn enable(&self) -> &gtk::Switch {
        &self.enable
    }

    pub fn cpg(&self) -> &gtk::CheckButton {
        &self.cpg
    }

    pub fn refresh(&self) -> &gtk::CheckButton {
        &self.refresh
    }

    pub fn priority(&self) -> &gtk::SpinButton {
        &self.priority
    }
}
