// Include the standard libraries.
#include <iostream>
#include <limits>
#include <random>

/**
 * Reads and returns a valid integer guess from the user.
 */
int readGuess() {
  while (true) {
    // Prompt the user for input.
    std::cout << "Your guess: ";

    // Read the input and check if it is a valid integer.
    int guess;
    if (std::cin >> guess) return guess;

    // Print an error message.
    std::cout << "Invalid input!\n";

    // Clear the error state.
    std::cin.clear();
    // Ignore the rest of the line.
    std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
  }
}

/**
 * Generates a random number between the minimum and maximum.
 *
 * Needs the following includes:
 * - `#include <cstdlib>`
 * - `#include <ctime>`
 */
int generateRandomNumberOld(int min, int max) {
  // Set the seed (start value) timebased for the `rand` function.
  std::srand(std::time(0));
  // Generate and return a random number between the minimum and maximum.
  return std::rand() % (max - min + 1) + min;
}

/**
 * Generates a random number between the minimum and maximum.
 */
int generateRandomNumberNew(int min, int max) {
  // Create a source for a random starting value.
  std::random_device rd;
  // Create a random number generator.
  std::mt19937 generator(rd());
  // Define the range for the random number.
  std::uniform_int_distribution<int> distribution(min, max);
  // Pick and return a random number.
  return distribution(generator);
}

/**
 * The entry point of the program.
 */
int main() {
  const int minNumber = 1;   // The minimum number.
  const int maxNumber = 100; // The maximum number.
  int attempts = 0;          // The number of attempts.

  // Generate a random number.
  int secretNumber = generateRandomNumberNew(minNumber, maxNumber);

  // Print the intro message.
  std::cout << "Guess my number between " << minNumber << " and " << maxNumber
            << "!\n\n";

  // The main loop.
  while (true) {
    // Read the user's guess.
    int guess = readGuess();

    // Increase the number of attempts.
    attempts++;

    // Check if the guess is too low, too high or correct.
    if (guess < secretNumber) {
      std::cout << "Too low!\n";
    } else if (guess > secretNumber) {
      std::cout << "Too high!\n";
    } else {
      std::cout << "\nYou win, the number was " << secretNumber << "!\n";
      std::cout << "You had " << attempts << " attempts.\n";
      break;
    }
  }

  return 0;
}
