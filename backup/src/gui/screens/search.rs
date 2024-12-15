use iced::widget::{container, Column, Row, text};
use iced::{Element, Length};
use crate::gui::{
    types::{Message, SearchArgs},
    components::{SearchInput, AnimeCard},
};
use crate::kodik::MediaResult;

#[derive(Debug, Clone)]
pub struct SearchScreen {
    query: String,
    results: Option<Vec<MediaResult>>,
    error: Option<String>,
    cards: Vec<AnimeCard>,
}

impl SearchScreen {
    pub fn new(args: SearchArgs) -> Self {
        Self {
            query: args.query,
            results: args.results.clone(),
            error: args.error,
            cards: Vec::new(),
        }
    }

    pub fn update_search(&mut self, query: String) {
        self.query = query;
    }

    pub fn update_results(&mut self, results: Vec<MediaResult>) {
        self.cards = results.iter()
            .map(|result| AnimeCard::new(result.clone()).0)
            .collect();
        self.results = Some(results);
    }

    pub fn update_error(&mut self, error: Option<String>) {
        self.error = error;
    }

    pub fn view(&self) -> Element<Message> {
        let mut content = Column::new()
            .spacing(20)
            .padding(20)
            .push(SearchInput::view(&self.query));

        if let Some(error) = &self.error {
            content = content.push(
                text(error)
                    .size(16)
                    .style(iced::Color::from_rgb(1.0, 0.0, 0.0))
            );
        }

        if !self.cards.is_empty() {
            let mut row = Row::new().spacing(20);
            let mut current_row_count = 0;
            
            for card in &self.cards {
                row = row.push(card.view());
                current_row_count += 1;
                
                if current_row_count == 4 {
                    content = content.push(row);
                    row = Row::new().spacing(20);
                    current_row_count = 0;
                }
            }
            
            // Add any remaining items
            if current_row_count > 0 {
                content = content.push(row);
            }
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}