use shaku::Interface;
use anyhow::Result;
use serde::Serialize;
use crate::kodik::{MediaResult, InfoResponse, Translation};
use std::future::Future;
use std::pin::Pin;
use crate::storage::AppSettings;
use reqwest::Response;
use crate::shikimori::{UserInfo, TokenResponse};

pub trait IKodikSearch: Interface {
    fn search_anime<'a>(&'a self, query: &'a str) -> Pin<Box<dyn Future<Output = Result<Vec<MediaResult>>> + Send + 'a>>;
}

pub trait IKodikInfo: Interface {
    fn get_anime_info<'a>(&'a self, shikimori_id: &'a str) -> Pin<Box<dyn Future<Output = Result<InfoResponse>> + Send + 'a>>;
    fn get_translations<'a>(&'a self, shikimori_id: &'a str) -> Pin<Box<dyn Future<Output = Result<Vec<Translation>>> + Send + 'a>>;
    fn get_series_count<'a>(&'a self, shikimori_id: &'a str) -> Pin<Box<dyn Future<Output = Result<i32>> + Send + 'a>>;
}

pub trait IKodikPlayback: Interface {
    fn get_episode_link<'a>(&'a self, shikimori_id: &'a str, episode: i32, translation_id: &'a str) -> Pin<Box<dyn Future<Output = Result<(String, i32)>> + Send + 'a>>;
    fn create_playlist<'a>(&'a self, title: &'a str, shikimori_id: &'a str, translation_id: &'a str) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>>;
}

pub trait IKodik: IKodikSearch + IKodikInfo + IKodikPlayback + Interface {}

pub trait IShikimoriOAuth: Interface {
    fn get_user_info(&self) -> Pin<Box<dyn Future<Output = Result<UserInfo>> + Send>>;
    fn logout(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;
    fn exchange_code(&self, code: &str) -> Pin<Box<dyn Future<Output = Result<TokenResponse>> + Send>>;
    fn get_auth_url(&self) -> Result<String>;
}

pub trait IShikimoriClient: Interface {
    fn get_user_info(&self) -> Pin<Box<dyn Future<Output = Result<UserInfo>> + Send>>;
    fn logout(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;
    fn exchange_code(&self, code: &str) -> Pin<Box<dyn Future<Output = Result<TokenResponse>> + Send>>;
    fn get_auth_url(&self) -> Result<String>;
}

pub trait IMpvClient: Interface {
    fn play(&self, url: &str) -> anyhow::Result<()>;
    fn get_playback_info(&self) -> anyhow::Result<PlaybackInfo>;
}

#[derive(Debug, Clone, Serialize)]
pub struct PlaybackInfo {
    pub position: f64,
    pub duration: f64,
    pub paused: bool,
}

pub trait IStorage: Interface {
    fn load(&self) -> AppSettings;
    fn save(&self, settings: &AppSettings) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait IReqwestClient: Interface {
    fn get<'a>(&'a self, url: &'a str) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>>;
    fn get_bytes<'a>(&'a self, url: &'a str) -> Pin<Box<dyn Future<Output = Result<Vec<u8>>> + Send + 'a>>;
    fn post<'a>(&'a self, url: &'a str, body: &'a str) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>>;
}

pub trait ShikimoriClient {
    fn get_auth_url(&self) -> Result<String, anyhow::Error>;
    // ... other methods
}