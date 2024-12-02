use iced::widget::{row, column, container, button, pick_list};
use iced::{Element, Length};
use crate::theme::Theme;
use crate::Message;
use crate::services::kodik::{SearchResult, Translation};
use crate::components::common::{Text, TextProps, Card, IconButton, IconButtonProps};
use crate::components::kodik::episode_list::{EpisodeList, Episode};

#[derive(Debug, Clone)]
struct TranslationOption {
    id: String,
    name: String,
}

impl std::fmt::Display for TranslationOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for TranslationOption {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TranslationOption {}

pub struct AnimeDetailsScreen;

impl AnimeDetailsScreen {
    pub fn view<'a>(
        result: &'a SearchResult, 
        selected_episode: Option<i32>,
        episodes: &'a [Episode],
        is_loading: bool,
        error: Option<&str>,
        theme: &'a Theme,
        translations: &'a [Translation],
        selected_translation: Option<&'a str>,
    ) -> Element<'a, Message> {
        let shikimori_id = &result.shikimori_id;

        let back_button = IconButton::view(
            IconButtonProps {
                icon_path: "arrow_back.svg",
                label: "",
                on_press: Message::GoBack,
                is_active: false,
                width: Length::Shrink,
                padding: 10,
                size: 24,
            },
            theme
        );

        let translation_options: Vec<TranslationOption> = translations
            .iter()
            .map(|t| TranslationOption {
                id: t.id.clone(),
                name: format!("{} ({})", t.name, t.translation_type),
            })
            .collect();

        let selected_option = selected_translation.and_then(|id| {
            translation_options
                .iter()
                .find(|t| t.id == id)
                .cloned()
        });

        let content: Element<Message> = if is_loading {
            container(
                Text::view(TextProps {
                    content: "Загрузка эпизодов...".to_string(),
                    size: 16,
                    color: theme.text_secondary,
                    ..Default::default()
                })
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
        } else if let Some(err) = error {
            column![
                Text::view(TextProps {
                    content: err.to_string(),
                    size: 16,
                    color: theme.error,
                    ..Default::default()
                }),
                button::Button::new(
                    Text::view(TextProps {
                        content: "Повторить".to_string(),
                        size: 14,
                        color: theme.text,
                        ..Default::default()
                    })
                )
                .on_press(Message::EpisodesLoadStarted)
                .padding(10)
            ]
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .into()
        } else if episodes.is_empty() {
            container(
                Text::view(TextProps {
                    content: "Нет доступных эпизодов".to_string(),
                    size: 16,
                    color: theme.text_secondary,
                    ..Default::default()
                })
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
        } else {
            column![
                row![
                    Text::view(TextProps {
                        content: "Перевод:".to_string(),
                        size: 14,
                        color: theme.text,
                        ..Default::default()
                    }),
                    pick_list(
                        translation_options,
                        selected_option,
                        |option| Message::TranslationSelected(option.id)
                    )
                    .width(Length::Fill)
                    .padding(10)
                    .style(iced::theme::PickList::Default)
                ]
                .spacing(10)
                .padding(10),
                
                row![
                    container(
                        EpisodeList::view(
                            episodes,
                            selected_episode,
                            &theme,
                            |episode| Message::EpisodeSelected(episode.number, episode.translation_id.clone())
                        )
                    )
                    .width(Length::FillPortion(3))
                    .height(Length::Fill),

                    container(
                        if let Some(episode) = selected_episode {
                            Text::view(TextProps {
                                content: episodes.iter()
                                    .find(|e| e.number == episode)
                                    .map(|e| e.display_title())
                                    .unwrap_or_else(|| format!("Эпизод {}", episode)),
                                size: 16,
                                color: theme.text,
                                ..Default::default()
                            })
                        } else {
                            Text::view(TextProps {
                                content: "Выберите эпизо�� для просмотра".to_string(),
                                size: 16,
                                color: theme.text_secondary,
                                ..Default::default()
                            })
                        }
                    )
                    .width(Length::FillPortion(7))
                    .height(Length::Fill)
                ]
                .spacing(20)
                .padding(20)
            ]
            .into()
        };

        Card::view(
            column![
                // Title and info
                column![
                    row![
                        back_button,
                        column![
                            Text::title(result.display_title().to_string(), theme.text),
                            Text::subtitle(result.display_subtitle().to_string(), theme.text_secondary),
                        ]
                    ]
                    .spacing(10)
                ]
                .spacing(10)
                .padding(20),
            ]
            .push(content)
            .into(),
            theme,
            0
        )
    }
}