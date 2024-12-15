use std::sync::Arc;
use iced::Command;

use crate::{
    di::Container,
    gui::types::{Message, Screen},
};

pub fn handle_navigation(screen: Screen, container: Arc<Container>) -> Command<Message> {
    match screen {
        Screen::Home => Command::perform(async {}, |_| Message::InitializeHome),
        Screen::Search(args) => Command::perform(async { args }, Message::InitializeSearch),
        Screen::Settings(args) => Command::perform(async { args }, Message::InitializeSettings),
        Screen::AnimeDetails(args) => Command::perform(async { args }, Message::InitializeAnimeDetails),
    }
} 