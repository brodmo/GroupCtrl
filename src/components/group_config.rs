use std::rc::Rc;

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

    // TODO extract to use_app_list_change_listener (see root)
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

    // TODO extract to group_name.rs
    let name = move || config_service.read().group(group_id).unwrap().name.clone();
    let mut draft_name = use_signal(|| name());
    let mut input_handle = use_signal(|| None::<Rc<MountedData>>);
    let onkeydown = move |evt: KeyboardEvent| {
        match evt.key() {
            Key::Enter => config_service.write().set_name(group_id, draft_name()),
            Key::Escape => draft_name.set(name()),
            _ => return,
        }
        let _ = input_handle.read().as_ref().unwrap().set_focus(false);
    };
    let onblur = move |_| draft_name.set(name());
    let apps = config_service
        .read()
        .group(group_id)
        .unwrap()
        .apps()
        .to_vec();
    rsx! {
        div {
            class: "flex flex-col gap-2",
            input {
                class: "input input-ghost input-xs font-bold text-sm w-full",
                value: "{draft_name}",
                onmounted: move |evt| input_handle.set(Some(evt.data())),
                oninput: move |evt| draft_name.set(evt.value()),
                onkeydown,
                onblur
            }
            HotkeyPicker { picked_hotkey }
            AppList { apps }
        }
    }
}
