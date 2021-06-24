use gtk::prelude::*;
use libhandy::prelude::*;
use libhandy::NavigationDirection;

use crate::repo_row::RepoRow;
use crate::zypper::{RepoInfo, Settings, Zypper};

#[derive(Clone)]
pub struct PageSettings {
    list_box: gtk::Box,
}

impl PageSettings {
    pub fn new(builder: &gtk::Builder) -> Self {
        let list_box: gtk::Box = builder.get_object("repo_box").unwrap();

        Self::button_connect(&builder);
        Self::build_repo_list(&list_box);

        Self { list_box }
    }

    fn build_repo_list(list_box: &gtk::Box) {
        let repo_list = match Zypper::get_repos() {
            Some(list) => list,
            None => vec![],
        };
        for info in repo_list {
            let row = RepoRow::new(info.clone());
            list_box.pack_start(&row.row().to_owned(), true, true, 0);
            PageSettings::row_button_connect(&row, info.clone(), list_box);
        }
    }

    fn clear_repo_list(list_box: &gtk::Box) {
        let children = list_box.get_children();
        for child in children {
            list_box.remove(&child);
        }
    }

    fn row_button_connect(row: &RepoRow, info: RepoInfo, list_box: &gtk::Box) {
        {
            let id = String::from(info.id.clone());
            let list_box = list_box.clone();
            row.enable().connect_changed_active(move |switch| {
                if !Zypper::change_repo(id.clone(), Settings::Enable(switch.get_active())) {
                    PageSettings::clear_repo_list(&list_box);
                    PageSettings::build_repo_list(&list_box);
                }
            });
        }
        {
            let id = String::from(info.id.clone());
            let list_box = list_box.clone();
            row.cpg().connect_toggled(move |b| {
                if !Zypper::change_repo(id.clone(), Settings::Cpg(b.get_active())) {
                    PageSettings::clear_repo_list(&list_box);
                    PageSettings::build_repo_list(&list_box);
                }
            });
        }
        {
            let id = String::from(info.id.clone());
            let list_box = list_box.clone();
            row.refresh().connect_toggled(move |b| {
                if !Zypper::change_repo(id.clone(), Settings::Refresh(b.get_active())) {
                    PageSettings::clear_repo_list(&list_box);
                    PageSettings::build_repo_list(&list_box);
                }
            });
        }
        {
            let id = String::from(info.id.clone());
            let list_box = list_box.clone();
            row.priority().connect_value_changed(move |b| {
                if !Zypper::change_repo(id.clone(), Settings::Priority(b.get_value() as i32)) {
                    PageSettings::clear_repo_list(&list_box);
                    PageSettings::build_repo_list(&list_box);
                }
            });
        }
    }

    fn button_connect(builder: &gtk::Builder) {
        {
            let button: gtk::Button = builder.get_object("button_leaflet_back").unwrap();
            let page_settings: libhandy::Leaflet = builder.get_object("page_settings").unwrap();
            button.connect_clicked(move |_| {
                page_settings.navigate(NavigationDirection::Back);
            });
        }

        {
            let stack: gtk::Stack = builder.get_object("setting_stack").unwrap();
            let page_settings: libhandy::Leaflet = builder.get_object("page_settings").unwrap();
            stack
                .connect_local("notify::visible-child", true, move |_| {
                    page_settings.navigate(NavigationDirection::Forward);
                    None
                })
                .expect("connecting to visible-child failed");
        }

        {
            let page_settings: libhandy::Leaflet = builder.get_object("page_settings").unwrap();
            let page_settings1: libhandy::Leaflet = builder.get_object("page_settings").unwrap();
            let header_bar_stack: gtk::Stack = builder.get_object("header_bar_stack").unwrap();
            let back_header_bar: libhandy::HeaderBar =
                builder.get_object("back_header_bar").unwrap();
            let empty_header_bar: libhandy::HeaderBar =
                builder.get_object("empty_header_bar").unwrap();
            page_settings1
                .connect_local("notify::folded", true, move |_| {
                    if page_settings.get_folded() {
                        header_bar_stack.set_visible_child(&back_header_bar);
                    } else {
                        header_bar_stack.set_visible_child(&empty_header_bar);
                    }
                    None
                })
                .expect("connecting to folded failed");
        }
    }
}
