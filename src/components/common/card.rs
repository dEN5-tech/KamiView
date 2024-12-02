use iced::widget::container;
use iced::{Element, Length};
use crate::theme::{Theme, ContainerVariant};
use crate::Message;

pub struct Card;

impl Card {
    pub fn view<'a>(content: Element<'a, Message>, theme: &Theme, padding: u16) -> Element<'a, Message> {
        container(content)
            .width(Length::Fill)
            .padding(padding)
            .style(theme.container(ContainerVariant::Card))
            .into()
    }
} 