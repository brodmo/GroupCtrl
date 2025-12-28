use dioxus::hooks::UnboundedSender;

use crate::models::{Action, Actionable, Config, Hotkey};
use crate::services::SharedHotkeySender;
use crate::services::hotkey::binder::{DioxusBinder, HotkeyBinder};

pub struct HotkeyService<B: HotkeyBinder = DioxusBinder> {
    binder: B,
}

impl HotkeyService<DioxusBinder> {
    pub fn new(
        record_registered_sender: SharedHotkeySender,
        action_sender: UnboundedSender<Action>,
    ) -> Self {
        Self {
            binder: DioxusBinder::new(record_registered_sender, action_sender),
        }
    }
}

impl<B: HotkeyBinder> HotkeyService<B> {
    fn find_conflict(config: &Config, hotkey: Option<Hotkey>) -> Option<Action> {
        config
            .groups()
            .iter()
            .find(|group| group.hotkey == hotkey)
            .map(|group| group.action())
    }

    pub fn bind_hotkey(
        &mut self,
        config: &Config,
        hotkey: Option<Hotkey>,
        existing_hotkey: Option<Hotkey>,
        action: Action,
    ) -> Option<Action> {
        if hotkey == existing_hotkey {
            return None;
        }
        if let Some(conflict) = Self::find_conflict(config, hotkey) {
            return Some(conflict);
        }

        if let Some(ex_hk) = existing_hotkey {
            self.binder.unbind_hotkey(ex_hk);
        }
        if let Some(hk) = hotkey {
            self.binder.bind_hotkey(hk, &action).unwrap() // TODO handle error
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use global_hotkey::hotkey::{Code, Modifiers};

    use super::super::binder::tests::MockBinder;
    use super::super::binder::tests::MockEvent::*;
    use super::*;
    use crate::services::hotkey::binder::tests::MockEvent;

    impl HotkeyService<MockBinder> {
        fn new_mock(binder: MockBinder) -> Self {
            Self { binder }
        }
    }

    fn setup_service() -> (HotkeyService<MockBinder>, Arc<Mutex<Vec<MockEvent>>>) {
        let events = Arc::new(Mutex::new(Vec::new()));
        let binder = MockBinder {
            events: events.clone(),
        };
        let service = HotkeyService::new_mock(binder);
        (service, events)
    }

    fn setup_group(config: &mut Config, hotkey: Option<Hotkey>) -> Action {
        let group_id = config.add_group("Test".to_string());
        config.set_hotkey(group_id, hotkey);
        Action::OpenGroup { group_id }
    }

    #[test]
    fn bind_hotkey_new() {
        // Arrange
        let (mut service, events) = setup_service();
        let mut config = Config::default();
        let action = setup_group(&mut config, None);
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);

        // Act
        let result = service.bind_hotkey(&config, Some(hotkey), None, action.clone());

        // Assert
        assert_eq!(result, None);
        assert_eq!(*events.lock().unwrap(), vec![Register(hotkey, action)]);
    }

    #[test]
    fn bind_hotkey_repeat_none() {
        // Arrange
        let (mut service, events) = setup_service();
        let mut config = Config::default();
        let action = setup_group(&mut config, None);

        // Act
        let result = service.bind_hotkey(&config, None, None, action.clone());

        // Assert
        assert_eq!(result, None);
        assert_eq!(*events.lock().unwrap(), vec![]);
    }

    #[test]
    fn bind_hotkey_repeat_some() {
        // Arrange
        let (mut service, events) = setup_service();
        let mut config = Config::default();
        let action = setup_group(&mut config, None);
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);

        // Act
        let result = service.bind_hotkey(&config, Some(hotkey), Some(hotkey), action.clone());

        // Assert
        assert_eq!(result, None);
        assert_eq!(*events.lock().unwrap(), vec![]);
    }

    #[test]
    fn bind_hotkey_change() {
        // Arrange
        let (mut service, events) = setup_service();
        let old_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let new_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyG);
        let mut config = Config::default();
        let action = setup_group(&mut config, Some(old_hotkey));

        // Act
        let result =
            service.bind_hotkey(&config, Some(new_hotkey), Some(old_hotkey), action.clone());

        // Assert
        assert_eq!(result, None);
        assert_eq!(
            *events.lock().unwrap(),
            vec![Unregister(old_hotkey), Register(new_hotkey, action)]
        );
    }

    #[test]
    fn bind_hotkey_conflict() {
        // Arrange
        let (mut service, events) = setup_service();
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let mut config = Config::default();
        let old_action = setup_group(&mut config, Some(hotkey));
        let new_action = setup_group(&mut config, None);

        // Act
        let result = service.bind_hotkey(&config, Some(hotkey), None, new_action);

        // Assert
        assert_eq!(result, Some(old_action.clone()));
        assert_eq!(*events.lock().unwrap(), vec![]);
    }
}
