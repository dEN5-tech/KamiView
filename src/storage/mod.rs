use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use iced::Theme;
use crate::di::interfaces::IStorage;
use shaku::Component;
use std::fmt;
use std::sync::OnceLock;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: ThemeType,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ThemeType {
    Light,
    Dark,
}

impl fmt::Display for ThemeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeType::Light => write!(f, "Light"),
            ThemeType::Dark => write!(f, "Dark"),
        }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: ThemeType::Light,
        }
    }
}

#[derive(Component)]
#[shaku(interface = IStorage)]
pub struct Storage {
    #[shaku(default)]
    path: PathBuf,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            path: Self::initialize_path(),
        }
    }
}

impl Storage {
    pub fn initialize_path() -> PathBuf {
        let project_dirs = ProjectDirs::from("com", "kamiview", "KamiView")
            .expect("Failed to get project directories");
            
        let config_dir = project_dirs.config_dir();
        fs::create_dir_all(config_dir).expect("Failed to create config directory");
        
        config_dir.join("settings.json")
    }

    pub fn load(&self) -> AppSettings {
        if let Ok(contents) = fs::read_to_string(&self.path) {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            AppSettings::default()
        }
    }

    pub fn save(&self, settings: &AppSettings) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(settings)?;
        fs::write(&self.path, json)?;
        Ok(())
    }
}

impl From<ThemeType> for Theme {
    fn from(theme_type: ThemeType) -> Self {
        match theme_type {
            ThemeType::Light => Theme::Light,
            ThemeType::Dark => Theme::Dark,
        }
    }
}

impl IStorage for Storage {
    fn load(&self) -> AppSettings {
        self.load()
    }

    fn save(&self, settings: &AppSettings) -> Result<(), Box<dyn std::error::Error>> {
        self.save(settings)
    }
}

// Static storage for the initialized path
static STORAGE_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn initialize_storage_path() {
    let mut path = dirs::config_dir()
        .expect("Failed to get config directory");
    path.push("kamiview");
    path.push("shikimori");
    path.push("tokens.json");
    
    STORAGE_PATH.get_or_init(|| path);
}

pub fn get_storage_path() -> &'static PathBuf {
    STORAGE_PATH.get().expect("Storage path not initialized")
}