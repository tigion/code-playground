# Imports from standard modules.
import sys
from random import randint


def generate_code(length: int) -> str:
    """Generates a random numerical code with the specified length.

    Args:
        length: The length of the code.

    Returns:
        The generated code.
    """

    # Checks and sets a valid minimum length.
    if length < 1:
        length = 1

    # Fills the code with random digits from 0 to 9.
    code = ""
    for _ in range(length):
        code += str(randint(0, 9))

    return code


def compare_result(length: int, code: str, input_: str) -> str:
    """Compares the input with the code.

    - `*` if the digit is wrong and not in the code.
    - `?` if the digit is in the wrong position.
    - `0` to `9` if the correct digit is in the right position.

    Args:
        length: The length of the code.
        code: The code.
        input: The input.

    Returns:
        The result of the comparison.
    """

    # Fills the result list with `*`.
    result = ["*"] * length

    # Sets the minimum length if the input is shorter.
    # length = min(len(code), len(input_))
    if len(input_) < length:
        length = len(input_)

    # Fills the result with `?` or the correct digit.
    for i in range(length):
        # Checks if a digit is correct.
        if input_[i] == code[i]:
            result[i] = code[i]
        else:
            for j in range(length):
                # Checks if a digit is in the wrong position.
                if input_[i] == code[j]:
                    result[i] = "?"
                    break

    # Joins the result list into a string.
    return "".join(result)


def main():
    """
    The entry point of the program.

    It contains the main loop for the functionality.
    """

    code_length = 5  # The length of the code.
    attempts = 6  # The number of attempts.
    code = generate_code(code_length)  # The secret code.
    input_ = ""  # The input.

    print("Can you open the safe?")
    print(f"The code is {code_length} digits long.\n")

    # The main loop.
    while attempts > 0:
        # Asks for the input.
        input_ = input(f" {attempts} > ")
        # Limits the input to the code length.
        input_ = input_[:code_length]
        # Prints the result.
        print("     " + compare_result(code_length, code, input_))
        # Reduces the number of attempts by one.
        attempts -= 1

        # Checks if the code is correct or the number of attempts is zero.
        if input_ == code:
            # The code is correct.
            print("\nYou got it, the safe is open!")
            break
        elif attempts == 0:
            # The number of attempts is zero.
            print(f"\nYou failed, the code was '{code}'.")

    return 0


# Runs the code only when the file is executed as a script,
# not when it is imported as a module.
if __name__ == "__main__":
    sys.exit(main())
