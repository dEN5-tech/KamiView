use iced::widget::container;
use iced::{BorderRadius, Color, Theme};
use super::constants::*;

#[derive(Debug, Clone, Copy)]
pub enum ContainerStyle {
    Primary,
    Box,
}

impl From<ContainerStyle> for iced::theme::Container {
    fn from(style: ContainerStyle) -> Self {
        match style {
            ContainerStyle::Primary => iced::theme::Container::Custom(std::boxed::Box::new(Primary)),
            ContainerStyle::Box => iced::theme::Container::Custom(std::boxed::Box::new(BoxStyle)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Primary;
#[derive(Debug, Clone, Copy)]
struct BoxStyle;

impl container::StyleSheet for Primary {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(TEXT_COLOR),
            background: Some(BACKGROUND_COLOR.into()),
            border_radius: BorderRadius::from(BORDER_RADIUS),
            border_width: BORDER_WIDTH,
            border_color: Color::TRANSPARENT,
        }
    }
}

impl container::StyleSheet for BoxStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(TEXT_COLOR),
            background: Some(Color {
                a: 0.5,
                ..SECONDARY_COLOR
            }.into()),
            border_radius: BorderRadius::from(BORDER_RADIUS),
            border_width: BORDER_WIDTH,
            border_color: Color::TRANSPARENT,
        }
    }
} 