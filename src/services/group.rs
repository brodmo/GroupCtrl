use uuid::Uuid;

use crate::os::Openable;
use crate::services::ConfigService;

#[derive(Default)]
pub struct GroupService {}

impl GroupService {
    pub fn open(&self, config_service: &ConfigService, group_id: Uuid) {
        let apps = config_service.group(group_id).unwrap().apps();
        if let Some(app) = apps.iter().next() {
            // TODO find current app and go next if in group
            let _ = app.open();
        }
    }
}
