use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;

const TOKEN_STORAGE_KEY: &str = "auth_tokens";

#[derive(Debug)]
pub struct StorageService {
    storage_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64,
}

impl StorageService {
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "kamiview", "KamiView")
            .ok_or_else(|| anyhow::anyhow!("Failed to get project directories"))?;

        let storage_path = project_dirs.data_dir().to_path_buf();
        fs::create_dir_all(&storage_path)?;

        Ok(Self { storage_path })
    }

    pub fn save_auth_tokens(&self, tokens: &AuthTokens) -> Result<()> {
        self.save(TOKEN_STORAGE_KEY, tokens)
    }

    pub fn load_auth_tokens(&self) -> Result<Option<AuthTokens>> {
        self.load(TOKEN_STORAGE_KEY)
    }

    pub fn delete_auth_tokens(&self) -> Result<()> {
        self.delete(TOKEN_STORAGE_KEY)
    }

    pub fn save<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let file_path = self.storage_path.join(format!("{}.json", key));
        let json = serde_json::to_string(value)?;
        fs::write(file_path, json)?;
        Ok(())
    }

    pub fn load<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        let file_path = self.storage_path.join(format!("{}.json", key));
        if !file_path.exists() {
            return Ok(None);
        }
        let json = fs::read_to_string(file_path)?;
        let value = serde_json::from_str(&json)?;
        Ok(Some(value))
    }

    fn delete(&self, key: &str) -> Result<()> {
        let file_path = self.storage_path.join(format!("{}.json", key));
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        Ok(())
    }
}

impl Clone for StorageService {
    fn clone(&self) -> Self {
        Self {
            storage_path: self.storage_path.clone(),
        }
    }
} 