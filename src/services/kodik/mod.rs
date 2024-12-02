pub mod api;

use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use api::{KodikParser, MediaResult};
pub use api::Translation;
use crate::components::kodik::episode_list::Episode;

#[derive(Debug, Clone)]
pub struct KodikService {
    parser: Arc<RwLock<KodikParser>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    pub title: String,
    pub title_orig: String,
    pub other_title: Option<String>,
    pub year: i32,
    pub screenshots: Vec<String>,
    pub shikimori_id: Option<String>,
    pub kinopoisk_id: Option<String>,
    pub imdb_id: Option<String>,
}

impl SearchResult {
    pub fn display_title(&self) -> &str {
        &self.title
    }

    pub fn display_subtitle(&self) -> String {
        let mut subtitle = Vec::new();

        if let Some(other) = &self.other_title {
            subtitle.push(other.clone());
        }
        
        if self.year > 0 {
            subtitle.push(self.year.to_string());
        }

        if subtitle.is_empty() {
            self.title_orig.clone()
        } else {
            subtitle.join(" | ")
        }
    }

    pub fn display_description(&self) -> String {
        self.title_orig.clone()
    }

    pub fn media_badges(&self) -> Vec<String> {
        let mut badges = Vec::new();

        if let Some(id) = &self.shikimori_id {
            badges.push(format!("Shikimori: {}", id));
        }

        if let Some(id) = &self.kinopoisk_id {
            badges.push(format!("Kinopoisk: {}", id));
        }

        if let Some(id) = &self.imdb_id {
            badges.push(format!("IMDB: {}", id));
        }

        badges
    }

    pub fn external_links(&self) -> Vec<(String, String)> {
        let mut links = Vec::new();

        if let Some(id) = &self.shikimori_id {
            links.push(("Shikimori".to_string(), format!("https://shikimori.one/animes/{}", id)));
        }

        if let Some(id) = &self.kinopoisk_id {
            links.push(("Kinopoisk".to_string(), format!("https://www.kinopoisk.ru/film/{}/", id)));
        }

        if let Some(id) = &self.imdb_id {
            links.push(("IMDB".to_string(), format!("https://www.imdb.com/title/{}/", id)));
        }

        links
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIEpisode {
    pub number: i32,
    pub title: String,
    pub translation_id: String,
    pub translation_name: String,
    pub url: String,
}

impl From<UIEpisode> for Episode {
    fn from(episode: UIEpisode) -> Self {
        Self {
            number: episode.number,
            title: episode.title,
            translation_id: episode.translation_id,
            translation_name: episode.translation_name,
            url: episode.url,
        }
    }
}

impl KodikService {
    pub fn empty() -> Self {
        Self {
            parser: Arc::new(RwLock::new(KodikParser::empty()))
        }
    }

    pub async fn new(token: Option<String>) -> Result<Self> {
        let parser = KodikParser::new(token, false).await?;
        
        Ok(Self {
            parser: Arc::new(RwLock::new(parser)),
        })
    }

    pub async fn search(
        &self, 
        query: &str, 
        limit: Option<i32>, 
        strict: bool
    ) -> Result<Vec<SearchResult>, String> {
        let parser = self.parser.read().await;
        let results = parser.search(query, limit, true, None, strict, true)
            .await
            .map_err(|e| e.to_string())?;

        Ok(results.into_iter()
            .map(|r| SearchResult {
                title: r.title,
                title_orig: r.title_orig,
                other_title: r.other_title,
                year: r.year,
                screenshots: r.screenshots,
                shikimori_id: r.shikimori_id,
                kinopoisk_id: r.kinopoisk_id,
                imdb_id: r.imdb_id,
            })
            .collect())
    }

    pub async fn get_episodes(&self, id: &str) -> Result<Vec<Episode>, String> {
        let parser = self.parser.read().await;
        let info = parser.get_info(id, "shikimori")
            .await
            .map_err(|e| e.to_string())?;

        let episodes = (1..=info.series_count)
            .map(|num| {
                let translation = info.translations.first().cloned().unwrap_or_else(|| Translation {
                    id: "0".to_string(),
                    translation_type: "Unknown".to_string(),
                    name: "Default".to_string(),
                });

                Episode {
                    number: num,
                    title: String::new(),
                    translation_id: translation.id,
                    translation_name: translation.name,
                    url: String::new(),
                }
            })
            .collect();

        Ok(episodes)
    }

    pub async fn get_video_link(
        &self,
        id: &str,
        id_type: &str,
        episode: i32,
        translation_id: &str,
    ) -> Result<(String, i32), String> {
        let parser = self.parser.read().await;
        let (download_link, quality) = parser.get_download_link(id, id_type, episode, translation_id)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok((format!("https://{}/720.mp4/", download_link), quality))
    }

    pub async fn get_translations(&self, id: &str) -> Result<Vec<Translation>, String> {
        log::debug!("Getting translations for id: {}", id);
        let parser = self.parser.read().await;
        let result = parser.translations(id, "shikimori")
            .await
            .map_err(|e| {
                log::error!("Failed to get translations: {}", e);
                e.to_string()
            })?;
        log::debug!("Found {} translations", result.len());
        Ok(result)
    }

    pub async fn search_by_id(
        &self,
        id: &str,
        id_type: &str,
        limit: Option<i32>
    ) -> Result<Vec<SearchResult>, String> {
        let parser = self.parser.read().await;
        let results = parser.search_by_id(id, id_type, limit)
            .await
            .map_err(|e| e.to_string())?;

        Ok(results.into_iter()
            .map(|r| SearchResult {
                title: r.title,
                title_orig: r.title_orig,
                other_title: r.other_title,
                year: r.year,
                screenshots: r.screenshots,
                shikimori_id: r.shikimori_id,
                kinopoisk_id: r.kinopoisk_id,
                imdb_id: r.imdb_id,
            })
            .collect())
    }

    pub async fn get_series_count(&self, id: &str) -> Result<i32, String> {
        let parser = self.parser.read().await;
        parser.series_count(id, "shikimori")
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_download_link(
        &self,
        id: &str,
        id_type: &str,
        episode: i32,
        translation_id: &str,
    ) -> Result<(String, i32), String> {
        let parser = self.parser.read().await;
        parser.get_download_link(id, id_type, episode, translation_id)
            .await
            .map_err(|e| e.to_string())
    }
}
