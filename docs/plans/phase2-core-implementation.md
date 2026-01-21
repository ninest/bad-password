# Phase 2: Core Implementation Plan

## Status: Pending

**Created:** 2026-01-21
**PRD Reference:** `/docs/prd.md` Section 6 - Phase 2

---

## Overview

This phase implements the core password generation functionality in Rust to achieve feature parity with the Python version. The CLI argument parsing is already complete from Phase 1; this phase focuses on the actual password generation logic.

---

## Tasks

### Task 1: Load Password Dictionary

**Description:** Read and parse the `common-passwords.txt` file.

**Implementation Details:**
```rust
// Location: src/main.rs
// Use std::fs::read_to_string() to load the file
// Split by newlines to get Vec<String>
// Handle file-not-found gracefully
```

**Acceptance Criteria:**
- [ ] Loads `common-passwords.txt` from current working directory
- [ ] Parses file into a `Vec<String>` with one password per line
- [ ] Handles file-not-found with error message to stderr and exit code 1
- [ ] Handles empty file with appropriate error message

**Reference (Python):**
```python
file = open("./common-passwords.txt", "r")
dictionary_words = file.read().splitlines()
```

---

### Task 2: Implement Warning Messages

**Description:** Print warnings when users try to make passwords "too secure."

**Implementation Details:**
- If `words > 1`: Print warning and reset to 1
- If `symbols > 0`: Print warning (symbols are ignored anyway)

**Acceptance Criteria:**
- [ ] Warning printed to stdout when `--words` > 1
- [ ] Warning printed to stdout when `--symbols` > 0
- [ ] Warning text matches Python output exactly

**Reference (Python):**
```python
if words > 1:
    print("WARNING: Password might actually become secure with more than 1 common word. Using 1 word.")
    words = 1
if symbols > 0:
    print("WARNING: Special characters may make your password secure. Using 0 specials characters.")
```

---

### Task 3: Select Random Password

**Description:** Randomly select one password from the loaded dictionary.

**Implementation Details:**
```rust
// Use rand::seq::SliceRandom trait
// Call .choose() on the password vector
// Handle empty vector case
```

**Dependencies:**
- `rand` crate (already in Cargo.toml)

**Acceptance Criteria:**
- [ ] Selects one random password from the dictionary
- [ ] Uses `rand::seq::SliceRandom::choose()`
- [ ] Thread-local RNG for performance

**Reference (Python):**
```python
password = random.choice(dictionary_words)
```

---

### Task 4: Implement `--caps` Transformation

**Description:** Capitalize the first letter of the password.

**Implementation Details:**
```rust
// Rust strings are UTF-8, need custom capitalize function
// Get first char, uppercase it, append rest of string
// Handle empty string edge case
```

**Acceptance Criteria:**
- [ ] Capitalizes first character of password
- [ ] Handles UTF-8 correctly
- [ ] Prints confirmation: `"✓ Capitalized first letter for maximum security!"`

**Reference (Python):**
```python
if caps:
    password = password.capitalize()
    print("✓ Capitalized first letter for maximum security!")
```

---

### Task 5: Implement `--numbers` Transformation

**Description:** Append a predictable number to the password.

**Implementation Details:**
```rust
// Predictable numbers: ["1", "123", "12345", current_year, "1234"]
// Use chrono::Local::now().year() for current year
// Randomly select one and append to password
```

**Dependencies:**
- `rand` crate for random selection
- `chrono` crate for current year

**Acceptance Criteria:**
- [ ] Appends one of: `1`, `123`, `12345`, `<current_year>`, `1234`
- [ ] Random selection from the five options
- [ ] Prints confirmation: `"✓ Added ultra-secure numbers: {chosen_number}"`

**Reference (Python):**
```python
if numbers:
    predictable_numbers = ["1", "123", "12345", str(datetime.now().year), "1234"]
    chosen_number = random.choice(predictable_numbers)
    password += chosen_number
    print(f"✓ Added ultra-secure numbers: {chosen_number}")
```

---

### Task 6: Implement `--exclamation` Transformation

**Description:** Append an exclamation mark to the password.

**Implementation Details:**
```rust
// Simply append "!" to the password string
```

**Acceptance Criteria:**
- [ ] Appends `!` to the end of the password
- [ ] Prints confirmation: `"✓ Added exclamation mark (now unhackable!)"`

**Reference (Python):**
```python
if exclamation:
    password += "!"
    print("✓ Added exclamation mark (now unhackable!)")
```

---

### Task 7: Output Final Password

**Description:** Print the generated password to stdout.

**Acceptance Criteria:**
- [ ] Password printed as the last line of output
- [ ] Uses `println!` for proper newline handling

**Reference (Python):**
```python
print(password)
```

---

## Implementation Order

Execute tasks in this sequence:

1. **Task 1** - Load password dictionary (foundation for everything)
2. **Task 2** - Warning messages (simple, validates args are processed)
3. **Task 3** - Random password selection (core functionality)
4. **Task 4** - `--caps` transformation
5. **Task 5** - `--numbers` transformation (depends on chrono)
6. **Task 6** - `--exclamation` transformation
7. **Task 7** - Final output

---

## Code Structure

The final `main.rs` should have this structure:

```rust
//! bad-password: A satirical weak password generator

use clap::Parser;
use rand::seq::SliceRandom;
use chrono::Datelike;
use std::fs;
use std::process;

#[derive(Parser, Debug)]
#[command(name = "bad-password")]
#[command(about = "Generate a secure, memorable password using the XKCD method")]
struct Args {
    // ... existing args from Phase 1
}

/// Capitalize the first character of a string
fn capitalize_first(s: &str) -> String {
    // Implementation
}

/// Load passwords from file
fn load_passwords(path: &str) -> Result<Vec<String>, std::io::Error> {
    // Implementation
}

fn main() {
    let args = Args::parse();

    // 1. Process warnings for words/symbols
    // 2. Load password dictionary
    // 3. Select random password
    // 4. Apply transformations in order: caps, numbers, exclamation
    // 5. Print final password
}
```

---

## Verification Checklist

After implementation, verify:

- [ ] `cargo build` compiles without errors
- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo fmt --check` passes
- [ ] `./target/debug/bad-password` generates a password
- [ ] `./target/debug/bad-password -c` capitalizes first letter
- [ ] `./target/debug/bad-password -n` appends a number
- [ ] `./target/debug/bad-password -e` appends exclamation mark
- [ ] `./target/debug/bad-password -c -n -e` combines all transformations
- [ ] `./target/debug/bad-password -w 5` shows warning and uses 1 word
- [ ] `./target/debug/bad-password -s 3` shows symbols warning
- [ ] Missing `common-passwords.txt` shows error and exits with code 1

---

## Dependencies Used

| Crate | Feature | Usage |
|-------|---------|-------|
| `clap` | derive | CLI argument parsing (Phase 1) |
| `rand` | default | Random password and number selection |
| `chrono` | default | Get current year for `--numbers` |

---

## Estimated Complexity

| Task | Complexity | Lines of Code |
|------|------------|---------------|
| Task 1 | Low | ~15 lines |
| Task 2 | Low | ~10 lines |
| Task 3 | Low | ~5 lines |
| Task 4 | Medium | ~10 lines |
| Task 5 | Low | ~10 lines |
| Task 6 | Low | ~5 lines |
| Task 7 | Low | ~1 line |
| **Total** | **Low-Medium** | **~56 lines** |

---

## Notes

- Transformations must be applied in order: caps, then numbers, then exclamation (matching Python)
- The emoji checkmarks (✓) are important for matching Python output
- File path is hardcoded as `./common-passwords.txt` (relative to current working directory)
- This is an educational/satirical project - the "weak" behavior is intentional
