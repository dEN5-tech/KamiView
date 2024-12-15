use iced::widget::{Container, Column, Button, Space};
use iced::{theme, Element, Length};
use crate::gui::{
    types::{Message, Screen, SearchArgs, SettingsArgs, SettingsTab},
    ButtonStyle, ContainerStyle,
};
use crate::storage::ThemeType;
use std::sync::Arc;
use crate::di::Container as DIContainer;

pub struct Sidebar;

impl Sidebar {
    fn create_nav_button(label: &'static str, screen: Screen, is_selected: bool) -> Button<'static, Message> {
        let style = if is_selected {
            ButtonStyle::Primary
        } else {
            ButtonStyle::Secondary
        };

        Button::new(label)
            .width(Length::Fill)
            .padding(10)
            .style(theme::Button::from(style))
            .on_press(Message::NavigateTo(screen))
    }

    pub fn view<'a>(current_screen: &Screen, container: Arc<DIContainer>) -> Element<'a, Message> {
        let search_args = SearchArgs {
            query: String::new(),
            error: None,
            results: Some(Vec::new()),
            container: container.clone(),
        };

        let settings_args = SettingsArgs {
            active_tab: SettingsTab::General,
            current_theme: ThemeType::Light,
        };

        let content = Column::new()
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(Self::create_nav_button(
                "Home",
                Screen::Home,
                matches!(current_screen, Screen::Home)
            ))
            .push(Space::with_height(Length::Fixed(5.0)))
            .push(Self::create_nav_button(
                "Search",
                Screen::Search(search_args),
                matches!(current_screen, Screen::Search(_))
            ))
            .push(Space::with_height(Length::Fixed(5.0)))
            .push(Self::create_nav_button(
                "Settings",
                Screen::Settings(settings_args),
                matches!(current_screen, Screen::Settings(_))
            ))
            .width(Length::Fixed(200.0))
            .spacing(5)
            .padding(10);

        Container::new(content)
            .style(theme::Container::from(ContainerStyle::Box))
            .into()
    }
} 