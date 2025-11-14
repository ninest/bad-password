# bad-password

Generates intentionally weak passwords from common password list.

## Usage

```bash
python bad-password.py [options]
```

## Options

- `-w, --words` - Number of words (default 1, max 1 enforced)
- `-s, --symbols` - Special characters (ignored, set to 0)
- `-c, --caps` - Capitalize first letter
- `-n, --numbers` - Append predictable numbers (1, 123, current year, etc)
- `-e, --exclamation` - Add exclamation mark at end

## Examples

```bash
# Basic weak password
python bad-password.py

# With capitalization + numbers
python bad-password.py -c -n

# All "enhancements"
python bad-password.py -c -n -e
```

## Requirements

- Python 3
- `common-passwords.txt` in same directory
