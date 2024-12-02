use iced::widget::container;
use iced::{Element, Length};
use crate::theme::Theme;
use crate::Message;

pub struct Card;

impl Card {
    pub fn view<'a>(content: Element<'a, Message>, theme: &Theme, padding: u16) -> Element<'a, Message> {
        let style = theme.card();
        
        container(content)
            .width(Length::Fill)
            .padding(padding)
            .style(iced::theme::Container::from(style))
            .into()
    }
} 