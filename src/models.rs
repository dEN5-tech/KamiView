use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub total: i32,
    pub results: Vec<MediaResult>,
    pub error: Option<String>,
    pub next_page_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MediaResult {
    pub id: String,
    pub title: String,
    pub media_type: String,
    pub year: i32,
    pub link: String,
    pub translation: Option<Translation>,
}

#[derive(Debug, Deserialize)]
pub struct InfoResponse {
    pub series_count: i32,
    pub translations: Vec<Translation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Translation {
    pub id: String,
    pub translation_type: String,
    pub name: String,
} 