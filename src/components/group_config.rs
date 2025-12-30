use dioxus::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::components::lists::{AppList, ListOperation};
use crate::components::util::{EditableText, HotkeyPicker};
use crate::os::{AppDialog, AppSelection};
use crate::services::ConfigService;

#[component]
pub fn GroupConfig(config_service: Signal<ConfigService>, group_id: Uuid) -> Element {
    let group = use_memo(move || config_service.read().group(group_id).unwrap().clone());
    let picked_hotkey = use_signal(|| group().hotkey);
    use_effect(move || {
        config_service.write().set_hotkey(group_id, picked_hotkey());
    });
    let name = use_signal(|| group().name.clone());
    use_effect(move || config_service.write().set_name(group_id, name()));
    use_app_list_listener(config_service, group_id);

    rsx! {
        div {
            class: "flex flex-col gap-2",
            EditableText { text: name }
            HotkeyPicker { picked_hotkey }
            AppList { apps: group().apps().to_vec() }
        }
    }
}

fn use_app_list_listener(config_service: Signal<ConfigService>, group_id: Uuid) {
    let app_list_listener = use_coroutine(
        move |mut receiver: UnboundedReceiver<ListOperation<String>>| async move {
            while let Some(list_operation) = receiver.next().await {
                do_app_list_operation(config_service, group_id, list_operation).await;
            }
        },
    );
    use_context_provider(|| app_list_listener.tx()); // used in the (generic) list
}

async fn do_app_list_operation(
    mut config_service: Signal<ConfigService>,
    group_id: Uuid,
    list_operation: ListOperation<String>,
) {
    match list_operation {
        ListOperation::Add => {
            if let Ok(Some(app)) = AppDialog::select_app().await {
                config_service.write().add_app(group_id, app)
            }
        }
        ListOperation::Remove(apps) => {
            for app_id in apps {
                config_service.write().remove_app(group_id, app_id);
            }
        }
    }
}
