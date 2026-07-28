// Import required modules and traits.
use rand::RngExt;
use std::io::{self, Write};

/// The length of the secret code.
///
/// Constants are written in `SCREAMING_SNAKE_CASE` and need an explicit type.
const CODE_LENGTH: usize = 5;

/// The number of attempts the player has.
const MAX_ATTEMPTS: u32 = 6;

/// Generates a random numerical code with the specified length.
///
/// # Arguments
///
/// * `length` - The length of the code. Values below one are raised to one.
///
/// # Returns
///
/// The generated code as a `String`.
fn generate_code(length: usize) -> String {
    // Checks and sets a valid minimum length.
    // Shadowing lets us reuse the name `length` for the corrected value.
    let length = length.max(1);

    // The random number generator of the current thread.
    // It has to be mutable, because every random value changes its state.
    let mut rng = rand::rng();

    // Reserves the memory for all digits up front to avoid re-allocations.
    let mut code = String::with_capacity(length);

    // Fills the code with random digits from 0 to 9.
    // `_` is used because the loop counter itself is not needed.
    for _ in 0..length {
        let digit: u32 = rng.random_range(0..=9);

        // `char::from_digit` converts a number to a character for a given
        // radix (10 = decimal). It returns an `Option`, because the digit
        // could be invalid for the radix - which cannot happen here.
        let digit_char = char::from_digit(digit, 10).expect("0 to 9 is a valid decimal digit");

        code.push(digit_char);
    }

    code
}

/// Compares the input with the code.
///
/// * `*` if the digit is wrong and not in the code.
/// * `?` if the digit is in the code but in the wrong position.
/// * `0` to `9` if the correct digit is in the right position.
///
/// # Arguments
///
/// * `length` - The length of the code.
/// * `code` - The secret code.
/// * `input` - The input of the player.
///
/// # Returns
///
/// The result of the comparison as a `String`.
fn compare_result(length: usize, code: &str, input: &str) -> String {
    // A Rust `String` is UTF-8 encoded, so it cannot be indexed by position.
    // Collecting the characters into a `Vec<char>` allows index based access.
    let code_chars: Vec<char> = code.chars().collect();
    let input_chars: Vec<char> = input.chars().collect();

    // Fills the result with `*`, one for every digit of the code.
    let mut result = vec!['*'; length];

    // Compares only as many digits as the shortest of the three lengths has,
    // so that all following indexing operations stay inside the bounds.
    let compare_length = length.min(code_chars.len()).min(input_chars.len());

    // Fills the result with `?` or the correct digit.
    for i in 0..compare_length {
        if input_chars[i] == code_chars[i] {
            // The digit is correct and in the right position.
            result[i] = code_chars[i];
        } else if code_chars.contains(&input_chars[i]) {
            // The digit occurs in the compared part of the code,
            // but in a different position.
            result[i] = '?';
        }
        // Otherwise the digit stays `*`.
    }

    // Joins the characters of the result into a `String`.
    result.into_iter().collect()
}

/// Asks the player for an input and returns it without surrounding whitespace.
///
/// # Arguments
///
/// * `attempts` - The number of remaining attempts, shown as the prompt.
///
/// # Returns
///
/// The trimmed input of the player as a `String`.
fn read_input(attempts: u32) -> String {
    print!(" {attempts} > ");
    // `print!` does not add a line break, so the output buffer has to be
    // flushed manually to show the prompt before the input is read.
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();

    // `read_line` appends the line to `input`, therefore it needs a mutable
    // reference. On end of input (Ctrl+D) `input` simply stays empty.
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Removes the trailing line break and any other whitespace.
    input.trim().to_string()
}

/// The entry point of the program.
///
/// It contains the main loop for the functionality.
fn main() {
    let code = generate_code(CODE_LENGTH); // The secret code.
    let mut attempts = MAX_ATTEMPTS; // The remaining attempts.

    // Prints the intro message.
    println!("Can you open the safe?");
    println!("The code is {CODE_LENGTH} digits long.\n");

    // The main loop.
    while attempts > 0 {
        // Asks for the input and limits it to the length of the code.
        let input: String = read_input(attempts).chars().take(CODE_LENGTH).collect();

        // Compares the input with the code and returns a result string.
        let result = compare_result(CODE_LENGTH, &code, &input);

        // Prints the result of the comparison.
        println!("     {}", result);

        // Reduces the number of attempts by one.
        attempts -= 1;

        // Checks if the code is correct or the number of attempts is zero.
        if input == code {
            // The code is correct.
            println!("\nYou got it, the safe is open!");
            break;
        } else if attempts == 0 {
            // The number of attempts is zero.
            println!("\nYou failed, the code was '{code}'.");
        }
    }
}

// Unit tests are placed in a submodule that is only compiled for `cargo test`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_code_has_requested_length_and_only_digits() {
        let code = generate_code(CODE_LENGTH);

        assert_eq!(code.chars().count(), CODE_LENGTH);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn generated_code_has_at_least_one_digit() {
        assert_eq!(generate_code(0).chars().count(), 1);
    }

    #[test]
    fn comparison_marks_correct_wrong_and_misplaced_digits() {
        // 3 digits
        assert_eq!(compare_result(3, "912", "917"), "91*");
        assert_eq!(compare_result(3, "912", "192"), "??2");
        assert_eq!(compare_result(3, "912", "912"), "912");
        // 5 digits
        assert_eq!(compare_result(5, "28910", "38411"), "*8*1?");
        assert_eq!(compare_result(5, "28910", "98212"), "?8?1?");
        assert_eq!(compare_result(5, "28910", "28917"), "2891*");
    }

    #[test]
    fn comparison_pads_a_short_or_empty_input() {
        assert_eq!(compare_result(5, "28910", ""), "*****");
        assert_eq!(compare_result(5, "28910", "28"), "28***");
        assert_eq!(compare_result(5, "28910", "21"), "2?***");
    }
}
