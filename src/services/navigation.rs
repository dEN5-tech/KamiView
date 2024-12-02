use std::collections::VecDeque;
use crate::navigation::Screen;

pub struct NavigationService {
    history: VecDeque<Screen>,
    current: Screen,
}

impl NavigationService {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            current: Screen::Main,
        }
    }

    pub fn navigate_to(&mut self, screen: Screen) {
        self.history.push_back(self.current.clone());
        self.current = screen;
    }

    pub fn go_back(&mut self) -> Option<Screen> {
        self.history.pop_back().map(|screen| {
            self.current = screen.clone();
            screen
        })
    }

    pub fn current_screen(&self) -> &Screen {
        &self.current
    }
} 