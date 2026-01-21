# bad-password

A satirical weak password generator that creates intentionally insecure passwords for **educational purposes**.

This tool demonstrates common password anti-patterns and poor security practices. Use it to learn what *not* to do when creating passwords.

## Installation

### Prerequisites

- Rust 1.70 or later

### Build from source

```bash
# Clone the repository
git clone https://github.com/ninest/bad-password.git
cd bad-password

# Build the release version
cargo build --release

# The binary will be at ./target/release/bad-password
```

### Run directly with Cargo

```bash
cargo run -- [OPTIONS]
```

## Usage

```bash
bad-password [OPTIONS]
```

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--words <N>` | `-w` | Number of words to include (max: 1 enforced) |
| `--symbols <N>` | `-s` | Special characters (ignored for your "safety") |
| `--caps` | `-c` | Capitalize first letter |
| `--numbers` | `-n` | Append predictable numbers |
| `--exclamation` | `-e` | Add exclamation mark at end |
| `--help` | `-h` | Show help information |

## Examples

### Generate a basic weak password

```bash
$ bad-password
princess
```

### Add capitalization for "maximum security"

```bash
$ bad-password --caps
✓ Capitalized first letter for maximum security!
Sunshine
```

### Add predictable numbers

```bash
$ bad-password -n
✓ Added ultra-secure numbers: 123
qwerty123
```

### Combine multiple options

```bash
$ bad-password -c -n -e
✓ Capitalized first letter for maximum security!
✓ Added ultra-secure numbers: 2026
✓ Added exclamation mark (now unhackable!)
Dragon2026!
```

### Try to be "too secure" (the tool will stop you)

```bash
$ bad-password --words 5 --symbols 3
WARNING: Password might actually become secure with more than 1 common word. Using 1 word.
WARNING: Special characters may make your password secure. Using 0 specials characters.
password
```

## How it works

1. Selects a random password from a list of 9,999 common passwords
2. Optionally applies "security enhancements":
   - **Capitalization**: Makes the first letter uppercase
   - **Numbers**: Appends one of: `1`, `123`, `12345`, current year, or `1234`
   - **Exclamation**: Adds `!` at the end

## Why?

This project exists to demonstrate:

- Why dictionary-based passwords are weak
- Why predictable number suffixes don't add security
- Why single-character special characters are easily guessed
- Common patterns that password crackers check first

**For actual password security**, use a reputable password manager with randomly generated passwords of 16+ characters.

## License

MIT
