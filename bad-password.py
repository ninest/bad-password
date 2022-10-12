from argparse import ArgumentParser
import random

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

args = parser.parse_args()
words = args.words
symbols = args.symbols

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

print(password)
