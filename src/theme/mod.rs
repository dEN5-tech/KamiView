mod colors;

pub use colors::*;

use iced::{application, widget::{button, container, text}, Color};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThemeVariant {
    Light,
    Dark,
}

impl Default for ThemeVariant {
    fn default() -> Self {
        Self::Dark
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Error,
    Text,
}

#[derive(Debug, Clone, Copy)]
pub enum ContainerVariant {
    Primary,
    Secondary,
    Box,
    Card,
    Badge,
    Transparent,
}

#[derive(Debug, Clone, Default)]
pub struct Theme {
    pub variant: ThemeVariant,
    pub background: Color,
    pub text: Color,
    pub text_secondary: Color,
    pub primary: Color,
    pub secondary: Color,
    pub error: Color,
    pub surface: Color,
}

impl Theme {
    pub fn with_variant(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::Dark => Self::dark(),
            ThemeVariant::Light => Self::light(),
        }
    }

    pub fn dark() -> Self {
        Self {
            variant: ThemeVariant::Dark,
            background: Color::from_rgb(0.07, 0.07, 0.07),
            text: Color::WHITE,
            text_secondary: Color::from_rgb(0.7, 0.7, 0.7),
            primary: Color::from_rgb(0.0, 0.59, 0.53),
            secondary: Color::from_rgb(0.41, 0.94, 0.68),
            error: Color::from_rgb(1.0, 0.33, 0.32),
            surface: Color::from_rgb(0.12, 0.12, 0.12),
        }
    }

    pub fn light() -> Self {
        Self {
            variant: ThemeVariant::Light,
            background: Color::WHITE,
            text: Color::from_rgb(0.07, 0.07, 0.07),
            text_secondary: Color::from_rgb(0.4, 0.4, 0.4),
            primary: Color::from_rgb(0.0, 0.59, 0.53),
            secondary: Color::from_rgb(0.0, 0.78, 0.33),
            error: Color::from_rgb(0.83, 0.18, 0.18),
            surface: Color::from_rgb(0.96, 0.96, 0.96),
        }
    }

    pub fn container(&self, variant: ContainerVariant) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(ContainerStyle {
            theme: self.clone(),
            variant,
        }))
    }

    pub fn button(&self, variant: ButtonVariant) -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(ButtonStyle {
            theme: self.clone(),
            variant,
        }))
    }

    pub fn text_color(&self) -> Color {
        self.text
    }

    pub fn error_color(&self) -> Color {
        self.error
    }

    pub fn primary_color(&self) -> Color {
        self.primary
    }
}

struct ContainerStyle {
    theme: Theme,
    variant: ContainerVariant,
}

impl container::StyleSheet for ContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        match self.variant {
            ContainerVariant::Primary => container::Appearance {
                text_color: Some(self.theme.text),
                background: Some(self.theme.background.into()),
                border_radius: 8.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            ContainerVariant::Secondary => container::Appearance {
                text_color: Some(self.theme.text),
                background: Some(Color::from_rgb(0.15, 0.15, 0.15).into()),
                border_radius: 6.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            ContainerVariant::Box => container::Appearance {
                text_color: Some(self.theme.text),
                background: Some(Color::from_rgb(0.18, 0.18, 0.18).into()),
                border_radius: 12.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.3, 0.3, 0.3),
            },
            ContainerVariant::Card => container::Appearance {
                text_color: Some(self.theme.text),
                background: Some(self.theme.surface.into()),
                border_radius: 8.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            ContainerVariant::Badge => container::Appearance {
                text_color: Some(self.theme.text),
                background: Some(self.theme.primary.into()),
                border_radius: 12.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            ContainerVariant::Transparent => container::Appearance {
                text_color: Some(self.theme.text),
                background: None,
                border_radius: 0.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }
}

struct ButtonStyle {
    theme: Theme,
    variant: ButtonVariant,
}

impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        match self.variant {
            ButtonVariant::Primary => button::Appearance {
                background: Some(self.theme.primary.into()),
                text_color: self.theme.text,
                border_radius: 4.0.into(),
                ..Default::default()
            },
            ButtonVariant::Secondary => button::Appearance {
                background: Some(self.theme.secondary.into()),
                text_color: self.theme.text,
                border_radius: 4.0.into(),
                ..Default::default()
            },
            ButtonVariant::Error => button::Appearance {
                background: Some(self.theme.error.into()),
                text_color: self.theme.text,
                border_radius: 4.0.into(),
                ..Default::default()
            },
            ButtonVariant::Text => button::Appearance {
                background: None,
                text_color: self.theme.text,
                border_radius: 0.0.into(),
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        let hover_color = match self.variant {
            ButtonVariant::Primary => self.theme.primary,
            ButtonVariant::Secondary => self.theme.secondary,
            ButtonVariant::Error => self.theme.error,
            ButtonVariant::Text => return active,
        };

        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgba(
                hover_color.r * 0.9,
                hover_color.g * 0.9,
                hover_color.b * 0.9,
                hover_color.a,
            ))),
            ..active
        }
    }
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.background,
            text_color: self.text,
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = iced::theme::Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            iced::theme::Text::Default => text::Appearance {
                color: Some(self.text),
            },
            iced::theme::Text::Color(color) => text::Appearance {
                color: Some(color),
            },
        }
    }
}