# Imports from standard modules.
import sys
from random import randint


def read_guess():
    """
    Reads and returns a valid integer guess from the user.
    """

    while True:
        try:
            guess = int(input("Your guess: "))
            return guess
        except ValueError:
            print("Invalid input!")


def main():
    """
    The entry point of the program.
    """

    min_number = 1  # The minimum number.
    max_number = 100  # The maximum number.
    attempts = 0  # The number of attempts.

    # Generates a random number between the minimum and maximum.
    secret_number = randint(min_number, max_number)

    print(f"Guess my number between {min_number} and {max_number}!\n")

    # The main loop.
    while True:
        # Reads the guess from the user.
        guess = read_guess()

        # Increases the number of attempts.
        attempts += 1

        # Checks if the guess is to low, to high or correct.
        if guess < secret_number:
            print("Too low!")
        elif guess > secret_number:
            print("Too high!")
        else:
            print(f"\nYou win, the number was {secret_number}!")
            print(f"You had {attempts} attempts.")
            break

    return 0


# Runs the code only when the file is executed as a script,
# not when it is imported as a module.
if __name__ == "__main__":
    sys.exit(main())
