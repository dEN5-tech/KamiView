use iced::widget::{column, row, Button, text};
use iced::{Element, Length};
use iced::alignment::{Horizontal, Vertical};
use crate::theme::{Theme, ContainerVariant, ButtonVariant};
use crate::Message;
use crate::services::kodik::SearchResult;
use crate::components::common::{Card, Text, TextProps};
use crate::navigation::Screen;
use super::Badge;

pub struct ResultCard;

impl ResultCard {
    pub fn view<'a>(result: &'a SearchResult, theme: &Theme) -> Element<'a, Message> {
        let title = result.display_title().to_string();
        let subtitle = result.display_subtitle();
        let description = result.display_description();
        let badges = result.media_badges()
            .into_iter()
            .map(|badge| (String::from("Type"), badge))
            .collect::<Vec<_>>();

        let result_clone = result.clone();
        
        Button::new(
            Card::view(
                column![
                    Text::view(TextProps {
                        content: title,
                        size: 24,
                        color: theme.text,
                        horizontal_alignment: Horizontal::Left,
                        vertical_alignment: Vertical::Center,
                    }),
                    Text::view(TextProps {
                        content: subtitle,
                        size: 16,
                        color: theme.text_secondary,
                        horizontal_alignment: Horizontal::Left,
                        vertical_alignment: Vertical::Center,
                    }),
                    Text::view(TextProps {
                        content: description,
                        size: 14,
                        color: theme.text_secondary,
                        horizontal_alignment: Horizontal::Left,
                        vertical_alignment: Vertical::Center,
                    }),
                    Badge::row(badges, theme),
                    Self::external_links(result, theme),
                ]
                .spacing(12)
                .into(),
                theme,
                20,
            )
        )
        .style(theme.button(ButtonVariant::Text))
        .width(Length::Fill)
        .on_press(Message::NavigateTo(Screen::Details(result_clone)))
        .into()
    }

    fn external_links<'a>(result: &'a SearchResult, _theme: &Theme) -> Element<'a, Message> {
        row(
            result.external_links()
                .into_iter()
                .map(|(site, url)| {
                    Button::new(text(&site))
                        .width(Length::Shrink)
                        .padding(8)
                        .on_press(Message::OpenUrl(url))
                        .into()
                })
                .collect::<Vec<Element<'_, Message>>>()
        )
        .spacing(8)
        .into()
    }
}