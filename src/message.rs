#[derive(Debug, Clone)]
pub enum Message {
    NavigateToWeb(String),
    // ... other messages
    ImageLoaded(String, Vec<u8>),
    ImageLoadFailed(String),
    EpisodeSelected(i32, String),
    VideoLinkReceived(Result<(String, i32), String>),
} 