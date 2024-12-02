use iced::widget::{column, scrollable};
use iced::{Element, Length, Renderer};
use crate::theme::Theme;
use crate::{Tab, Message};
use crate::services::kodik::SearchResult;
use crate::components::common::{Text, AppContainer, ContainerProps};
use crate::components::kodik::{KodikSearch, ResultCard};

pub struct MainContent;

impl MainContent {
    pub fn view<'a>(
        selected_tab: &'a Tab,
        input_value: &'a str,
        kodik_results: &'a [SearchResult],
        theme: &Theme,
    ) -> Element<'a, Message> {
        let content = match selected_tab {
            Tab::Home => {
                scrollable(
                    column![
                        KodikSearch::view(input_value, theme),
                        if !kodik_results.is_empty() {
                            column(
                                kodik_results.iter()
                                    .map(|result| ResultCard::view(result, theme))
                                    .collect()
                            )
                            .spacing(20)
                        } else {
                            column::<Message, Renderer>(Vec::new())
                                .spacing(20)
                        },
                    ]
                    .spacing(20)
                    .padding(20)
                )
                .height(Length::Fill)
                .into()
            }
            Tab::Settings => scrollable(
                column![
                    Text::title("Settings Page".to_string(), theme.text)
                ]
            )
            .height(Length::Fill)
            .into(),
            Tab::Profile => scrollable(
                column![
                    Text::title("Profile Page".to_string(), theme.text)
                ]
            )
            .height(Length::Fill)
            .into(),
        };

        AppContainer::view(content, ContainerProps::default(), theme)
    }
}