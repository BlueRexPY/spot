use gtk::prelude::*;
use gtk::SearchBarExt;
use std::rc::Rc;

use super::SearchBarModel;
use crate::app::components::EventListener;

pub struct SearchBar;

impl SearchBar {
    pub fn new(
        model: SearchBarModel,
        search_button: gtk::Button,
        search_bar: gtk::SearchBar,
        search_entry: gtk::SearchEntry,
    ) -> Self {
        let model = Rc::new(model);

        {
            let model = model.clone();
            search_entry.connect_changed(move |s| {
                let query = s.get_text().as_str().to_string();
                if !query.is_empty() {
                    model.search(query);
                }
            });
        }

        search_entry.connect_focus_in_event(move |s, _| {
            let query = s.get_text().as_str().to_string();
            if !query.is_empty() {
                model.search(query);
            }
            Inhibit(false)
        });

        search_button.connect_clicked(clone!(@weak search_bar => move |_| {
            search_bar.set_search_mode(true);
        }));

        search_bar.connect_entry(&search_entry);

        Self
    }
}

impl EventListener for SearchBar {}
