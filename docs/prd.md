# Product Requirements Document: Python to Rust Conversion

## Project: bad-password

**Version:** 1.0
**Date:** 2026-01-20
**Status:** In Progress (Phase 1 Complete)

---

## 1. Executive Summary

This document outlines the plan to convert the `bad-password` project from Python to Rust. The project is a satirical password generator that intentionally creates weak passwords for educational purposes, demonstrating common insecure password patterns.

### 1.1 Current State
- Single Python file (`bad-password.py`, ~84 lines)
- Zero external dependencies (uses only Python standard library)
- CLI tool with argparse-based argument handling
- Reads from a dictionary file of 9,999 common passwords

### 1.2 Target State
- Native Rust binary with equivalent functionality
- Improved performance and memory safety
- Cross-platform distribution without Python runtime dependency
- Maintained satirical/educational purpose

---

## 2. Goals and Objectives

### 2.1 Primary Goals
1. **Feature Parity**: Maintain all existing CLI options and behavior
2. **Performance**: Leverage Rust's performance for faster execution
3. **Distribution**: Enable easy distribution as a single binary (no runtime required)
4. **Learning**: Serve as an example of Python-to-Rust migration

### 2.2 Non-Goals
- Adding new features beyond current functionality
- Changing the satirical nature of the project
- Modifying the password dictionary file

---

## 3. Functional Requirements

### 3.1 Command-Line Interface

The Rust version must support identical CLI arguments:

| Argument | Short | Long | Type | Default | Description |
|----------|-------|------|------|---------|-------------|
| Words | `-w` | `--words` | integer | 1 | Number of words (max: 1) |
| Symbols | `-s` | `--symbols` | integer | 0 | Special characters (ignored) |
| Caps | `-c` | `--caps` | flag | false | Capitalize first letter |
| Numbers | `-n` | `--numbers` | flag | false | Append predictable numbers |
| Exclamation | `-e` | `--exclamation` | flag | false | Add exclamation mark |

### 3.2 Core Functionality

1. **Password Dictionary Loading**
   - Read `common-passwords.txt` from the same directory as the binary
   - Handle file-not-found gracefully with error message
   - Parse one password per line

2. **Random Password Selection**
   - Select one random password from the dictionary
   - Use cryptographically-appropriate randomness (or intentionally weak randomness to match Python's `random` module behavior)

3. **Password Transformations**
   - `--caps`: Capitalize the first character of the password
   - `--numbers`: Append one of: `1`, `123`, `12345`, current year, `1234` (randomly selected)
   - `--exclamation`: Append `!` to the end of the password

4. **Output**
   - Print the generated password to stdout
   - No trailing newline formatting changes from Python behavior

### 3.3 Behavioral Requirements

1. Maintain satirical help text messaging (e.g., "Add a number for extra security")
2. Preserve the intentionally limited functionality (single word only)
3. Match Python output exactly for the same transformations

---

## 4. Non-Functional Requirements

### 4.1 Performance
- Binary startup time < 50ms
- Password generation < 10ms
- Memory usage < 10MB

### 4.2 Compatibility
- Support Linux, macOS, and Windows
- Binary should work without any runtime dependencies

### 4.3 Code Quality
- Follow Rust idioms and best practices
- Use `clippy` for linting
- Format with `rustfmt`
- Include inline documentation

### 4.4 Build Requirements
- Minimum Rust version: 1.70+
- Use Cargo for build management
- Provide release build instructions

---

## 5. Technical Specifications

### 5.1 Recommended Crate Dependencies

| Crate | Purpose | Justification |
|-------|---------|---------------|
| `clap` | CLI argument parsing | Industry standard, derives macro support |
| `rand` | Random number generation | Standard for randomness in Rust |
| `chrono` | Date/time handling | For current year in `--numbers` option |

### 5.2 Project Structure

```
bad-password/
├── Cargo.toml              # Rust package manifest
├── Cargo.lock              # Dependency lock file
├── src/
│   └── main.rs             # Main application code
├── docs/
│   ├── prd.md              # This document
│   └── plans/              # Implementation plans
├── common-passwords.txt    # Existing password dictionary
├── bad-password.py         # Original Python (kept for reference)
└── README.md               # Updated with Rust instructions
```

### 5.3 Error Handling

| Scenario | Behavior |
|----------|----------|
| Missing password file | Print error to stderr, exit with code 1 |
| Empty password file | Print error to stderr, exit with code 1 |
| Invalid arguments | Display help text via clap, exit with code 2 |

---

## 6. Implementation Plan

### Phase 1: Project Setup ✅
- [x] Initialize Cargo project
- [x] Add dependencies to `Cargo.toml`
- [x] Create basic project structure

### Phase 2: Core Implementation
- [ ] Implement CLI argument parsing with `clap`
- [ ] Implement password file loading
- [ ] Implement random password selection
- [ ] Implement transformation functions (`--caps`, `--numbers`, `--exclamation`)

### Phase 3: Polish and Testing
- [ ] Add error handling for edge cases
- [ ] Write unit tests for transformation functions
- [ ] Test CLI argument combinations
- [ ] Verify output matches Python version

### Phase 4: Documentation and Release
- [ ] Update README with Rust build/usage instructions
- [ ] Add inline code documentation
- [ ] Create release build for multiple platforms
- [ ] Archive or deprecate Python version

---

## 7. Testing Strategy

### 7.1 Unit Tests
- Test each transformation function independently
- Test password file parsing
- Test random selection distribution

### 7.2 Integration Tests
- Compare output with Python version for same inputs
- Test all CLI argument combinations
- Test error conditions (missing file, etc.)

### 7.3 Manual Testing Checklist
- [ ] `./bad-password` generates a password
- [ ] `./bad-password -c` capitalizes first letter
- [ ] `./bad-password -n` appends a number
- [ ] `./bad-password -e` appends exclamation mark
- [ ] `./bad-password -c -n -e` combines all transformations
- [ ] `./bad-password --help` shows help text
- [ ] Error message when `common-passwords.txt` is missing

---

## 8. Success Criteria

1. **Functional**: All CLI options work identically to Python version
2. **Performance**: Binary executes faster than Python script
3. **Distribution**: Single binary runs on target platforms without dependencies
4. **Code Quality**: Passes `cargo clippy` with no warnings
5. **Testing**: All unit and integration tests pass

---

## 9. Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Random number differences | Output may differ from Python | Document that randomness is intentionally different per run |
| Cross-platform file paths | May fail on some OSes | Use `std::path` for portable path handling |
| Unicode handling | Password transformations may differ | Test with Unicode passwords in dictionary |

---

## 10. Open Questions

1. Should the Python version be removed after Rust conversion is complete?
2. Should we embed the password dictionary in the binary for true single-file distribution?
3. Should we add `--version` flag to the CLI?

---

## Appendix A: Python to Rust Mapping

| Python | Rust Equivalent |
|--------|-----------------|
| `argparse` | `clap` crate |
| `random.choice()` | `rand::seq::SliceRandom::choose()` |
| `datetime.datetime.now().year` | `chrono::Local::now().year()` |
| `str.capitalize()` | Custom function (Rust strings are UTF-8) |
| File reading with `open()` | `std::fs::read_to_string()` |

---

## Appendix B: Example Cargo.toml

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
