use crate::app::AppInterface;
use crate::util::capitalize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct App {
    bundle_id: String,
}

impl AppInterface for App {
    fn new(bundle_id: &str) -> Self {
        Self { bundle_id }
    }

    fn id(&self) -> &String {
        self.bundle_id
    }

    fn display(&self) -> String {
        let name = self
            .bundle_id
            .split(".")
            .last()
            .unwrap_or(self.bundle_id.as_str());
        write!(f, "{}", capitalize(name))
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
