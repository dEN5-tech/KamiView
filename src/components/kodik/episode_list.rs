use iced::widget::{row, column, scrollable, button};
use iced::{Element, Length};
use crate::Message;
use crate::theme::{Theme, ButtonVariant};
use crate::components::common::{Text, TextProps};

pub struct EpisodeList;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Episode {
    pub number: i32,
    pub title: String,
    pub translation_id: String,
    pub translation_name: String,
    pub url: String,
}

impl Episode {
    pub fn display_title(&self) -> String {
        if self.title.is_empty() {
            format!("Эпизод {} ({})", self.number, self.translation_name)
        } else {
            format!("Эпизод {} - {} ({})", self.number, self.title, self.translation_name)
        }
    }
}

impl EpisodeList {
    pub fn view<'a>(
        episodes: &[Episode],
        selected: Option<i32>,
        theme: &Theme,
        on_select: impl Fn(&Episode) -> Message + 'a,
    ) -> Element<'a, Message> {
        let episodes_list = episodes.iter().map(|episode| {
            let is_selected = selected == Some(episode.number);
            
            button(
                row![
                    Text::view(TextProps {
                        content: format!("Episode {}", episode.number),
                        size: 16,
                        color: if is_selected { theme.primary } else { theme.text },
                        ..Default::default()
                    }),
                    if !episode.title.is_empty() {
                        Text::view(TextProps {
                            content: episode.title.clone(),
                            size: 14,
                            color: theme.text_secondary,
                            ..Default::default()
                        })
                    } else {
                        Text::view(TextProps {
                            content: String::new(),
                            size: 14,
                            color: theme.text_secondary,
                            ..Default::default()
                        })
                    }
                ]
                .spacing(10)
            )
            .width(Length::Fill)
            .padding(10)
            .style(if is_selected {
                ButtonVariant::Primary.into()
            } else {
                ButtonVariant::Text.into()
            })
            .on_press(on_select(episode))
            .into()
        });

        scrollable(
            column(episodes_list.collect())
                .spacing(5)
                .width(Length::Fill)
        )
        .height(Length::Fill)
        .into()
    }
} 