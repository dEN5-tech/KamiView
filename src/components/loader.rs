use iced::{Element, Length};
use iced::alignment::{Horizontal, Vertical};

use crate::theme::Theme;
use crate::Message;
use crate::components::common::{Text, TextProps, AppContainer, ContainerProps};

pub struct Loader;

impl Loader {
    pub fn view<'a>(theme: &Theme) -> Element<'a, Message> {
        AppContainer::view(
            Text::view(TextProps {
                content: "Loading...".to_string(),
                size: 20,
                color: theme.text,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
            }),
            ContainerProps {
                width: Length::Fill,
                height: Length::Fill,
                padding: 0,
                center_x: true,
                center_y: true,
            },
            theme
        )
    }
} 