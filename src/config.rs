use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub kodik_token: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    
    Config {
        kodik_token: env::var("KODIK_TOKEN")
            .expect("KODIK_TOKEN must be set in environment"),
    }
}); 