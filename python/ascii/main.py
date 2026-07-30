"""
ASCII table generator.

Prints a formatted ASCII table showing decimal, hexadecimal, and character
representations. Non-printable characters are displayed as dots for readability.
"""

import math
import sys


def printable(code: int) -> str:
    """
    Return a printable character for the given ASCII code.

    Non-printable characters are replaced with a dot.
    """

    # Convert the ASCII code to a character (string).
    char = chr(code)

    return char if char.isprintable() else "."


def main() -> int:
    """Print the ASCII table and return the exit code."""

    ASCII_FIRST = 0  # The first ASCII code to display.
    ASCII_LAST = 127  # The last ASCII code to display.

    # The number of columns to use for the ASCII table.
    # Use 1, 2, 4, or 8 columns for an optimal layout.
    COL_COUNT = 4

    # The number of ASCII codes to show in the table.
    code_count = ASCII_LAST - ASCII_FIRST + 1

    # The number of rows needed to fit all codes into the columns.
    row_count = math.ceil(code_count / COL_COUNT)
    # Or without using the math module:
    # row_count = (code_count + COL_COUNT - 1) // COL_COUNT

    # Print the header of the table based on the number of columns.
    print(" | ".join(["Dec Hex Chr"] * COL_COUNT))
    print("-+-".join(["-----------"] * COL_COUNT))

    for row in range(row_count):
        for col in range(COL_COUNT):
            # Add separators between columns.
            if col > 0:
                print(" | ", end="")  # `end=""` prevents a newline at the end.

            # Calculate the index of the ASCII code.
            index = row + col * row_count

            # Stop when the ASCII code exceeds the end of the range.
            if index >= code_count:
                break

            # Calculate the ASCII code from the index.
            code = ASCII_FIRST + index

            print(f"{code:>3} {code:3X}  {printable(code)} ", end="")
        # Newline after each row.
        print()

    return 0


if __name__ == "__main__":
    sys.exit(main())
