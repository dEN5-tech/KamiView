use iced::widget::text_input;
use iced::{Color, Theme, BorderRadius};

pub struct CustomTextInput {
    pub color: Color,
    pub text_color: Color,
}

impl text_input::StyleSheet for CustomTextInput {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: self.color.into(),
            border_radius: BorderRadius::from(4.0),
            border_width: 1.0,
            border_color: self.color,
            icon_color: self.text_color,
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: self.color.into(),
            border_radius: BorderRadius::from(4.0),
            border_width: 2.0,
            border_color: self.text_color,
            icon_color: self.text_color,
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color { a: 0.5, ..self.text_color }
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.text_color
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color { a: 0.2, ..self.text_color }
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color { a: 0.5, ..self.text_color }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let active = self.active(style);
        text_input::Appearance {
            background: Color { a: 0.5, ..self.color }.into(),
            ..active
        }
    }
} 