use std::sync::Once;
use std::sync::Mutex as StdMutex;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use std::time::SystemTime;
use std::sync::Arc;

static INSTANCE: StdMutex<Option<DiscordService>> = StdMutex::new(None);
static INIT: Once = Once::new();

const DISCORD_CLIENT_ID: &str = "YOUR_DISCORD_CLIENT_ID"; // Replace with your Discord app ID

#[derive(Clone)]
pub struct DiscordService {
    client: Arc<StdMutex<Option<DiscordIpcClient>>>,
}

#[derive(Debug)]
pub struct ActivityInfo {
    pub title: String,
    pub episode: Option<i32>,
    pub translation: Option<String>,
    pub state: PlaybackState,
}

#[derive(Debug)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

impl DiscordService {
    pub fn instance() -> DiscordService {
        INIT.call_once(|| {
            let service = DiscordService::new_internal();
            *INSTANCE.lock().unwrap() = Some(service);
        });

        INSTANCE.lock().unwrap().clone().unwrap()
    }

    fn new_internal() -> Self {
        let mut client_instance = DiscordIpcClient::new(DISCORD_CLIENT_ID)
            .map_err(|e| log::error!("Failed to create Discord client: {}", e))
            .ok();

        if let Some(client) = &mut client_instance {
            if let Err(e) = client.connect() {
                log::error!("Failed to connect to Discord: {}", e);
                client_instance = None;
            }
        }

        Self {
            client: Arc::new(StdMutex::new(client_instance)),
        }
    }

    pub fn update_activity(&self, info: ActivityInfo) {
        if let Some(ref mut client) = *self.client.lock().unwrap() {
            let start_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let state = match info.state {
                PlaybackState::Playing => "Playing",
                PlaybackState::Paused => "Paused",
                PlaybackState::Stopped => "Stopped",
            };

            let details = if let Some(ep) = info.episode {
                format!("{} - Episode {}", info.title, ep)
            } else {
                info.title
            };

            let activity = discord_rich_presence::activity::Activity::new()
                .state(state)
                .details(&details)
                .assets(
                    discord_rich_presence::activity::Assets::new()
                        .large_image("app_logo")
                        .large_text("KamiView")
                )
                .timestamps(
                    discord_rich_presence::activity::Timestamps::new()
                        .start(start_time)
                );

            if let Err(e) = client.set_activity(activity) {
                log::error!("Failed to update Discord activity: {}", e);
            }
        }
    }

    pub fn clear_activity(&self) {
        if let Some(ref mut client) = *self.client.lock().unwrap() {
            if let Err(e) = client.clear_activity() {
                log::error!("Failed to clear Discord activity: {}", e);
            }
        }
    }
}

impl Drop for DiscordService {
    fn drop(&mut self) {
        if let Some(ref mut client) = *self.client.lock().unwrap() {
            let _ = client.clear_activity();
            let _ = client.close();
        }
    }
} 