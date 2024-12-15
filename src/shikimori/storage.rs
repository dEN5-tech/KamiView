use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use crate::utils::constants::CONFIG;
use crate::storage::get_storage_path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64,
}

#[derive(Debug, Clone)]
pub struct Storage {
    pub path: PathBuf,
    pub client_id: String,
    pub client_secret: String,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let path = get_storage_path().clone();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        Ok(Self {
            path,
            client_id: CONFIG.shikimori_client_id.to_string(),
            client_secret: CONFIG.shikimori_client_secret.to_string(),
        })
    }

    pub fn save_auth_tokens(&self, tokens: &AuthTokens) -> Result<()> {
        let json = serde_json::to_string_pretty(tokens)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    pub fn load_auth_tokens(&self) -> Result<Option<AuthTokens>> {
        if !self.path.exists() {
            return Ok(None);
        }
        let json = fs::read_to_string(&self.path)?;
        let tokens = serde_json::from_str(&json)?;
        Ok(Some(tokens))
    }

    pub fn delete_auth_tokens(&self) -> Result<()> {
        if self.path.exists() {
            fs::remove_file(&self.path)?;
        }
        Ok(())
    }

    pub fn shikimori_client_id(&self) -> &str {
        &self.client_id
    }

    pub fn shikimori_client_secret(&self) -> &str {
        &self.client_secret
    }
} 