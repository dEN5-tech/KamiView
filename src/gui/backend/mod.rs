mod webview;
mod ipc;
mod event_loop;
mod server;
mod handlers;
mod types;
mod scripts;

pub use webview::*;
pub use ipc::*;
pub use event_loop::*;
pub use handlers::*;
pub use types::*;
pub use crate::utils::routes::*;

// Re-export specific handlers
pub use handlers::{
    handle_search,
    handle_anime_selected,
    handle_play_episode,
    handle_get_playback_info,
    handle_toggle_playback,
    handle_stop_playback,
    handle_start_download,
    handle_exchange_code,
    handle_get_user_info,
    handle_logout,
    handle_open_auth_url,
};