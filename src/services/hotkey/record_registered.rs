use std::sync::{Arc, Mutex};

use crate::models::Hotkey;

pub type RecordRegisteredFn = Arc<dyn Fn(Hotkey) + Send + Sync>;

#[derive(Clone, Default)]
pub struct RecordRegistered(Arc<Mutex<Option<RecordRegisteredFn>>>);

impl RecordRegistered {
    pub fn set(&self, callback: Option<RecordRegisteredFn>) {
        *self.0.lock().unwrap() = callback;
    }

    pub(super) fn get(&self) -> Option<RecordRegisteredFn> {
        self.0.lock().unwrap().clone()
    }
}
