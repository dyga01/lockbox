// src/store.rs

use iced::{scrollable, Column, Container, Element, Image, Length, Scrollable, Text};

#[derive(Default)]
pub struct StorePage {
    scroll: scrollable::State, // State for the scrollable content
}

impl StorePage {
    // Constructor for StorePage
    pub fn new() -> Self {
        Self {
            scroll: scrollable::State::new(), // Initialize scroll state
        }
    }

    // Method to create the view for the store page
    pub fn view(&mut self) -> Element<crate::login::Message> {
        // Create a container for the logo image
        let logo = Container::new(
            Image::new("images/logo.png")
                .width(Length::Units(40))
                .height(Length::Units(40)),
        )
        .padding(10);

        // Create the main content column
        let content = Column::new()
            .push(logo) // Add logo to the column
            .push(Text::new("Welcome to the store page!")); // Add welcome text

        // Create a scrollable container for the content
        let scrollable_content = Scrollable::new(&mut self.scroll)
            .push(content) // Add content to the scrollable container
            .width(Length::Fill) // Set width to fill the available space
            .height(Length::Fill); // Set height to fill the available space

        // Create the main container and convert it to an Element
        Container::new(scrollable_content)
            .width(Length::Fill) // Set width to fill the available space
            .height(Length::Fill) // Set height to fill the available space
            .into()
    }
}
