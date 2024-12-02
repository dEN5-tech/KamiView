use iced::widget::button;
use iced::{Color, Theme};

pub struct CustomButton {
    pub color: Color,
}

impl button::StyleSheet for CustomButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(self.color.into()),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
} 