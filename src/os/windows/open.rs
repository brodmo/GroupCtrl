use crate::os::interface::OpenInterface;
use crate::os::windows::app::App;

impl OpenInterface for App {
    fn open(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
