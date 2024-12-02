use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub kodik_token: String,
    pub shikimori_client_id: String,
    pub shikimori_client_secret: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::load()
});

impl Config {
    pub fn load() -> Self {
        dotenv::dotenv().ok();
        
        Self {
            kodik_token: std::env::var("KODIK_TOKEN")
                .expect("KODIK_TOKEN must be set"),
            shikimori_client_id: std::env::var("SHIKIMORI_CLIENT_ID")
                .expect("SHIKIMORI_CLIENT_ID must be set"),
            shikimori_client_secret: std::env::var("SHIKIMORI_CLIENT_SECRET")
                .expect("SHIKIMORI_CLIENT_SECRET must be set"),
        }
    }
} 