// Import required modules and traits.
use rand::RngExt;
use std::cmp::Ordering;
use std::io::{self, Write};

/// The minimum number.
const MIN_NUMBER: i32 = 1;

/// The maximum number.
const MAX_NUMBER: i32 = 100;

/// Reads and returns a valid integer guess from the user.
///
/// # Returns
///
/// * `Some(guess)` - The number entered by the user.
/// * `None` - The input has ended (EOF, e.g. Ctrl+D).
fn read_guess() -> Option<i32> {
    loop {
        let mut input = String::new();

        print!("Your guess: ");
        // Flush the output to ensure the prompt is displayed.
        io::stdout().flush().expect("Failed to flush stdout");

        // Read a line of input from the user.
        let bytes_read = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // If no bytes were read, it indicates EOF (Ctrl+D), so return None.
        if bytes_read == 0 {
            return None;
        }

        // Attempt to parse the input into an integer.
        if let Ok(number) = input.trim().parse::<i32>() {
            return Some(number);
        }

        println!("Invalid input!");
    }
}

/// The entry point of the program.
///
/// It contains the main loop for the functionality.
fn main() {
    let mut attempts: u32 = 0; // The number of attempts.

    // Generate a random number between the minimum and maximum.
    let mut rng = rand::rng();
    let secret_number = rng.random_range(MIN_NUMBER..=MAX_NUMBER);

    println!("Guess my number between {MIN_NUMBER} and {MAX_NUMBER}!\n");

    // The main loop.
    // It ends when the user guesses the correct number or
    // when the input is None (EOF, e.g., Ctrl+D).
    while let Some(guess) = read_guess() {
        // Increase the number of attempts.
        attempts += 1;

        // Check if the guess is too low, too high or correct.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("\nYou win, the number was {secret_number}!");
                println!("You had {attempts} attempts.");
                break;
            }
        }

        // The following commented-out code is an alternative way to check
        // the guess using if-else statements instead of pattern matching.
        // if guess < secret_number {
        //     println!("Too low!");
        // } else if guess > secret_number {
        //     println!("Too high!");
        // } else {
        //     println!("\nYou win, the number was {secret_number}!");
        //     println!("You had {attempts} attempts.");
        //     break;
        // }
    }
}
