use iced::Command;
use log::error;
use crate::gui::types::Message;
use crate::di::Container;
use crate::di::interfaces::PlaybackInfo as DomainPlaybackInfo;

pub fn handle_play_video(
    container: &std::sync::Arc<Container>,
    url: String,
) -> Command<Message> {
    if let Err(e) = container.mpv().play(&url) {
        error!("Failed to play video: {}", e);
    }
    Command::none()
}

pub fn handle_playback_progress(
    info: DomainPlaybackInfo,
    playback_info: &mut Option<DomainPlaybackInfo>,
) -> Command<Message> {
    *playback_info = Some(info);
    Command::none()
} 