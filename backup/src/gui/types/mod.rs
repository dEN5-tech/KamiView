use crate::di::interfaces::PlaybackInfo;
use crate::storage::ThemeType;
use crate::kodik::{InfoResponse, Translation, MediaResult};
use uuid::Uuid;
use iced;
use std::sync::Arc;
use crate::di::Container;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    Navigate(Screen),
    NavigateTo(Screen),
    GoBack,
    InitializeHome,
    
    // Search
    Search(String),
    SearchQueryChanged(String),
    SearchSubmit,
    SearchSubmitted,
    SearchResultsReceived(Vec<MediaResult>),
    SearchFailed(String),
    SearchScreenUpdated,
    SearchPageChanged(u32),
    
    // Playback
    PlaybackUpdate(PlaybackInfo),
    PlaybackProgress(f32),
    PlayEpisode(String, String, String),
    PlayVideo(String),
    Play(String),
    
    // Theme
    ToggleTheme,
    ThemeChanged(ThemeType),
    
    // Settings
    ChangeSettingsTab(SettingsTab),
    SettingsTabChanged(SettingsTab),
    
    // Anime Details
    LoadAnimeDetails(String),
    AnimeDetailsLoaded(InfoResponse),
    AnimeDetailsError(String),
    TranslationsLoaded(Vec<Translation>),
    SelectTranslation(String),
    SelectEpisode(String, String, String),
    
    // Images
    LoadImage(String, Uuid),
    ImageLoaded(String),
    ImageLoadFailed(String),
    
    // Modal
    OpenModal(AnimeDetailsArgs),
    CloseModal,
    
    // Events
    Event(iced::Event),
    None,
    InitializeSearch(SearchArgs),
    InitializeSettings(SettingsArgs),
    InitializeAnimeDetails(AnimeDetailsArgs),
    UpdatePlaybackInfo(PlaybackInfo),
}

#[derive(Debug, Clone)]
pub enum Screen {
    Home,
    Search(SearchArgs),
    Settings(SettingsArgs),
    AnimeDetails(AnimeDetailsArgs),
}

#[derive(Clone)]
pub struct SearchArgs {
    pub query: String,
    pub results: Option<Vec<MediaResult>>,
    pub error: Option<String>,
    pub container: Arc<Container>,
}

impl fmt::Debug for SearchArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SearchArgs")
            .field("query", &self.query)
            .field("results", &self.results)
            .field("error", &self.error)
            .field("container", &"<container>")
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct AnimeDetailsArgs {
    pub anime_id: String,
}

#[derive(Debug, Clone)]
pub struct SettingsArgs {
    pub active_tab: SettingsTab,
    pub current_theme: ThemeType,
}

impl Default for SettingsArgs {
    fn default() -> Self {
        Self {
            active_tab: SettingsTab::default(),
            current_theme: ThemeType::Light,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SettingsTab {
    General,
    Appearance,
    Playback,
    About,
}

impl Default for SettingsTab {
    fn default() -> Self {
        Self::General
    }
}
