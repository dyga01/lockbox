use iced::{
    alignment,
    button, scrollable, Button, Column, Container, Element, 
    Image, Length, Row, Text, Alignment, Background, Color,
};
use std::path::PathBuf;

#[derive(Default)]
pub struct StorePage {
    scroll: scrollable::State,
    file_select_button: button::State,
    encrypt_button: button::State,
    decrypt_button: button::State,
    selected_file: Option<PathBuf>,
}

impl StorePage {
    // Constructor for StorePage
    pub fn new() -> Self {
        Self {
            scroll: scrollable::State::new(),
            file_select_button: button::State::new(),
            encrypt_button: button::State::new(),
            decrypt_button: button::State::new(),
            selected_file: None,
        }
    }

    // Method to create the view for the store page
    pub fn view(&mut self) -> Element<crate::Message> {
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
            Text::new("Select File")
                .size(20) // Increase button text size
        )
        .style(BlueButton) // Apply the custom style
        .on_press(crate::Message::TriggerFileSelection);
    
        // Create the main content column
        let mut content = Column::new()
            .spacing(20)
            .align_items(Alignment::Center) // Center all items in the column
            .push(Text::new("Select a file to encrypt or decrypt!").size(24)) // Increase text size
            .push(file_select_button);

        // Display the selected file path if available
        if let Some(path) = &self.selected_file {
            content = content.push(Text::new(format!("Selected: {}", path.display())).size(20));

            // Add Encrypt and Decrypt buttons in a row
            let buttons_row = Row::new()
                .spacing(20)
                .push(
                    Button::new(
                        &mut self.encrypt_button,
                        Text::new("Encrypt").size(20)
                    )
                    .style(GreenButton)
                    .on_press(crate::Message::EncryptFile)
                )
                .push(
                    Button::new(
                        &mut self.decrypt_button,
                        Text::new("Decrypt").size(20)
                    )
                    .style(OrangeButton)
                    .on_press(crate::Message::DecryptFile)
                );

            content = content.push(buttons_row);
        }

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

// Define a custom button style for the blue button
struct BlueButton;

impl button::StyleSheet for BlueButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(66.0 / 255.0, 144.0 / 255.0, 245.0 / 255.0))), // Set to #4290f5
            border_radius: 5.0,
            text_color: Color::WHITE,
            shadow_offset: iced::Vector::new(0.0, 0.0),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))), // Darker blue on hover
            ..self.active()
        }
    }
}

// Define a custom button style for the green button
struct GreenButton;

impl button::StyleSheet for GreenButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.52, 0.72, 0.59))), // #84b896
            border_radius: 5.0,
            text_color: Color::WHITE,
            shadow_offset: iced::Vector::new(0.0, 0.0),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.42, 0.62, 0.49))), // Darker green on hover
            ..self.active()
        }
    }
}

// Define a custom button style for the orange button
struct OrangeButton;

impl button::StyleSheet for OrangeButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.96, 0.69, 0.36))), // #f5af5b
            border_radius: 5.0,
            text_color: Color::WHITE,
            shadow_offset: iced::Vector::new(0.0, 0.0),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.86, 0.59, 0.26))), // Darker orange on hover
            ..self.active()
        }
    }
}