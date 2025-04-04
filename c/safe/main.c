// Includes the standard libraries.
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

/**
 * Generates a random numerical code with the specified length.
 *
 * @param len The length of the code.
 * @return The generated code.
 */
char *generateCode(unsigned int length) {
  // Checks and sets a valid minimum length.
  if (length < 1) length = 1;
  // Sets the seed for the `rand` function.
  srand(time(NULL));
  // Reserves memory for the code variable.
  char *code = malloc(length + 1);
  // Fills the code with random digits from 0 to 9.
  for (int i = 0; i < length; i++) code[i] = rand() % 10 + '0';

  return code;
}

/**
 * Compares the input with the code.
 *
 * - `*` if the digit is wrong and not in the code.
 * - `?` if the digit is in the wrong position.
 * - `0` to `9` if the correct digit is in the right position.
 *
 * @param len The length of the code.
 * @param secret The code.
 * @param input The input.
 * @return The result of the comparison.
 */
char *compareResult(int length, char *code, char *input) {
  // Reserves memory for the result variable.
  char *result = malloc(length + 1);

  // Fills the result with `*`.
  for (int i = 0; i < length; i++) result[i] = '*';

  // Sets the minimum length if the input is shorter.
  if (strlen(input) < length) length = strlen(input);

  // Fills the result with `?` or the correct digit.
  for (int i = 0; i < length; i++) {
    // Checks if a digit is correct.
    if (input[i] == code[i]) result[i] = code[i];
    else {
      // Checks if a digit is in the wrong position.
      for (int j = 0; j < length; j++) {
        if (input[i] == code[j]) {
          result[i] = '?';
          break;
        }
      }
    }
  }

  return result;
}

/**
 * The main function.
 */
int main(int argc, char *argv[]) {
  int codeLength = 5;                    // The length of the code.
  int attempts = 6;                      // The number of attempts.
  char *code = generateCode(codeLength); // The secret code.
  char input[256];                       // The input buffer.

  puts("Can you open the safe?");
  printf("The code is %d digits long.\n\n", codeLength);

  // The main loop.
  while (attempts > 0) {
    // Asks for the input.
    printf(" %d > ", attempts);
    fgets(input, 256, stdin);
    // Removes the newline character.
    input[strlen(input) - 1] = 0;
    // Limits the input to the code length.
    input[codeLength] = 0;
    // Prints the result.
    printf("     %s\n", compareResult(codeLength, code, input));
    // Reduces the number of attempts by one.
    attempts--;

    // Checks if the code is correct or the number of attempts is zero.
    if (strcmp(input, code) == 0) {
      // The code is correct.
      puts("\nYou got it, the safe is open!");
      break;
    } else if (attempts == 0) {
      // The number of attempts is zero.
      printf("\nYou failed, the code was '%s'.\n", code);
    };
  }

  return 0;
}
