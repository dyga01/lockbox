// src/store.rs

use iced::{Element, Text};

#[derive(Default)]
pub struct StorePage;

impl StorePage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view(&self) -> Element<crate::login::Message> {
        Text::new("Welcome to the store page!").into()
    }
}
