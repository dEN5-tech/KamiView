#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod styles;
mod components;
mod services;
mod theme;
mod navigation;
mod screens;
mod resources;
mod config;
mod logger;
mod fonts;

use std::sync::Arc;
use tokio::sync::RwLock;

use components::common::{AppContainer, ContainerProps};
use iced::widget::{row, column, toggler};
use iced::{Application, Command, Element, Length, Settings, executor, Theme};
use iced::window;
use components::{
    Sidebar, MainContent, Loader, ErrorView,
    common::Text
};
use services::kodik::{KodikService, SearchResult, Translation};
use theme::{Theme as AppTheme, ThemeVariant};
use navigation::Screen;
use config::CONFIG;
use screens::anime_details::AnimeDetailsScreen;
use open::that;
use components::kodik::episode_list::Episode;
use services::mpv::{MpvService, MpvEvent};
use iced::Font;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tab {
    Home,
    Settings,
    Profile,
}


#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(Tab),
    SearchInputChanged(String),
    SearchKodik,
    KodikResultsReceived(Result<Vec<SearchResult>, String>),
    KodikServiceInitialized(Arc<RwLock<KodikService>>),
    Error(String),
    NavigateTo(Screen),
    GoBack,
    Exit,
    CancelExit,
    Confirm,
    ThemeChanged(ThemeVariant),
    OpenUrl(String),
    ImageLoaded(String, Vec<u8>),
    ImageLoadFailed(String),
    EpisodeSelected(i32, String),
    EpisodesLoaded(Result<Vec<Episode>, String>),
    VideoLinkReceived(Result<(String, i32), String>),
    EpisodesLoadStarted,
    EpisodesLoadFailed(String),
    EpisodesLoadSucceeded(Vec<Episode>),
    TranslationSelected(String),
    VideoLoadStarted,
    VideoLoadFailed(String),
    VideoLoadSucceeded(String, i32),
    TranslationsLoaded(Result<Vec<Translation>, String>),
    PlayVideo(String, String),
    PauseVideo,
    ResumeVideo,
    SeekVideo(f64),
    SetVolume(i64),
    StopVideo,
    MpvEvent(MpvEvent),
}


struct MaterialApp {
    selected_tab: Tab,
    search_input: String,
    theme: AppTheme,
    theme_variant: ThemeVariant,
    is_loading: bool,
    current_screen: Screen,
    navigation_history: Vec<Screen>,
    error: Option<String>,
    kodik_service: Arc<RwLock<KodikService>>,
    kodik_results: Vec<SearchResult>,
    selected_episode: Option<i32>,
    episodes: Vec<Episode>,
    is_loading_episodes: bool,
    current_translation: Option<String>,
    video_loading: bool,
    translations: Vec<Translation>,
    selected_translation: Option<String>,
    mpv_service: MpvService,
}

impl Application for MaterialApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let app = MaterialApp {
            selected_tab: Tab::Home,
            search_input: String::new(),
            theme: AppTheme::default(),
            theme_variant: ThemeVariant::default(),
            is_loading: false,
            current_screen: Screen::Main,
            navigation_history: Vec::new(),
            error: None,
            kodik_service: Arc::new(RwLock::new(KodikService::empty())),
            kodik_results: Vec::new(),
            selected_episode: None,
            episodes: Vec::new(),
            is_loading_episodes: false,
            current_translation: None,
            video_loading: false,
            translations: Vec::new(),
            selected_translation: None,
            mpv_service: MpvService::new(),
        };

        (
            app,
            Command::perform(
                async {
                    KodikService::new(Some(CONFIG.kodik_token.clone())).await
                },
                |result| match result {
                    Ok(service) => Message::KodikServiceInitialized(Arc::new(RwLock::new(service))),
                    Err(e) => Message::Error(e.to_string()),
                }
            )
        )
    }

    fn title(&self) -> String {
        String::from("KamiView")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TabSelected(tab) => {
                self.selected_tab = tab;
                Command::none()
            }
            Message::SearchInputChanged(value) => {
                self.search_input = value;
                Command::none()
            }
            Message::SearchKodik => {
                self.is_loading = true;
                let service = Arc::clone(&self.kodik_service);
                let query = self.search_input.clone();
                Command::perform(
                    async move {
                        let service = service.read().await;
                        service.search(&query, Some(10), false).await
                    },
                    Message::KodikResultsReceived
                )
            }
            Message::KodikResultsReceived(Ok(results)) => {
                self.is_loading = false;
                self.kodik_results = results;
                Command::none()
            }
            Message::KodikResultsReceived(Err(error)) => {
                self.is_loading = false;
                self.error = Some(error);
                Command::none()
            }
            Message::KodikServiceInitialized(service) => {
                self.kodik_service = service;
                Command::none()
            }
            Message::Error(error) => {
                self.error = Some(error);
                Command::none()
            }
            Message::NavigateTo(Screen::Details(result)) => {
                self.navigation_history.push(self.current_screen.clone());
                self.current_screen = Screen::Details(result.clone());
                self.episodes.clear();
                self.selected_episode = None;
                self.translations.clear();
                self.selected_translation = None;
                self.is_loading_episodes = true;
                self.error = None;
                
                let service = self.kodik_service.clone();
                let id = result.shikimori_id.clone();
                
                Command::batch(vec![
                    Command::perform(
                        async move { Message::EpisodesLoadStarted },
                        |msg| msg
                    ),
                    Command::perform(
                        async move {
                            if let Some(id) = id {
                                let service = service.read().await;
                                match service.get_translations(&id).await {
                                    Ok(translations) => Message::TranslationsLoaded(Ok(translations)),
                                    Err(e) => Message::TranslationsLoaded(Err(e)),
                                }
                            } else {
                                Message::TranslationsLoaded(Ok(vec![]))
                            }
                        },
                        |msg| msg
                    ),
                ])
            }
            Message::TranslationsLoaded(Ok(translations)) => {
                self.translations = translations;
                if let Some(first) = self.translations.first() {
                    self.selected_translation = Some(first.id.clone());
                }
                Command::none()
            }
            Message::TranslationsLoaded(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::TranslationSelected(translation_id) => {
                self.selected_translation = Some(translation_id);
                Command::none()
            }
            Message::NavigateTo(screen) => {
                self.navigation_history.push(self.current_screen.clone());
                self.current_screen = screen;
                Command::none()
            }
            Message::GoBack => {
                if let Some(previous_screen) = self.navigation_history.pop() {
                    self.current_screen = previous_screen;
                }
                Command::none()
            }
            Message::Exit => {
                window::close()
            }
            Message::CancelExit => {
                Command::none()
            }
            Message::Confirm => {
                window::close()
            }
            Message::ThemeChanged(variant) => {
                self.theme_variant = variant;
                self.theme = AppTheme::new(variant);
                Command::none()
            }
            Message::ImageLoaded(_url, _bytes) => {
                // Handle loaded image if needed
                Command::none()
            }
            Message::ImageLoadFailed(_url) => {
                // Handle failed image load if needed
                Command::none()
            }
            Message::OpenUrl(url) => {
                if let Err(e) = that(&url) {
                    self.error = Some(format!("Failed to open URL: {}", e));
                }
                Command::none()
            }
            Message::EpisodesLoadStarted => {
                if let Screen::Details(result) = &self.current_screen {
                    if let Some(id) = &result.shikimori_id {
                        let service = self.kodik_service.clone();
                        let id = id.clone();
                        
                        self.is_loading_episodes = true;
                        self.error = None;
                        
                        Command::perform(
                            async move {
                                let service = service.read().await;
                                match service.get_episodes(&id).await {
                                    Ok(episodes) => Message::EpisodesLoadSucceeded(episodes),
                                    Err(e) => Message::EpisodesLoadFailed(e)
                                }
                            },
                            |msg| msg
                        )
                    } else {
                        self.is_loading_episodes = false;
                        self.error = Some("Нет доступных эпизодов".to_string());
                        Command::none()
                    }
                } else {
                    Command::none()
                }
            }
            Message::EpisodesLoadSucceeded(episodes) => {
                let episodes_clone = episodes.clone();
                self.episodes = episodes.into_iter().map(|e| Episode {
                    number: e.number,
                    title: e.title,
                    translation_id: e.translation_id,
                    translation_name: e.translation_name,
                    url: e.url,
                }).collect();
                self.is_loading_episodes = false;
                if episodes_clone.is_empty() {
                    self.error = Some("Нет доступных эпизодов".to_string());
                }
                Command::none()
            }
            Message::EpisodesLoadFailed(error) => {
                self.is_loading_episodes = false;
                self.error = Some(error);
                Command::none()
            }
            Message::EpisodeSelected(number, translation_id) => {
                self.selected_episode = Some(number);
                self.video_loading = true;
                
                if let Screen::Details(ref result) = self.current_screen {
                    if let Some(id) = &result.shikimori_id {
                        let service = self.kodik_service.clone();
                        let id = id.clone();
                        let translation_id = translation_id.clone();
                        let title = result.title.clone();
                        
                        Command::perform(
                            async move {
                                let service = service.read().await;
                                match service.get_video_link(&id, "shikimori", number, &translation_id).await {
                                    Ok((url, quality)) => {
                                        Message::PlayVideo(
                                            url,
                                            format!("{} - Эпизод {}", title, number)
                                        )
                                    }
                                    Err(e) => Message::Error(e),
                                }
                            },
                            |msg| msg
                        )
                    } else {
                        Command::none()
                    }
                } else {
                    Command::none()
                }
            }
            Message::VideoLoadSucceeded(url, quality) => {
                self.video_loading = false;
                // Handle successful video load
                log::info!("Video loaded: {} ({}p)", url, quality);
                Command::none()
            }
            Message::VideoLoadFailed(error) => {
                self.video_loading = false;
                self.error = Some(error);
                Command::none()
            }
            Message::EpisodesLoaded(Ok(episodes)) => {
                log::info!("Episodes loaded: {:?}", episodes);
                self.is_loading_episodes = false;
                if episodes.is_empty() {
                    self.error = Some("Нет доступных эпизодов".to_string());
                } else {
                    self.episodes = episodes;
                }
                Command::none()
            }
            Message::EpisodesLoaded(Err(error)) => {
                self.is_loading_episodes = false;
                self.error = Some(error);
                Command::none()
            }
            Message::VideoLinkReceived(Ok((url, quality))) => {
                self.video_loading = false;
                log::info!("Video loaded: {} ({}p)", url, quality);
                Command::none()
            }
            Message::VideoLinkReceived(Err(error)) => {
                self.video_loading = false;
                self.error = Some(error);
                Command::none()
            }
            Message::VideoLoadStarted => {
                self.video_loading = true;
                Command::none()
            }
            Message::PlayVideo(url, title) => {
                let service = self.mpv_service.clone();
                Command::perform(
                    async move {
                        match service.start_playback(&url, &title).await {
                            Ok(mut rx) => {
                                while let Some(event) = rx.recv().await {
                                    return Message::MpvEvent(event);
                                }
                                Message::VideoLoadSucceeded(url, 0)
                            }
                            Err(e) => Message::Error(e.to_string()),
                        }
                    },
                    |msg| msg
                )
            }
            Message::PauseVideo => {
                let service = self.mpv_service.clone();
                Command::perform(
                    async move {
                        if let Err(e) = service.pause().await {
                            Message::Error(e.to_string())
                        } else {
                            Message::VideoLoadSucceeded("".to_string(), 0)
                        }
                    },
                    |msg| msg
                )
            }
            Message::ResumeVideo => {
                let service = self.mpv_service.clone();
                Command::perform(
                    async move {
                        if let Err(e) = service.resume().await {
                            Message::Error(e.to_string())
                        } else {
                            Message::VideoLoadSucceeded("".to_string(), 0)
                        }
                    },
                    |msg| msg
                )
            }
            Message::SeekVideo(position) => {
                let service = self.mpv_service.clone();
                Command::perform(
                    async move {
                        if let Err(e) = service.seek(position).await {
                            Message::Error(e.to_string())
                        } else {
                            Message::VideoLoadSucceeded("".to_string(), 0)
                        }
                    },
                    |msg| msg
                )
            }
            Message::SetVolume(volume) => {
                let service = self.mpv_service.clone();
                Command::perform(
                    async move {
                        if let Err(e) = service.set_volume(volume).await {
                            Message::Error(e.to_string())
                        } else {
                            Message::VideoLoadSucceeded("".to_string(), 0)
                        }
                    },
                    |msg| msg
                )
            }
            Message::StopVideo => {
                let service = self.mpv_service.clone();
                Command::perform(
                    async move {
                        if let Err(e) = service.stop().await {
                            Message::Error(e.to_string())
                        } else {
                            Message::VideoLoadSucceeded("".to_string(), 0)
                        }
                    },
                    |msg| msg
                )
            }
            Message::MpvEvent(event) => {
                match event {
                    MpvEvent::PropertyChange { name, value } => {
                        log::info!("MPV property changed: {} = {}", name, value);
                    }
                    MpvEvent::PlaybackFinished => {
                        log::info!("Video playback finished");
                    }
                    MpvEvent::Error(e) => {
                        self.error = Some(e);
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match &self.current_screen {
            Screen::Main => {
                let sidebar = Sidebar::view(&self.selected_tab, &self.theme);
                
                let main_content = if self.is_loading {
                    Loader::view(&self.theme)
                } else if let Some(error) = &self.error {
                    ErrorView::view(error, Message::SearchKodik, &self.theme)
                } else {
                    match self.selected_tab {
                        Tab::Settings => {
                            column![
                                Text::title("Theme Settings".to_string(), self.theme.text),
                                toggler(
                                    String::from("Dark Theme"),
                                    matches!(self.theme_variant, ThemeVariant::Dark),
                                    |enabled| Message::ThemeChanged(
                                        if enabled { ThemeVariant::Dark } else { ThemeVariant::Light }
                                    ),
                                )
                                .width(Length::Fill)
                                .text_size(16)
                            ]
                            .spacing(20)
                            .padding(20)
                            .into()
                        }
                        _ => MainContent::view(
                            &self.selected_tab,
                            &self.search_input,
                            &self.kodik_results,
                            &self.theme
                        )
                    }
                };

                
                row![
                    AppContainer::view(
                        sidebar,
                        ContainerProps {
                            width: Length::FillPortion(2),
                            ..ContainerProps::default()
                        },
                        &self.theme
                    ),
                    AppContainer::view(
                        main_content,
                        ContainerProps {
                            width: Length::FillPortion(8),
                            ..ContainerProps::default()
                        },
                        &self.theme
                    ),
                ]
                .spacing(1)
                .into()
            }
            Screen::Details(result) => {
                AnimeDetailsScreen::view(
                    result,
                    self.selected_episode,
                    &self.episodes,
                    self.is_loading_episodes,
                    self.error.as_deref(),
                    &self.theme,
                    &self.translations,
                    self.selected_translation.as_deref(),
                )
            }
        }
    }
}

fn main() -> iced::Result {
    logger::init();
    
    // Initialize fonts
    fonts::init_fonts();
    
    let settings = Settings {
        antialiasing: true,
        default_font: fonts::get_regular_font(),
        default_text_size: 16.0,
        window: window::Settings {
            icon: None,
            ..window::Settings::default()
        },
        ..Settings::default()
    };
    
    MaterialApp::run(settings)
}