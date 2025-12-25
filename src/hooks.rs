use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use dioxus::prelude::*;

pub fn use_recording_state() -> (Signal<bool>, Arc<AtomicBool>) {
    let signal = use_signal(|| false); // for UI code
    let shared = use_hook(|| Arc::new(AtomicBool::new(false))); // for callbacks
    let shared_clone = shared.clone();
    use_effect(move || {
        shared_clone.store(signal(), Ordering::Relaxed);
    });
    (signal, shared)
}
