// Using directives
using System;

// Reads and returns a valid integer guess from the user.
static int ReadGuess()
{
    while (true)
    {
        Console.Write("Your guess: ");
        string? input = Console.ReadLine();

        // Exit cleanly if input stream is closed (e.g., Ctrl + D / Ctrl + Z).
        if (input is null)
        {
            Environment.Exit(0);
        }

        // Validate that the input is an integer.
        if (int.TryParse(input, out int guess))
        {
            return guess;
        }

        Console.WriteLine("Invalid input!");
    }
}

const int minNumber = 1;
const int maxNumber = 100;
int attempts = 0;

// Generate a random number between minNumber and maxNumber (inclusive).
int secretNumber = Random.Shared.Next(minNumber, maxNumber + 1);

Console.WriteLine($"Guess my number between {minNumber} and {maxNumber}!\n");

// The main loop.
while (true)
{
    // Read the guess from the user.
    int guess = ReadGuess();

    // Increase the number of attempts.
    attempts++;

    // Check if the guess is too low, too high or correct.
    if (guess < secretNumber)
    {
        Console.WriteLine("Too low!");
    }
    else if (guess > secretNumber)
    {
        Console.WriteLine("Too high!");
    }
    else
    {
        Console.WriteLine($"\nYou win, the number was {secretNumber}!");
        Console.WriteLine($"You had {attempts} attempts.");
        break;
    }
}
