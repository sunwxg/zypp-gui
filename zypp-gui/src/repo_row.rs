use crate::zypper::RepoInfo;
use adw::prelude::*;

#[derive(Clone)]
pub struct RepoRow {
    row: adw::PreferencesGroup,
    enable: adw::SwitchRow,
    refresh: adw::SwitchRow,
    priority: adw::SpinRow,
    delete: gtk::Button,
    url: adw::ActionRow,
}

impl RepoRow {
    pub fn new(info: RepoInfo) -> Self {
        let builder = gtk::Builder::from_resource("/zypp/gui/ui/repo_row.ui");
        let row: adw::PreferencesGroup = builder.object("repo_row").unwrap();
        //let name: gtk::Label = builder.object("name").unwrap();
        let enable: adw::SwitchRow = builder.object("enable_switch").unwrap();
        let refresh: adw::SwitchRow = builder.object("refresh_button").unwrap();
        let priority: adw::SpinRow = builder.object("priority_button").unwrap();
        let url: adw::ActionRow = builder.object("url").unwrap();
        let delete: gtk::Button = builder.object("delete_button").unwrap();

        row.set_title(info.name.as_str());
        enable.set_active(info.enable);
        refresh.set_active(info.refresh);
        priority.set_value(info.priority as f64);
        url.set_title(info.url.as_str());

        Self {
            row: row,
            enable: enable,
            refresh: refresh,
            priority: priority,
            delete: delete,
            url: url,
        }
    }

    pub fn row(&self) -> &adw::PreferencesGroup {
        &self.row
    }

    pub fn enable(&self) -> &adw::SwitchRow {
        &self.enable
    }

    pub fn refresh(&self) -> &adw::SwitchRow {
        &self.refresh
    }

    pub fn priority(&self) -> &adw::SpinRow {
        &self.priority
    }

    pub fn delete(&self) -> &gtk::Button {
        &self.delete
    }

    pub fn update(&self, info: RepoInfo) {
        self.row.set_title(info.name.as_str());
        self.enable.set_active(info.enable);
        self.refresh.set_active(info.refresh);
        self.priority.set_value(info.priority as f64);
        self.url.set_title(info.url.as_str());
    }
}
