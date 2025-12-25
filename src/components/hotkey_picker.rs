use std::sync::Arc;

use dioxus::prelude::*;
use futures_util::StreamExt;

use crate::models::Hotkey;
use crate::services::{HotkeyCallback, SharedHotkeyCallback};

#[component]
pub(super) fn HotkeyPicker(mut picked_hotkey: Signal<Option<Hotkey>>) -> Element {
    let mut recording = use_signal(|| false);
    let start_recording = move |_| {
        recording.set(true);
    };

    let record_unregistered = move |evt: KeyboardEvent| {
        record_unregistered(recording, picked_hotkey, evt);
    };
    use_record_registered(recording, picked_hotkey);

    let label = if recording() {
        "Recording...".to_string()
    } else {
        match picked_hotkey() {
            None => "None".to_string(),
            Some(key) => key.to_string(),
        }
    };
    let label_color = if label == "None" { "gray" } else { "black" };
    rsx! {
        div {
            onkeydown: record_unregistered, // globally registered keys never make it here
            tabindex: 0,
            button {
                onclick: start_recording,
                style: "color: {label_color};",
                "{label}"
            }
        }
    }
}

fn record_unregistered(
    mut recording: Signal<bool>,
    mut picked_hotkey: Signal<Option<Hotkey>>,
    evt: KeyboardEvent,
) {
    fn is_modifier(code: &Code) -> bool {
        let code_str = code.to_string();
        code_str.contains("Control")
            || code_str.contains("Meta")
            || code_str.contains("Alt")
            || code_str.contains("Shift")
    }

    let code = evt.code();
    if !recording() || is_modifier(&code) {
        return;
    }
    recording.set(false);
    picked_hotkey.set(if code == Code::Escape {
        None
    } else {
        Some(Hotkey::new(evt.modifiers(), code))
    })
}

fn use_record_registered(mut recording: Signal<bool>, mut picked_hotkey: Signal<Option<Hotkey>>) {
    let record_registered = use_context::<SharedHotkeyCallback>();
    let hotkey_coroutine = use_coroutine(move |mut rx: UnboundedReceiver<Hotkey>| async move {
        while let Some(hotkey) = rx.next().await {
            recording.set(false);
            picked_hotkey.set(Some(hotkey));
        }
    });
    // This is called by the OS thread and therefore can't manipulate UI
    // Thus we need to send UI updates to a coroutine
    use_effect(move || {
        let callback = if recording() {
            let tx = hotkey_coroutine.tx();
            let cb: HotkeyCallback = Arc::new(move |hotkey: Hotkey| {
                let _ = tx.unbounded_send(hotkey);
            });
            Some(cb)
        } else {
            None
        };
        record_registered.set(callback);
    });
}
