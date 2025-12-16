use crate::action::Action::OpenApp;
use crate::app::App;
use crate::hotkeys::convert::convert_hotkey;
use crate::hotkeys::{Hotkey, HotkeyManager};
use anyhow::Error;
use global_hotkey::hotkey::Code;
use iced::keyboard::Event;
use iced::widget::{Button, button, text};
use iced::{color, keyboard};

#[derive(Default)]
pub struct HotkeyPicker {
    recording: bool,
    picked: Option<Hotkey>,
    error: Option<Error>,
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
                self.error = hotkey_manager.pause_hotkeys().err();
            }
            Message::KeyPress(hotkey) => {
                if self.recording {
                    self.recording = false;
                    self.error = hotkey_manager.unpause_hotkeys().err();
                    if hotkey.0.key == Code::Escape {
                        self.picked = None;
                        return;
                    }
                    self.picked = Some(hotkey);
                    let action = OpenApp(App::new("com.apple.finder"));
                    self.error = hotkey_manager.bind_hotkey(hotkey, action).err();
                }
            }
        }
    }

    pub fn view(&self) -> Button<'_, Message> {
        let label = if let Some(err) = &self.error {
            text(format!("Error: {}", err)).color(color!(0xff0000))
        } else if self.recording {
            text("Recording...")
        } else {
            match self.picked {
                None => text("None").color(color!(0x888888)),
                Some(key) => text(key.to_string()),
            }
        };
        button(label).on_press(Message::StartRecording)
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        keyboard::listen().filter_map(|event| match event {
            Event::KeyPressed {
                modifiers,
                physical_key,
                ..
            } => convert_hotkey(modifiers, physical_key).map(Message::KeyPress),
            _ => None,
        })
    }
}
