use std::sync::Arc;
use tao::window::Window;
use wry::{WebViewBuilder, WebView};
use super::scripts::get_full_init_script;
use super::server::LocalServer;
use tokio::sync::mpsc::Sender;
use crate::di::Container;
use log::{info, error};
use tokio::sync::mpsc;
use crate::gui::backend::types::CurrentEpisode;
use tokio::sync::Mutex;
use crate::gui::backend::ipc::{self, IpcResponse};
use crate::gui::backend::handlers;
use crate::utils::routes::*;  // Import routes from parent module

pub fn create_webview(
    window: &Window,
    html: &str,
    container: Arc<Container>,
    tx: Sender<String>
) -> wry::Result<WebView> {
    let container = container.clone();
    
    // Create channel with capacity to avoid blocking
    let (script_tx, mut script_rx) = mpsc::channel::<(String, String)>(100);
    
    // Create current episode state with proper type
    let current_episode = Arc::new(Mutex::new(None::<CurrentEpisode>));

    // In release mode, start local server
    #[cfg(not(debug_assertions))]
    let url = {
        let mut server = LocalServer::new(html.to_string());
        server.start()
    };

    // In debug mode, use the direct HTML
    #[cfg(debug_assertions)]
    let url = html;

    // Create webview with proper error handling
    let webview = WebViewBuilder::new(window)
        .with_initialization_script(&get_full_init_script())
        .with_ipc_handler(move |msg| {
            let container = container.clone();
            let script_tx = script_tx.clone();
            let current_episode = current_episode.clone();
            
            tokio::spawn(async move {
                if let Err(e) = async {
                    match serde_json::from_str::<serde_json::Value>(&msg) {
                        Ok(value) => {
                            let id = value.get("id")
                                .and_then(|i| i.as_str())
                                .unwrap_or("unknown")
                                .to_string();
                            
                            process_message(
                                &container,
                                &msg,
                                id.clone(),
                                &script_tx,
                                current_episode
                            ).await?;
                        },
                        Err(e) => {
                            error!("Failed to parse message: {}", e);
                            return Ok(());
                        }
                    }
                    Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
                }.await {
                    error!("Message processing task failed: {}", e);
                }
            });
        })
        .with_url(&url)?
        .build()?;

    // Spawn script evaluation task with proper cleanup
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        while let Some((id, script)) = script_rx.recv().await {
            match tx_clone.send(script).await {
                Ok(_) => {
                    info!("Script for message {} sent successfully", id);
                },
                Err(e) => {
                    error!("Failed to send script: {}", e);
                    break;
                }
            }
        }
        info!("Script evaluation task terminated");
    });

    Ok(webview)
}

async fn process_message(
    container: &Arc<Container>,
    msg: &str,
    id: String,
    script_tx: &Sender<(String, String)>,
    current_episode: Arc<Mutex<Option<CurrentEpisode>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let value: serde_json::Value = serde_json::from_str(msg)?;
    
    log::debug!("Processing message: {}", msg);

    let msg_type = value.get("type")
        .and_then(|t| t.as_str())
        .ok_or("Missing message type")?;
    
    let payload = value.get("payload")
        .cloned()
        .unwrap_or(serde_json::json!({}));

    let response = match msg_type {
        API_SEARCH => handlers::handle_search(container, payload).await,
        API_ANIME_SELECTED => handlers::handle_anime_selected(container, payload).await,
        API_PLAY_EPISODE => handlers::handle_play_episode(container, payload, current_episode).await,
        API_GET_PLAYBACK_INFO => handlers::handle_get_playback_info(container).await,
        API_TOGGLE_PLAYBACK => handlers::handle_toggle_playback(container, payload).await,
        API_STOP_PLAYBACK => handlers::handle_stop_playback(container).await,
        API_START_DOWNLOAD => handlers::handle_start_download(container, payload).await,
        API_EXCHANGE_CODE => handlers::handle_exchange_code(container, payload).await,
        API_GET_USER_INFO => handlers::handle_get_user_info(container).await,
        API_LOGOUT => handlers::handle_logout(container).await,
        API_OPEN_AUTH_URL => handlers::handle_open_auth_url(container).await,
        _ => {
            let err = format!("Unknown message type: {}", msg_type);
            log::error!("{}", err);
            Some(ipc::IpcResponse::Error { message: err })
        }
    };

    // Convert response to JSON with proper error handling
    let response_json = match &response {
        Some(resp) => {
            let response_type = match resp {
                IpcResponse::Success { .. } => "success",
                IpcResponse::Error { .. } => "error", 
                IpcResponse::SearchResults { .. } => "searchResults",
                IpcResponse::AnimeInfo { .. } => "animeInfo",
                IpcResponse::AuthUrl { .. } => "authUrl",
                IpcResponse::AuthStatus { .. } => "authStatus",
                IpcResponse::UserInfo { .. } => "userInfo",
            };
            
            serde_json::json!({
                "id": id,
                "type": response_type,
                "data": resp
            })
        },
        None => serde_json::json!({
            "id": id,
            "type": "error",
            "data": {
                "message": "No response from handler"
            }
        })
    };

    let js_code = format!(
        "window.__IPC_CALLBACK__({})",
        response_json.to_string()
    );

    script_tx.send((id.clone(), js_code)).await?;
    Ok(())
}