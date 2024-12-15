use std::sync::Arc;
use iced::subscription;
use crate::{
    di::{Container, interfaces::PlaybackInfo},
    gui::types::Message,
};

pub fn handle_subscription(container: Arc<Container>) -> iced::Subscription<Message> {
    subscription::unfold(
        "playback_subscription",
        container,
        move |container| async move {
            let playback_info = container.mpv().get_playback_info()
                .unwrap_or_else(|_| PlaybackInfo {
                    position: 0.0,
                    duration: 0.0,
                    paused: true
                });
            (Message::UpdatePlaybackInfo(playback_info), container)
        },
    )
}