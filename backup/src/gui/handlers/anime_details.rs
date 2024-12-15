use iced::Command;
use crate::di::Container;
use crate::gui::types::{Message, Screen};

pub fn handle_load_anime_details(
    container: &std::sync::Arc<Container>,
    shikimori_id: String,
    current_screen: &mut Screen,
) -> Command<Message> {
    let container1 = container.clone();
    let container2 = container.clone();
    let id = shikimori_id.clone();

    Command::batch(vec![
        Command::perform(
            async move {
                container1.kodik().get_anime_info(&id).await
            },
            |result| match result {
                Ok(info) => Message::AnimeDetailsLoaded(info),
                Err(e) => Message::AnimeDetailsError(e.to_string()),
            }
        ),
        Command::perform(
            async move {
                container2.kodik().get_translations(&shikimori_id).await
            },
            |result| match result {
                Ok(translations) => Message::TranslationsLoaded(translations),
                Err(e) => Message::AnimeDetailsError(e.to_string()),
            }
        ),
    ])
}