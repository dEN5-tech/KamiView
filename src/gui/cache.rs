use std::collections::HashMap;
use iced::widget;

#[derive(Debug, Default)]
pub struct ImageCache {
    images: HashMap<String, widget::image::Handle>,
}

impl ImageCache {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
        }
    }

    pub fn get(&self, url: &str) -> Option<widget::image::Handle> {
        self.images.get(url).cloned()
    }

    pub fn insert(&mut self, url: String, handle: widget::image::Handle) {
        self.images.insert(url, handle);
    }
} 