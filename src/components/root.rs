use dioxus::prelude::*;

use super::app_selector::AppSelector;
use super::hotkey_picker::HotkeyPicker;
use crate::hooks::use_recording_state;
use crate::models::{Action, Hotkey};
use crate::os::App;
use crate::services::HotkeyService;

#[component]
pub fn Root() -> Element {
    let (recording, recording_atomic) = use_recording_state();
    let mut hotkey_service = use_signal(|| HotkeyService::new(recording_atomic));
    use_context_provider(|| recording);
    use_context_provider(|| hotkey_service);

    let picked_hotkey = use_signal(|| None::<Hotkey>);
    let selected_app = use_signal(|| None::<App>);
    use_effect(move || {
        if let (Some(hotkey), Some(app)) = (picked_hotkey(), selected_app()) {
            let action = Action::OpenApp(app);
            let _ = hotkey_service.write().bind_hotkey(hotkey, action);
        }
    });

    rsx! {
        div {
            style: "display: flex; gap: 10px; padding: 20px;",
            HotkeyPicker { picked_hotkey }
            AppSelector { selected_app }
        }
    }
}
