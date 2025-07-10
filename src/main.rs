// main.rs

mod crypto_control;
mod lunchbox;
mod ascii;

use crossterm::{
    event::{self, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use clap::Parser;

#[derive(Parser)]
#[command(name = "Key Generator", version = "0.1.0", about = "The Rust-based cryptographic multi-tool.")]
struct Args {
    #[arg(short = 'k', help = "Key type: 1 (OpenSSL), 2 (Ring), 3 (UUID), 4 (API Key), 5 (16char Password), 6 (32char Password), 7 (Username)")]
    key: Option<u32>,
}

fn generate_key(choice: u32) -> Option<(String, &'static str, u32)> {
    match choice {
        1 => Some((crypto_control::generate_openssl_key(), "Base64 encoded, 256-bit (OpenSSL)", 1)),
        2 => Some((crypto_control::generate_ring_key(), "Base64 encoded, 256-bit (Ring)", 2)),
        3 => Some((crypto_control::generate_uuid(), "UUID (v4)", 3)),
        4 => Some((crypto_control::generate_api_key(), "Base64 encoded, 256-bit (API Key)", 4)),
        5 => Some((crypto_control::generate_password(16), "16-character password", 5)),
        6 => Some((crypto_control::generate_password(32), "32-character password", 6)),
        7 => Some((crypto_control::generate_username(), "Username (Word list generated)", 7)),
        _ => None,
    }
}

fn get_keypress() -> Option<KeyEvent> {
    enable_raw_mode().unwrap();
    let result = loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                break Some(key);
            }
        }
    };
    disable_raw_mode().unwrap();
    result
}

fn main() {
    let images = ascii::AsciiImages {
        image1: "src/ascii/fsoc.txt",
        image2: "src/ascii/crsbrks.txt",
    };

    if let Err(e) = images.print_random_ascii() {
        eprintln!("\x1b[91mFailed to load ASCII image: {}\x1b[0m", e);
    }
    println!("\n    \x1b[92mHello, friend.\x1b[0m\n");

    let args = Args::parse();
    let mut preselected_key = args.key.and_then(generate_key);

    loop {
        let (key, key_type, key_choice) = if let Some(k) = preselected_key.take() {
            k
        } else {

            println!("________________________________\n");
            println!("    Main Menu:\n");
            println!("    [1] OpenSSL Cryptographic Key (Base64 encoded, 256-bit)");
            println!("    [2] Ring Cryptographic Key    (Base64 encoded, 256-bit)");
            println!("    [3] UUID                      (v4 format)");
            println!("    [4] API Key                   (Base64 encoded, 256-bit)");
            println!("    [5] Password                  (16-characters)");
            println!("    [6] Password                  (32-characters)");
            println!("    [7] Username                  (Word list generated)");
            println!("    [8] Pack .env file            (Encrypt)");
            println!("    [9] Unpack .env file          (Decrypt)\n");
            println!("    [q] Quit\n");
            println!("________________________________\n");

		    enable_raw_mode().unwrap();

            print!("\nPress a key (1-7, q): ");
            io::stdout().flush().unwrap();

            let key_event = get_keypress().unwrap();
            match key_event.code {
                KeyCode::Char('q') => {
                    println!("\n\x1b[92mGoodbye!\x1b[0m\n");
                    disable_raw_mode().unwrap();
                    return;
                }
                KeyCode::Char(c) if c.is_digit(10) => {
                    let choice = c.to_digit(10).unwrap();
                    if choice >= 1 && choice <= 7 {
                        if let Some(k) = generate_key(choice) {
                            k
                        } else {
                            println!("\x1b[91mInvalid choice!\x1b[0m");
                            continue;
                        }
                    } else if choice == 8 {
                        lunchbox::encrypt_menu_loop();
                        continue;
                    } else if choice == 9 {
                        lunchbox::decrypt_menu_loop();
                        continue;
                    } else {
                        println!("\x1b[91mInvalid choice!\x1b[0m");
                        continue;
                    }
                }
                _ => {
                    println!("\x1b[91mInvalid input! Press a number (1-9) or 'q' to quit.\x1b[0m");
                    continue;
                }
            }
        };

        println!("\n________________________________\n");
        println!("\x1b[92mGenerated Key:\x1b[0m \x1b[38;5;198m({})\x1b[0m", key_type);
        println!("\x1b[96m{}\x1b[0m\n", key);

        loop {
            println!("\nOptions:");
            println!("    [c] Copy to clipboard");
            println!("    [r] Regenerate");
            println!("    [m] Main menu");
            println!("    [q] Exit the app\n");

            print!("\nPress a key (c, r, m, q): ");
            io::stdout().flush().unwrap();

            let option_event = get_keypress().unwrap();
            match option_event.code {
                KeyCode::Char('c') => {
                    if crypto_control::copy_to_clipboard(&key) {
                        println!("\x1b[92mKey copied to clipboard successfully!\x1b[0m");
                    } else {
                        println!("\x1b[91mFailed to copy key to clipboard.\x1b[0m");
                    }
                }
                KeyCode::Char('r') => {
                    if let Some((new_key, new_key_type, _)) = generate_key(key_choice) {
                        println!("\n________________________________\n");
                        println!("\x1b[92mRegenerated Key:\x1b[0m \x1b[38;5;198m({})\x1b[0m", new_key_type);
                        println!("\x1b[96m{}\x1b[0m\n", new_key);
                    } else {
                        println!("\x1b[91mError in regenerating key!\x1b[0m");
                    }
                }
                KeyCode::Char('m') => break,
                KeyCode::Char('q') => {
                    println!("\n\x1b[92mGoodbye!\x1b[0m\n");
                    disable_raw_mode().unwrap();
                    return;
                }
                _ => println!("\x1b[91mInvalid option! Press 'c', 'r', 'm', or 'q'.\x1b[0m"),
            }
        }
    }
}

