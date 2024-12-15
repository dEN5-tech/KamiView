use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentEpisode {
    pub shikimori_id: String,
    pub episode: i32,
    pub translation_id: String
}
