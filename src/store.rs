use iced::{
    alignment,
    button, scrollable, Button, Column, Container, Element, 
    Image, Length, Scrollable, Text, Alignment,
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
                .height(Length::Units(40))
        )
        .padding(10)
        .align_x(alignment::Horizontal::Center); // Center the logo horizontally
    
        // Create file select button
        let file_select_button = Button::new(
            &mut self.file_select_button, 
            Text::new(match &self.selected_file {
                Some(path) => format!("Selected: {}", path.display()),
                None => "Select File".to_string(),
            })
        )
        .on_press(crate::Message::TriggerFileSelection);
    
        // Create the main content column
        let content = Column::new()
            .spacing(10)
            .align_items(Alignment::Center) // Center all items in the column
            .push(Text::new("Welcome to the store page!")) 
            .push(file_select_button);
    
        // Create a container for the content
        let container = Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();
    
        // Create the main column and add the logo and container
        Column::new()
            .push(logo) // Add logo to the top
            .push(container)
            .into()
    }

    // Method to trigger file selection
    pub fn trigger_file_selection(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            self.selected_file = Some(path);
        }
    }
}