use gtk::prelude::*;

#[derive(Clone)]
pub struct Notification {
    notification_bar: gtk::Revealer,
    label: gtk::Label,
}

impl Notification {
    pub fn new(builder: &gtk::Builder) -> Self {
        let notification_bar: gtk::Revealer = builder.object("notification_bar").unwrap();
        let label: gtk::Label = builder.object("notification_label").unwrap();

        Self::signal_connect(&builder);

        Self {
            notification_bar,
            label,
        }
    }

    fn signal_connect(builder: &gtk::Builder) {
        let button: gtk::Button = builder.object("notification_button").unwrap();
        let notification_bar: gtk::Revealer = builder.object("notification_bar").unwrap();
        button.connect_clicked(move |_| {
            notification_bar.set_reveal_child(false);
        });
    }

    pub fn set_label(&self, text: String) {
        self.label.set_text(text.as_str());
        self.notification_bar.set_reveal_child(true);
    }
}
