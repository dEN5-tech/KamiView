use std::sync::Arc;
use crate::{
    di::Container,
    gui::types::{Message, Screen, SearchArgs},
    kodik::MediaResult,
};

pub fn handle_search(query: String, container: Arc<Container>) -> Message {
    let args = SearchArgs {
        query,
        results: None,
        error: None,
        container,
    };
    Message::Navigate(Screen::Search(args))
}