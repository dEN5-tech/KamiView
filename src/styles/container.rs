use iced::widget::container;
use iced::{Color, Theme, Background};

#[derive(Debug, Clone)]
pub enum ContainerStyle {
    Primary,
    Secondary,
    Card,
    Transparent,
    Custom(CustomContainer),
}

#[derive(Debug, Clone)]
pub struct CustomContainer {
    pub background: Option<Background>,
    pub text_color: Option<Color>,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Option<Color>,
}

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, theme: &Self::Style) -> container::Appearance {
        match self {
            ContainerStyle::Primary => container::Appearance {
                background: Some(Color::from_rgb(0.2, 0.2, 0.2).into()),
                text_color: Some(Color::WHITE),
                border_radius: 8.0,
                border_width: 0.0,
                border_color: None,
            },
            ContainerStyle::Secondary => container::Appearance {
                background: Some(Color::from_rgb(0.15, 0.15, 0.15).into()),
                text_color: Some(Color::WHITE),
                border_radius: 6.0,
                border_width: 0.0,
                border_color: None,
            },
            ContainerStyle::Card => container::Appearance {
                background: Some(Color::from_rgb(0.18, 0.18, 0.18).into()),
                text_color: Some(Color::WHITE),
                border_radius: 12.0,
                border_width: 1.0,
                border_color: Some(Color::from_rgb(0.3, 0.3, 0.3)),
            },
            ContainerStyle::Transparent => container::Appearance {
                background: None,
                text_color: Some(Color::WHITE),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: None,
            },
            ContainerStyle::Custom(custom) => container::Appearance {
                background: custom.background,
                text_color: custom.text_color,
                border_radius: custom.border_radius,
                border_width: custom.border_width,
                border_color: custom.border_color,
            },
        }
    }
} 