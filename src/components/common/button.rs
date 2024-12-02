use iced::widget::{button, text};
use iced::{Element, Length};
use crate::theme::Theme;
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
        let style = theme.button(props.is_primary, props.is_active);
        
        button(text(&props.label))
            .width(props.width)
            .padding(props.padding)
            .style(iced::theme::Button::from(style))
            .on_press(props.on_press)
            .into()
    }
} 