# Key Generator | Rust

**v0.0.9** - "crypto_bro"

*A CLI tool for generating credentials, cryptographic keys and ID's.*

---

## Features

- **OpenSSL Cryptographic Key (256-bit)**: Generates a cryptographic key using OpenSSL.
- **Ring Cryptographic Key (256-bit)**: Generates a cryptographic key using the Ring library.
- **UUID (v4 format)**: Generates a version 4 UUID.
- **API Key (Base64 encoded, 256-bit)**: Generates a Base64 encoded API key.
- **Passwords: (16 and 32 characters)**: Generates random passwords.
- **Usernames (Word List Generated)**: Generate usernames using word lists.

---

## Installation

*If needed, install Rust*

`curl https://sh.rustup.rs -sSf | sh`

1. Fork the repository:

`gh repo fork https://github.com/cursebreakers/crypto-bro.git --clone`

2. Compile and launch the program:

`cd crypto-bro && cargo run`

---

### Usage

There a few arguments that can be used with this program.

**Quick-start**

Use `cargo run` with the following flags to quickly return a key of choice.

`-- -k (1, 2, 3, 4, 5, 6 or 7)` (corresponds with the "Main menu" below)

eg: `cargo run -- -k 4` immediately generates an API key.

**Info**

`-- -h --help` - displays the help menu

`-- --version` - prints the app version

---

## Main menu

Once initialized, you may select an option from the main menu:

1. OpenSSL Cryptographic Key  (Base64 encoded, 256-bit)
2. Ring Cryptographic Key     (Base64 encoded, 256-bit)
3. UUID                       (v4 format)
4. API Key                    (Base64 encoded, 256-bit)
5. Password                   (16-characters)
6. Password                   (32-characters)
7. Username                   (Randomly generated)

q. Quit

---

## Sub menu

The sub menu will open once a key is generated. This displays four options:

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

# WORKING/NEXT

- Improving the greeting/ascii art appearance

**FUTURE**
- Listener for option keys (instead of pressing enter to confirm choice)
- Batch key generation
- Export/save keys to files
- Password/username variance
  - Improved/expanded word lists
- Expanded flags/arguments and man pages

*This is the planning section. These features are (99% likely) not yet implemented, nor may they ever be.* 

Please [let me know](mailto:hello@cursebreakers.net) if you would like to request a feature.

---

# Credits & Acknowledgements

## Author: 

Esau @ [Cursebreakers LLC](https://cursebreakers.net)

## Dependencies:

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

## License:

This project is (to be) released under the MIT License.

## Contributions:

Contributions are welcome! Feel free to [submit issues](mailto:hello@cursebreakers.net), pull requests, or suggestions for improvement.

