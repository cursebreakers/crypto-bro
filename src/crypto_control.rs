// crypto_control.rs

use rand::RngCore;
use rand::Rng;
use uuid::Uuid;
use openssl::rand::rand_bytes;
use openssl::symm::{encrypt, decrypt, Cipher};
use ring::rand::{SecureRandom, SystemRandom};
use sha2::{Sha256, Digest};
use base64;
use arboard::Clipboard;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Generates a 256-bit key using OpenSSL
pub fn generate_openssl_key() -> String {
	println!();
	println!("Generating...");
	println!();
    let mut key = vec![0u8; 32];
    rand_bytes(&mut key).unwrap();
    base64::encode(&key)
}

/// Generates a 256-bit key using Ring
pub fn generate_ring_key() -> String {
	println!();
	println!("Generating...");
	println!();
    let rng = SystemRandom::new();
    let mut key = vec![0u8; 32];
    rng.fill(&mut key).unwrap();
    base64::encode(&key)
}

/// Generates a UUID
pub fn generate_uuid() -> String {
	println!();
	println!("Generating...");
	println!();
    Uuid::new_v4().to_string()
}

/// Generates an API key using Base64
pub fn generate_api_key() -> String {
	println!();
	println!("Generating...");
	println!();
    let mut random_bytes = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut random_bytes);
    base64::encode(random_bytes)
}

/// Generates a 16 or 32 bit password
pub fn generate_password(length: u32) -> String {
    // Define possible characters for the password
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                   abcdefghijklmnopqrstuvwxyz\
                   0123456789!@#$%^&*()-_=+";
    let mut rng = rand::thread_rng();

    // Choose the password length based on the input (16 or 32)
    let password_length = match length {
        16 | 32 => length as usize,
        _ => {
            println!("\x1b[91mInvalid password length! Please choose 16 or 32.\x1b[0m");
            return String::new();  // Return an empty string for invalid input
        }
    };

    // Generate random password
    let password: Vec<u8> = (0..password_length)
        .map(|_| charset[rng.next_u32() as usize % charset.len()] as u8)
        .collect();

    // Convert the password bytes to a string
    password.iter().map(|&byte| byte as char).collect::<String>()
}

// Loads word lists
fn load_word_list(path: &str) -> Vec<String> {
    let path = Path::new(path);
    let file = File::open(&path).expect("Could not open file");
    let reader = io::BufReader::new(file);
    
    reader
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>()
}

// Uses word lists to gen a username
pub fn generate_username() -> String {
    let descriptors = load_word_list("src/words/descriptors.txt");
    let names = load_word_list("src/words/names.txt");

    let mut rng = rand::thread_rng();

    // Randomly select descriptors and names, occasionally adding underscores
    let descriptor = descriptors[rng.gen_range(0..descriptors.len())].clone();
    let name = names[rng.gen_range(0..names.len())].clone();
	let add_underscore: bool = rng.gen_bool(0.72); 

    // Combine them into a username
	if add_underscore {
        format!("{}_{}", descriptor, name)
    } else {
        format!("{}{}", descriptor, name)
    }
}

/// Copies text to clipboard
pub fn copy_to_clipboard(text: &str) -> bool {
    if let Ok(mut clipboard) = Clipboard::new() {
        if clipboard.set_text(text.to_string()).is_ok() {
            // Verify that the clipboard has the correct content
			// DO NOT REMOVE - COPY FUNCTION IS NOT EFFECTIVE WITHOUT IT
            if let Ok(clip_content) = clipboard.get_text() {
                if clip_content == text {
                    return true;
                }
            }
        }
    }
    false
}

/// Encrypts a plaintext using AES-256-CBC with the given 32-byte key
pub fn encrypt_aes256_cbc(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes.".into());
    }
    let mut iv = [0u8; 16];
    rand::thread_rng().fill(&mut iv);

    
    let mut ciphertext = encrypt(Cipher::aes_256_cbc(), key, Some(&iv), plaintext)
        .map_err(|e| format!("Encryption failed: {}", e))?;
    
    let mut result = iv.to_vec();
    result.append(&mut ciphertext);
    Ok(result)
}

/// Decrypts AES-256-CBC ciphertext using the provided 32-byte key
pub fn decrypt_aes256_cbc(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes.".into());
    }
    if ciphertext.len() < 16 {
        return Err("Ciphertext too short.".into());
    }

    let (iv, data) = ciphertext.split_at(16);
    openssl::symm::decrypt(Cipher::aes_256_cbc(), key, Some(iv), data)
        .map_err(|e| format!("Decryption failed: {}", e))
}

pub fn sha256_checksum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}