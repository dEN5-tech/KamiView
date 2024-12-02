use iced::widget::{container, row, column, button};
use iced::{Element, Length, Alignment};
use crate::theme::{Theme, ButtonVariant, ContainerVariant};
use crate::components::common::{Text, TextProps};
use crate::Message;
use crate::Tab;

pub struct Sidebar;

impl Sidebar {
    pub fn view<'a>(selected_tab: &Tab, theme: &Theme) -> Element<'a, Message> {
        let nav_buttons = column![
            // Back button
            button(
                row![
                    Text::view(TextProps {
                        content: "←".to_string(), // Unicode arrow or use an icon
                        size: 20,
                        color: theme.text,
                        ..Default::default()
                    }),
                    Text::view(TextProps {
                        content: "Back".to_string(),
                        size: 16,
                        color: theme.text,
                        ..Default::default()
                    })
                ]
                .spacing(10)
                .align_items(Alignment::Center)
            )
            .style(theme.button(ButtonVariant::Text))
            .width(Length::Fill)
            .on_press(Message::GoBack),

            // Existing navigation buttons
            button(
                Text::view(TextProps {
                    content: "Home".to_string(),
                    size: 16,
                    color: if matches!(selected_tab, Tab::Home) {
                        theme.primary
                    } else {
                        theme.text
                    },
                    ..Default::default()
                })
            )
            .style(theme.button(ButtonVariant::Text))
            .width(Length::Fill)
            .on_press(Message::TabSelected(Tab::Home)),

            button(
                Text::view(TextProps {
                    content: "Settings".to_string(),
                    size: 16,
                    color: if matches!(selected_tab, Tab::Settings) {
                        theme.primary
                    } else {
                        theme.text
                    },
                    ..Default::default()
                })
            )
            .style(theme.button(ButtonVariant::Text))
            .width(Length::Fill)
            .on_press(Message::TabSelected(Tab::Settings)),

            button(
                Text::view(TextProps {
                    content: "Profile".to_string(),
                    size: 16,
                    color: if matches!(selected_tab, Tab::Profile) {
                        theme.primary
                    } else {
                        theme.text
                    },
                    ..Default::default()
                })
            )
            .style(theme.button(ButtonVariant::Text))
            .width(Length::Fill)
            .on_press(Message::TabSelected(Tab::Profile)),
        ]
        .spacing(10)
        .padding(20);

        container(nav_buttons)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme.container(ContainerVariant::Box))
            .into()
    }
} 