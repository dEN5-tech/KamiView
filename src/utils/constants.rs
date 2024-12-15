use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

pub struct Config {
    pub shikimori_client_id: String,
    pub shikimori_client_secret: String,
    pub mpv_socket_path: &'static str,
}

impl Config {
    fn new() -> Self {
        Self {
            shikimori_client_id: env::var("SHIKIMORI_CLIENT_ID")
                .expect("SHIKIMORI_CLIENT_ID must be set"),
            shikimori_client_secret: env::var("SHIKIMORI_CLIENT_SECRET")
                .expect("SHIKIMORI_CLIENT_SECRET must be set"),
            mpv_socket_path: if cfg!(target_os = "windows") {
                r"\\.\pipe\mpv-socket"
            } else {
                "/tmp/mpv-socket"
            },
        }
    }
}