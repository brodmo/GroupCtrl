mod app;
mod open;

use crate::app::App;
use crate::open::Open;
use eframe::egui;
use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager,
    hotkey::{Code, HotKey, Modifiers},
};
use std::thread;

struct GroupCtrl {
    _hotkey_manager: GlobalHotKeyManager,
}

impl GroupCtrl {
    fn listen_for_hotkeys() {
        loop {
            if let Ok(event) = GlobalHotKeyEvent::receiver().recv()
                && event.state == global_hotkey::HotKeyState::Pressed
            {
                App::new("com.apple.finder").open().unwrap();
            }
        }
    }

    fn new() -> Self {
        let manager = GlobalHotKeyManager::new().unwrap();
        // TODO these are HARDWARE keys!
        let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyF);
        manager.register(hotkey).unwrap();
        thread::spawn(Self::listen_for_hotkeys);
        Self {
            // need to keep it alive
            _hotkey_manager: manager,
        }
    }
}

impl eframe::App for GroupCtrl {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // draw UI
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "GroupCtrl",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(GroupCtrl::new()))),
    )
}
