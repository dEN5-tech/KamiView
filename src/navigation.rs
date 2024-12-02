use crate::services::kodik::SearchResult;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Main,
    Details(SearchResult),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Main
    }
} 