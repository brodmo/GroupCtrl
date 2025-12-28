use uuid::Uuid;

use crate::models::Config;

#[derive(Default)]
pub struct GroupService {}

impl GroupService {
    pub fn open(&self, config: &Config, group_id: &Uuid) {}
}
