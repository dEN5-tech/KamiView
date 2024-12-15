use iced::Command;
use crate::gui::types::{Message, Screen, SettingsTab};

pub(crate) fn handle_settings_tab_changed(
    tab: SettingsTab,
    current_screen: &mut Screen,
) -> Command<Message> {
    if let Screen::Settings(args) = current_screen {
        args.active_tab = tab;
    }
    Command::none()
} 