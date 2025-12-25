use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use dioxus::prelude::*;

use crate::components::app_selector::AppSelector;
use crate::components::hotkey_picker::HotkeyPicker;
use crate::models::action::Action;
use crate::models::hotkey::Hotkey;
use crate::os::App;
use crate::services::hotkey::HotkeyService;

#[component]
pub fn Root() -> Element {
    let selected_app = use_signal(|| None::<App>);
    let picked_hotkey = use_signal(|| None::<Hotkey>);

    // We need both a signal for UI updates and an Arc<AtomicBool> for the background hotkey service
    let recording_atomic = use_signal(|| Arc::new(AtomicBool::new(false)));
    let recording = use_signal(|| false);

    // Sync the signal to the atomic bool
    use_effect(move || {
        recording_atomic().store(recording(), Ordering::SeqCst);
    });

    let mut hotkey_service = use_signal(|| HotkeyService::new(recording_atomic()));

    // Provide recording signal to child components (UI interaction)
    use_context_provider(|| recording);
    use_context_provider(|| hotkey_service);

    use_effect(move || {
        if let (Some(app), Some(hotkey)) = (selected_app(), picked_hotkey()) {
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