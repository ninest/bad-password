//! bad-password: A satirical weak password generator
//!
//! This tool intentionally generates weak passwords for educational purposes,
//! demonstrating common insecure password patterns.

use clap::Parser;

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

fn main() {
    let args = Args::parse();

    // TODO: Implement in Phase 2
    println!("bad-password CLI initialized");
    println!("Args: {:?}", args);
}
