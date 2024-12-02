mod colors;
mod components;

pub use colors::*;
pub use components::*;

use iced::Color;
use iced::widget::pick_list;
use iced::widget::text_input;

pub struct Theme {
    // Base colors
    pub background: Color,
    pub surface: Color,
    pub surface_variant: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,

    // Text colors
    pub text: Color,
    pub text_secondary: Color,
    pub text_on_primary: Color,
    pub text_on_surface: Color,

    // State colors
    pub error: Color,
    pub success: Color,
    pub warning: Color,
    pub info: Color,

    // Interactive colors
    pub hover: Color,
    pub active: Color,
    pub disabled: Color,
}

impl Theme {
    pub fn new(variant: ThemeVariant) -> Self {
        let colors: Box<dyn ThemeColors> = match variant {
            ThemeVariant::Light => Box::new(LightColors::new()),
            ThemeVariant::Dark => Box::new(DarkColors::new()),
        };

        Self {
            background: colors.background(),
            surface: colors.surface(),
            surface_variant: colors.surface_variant(),
            primary: colors.primary(),
            secondary: colors.secondary(),
            accent: colors.accent(),
            text: colors.text(),
            text_secondary: colors.text_secondary(),
            text_on_primary: colors.text_on_primary(),
            text_on_surface: colors.text_on_surface(),
            error: colors.error(),
            success: colors.success(),
            warning: colors.warning(),
            info: colors.info(),
            hover: colors.hover(),
            active: colors.active(),
            disabled: colors.disabled(),
        }
    }

    pub fn button(&self, is_primary: bool, is_active: bool) -> ButtonStyle {
        if is_primary {
            ButtonStyle {
                background: self.primary,
                text: self.text_on_primary,
                border: None,
                shadow: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.1)),
            }
        } else {
            ButtonStyle {
                background: if is_active { self.active } else { self.surface },
                text: self.text,
                border: Some(self.surface_variant),
                shadow: None,
            }
        }
    }

    pub fn card(&self) -> CardStyle {
        CardStyle {
            background: self.surface,
            border: Some(self.surface_variant),
            shadow: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.1)),
        }
    }

    pub fn badge(&self) -> BadgeStyle {
        BadgeStyle {
            background: self.surface_variant,
            text: self.text_secondary,
            border: None,
        }
    }

    pub fn error_button(&self) -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(ButtonStyle {
            background: self.error,
            text: Color::WHITE,
            border: None,
            shadow: None,
        }))
    }

    pub fn transparent(&self) -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(ButtonStyle {
            background: Color::TRANSPARENT,
            text: self.text,
            border: None,
            shadow: None,
        }))
    }

    pub fn pick_list_bold(&self) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: self.text,
            placeholder_color: self.text_secondary,
            handle_color: self.primary,
            background: self.background.into(),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: self.primary,
        }
    }
}

impl pick_list::StyleSheet for Theme {
    type Style = iced::theme::PickList;

    fn active(&self, _style: &Self::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: self.text,
            placeholder_color: self.text_secondary,
            handle_color: self.primary,
            background: self.background.into(),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: self.primary,
        }
    }

    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        let mut appearance = self.active(style);
        appearance.border_color = self.hover;
        appearance
    }
}

impl text_input::StyleSheet for Theme {
    type Style = iced::theme::TextInput;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: self.surface.into(),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: self.surface_variant,
            icon_color: self.text_secondary,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let mut appearance = self.active(style);
        appearance.border_color = self.primary;
        appearance
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.text_secondary
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.text
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color { a: 0.3, ..self.primary }
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        self.disabled
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let mut appearance = self.active(style);
        appearance.background = self.disabled.into();
        appearance
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new(ThemeVariant::Dark)
    }
}