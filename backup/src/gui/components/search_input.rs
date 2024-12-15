use iced::widget::{Button, Row, TextInput};
use iced::Element;
use crate::gui::types::Message;

pub struct SearchInput;

impl SearchInput {
    pub fn view<'a>(value: &str) -> Element<'a, Message> {
        let search_input = TextInput::new(
            "Search anime...",
            value,
        )
        .on_input(Message::SearchQueryChanged)
        .padding(10)
        .size(16);

        let search_button = Button::new("Search")
            .padding(10)
            .on_press(Message::SearchSubmit);

        Row::new()
            .spacing(10)
            .push(search_input)
            .push(search_button)
            .into()
    }
} 