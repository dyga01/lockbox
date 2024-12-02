use iced::{
    button, scrollable, Button, Column, Container, Element, 
    Image, Length, Scrollable, Text,
};
use std::path::PathBuf;

#[derive(Default)]
pub struct StorePage {
    scroll: scrollable::State,
    file_select_button: button::State,
    selected_file: Option<PathBuf>,
}

impl StorePage {
    // Constructor for StorePage
    pub fn new() -> Self {
        Self {
            scroll: scrollable::State::new(),
            file_select_button: button::State::new(),
            selected_file: None,
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

        // Create file select button
        let file_select_button = Button::new(
            &mut self.file_select_button, 
            Text::new(match &self.selected_file {
                Some(path) => format!("Selected: {}", path.display()),
                None => "Select File".to_string(),
            })
        )
        .on_press(crate::login::Message::SwitchToStorePage);

        // Create the main content column
        let content = Column::new()
            .spacing(10)
            .push(logo) // Add logo to the column
            .push(Text::new("Welcome to the store page!")) // Add welcome text
            .push(file_select_button); // Add file select button

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

    // Method to trigger file selection
    pub fn trigger_file_selection(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            self.selected_file = Some(path);
        }
    }
}