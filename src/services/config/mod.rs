use serde::{Deserialize, Serialize};
use anyhow::Result;
use super::storage::StorageService;

const CONFIG_KEY: &str = "app_config";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub auto_auth: bool,
    pub theme_variant: String,
    pub last_login: Option<String>,
}

pub struct ConfigService {
    storage: StorageService,
}

impl ConfigService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            storage: StorageService::new()?,
        })
    }

    pub fn load_config(&self) -> Result<AppConfig> {
        Ok(self.storage
            .load::<AppConfig>(CONFIG_KEY)?
            .unwrap_or_default())
    }

    pub fn save_config(&self, config: &AppConfig) -> Result<()> {
        self.storage.save(CONFIG_KEY, config)
    }
} 