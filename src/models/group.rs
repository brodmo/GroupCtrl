use std::fmt::{Display, Formatter};

use uuid::Uuid;

use crate::models::Hotkey;
use crate::os::App;

#[derive(Debug)]
pub struct Group {
    id: Uuid,
    pub name: String,
    pub hotkey: Option<Hotkey>,
    apps: Vec<App>,
}

impl Group {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            hotkey: None,
            apps: Vec::new(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn apps(&self) -> &Vec<App> {
        &self.apps
    }

    pub fn add_app(&mut self, app: App) {
        self.apps.push(app);
    }

    pub fn remove_app(&mut self, app: &App) {
        self.apps.retain(|a| a != app)
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
