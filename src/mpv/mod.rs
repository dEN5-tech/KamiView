use serde_json::Value;
use shaku::Component;
use crate::di::interfaces::{IMpvClient, PlaybackInfo};
use std::sync::{Arc, Mutex};
use std::process::{Command, Child};
use std::time::{Duration, Instant};
use tokio::sync::mpsc::{self, Sender, Receiver};
use anyhow::Result;
use mpv_socket::Property;
use std::path::Path;
use std::fs;
use log::{error, info, debug};

// Message types for the MPV handler thread
#[derive(Debug)]
pub enum MpvMessage {
    Play(String),
    GetPlaybackInfo(tokio::sync::oneshot::Sender<Result<PlaybackInfo>>),
    Pause,
    Resume,
    Stop,
    Shutdown,
}

// Add Debug derive to MpvInstance
struct MpvInstance {
    process: Child,
    socket: mpv_socket::MpvSocket,
}

impl std::fmt::Debug for MpvInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MpvInstance")
            .field("process_id", &self.process.id())
            .finish()
    }
}

// Add new state tracking struct
#[derive(Debug)]
struct MpvState {
    #[allow(dead_code)]
    instance: MpvInstance,
    last_successful_check: Instant,
    consecutive_failures: u32,
}

impl MpvState {
    fn new(instance: MpvInstance) -> Self {
        Self {
            instance,
            last_successful_check: Instant::now(),
            consecutive_failures: 0,
        }
    }

    fn record_success(&mut self) {
        self.last_successful_check = Instant::now();
        self.consecutive_failures = 0;
    }

    fn record_failure(&mut self) -> bool {
        self.consecutive_failures += 1;
        // Return true if we should cleanup (3 consecutive failures or 5 seconds since last success)
        self.consecutive_failures >= 3 || 
        self.last_successful_check.elapsed() > Duration::from_secs(5)
    }
}

#[derive(Component)]
#[shaku(interface = IMpvClient)]
pub struct MpvClient {
    #[shaku(default = String::new())]
    socket_path: String,
    #[shaku(default = Arc::new(Mutex::new(None::<Sender<MpvMessage>>)))]
    sender: Arc<Mutex<Option<Sender<MpvMessage>>>>,
}

impl MpvClient {
    async fn ensure_mpv_handler(&self) -> Result<Sender<MpvMessage>> {
        let mut sender_guard = self.sender.lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock sender"))?;

        if sender_guard.is_none() {
            // Ensure socket directory exists
            if let Some(parent) = Path::new(&self.socket_path).parent() {
                fs::create_dir_all(parent)?;
            }

            // Remove existing socket if any
            let _ = fs::remove_file(&self.socket_path);

            let (tx, rx) = mpsc::channel(32);
            let socket_path = self.socket_path.clone();

            // Move rx into the spawned task
            tokio::task::spawn_blocking(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(Self::run_mpv_handler(socket_path, rx));
            });

            // Wait for MPV handler to initialize
            tokio::time::sleep(Duration::from_millis(100)).await;
            *sender_guard = Some(tx);
        }

        Ok(sender_guard.as_ref().unwrap().clone())
    }

    async fn run_mpv_handler(socket_path: String, mut rx: Receiver<MpvMessage>) {
        let mut current_state: Option<MpvState> = None;

        while let Some(msg) = rx.recv().await {
            match msg {
                MpvMessage::Play(url) => {
                    // Cleanup previous instance if exists
                    if let Some(state) = current_state.take() {
                        Self::cleanup_instance(state.instance).await;
                    }

                    match Self::create_mpv_instance(&socket_path, &url).await {
                        Ok(instance) => {
                            current_state = Some(MpvState::new(instance));
                        }
                        Err(e) => error!("Failed to start MPV: {}", e),
                    }
                },
                MpvMessage::GetPlaybackInfo(response) => {
                    let result = if let Some(ref mut state) = current_state {
                        match Self::get_playback_info(&mut state.instance.socket) {
                            Ok(info) => {
                                state.record_success();
                                Ok(info)
                            },
                            Err(e) => {
                                if state.record_failure() {
                                    debug!("MPV state degraded, cleaning up");
                                    if let Some(state) = current_state.take() {
                                        Self::cleanup_instance(state.instance).await;
                                    }
                                    Ok(PlaybackInfo::default())
                                } else {
                                    // Return last known good state or error if critical
                                    if e.to_string().contains("socket connection lost") {
                                        Ok(PlaybackInfo::default())
                                    } else {
                                        Err(e)
                                    }
                                }
                            }
                        }
                    } else {
                        Ok(PlaybackInfo::default())
                    };
                    let _ = response.send(result);
                },
                MpvMessage::Pause => {
                    if let Some(ref mut state) = current_state {
                        let _ = state.instance.socket.set_property(Property::Pause, true);
                    }
                },
                MpvMessage::Resume => {
                    if let Some(ref mut state) = current_state {
                        let _ = state.instance.socket.set_property(Property::Pause, false);
                    }
                },
                MpvMessage::Stop => {
                    if let Some(state) = current_state.take() {
                        Self::cleanup_instance(state.instance).await;
                    }
                },
                MpvMessage::Shutdown => break,
            }
        }

        // Final cleanup
        if let Some(state) = current_state {
            Self::cleanup_instance(state.instance).await;
        }
        let _ = fs::remove_file(&socket_path);
    }

    async fn create_mpv_instance(socket_path: &str, url: &str) -> Result<MpvInstance> {
        // Kill any existing MPV processes first
        #[cfg(target_os = "windows")]
        let _ = Command::new("taskkill")
            .args(["/F", "/IM", "mpv.exe"])
            .output();
        
        #[cfg(unix)]
        let _ = Command::new("pkill")
            .arg("mpv")
            .output();

        // Remove existing socket
        let _ = fs::remove_file(socket_path);
        
        debug!("Starting MPV with URL: {}", url);
        
        // Add mut to process
        let mut process = Command::new("mpv")
            .arg(format!("--input-ipc-server={}", socket_path))
            .arg("--force-window=yes")
            .arg("--keep-open=yes")
            .arg("--fs")
            .arg("--no-terminal")
            .arg("--msg-level=all=no")
            .arg("--cache=yes")
            .arg("--cache-secs=30")
            .arg("--demuxer-max-bytes=500M")
            .arg("--demuxer-readahead-secs=20")
            .arg("--user-agent=Mozilla/5.0")
            .arg("--no-ytdl")
            .arg("--profile=low-latency")
            .arg("--idle=yes")
            .arg("--input-default-bindings=yes")
            .arg("--input-vo-keyboard=yes")
            .arg("--osc=yes")
            .arg("--no-input-terminal")
            .arg("--no-config")
            .arg(url)
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to start MPV: {}", e))?;

        info!("MPV process started, waiting for socket...");

        // Wait longer for socket initialization
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Wait for socket with better retry logic
        let mut retries = 20;
        let retry_delay = Duration::from_millis(200);
        let mut last_error = None;

        while retries > 0 {
            if Path::new(socket_path).exists() {
                match mpv_socket::MpvSocket::connect(socket_path) {
                    Ok(mut socket) => {
                        info!("Successfully connected to MPV socket");
                        
                        // Initialize socket properties
                        let _ = socket.set_property(Property::Pause, false);
                        let _ = socket.set_property(Property::Volume, 100);
                        
                        // Verify connection is working
                        match socket.get_property::<bool>(Property::Pause) {
                            Ok(_) => {
                                let instance = MpvInstance { process, socket };
                                return Ok(instance);
                            }
                            Err(e) => {
                                last_error = Some(e);
                                debug!("Socket connection test failed, retrying...");
                            }
                        }
                    }
                    Err(e) => {
                        last_error = Some(e);
                        debug!("Failed to connect to socket, retrying... ({})", retries);
                    }
                }
            }
            tokio::time::sleep(retry_delay).await;
            retries -= 1;
        }

        // If we get here, we failed to connect
        error!("Failed to connect to MPV socket after multiple attempts");
        if let Some(e) = last_error {
            error!("Last error: {}", e);
        }
        
        // Now process is mutable, so we can kill it
        let _ = process.kill();
        
        Err(anyhow::anyhow!("Failed to establish stable MPV socket connection"))
    }

    async fn cleanup_instance(mut instance: MpvInstance) {
        debug!("Cleaning up MPV instance");
        
        // Try to pause first to prevent any buffering
        let _ = instance.socket.set_property(Property::Pause, true);
        
        // Try to stop playback
        let _ = instance.socket.set_property(Property::Path, Value::Null);
        
        // Give it a moment to stop
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Force kill if still running
        let _ = instance.process.kill();
        drop(instance.socket);
        
        debug!("MPV instance cleanup completed");
    }

    fn get_playback_info(socket: &mut mpv_socket::MpvSocket) -> Result<PlaybackInfo> {
        // First verify socket is still alive
        match socket.get_property::<bool>(Property::Pause) {
            Ok(_) => {
                // Socket is working, get playback info
                let position = socket.get_property(Property::TimePos)
                    .map_err(|e| anyhow::anyhow!("Failed to get position: {}", e))?;
                let duration = socket.get_property(Property::Duration)
                    .map_err(|e| anyhow::anyhow!("Failed to get duration: {}", e))?;
                let paused = socket.get_property(Property::Pause)
                    .map_err(|e| anyhow::anyhow!("Failed to get pause state: {}", e))?;

                Ok(PlaybackInfo {
                    position,
                    duration,
                    paused,
                })
            }
            Err(e) => {
                Err(anyhow::anyhow!("MPV socket connection lost: {}", e))
            }
        }
    }
}

impl IMpvClient for MpvClient {
    fn play(&self, url: &str) -> Result<()> {
        let sender = futures::executor::block_on(self.ensure_mpv_handler())?;
        futures::executor::block_on(sender.send(MpvMessage::Play(url.to_string())))
            .map_err(|e| anyhow::anyhow!("Failed to send play command: {}", e))
    }

    fn get_playback_info(&self) -> Result<PlaybackInfo> {
        let sender = futures::executor::block_on(self.ensure_mpv_handler())?;
        let (tx, rx) = tokio::sync::oneshot::channel();
        
        match futures::executor::block_on(sender.send(MpvMessage::GetPlaybackInfo(tx))) {
            Ok(_) => match futures::executor::block_on(rx) {
                Ok(result) => result,
                Err(_) => Ok(PlaybackInfo::default())
            },
            Err(_) => Ok(PlaybackInfo::default())
        }
    }
}

impl Drop for MpvClient {
    fn drop(&mut self) {
        if let Ok(guard) = self.sender.lock() {
            if let Some(sender) = guard.as_ref() {
                let _ = futures::executor::block_on(sender.send(MpvMessage::Shutdown));
            }
        }
    }
}

// Add Default implementation for PlaybackInfo
impl Default for PlaybackInfo {
    fn default() -> Self {
        Self {
            position: 0.0,
            duration: 0.0,
            paused: true,
        }
    }
}
