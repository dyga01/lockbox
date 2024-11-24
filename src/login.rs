// src/login.rs

use crate::store::StorePage;
use crate::AppState;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use iced::{button, text_input};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use hex::{encode, decode};
use rand::Rng;

// Create an alias for convenience
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

// Define the state of the login page
#[derive(Default)]
pub struct LoginPage {
    pub username: String,
    pub password: String,
    pub username_input: text_input::State,
    pub password_input: text_input::State,
    pub login_button: button::State,
    pub authenticated: bool,
    pub state: AppState,
    pub store_page: StorePage, // Add this line
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

                let key = b"an example very very secret key."; // 32 bytes
                let iv = rand::thread_rng().gen::<[u8; 16]>(); // 16 bytes

                let auth_data = AuthData {
                    username: self.username.clone(),
                    password: self.password.clone(),
                };

                let plaintext = serde_json::to_string(&auth_data).unwrap();
                let cipher = Aes256Cbc::new_from_slices(key, &iv).unwrap();
                let ciphertext = cipher.encrypt_vec(plaintext.as_bytes());

                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open("auth/authentication.json")
                    .unwrap();

                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();

                // Clean the contents by removing null characters
                contents = contents.trim_matches(char::from(0)).to_string();

                println!("contents: {}", contents);
                if contents.trim().is_empty() || contents == r#"{"username":"","password":""}"# {
                    // First time login, save the credentials
                    let encrypted_data = format!("{}:{}", encode(iv), encode(ciphertext));
                    file.set_len(0).unwrap(); // Truncate the file
                    file.write_all(encrypted_data.as_bytes()).unwrap();
                    self.authenticated = true;
                    // Switch to the store page
                    self.update(Message::SwitchToStorePage);
                } else {
                    // Check if the credentials match
                    let parts: Vec<&str> = contents.split(':').collect();
                    if parts.len() != 2 {
                        self.authenticated = false;
                        return;
                    }

                    let iv = decode(parts[0]).unwrap();
                    let ciphertext = decode(parts[1]).unwrap();
                    let cipher = Aes256Cbc::new_from_slices(key, &iv).unwrap();
                    let decrypted_data = cipher.decrypt_vec(&ciphertext).unwrap();
                    let stored_auth: AuthData = serde_json::from_slice(&decrypted_data).unwrap();

                    if stored_auth.username == self.username
                        && stored_auth.password == self.password
                    {
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
                self.state = AppState::StorePage; // Update the state
            }
        }
    }
}