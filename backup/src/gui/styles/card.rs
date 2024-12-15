use iced::widget::{button, container};
use iced::{Color, theme};
use super::constants::*;

#[derive(Debug, Clone, Copy)]
pub struct CardStyle {
    pub is_hoverable: bool,
    pub has_shadow: bool,
    pub is_glassmorphic: bool,
}

impl Default for CardStyle {
    fn default() -> Self {
        Self {
            is_hoverable: false,
            has_shadow: false,
            is_glassmorphic: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CardContainerStyle {
    pub is_glassmorphic: bool,
}

impl Default for CardContainerStyle {
    fn default() -> Self {
        Self {
            is_glassmorphic: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlaceholderStyle;

impl container::StyleSheet for CardContainerStyle {
    type Style = theme::Theme;

    fn appearance(&self, theme: &Self::Style) -> container::Appearance {
        let is_dark = matches!(theme, theme::Theme::Dark);
        container::Appearance {
            background: Some(if self.is_glassmorphic {
                if is_dark {
                    Color { a: 0.2, ..BACKGROUND_COLOR }.into()
                } else {
                    Color { a: 0.1, ..Color::from_rgb(1.0, 1.0, 1.0) }.into()
                }
            } else if is_dark {
                BACKGROUND_COLOR.into()
            } else {
                Color::from_rgb(1.0, 1.0, 1.0).into()
            }),
            border_radius: 8.0.into(),
            border_width: if self.is_glassmorphic { 0.5 } else { 1.0 },
            border_color: if is_dark {
                Color { a: 0.3, ..Color::from_rgb(0.3, 0.3, 0.3) }
            } else {
                Color { a: 0.2, ..Color::from_rgb(0.9, 0.9, 0.9) }
            },
            text_color: None,
        }
    }
}

impl button::StyleSheet for CardStyle {
    type Style = theme::Theme;

    fn active(&self, theme: &Self::Style) -> button::Appearance {
        let is_dark = matches!(theme, theme::Theme::Dark);
        button::Appearance {
            background: Some(if self.is_glassmorphic {
                if is_dark {
                    Color { a: 0.2, ..BACKGROUND_COLOR }.into()
                } else {
                    Color { a: 0.1, ..Color::from_rgb(1.0, 1.0, 1.0) }.into()
                }
            } else if is_dark {
                BACKGROUND_COLOR.into()
            } else {
                Color::from_rgb(1.0, 1.0, 1.0).into()
            }),
            border_radius: 8.0.into(),
            border_width: if self.is_glassmorphic { 0.5 } else { 0.0 },
            border_color: if self.is_glassmorphic {
                if is_dark {
                    Color { a: 0.3, ..Color::from_rgb(0.3, 0.3, 0.3) }
                } else {
                    Color { a: 0.2, ..Color::from_rgb(0.9, 0.9, 0.9) }
                }
            } else {
                Color::TRANSPARENT
            },
            text_color: if is_dark { TEXT_COLOR } else { Color::BLACK },
            shadow_offset: if self.has_shadow {
                iced::Vector::new(0.0, 2.0)
            } else {
                iced::Vector::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let mut active = self.active(style);
        if self.is_hoverable {
            active.shadow_offset = iced::Vector::new(0.0, 4.0);
            active.background = Some(if self.is_glassmorphic {
                if matches!(style, theme::Theme::Dark) {
                    Color { a: 0.3, ..BACKGROUND_COLOR }.into()
                } else {
                    Color { a: 0.2, ..Color::from_rgb(0.95, 0.95, 0.95) }.into()
                }
            } else if matches!(style, theme::Theme::Dark) {
                Color { a: 0.8, ..BACKGROUND_COLOR }.into()
            } else {
                Color::from_rgb(0.95, 0.95, 0.95).into()
            });
        }
        active
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let mut active = self.active(style);
        if self.is_hoverable {
            active.shadow_offset = iced::Vector::new(0.0, 1.0);
            active.background = Some(if self.is_glassmorphic {
                if matches!(style, theme::Theme::Dark) {
                    Color { a: 0.4, ..BACKGROUND_COLOR }.into()
                } else {
                    Color { a: 0.3, ..Color::from_rgb(0.9, 0.9, 0.9) }.into()
                }
            } else if matches!(style, theme::Theme::Dark) {
                Color { a: 0.7, ..BACKGROUND_COLOR }.into()
            } else {
                Color::from_rgb(0.9, 0.9, 0.9).into()
            });
        }
        active
    }
}

impl container::StyleSheet for PlaceholderStyle {
    type Style = theme::Theme;

    fn appearance(&self, theme: &Self::Style) -> container::Appearance {
        let is_dark = matches!(theme, theme::Theme::Dark);
        container::Appearance {
            background: Some(if is_dark {
                Color::from_rgb(0.25, 0.25, 0.25).into()
            } else {
                Color::from_rgb(0.95, 0.95, 0.95).into()
            }),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: if is_dark {
                Color::from_rgb(0.3, 0.3, 0.3)
            } else {
                Color::from_rgb(0.9, 0.9, 0.9)
            },
            text_color: None,
        }
    }
}
