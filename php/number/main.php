<?php

// Use strict typing for better type safety.
declare(strict_types=1);

/**
 * Reads and returns a valid integer guess from the user.
 */
function readGuess(): int
{
    while (true) {
        echo 'Your guess: ';

        $input = fgets(STDIN);

        // Exit cleanly if input stream is closed (e.g., Ctrl + D).
        if ($input === false) {
            exit(0);
        }

        // Trim whitespace and newline characters from the input.
        $text = trim($input);

        // Validate that the input is an integer.
        if (filter_var($text, FILTER_VALIDATE_INT) !== false) {
            return (int) $text;
        }

        echo "Invalid input!\n";
    }
}

/**
 * Entry point of the program.
 */
function main(): void
{
    $minNumber = 1;
    $maxNumber = 100;
    $attempts = 0;

    // Generate a random number between the minimum and maximum.
    $secretNumber = random_int($minNumber, $maxNumber);

    echo "Guess my number between {$minNumber} and {$maxNumber}!\n\n";

    // The main loop.
    while (true) {
        // Read the guess from the user.
        $guess = readGuess();

        // Increase the number of attempts.
        $attempts++;

        // Check if the guess is too low, too high or correct.
        if ($guess < $secretNumber) {
            echo "Too low!\n";
        } elseif ($guess > $secretNumber) {
            echo "Too high!\n";
        } else {
            echo "\nYou win, the number was {$secretNumber}!\n";
            echo "You had {$attempts} attempts.\n";
            break;
        }
    }
}

// Start the program.
main();
