// src/login.rs

use crate::store::StorePage;
use crate::AppState;
use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use dotenv::dotenv;
use hex::{decode, encode};
use iced::{button, text_input};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use dirs::config_dir;
use std::path::PathBuf;

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
    TriggerFileSelection, // Add this line
    EncryptFile,
    DecryptFile,
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
            // Handle username change
            Message::UsernameChanged(value) => {
                self.username = value;
            }
            // Handle password change
            Message::PasswordChanged(value) => {
                self.password = value;
            }
            // Handle login button press
            Message::LoginPressed => {
                // Check if username or password is empty
                if self.username.is_empty() || self.password.is_empty() {
                    return;
                }

                // Get the platform-specific directory
                let config_dir = config_dir().expect("Failed to get config directory");
                let key_file_path = config_dir.join("lockbox").join("secret_key");

                // Check if the key file exists
                if !key_file_path.exists() {
                    println!("Secret key file does not exist. Please create it using the following commands:");
                    println!(r#"mkdir -p "$(dirname "$(dirs::config_dir)/lockbox/secret_key")""#);
                    println!(r#"echo "anexampleveryverysecretkey123456" > "$(dirs::config_dir)/lockbox/secret_key""#);
                    return;
                }

                // Read the key from the file
                let stored_key = fs::read(&key_file_path).expect("Failed to read secret key from file");
                let key = stored_key.as_slice();

                // Convert the key to a string slice
                // let key_str = std::str::from_utf8(key).expect("Failed to convert key to string");
                if key.len() != 32 {
                    panic!("The key length is not 32 bytes. You either never set a key or your key is too short. Follow the programs readme to learn more.");
                }

                // Generate a random IV (Initialization Vector)
                let mut rng = rand::thread_rng();
                let mut iv = [0u8; 16];
                rng.fill(&mut iv);

                // Encrypt the password
                let cipher = Aes256Cbc::new_from_slices(key, &iv).unwrap();
                let ciphertext = cipher.encrypt_vec(self.password.as_bytes());

                // Store the encrypted password and IV
                let encrypted_data = format!("{}:{}", encode(iv), encode(ciphertext));

                // Create auth data
                let auth_data = AuthData {
                    username: self.username.clone(),
                    password: self.password.clone(),
                };

                // Serialize auth data to JSON
                let plaintext = serde_json::to_string(&auth_data).unwrap();
                let cipher = Aes256Cbc::new_from_slices(key, &iv).unwrap();
                let ciphertext = cipher.encrypt_vec(plaintext.as_bytes());

                // Open authentication file
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

                    // Verify credentials
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
            // Handle switching to the store page
            Message::SwitchToStorePage => {
                // Logic to switch to the store page
                println!("switched to store page, need logic");
                self.state = AppState::StorePage; // Update the state
            }
            Message::TriggerFileSelection => {
                // Handle TriggerFileSelection
                self.store_page.trigger_file_selection();
            }
            Message::EncryptFile => {
                // Call the function to encrypt a file
                println!("Encrypting file...");
                self.store_page.encrypt_file();
            }
            Message::DecryptFile => {
                // Call the function to decrypt a file
                println!("Decrypting file...");
                self.store_page.decrypt_file();
            }
        }
    }
}
