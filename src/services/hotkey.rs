use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::anyhow;
use bimap::BiMap;
use dioxus::desktop::{ShortcutHandle, window};
use global_hotkey::HotKeyState;
use global_hotkey::hotkey::HotKey as GlobalHotkey;
use log::info;

use crate::models::action::Action;
use crate::models::hotkey::Hotkey;

pub trait HotkeyBinder {
    fn create_shortcut(
        &mut self,
        hotkey: GlobalHotkey,
        action: &Action,
    ) -> anyhow::Result<()>;

    fn remove_shortcut(&mut self, hotkey: GlobalHotkey) -> anyhow::Result<()>;
}

pub struct DioxusBinder {
    recording: Arc<AtomicBool>,
    handles: HashMap<u32, ShortcutHandle>,
}

impl DioxusBinder {
    pub fn new(recording: Arc<AtomicBool>) -> Self {
        Self {
            recording,
            handles: HashMap::new(),
        }
    }
}

impl HotkeyBinder for DioxusBinder {
    fn create_shortcut(
        &mut self,
        hotkey: GlobalHotkey,
        action: &Action,
    ) -> anyhow::Result<()> {
        let my_action = action.clone();
        let my_recording = self.recording.clone();
        let callback = move |state| {
            if state == HotKeyState::Pressed && !my_recording.load(Ordering::SeqCst) {
                let _ = my_action.execute();
            }
        };

        let handle = window()
            .create_shortcut(hotkey, callback)
            .map_err(|e| anyhow!("Failed to create shortcut: {:?}", e))?;

        self.handles.insert(hotkey.id, handle);
        Ok(())
    }

    fn remove_shortcut(&mut self, hotkey: GlobalHotkey) -> anyhow::Result<()> {
        if let Some(handle) = self.handles.remove(&hotkey.id) {
            window().remove_shortcut(handle);
        }
        Ok(())
    }
}

pub struct HotkeyService<B: HotkeyBinder = DioxusBinder> {
    bindings: BiMap<Hotkey, Action>,
    binder: B,
}

impl HotkeyService<DioxusBinder> {
    pub fn new(recording: Arc<AtomicBool>) -> Self {
        Self {
            bindings: BiMap::new(),
            binder: DioxusBinder::new(recording),
        }
    }
}

impl<B: HotkeyBinder> HotkeyService<B> {
    pub fn new_with_binder(binder: B) -> Self {
        Self {
            bindings: BiMap::new(),
            binder,
        }
    }

    /// Returns existing bind if hotkey is already in use
    pub fn bind_hotkey(
        &mut self,
        hotkey: Hotkey,
        action: Action,
    ) -> anyhow::Result<Option<Action>> {
        info!("Binding {hotkey} to '{action}'");
        if let Some(previous_action) = self.bindings.get_by_left(&hotkey) {
            if *previous_action == action {
                return Ok(None); // equivalent to registration
            }
            info!("Hotkey is already assigned to {previous_action}");
            return Ok(Some(previous_action.clone()));
        }
        if let Some((previous_hotkey, _)) = self.bindings.remove_by_right(&action) {
            self.binder.remove_shortcut(previous_hotkey.0)?;
        }

        self.binder.create_shortcut(hotkey.0, &action)?;

        self.bindings.insert(hotkey, action);
        Ok(None)
    }

    pub fn hotkeys(&self) -> Vec<GlobalHotkey> {
        self.bindings.left_values().map(|h| h.0).collect()
    }
}

#[cfg(test)]
mod tests {
    use global_hotkey::hotkey::{Code, Modifiers};
    use serial_test::serial;
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::os::App;
    use crate::os::prelude::*;

    #[derive(Debug, PartialEq, Clone)]
    enum MockEvent {
        Register(u32, Action),
        Unregister(u32),
    }

    struct MockBinder {
        events: Arc<Mutex<Vec<MockEvent>>>,
    }

    impl HotkeyBinder for MockBinder {
        fn create_shortcut(
            &mut self,
            hotkey: GlobalHotkey,
            action: &Action,
        ) -> anyhow::Result<()> {
            self.events
                .lock()
                .unwrap()
                .push(MockEvent::Register(hotkey.id, action.clone()));
            Ok(())
        }

        fn remove_shortcut(&mut self, hotkey: GlobalHotkey) -> anyhow::Result<()> {
            self.events
                .lock()
                .unwrap()
                .push(MockEvent::Unregister(hotkey.id));
            Ok(())
        }
    }

    #[test]
    #[serial]
    fn bind_hotkey_new() {
        // Arrange
        let events = Arc::new(Mutex::new(Vec::new()));
        let binder = MockBinder {
            events: events.clone(),
        };
        let mut service = HotkeyService::new_with_binder(binder);
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let action = Action::OpenApp(App::new("some-app"));

        // Act
        let result = service.bind_hotkey(hotkey, action.clone()).unwrap();

        // Assert
        assert_eq!(result, None);
        let events = events.lock().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], MockEvent::Register(hotkey.id(), action));
    }

    #[test]
    #[serial]
    fn bind_hotkey_repeat() {
        // Arrange
        let events = Arc::new(Mutex::new(Vec::new()));
        let binder = MockBinder {
            events: events.clone(),
        };
        let mut service = HotkeyService::new_with_binder(binder);
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let action = Action::OpenApp(App::new("some-app"));

        // Act
        service.bind_hotkey(hotkey, action.clone()).unwrap();
        let result = service.bind_hotkey(hotkey, action.clone()).unwrap();

        // Assert
        assert_eq!(result, None);
        let events = events.lock().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], MockEvent::Register(hotkey.id(), action));
    }

    #[test]
    #[serial]
    fn bind_hotkey_conflict() {
        // Arrange
        let events = Arc::new(Mutex::new(Vec::new()));
        let binder = MockBinder {
            events: events.clone(),
        };
        let mut service = HotkeyService::new_with_binder(binder);
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let old_action = Action::OpenApp(App::new("some-app"));
        let new_action = Action::OpenApp(App::new("some-other-app"));

        // Act
        service.bind_hotkey(hotkey, old_action.clone()).unwrap();
        let result = service.bind_hotkey(hotkey, new_action).unwrap();

        // Assert
        assert_eq!(result, Some(old_action.clone()));
        let events = events.lock().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], MockEvent::Register(hotkey.id(), old_action));
    }

    #[test]
    #[serial]
    fn bind_hotkey_change() {
        // Arrange
        let events = Arc::new(Mutex::new(Vec::new()));
        let binder = MockBinder {
            events: events.clone(),
        };
        let mut service = HotkeyService::new_with_binder(binder);
        let old_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let new_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyG);
        let action = Action::OpenApp(App::new("some-app"));

        // Act
        service.bind_hotkey(old_hotkey, action.clone()).unwrap();
        let result = service.bind_hotkey(new_hotkey, action.clone()).unwrap();

        // Assert
        assert_eq!(result, None);
        let events = events.lock().unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(
            events[0],
            MockEvent::Register(old_hotkey.id(), action.clone())
        );
        assert_eq!(events[1], MockEvent::Unregister(old_hotkey.id()));
        assert_eq!(events[2], MockEvent::Register(new_hotkey.id(), action));
    }
}