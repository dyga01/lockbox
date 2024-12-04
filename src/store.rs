use age::{secrecy::SecretString, Decryptor, Encryptor};
use iced::{
    alignment, button, container, scrollable, Alignment, Background, Button, Color, Column, 
    Container, Element, Image, Length, Row, Text,
};
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Instant;
use sysinfo::{System, SystemExt};

#[derive(Default)]
pub struct StorePage {
    scroll: scrollable::State,
    file_select_button: button::State,
    encrypt_button: button::State,
    decrypt_button: button::State,
    selected_file: Option<PathBuf>,
    file_details: Option<FileDetails>,
}

#[derive(Clone)]
struct FileDetails {
    filename: String,
    size: String,
    file_type: String,
    path: String,
    encryption_time: Option<std::time::Duration>,
    encryption_memory_used: Option<u64>,
    decryption_time: Option<std::time::Duration>,
    decryption_memory_used: Option<u64>,
}

impl StorePage {
    pub fn new() -> Self {
        Self {
            scroll: scrollable::State::new(),
            file_select_button: button::State::new(),
            encrypt_button: button::State::new(),
            decrypt_button: button::State::new(),
            selected_file: None,
            file_details: None,
        }
    }

    // Method to extract file details
    fn get_file_details(&self) -> Option<FileDetails> {
        self.selected_file.as_ref().and_then(|path| {
            // Get file metadata
            match fs::metadata(path) {
                Ok(metadata) => {
                    // Convert file size to human-readable format
                    let size = if metadata.len() < 1024 {
                        format!("{} bytes", metadata.len())
                    } else if metadata.len() < 1024 * 1024 {
                        format!("{:.2} KB", metadata.len() as f64 / 1024.0)
                    } else {
                        format!("{:.2} MB", metadata.len() as f64 / (1024.0 * 1024.0))
                    };

                    // Determine file type
                    let file_type = path.extension()
                        .map(|ext| ext.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Unknown".to_string());

                    Some(FileDetails {
                        filename: path.file_name().unwrap().to_string_lossy().to_string(),
                        size,
                        file_type: "Unknown".to_string(), // Update this as needed
                        path: path.to_string_lossy().to_string(),
                        encryption_time: None,
                        encryption_memory_used: None,
                        decryption_time: None,
                        decryption_memory_used: None,
                    })
                }
                Err(_) => None
            }
        })
    }

    pub fn view(&mut self) -> Element<crate::Message> {
        let logo = Container::new(
            Image::new("images/logo.png")
                .width(Length::Units(40))
                .height(Length::Units(40)),
        )
        .padding(10)
        .align_x(alignment::Horizontal::Center);

        let file_select_button = Button::new(
            &mut self.file_select_button,
            Text::new("Select File").size(20),
        )
        .style(BlueButton)
        .on_press(crate::Message::TriggerFileSelection);

        let encrypt_button = Button::new(&mut self.encrypt_button, Text::new("Encrypt").size(20))
            .style(GreenButton)
            .on_press(crate::Message::EncryptFile);

        let decrypt_button = Button::new(&mut self.decrypt_button, Text::new("Decrypt").size(20))
            .style(OrangeButton)
            .on_press(crate::Message::DecryptFile);

        let button_row = Row::new()
            .spacing(20)
            .push(file_select_button)
            .push(encrypt_button)
            .push(decrypt_button);

        let mut content = Column::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .push(Text::new("Select a file to encrypt or decrypt!").size(24))
            .push(button_row);

        if let Some(details) = &self.file_details {
            let details_layout = Column::new()
                .spacing(10)
                .push(Text::new("File Details").size(24))
                .push(
                    Container::new(
                        Row::new()
                            .spacing(20)
                            .push(Text::new("Filename:").size(18))
                            .push(Text::new(&details.filename).size(18))
                    )
                    .style(AlternateRowLight)
                    .width(Length::Units(300))
                )
                .push(
                    Container::new(
                        Row::new()
                            .spacing(20)
                            .push(Text::new("Size:").size(18))
                            .push(Text::new(&details.size).size(18))
                    )
                    .style(AlternateRowDark)
                    .width(Length::Units(300))
                )
                .push(
                    Container::new(
                        Row::new()
                            .spacing(20)
                            .push(Text::new("Type:").size(18))
                            .push(Text::new(&details.file_type).size(18))
                    )
                    .style(AlternateRowLight)
                    .width(Length::Units(300))
                )
                .push(
                    Container::new(
                        Row::new()
                            .spacing(20)
                            .push(Text::new("Path:").size(18))
                            .push(Text::new(&details.path).size(18))
                    )
                    .style(AlternateRowDark)
                    .width(Length::Units(300))
                );

            let mut performance_layout = Column::new()
                .spacing(10)
                .push(Text::new("Performance Metrics").size(24));

            if let Some(encryption_time) = details.encryption_time {
                performance_layout = performance_layout.push(
                    Container::new(
                        Row::new()
                            .spacing(20)
                            .push(Text::new("Encryption Time:").size(18))
                            .push(Text::new(&format!("{:?}", encryption_time)).size(18))
                    )
                    .style(AlternateRowLight)
                    .width(Length::Units(300))
                );
            }

            if let Some(encryption_memory_used) = details.encryption_memory_used {
                performance_layout = performance_layout.push(
                    Container::new(
                        Row::new()
                            .spacing(20)
                            .push(Text::new("Encryption Memory Used:").size(18))
                            .push(Text::new(&format!("{} KB", encryption_memory_used)).size(18))
                    )
                    .style(AlternateRowDark)
                    .width(Length::Units(300))
                );
            }

            if let Some(decryption_time) = details.decryption_time {
                performance_layout = performance_layout.push(
                    Container::new(
                        Row::new()
                            .spacing(20)
                            .push(Text::new("Decryption Time:").size(18))
                            .push(Text::new(&format!("{:?}", decryption_time)).size(18))
                    )
                    .style(AlternateRowLight)
                    .width(Length::Units(300))
                );
            }

            if let Some(decryption_memory_used) = details.decryption_memory_used {
                performance_layout = performance_layout.push(
                    Container::new(
                        Row::new()
                            .spacing(20)
                            .push(Text::new("Decryption Memory Used:").size(18))
                            .push(Text::new(&format!("{} KB", decryption_memory_used)).size(18))
                    )
                    .style(AlternateRowDark)
                    .width(Length::Units(300))
                );
            }
    
            let bordered_details = Container::new(details_layout)
                .style(BlueBorderContainer)
                .padding(10);

            let bordered_performance = Container::new(performance_layout)
                .style(BlueBorderContainer)
                .padding(10);

            content = content.push(bordered_details).push(bordered_performance);
        }

        let container = Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        Column::new()
            .push(logo)
            .push(container)
            .into()
    }

    pub fn trigger_file_selection(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            self.selected_file = Some(path);
            // Update file details when a new file is selected
            self.file_details = self.get_file_details();
        }
    }

    pub fn encrypt_file(&mut self) {
        if let Some(path) = &self.selected_file {
            let mut system = System::new_all();
            system.refresh_all();

            let start_time = Instant::now();
            let initial_memory = system.used_memory();

            let file_content = fs::read(path).expect("Failed to read file");
            let encryptor =
                Encryptor::with_user_passphrase(SecretString::new("password".to_string()));
            let mut encrypted_output = Vec::new();
            let mut writer = encryptor
                .wrap_output(&mut encrypted_output)
                .expect("Failed to create encryptor");
            writer
                .write_all(&file_content)
                .expect("Failed to encrypt file");
            writer.finish().expect("Failed to finalize encryption");
            fs::write(path, encrypted_output).expect("Failed to write encrypted file");

            let duration = start_time.elapsed();
            system.refresh_all();
            let final_memory = system.used_memory();

            let memory_used = if final_memory >= initial_memory {
                final_memory - initial_memory
            } else {
                0
            };

            println!("Encryption time: {:?}", duration);
            println!("Memory used: {} KB", memory_used);

            // Update file details with encryption time and memory used
            if let Some(details) = &mut self.file_details {
                details.encryption_time = Some(duration);
                details.encryption_memory_used = Some(memory_used);
            }
        }
    }
    
    pub fn decrypt_file(&mut self) {
        if let Some(path) = &self.selected_file {
            let mut system = System::new_all();
            system.refresh_all();

            let start_time = Instant::now();
            let initial_memory = system.used_memory();

            let file_content = fs::read(path).expect("Failed to read file");
            let decryptor =
                Decryptor::new(file_content.as_slice()).expect("Failed to create decryptor");
            let mut decrypted_content = Vec::new();
            match decryptor {
                Decryptor::Passphrase(decryptor) => {
                    decryptor
                        .decrypt(&SecretString::new("password".to_string()), None)
                        .expect("Failed to decrypt file")
                        .read_to_end(&mut decrypted_content)
                        .expect("Failed to read decrypted content");
                }
                _ => panic!("Unsupported decryptor"),
            };
            fs::write(path, decrypted_content).expect("Failed to write decrypted file");

            let duration = start_time.elapsed();
            system.refresh_all();
            let final_memory = system.used_memory();

            let memory_used = if final_memory >= initial_memory {
                final_memory - initial_memory
            } else {
                0
            };

            println!("Decryption time: {:?}", duration);
            println!("Memory used: {} KB", memory_used);

            // Update file details with decryption time and memory used
            if let Some(details) = &mut self.file_details {
                details.decryption_time = Some(duration);
                details.decryption_memory_used = Some(memory_used);
            }
        }
    }
}

// Custom button styles remain the same as in the previous implementation
struct BlueButton;

impl button::StyleSheet for BlueButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(
                66.0 / 255.0,
                144.0 / 255.0,
                245.0 / 255.0,
            ))), // Set to #4290f5
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

// Add these struct definitions near your other style structs
struct BlueBorderContainer;
struct AlternateRowLight;
struct AlternateRowDark;

impl container::StyleSheet for BlueBorderContainer {
    fn style(&self) -> container::Style {
        container::Style {
            border_width: 1.0,
            border_color: Color::from_rgb(66.0 / 255.0, 144.0 / 255.0, 245.0 / 255.0),
            ..Default::default()
        }
    }
}

impl container::StyleSheet for AlternateRowLight {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::WHITE)),
            border_width: 1.0,
            border_radius: 0.0,
            ..container::Style::default()
        }
    }
}

impl container::StyleSheet for AlternateRowDark {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))), // Light grey
            border_width: 1.0,
            border_radius: 0.0,
            ..container::Style::default()
        }
    }
}