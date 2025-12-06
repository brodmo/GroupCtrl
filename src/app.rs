#[derive(Debug)]
pub struct App {
    pub bundle_id: String,
}

impl App {
    pub fn new(bundle_id: &str) -> App {
        Self {
            bundle_id: bundle_id.to_string(),
        }
    }
}
