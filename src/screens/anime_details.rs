use iced::widget::{container, row, column, button, scrollable};
use iced::{Element, Length, Alignment};
use crate::theme::{Theme, ButtonVariant, ContainerVariant};
use crate::components::common::{Text, TextProps};
use crate::components::Sidebar;
use crate::Message;
use crate::components::kodik::episode_list::{Episode, EpisodeList};
use crate::services::kodik::{SearchResult, Translation};
use crate::Tab;
use regex::Regex;
use crate::services::mpv::{MpvService};

pub fn view<'a>(
    result: &'a SearchResult,
    selected_episode: Option<i32>,
    episodes: &'a [Episode],
    is_loading: bool,
    error: Option<&'a str>,
    theme: &'a Theme,
    translations: &'a [Translation],
    selected_translation: Option<&'a str>,
) -> Element<'a, Message> {
    // Helper function to extract episode count from translation name
    fn get_episode_count(name: &str) -> Option<usize> {
        let re = Regex::new(r"\((\d+) эп\.\)").unwrap();
        re.captures(name)
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse().ok())
    }

    let episodes_view = if is_loading {
        column![
            Text::view(TextProps {
                content: "Загрузка эпизодов...".to_string(),
                size: 16,
                color: theme.text,
                ..Default::default()
            })
        ].into()
    } else if let Some(error) = error {
        column![
            Text::view(TextProps {
                content: error.to_string(),
                size: 16,
                color: theme.error,
                ..Default::default()
            })
        ].into()
    } else {
        let scrollable_content: Element<'_, Message> = scrollable(
            EpisodeList::view(
                episodes,
                selected_episode,
                theme,
                translations,
                selected_translation,
                |ep| Message::EpisodeSelected(ep.number, ep.translation_id.clone()),
                |id| Message::TranslationSelected(id.to_string()),
            )
        )
        .height(Length::Fill)
        .into();
        scrollable_content
    };

    row![
        // Global sidebar
        Sidebar::view(&Tab::Home, theme),

        // Main content
        container(
            column![
                // Header section with title and info
                container(
                    row![
                        // Left side - Title and basic info
                        column![
                            Text::view(TextProps {
                                content: result.title.clone(),
                                size: 24,
                                color: theme.text,
                                ..Default::default()
                            }),
                            Text::view(TextProps {
                                content: format!("Год: {}", result.year),
                                size: 14,
                                color: theme.text_secondary,
                                ..Default::default()
                            }),
                        ]
                        .spacing(10)
                        .width(Length::Fill),

                        // Right side - Additional info
                        column![
                            Text::view(TextProps {
                                content: format!("ID: {}", result.shikimori_id.as_ref().map(|id| id.to_string()).unwrap_or_default()),
                                size: 16,
                                color: theme.text_secondary,
                                ..Default::default()
                            }),
                        ]
                        .align_items(Alignment::End)
                    ]
                    .padding(20)
                    .spacing(20)
                )
                .style(theme.container(ContainerVariant::Box)),

                // Episodes and translations section
                row![
                    // Left side - Episodes list
                    container(episodes_view)
                        .width(Length::FillPortion(7))
                        .style(theme.container(ContainerVariant::Box)),

                    // Right side - Translations list
                    container(
                        column![
                            Text::view(TextProps {
                                content: format!("Озвучки ({})", translations.len()),
                                size: 18,
                                color: theme.text,
                                ..Default::default()
                            }),
                            column(
                                translations
                                    .iter()
                                    .map(|t| {
                                        let backend_count = get_episode_count(&t.name).unwrap_or(0);
                                        let actual_count = episodes.iter()
                                            .filter(|ep| ep.translation_id == t.id)
                                            .count();
                                        
                                        button(
                                            row![
                                                Text::view(TextProps {
                                                    content: t.name.clone(),
                                                    size: 14,
                                                    color: if Some(&t.id) == selected_translation.map(String::from).as_ref() {
                                                        theme.primary
                                                    } else {
                                                        theme.text
                                                    },
                                                    ..Default::default()
                                                }),
                                                if actual_count > 0 && actual_count != backend_count {
                                                    Text::view(TextProps {
                                                        content: format!("({}/{})", actual_count, backend_count),
                                                        size: 12,
                                                        color: theme.text_secondary,
                                                        ..Default::default()
                                                    })
                                                } else {
                                                    Text::view(TextProps {
                                                        content: format!("({})", backend_count),
                                                        size: 12,
                                                        color: theme.text_secondary,
                                                        ..Default::default()
                                                    })
                                                }
                                            ]
                                            .spacing(5)
                                            .align_items(Alignment::Center)
                                        )
                                        .style(theme.button(if Some(&t.id) == selected_translation.map(String::from).as_ref() {
                                            ButtonVariant::Primary
                                        } else {
                                            ButtonVariant::Secondary
                                        }))
                                        .width(Length::Fill)
                                        .on_press(Message::TranslationSelected(t.id.clone()))
                                        .into()
                                    })
                                    .collect()
                            )
                            .spacing(10)
                        ]
                        .spacing(20)
                        .padding(20)
                    )
                    .width(Length::FillPortion(3))
                    .style(theme.container(ContainerVariant::Box))
                ]
                .spacing(20)
                .height(Length::Fill)
            ]
            .spacing(20)
            .padding(20)
        )
        .width(Length::FillPortion(8))
        .style(theme.container(ContainerVariant::Box))
    ]
    .spacing(1)
    .into()
}