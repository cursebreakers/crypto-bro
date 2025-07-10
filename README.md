# Key Generator | Rust

**v0.0.9** - "crypto-bro"

*A CLI tool for generating credentials, cryptographic keys and ID's.*

---

## Features

- **OpenSSL Cryptographic Key (256-bit)**: Generates a cryptographic key using OpenSSL.
- **Ring Cryptographic Key (256-bit)**: Generates a cryptographic key using the Ring library.
- **UUID (v4 format)**: Generates a version 4 UUID.
- **API Key (Base64 encoded, 256-bit)**: Generates a Base64 encoded API key.
- **Passwords: (16 and 32 characters)**: Generates random passwords.
- **Usernames (Word list Generated)**: Generate usernames using word lists.
- **File Encryption/Decryption**: Secure your files using 256-bit encryption. *EXPERIMENTAL*
---

## Installation

*If needed, install Rust*

`curl https://sh.rustup.rs -sSf | sh`

1. Fork the repository:

`git clone https://github.com/cursebreakers/crypto-bro`

2. Compile and launch the program:

`cd crypto-bro && cargo run`

---

### Usage

There a few arguments that can be used with this program.

**Info**

`cargo run -- --help` or `cargo run -- -h` displays the help menu.

`cargo run -- --version` prints the app version.

**Quick-start**

Use `cargo run` with the following flags to quickly return a key of choice.

`cargo run -- -k (1, 2, 3, 4, 5, 6 or 7)` 

The numbers 1-7 correspond with the "Main menu" below.

Flag options have not yet been added for the file encrypt/decrypt feature, but are coming soon.

*examples:*

`cargo run -- -k 4` would immediately generate an API key.

`cargo run -- -k 6` would generate a 32 character password.

And so on.

---

## Main menu

Once initialized, you may select an option from the main menu:

1. OpenSSL Cryptographic Key  (Base64 encoded, 256-bit)
2. Ring Cryptographic Key     (Base64 encoded, 256-bit)
3. UUID                       (v4 format)
4. API Key                    (Base64 encoded, 256-bit)
5. Password                   (16-characters)
6. Password                   (32-characters)
7. Username                   (Word list generated)
8. Encrypt file               (AES 256-bit)
9. Decrypt file               (AES 256-bit)
q. Quit

---

## Submenu

The submenu will open once a key is generated, whether from the main menu or quick start. 

The submenu displays four options:

**Copy to clipboard**

- Select "c" to copy the key to your clipboard.

**Regenerate**

- Select "r" to regenerate the key.

**Main menu**

- Select "m" to return to the main menu.

**Quit**

- Select "q" to terminate the program.

---

## Generator options

Most options are straightforward. Just select the key type and it will be generated. Usernames are a bit more fun.

### OpenSSL Cryptographic Key (256-bit)

*No special configurations for OpenSSL Cryptographic Key (256-bit) at this time.*

### Ring Cryptographic Key (256-bit)

*No special configurations for Ring Cryptographic Key (256-bit) at this time.*

### UUID (v4 format)

*No special configurations for UUID (v4 format) at this time.*

### API Key (Base64 encoded, 256-bit)

*No special configurations for API Key (Base64 encoded, 256-bit) at this time.*

### Passwords: (16 and 32 characters)

*No special configurations for Passwords: (16 and 32 characters) at this time.*

### Usernames (Word List Generated)

**Word Lists**

You are encouraged to edit and customize your word lists. They can be found under *words* in the *src* folder.

- *names.txt* provides noun type values.
- *descriptors.txt* provides adjective types.

*NOTE: You will need to relaunch the program to use the new words you've added.*

---

## File Encryption/Decryption

Options 8 and 9 in the main menu use a CBC AES-256 engine to encode and decode your files.

**How it works:**
- Encryption:
  1. *Make sure you place a file in the "crypto-bro/data" directory first.*
  2. At the submenu, you will be prompted to select target file.
  3. Next, you will generate a key. *Don't lose it!*
  4. Once you accept the key, the file will be encrypted.
  5. Once complete, you will be given a checksum for the encrypted file.
- Decryption:
  1. *Make sure you place a file in the "crypto-bro/data/encrypted" directory first.*
  2. At the submenu, you will be prompted to select target file.
  3. Next, you will be asked to provide your key. *You saved it, right?*
  4. Once you enter the key, the file will be decrypted.
  5. Once complete, you will be given a checksum for the encrypted file.

*NOTE:* Max filesize is set to 25% of your device's available RAM by default. This can be adjusted by tweaking the `max_allowed_file_size` function in `src/lunchbox.rs`

---

# WORKING/NEXT

- Known bug: submenu option "c" to copy key to clipboard doesn't always work on the first try
- Improving the greeting/ascii art appearance
- Checksums for decryption seem to be malfunctioning (often).

**FUTURE**

- Add a man/help option to main menu
- Batch key generation
- Export/save to file

- Password/username variance
  - Improved/expanded word lists

- Syntactical flags for menu options
  - -o flag: OpenSSL
  - -r flag: Ring
  - -u flag: UUIDv4
  - -a flag: API key
  - -p flag: passwords
    - two args (16 or 32)
  - -n flag: username

- Expanded flags/arguments (and man pages!)
  - var flags for pass and unames?
  - -b flag: generate batches of keys
  - -exp flag: pipe mode
    - bypass menu, allowing generated keys to be piped elsewhere
    - txt and md args for saving keys/batches to files
- pipe mode for file encryption/decryption invoked directly from cli

*This is the planning section. These features are (99% likely) not yet implemented, nor may they ever be.* 

Please [let me know](mailto:hello@cursebreakers.net) if you would like to request a feature.

---

# Credits & Acknowledgements

## Author: 

Esau @ [Cursebreakers LLC](https://cursebreakers.net)

## Built with:

**Rust**

Crates used in this program.

| Dependency          | Version | Purpose                                         |
|---------------------|---------|-------------------------------------------------|
| `aes`               | 0.7     | AES encryption                                  |
| `arboard`           | 3.2     | Clipboard management                            |
| `base64`            | 0.13    | Base64 encoding and decoding                    |
| `chacha20poly1305`  | 0.8     | ChaCha20-Poly1305 encryption                    |
| `clap`              | 4.5     | Command-line argument parser                    |
| `hmac`              | 0.11    | HMAC (Hash-based Message Authentication)        |
| `openssl`           | 0.10    | Cryptographic operations                        |
| `pbkdf2`            | 0.9     | Password-based key derivation function (PBKDF2) |
| `rand`              | 0.8     | Random number generation                        |
| `ring`              | 0.16    | Cryptographic primitives                        |
| `uuid`              | 1.0     | UUID generation (v4 feature enabled)            |

Honorable mention to [cargo-mommy](https://github.com/Gankra/cargo-mommy), for making programming with Rust much more fun.

## License:

This project is to be released under either MIT, Apache 2.0 or both.

## Contributions:

Contributions are welcome! Feel free to [submit issues](mailto:hello@cursebreakers.net), pull requests, or suggestions for improvement.

