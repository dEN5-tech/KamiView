use iced::widget::{container, button, text, row};
use iced::{Element, Length};
use crate::Message;
use crate::theme::{Theme, ButtonVariant};

pub struct ErrorView;

impl ErrorView {
    pub fn view<'a>(error: &str, retry_message: Message, theme: &Theme) -> Element<'a, Message> {
        container(
            row![
                text(error)
                    .size(16)
                    .style(iced::theme::Text::Color(theme.error)),
                button("Повторить")
                    .on_press(retry_message)
                    .style(theme.button(ButtonVariant::Error))
            ]
            .spacing(20)
        )
        .width(Length::Fill)
        .padding(20)
        .center_x()
        .center_y()
        .into()
    }
} 