// src/lunchbox.rs

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use rand::Rng;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;

use crate::crypto_control::{encrypt_aes256_cbc, decrypt_aes256_cbc, sha256_checksum};

pub fn load_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

pub fn save_file(path: &str, data: &[u8]) -> Result<(), std::io::Error> {
    fs::write(path, data)
}

// mKmYyAFP8QMZDQPKCzAJsPwjXt7dpG0BrgbN7RhxQ+M=  key
// 4f02a09796005d68cb3735884e26cdf8e540c011f48765976f6432cb24b24443 chksum

fn max_allowed_file_size() -> u64 {
    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemAvailable:") {
                if let Some(kb_str) = line.split_whitespace().nth(1) {
                    if let Ok(kb) = kb_str.parse::<u64>() {
                        return (kb * 1024) / 4; // ¼ of free RAM in bytes
                    }
                }
            }
        }
    }
    5 * 1024 * 1024 // fallback: 5 MiB
}

pub fn show_encrypt_menu() {
    println!("\n\n\n  📦 Pack Mode Selected");
    println!("\n\n  WARNING:\n\n  Please ensure the target file to encrypt has been placed in /crypto-bro/data directory.\n  Any duplicate namespace in data/encrypted will be overwritten. \n\n  This program is experimental. Using filetypes other than .env or .txt may have unpredicatable results.");
    println!("\n  Max file size allowed: {:.2} MiB\n", max_allowed_file_size() as f64 / 1024.0 / 1024.0);
    println!("\n  Options:");
    println!("    [e] Encrypt file");
    println!("    [m] Main menu");
    println!("    [q] Exit the app\n");
    print!("\nType option (e, m, q) and press [Enter]: ");
}

pub fn encrypt_menu_loop() {
    loop {
        show_encrypt_menu();
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "e" => {
                let file_path = match select_file("data") {
                    Some(f) => f,
                    None => {
                        println!("\x1b[91mNo valid file selected.\x1b[0m");
                        continue;
                    }
                };

                let metadata = fs::metadata(&file_path).expect("Could not read file metadata");
                let file_size = metadata.len();
                let max_size = max_allowed_file_size();

                if file_size > max_size {
                    println!("\x1b[91mFile too large. Limit is {:.2} MiB\x1b[0m", max_size as f64 / 1024.0 / 1024.0);
                    continue;
                }

                let mut decryption_key = generate_key();
                loop {
                    println!("\n\x1b[92mGenerated Decryption Key:\x1b[0m\n\x1b[96m{}\x1b[0m", decryption_key);
                    println!("\nOptions:\n  [a] Accept key\n  [r] Regenerate\n  [q] Cancel\n");
                    print!("\nType option (a, r, q) and press [Enter]: ");
                    io::stdout().flush().unwrap();

                    let mut kinput = String::new();
                    io::stdin().read_line(&mut kinput).unwrap();
                    match kinput.trim() {
                        "a" => break,
                        "r" => decryption_key = generate_key(),
                        "q" => return,
                        _ => println!("\x1b[91mInvalid input.\x1b[0m"),
                    }
                }

                let plaintext = fs::read(&file_path).expect("Failed to read file.");
                let key_bytes = base64::decode(&decryption_key).expect("Invalid base64 key");
                let encrypted = encrypt_aes256_cbc(&plaintext, &key_bytes)
                    .expect("Encryption failed");

                let out_path = format!("data/encrypted/{}.locked",
                    Path::new(&file_path)
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                );
                fs::create_dir_all("data/encrypted").ok();
                fs::write(&out_path, &encrypted).expect("Failed to write encrypted file.");

                let checksum = sha256_checksum(&encrypted);

                println!("\n\n\x1b[92m✅ Encryption Complete\x1b[0m");
                println!("→ Encrypted file saved to: \x1b[96m{}\x1b[0m", out_path);
                println!("→ Integrity checksum: \x1b[95m{}\x1b[0m", checksum);
                println!("\n\x1b[93mIMPORTANT:\x1b[0m Keep your decryption key and checksum hash safe.\nThis is the only time you’ll see the key.\n");

                loop {
                    println!("Options:\n  [m] Main menu\n  [q] Quit");
                    let mut final_input = String::new();
                    io::stdin().read_line(&mut final_input).unwrap();
                    match final_input.trim() {
                        "m" => return,
                        "q" => std::process::exit(0),
                        _ => println!("\x1b[91mInvalid input.\x1b[0m"),
                    }
                }
            }
            "m" => return,
            "q" => std::process::exit(0),
            _ => println!("\x1b[91mInvalid choice.\x1b[0m"),
        }
    }
}

fn select_file(dir: &str) -> Option<String> {
    let entries = fs::read_dir(dir).ok()?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .collect::<Vec<_>>();

    if entries.is_empty() {
        println!("\x1b[91mNo files found in {}/\x1b[0m", dir);
        return None;
    }

    println!("\nAvailable files:\n");
    for (i, entry) in entries.iter().enumerate() {
        println!("  [{}] {}", i + 1, entry.path().display());
    }

    print!("\nSelect file by number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<usize>().ok()
        .and_then(|i| entries.get(i - 1))
        .map(|e| e.path().to_string_lossy().to_string())
}

fn generate_key() -> String {
    let mut rng = rand::thread_rng();
    let mut key_bytes = [0u8; 32];
    rng.fill(&mut key_bytes);
    base64::encode(key_bytes)
}

pub fn show_decrypt_menu() {
    println!("\n\n\n  📥 Unpack Mode Selected");
    println!("\n\n  WARNING:\n\n  Please ensure the target file to decrypt has been placed in /crypto-bro/data/encrypted directory.\n  Any duplicate namespace in data/decrypted will be overwritten.");
    println!("\n\n  For files secured by this program only. If a file was encrypted with a different tool or parameters,\n  this decryptor likely won’t work.");
    println!("\n  Max file size allowed: {:.2} MiB\n", max_allowed_file_size() as f64 / 1024.0 / 1024.0);
    println!("\n\n  Be prepared to provide your recovery key on the next screen.");
    println!("\n  Options:");
    println!("    [d] Decrypt file");
    println!("    [m] Main menu");
    println!("    [q] Exit the app\n");
    print!("\nType option (d, m, q) and press [Enter]: ");
}

pub fn decrypt_menu_loop() {
    loop {
        show_decrypt_menu();
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "d" => {
                let file_path = match select_file("data/encrypted") {
                    Some(f) => f,
                    None => {
                        println!("\x1b[91mNo valid file selected.\x1b[0m");
                        continue;
                    }
                };

                let metadata = fs::metadata(&file_path).expect("Could not read file metadata");
                let file_size = metadata.len();
                let max_size = max_allowed_file_size();

                if file_size > max_size {
                    println!("\x1b[91mFile too large. Limit is {:.2} MiB\x1b[0m", max_size as f64 / 1024.0 / 1024.0);
                    continue;
                }

                print!("Enter your Base64 encoded decryption key: ");
                io::stdout().flush().unwrap();
                let mut key_input = String::new();
                io::stdin().read_line(&mut key_input).unwrap();
                let key_input = key_input.trim();

                let key_bytes = match base64::decode(key_input) {
                    Ok(k) if k.len() == 32 => k,
                    _ => {
                        println!("\x1b[91mInvalid key format or length.\x1b[0m");
                        continue;
                    }
                };

                let ciphertext = match fs::read(&file_path) {
                    Ok(c) => c,
                    Err(_) => {
                        println!("\x1b[91mFailed to read encrypted file.\x1b[0m");
                        continue;
                    }
                };

                let decrypted: Vec<u8> = match decrypt_aes256_cbc(&ciphertext, &key_bytes) {
                    Ok(vec) => vec, 
                    Err(e) => {
                        println!("\x1b[91mDecryption failed: {}\x1b[0m", e);
                        continue;
                    }
                };

                let out_path = format!("data/decrypted/{}",
                    Path::new(&file_path)
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .replace(".locked", "")
                );

                fs::create_dir_all("data/decrypted").ok();
                if let Err(e) = fs::write(&out_path, &decrypted) {
                    println!("\x1b[91mFailed to write decrypted file: {}\x1b[0m", e);
                    continue;
                }

                let checksum = sha256_checksum(&decrypted);

                println!("\n\n\x1b[92m✅ Decryption Complete\x1b[0m");
                println!("→ Decrypted file saved to: \x1b[96m{}\x1b[0m", out_path);
                println!("→ Integrity checksum: \x1b[95m{}\x1b[0m", checksum);
                println!("\n\x1b[93mIMPORTANT:\x1b[0m Verify checksum matches expected value.\n");

                loop {
                    println!("Options:\n  [m] Main menu\n  [q] Quit");
                    let mut final_input = String::new();
                    io::stdin().read_line(&mut final_input).unwrap();
                    match final_input.trim() {
                        "m" => return,
                        "q" => std::process::exit(0),
                        _ => println!("\x1b[91mInvalid input.\x1b[0m"),
                    }
                }
            }
            "m" => return,
            "q" => std::process::exit(0),
            _ => println!("\x1b[91mInvalid choice.\x1b[0m"),
        }
    }
}