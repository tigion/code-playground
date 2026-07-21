// Import required modules and traits.
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

/// Reads and returns a valid integer guess from the user.
fn read_guess() -> i32 {
    loop {
        let mut input = String::new();

        print!("Your guess: ");
        // Flush the output to ensure the prompt is displayed.
        io::stdout().flush().expect("Failed to flush stdout");

        // Read a line of input from the user.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Attempt to parse the input into an integer.
        match input.trim().parse() {
            Ok(number) => return number,
            Err(_) => println!("Invalid input!"),
        }
    }
}

/// The entry point of the program.
fn main() {
    let min_number = 1; // The minimum number.
    let max_number = 100; // The maximum number.
    let mut attempts: u32 = 0; // The number of attempts.

    // Generates a random number between the minimum and maximum.
    let secret_number = rand::rng().random_range(min_number..=max_number);

    println!("Guess my number between {min_number} and {max_number}!\n",);

    // The main loop.
    loop {
        // Reads the user's guess.
        let guess = read_guess();

        // Increases the number of attempts.
        attempts += 1;

        // Checks if the guess is to low, to high or correct.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("\nYou win, the number was {secret_number}!");
                println!("You had {attempts} attempts.");
                break;
            }
        }
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
