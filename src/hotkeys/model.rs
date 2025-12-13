use global_hotkey::hotkey::{Code, HotKey as GlobalHotkey, Modifiers};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Hotkey(pub GlobalHotkey);

impl Hotkey {
    pub fn new(mods: Modifiers, key: Code) -> Self {
        Self(GlobalHotkey::new(Some(mods), key))
    }

    pub fn id(&self) -> u32 {
        self.0.id
    }
}

impl Display for Hotkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (modifier, text) in [
            (Modifiers::SUPER, "Cmd+"),
            (Modifiers::ALT, "Opt+"),
            (Modifiers::CONTROL, "Ctrl+"),
            (Modifiers::SHIFT, "Shift+"),
        ] {
            if self.0.mods.contains(modifier) {
                write!(f, "{}", text)?;
            }
        }
        write!(f, "{}", self.0.key)
    }
}
