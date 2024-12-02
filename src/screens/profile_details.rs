use iced::widget::{button, column, container, row, text_input, toggler};
use iced::{Element, Length, Alignment};
use chrono::{DateTime, Local, TimeZone, Duration};
use crate::theme::{Theme, ButtonVariant, ContainerVariant};
use crate::Message;
use crate::components::common::{Text, TextProps, Card, AppContainer, ContainerProps};
use crate::services::shikimori::UserInfo;
use crate::services::config::AppConfig;

#[derive(Debug, Clone, PartialEq)]
pub enum ProfileState {
    Initial,
    WaitingForCode,
    Loading,
    Authenticated,
    Error(String),
}

#[derive(Debug, Default)]
pub struct ProfileDetailsScreen {
    auth_code: String,
    state: ProfileState,
}

impl Default for ProfileState {
    fn default() -> Self {
        Self::Initial
    }
}

impl ProfileDetailsScreen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_loading(&mut self) {
        self.state = ProfileState::Loading;
    }

    pub fn set_auth_code(&mut self, code: String) {
        self.auth_code = code;
    }

    pub fn set_waiting_for_code(&mut self, waiting: bool) {
        if waiting {
            self.state = ProfileState::WaitingForCode;
        } else {
            self.state = ProfileState::Initial;
            self.auth_code = String::new();
        }
    }

    pub fn set_error(&mut self, error: String) {
        self.state = ProfileState::Error(error);
    }

    pub fn set_authenticated(&mut self) {
        self.state = ProfileState::Authenticated;
    }

    fn format_last_online(timestamp: &str) -> String {
        if let Ok(dt) = DateTime::parse_from_rfc3339(timestamp) {
            let local = dt.with_timezone(&Local);
            let now = Local::now();
            let diff = now.signed_duration_since(local);

            if diff < Duration::minutes(1) {
                "Online now".to_string()
            } else if diff < Duration::hours(1) {
                format!("{} minutes ago", diff.num_minutes())
            } else if diff < Duration::days(1) {
                format!("{} hours ago", diff.num_hours())
            } else if diff < Duration::days(7) {
                format!("{} days ago", diff.num_days())
            } else {
                local.format("%d.%m.%Y %H:%M").to_string()
            }
        } else {
            timestamp.to_string()
        }
    }

    fn format_token_expiry(expires_at: u64) -> String {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if expires_at > now {
            let remaining = expires_at - now;
            if remaining < 3600 {
                format!("Token expires in {} minutes", remaining / 60)
            } else if remaining < 86400 {
                format!("Token expires in {} hours", remaining / 3600)
            } else {
                format!("Token expires in {} days", remaining / 86400)
            }
        } else {
            "Token expired".to_string()
        }
    }

    fn view_content(&self, user_info: Option<&UserInfo>, config: &AppConfig, theme: &Theme) -> Element<Message> {
        let content = match &self.state {
            ProfileState::Loading => column![
                Text::view(TextProps {
                    content: "Loading...".to_string(),
                    size: 20,
                    color: theme.text,
                    ..Default::default()
                })
            ],
            ProfileState::Authenticated if user_info.is_some() => {
                let user = user_info.unwrap();
                column![
                    // User header with status
                    container(
                        row![
                            column![
                                Text::view(TextProps {
                                    content: format!("Welcome, {}!", user.nickname),
                                    size: 24,
                                    color: theme.text,
                                    ..Default::default()
                                }),
                                row![
                                    Text::view(TextProps {
                                        content: "ID: ".to_string(),
                                        size: 14,
                                        color: theme.text_secondary,
                                        ..Default::default()
                                    }),
                                    Text::view(TextProps {
                                        content: user.id.to_string(),
                                        size: 14,
                                        color: theme.primary,
                                        ..Default::default()
                                    }),
                                ]
                            ],
                            column![
                                Text::view(TextProps {
                                    content: "Status".to_string(),
                                    size: 14,
                                    color: theme.text_secondary,
                                    ..Default::default()
                                }),
                                Text::view(TextProps {
                                    content: Self::format_last_online(&user.last_online_at),
                                    size: 14,
                                    color: if Self::format_last_online(&user.last_online_at) == "Online now" {
                                        theme.primary
                                    } else {
                                        theme.text
                                    },
                                    ..Default::default()
                                })
                            ]
                            .align_items(Alignment::End)
                        ]
                        .spacing(20)
                        .width(Length::Fill)
                    )
                    .style(theme.container(ContainerVariant::Box))
                    .padding(15),

                    // Settings section
                    container(
                        column![
                            Text::view(TextProps {
                                content: "Account Settings".to_string(),
                                size: 18,
                                color: theme.text,
                                ..Default::default()
                            }),
                            row![
                                Text::view(TextProps {
                                    content: "Auto-login".to_string(),
                                    size: 14,
                                    color: theme.text,
                                    ..Default::default()
                                }),
                                toggler(
                                    String::new(),
                                    config.auto_auth,
                                    Message::ToggleAutoAuth
                                )
                            ]
                            .spacing(10)
                            .align_items(Alignment::Center),
                            if let Some(last_login) = &config.last_login {
                                Text::view(TextProps {
                                    content: format!("Last login: {}", Self::format_last_online(last_login)),
                                    size: 12,
                                    color: theme.text_secondary,
                                    ..Default::default()
                                })
                            } else {
                                Text::view(TextProps {
                                    content: String::new(),
                                    size: 12,
                                    color: theme.text_secondary,
                                    ..Default::default()
                                })
                            }
                        ]
                        .spacing(10)
                    )
                    .style(theme.container(ContainerVariant::Box))
                    .padding(15),

                    // Logout button
                    button(
                        Text::view(TextProps {
                            content: "Logout".to_string(),
                            size: 14,
                            color: theme.text,
                            ..Default::default()
                        })
                    )
                    .style(theme.button(ButtonVariant::Error))
                    .width(Length::Fill)
                    .on_press(Message::Logout)
                ]
                .spacing(20)
            },
            ProfileState::Error(error) => column![
                Text::view(TextProps {
                    content: error.clone(),
                    size: 20,
                    color: theme.error,
                    ..Default::default()
                }),
                button(
                    Text::view(TextProps {
                        content: "Try Again".to_string(),
                        size: 16,
                        color: theme.text,
                        ..Default::default()
                    })
                )
                .style(theme.button(ButtonVariant::Primary))
                .on_press(Message::StartAuth)
            ],
            ProfileState::WaitingForCode => column![
                Text::view(TextProps {
                    content: "Enter the authorization code from Shikimori".to_string(),
                    size: 20,
                    color: theme.text,
                    ..Default::default()
                }),
                text_input(
                    "Enter authorization code",
                    &self.auth_code,
                )
                .width(Length::Fixed(300.0))
                .padding(10)
                .size(16)
                .on_input(Message::AuthCodeEntered)
                .style(iced::theme::TextInput::Default),
                row![
                    button(
                        Text::view(TextProps {
                            content: "Cancel".to_string(),
                            size: 16,
                            color: theme.text,
                            ..Default::default()
                        })
                    )
                    .style(theme.button(ButtonVariant::Secondary))
                    .on_press(Message::CancelAuth),
                    button(
                        Text::view(TextProps {
                            content: "Submit".to_string(),
                            size: 16,
                            color: theme.text,
                            ..Default::default()
                        })
                    )
                    .style(theme.button(ButtonVariant::Primary))
                    .on_press(Message::OAuthCallback(self.auth_code.clone()))
                ]
                .spacing(10)
            ],
            _ => column![
                Text::view(TextProps {
                    content: "Please log in to continue".to_string(),
                    size: 20,
                    color: theme.text,
                    ..Default::default()
                }),
                button(
                    row![
                        Text::view(TextProps {
                            content: "Login with Shikimori".to_string(),
                            size: 16,
                            color: theme.text,
                            ..Default::default()
                        })
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center)
                )
                .style(theme.button(ButtonVariant::Primary))
                .on_press(Message::StartAuth)
            ]
        };

        content
            .spacing(20)
            .align_items(Alignment::Center)
            .into()
    }

    pub fn view(&self, is_authenticated: bool, user_info: Option<&UserInfo>, config: &AppConfig, theme: &Theme) -> Element<Message> {
        AppContainer::view(
            Card::view(
                container(self.view_content(user_info, config, theme))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .style(theme.container(ContainerVariant::Box))
                    .into(),
                theme,
                20
            ),
            ContainerProps {
                width: Length::Fill,
                height: Length::Fill,
                padding: 20,
                center_x: true,
                center_y: true,
            },
            theme
        )
    }
} 