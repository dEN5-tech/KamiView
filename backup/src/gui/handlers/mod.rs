pub mod subscription_handler;
pub mod update_handler;
pub mod navigation;

mod search;
mod theme;
mod playback;
mod settings;
mod anime_details;

pub(crate) use {
    search::*,
    theme::*,
    playback::*,
    navigation::handle_navigation,
    settings::*,
}; 