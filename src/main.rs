mod action;
mod hotkeys;
mod os;
mod util;

use crate::hotkeys::{HotkeyManager, HotkeyPicker, PickerMessage};
use anyhow::Result;
use iced::Element;
use simplelog::*;
use std::fs;

#[derive(Default)]
struct GroupCtrl {
    hotkey_manager: HotkeyManager,
    hotkey_picker: HotkeyPicker,
}

#[derive(Clone, Debug)]
enum Message {
    Picker(PickerMessage),
}

impl GroupCtrl {
    fn update(&mut self, message: Message) {
        match message {
            Message::Picker(picker_message) => {
                self.hotkey_picker
                    .update(picker_message, &mut self.hotkey_manager);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        Element::from(self.hotkey_picker.view()).map(Message::Picker)
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        self.hotkey_picker.subscription().map(Message::Picker)
    }
}

fn setup_logging() -> Result<()> {
    fs::create_dir_all("logs")?;
    let log_file = fs::File::create("logs/app.log")?;
    let config = ConfigBuilder::new().build();
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Debug, config, log_file),
    ])?;
    Ok(())
}

fn main() -> iced::Result {
    setup_logging().expect("Logging setup failed");

    // Make panics crash loudly during development
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("PANIC: {}", panic_info);
        std::process::exit(1);
    }));

    iced::application(GroupCtrl::default, GroupCtrl::update, GroupCtrl::view)
        .subscription(GroupCtrl::subscription)
        .run()
}
