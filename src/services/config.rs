use uuid::Uuid;

use crate::models::{Action, Config, Group, Hotkey};
use crate::os::App;
use crate::services::{HotkeyService, SharedSender};

pub struct ConfigService {
    config: Config,
    hotkey_service: HotkeyService,
}

impl ConfigService {
    pub fn new(
        record_registered_sender: SharedSender<Hotkey>,
        action_sender: SharedSender<Action>,
    ) -> Self {
        Self {
            config: Config::default(),
            hotkey_service: HotkeyService::new(record_registered_sender, action_sender),
        }
    }

    pub fn groups(&self) -> &Vec<Group> {
        self.config.groups()
    }

    pub fn group(&self, group_id: Uuid) -> Option<&Group> {
        self.config.group(group_id)
    }

    pub fn add_group(&mut self, name: String) -> Uuid {
        self.config.add_group(name)
    }

    pub fn remove_group(&mut self, group_id: Uuid) {
        self.config.remove_group(group_id)
    }

    pub fn set_name(&mut self, group_id: Uuid, name: String) {
        self.config.set_name(group_id, name)
    }

    pub fn add_app(&mut self, group_id: Uuid, app: App) {
        self.config.add_app(group_id, app)
    }

    pub fn remove_app(&mut self, group_id: Uuid, app_id: String) {
        self.config.remove_app(group_id, app_id)
    }

    pub fn set_hotkey(&mut self, group_id: Uuid, hotkey: Option<Hotkey>) -> Option<Action> {
        let (existing_hotkey, action) = self.config.get_binding(group_id).unwrap();
        let conflict =
            self.hotkey_service
                .bind_hotkey(&self.config, hotkey, existing_hotkey, action);
        if conflict.is_none() {
            self.config.set_hotkey(group_id, hotkey);
        }
        conflict
    }
}
