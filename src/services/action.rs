use crate::models::{Action, Config};
use crate::services::group::GroupService;

#[derive(Default)]
pub struct ActionService {
    group_service: GroupService,
}

impl ActionService {
    pub fn execute(&mut self, config: &Config, action: &Action) {
        match action {
            Action::OpenGroup { group_id } => self.group_service.open(config, group_id),
            #[cfg(test)]
            Action::Mock(_) => {}
        }
    }
}
