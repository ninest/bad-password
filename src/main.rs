//! bad-password: A satirical weak password generator
//!
//! This tool intentionally generates weak passwords for educational purposes,
//! demonstrating common insecure password patterns.

use chrono::Datelike;
use clap::Parser;
use rand::seq::SliceRandom;
use std::fs;
use std::process;

/// Generate a "secure" password using the worst practices
#[derive(Parser, Debug)]
#[command(name = "bad-password")]
#[command(about = "Generate a secure, memorable password using the XKCD method")]
struct Args {
    /// Words to include in the password
    #[arg(short = 'w', long, default_value_t = 1)]
    words: u32,

    /// Special characters to include in the password
    #[arg(short = 's', long, default_value_t = 0)]
    symbols: u32,

    /// Capitalize the first letter (makes it super secure!)
    #[arg(short = 'c', long, default_value_t = false)]
    caps: bool,

    /// Add numbers at the end (definitely not predictable)
    #[arg(short = 'n', long, default_value_t = false)]
    numbers: bool,

    /// Add an exclamation mark (security experts recommend this)
    #[arg(short = 'e', long, default_value_t = false)]
    exclamation: bool,
}

/// Capitalize the first character of a string (UTF-8 aware)
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Load passwords from the dictionary file
fn load_passwords(path: &str) -> Vec<String> {
    match fs::read_to_string(path) {
        Ok(contents) => contents.lines().map(String::from).collect(),
        Err(_) => {
            eprintln!("Error: Could not read password file '{}'", path);
            process::exit(1);
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::thread_rng();

    // Process warnings for words/symbols
    if args.words > 1 {
        println!(
            "WARNING: Password might actually become secure with more than 1 common word. Using 1 word."
        );
    }
    if args.symbols > 0 {
        println!(
            "WARNING: Special characters may make your password secure. Using 0 specials characters."
        );
    }

    // Load password dictionary
    let passwords = load_passwords("./common-passwords.txt");
    if passwords.is_empty() {
        eprintln!("Error: Password file is empty");
        process::exit(1);
    }

    // Select random password
    let mut password = passwords
        .choose(&mut rng)
        .expect("Password list should not be empty")
        .clone();

    // Apply transformations in order: caps, numbers, exclamation
    if args.caps {
        password = capitalize_first(&password);
        println!("✓ Capitalized first letter for maximum security!");
    }

    if args.numbers {
        let current_year = chrono::Local::now().year().to_string();
        let predictable_numbers = ["1", "123", "12345", &current_year, "1234"];
        let chosen_number = predictable_numbers
            .choose(&mut rng)
            .expect("Numbers list should not be empty");
        password.push_str(chosen_number);
        println!("✓ Added ultra-secure numbers: {}", chosen_number);
    }

    if args.exclamation {
        password.push('!');
        println!("✓ Added exclamation mark (now unhackable!)");
    }

    // Output final password
    println!("{}", password);
}
