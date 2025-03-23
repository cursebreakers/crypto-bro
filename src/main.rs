// main.rs

mod crypto_control;
mod ascii;

use std::io::{self, Write};
use clap::Parser;

#[derive(Parser)]
#[command(name = "Key Generator", version = "0.1.0", about = "Generate cryptographic keys")]
struct Args {
    #[arg(short = 'k', help = "Key type: 1 (OpenSSL), 2 (Ring), 3 (UUID), 4 (API Key), 5 (16char Password), 6 (32char Password), 7 (Username)")]
    key: Option<u32>,
}

fn main() {

	let images = ascii::AsciiImages {
        image1: "src/ascii/fsoc.txt",
        image2: "src/ascii/crsbrks.txt",
    };

    let args = Args::parse();

    let mut preselected_key: Option<(String, &'static str, u32)> = None;

    if let Some(choice) = args.key {
        preselected_key = Some(match choice {
			1 => (crypto_control::generate_openssl_key(), "Base64 encoded, 256-bit (OpenSSL)", 1),
			2 => (crypto_control::generate_ring_key(), "Base64 encoded, 256-bit (Ring)", 2),
			3 => (crypto_control::generate_uuid(), "UUID (v4)", 3),
			4 => (crypto_control::generate_api_key(), "Base64 encoded, 256-bit (API Key)", 4),
			5 => (crypto_control::generate_password(16), "16-character password", 5),
            6 => (crypto_control::generate_password(32), "32-character password", 6),
            7 => (crypto_control::generate_username(), "Username (Word list generated)", 7),
			_ => {
				println!("\x1b[91mInvalid key type! Use -k 1, 2, 3, or 4.\x1b[0m");
				return;
			}
		});
    }

    loop {
        let (key, key_type, key_choice) = if let Some(k) = preselected_key.take() {
            k
        } else {
            println!("\n");
			if let Err(e) = images.print_random_ascii() {
		        eprintln!("Error reading file: {}", e);
		    }
            println!("    \x1b[92mHello, friend.\x1b[0m");
            println!("\n________________________________");
            println!("\n");
            println!("    Key Generation Options:");
            println!("\n");
            println!("    1. OpenSSL Cryptographic Key (Base64 encoded, 256-bit)");
            println!("    2. Ring Cryptographic Key    (Base64 encoded, 256-bit)");
            println!("    3. UUID                      (v4 format)");
            println!("    4. API Key                   (Base64 encoded, 256-bit)");
            println!("    5. Password                  (16-characters)");
            println!("    6. Password                  (32-characters)");
            println!("    7. Username                  (Word list generated)");
            println!("\n");
            println!("    q. Quit");
            println!("\n________________________________\n");

            let mut input = String::new();
            println!("\n");
            print!("\nChoose an option (1-7, or q): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let choice = input.trim().to_lowercase();

            if choice == "q" {
                println!("\n");
                println!("\x1b[92mGoodbye!\x1b[0m");
                println!("\n");
                break;
            }

            let choice: u32 = match choice.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("\n");
                    println!("\x1b[91mInvalid input! Please enter a number between 1 and 6 or 'q' to quit.\x1b[0m");
                    println!("\n");
                    continue;
                }
            };

            match choice {
                1 => (crypto_control::generate_openssl_key(), "Base64 encoded, 256-bit (OpenSSL)", 1),
                2 => (crypto_control::generate_ring_key(), "Base64 encoded, 256-bit (Ring)", 2),
                3 => (crypto_control::generate_uuid(), "UUID (v4)", 3),
                4 => (crypto_control::generate_api_key(), "Base64 encoded, 256-bit (API Key)", 4),
				5 => (crypto_control::generate_password(16), "16-character password", 5),
                6 => (crypto_control::generate_password(32), "32-character password", 6),
                7 => (crypto_control::generate_username(), "Username (Word list generated)", 7),

                _ => {
                    println!("\x1b[91mInvalid choice!\x1b[0m");
                    continue;
                }
            }
        };

        println!("\n________________________________\n\n");
		println!("\x1b[92mGenerated Key:\x1b[0m \x1b[38;5;198m({})\x1b[0m", key_type);
        println!("\x1b[96m{}\x1b[0m\n", key);

        loop {
            println!("\nOptions:"); 
            println!("    (c) Copy to clipboard");
            println!("    (r) Regenerate");
            println!("    (m) Main menu");
            println!("    (q) Exit the app");
			println!("\n");
            print!("\nEnter your choice: ");
            io::stdout().flush().unwrap();

            let mut option = String::new();
            io::stdin().read_line(&mut option).unwrap();
            let option = option.trim().to_lowercase();

            match option.as_str() {
                "c" => {
					if crypto_control::copy_to_clipboard(&key) {
						println!("\x1b[92mKey copied to clipboard successfully!\x1b[0m");
						io::stdout().flush().unwrap();  // Ensure the message is printed before continuing
					} else {
						println!("\x1b[91mFailed to copy key to clipboard.\x1b[0m");
						io::stdout().flush().unwrap();  // Ensure error is printed before continuing
					}
				}
				"r" => {
                    // Regenerate the key of the same type
                    let (key, key_type) = match key_choice {
                        1 => (crypto_control::generate_openssl_key(), "Base64 encoded, 256-bit (OpenSSL)"),
                        2 => (crypto_control::generate_ring_key(), "Base64 encoded, 256-bit (Ring)"),
                        3 => (crypto_control::generate_uuid(), "UUID (v4)"),
                        4 => (crypto_control::generate_api_key(), "Base64 encoded, 256-bit (API Key)"),
						5 => (crypto_control::generate_password(16), "16-character password"),
                        6 => (crypto_control::generate_password(32), "32-character password"),
                        7 => (crypto_control::generate_username(), "Username (Word list generated)"),
                        _ => {
                            println!("\x1b[91mError in regenerating key!\x1b[0m");
                            continue;
                        }
                    };
		            println!("\n________________________________\n");
                    println!("\x1b[92mRegenerated Key:\x1b[0m \x1b[38;5;198m({})\x1b[0m", key_type);
                    println!("\x1b[96m{}\x1b[0m\n", key);
                }
                "m" => break,
                "q" => {
                    println!("\n");
                    println!("\x1b[92mGoodbye!\x1b[0m");
	                println!("\n");
                    return;
                }
                _ => println!("\x1b[91mInvalid option! Choose 'c', 'm', or 'q'.\x1b[0m"),
            }
        }
    }
}


