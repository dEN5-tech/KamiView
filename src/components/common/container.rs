use iced::widget::container;
use iced::{Element, Length};
use iced::theme::Container;

use crate::styles::CustomContainer;
use crate::theme::Theme;
use crate::Message;

pub struct AppContainer;

#[derive(Debug, Clone)]
pub struct ContainerProps {
    pub width: Length,
    pub height: Length,
    pub padding: u16,
    pub center_x: bool,
    pub center_y: bool,
}

impl Default for ContainerProps {
    fn default() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            padding: 0,
            center_x: false,
            center_y: false,
        }
    }
}

impl AppContainer {
    pub fn view<'a>(
        content: Element<'a, Message>,
        props: ContainerProps,
        theme: &Theme,
    ) -> Element<'a, Message> {
        let ContainerProps {
            width,
            height,
            padding,
            center_x,
            center_y,
        } = props;

        let mut container = container(content)
            .width(width)
            .height(height)
            .padding(padding)
            .style(Container::Custom(Box::new(CustomContainer {
                color: theme.background,
                text_color: None,
                hover_color: None,
            })));

        if center_x {
            container = container.center_x();
        }
        if center_y {
            container = container.center_y();
        }

        container.into()
    }
} 