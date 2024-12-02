use iced::Font;
use once_cell::sync::OnceCell;

static KANIT_REGULAR_FONT: OnceCell<Font> = OnceCell::new();
static KANIT_MEDIUM_FONT: OnceCell<Font> = OnceCell::new();
static KANIT_BOLD_FONT: OnceCell<Font> = OnceCell::new();

pub fn init_fonts() {
    let regular = Font::with_name("Kanit-Regular");
    let medium = Font::with_name("Kanit-Medium");
    let bold = Font::with_name("Kanit-Bold");

    KANIT_REGULAR_FONT.set(regular).ok();
    KANIT_MEDIUM_FONT.set(medium).ok();
    KANIT_BOLD_FONT.set(bold).ok();
}

pub fn get_regular_font() -> Font {
    KANIT_REGULAR_FONT.get().copied().unwrap_or(Font::DEFAULT)
}

pub fn get_medium_font() -> Font {
    KANIT_MEDIUM_FONT.get().copied().unwrap_or(Font::DEFAULT)
}

pub fn get_bold_font() -> Font {
    KANIT_BOLD_FONT.get().copied().unwrap_or(Font::DEFAULT)
}

// Font data to be embedded in the binary
pub const KANIT_REGULAR_BYTES: &[u8] = include_bytes!("../resources/fonts/Kanit-Regular.ttf");
pub const KANIT_MEDIUM_BYTES: &[u8] = include_bytes!("../resources/fonts/Kanit-Medium.ttf");
pub const KANIT_BOLD_BYTES: &[u8] = include_bytes!("../resources/fonts/Kanit-Bold.ttf");
 