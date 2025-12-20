use crate::os::interface::ModifierFormatInterface;
use global_hotkey::hotkey::Modifiers;

pub struct ModifierFormat;

impl ModifierFormatInterface for ModifierFormat {
    fn get() -> [(Modifiers, &'static str); 4] {
        [
            (Modifiers::CONTROL, "Ctrl+"),
            (Modifiers::SUPER, "Win+"),
            (Modifiers::ALT, "Alt+"),
            (Modifiers::SHIFT, "Shift+"),
        ]
    }
}
