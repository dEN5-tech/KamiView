use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::config::CONFIG;
use tokio::sync::RwLock;
use std::sync::Arc;
use super::storage::{StorageService, AuthTokens};

const AUTH_URL: &str = "https://shikimori.one/oauth/authorize";
const TOKEN_URL: &str = "https://shikimori.one/oauth/token";
const API_URL: &str = "https://shikimori.one/api";
const USER_AGENT: &str = "mpv-integrade";
const REDIRECT_URI: &str = "urn:ietf:wg:oauth:2.0:oob";
const TOKEN_STORAGE_KEY: &str = "shikimori_token";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
    pub created_at: u64,
}

impl From<TokenResponse> for AuthTokens {
    fn from(token: TokenResponse) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            expires_at: now + token.expires_in,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i64,
    pub nickname: String,
    pub avatar: String,
    pub last_online_at: String,
}

pub struct ShikimoriOAuth {
    client: Arc<Client>,
    tokens: Arc<RwLock<Option<TokenResponse>>>,
    storage: StorageService,
}

impl ShikimoriOAuth {
    pub fn new() -> Result<Self> {
        log::info!("Initializing ShikimoriOAuth service");
        let storage = StorageService::new()?;
        
        // Try to load saved tokens
        let tokens = if let Some(auth_tokens) = storage.load_auth_tokens()? {
            log::info!("Loaded saved token");
            Some(TokenResponse {
                access_token: auth_tokens.access_token,
                refresh_token: auth_tokens.refresh_token,
                expires_in: auth_tokens.expires_at.saturating_sub(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                ),
                token_type: "Bearer".to_string(),
                scope: "user_rates".to_string(),
                created_at: auth_tokens.expires_at - 3600, // Approximate creation time
            })
        } else {
            None
        };

        Ok(Self {
            client: Arc::new(Client::new()),
            tokens: Arc::new(RwLock::new(tokens)),
            storage,
        })
    }

    pub fn get_auth_url(&self) -> String {
        log::debug!("Generating auth URL");
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope=user_rates",
            AUTH_URL,
            CONFIG.shikimori_client_id,
            REDIRECT_URI
        )
    }

    pub async fn exchange_code(&self, code: &str) -> Result<TokenResponse> {
        log::info!("Exchanging auth code for token");
        log::debug!("Auth code: {}", code);

        let response = self.client
            .post(TOKEN_URL)
            .header("User-Agent", USER_AGENT)
            .form(&[
                ("grant_type", "authorization_code"),
                ("client_id", &CONFIG.shikimori_client_id),
                ("client_secret", &CONFIG.shikimori_client_secret),
                ("code", code),
                ("redirect_uri", REDIRECT_URI),
            ])
            .send()
            .await?;

        let token = response.json::<TokenResponse>().await?;
        log::info!("Successfully obtained token");
        log::debug!("Token expires in {} seconds", token.expires_in);
        
        *self.tokens.write().await = Some(token.clone());
        self.storage.save_auth_tokens(&AuthTokens::from(token.clone()))?;
        Ok(token)
    }

    pub async fn refresh_token(&self) -> Result<TokenResponse> {
        log::info!("Refreshing token");
        
        let current_tokens = self.tokens.read().await;
        let refresh_token = current_tokens.as_ref()
            .map(|t| t.refresh_token.clone())
            .ok_or_else(|| anyhow::anyhow!("No refresh token available"))?;

        let response = self.client
            .post(TOKEN_URL)
            .header("User-Agent", USER_AGENT)
            .form(&[
                ("grant_type", "refresh_token"),
                ("client_id", &CONFIG.shikimori_client_id),
                ("client_secret", &CONFIG.shikimori_client_secret),
                ("refresh_token", &refresh_token),
            ])
            .send()
            .await?;

        let token = response.json::<TokenResponse>().await?;
        log::info!("Successfully refreshed token");
        log::debug!("New token expires in {} seconds", token.expires_in);
        
        *self.tokens.write().await = Some(token.clone());
        self.storage.save_auth_tokens(&AuthTokens::from(token.clone()))?;
        Ok(token)
    }

    pub async fn get_user_info(&self) -> Result<UserInfo> {
        log::info!("Fetching user info");
        
        let tokens = self.tokens.read().await;
        let access_token = tokens.as_ref()
            .map(|t| t.access_token.clone())
            .ok_or_else(|| anyhow::anyhow!("No access token available"))?;

        let response = self.client
            .get(format!("{}/users/whoami", API_URL))
            .header("User-Agent", USER_AGENT)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            log::warn!("Access token expired, attempting to refresh");
            drop(tokens);
            let new_token = self.refresh_token().await?;
            
            log::debug!("Retrying user info request with new token");
            let response = self.client
                .get(format!("{}/users/whoami", API_URL))
                .header("User-Agent", USER_AGENT)
                .header("Authorization", format!("Bearer {}", new_token.access_token))
                .send()
                .await?;

            let user_info = response.json::<UserInfo>().await?;
            log::info!("Successfully fetched user info for: {} (ID: {})", user_info.nickname, user_info.id);
            Ok(user_info)
        } else {
            let user_info = response.json::<UserInfo>().await?;
            log::info!("Successfully fetched user info for: {} (ID: {})", user_info.nickname, user_info.id);
            Ok(user_info)
        }
    }

    pub async fn logout(&self) -> Result<()> {
        *self.tokens.write().await = None;
        self.storage.delete_auth_tokens()?;
        log::info!("User logged out");
        Ok(())
    }
}

impl Clone for ShikimoriOAuth {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
            tokens: Arc::clone(&self.tokens),
            storage: self.storage.clone(),
        }
    }
} 