use iced::alignment::{Horizontal, Vertical};
use iced::widget::text;
use iced::{Element, Color};

use crate::Message;
use crate::fonts;

pub struct Text;

#[derive(Debug, Clone)]
pub struct TextProps {
    pub content: String,
    pub size: u16,
    pub color: Color,
    pub horizontal_alignment: Horizontal,
    pub vertical_alignment: Vertical,
}

impl Default for TextProps {
    fn default() -> Self {
        Self {
            content: String::new(),
            size: 16,
            color: Color::BLACK,
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Center,
        }
    }
}

impl Text {
    pub fn view(props: TextProps) -> Element<'static, Message> {
        text(props.content)
            .font(fonts::get_regular_font())
            .size(props.size)
            .style(props.color)
            .horizontal_alignment(props.horizontal_alignment)
            .vertical_alignment(props.vertical_alignment)
            .into()
    }

    pub fn title<'a>(content: String, color: Color) -> Element<'a, Message> {
        Self::view(TextProps {
            content,
            size: 24,
            color,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
        })
    }

    pub fn subtitle<'a>(content: String, color: Color) -> Element<'a, Message> {
        Self::view(TextProps {
            content,
            size: 20,
            color,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
        })
    }

    pub fn body<'a>(
        content: String,
        color: Color,
    ) -> Element<'a, Message> {
        text(content)
            .font(fonts::get_regular_font())
            .size(16)
            .style(color)
            .into()
    }
} 