use tokio::sync::{mpsc, oneshot, broadcast};
use std::io;
use mpv_socket::{MpvSocket, Property};
use std::process::Command as ProcessCommand;
use tokio::time::{sleep, Duration};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Once;
use std::sync::Mutex as StdMutex;
use crate::services::discord::{DiscordService, ActivityInfo, PlaybackState};

type MpvResult<T> = std::result::Result<T, io::Error>;

#[derive(Clone)]
pub struct MpvService {
    command_tx: mpsc::Sender<MpvCommand>,
    event_tx: broadcast::Sender<MpvEvent>,
    is_running: Arc<AtomicBool>,
    shutdown_complete: Arc<Mutex<bool>>,
    current_media: Arc<Mutex<Option<CurrentMedia>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MpvConfig {
    // Track last position for each video
    last_positions: HashMap<String, f64>,
    // Track last volume setting
    last_volume: i64,
    // Track last used settings per video
    video_settings: HashMap<String, VideoSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSettings {
    volume: i64,
    subtitle_id: Option<String>,
    audio_track_id: Option<String>,
}

#[derive(Debug)]
enum MpvCommand {
    Play { 
        url: String, 
        title: String,
        last_position: Option<f64>,
        settings: Option<VideoSettings>,
        response: oneshot::Sender<MpvResult<mpsc::Receiver<MpvEvent>>> 
    },
    Pause { response: oneshot::Sender<MpvResult<()>> },
    Resume { response: oneshot::Sender<MpvResult<()>> },
    Seek { position: f64, response: oneshot::Sender<MpvResult<()>> },
    SetVolume { volume: i64, response: oneshot::Sender<MpvResult<()>> },
    Stop { response: oneshot::Sender<MpvResult<()>> },
    Shutdown,
}

#[derive(Debug, Clone)]
pub enum MpvEvent {
    PropertyChange {
        name: String,
        value: String,
    },
    PlaybackFinished,
    Error(String),
}

// Singleton instance
static INSTANCE: StdMutex<Option<MpvService>> = StdMutex::new(None);
static INIT: Once = Once::new();

#[derive(Clone)]
struct CurrentMedia {
    title: String,
    episode: Option<i32>,
    translation: Option<String>,
}

impl MpvService {
    pub fn instance() -> MpvService {
        INIT.call_once(|| {
            let service = MpvService::new_internal();
            *INSTANCE.lock().unwrap() = Some(service);
        });

        INSTANCE.lock().unwrap().clone().unwrap()
    }

    // Rename new() to new_internal() as it's now private
    fn new_internal() -> Self {
        let (command_tx, mut command_rx) = mpsc::channel(32);
        let (event_tx, _) = broadcast::channel(100); // For broadcasting events
        let is_running = Arc::new(AtomicBool::new(true));
        let shutdown_complete = Arc::new(Mutex::new(false));
        let current_media = Arc::new(Mutex::new(None));
        
        let service = Self { 
            command_tx,
            event_tx: event_tx.clone(),
            is_running: is_running.clone(),
            shutdown_complete: shutdown_complete.clone(),
            current_media: current_media.clone(),
        };

        let running = is_running.clone();
        let event_tx = event_tx.clone();
        
        let discord = DiscordService::instance();
        
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let socket_path = r#"\\.\pipe\mpv-socket"#.to_string();
                let mut current_socket: Option<MpvSocket> = None;
                let mut current_process: Option<std::process::Child> = None;
                let mut position_tracker: Option<(String, tokio::time::Interval)> = None;

                while let Some(cmd) = command_rx.recv().await {
                    // Handle position tracking
                    if let Some((url, ref mut interval)) = position_tracker.as_mut() {
                        if let Some(socket) = current_socket.as_mut() {
                            interval.tick().await;
                            if let Ok(position) = socket.get_property::<f64>(Property::TimePos) {
                                // Broadcast position update
                                let _ = event_tx.send(MpvEvent::PropertyChange {
                                    name: "time-pos".to_string(),
                                    value: position.to_string(),
                                });
                                
                                if let Err(e) = Self::save_position(url, position).await {
                                    log::error!("Failed to save position: {}", e);
                                }
                            }
                        }
                    }

                    match cmd {
                        MpvCommand::Shutdown => {
                            if let Err(e) = Self::cleanup_resources(&mut current_socket, &mut current_process).await {
                                log::error!("Error during shutdown cleanup: {}", e);
                            }
                            break;
                        }
                        MpvCommand::Play { url, title, last_position, settings, response } => {
                            let result = async {
                                // Clean up previous session
                                Self::cleanup_resources(&mut current_socket, &mut current_process).await?;

                                // Start new MPV process
                                let process = ProcessCommand::new("mpv")
                                    .args([
                                        &url,
                                        "--idle=yes",
                                        "--cache=yes",
                                        "--cache-secs=30",
                                        "--demuxer-max-bytes=500M",
                                        "--demuxer-readahead-secs=20",
                                        &format!("--title={}", title),
                                        "--user-agent=Mozilla/5.0",
                                        "--volume=100",
                                        "--fullscreen",
                                        "--save-position-on-quit",
                                        &format!("--input-ipc-server={}", socket_path),
                                    ])
                                    .spawn()
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to start MPV: {}", e)))?;

                                current_process = Some(process);

                                // Wait for socket to be available
                                let mut retries = 5;
                                let mut socket = None;
                                while retries > 0 {
                                    match MpvSocket::connect(&socket_path) {
                                        Ok(s) => {
                                            socket = Some(s);
                                            break;
                                        }
                                        Err(_) => {
                                            sleep(Duration::from_millis(200)).await;
                                            retries -= 1;
                                        }
                                    }
                                }

                                let mut socket = socket.ok_or_else(|| {
                                    io::Error::new(io::ErrorKind::Other, "Failed to connect to MPV socket")
                                })?;

                                // Apply saved settings if available
                                if let Some(settings) = settings {
                                    if let Err(e) = socket.set_property(Property::Volume, settings.volume) {
                                        log::error!("Failed to restore volume: {}", e);
                                    }
                                    // Add other settings restoration here
                                }

                                // Seek to last position if available
                                if let Some(position) = last_position {
                                    if let Err(e) = socket.set_property(Property::TimePos, position) {
                                        log::error!("Failed to seek to last position: {}", e);
                                    }
                                }

                                let (tx, rx) = mpsc::channel(100);
                                current_socket = Some(socket);

                                // Start position tracking
                                position_tracker = Some((
                                    url.clone(),
                                    tokio::time::interval(Duration::from_secs(5))
                                ));

                                // Subscribe to events
                                let mut event_rx = event_tx.subscribe();
                                tokio::spawn(async move {
                                    while let Ok(event) = event_rx.recv().await {
                                        if tx.send(event).await.is_err() {
                                            break;
                                        }
                                    }
                                });

                                Ok(rx)
                            }.await;

                            // Always send response, even if it's an error
                            if response.send(result).is_err() {
                                log::error!("Failed to send MPV response - channel closed");
                                if let Err(e) = Self::cleanup_resources(&mut current_socket, &mut current_process).await {
                                    log::error!("Error during cleanup after failed response: {}", e);
                                }
                            }

                            // Update Discord presence
                            if let Ok(mut media) = current_media.lock() {
                                *media = Some(CurrentMedia {
                                    title: title.clone(),
                                    episode: None, // Update this when episode info is available
                                    translation: None, // Update this when translation info is available
                                });
                            }
                        },
                        MpvCommand::Pause { response } => {
                            let result = if let Some(ref mut socket) = current_socket {
                                socket.set_property(Property::Pause, true)
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
                            } else {
                                Ok(())
                            };
                            let _ = response.send(result);

                            // Update Discord presence
                            if let Ok(media) = current_media.lock() {
                                if let Some(ref media) = *media {
                                    discord.update_activity(ActivityInfo {
                                        title: media.title.clone(),
                                        episode: media.episode,
                                        translation: media.translation.clone(),
                                        state: PlaybackState::Paused,
                                    });
                                }
                            }
                        },
                        MpvCommand::Resume { response } => {
                            let result = if let Some(ref mut socket) = current_socket {
                                socket.set_property(Property::Pause, false)
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
                            } else {
                                Ok(())
                            };
                            let _ = response.send(result);
                        },
                        MpvCommand::Seek { position, response } => {
                            let result = if let Some(ref mut socket) = current_socket {
                                socket.set_property(Property::PercentPos, position)
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
                            } else {
                                Ok(())
                            };
                            let _ = response.send(result);
                        },
                        MpvCommand::SetVolume { volume, response } => {
                            let result = if let Some(ref mut socket) = current_socket {
                                socket.set_property(Property::Volume, volume)
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
                            } else {
                                Ok(())
                            };
                            let _ = response.send(result);
                        },
                        MpvCommand::Stop { response } => {
                            position_tracker = None;
                            let result = async {
                                if let Some(mut process) = current_process.take() {
                                    let _ = process.kill();
                                    let _ = process.wait();
                                }
                                if let Some(socket) = current_socket.take() {
                                    drop(socket);
                                }
                                Ok(())
                            }.await;
                            let _ = response.send(result);

                            discord.clear_activity();
                        },
                    }
                }

                // Final cleanup
                if let Err(e) = Self::cleanup_resources(&mut current_socket, &mut current_process).await {
                    log::error!("Error during final cleanup: {}", e);
                }

                // Set shutdown complete flag
                if let Ok(mut complete) = shutdown_complete.lock() {
                    *complete = true;
                }
            });
        });

        service
    }

    pub async fn start_playback(&self, url: &str, title: &str) -> MpvResult<mpsc::Receiver<MpvEvent>> {
        // Load last position and settings
        let last_position = self.get_last_position(url).await?;
        let settings = self.get_video_settings(url).await?;

        let (response_tx, response_rx) = oneshot::channel();
        self.command_tx.send(MpvCommand::Play {
            url: url.to_string(),
            title: title.to_string(),
            last_position,
            settings,
            response: response_tx,
        }).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        response_rx.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    }

    pub async fn pause(&self) -> MpvResult<()> {
        let (response_tx, response_rx) = oneshot::channel();
        self.command_tx.send(MpvCommand::Pause { response: response_tx })
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        response_rx.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    }

    pub async fn resume(&self) -> MpvResult<()> {
        let (response_tx, response_rx) = oneshot::channel();
        self.command_tx.send(MpvCommand::Resume { response: response_tx })
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        response_rx.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    }

    pub async fn seek(&self, position: f64) -> MpvResult<()> {
        let (response_tx, response_rx) = oneshot::channel();
        self.command_tx.send(MpvCommand::Seek { position, response: response_tx })
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        response_rx.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    }

    pub async fn set_volume(&self, volume: i64) -> MpvResult<()> {
        let (response_tx, response_rx) = oneshot::channel();
        self.command_tx.send(MpvCommand::SetVolume { volume, response: response_tx })
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        response_rx.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    }

    pub async fn stop(&self) -> MpvResult<()> {
        let (response_tx, response_rx) = oneshot::channel();
        self.command_tx.send(MpvCommand::Stop { response: response_tx })
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        response_rx.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    }

    pub async fn play_video(url: &str) -> MpvResult<()> {
        log::info!("Playing video: {}", url);
        
        let mut socket = MpvSocket::connect("mpv")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        
        // Load the file
        socket.set_property(Property::Path, url)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        
        // Start playback
        socket.set_property(Property::Pause, false)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        
        log::debug!("Video playback started");
        Ok(())
    }

    pub async fn shutdown(&self) -> MpvResult<()> {
        self.is_running.store(false, Ordering::SeqCst);
        self.command_tx.send(MpvCommand::Shutdown).await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(())
    }

    // Add config loading/saving
    async fn load_config() -> MpvResult<MpvConfig> {
        if let Ok(data) = std::fs::read_to_string("config/mpv.json") {
            serde_json::from_str(&data)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        } else {
            Ok(MpvConfig::default())
        }
    }

    async fn save_config(config: &MpvConfig) -> MpvResult<()> {
        let data = serde_json::to_string_pretty(config)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        
        std::fs::create_dir_all("config")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            
        std::fs::write("config/mpv.json", data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            
        Ok(())
    }

    // Add position tracking
    async fn save_position(url: &str, position: f64) -> MpvResult<()> {
        let mut config = Self::load_config().await?;
        config.last_positions.insert(url.to_string(), position);
        Self::save_config(&config).await
    }

    async fn get_last_position(&self, url: &str) -> MpvResult<Option<f64>> {
        let config = Self::load_config().await?;
        Ok(config.last_positions.get(url).copied())
    }

    // Add settings tracking
    async fn save_video_settings(&self, url: &str, settings: VideoSettings) -> MpvResult<()> {
        let mut config = Self::load_config().await?;
        config.video_settings.insert(url.to_string(), settings);
        Self::save_config(&config).await
    }

    async fn get_video_settings(&self, url: &str) -> MpvResult<Option<VideoSettings>> {
        let config = Self::load_config().await?;
        Ok(config.video_settings.get(url).cloned())
    }

    async fn cleanup_resources(socket: &mut Option<MpvSocket>, process: &mut Option<std::process::Child>) -> MpvResult<()> {
        if let Some(mut p) = process.take() {
            let _ = p.kill();
            let _ = p.wait();
        }
        if let Some(s) = socket.take() {
            drop(s);
        }
        sleep(Duration::from_millis(100)).await;
        Ok(())
    }

    // Add method to subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<MpvEvent> {
        self.event_tx.subscribe()
    }
}

impl Drop for MpvService {
    fn drop(&mut self) {
        // Set the shutdown flag
        self.is_running.store(false, Ordering::SeqCst);
        
        // Try to send shutdown command without blocking
        let _ = self.command_tx.try_send(MpvCommand::Shutdown);
        
        // Wait for shutdown to complete with timeout
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(1);
        
        while start.elapsed() < timeout {
            if let Ok(complete) = self.shutdown_complete.lock() {
                if *complete {
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

// Usage example:
impl MpvService {
    pub async fn get_or_create() -> MpvService {
        MpvService::instance()
    }
} 