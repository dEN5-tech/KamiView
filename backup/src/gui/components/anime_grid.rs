use iced::{
    widget::{container, scrollable, Row},
    Element, Length,
};
use super::anime_card::AnimeCard;

#[derive(Debug, Clone)]
pub struct AnimeGrid {
    cards: Vec<AnimeCard>,
}

impl AnimeGrid {
    pub fn new(cards: Vec<AnimeCard>) -> Self {
        Self { cards }
    }

    pub fn view(&self) -> Element<'_, crate::gui::types::Message> {
        let mut grid = Row::new()
            .spacing(20)
            .padding(20);

        for card in &self.cards {
            grid = grid.push(card.view());
        }

        let content = scrollable(grid)
            .width(Length::Fill)
            .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}