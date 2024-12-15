use crate::di::Container;
use std::sync::Arc;
use serde_json::Value;
use tokio::sync::Mutex;
use crate::gui::backend::ipc::{IpcResponse, TranslationInfo};
use crate::gui::backend::types::CurrentEpisode;
use log::{error, debug};
use anyhow::Result;
use regex::Regex;
use open;

// Message type constants
pub const MSG_TYPE_SEARCH: &str = "search";
pub const MSG_TYPE_ANIME_SELECTED: &str = "animeSelected";
pub const MSG_TYPE_PLAY_EPISODE: &str = "playEpisode";
pub const MSG_TYPE_GET_PLAYBACK_INFO: &str = "getPlaybackInfo";
pub const MSG_TYPE_TOGGLE_PLAYBACK: &str = "togglePlayback";
pub const MSG_TYPE_STOP_PLAYBACK: &str = "stopPlayback";

// Helper function to extract string field from payload
fn extract_str_field<'a>(payload: &'a Value, field: &str) -> Result<&'a str> {
    payload.get(field)
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing {} field", field))
}

// Helper function to extract i64 field from payload
fn extract_i64_field(payload: &Value, field: &str) -> Result<i64> {
    payload.get(field)
        .and_then(|v| v.as_i64())
        .ok_or_else(|| anyhow::anyhow!("Missing {} field", field))
}

pub async fn handle_search(
    container: &Arc<Container>, 
    payload: Value
) -> Option<IpcResponse> {
    debug!("Handling search request with payload: {:?}", payload);
    
    let query = match extract_str_field(&payload, "query") {
        Ok(q) => q,
        Err(e) => {
            error!("Search error: {}", e);
            return Some(IpcResponse::Error {
                message: format!("Invalid search request: {}", e)
            });
        }
    };
    
    match container.kodik().search_anime(query).await {
        Ok(results) => {
            debug!("Found {} results for query: {}", results.len(), query);
            Some(IpcResponse::SearchResults { results })
        },
        Err(e) => {
            error!("Search failed: {}", e);
            Some(IpcResponse::Error { 
                message: format!("Search failed: {}", e)
            })
        }
    }
}

pub async fn handle_anime_selected(
    container: &Arc<Container>,
    payload: Value
) -> Option<IpcResponse> {
    debug!("Handling anime_selected request: {:?}", payload);

    let shikimori_id = match extract_str_field(&payload, "shikimoriId") {
        Ok(id) => id,
        Err(e) => {
            error!("Anime selection error: {}", e);
            return Some(IpcResponse::Error { 
                message: format!("Invalid anime selection request: {}", e)
            });
        }
    };
    
    match container.kodik().get_anime_info(shikimori_id).await {
        Ok(info) => {
            // Parse translations with episode count
            let re = Regex::new(r"(.*?)\s*\((\d+)\s*эп\.\)").unwrap();
            let translations: Vec<TranslationInfo> = info.translations
                .into_iter()
                .filter_map(|t| {
                    if let Some(caps) = re.captures(&t.name) {
                        Some(TranslationInfo {
                            id: t.id,
                            title: caps[1].trim().to_string(),
                            episodes: caps[2].parse().unwrap_or(0)
                        })
                    } else {
                        None
                    }
                })
                .collect();
                
            debug!("Found anime info: {} translations", translations.len());
            
            Some(IpcResponse::AnimeInfo {
                translations,
                episodes: info.series_count
            })
        },
        Err(e) => {
            error!("Failed to get anime info: {}", e);
            Some(IpcResponse::Error {
                message: format!("Failed to get anime info: {}", e)
            })
        }
    }
}

pub async fn handle_play_episode(
    container: &Arc<Container>,
    payload: Value,
    current_episode: Arc<Mutex<Option<CurrentEpisode>>>
) -> Option<IpcResponse> {
    debug!("Handling play_episode request: {:?}", payload);

    // Extract required fields
    let shikimori_id = extract_str_field(&payload, "shikimoriId");
    let episode = extract_i64_field(&payload, "episode");
    let translation_id = extract_str_field(&payload, "translationId");

    // Match all results together
    let (shikimori_id, episode, translation_id) = match (shikimori_id, episode, translation_id) {
        (Ok(id), Ok(ep), Ok(tr_id)) => (id, ep as i32, tr_id),
        _ => {
            error!("Invalid play episode request parameters");
            return Some(IpcResponse::Error {
                message: "Invalid play episode parameters".to_string()
            });
        }
    };

    // Update current episode with proper scope
    {
        let mut episode_guard = current_episode.lock().await;
        *episode_guard = Some(CurrentEpisode {
            shikimori_id: shikimori_id.to_string(),
            episode,
            translation_id: translation_id.to_string()
        });
    }

    match container.kodik().get_episode_link(
        shikimori_id,
        episode,
        translation_id
    ).await {
        Ok((url, _)) => {
            match container.mpv().play(&format!("https://{}/720.mp4/", url)) {
                Ok(_) => Some(IpcResponse::Success { 
                    data: serde_json::json!({
                        "message": "Started playback"
                    })
                }),
                Err(e) => {
                    error!("Failed to start playback: {}", e);
                    Some(IpcResponse::Error {
                        message: format!("Failed to start playback: {}", e)
                    })
                }
            }
        },
        Err(e) => {
            error!("Failed to get episode link: {}", e);
            Some(IpcResponse::Error {
                message: format!("Failed to get episode link: {}", e)
            })
        }
    }
}

pub async fn handle_get_playback_info(
    container: &Arc<Container>,
) -> Option<IpcResponse> {
    debug!("Handling get_playback_info request");
    
    match container.mpv().get_playback_info() {
        Ok(info) => Some(IpcResponse::Success { 
            data: serde_json::json!({
                "position": info.position,
                "duration": info.duration,
                "paused": info.paused
            })
        }),
        Err(e) => {
            error!("Failed to get playback info: {}", e);
            Some(IpcResponse::Error {
                message: format!("Failed to get playback info: {}", e)
            })
        }
    }
}

pub async fn handle_toggle_playback(
    _container: &Arc<Container>,
    payload: Value
) -> Option<IpcResponse> {
    debug!("Handling toggle_playback request: {:?}", payload);
    
    let paused = payload.get("paused")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    // TODO: Implement MPV toggle playback
    Some(IpcResponse::Success { 
        data: serde_json::json!({
            "paused": paused
        })
    })
}

pub async fn handle_stop_playback(
    _container: &Arc<Container>
) -> Option<IpcResponse> {
    debug!("Handling stop_playback request");
    
    // TODO: Implement MPV stop playback
    Some(IpcResponse::Success { 
        data: serde_json::json!({
            "message": "Playback stopped"
        })
    })
}


pub async fn handle_start_download(
    _container: &Arc<Container>,
    _payload: Value
) -> Option<IpcResponse> {
    // TODO: Implement download handling
    Some(IpcResponse::Success { 
        data: serde_json::json!({ "status": "started" })
    })
}

pub async fn handle_exchange_code(
    container: &Arc<Container>,
    payload: Value
) -> Option<IpcResponse> {
    let code = match payload.get("code").and_then(|c| c.as_str()) {
        Some(code) => code,
        None => return Some(IpcResponse::Error {
            message: "Missing code parameter".to_string()
        })
    };

    match container.shikimori().exchange_code(code).await {
        Ok(_token) => {
            // After exchanging code, immediately get user info
            match container.shikimori().get_user_info().await {
                Ok(user) => Some(IpcResponse::Success { 
                    data: serde_json::json!({
                        "username": user.nickname,
                        "avatar": user.avatar,
                        "id": user.id
                    })
                }),
                Err(e) => Some(IpcResponse::Error {
                    message: format!("Failed to get user info after auth: {}", e)
                })
            }
        },
        Err(e) => Some(IpcResponse::Error { 
            message: e.to_string() 
        })
    }
}

pub async fn handle_get_user_info(
    container: &Arc<Container>
) -> Option<IpcResponse> {
    debug!("Handling get_user_info request");

    match container.shikimori().get_user_info().await {
        Ok(user) => {
            debug!("Returning user info: {} (ID: {})", user.nickname, user.id);
            Some(IpcResponse::Success { 
                data: serde_json::json!({
                    "username": user.nickname,
                    "avatar": user.avatar,
                    "id": user.id
                })
            })
        },
        Err(e) => {
            error!("Failed to get user info: {}", e);
            Some(IpcResponse::Error {
                message: format!("Failed to get user info: {}", e)
            })
        }
    }
}

pub async fn handle_logout(
    container: &Arc<Container>
) -> Option<IpcResponse> {
    match container.shikimori().logout().await {
        Ok(_) => Some(IpcResponse::Success {
            data: serde_json::json!({ "status": "logged_out" })
        }),
        Err(e) => Some(IpcResponse::Error {
            message: e.to_string()
        })
    }
}

pub async fn handle_open_auth_url(
    container: &Arc<Container>,
) -> Option<IpcResponse> {
    debug!("Handling open_auth_url request");

    match container.shikimori().get_auth_url() {
        Ok(url) => {
            // Open URL in default browser
            if let Err(e) = open::that(&url) {
                error!("Failed to open auth URL in browser: {}", e);
                return Some(IpcResponse::Error {
                    message: format!("Failed to open auth URL: {}", e)
                });
            }
            
            Some(IpcResponse::AuthUrl { url })
        },
        Err(e) => {
            error!("Failed to get auth URL: {}", e);
            Some(IpcResponse::Error {
                message: format!("Failed to get auth URL: {}", e)
            })
        }
    }
}