pub mod client;
mod storage;

pub use client::{
    ShikimoriOAuth, 
    UserInfo, 
    TokenResponse,
    ShikimoriClientParameters as ShikimoriClientConfig
};
pub use storage::{Storage, AuthTokens};

use shaku::Component;
use crate::di::interfaces::{IShikimoriClient, IShikimoriOAuth};
use anyhow::Result;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use reqwest::Client;

#[derive(Debug)]
pub struct ShikimoriOAuthParameters {
    pub oauth: Arc<ShikimoriOAuth>
}

impl Default for ShikimoriOAuthParameters {
    fn default() -> Self {
        Self {
            oauth: Arc::new(ShikimoriOAuth::new().expect("Failed to create ShikimoriOAuth"))
        }
    }
}

#[derive(Component)]
#[shaku(interface = IShikimoriOAuth)]
pub struct ShikimoriOAuthComponent {
    #[shaku(default = Arc::new(Client::new()))]
    client: Arc<Client>,
    #[shaku(default = Arc::new(ShikimoriOAuth::new().expect("Failed to create ShikimoriOAuth")))]
    oauth: Arc<ShikimoriOAuth>
}

impl IShikimoriOAuth for ShikimoriOAuthComponent {
    fn get_user_info(&self) -> Pin<Box<dyn Future<Output = Result<UserInfo>> + Send>> {
        let oauth = self.oauth.clone();
        Box::pin(async move {
            oauth.get_user_info().await
        })
    }

    fn logout(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        let oauth = self.oauth.clone();
        Box::pin(async move {
            oauth.logout().await
        })
    }

    fn exchange_code(&self, code: &str) -> Pin<Box<dyn Future<Output = Result<TokenResponse>> + Send>> {
        let oauth = self.oauth.clone();
        let code = code.to_string();
        Box::pin(async move {
            oauth.exchange_code(&code).await
        })
    }

    fn get_auth_url(&self) -> Result<String> {
        Ok(self.oauth.get_auth_url())
    }
}

#[derive(Component)]
#[shaku(interface = IShikimoriClient)]
pub struct ShikimoriClient {
    #[shaku(default = String::from("https://shikimori.one/api"))]
    base_url: String,
    #[shaku(default = String::new())]
    client_id: String,
    #[shaku(default = String::new())]
    client_secret: String,
    #[shaku(inject)]
    oauth: Arc<dyn IShikimoriOAuth>,
}

impl IShikimoriClient for ShikimoriClient {
    fn get_user_info(&self) -> Pin<Box<dyn Future<Output = Result<UserInfo>> + Send>> {
        let oauth = self.oauth.clone();
        Box::pin(async move {
            oauth.get_user_info().await
        })
    }

    fn logout(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        let oauth = self.oauth.clone();
        Box::pin(async move {
            oauth.logout().await
        })
    }

    fn exchange_code(&self, code: &str) -> Pin<Box<dyn Future<Output = Result<TokenResponse>> + Send>> {
        let oauth = self.oauth.clone();
        let code = code.to_string();
        Box::pin(async move {
            oauth.exchange_code(&code).await
        })
    }

    fn get_auth_url(&self) -> Result<String> {
        self.oauth.get_auth_url()
    }
}
