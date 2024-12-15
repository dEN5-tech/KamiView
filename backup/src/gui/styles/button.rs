use iced::widget::button;
use iced::{BorderRadius, Color, Theme};
use super::constants::*;

#[derive(Debug, Clone, Copy)]
pub enum ButtonStyle {
    Primary,
    Secondary,
}

impl From<ButtonStyle> for iced::theme::Button {
    fn from(style: ButtonStyle) -> Self {
        match style {
            ButtonStyle::Primary => iced::theme::Button::Custom(std::boxed::Box::new(Primary)),
            ButtonStyle::Secondary => iced::theme::Button::Custom(std::boxed::Box::new(Secondary)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Primary;
#[derive(Debug, Clone, Copy)]
struct Secondary;

impl button::StyleSheet for Primary {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(PRIMARY_COLOR.into()),
            text_color: TEXT_COLOR,
            border_radius: BorderRadius::from(BORDER_RADIUS),
            border_width: BORDER_WIDTH,
            border_color: Color::TRANSPARENT,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        let active = self.active(_style);
        button::Appearance {
            background: Some(Color {
                a: 0.8,
                ..PRIMARY_COLOR
            }.into()),
            ..active
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        let active = self.active(_style);
        button::Appearance {
            background: Some(Color {
                a: 0.7,
                ..PRIMARY_COLOR
            }.into()),
            ..active
        }
    }
}

impl button::StyleSheet for Secondary {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(SECONDARY_COLOR.into()),
            text_color: TEXT_COLOR,
            border_radius: BorderRadius::from(BORDER_RADIUS),
            border_width: BORDER_WIDTH,
            border_color: Color::TRANSPARENT,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        let active = self.active(_style);
        button::Appearance {
            background: Some(Color {
                a: 0.8,
                ..SECONDARY_COLOR
            }.into()),
            ..active
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        let active = self.active(_style);
        button::Appearance {
            background: Some(Color {
                a: 0.7,
                ..SECONDARY_COLOR
            }.into()),
            ..active
        }
    }
} 