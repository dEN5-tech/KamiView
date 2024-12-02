use iced::Color;

#[derive(Debug, Clone, Copy)]
pub enum ThemeVariant {
    Light,
    Dark,
}

impl Default for ThemeVariant {
    fn default() -> Self {
        Self::Dark
    }
}

pub(crate) trait ThemeColors {
    fn background(&self) -> Color;
    fn surface(&self) -> Color;
    fn surface_variant(&self) -> Color;
    fn primary(&self) -> Color;
    fn secondary(&self) -> Color;
    fn accent(&self) -> Color;
    fn text(&self) -> Color;
    fn text_secondary(&self) -> Color;
    fn text_on_primary(&self) -> Color;
    fn text_on_surface(&self) -> Color;
    fn error(&self) -> Color;
    fn success(&self) -> Color;
    fn warning(&self) -> Color;
    fn info(&self) -> Color;
    fn hover(&self) -> Color;
    fn active(&self) -> Color;
    fn disabled(&self) -> Color;
}

pub(crate) struct LightColors {
    pub background: Color,
    pub surface: Color,
    pub surface_variant: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub text: Color,
    pub text_secondary: Color,
    pub text_on_primary: Color,
    pub text_on_surface: Color,
    pub error: Color,
    pub success: Color,
    pub warning: Color,
    pub info: Color,
    pub hover: Color,
    pub active: Color,
    pub disabled: Color,
}

impl LightColors {
    pub fn new() -> Self {
        Self {
            background: Color::from_rgb(0.98, 0.98, 0.98),      // #FAFAFA
            surface: Color::WHITE,                              // #FFFFFF
            surface_variant: Color::from_rgb(0.95, 0.95, 0.95), // #F2F2F2
            primary: Color::from_rgb(0.0, 0.47, 1.0),          // #0078FF
            secondary: Color::from_rgb(0.0, 0.55, 0.55),       // #008C8C
            accent: Color::from_rgb(0.8, 0.3, 0.9),            // #CC4CE6
            text: Color::from_rgb(0.1, 0.1, 0.1),              // #1A1A1A
            text_secondary: Color::from_rgb(0.45, 0.45, 0.45),  // #737373
            text_on_primary: Color::WHITE,                      // #FFFFFF
            text_on_surface: Color::from_rgb(0.2, 0.2, 0.2),   // #333333
            error: Color::from_rgb(0.87, 0.13, 0.13),          // #DE2121
            success: Color::from_rgb(0.2, 0.7, 0.2),           // #33B333
            warning: Color::from_rgb(0.9, 0.6, 0.0),           // #E69900
            info: Color::from_rgb(0.0, 0.6, 0.9),              // #0099E6
            hover: Color::from_rgb(0.95, 0.95, 0.95),          // #F2F2F2
            active: Color::from_rgb(0.9, 0.9, 0.9),            // #E6E6E6
            disabled: Color::from_rgb(0.7, 0.7, 0.7),          // #B3B3B3
        }
    }
}

impl ThemeColors for LightColors {
    fn background(&self) -> Color { self.background }
    fn surface(&self) -> Color { self.surface }
    fn surface_variant(&self) -> Color { self.surface_variant }
    fn primary(&self) -> Color { self.primary }
    fn secondary(&self) -> Color { self.secondary }
    fn accent(&self) -> Color { self.accent }
    fn text(&self) -> Color { self.text }
    fn text_secondary(&self) -> Color { self.text_secondary }
    fn text_on_primary(&self) -> Color { self.text_on_primary }
    fn text_on_surface(&self) -> Color { self.text_on_surface }
    fn error(&self) -> Color { self.error }
    fn success(&self) -> Color { self.success }
    fn warning(&self) -> Color { self.warning }
    fn info(&self) -> Color { self.info }
    fn hover(&self) -> Color { self.hover }
    fn active(&self) -> Color { self.active }
    fn disabled(&self) -> Color { self.disabled }
}

pub(crate) struct DarkColors {
    pub background: Color,
    pub surface: Color,
    pub surface_variant: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub text: Color,
    pub text_secondary: Color,
    pub text_on_primary: Color,
    pub text_on_surface: Color,
    pub error: Color,
    pub success: Color,
    pub warning: Color,
    pub info: Color,
    pub hover: Color,
    pub active: Color,
    pub disabled: Color,
}

impl DarkColors {
    pub fn new() -> Self {
        Self {
            background: Color::from_rgb(0.133, 0.133, 0.133),      // #222222
            surface: Color::from_rgb(0.196, 0.196, 0.196),         // #323232
            surface_variant: Color::from_rgb(0.25, 0.25, 0.25),    // #404040
            primary: Color::from_rgb(0.4, 0.6, 1.0),              // #6699FF
            secondary: Color::from_rgb(0.4, 0.8, 0.8),            // #66CCCC
            accent: Color::from_rgb(0.9, 0.4, 1.0),               // #E666FF
            text: Color::from_rgb(0.9, 0.9, 0.9),                 // #E6E6E6
            text_secondary: Color::from_rgb(0.7, 0.7, 0.7),       // #B3B3B3
            text_on_primary: Color::BLACK,                         // #000000
            text_on_surface: Color::from_rgb(0.8, 0.8, 0.8),      // #CCCCCC
            error: Color::from_rgb(1.0, 0.4, 0.4),                // #FF6666
            success: Color::from_rgb(0.4, 0.9, 0.4),              // #66E666
            warning: Color::from_rgb(1.0, 0.7, 0.0),              // #FFB300
            info: Color::from_rgb(0.4, 0.7, 1.0),                 // #66B3FF
            hover: Color::from_rgb(0.25, 0.25, 0.25),             // #404040
            active: Color::from_rgb(0.3, 0.3, 0.3),               // #4D4D4D
            disabled: Color::from_rgb(0.4, 0.4, 0.4),             // #666666
        }
    }
}

impl ThemeColors for DarkColors {
    fn background(&self) -> Color { self.background }
    fn surface(&self) -> Color { self.surface }
    fn surface_variant(&self) -> Color { self.surface_variant }
    fn primary(&self) -> Color { self.primary }
    fn secondary(&self) -> Color { self.secondary }
    fn accent(&self) -> Color { self.accent }
    fn text(&self) -> Color { self.text }
    fn text_secondary(&self) -> Color { self.text_secondary }
    fn text_on_primary(&self) -> Color { self.text_on_primary }
    fn text_on_surface(&self) -> Color { self.text_on_surface }
    fn error(&self) -> Color { self.error }
    fn success(&self) -> Color { self.success }
    fn warning(&self) -> Color { self.warning }
    fn info(&self) -> Color { self.info }
    fn hover(&self) -> Color { self.hover }
    fn active(&self) -> Color { self.active }
    fn disabled(&self) -> Color { self.disabled }
} 