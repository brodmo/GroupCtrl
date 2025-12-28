use dioxus::prelude::*;
use futures_util::StreamExt;

use super::app_selector::AppSelector;
use super::hotkey_picker::HotkeyPicker;
use crate::models::{Action, Hotkey};
use crate::os::App;
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

fn use_action_listener(config_service: Signal<ConfigService>) -> UnboundedSender<Action> {
    let listener = use_coroutine(move |mut receiver: UnboundedReceiver<Action>| async move {
        let mut action_service = ActionService::default();
        while let Some(action) = receiver.next().await {
            action_service.execute(&config_service.read(), &action)
        }
    });
    listener.tx()
}
