use iced::widget::{Container, Row, Text};
use iced::{Element, Length};
use crate::gui::types::Message;

pub struct Header;

impl Header {
    pub fn view() -> Element<'static, Message> {
        Container::new(
            Row::new()
                .push(Text::new("KamiView"))
                .width(Length::Fill)
        )
        .into()
    }
} 