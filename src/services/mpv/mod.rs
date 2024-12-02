use tokio::sync::{mpsc, oneshot};
use std::io;
use mpv_socket::{MpvSocket, Property};
use std::process::Command as ProcessCommand;
use tokio::time::sleep;
use tokio::time::Duration;
use std::thread;

type Result<T> = std::result::Result<T, io::Error>;

#[derive(Clone)]
pub struct MpvService {
    command_tx: mpsc::Sender<MpvCommand>,
}

#[derive(Debug)]
enum MpvCommand {
    Play { url: String, title: String, response: oneshot::Sender<Result<mpsc::Receiver<MpvEvent>>> },
    Pause { response: oneshot::Sender<Result<()>> },
    Resume { response: oneshot::Sender<Result<()>> },
    Seek { position: f64, response: oneshot::Sender<Result<()>> },
    SetVolume { volume: i64, response: oneshot::Sender<Result<()>> },
    Stop { response: oneshot::Sender<Result<()>> },
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

impl MpvService {
    pub fn new() -> Self {
        let (command_tx, mut command_rx) = mpsc::channel(32);
        let service = Self { command_tx };

        // Spawn a dedicated thread for MPV operations
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let socket_path = r#"\\.\pipe\mpv-socket"#.to_string();
                let mut current_socket: Option<MpvSocket> = None;

                while let Some(cmd) = command_rx.recv().await {
                    match cmd {
                        MpvCommand::Play { url, title, response } => {
                            let result = async {
                                // Stop any existing playback
                                if let Some(socket) = current_socket.take() {
                                    drop(socket); // Close the current socket
                                }

                                // Start new MPV process
                                ProcessCommand::new("mpv")
                                    .args([
                                        &url,
                                        "--idle=yes",
                                        "--cache=yes",
                                        "--prefetch-playlist=yes",
                                        "--demuxer-seekable-cache=yes",
                                        "--demuxer-max-bytes=2048MiB",
                                        "--demuxer-max-back-bytes=1024MiB",
                                        "--demuxer-readahead-secs=1500",
                                        "--demuxer-hysteresis-secs=20",
                                        &format!("--title={}", title),
                                        "--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
                                        "--start=00:00",
                                        "--speed=1",
                                        "--volume=100",
                                        "--mute=no",
                                        "--sub-font-size=55",
                                        "--sub-back-color=#00000000",
                                        "--fullscreen",
                                        "--save-position-on-quit",
                                        &format!("--input-ipc-server={}", socket_path),
                                    ])
                                    .spawn()?;

                                sleep(Duration::from_millis(100)).await;

                                let socket = MpvSocket::connect(&socket_path)
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

                                let (tx, rx) = mpsc::channel(100);
                                current_socket = Some(socket);

                                // Start polling events in the background
                                let event_tx = tx.clone();
                                tokio::spawn(async move {
                                    loop {
                                        sleep(Duration::from_millis(100)).await;
                                        let event = MpvEvent::PropertyChange {
                                            name: "poll".to_string(),
                                            value: "tick".to_string(),
                                        };
                                        if event_tx.send(event).await.is_err() {
                                            break;
                                        }
                                    }
                                });

                                Ok(rx)
                            }.await;

                            let _ = response.send(result);
                        }
                        MpvCommand::Pause { response } => {
                            let result = if let Some(ref mut socket) = current_socket {
                                socket.set_property(Property::Pause, true)
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
                            } else {
                                Ok(())
                            };
                            let _ = response.send(result);
                        }
                        MpvCommand::Resume { response } => {
                            let result = if let Some(ref mut socket) = current_socket {
                                socket.set_property(Property::Pause, false)
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
                            } else {
                                Ok(())
                            };
                            let _ = response.send(result);
                        }
                        MpvCommand::Seek { position, response } => {
                            let result = if let Some(ref mut socket) = current_socket {
                                socket.set_property(Property::PercentPos, position)
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
                            } else {
                                Ok(())
                            };
                            let _ = response.send(result);
                        }
                        MpvCommand::SetVolume { volume, response } => {
                            let result = if let Some(ref mut socket) = current_socket {
                                socket.set_property(Property::Volume, volume)
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
                            } else {
                                Ok(())
                            };
                            let _ = response.send(result);
                        }
                        MpvCommand::Stop { response } => {
                            let result = if let Some(ref mut socket) = current_socket {
                                socket.set_property(Property::Pause, true)
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
                            } else {
                                Ok(())
                            };
                            let _ = response.send(result);
                        }
                    }
                }
            });
        });

        service
    }

    pub async fn start_playback(&self, url: &str, title: &str) -> Result<mpsc::Receiver<MpvEvent>> {
        let (response_tx, response_rx) = oneshot::channel();
        self.command_tx.send(MpvCommand::Play {
            url: url.to_string(),
            title: title.to_string(),
            response: response_tx,
        }).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        response_rx.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    }

    pub async fn pause(&self) -> Result<()> {
        let (response_tx, response_rx) = oneshot::channel();
        self.command_tx.send(MpvCommand::Pause { response: response_tx })
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        response_rx.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    }

    pub async fn resume(&self) -> Result<()> {
        let (response_tx, response_rx) = oneshot::channel();
        self.command_tx.send(MpvCommand::Resume { response: response_tx })
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        response_rx.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    }

    pub async fn seek(&self, position: f64) -> Result<()> {
        let (response_tx, response_rx) = oneshot::channel();
        self.command_tx.send(MpvCommand::Seek { position, response: response_tx })
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        response_rx.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    }

    pub async fn set_volume(&self, volume: i64) -> Result<()> {
        let (response_tx, response_rx) = oneshot::channel();
        self.command_tx.send(MpvCommand::SetVolume { volume, response: response_tx })
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        response_rx.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    }

    pub async fn stop(&self) -> Result<()> {
        let (response_tx, response_rx) = oneshot::channel();
        self.command_tx.send(MpvCommand::Stop { response: response_tx })
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        response_rx.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    }
} 