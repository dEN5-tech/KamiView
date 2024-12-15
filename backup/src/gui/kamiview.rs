use iced::{
    event, keyboard, subscription, widget::{button, pick_list, text, Column, Container, Row}, 
    Application, Command, Element, Length, Settings, Subscription, Theme
};

use std::sync::Arc;
use std::collections::HashMap;
use iced_aw::{modal, Card, style::card};

use crate::{
    di,
    gui::{
        screens::{HomeScreen, SearchScreen, SettingsScreen, AnimeDetailsScreen},
        types::{Message, Screen},
        handlers::{subscription_handler, update_handler, handle_navigation},
    },
    gui::components::Sidebar,
    di::interfaces::PlaybackInfo,
};

pub struct KamiView {
    container: Arc<di::Container>,
    current_screen: Screen,
    theme: Theme,
    playback_info: Option<PlaybackInfo>,
    image_cache: HashMap<String, iced::widget::image::Handle>,
    search_screen: Option<SearchScreen>,
    settings_screen: Option<SettingsScreen>,
    show_modal: bool,
}

impl Application for KamiView {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = Arc<di::Container>;
    fn new(container: Arc<di::Container>) -> (Self, Command<Message>) {
        let settings = container.storage().load();
        
        (
            Self {
                container,
                current_screen: Screen::Home,
                theme: settings.theme.into(),
                playback_info: None,
                image_cache: HashMap::new(),
                search_screen: None,
                settings_screen: None,
                show_modal: true,
            },
            Command::none(),
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            subscription_handler::handle_subscription(self.container.clone()),
            subscription::events().map(Message::Event)
        ])
    }

    fn view(&self) -> Element<'_, Message> {
        let content = match &self.current_screen {
            Screen::Home => HomeScreen::view_with_playback(self.playback_info.as_ref()),
            Screen::Search(_) => {
                self.search_screen.as_ref()
                    .map(|screen| screen.view())
                    .unwrap_or_else(|| text("Loading...").into())
            },
            Screen::Settings(_) => {
                self.settings_screen.as_ref()
                    .map(|screen| screen.view())
                    .unwrap_or_else(|| text("Loading...").into())
            },
            Screen::AnimeDetails(args) => AnimeDetailsScreen::new(args.clone()).view(),
        };

        let row = Row::new()
            .push(Sidebar::view(&self.current_screen, self.container.clone()))
            .push(content)
            .spacing(20);

        let container = Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20);

        if self.show_modal {
            let modal_content = Card::new(
                text("Select Translation").size(24),
                Column::new()
                    .push(
                        pick_list(
                            &["Translation 1", "Translation 2", "Translation 3"],
                            Some("Translation 1"),
                            |s: &str| Message::SelectTranslation(s.to_string())
                        )
                    )
                    .push(
                        pick_list(
                            &["Episode 1", "Episode 2", "Episode 3"], 
                            Some("Episode 1"),
                            |s: &str| Message::SelectEpisode(s.to_string(), "default".to_string(), "1".to_string())
                        )
                    )
                    .spacing(10)
            )
            .foot(
                Row::new()
                    .push(button(text("Cancel")).on_press(Message::CloseModal))
                    .push(button(text("Watch")).on_press(Message::Play("".to_string())))
                    .spacing(10)
            )
            .max_width(300.0)
            .on_close(Message::CloseModal)
            .style(card::CardStyles::Primary);

            modal(container, Some(modal_content))
                .backdrop(Message::CloseModal)
                .on_esc(Message::CloseModal)
                .into()
        } else {
            container.into()
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NavigateTo(screen) => {
                self.current_screen = screen.clone();
                handle_navigation(screen, self.container.clone())
            },
            Message::InitializeSearch(args) => {
                self.search_screen = Some(SearchScreen::new(args));
                Command::none()
            },
            Message::InitializeSettings(args) => {
                self.settings_screen = Some(SettingsScreen::new(args));
                Command::none()
            },
            Message::OpenModal(_) => {
                self.show_modal = true;
                self.handle_update(message)
            }
            Message::CloseModal => {
                self.show_modal = false;
                Command::none()
            }
            Message::Event(event) => self.handle_event(event),
            _ => self.handle_update(message)
        }
    }

    fn title(&self) -> String {
        String::from("KamiView")
    }
}

impl KamiView {
    pub fn run(container: Arc<di::Container>) -> iced::Result {
        let settings = Settings {
            window: iced::window::Settings {
                min_size: Some((
                    crate::utils::constants::MIN_WINDOW_WIDTH,
                    crate::utils::constants::MIN_WINDOW_HEIGHT,
                )),
                size: (
                    crate::utils::constants::DEFAULT_WINDOW_WIDTH,
                    crate::utils::constants::DEFAULT_WINDOW_HEIGHT,
                ),
                position: iced::window::Position::Centered,
                ..Default::default()
            },
            flags: container,
            antialiasing: true,
            exit_on_close_request: true,
            id: None,
            default_font: iced::Font::default(),
            default_text_size: 16.0,
        };

        <KamiView as Application>::run(settings)
    }

    fn handle_event(&mut self, event: event::Event) -> Command<Message> {
        match event {
            event::Event::Keyboard(keyboard::Event::KeyPressed {
                key_code: keyboard::KeyCode::Escape,
                ..
            }) => {
                self.show_modal = false;
                handle_navigation(Screen::Home, self.container.clone())
            }
            _ => Command::none()
        }
    }

    fn handle_update(&mut self, message: Message) -> Command<Message> {
        update_handler::handle_update(
            message,
            &mut self.container,
            &mut self.current_screen,
            &mut self.theme,
            &mut self.playback_info,
            &mut self.image_cache,
            &mut self.search_screen,
        )
    }
}
