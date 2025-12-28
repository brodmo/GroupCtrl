use dioxus::prelude::*;
use futures_util::StreamExt;

use super::app_selector::AppSelector;
use super::hotkey_picker::HotkeyPicker;
use crate::models::{Action, Config, Hotkey};
use crate::os::App;
use crate::services::{ActionService, ConfigService, HotkeyService, SharedHotkeySender};

#[component]
pub fn Root() -> Element {
    let registered_record_sender = use_hook(SharedHotkeySender::default);
    let config = use_signal(|| Config::default());
    let action_sender = use_action_listener(config);
    // let config_service = use_signal(|| ConfigService::)
    let hotkey_service =
        use_signal(|| HotkeyService::new(registered_record_sender.clone(), action_sender));
    use_context_provider(|| registered_record_sender); // provide to hotkey pickers

    let picked_hotkey = use_signal(|| None::<Hotkey>);
    let selected_app = use_signal(|| None::<App>);
    use_effect(move || {
        if let (Some(hotkey), Some(app)) = (picked_hotkey(), selected_app()) {
            todo!();
            // let action = Action::OpenGroup(app);
            // let _ = hotkey_service.write().bind_hotkey(hotkey, action);
        }
    });

    rsx! {
        div {
            class: "flex gap-4 p-6 items-center justify-center h-screen",
            style { "{include_str!(\"../../target/tailwind.css\")}" }
            HotkeyPicker { picked_hotkey }
            AppSelector { selected_app }
        }
    }
}

fn use_action_listener(config: Signal<Config>) -> UnboundedSender<Action> {
    let listener = use_coroutine(move |mut receiver: UnboundedReceiver<Action>| async move {
        let mut action_service = ActionService::default();
        while let Some(action) = receiver.next().await {
            action_service.execute(&config.read(), &action)
        }
    });
    listener.tx()
}
