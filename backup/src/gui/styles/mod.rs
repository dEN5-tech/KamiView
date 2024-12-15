pub mod button;
pub mod container;
pub mod theme;
pub mod card;

pub use self::button::ButtonStyle;
pub use self::container::ContainerStyle;
pub use self::theme::*;
pub use self::card::{CardStyle, CardContainerStyle, PlaceholderStyle};

// Common style constants
pub mod constants {
    use iced::Color;

    // Telegram-like dark theme colors
    pub const PRIMARY_COLOR: Color = Color::from_rgb(
        0x2E as f32 / 255.0,
        0xA6 as f32 / 255.0,
        0xDD as f32 / 255.0,
    );

    pub const SECONDARY_COLOR: Color = Color::from_rgb(
        0x45 as f32 / 255.0,
        0x45 as f32 / 255.0,
        0x45 as f32 / 255.0,
    );

    pub const BACKGROUND_COLOR: Color = Color::from_rgb(
        0x1F as f32 / 255.0,
        0x1F as f32 / 255.0,
        0x1F as f32 / 255.0,
    );

    pub const TEXT_COLOR: Color = Color::from_rgb(
        0xDD as f32 / 255.0,
        0xDD as f32 / 255.0,
        0xDD as f32 / 255.0,
    );

    pub const BORDER_RADIUS: f32 = 8.0;
    pub const BORDER_WIDTH: f32 = 0.0;
}
