use std::collections::HashSet;

use dioxus::prelude::*;

use crate::components::list::List;
use crate::components::list_cell::ListCell;
use crate::models::Identifiable;
use crate::os::App;

#[component]
pub fn AppList(apps: Vec<App>) -> Element {
    let selected = use_signal(|| HashSet::<String>::new());

    rsx! {
        List {
            elements: apps,
            selected,
        }
    }
}

impl ListCell<String> for App {
    fn render(&self) -> Element {
        rsx! {
            span { "{self.id()}" }
        }
    }
}
