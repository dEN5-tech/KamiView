use iced::widget::{container, row};
use iced::Element;
use crate::theme::Theme;
use crate::Message;
use crate::components::common::{Text, TextProps};
use iced::alignment::{Horizontal, Vertical};

pub struct Badge;

impl Badge {
    pub fn row<'a>(badges: Vec<(String, String)>, theme: &Theme) -> Element<'a, Message> {
        let style = theme.badge();
        let style = style.clone();
        
        row(
            badges.into_iter()
                .map(move |(label, value)| {
                    container(
                        Text::view(TextProps {
                            content: format!("{}: {}", label, value),
                            size: 12,
                            color: style.text,
                            horizontal_alignment: Horizontal::Left,
                            vertical_alignment: Vertical::Center,
                        })
                    )
                    .padding(4)
                    .style(style.clone())
                    .into()
                })
                .collect::<Vec<Element<'_, Message>>>()
        )
        .spacing(8)
        .into()
    }
} 