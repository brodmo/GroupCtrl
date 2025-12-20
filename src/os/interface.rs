use anyhow::Result;
use global_hotkey::hotkey::Modifiers;

pub trait AppInterface {
    fn id(&self) -> &str;
    fn new(id: &str) -> Self;
    fn display(&self) -> String;
}

pub trait OpenInterface {
    fn open(&self) -> Result<()>;
}

pub trait ModifierFormatInterface {
    fn get() -> [(Modifiers, &'static str); 4];
}
