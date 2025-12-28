use uuid::Uuid;

use crate::models::group::Group;
use crate::models::hotkey::Hotkey;
use crate::models::{Action, Actionable};
use crate::os::App;

#[derive(Default)]
pub struct Config {
    groups: Vec<Group>,
    // settings: Settings as enum (can implement Actionable)
}

impl Config {
    pub fn groups(&self) -> &Vec<Group> {
        &self.groups
    }

    pub fn add_group(&mut self, name: String) -> Uuid {
        let group = Group::new(name);
        let group_id = group.id();
        self.groups.push(group);
        group_id
    }

    fn find_group(&self, group_id: Uuid) -> &Group {
        self.groups.iter().find(|g| g.id() == group_id).unwrap()
    }

    fn find_group_mut(&mut self, group_id: Uuid) -> &mut Group {
        self.groups.iter_mut().find(|g| g.id() == group_id).unwrap()
    }

    pub fn set_name(&mut self, group_id: Uuid, name: String) {
        let group = self.find_group_mut(group_id);
        group.name = name;
    }

    pub fn add_app(&mut self, group_id: Uuid, app: App) {
        let group = self.find_group_mut(group_id);
        group.add_app(app)
    }

    pub fn remove_app(&mut self, group_id: Uuid, app: &App) {
        let group = self.find_group_mut(group_id);
        group.remove_app(&app)
    }

    pub fn get_binding(&self, group_id: Uuid) -> (Option<Hotkey>, Action) {
        let group = self.find_group(group_id);
        (group.hotkey, group.action())
    }

    pub fn set_hotkey(&mut self, group_id: Uuid, hotkey: Option<Hotkey>) {
        let group = self.find_group_mut(group_id);
        group.hotkey = hotkey;
    }
}
