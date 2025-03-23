// ascii.rs

use std::fs::File;
use std::io::{self, Read};
use rand::Rng;

#[derive(Debug)]
pub struct AsciiImages {
    pub image1: &'static str,
    pub image2: &'static str,
}

impl AsciiImages {
    fn random_image(&self) -> &str {
        let mut rng = rand::thread_rng();
        let random_choice = rng.gen_range(0..2);

        if random_choice == 0 {
            self.image1
        } else {
            self.image2
        }
    }

    // Function to print the selected ASCII art
    pub fn print_random_ascii(&self) -> io::Result<()> {
        let filename = self.random_image();
        let mut file = File::open(filename)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        println!("\x1b[96m{}\x1b[0m", content);
        Ok(())
    }
}

