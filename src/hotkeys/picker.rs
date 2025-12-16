use crate::action::Action::OpenApp;
use crate::app::App;
use crate::hotkeys::convert::convert_hotkey;
use crate::hotkeys::{Hotkey, HotkeyManager};
use global_hotkey::hotkey::Code;
use iced::keyboard;
use iced::keyboard::Event;
use iced::widget::{Button, button, text};

#[derive(Default)]
pub struct HotkeyPicker {
    recording: bool,
    picked: Option<Hotkey>,
}

#[derive(Clone, Debug)]
pub enum Message {
    StartRecording,
    KeyPress(Hotkey),
}

impl HotkeyPicker {
    pub fn update(&mut self, message: Message, hotkey_manager: &mut HotkeyManager) {
        match message {
            Message::StartRecording => {
                self.recording = true;
                hotkey_manager.pause_hotkeys().unwrap();
            }
            Message::KeyPress(hotkey) => {
                if self.recording {
                    self.recording = false;
                    hotkey_manager.unpause_hotkeys().unwrap();
                    if hotkey.0.key == Code::Escape {
                        return;
                    }
                    let action = OpenApp(App::new("com.apple.finder"));
                    hotkey_manager.bind_hotkey(hotkey, action).unwrap();
                    self.picked = Some(hotkey);
                }
            }
        }
    }

    pub fn view(&self) -> Button<'_, Message> {
        let label = if self.recording {
            text("Recording...")
        } else {
            match self.picked {
                None => text("None"),
                Some(key) => text(key.to_string()),
            }
        };
        button(label).on_press(Message::StartRecording)
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        keyboard::listen().filter_map(|event| {
            if let Event::KeyPressed {
                modifiers,
                physical_key,
                ..
            } = event
            {
                Some(Message::KeyPress(convert_hotkey(modifiers, physical_key)?))
            } else {
                None
            }
        })
    }
}
