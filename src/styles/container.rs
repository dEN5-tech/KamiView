use iced::widget::container;
use iced::{Color, Theme};

pub struct CustomContainer {
    pub color: Color,
    pub text_color: Option<Color>,
}

impl container::StyleSheet for CustomContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(self.color.into()),
            text_color: self.text_color,
            ..Default::default()
        }
    }
} 