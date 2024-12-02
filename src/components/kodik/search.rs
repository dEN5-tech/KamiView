use iced::widget::{column, row, Button, TextInput, Text};
use iced::{Element, Length};
use crate::theme::Theme;
use crate::Message;
use crate::fonts;

pub struct KodikSearch;

impl KodikSearch {
    pub fn view<'a>(input_value: &str, theme: &Theme) -> Element<'a, Message> {
        column![
            row![
                TextInput::new(
                    "Search anime...",
                    input_value
                )
                .on_input(Message::SearchInputChanged)
                .width(Length::Fill)
                .padding(12)
                .size(14)
                .font(fonts::get_regular_font())
                .style(iced::theme::TextInput::Default),
                Button::new(
                    Text::new("Search")
                        .font(fonts::get_regular_font())
                        .size(14)
                        .style(iced::theme::Text::Color(theme.text_on_primary))
                )
                .width(Length::Shrink)
                .padding(12)
                .style(iced::theme::Button::Custom(Box::new(theme.button(true, false))))
                .on_press(Message::SearchKodik)
            ]
            .spacing(8)
        ]
        .spacing(20)
        .into()
    }
}