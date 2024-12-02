use iced::widget::{column, scrollable, row};
use iced::{Element, Length, ContentFit};

use crate::components::common::{
    Text, TextProps, AppContainer, ContainerProps, 
    IconButton, IconButtonProps, Image, ImageProps
};
use crate::services::post_service::{Post, PostDetails};
use crate::theme::Theme;
use crate::Message;

pub struct PostDetailsScreen;

impl PostDetailsScreen {
    pub fn view<'a>(
        post: &'a Post,
        details: Option<&'a PostDetails>,
        is_loading: bool,
        theme: &Theme
    ) -> Element<'a, Message> {
        let content = if is_loading {
            column![
                Text::view(TextProps {
                    content: "Loading details...",
                    size: 16,
                    color: theme.text,
                })
            ]
        } else if let Some(details) = details {
            column![
                Image::view(
                    ImageProps {
                        url: details.id.to_string(),
                        width: Length::Fill,
                        height: Length::Fill,
                        content_fit: ContentFit::Contain,
                    },
                    theme
                ),
                Text::view(TextProps {
                    content: &details.title,
                    size: 24,
                    color: theme.text,
                }),
                Text::view(TextProps {
                    content: &details.author_text,
                    size: 16,
                    color: theme.text_secondary,
                }),
                Text::view(TextProps {
                    content: &details.body,
                    size: 16,
                    color: theme.text,
                }),
                Text::view(TextProps {
                    content: "Comments",
                    size: 20,
                    color: theme.text,
                }),
                column(
                    details.comments.iter().map(|comment| {
                        column![
                            Text::view(TextProps {
                                content: &comment.name,
                                size: 16,
                                color: theme.text,
                            }),
                            Text::view(TextProps {
                                content: &comment.body,
                                size: 14,
                                color: theme.text_secondary,
                            }),
                        ].spacing(10).into()
                    }).collect()
                ).spacing(20)
            ].spacing(20)
        } else {
            column![
                Image::view(
                    ImageProps {
                        url: post.id.to_string(),
                        width: Length::Fill,
                        height: Length::Fill,
                        content_fit: ContentFit::Contain,
                    },
                    theme
                ),
                Text::view(TextProps {
                    content: &post.title,
                    size: 24,
                    color: theme.text,
                }),
                Text::view(TextProps {
                    content: &post.body,
                    size: 16,
                    color: theme.text,
                }),
            ].spacing(20)
        };

        let container_content = scrollable(
            column![
                row![
                    IconButton::view(
                        IconButtonProps {
                            icon_path: "resources/arrow_back.svg",
                            label: "Back",
                            on_press: Message::GoBack,
                            is_active: false,
                            width: Length::Shrink,
                            padding: 12,
                            size: 16,
                        },
                        theme
                    ),
                ].padding(20),
                content.padding(20),
            ]
        )
        .height(Length::Fill)
        .into();

        AppContainer::view(
            container_content,
            ContainerProps {
                width: Length::Fill,
                height: Length::Fill,
                padding: 0,
                center_x: false,
                center_y: false,
            },
            theme
        )
    }
} 