#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
pub use macos::App;
#[cfg(target_os = "windows")]
pub use windows::App;

use anyhow::Result;

pub trait AppInterface {
    fn id(&self) -> &str;
    fn new(id: &str) -> Self;
    fn open(&self) -> Result<()>;
    fn display(&self) -> String;
}
