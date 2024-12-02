use iced::widget::{button, text};
use iced::{Element, Length};
use crate::theme::{Theme, ButtonVariant};
use crate::Message;

pub struct Button;

#[derive(Debug, Clone)]
pub struct ButtonProps {
    pub label: String,
    pub on_press: Message,
    pub is_primary: bool,
    pub is_active: bool,
    pub width: Length,
    pub padding: u16,
    pub size: u16,
}

impl Button {
    pub fn view<'a>(props: ButtonProps, theme: &Theme) -> Element<'a, Message> {
        let style = if props.is_primary {
            ButtonVariant::Primary
        } else {
            ButtonVariant::Secondary
        };
        
        button(text(&props.label))
            .width(props.width)
            .padding(props.padding)
            .style(theme.button(style))
            .on_press(props.on_press)
            .into()
    }
} 