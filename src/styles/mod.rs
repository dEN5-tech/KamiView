use iced::{Color, Theme};
use iced::widget::{container, button, text_input};

pub struct CustomContainer {
    pub color: Color,
    pub text_color: Option<Color>,
    pub hover_color: Option<Color>,
}

impl container::StyleSheet for CustomContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: self.text_color,
            background: Some(self.color.into()),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: self.hover_color.unwrap_or(self.color),
        }
    }
}

pub struct CustomButton {
    pub color: Color,
}

impl button::StyleSheet for CustomButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(self.color.into()),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: self.color,
            text_color: Color::WHITE,
        }
    }
}

pub struct CustomTextInput {
    pub color: Color,
}

impl text_input::StyleSheet for CustomTextInput {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: self.color.into(),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: self.color,
            icon_color: Color::WHITE,
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        self.active(_style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.7, 0.7, 0.7)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::WHITE
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.3, 0.3, 0.3)
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let active = self.active(style);
        text_input::Appearance {
            background: Color::from_rgb(0.2, 0.2, 0.2).into(),
            ..active
        }
    }
}

impl Default for CustomContainer {
    fn default() -> Self {
        Self {
            color: Color::TRANSPARENT,
            text_color: None,
            hover_color: None,
        }
    }
}

impl Default for CustomButton {
    fn default() -> Self {
        Self {
            color: Color::TRANSPARENT,
        }
    }
}

impl Default for CustomTextInput {
    fn default() -> Self {
        Self {
            color: Color::TRANSPARENT,
        }
    }
} 