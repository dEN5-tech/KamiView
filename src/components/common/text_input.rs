use iced::widget::text_input;
use iced::{Element, Length};
use iced::theme;

use crate::Message;
use crate::theme::Theme;
use crate::styles::CustomTextInput;

pub struct TextInput;

#[derive(Debug, Clone)]
pub struct TextInputProps<'a> {
    pub placeholder: &'a str,
    pub value: &'a str,
    pub on_change: fn(String) -> Message,
    pub width: Length,
    pub padding: u16,
    pub size: u16,
}

impl TextInput {
    pub fn view<'a>(props: TextInputProps<'a>, theme: &Theme) -> Element<'a, Message> {
        let TextInputProps {
            placeholder,
            value,
            on_change,
            width,
            padding,
            size,
        } = props;

        text_input(placeholder, value)
            .on_input(on_change)
            .padding(padding)
            .size(size)
            .width(width)
            .style(theme::TextInput::Custom(Box::new(CustomTextInput {
                color: theme.surface,
            })))
            .into()
    }
} 