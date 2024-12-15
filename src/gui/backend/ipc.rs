use serde::Serialize;
use crate::kodik::MediaResult;
use serde_json::Value;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum IpcResponse {
    #[serde(rename = "success")]
    Success {
        data: Value
    },

    #[serde(rename = "error")] 
    Error {
        message: String
    },

    #[serde(rename = "searchResults")]
    SearchResults {
        results: Vec<MediaResult>
    },

    #[serde(rename = "animeInfo")]
    AnimeInfo {
        translations: Vec<TranslationInfo>,
        episodes: i32
    },

    #[serde(rename = "authUrl")]
    AuthUrl {
        url: String
    },

    #[serde(rename = "authStatus")]
    AuthStatus {
        status: bool
    },

    #[serde(rename = "userInfo")]
    UserInfo {
        username: String,
        avatar: String
    }
}

#[derive(Serialize, Debug)]
pub struct TranslationInfo {
    pub id: String,
    pub title: String,
    pub episodes: i32
}
