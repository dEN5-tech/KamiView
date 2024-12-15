use iced::{
    widget::{container, text, Column, Row, Space},
    Element, Length, Command,
};
use crate::di::interfaces::PlaybackInfo;
use crate::gui::types::Message;

#[derive(Debug, Clone)]
pub struct HomeScreen {
}

impl HomeScreen {
    pub fn new() -> (Self, Command<Message>) {
        let commands = Command::none();

        (
            Self {},
            commands
        )
    }

    pub fn view_playback_info(playback_info: &PlaybackInfo) -> Element<'_, Message> {
        let position = text(format!("Position: {:.1}s", playback_info.position))
            .size(16);
            
        let duration = text(format!("Duration: {:.1}s", playback_info.duration))
            .size(16);
            
        let status = text(if playback_info.paused { "Paused" } else { "Playing" })
            .size(16);

        Row::new()
            .push(position)
            .push(Space::with_width(Length::Fixed(10.0)))
            .push(duration)
            .push(Space::with_width(Length::Fixed(10.0)))
            .push(status)
            .spacing(5)
            .into()
    }

    pub fn view_with_playback<'a>(playback_info: Option<&'a PlaybackInfo>) -> Element<'a, Message> {
        let (screen, _) = Self::new();
        
        let title = text("Welcome to KamiView")
            .size(32)
            .width(Length::Fill);

        let mut content = Column::new()
            .spacing(20)
            .padding(20)
            .push(title);

        if let Some(info) = playback_info {
            content = content.push(Self::view_playback_info(info));
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}