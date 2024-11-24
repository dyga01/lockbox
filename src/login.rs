// login.rs

use iced::{button, text_input};
use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions};
use std::io::{Read, Write};

// Define the state of the login page
#[derive(Default)]
pub struct LoginPage {
    pub username: String,
    pub password: String,
    pub username_input: text_input::State,
    pub password_input: text_input::State,
    pub login_button: button::State,
    pub authenticated: bool,
}

// Define the messages that can be sent in the application
#[derive(Debug, Clone)]
pub enum Message {
    UsernameChanged(String),
    PasswordChanged(String),
    LoginPressed,
    SwitchToStorePage,
}

// Define the authentication data structure
#[derive(Serialize, Deserialize)]
struct AuthData {
    username: String,
    password: String,
}

impl LoginPage {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::UsernameChanged(value) => {
                self.username = value;
            }
            Message::PasswordChanged(value) => {
                self.password = value;
            }
            Message::LoginPressed => {
                if self.username.is_empty() || self.password.is_empty() {
                    return;
                }

                let auth_data = AuthData {
                    username: self.username.clone(),
                    password: self.password.clone(),
                };

                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open("db/authentication.json")
                    .unwrap();

                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                
                println!("contents: {}", contents);
                if contents.trim().is_empty() || contents == r#"{"username":"","password":""}"# {
                    // First time login, save the credentials
                    let json = serde_json::to_string(&auth_data).unwrap();
                    file.set_len(0).unwrap(); // Truncate the file
                    file.write_all(json.as_bytes()).unwrap();
                    self.authenticated = true;
                } else {
                    // Check if the credentials match
                    let stored_auth: AuthData = serde_json::from_str(&contents).unwrap();
                    if stored_auth.username == self.username && stored_auth.password == self.password {
                        self.authenticated = true;
                        // Switch to the store page
                        self.update(Message::SwitchToStorePage);
                    } else {
                        self.authenticated = false;
                    }
                }
            }
            Message::SwitchToStorePage => {
                // Logic to switch to the store page
                println!("switched to store page, need logic");
                // This can be implemented by changing the application's state
            }
        }
    }
}