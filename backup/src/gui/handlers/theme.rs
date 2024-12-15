use iced::Command;
use log::error;
use crate::gui::types::Message;
use crate::di::Container;
use crate::storage::ThemeType;

pub fn handle_theme_changed(
    container: &std::sync::Arc<Container>,
    new_theme: iced::Theme,
    theme: &mut iced::Theme,
) -> Command<Message> {
    *theme = new_theme.clone();
    
    let mut settings = container.storage().load();
    settings.theme = match new_theme {
        iced::Theme::Light => ThemeType::Light,
        _ => ThemeType::Dark,
    };
    
    if let Err(e) = container.storage().save(&settings) {
        error!("Failed to save settings: {}", e);
    }
    
    Command::none()
} 