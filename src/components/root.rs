use std::collections::HashSet;

use dioxus::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::components::group_config::GroupConfig;
use crate::components::group_list::GroupList;
use crate::components::list::CellChange;
use crate::models::Action;
use crate::services::{ActionService, ConfigService, SharedSender};

#[component]
pub fn Root() -> Element {
    let registered_record_sender = use_hook(|| SharedSender::new());
    let action_sender = use_hook(|| SharedSender::new());
    let config_service =
        use_signal(|| ConfigService::new(registered_record_sender.clone(), action_sender.clone()));
    action_sender.set(Some(use_action_listener(config_service)));
    use_context_provider(|| registered_record_sender);
    use_context_provider(|| action_sender);

    let selected = use_signal(|| HashSet::<Uuid>::new());
    use_groups_list_change_listener(config_service, selected);
    let active_group = use_memo(move || {
        let sel = selected.read();
        if sel.len() == 1 {
            sel.iter().next().copied()
        } else {
            None
        }
    });

    rsx! {
        Stylesheet { href: asset!("/assets/tailwind.css") }
        div {
            class: "flex h-screen",
            aside {
                class: "flex-1 p-2 border-r",
                GroupList {
                    groups: config_service.read().groups().clone(),
                    selected
                }
            }
            main {
                class: "flex-1 p-2",
                if let Some(group_id) = active_group() {
                    GroupConfig {
                        key: "{group_id}",
                        config_service,
                        group_id
                    }
                }
            }
        }
    }
}

fn use_groups_list_change_listener(
    mut config_service: Signal<ConfigService>,
    mut selected: Signal<HashSet<Uuid>>,
) {
    let handle_app_change = use_coroutine(
        move |mut receiver: UnboundedReceiver<CellChange<Uuid>>| async move {
            while let Some(cc) = receiver.next().await {
                let mut cs = config_service.write();
                match cc {
                    CellChange::Add => {
                        let group_id = cs.add_group("New Group".to_string());
                        let mut sel = selected.write();
                        sel.clear();
                        sel.insert(group_id);
                    }
                    CellChange::Remove(groups) => {
                        for group_id in groups {
                            cs.remove_group(group_id)
                        }
                    }
                }
            }
        },
    );
    use_context_provider(|| handle_app_change.tx()); // used in the list
}

fn use_action_listener(config_service: Signal<ConfigService>) -> UnboundedSender<Action> {
    let listener = use_coroutine(move |mut receiver: UnboundedReceiver<Action>| async move {
        let mut action_service = ActionService::default();
        while let Some(action) = receiver.next().await {
            action_service.execute(&config_service.read(), &action)
        }
    });
    listener.tx()
}
