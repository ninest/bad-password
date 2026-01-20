# Phase 1: Rust Project Setup - Implementation Plan

**Goal:** Initialize the Rust project structure with all dependencies configured and ready for core implementation.

---

## Overview

Phase 1 focuses on setting up the foundational Rust project structure. This is a straightforward task that involves creating the Cargo project, configuring dependencies, and establishing the basic file layout.

---

## Tasks

### Task 1.1: Initialize Cargo Project

**What to do:**
```bash
cargo init
```

This will create:
- `Cargo.toml` - Package manifest
- `src/main.rs` - Entry point with hello world placeholder

**Notes:**
- Run this in the existing `bad-password` directory
- The `cargo init` command works with existing files (unlike `cargo new`)

---

### Task 1.2: Configure Cargo.toml

**Replace the auto-generated `Cargo.toml` with:**

```toml
[package]
name = "bad-password"
version = "0.1.0"
edition = "2021"
description = "A satirical weak password generator"
license = "MIT"

[dependencies]
clap = { version = "4", features = ["derive"] }
rand = "0.8"
chrono = "0.4"

[profile.release]
lto = true
strip = true
```

**Dependency Justification:**
| Crate | Version | Purpose |
|-------|---------|---------|
| `clap` | 4.x with derive | CLI argument parsing matching Python's argparse functionality |
| `rand` | 0.8 | Random password selection from dictionary |
| `chrono` | 0.4 | Get current year for `--numbers` option |

**Build Profile Notes:**
- `lto = true` - Link-time optimization for smaller, faster binary
- `strip = true` - Strip debug symbols from release build

---

### Task 1.3: Create src/main.rs Skeleton

**Create a minimal skeleton that compiles:**

```rust
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
```

---

### Task 1.4: Verify Build

**Run these commands to verify setup:**

```bash
# Check that dependencies resolve
cargo check

# Build debug version
cargo build

# Run with --help to verify CLI
cargo run -- --help

# Build optimized release
cargo build --release
```

**Expected Results:**
- `cargo check` - No errors, dependencies download successfully
- `cargo build` - Compiles successfully
- `cargo run -- --help` - Shows help text with all options
- `cargo build --release` - Creates optimized binary in `target/release/`

---

## Verification Checklist

After completing Phase 1, verify:

- [ ] `Cargo.toml` exists with correct dependencies
- [ ] `src/main.rs` exists and compiles
- [ ] `cargo check` passes with no errors
- [ ] `cargo build` creates debug binary
- [ ] `cargo run -- --help` shows correct CLI options:
  - `-w, --words`
  - `-s, --symbols`
  - `-c, --caps`
  - `-n, --numbers`
  - `-e, --exclamation`
- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo fmt --check` passes (code is formatted)

---

## File Structure After Phase 1

```
bad-password/
├── Cargo.toml              # NEW: Rust package manifest
├── Cargo.lock              # NEW: Dependency lock file (auto-generated)
├── src/
│   └── main.rs             # NEW: CLI skeleton
├── target/                 # NEW: Build artifacts (gitignored)
├── common-passwords.txt    # EXISTING: Password dictionary
├── bad-password.py         # EXISTING: Python reference
├── README.md               # EXISTING: Documentation
└── PRD-PYTHON-TO-RUST.md   # EXISTING: PRD document
```

---

## Next Steps (Phase 2 Preview)

After Phase 1 is complete, Phase 2 will implement:
1. Password file loading (`common-passwords.txt`)
2. Random password selection
3. Transformation functions (`--caps`, `--numbers`, `--exclamation`)
4. Warning messages matching Python behavior

---

## Notes

- The `target/` directory should be added to `.gitignore` if not already present
- The `Cargo.lock` should be committed for binary projects (ensures reproducible builds)
- Keep the Python file (`bad-password.py`) for reference during development
