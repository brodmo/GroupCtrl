use dioxus::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::components::app_list::AppList;
use crate::components::hotkey_picker::HotkeyPicker;
use crate::components::list::CellChange;
use crate::os::{AppDialog, AppSelection};
use crate::services::ConfigService;

#[component]
pub fn GroupConfig(config_service: Signal<ConfigService>, group_id: Uuid) -> Element {
    let picked_hotkey = use_signal(|| {
        config_service
            .read()
            .group(group_id)
            .unwrap()
            .hotkey
            .clone()
    });
    use_effect(move || {
        let hotkey = *picked_hotkey.read();
        let mut service = config_service.write();
        service.set_hotkey(group_id, hotkey);
    });

    let handle_app_list_change = use_coroutine(
        move |mut receiver: UnboundedReceiver<CellChange<String>>| async move {
            while let Some(cc) = receiver.next().await {
                let mut cs = config_service.write();
                match cc {
                    CellChange::Add => {
                        if let Ok(Some(app)) = AppDialog::select_app().await {
                            cs.add_app(group_id, app)
                        }
                    }
                    CellChange::Remove(apps) => {
                        for app_id in apps {
                            cs.remove_app(group_id, app_id);
                        }
                    }
                }
            }
        },
    );
    use_context_provider(|| handle_app_list_change.tx()); // used in the (generic) list

    let name = config_service.read().group(group_id).unwrap().name.clone();
    let apps = config_service
        .read()
        .group(group_id)
        .unwrap()
        .apps()
        .to_vec();
    rsx! {
        div {
        ul {
            class: "menu",
            li { "{name}" }
            li { HotkeyPicker { picked_hotkey } }
            li { AppList { apps }}
        }
        }
    }
}
