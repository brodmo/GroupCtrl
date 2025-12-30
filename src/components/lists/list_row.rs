use std::collections::HashSet;
use std::hash::Hash;

use dioxus::prelude::*;

use super::list::Renderable;
use crate::os::{Keyboard, KeyboardBehavior};

#[component]
pub(super) fn ListRow<E, I>(element: E, mut selected: Signal<HashSet<I>>) -> Element
where
    I: Clone + Eq + Hash + 'static,
    E: Renderable<I> + Clone + PartialEq + 'static,
{
    let my_element_id = element.id();
    let is_selected = use_memo(move || selected().contains(&my_element_id));
    let element_id = element.id();
    let toggle_active = move |evt: Event<MouseData>| {
        let mut sel = selected.write();
        if Keyboard::is_multi_select(evt.modifiers()) {
            if !sel.contains(&element_id) {
                sel.insert(element_id.clone());
            } else {
                sel.remove(&element_id);
            }
        } else {
            sel.clear();
            sel.insert(element_id.clone());
        }
    };

    rsx! {
        button {
            // TODO maybe daisy join buttons for styling
           class: format_args!(
                "btn btn-sm w-full justify-start {}",
                if is_selected() { "btn-active" } else { "btn-ghost" }
            ),
            onclick: toggle_active,
            { element.render() }
        }
    }
}
