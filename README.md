# lockbox

![Logo](images/logo-full-transparent.png)

## Overview

lockbox is a secure application that allows users to encrypt and decrypt files

## Features

- Secure login with AES-256 encryption
- Encrypt and decrypt files using the age crate
- Detailed file and encryption information
- User-friendly interface

## Getting Started

### Prerequisites

- Rust and Cargo installed on your machine
- Mac OS

### Setup

1. Clone the repository:

    ```sh
    git clone https://github.com/dyga01/lockbox.git
    cd lockbox
    ```

2. Create the necessary directory and secret key file:

    ```sh
    mkdir -p ~/Library/Application\ Support/lockbox
    echo -n "anexampleveryverysecretkey12345678" > ~/Library/Application\ Support/lockbox/secret_key
    cat ~/Library/Application\ Support/lockbox/secret_key
    ```

3. Build and run the project:

    ```sh
    cargo run
    ```

### First Time Login

The first time you run the application, you will be prompted to enter a username and password. These credentials will be stored securely and used for future logins.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
