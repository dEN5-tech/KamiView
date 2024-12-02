use iced::Color;
use iced::widget::{button, container};
use iced::widget::button::Appearance as ButtonAppearance;
use iced::widget::container::Appearance as ContainerAppearance;
use iced::Theme;
use iced::theme;

#[derive(Clone)]
pub struct ButtonStyle {
    pub background: Color,
    pub text: Color,
    pub border: Option<Color>,
    pub shadow: Option<Color>,
}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> ButtonAppearance {
        ButtonAppearance {
            shadow_offset: Default::default(),
            background: Some(self.background.into()),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: self.border.unwrap_or(self.background),
            text_color: self.text,
        }
    }
}

#[derive(Clone)]
pub struct CardStyle {
    pub background: Color,
    pub border: Option<Color>,
    pub shadow: Option<Color>,
}

impl container::StyleSheet for CardStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> ContainerAppearance {
        ContainerAppearance {
            text_color: None,
            background: Some(self.background.into()),
            border_radius: 8.0.into(),
            border_width: 1.0,
            border_color: self.border.unwrap_or(self.background),
        }
    }
}

#[derive(Clone)]
pub struct BadgeStyle {
    pub background: Color,
    pub text: Color,
    pub border: Option<Color>,
}

impl container::StyleSheet for BadgeStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> ContainerAppearance {
        ContainerAppearance {
            text_color: Some(self.text),
            background: Some(self.background.into()),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: self.border.unwrap_or(self.background),
        }
    }
}

impl From<ButtonStyle> for iced::theme::Button {
    fn from(style: ButtonStyle) -> Self {
        iced::theme::Button::Custom(Box::new(style))
    }
}

impl From<CardStyle> for iced::theme::Container {
    fn from(style: CardStyle) -> Self {
        iced::theme::Container::Custom(Box::new(style))
    }
}

impl From<BadgeStyle> for iced::theme::Container {
    fn from(style: BadgeStyle) -> Self {
        iced::theme::Container::Custom(Box::new(style))
    }
}

#[derive(Debug, Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Text,
}

impl From<ButtonVariant> for theme::Button {
    fn from(variant: ButtonVariant) -> Self {
        match variant {
            ButtonVariant::Primary => theme::Button::Primary,
            ButtonVariant::Secondary => theme::Button::Secondary,
            ButtonVariant::Text => theme::Button::Text,
        }
    }
} 