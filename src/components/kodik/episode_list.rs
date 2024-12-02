use iced::widget::{container, row, column, button};
use iced::{Element, Length, Alignment};
use crate::theme::{Theme, ButtonVariant, ContainerVariant};
use crate::components::common::{Text, TextProps};
use crate::Message;
use crate::services::kodik::Translation;

#[derive(Debug, Clone)]
pub struct Episode {
    pub number: i32,
    pub title: String,
    pub translation_id: String,
    pub translation_name: String,
    pub url: String,
}

pub struct EpisodeList;

impl EpisodeList {
    pub fn view<'a>(
        episodes: &'a [Episode],
        selected_episode: Option<i32>,
        theme: &'a Theme,
        translations: &'a [Translation],
        selected_translation: Option<&str>,
        on_episode_select: impl Fn(&Episode) -> Message + 'a,
        on_translation_select: impl Fn(&str) -> Message + 'a,
    ) -> Element<'a, Message> {
        // Group episodes by translation
        let mut grouped_episodes: Vec<(&str, &str, Vec<&Episode>)> = Vec::new();
        
        for translation in translations {
            let translation_episodes: Vec<&Episode> = episodes.iter()
                .filter(|ep| ep.translation_id == translation.id)
                .collect();

            if !translation_episodes.is_empty() {
                grouped_episodes.push((
                    &translation.id,
                    &translation.name,
                    translation_episodes
                ));
            }
        }

        column(
            grouped_episodes.iter()
                .map(|(translation_id, translation_name, translation_episodes)| {
                    container(
                        column![
                            // Translation header
                            Text::view(TextProps {
                                content: format!("{} ({} эп.)", translation_name, translation_episodes.len()),
                                size: 16,
                                color: theme.text,
                                ..Default::default()
                            }),

                            // Episodes grid
                            column(
                                translation_episodes.iter()
                                    .map(|episode| {
                                        button(
                                            row![
                                                Text::view(TextProps {
                                                    content: format!("Эпизод {}", episode.number),
                                                    size: 14,
                                                    color: if selected_episode == Some(episode.number) {
                                                        theme.primary
                                                    } else {
                                                        theme.text
                                                    },
                                                    ..Default::default()
                                                }),
                                                if !episode.title.is_empty() {
                                                    Text::view(TextProps {
                                                        content: episode.title.clone(),
                                                        size: 12,
                                                        color: theme.text_secondary,
                                                        ..Default::default()
                                                    })
                                                } else {
                                                    Text::view(TextProps::default())
                                                }
                                            ]
                                            .spacing(10)
                                            .align_items(Alignment::Center)
                                        )
                                        .style(theme.button(if selected_episode == Some(episode.number) {
                                            ButtonVariant::Primary
                                        } else {
                                            ButtonVariant::Secondary
                                        }))
                                        .width(Length::Fill)
                                        .on_press(on_episode_select(episode))
                                        .into()
                                    })
                                    .collect()
                            )
                            .spacing(5)
                        ]
                        .spacing(10)
                        .padding(10)
                    )
                    .style(theme.container(ContainerVariant::Box))
                    .into()
                })
                .collect()
        )
        .spacing(20)
        .into()
    }
} 