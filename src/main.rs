use iced::{
    text_input, Alignment, Button, Column, Container, Element, Length, Sandbox, Settings, Text, TextInput,
    Background, Color,
};
use iced::widget::Image;

mod login;
mod store;
use login::{LoginPage, Message};
use store::StorePage;

// Custom style for the text input fields
struct CustomTextInput;

impl text_input::StyleSheet for CustomTextInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(Color::WHITE),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::from_rgb(66.0 / 255.0, 144.0 / 255.0, 245.0 / 255.0), // Set to #4290f5
        }
    }

    fn focused(&self) -> text_input::Style {
        self.active()
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn value_color(&self) -> Color {
        Color::BLACK
    }

    fn selection_color(&self) -> Color {
        Color::from_rgb(66.0 / 255.0, 144.0 / 255.0, 245.0 / 255.0) // Set to #4290f5
    }
}

// Implement the Sandbox trait for the LoginPage struct
impl Sandbox for LoginPage {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("lockbox")
    }

    // Update the state of the application based on the received message
    fn update(&mut self, message: Message) {
        self.update(message);
    }

    // Define the view of the application
    fn view(&mut self) -> Element<Message> {
        let logo = Container::new(
            Image::new("images/logo.png")
                .width(Length::Units(40))
                .height(Length::Units(40))
        )
        .padding(10); // Add padding to the logo
    
        let logo_full = Container::new(
            Image::new("images/logo-full.png")
                .width(Length::Units(200))
                .height(Length::Units(100))
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
        .password()
        .width(Length::Units(200)) // Set fixed width
        .style(CustomTextInput);

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
}

// Main function that calls the iced front-end
fn main() {
    LoginPage::run(Settings::default());
}

// decide on formatting
// decide on encryption tecnique