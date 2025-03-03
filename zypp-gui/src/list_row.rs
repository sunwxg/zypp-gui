pub struct ListRow {
    row: gtk::Box,
    name: gtk::Label,
    version: gtk::Label,
}

impl ListRow {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_resource("/zypp/gui/ui/list_row.ui");
        let row: gtk::Box = builder.object("row").unwrap();
        let name: gtk::Label = builder.object("name").unwrap();
        let version: gtk::Label = builder.object("version").unwrap();

        Self {
            row: row,
            name: name,
            version: version,
        }
    }

    pub fn row(&self) -> &gtk::Box {
        &self.row
    }

    pub fn set_name(&self, name: String) {
        self.name.set_label(&name.to_string());
    }

    pub fn set_version(&self, version: String) {
        self.version.set_label(&version.to_string());
    }
}
