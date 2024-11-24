use iced::{Element, Text};

#[derive(Default)]
pub struct StorePage;

impl StorePage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view(&self) -> Element<crate::login::Message> {
        Text::new("Welcome to the where files will be stored!").into()
    }
}

// when the user is authenticated they should be moved to the store.rs page
