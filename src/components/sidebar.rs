use iced::widget::column;
use iced::{Element, Length};

use crate::theme::Theme;
use crate::Tab;
use crate::Message;
use crate::components::common::{IconButton, IconButtonProps, AppContainer, ContainerProps};

pub struct Sidebar;

impl Sidebar {
    pub fn view<'a>(selected_tab: &'a Tab, theme: &Theme) -> Element<'a, Message> {
        let sidebar_width = 250;

        AppContainer::view(
            column![
                IconButton::view(
                    IconButtonProps {
                        icon_path: "resources/home.svg",
                        label: "Home",
                        on_press: Message::TabSelected(Tab::Home),
                        is_active: *selected_tab == Tab::Home,
                        width: Length::Fill,
                        padding: 12,
                        size: 16,
                    },
                    theme
                ),
                IconButton::view(
                    IconButtonProps {
                        icon_path: "resources/settings.svg",
                        label: "Settings",
                        on_press: Message::TabSelected(Tab::Settings),
                        is_active: *selected_tab == Tab::Settings,
                        width: Length::Fill,
                        padding: 12,
                        size: 16,
                    },
                    theme
                ),
                IconButton::view(
                    IconButtonProps {
                        icon_path: "resources/profile.svg",
                        label: "Profile",
                        on_press: Message::TabSelected(Tab::Profile),
                        is_active: *selected_tab == Tab::Profile,
                        width: Length::Fill,
                        padding: 12,
                        size: 16,
                    },
                    theme
                ),
            ]
            .spacing(8)
            .padding(20)
            .into(),
            ContainerProps {
                width: Length::Fixed(sidebar_width as f32),
                height: Length::Fill,
                padding: 0,
                center_x: false,
                center_y: false,
            },
            theme
        )
    }
} 