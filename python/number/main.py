# Imports from standard modules.
import sys
from random import randint


def main():
    """
    The entry point of the program.

    It contains the main loop for the functionality.
    """

    min_number = 1  # The minimum number.
    max_number = 100  # The maximum number.
    attempts = 0  # The number of attempts.

    # Generates a random number between the minimum and maximum.
    secret_number = randint(min_number, max_number)

    print(f"Guess my number between {min_number} and {max_number}!\n")

    # The main loop.
    while True:
        # Increases the number of attempts.
        attempts += 1

        # Asks for the input and checks if it is a valid number.
        try:
            guess = int(input("Your guess: "))
        except ValueError:
            print("Invalid input!")
            continue

        # Checks if the guess is correct, to low or to high.
        if guess == secret_number:
            # The guess is correct.
            print(f"\nYou win, the number was {secret_number}!")
            print(f"You had {attempts} attempts.")
            break
        elif guess < secret_number:
            # The guess is too low.
            print("Too low!")
        elif guess > secret_number:
            # The guess is too high.
            print("Too high!")

    return 0


# Runs the code only when the file is executed as a script,
# not when it is imported as a module.
if __name__ == "__main__":
    sys.exit(main())
