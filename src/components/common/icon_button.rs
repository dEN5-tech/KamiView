use iced::widget::{button, row, svg};
use iced::{Element, Length};
use iced::theme;
use iced::alignment::{Horizontal, Vertical};

use crate::styles::CustomButton;
use crate::theme::Theme;
use crate::Message;
use crate::resources;
use super::{Text, TextProps};

pub struct IconButton;

#[derive(Debug, Clone)]
pub struct IconButtonProps<'a> {
    pub icon_path: &'a str,
    pub label: &'a str,
    pub on_press: Message,
    pub is_active: bool,
    pub width: Length,
    pub padding: u16,
    pub size: u16,
}

impl IconButton {
    pub fn view<'a>(props: IconButtonProps<'a>, theme: &Theme) -> Element<'a, Message> {
        let IconButtonProps {
            icon_path,
            label,
            on_press,
            is_active,
            width,
            padding,
            size,
        } = props;

        let icon_data = resources::get_svg(icon_path)
            .expect("SVG resource not found");
            
        let icon = svg::Handle::from_memory(icon_data.as_bytes().to_vec());

        button(
            row![
                svg(icon).width(Length::Fixed(24.0)),
                Text::view(TextProps {
                    content: label.to_string(),
                    size,
                    color: if is_active { theme.surface } else { theme.text },
                    horizontal_alignment: Horizontal::Left,
                    vertical_alignment: Vertical::Center,
                })
            ].spacing(10)
        )
        .width(width)
        .padding(padding)
        .style(theme::Button::Custom(Box::new(CustomButton {
            color: if is_active { theme.primary } else { theme.surface },
        })))
        .on_press(on_press)
        .into()
    }
} 