// Include the standard libraries.
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

/**
 * Reads and returns a valid integer guess from the user.
 *
 * NOTE: The number 0 is a special case here.
 */
int readGuess() {
  char input[256];

  while (1) {
    printf("Your guess: ");
    fgets(input, 256, stdin);

    // Convert the input to an integer.
    // If the input is not a valid number, it will return 0.
    int guess = atoi(input);

    // Check if the guess is valid and return it.
    if (guess != 0) return guess;

    printf("Invalid input!\n");
  }
}

/**
 * The main function.
 */
int main(int argc, char *argv[]) {
  int minNumber = 1;   // The minimum number.
  int maxNumber = 100; // The maximum number.
  int attempts = 0;    // The number of attempts.

  // Set the seed for the `rand` function.
  srand(time(NULL));
  // Generate a random number between the minimum and maximum.
  int secretNumber = rand() % (maxNumber - minNumber + 1) + minNumber;

  printf("Guess my number between %d and %d!\n\n", minNumber, maxNumber);

  // The main loop.
  while (1) {
    // Read the user's guess.
    int guess = readGuess();

    // Increase the number of attempts.
    attempts++;

    // Check if the guess is to low, to high or correct.
    if (guess < secretNumber) {
      printf("Too low!\n");
    } else if (guess > secretNumber) {
      printf("Too high!\n");
    } else {
      printf("\nYou win, the number was %d!\n", secretNumber);
      printf("You had %d attempts.\n", attempts);
      break;
    }
  }

  return 0;
}
