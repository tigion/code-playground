// Import the needed module from Node.js.
import * as readline from "node:readline";

// Reads a guess from the user and returns it as an integer.
function readGuess(reader: readline.Interface): Promise<number> {
  // Create a Promise to handle the asynchronous user input
  return new Promise((resolve) => {
    // Function that asks the user for input
    const ask = (): void => {
      // Ask the user for a guess and wait for the answer
      reader.question("Your guess: ", (input: string) => {
        // Convert input into a number.
        const guess = Number(input.trim());

        // Accept only whole numbers.
        if (!Number.isInteger(guess)) {
          console.log("Invalid input!");
          // Ask again (recursive call)
          ask();
        } else {
          // Finish the Promise and return the valid number
          resolve(guess);
        }
      });
    };

    // Start the first question
    ask();
  });
}

// Main function that runs the number guessing game.
async function main(): Promise<void> {
  const minNumber = 1; // The minimum number.
  const maxNumber = 100; // The maximum number.
  let attempts = 0; // The number of attempts.

  // Generate a random secret number between minNumber and maxNumber (inclusive).
  const rangeSize = maxNumber - minNumber + 1;
  const secretNumber = Math.floor(Math.random() * rangeSize) + minNumber;

  // Create a readline interface to read user input from the console.
  const reader = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  console.log(`Guess my number between ${minNumber} and ${maxNumber}!\n`);

  try {
    // The main loop.
    while (true) {
      // Read a guess from the user.
      const guess = await readGuess(reader);

      // Increment the number of attempts.
      attempts += 1;

      // Check if the guess is too low, too high, or correct.
      if (guess < secretNumber) {
        console.log("Too low!");
      } else if (guess > secretNumber) {
        console.log("Too high!");
      } else {
        console.log(`\nYou win, the number was ${secretNumber}!`);
        console.log(`You had ${attempts} attempts.`);
        break;
      }
    }
  } finally {
    // Always close the readline interface to free up resources,
    // even if an error occurs.
    reader.close();
  }
}

// Start the game by calling the main function.
// main()

// Start the game with top-level error handling.
main().catch((error: unknown) => {
  console.error("\nUnexpected error:", error);
  process.exitCode = 1;
});
