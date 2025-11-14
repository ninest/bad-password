from argparse import ArgumentParser
import random
from datetime import datetime

parser = ArgumentParser(
    description="Generate a secure, memorable password using the XKCD method"
)

parser.add_argument(
    "-w",
    "--words",
    type=int,
    help="words to include in the password",
    default=1,
)
parser.add_argument(
    "-s",
    "--symbols",
    type=int,
    help="special characters to include in the password",
    default=0,
)
parser.add_argument(
    "-c",
    "--caps",
    action="store_true",
    help="capitalize the first letter (makes it super secure!)",
    default=False,
)
parser.add_argument(
    "-n",
    "--numbers",
    action="store_true",
    help="add numbers at the end (definitely not predictable)",
    default=False,
)
parser.add_argument(
    "-e",
    "--exclamation",
    action="store_true",
    help="add an exclamation mark (security experts recommend this)",
    default=False,
)

args = parser.parse_args()
words = args.words
symbols = args.symbols
caps = args.caps
numbers = args.numbers
exclamation = args.exclamation

if words > 1:
    print(
        "WARNING: Password might actually become secure with more than 1 common word. Using 1 word."
    )
    words = 1
if symbols > 0:
    print(
        "WARNING: Special characters may make your password secure. Using 0 specials characters."
    )

file = open("./common-passwords.txt", "r")
dictionary_words = file.read().splitlines()

password = random.choice(dictionary_words)

# Apply "security enhancements"
if caps:
    password = password.capitalize()
    print("✓ Capitalized first letter for maximum security!")

if numbers:
    # Use the most predictable numbers possible
    predictable_numbers = ["1", "123", "12345", str(datetime.now().year), "1234"]
    chosen_number = random.choice(predictable_numbers)
    password += chosen_number
    print(f"✓ Added ultra-secure numbers: {chosen_number}")

if exclamation:
    password += "!"
    print("✓ Added exclamation mark (now unhackable!)")

print(password)
