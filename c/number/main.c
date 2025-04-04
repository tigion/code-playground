// Includes the standard libraries.
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

/**
 * The main function.
 */
int main(int argc, char *argv[]) {
  int minNumber = 1;   // The minimum number.
  int maxNumber = 100; // The maximum number.
  int attempts = 0;    // The number of attempts.
  char input[256];     // The input buffer.

  // Sets the seed for the `rand` function.
  srand(time(NULL));
  // Generates a random number between the minimum and maximum.
  int secretNumber = rand() % (maxNumber - minNumber + 1) + minNumber;

  printf("Guess my number between %d and %d!\n\n", minNumber, maxNumber);

  // The main loop.
  while (1) {
    // Increases the number of attempts.
    attempts++;

    // Asks for the input and checks if it is a valid number.
    printf("Your guess: ");
    fgets(input, 256, stdin);
    int guess = atoi(input);
    if (guess == 0) {
      printf("Invalid input!\n");
      continue;
    }

    // Checks if the guess is correct, to low or to high.
    if (guess == secretNumber) {
      // The guess is correct.
      printf("\nYou win, the number was %d!\n", secretNumber);
      printf("You had %d attempts.\n", attempts);
      break;
    } else if (guess < secretNumber) {
      // The guess is too low.
      printf("Too low!\n");
    } else {
      // The guess is too high.
      printf("Too high!\n");
    }
  }

  return 0;
}
