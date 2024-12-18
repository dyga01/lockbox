// src/main.rs

use iced::widget::Image;
use iced::{
    text_input, Alignment, Background, Button, Color, Column, Container, Element, Length, Sandbox,
    Settings, Text, TextInput,
};

mod login;
mod store;
use login::{LoginPage, Message};
use store::StorePage;

// Define all of the app states
#[derive(Debug, Clone)]
pub enum AppState {
    LoginPage,
    StorePage,
}

// Default state is the login page
impl Default for AppState {
    fn default() -> Self {
        AppState::LoginPage
    }
}

// Custom style for the text input fields
struct CustomTextInput;

impl text_input::StyleSheet for CustomTextInput {
    // Style for active state
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(Color::WHITE),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::from_rgb(66.0 / 255.0, 144.0 / 255.0, 245.0 / 255.0), // Set to #4290f5
        }
    }

    // Style for focused state
    fn focused(&self) -> text_input::Style {
        self.active()
    }

    // Color for placeholder text
    fn placeholder_color(&self) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    // Color for input value text
    fn value_color(&self) -> Color {
        Color::BLACK
    }

    // Color for text selection
    fn selection_color(&self) -> Color {
        Color::from_rgb(66.0 / 255.0, 144.0 / 255.0, 245.0 / 255.0) // Set to #4290f5
    }
}

// Implement the Sandbox trait for LoginPage
impl Sandbox for LoginPage {
    type Message = Message;

    // Create a new instance of LoginPage
    fn new() -> Self {
        Self::default()
    }

    // Set the title of the application
    fn title(&self) -> String {
        String::from("lockbox")
    }

    // Update the state based on the received message
    fn update(&mut self, message: Message) {
        self.update(message);
    }

    // Define the view for the application
    fn view(&mut self) -> Element<Message> {
        match self.state {
            AppState::LoginPage => {
                let logo = Container::new(
                    Image::new("images/logo.png")
                        .width(Length::Units(40))
                        .height(Length::Units(40)),
                )
                .padding(10); // Add padding to the logo

                let logo_full = Container::new(
                    Image::new("images/logo-full.png")
                        .width(Length::Units(200))
                        .height(Length::Units(100)),
                );

                let username_input = TextInput::new(
                    &mut self.username_input,
                    "username",
                    &self.username,
                    Message::UsernameChanged,
                )
                .padding(10)
                .size(20)
                .width(Length::Units(200)) // Set fixed width
                .style(CustomTextInput);

                let password_input = TextInput::new(
                    &mut self.password_input,
                    "password",
                    &self.password,
                    Message::PasswordChanged,
                )
                .padding(10)
                .size(20)
                .width(Length::Units(200)) // Set fixed width
                .style(CustomTextInput)
                .password();

                let login_button = Button::new(&mut self.login_button, Text::new("login"))
                    .on_press(Message::LoginPressed);

                let content = Column::new()
                    .padding(20)
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .push(logo_full) // Add full logo to the column
                    .push(username_input)
                    .push(password_input)
                    .push(login_button);

                let container = Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y();

                Column::new()
                    .push(logo) // Add logo to the top left
                    .push(container)
                    .into()
            }
            AppState::StorePage => {
                // Store page view
                self.store_page.view()
            }
        }
    }
}

// Main function that calls the iced front-end
fn main() {
    let settings = Settings {
        window: iced::window::Settings {
            size: (800, 600), // Set the initial size (width, height)
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    };

    LoginPage::run(settings).unwrap();
}

/*
COMMANDS
- cargo run (run the project quickly)
- cargo fmt (format the whole project)
- cargo check (check the current project for errors without producing an executable)
- cargo doc (build the documentation for the current project)
*/
