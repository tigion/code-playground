// Import required modules and traits.
use rand::RngExt;
use std::io::{self, Write};

// Declares the other files of this crate as modules.
mod hangman;
mod wordlist;

use crate::hangman::{Guess, Hangman, MAX_LIFE_POINTS};

/// Picks a random word from the given word list and converts it to upper case.
///
/// # Arguments
///
/// * `words` - The list to pick from. It must not be empty.
///
/// # Returns
///
/// The chosen word in upper case.
fn random_word(words: &[&str]) -> String {
    // Without this check `random_range(0..0)` would panic with a less
    // helpful message.
    assert!(!words.is_empty(), "The word list must not be empty!");

    let mut rng = rand::rng();
    let index = rng.random_range(0..words.len());

    // `to_uppercase` also turns 'ß' into "SS", so the word only contains
    // letters that the player is able to type.
    words[index].to_uppercase()
}

/// Turns one line of input into a single letter the game can work with.
///
/// # Arguments
///
/// * `input` - One raw input line. Surrounding whitespace is ignored.
///
/// # Returns
///
/// * `Ok(letter)` - A single letter from `A` to `Z` in upper case.
/// * `Err(message)` - The reason for the rejection, meant for the player.
fn parse_letter(input: &str) -> Result<char, &'static str> {
    // Removes surrounding whitespace and turns the string into an iterator of
    // characters. The iterator is used to check the length and to get the
    // first character in one go.
    let mut chars = input.trim().chars();

    // The input must contain exactly one character.
    let (Some(letter), None) = (chars.next(), chars.next()) else {
        return Err("Please enter exactly one letter!");
    };

    match letter {
        // `to_uppercase` turns 'ß' into "SS", so a single 'S' reveals both
        // positions. Accepting 'ß' therefore saves the player a translation.
        'ß' => Ok('S'),
        // A-Z letters are accepted in any case and returned in upper case.
        letter if letter.is_ascii_alphabetic() => Ok(letter.to_ascii_uppercase()),
        // All other characters are rejected.
        _ => Err("Please use only the letters A-Z!"),
    }
}

/// Asks the player for a letter until the input is valid.
///
/// # Returns
///
/// * `Some(letter)` - A single letter from `A` to `Z` in upper case.
/// * `None` - The input has ended (EOF, for example through Ctrl+D).
fn read_letter() -> Option<char> {
    loop {
        print!("Which letter is in the word? ");
        // `print!` adds no line break, so the buffer has to be flushed
        // manually to show the prompt before the input is read.
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        // `read_line` returns the number of bytes it has read.
        let bytes_read = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Zero bytes mean the input has ended. Asking again would loop forever.
        if bytes_read == 0 {
            return None;
        }

        // The validation itself lives in `parse_letter`, this loop only
        // repeats the question as long as the input is rejected.
        match parse_letter(&input) {
            Ok(letter) => return Some(letter),
            Err(message) => println!("> {message}"),
        }
    }
}

/// The entry point of the program.
///
/// It contains the main loop for the functionality.
fn main() {
    // Starts a new round with a random word from the word list.
    let word = random_word(wordlist::WORDS);
    let mut hangman = Hangman::new(&word).expect("The word list must only contain valid words");

    println!("Welcome to Hangman!");
    println!("\nFind a word with {} letters.", hangman.word_length());

    // The main loop.
    loop {
        // Prints the current state of the round.
        println!("\n{}\n", hangman.formatted_word());
        println!("{}/{MAX_LIFE_POINTS} lives", hangman.life_points());
        println!("Used: {}", hangman.formatted_used_letters());

        // Checks both end conditions before asking for the next letter.
        if hangman.is_word_complete() {
            println!("\nYou won!\nYou guessed '{}' correctly.", hangman.word());
            break;
        } else if hangman.is_player_dead() {
            println!("\nYou lost!\nThe word was '{}'.", hangman.word());
            break;
        }

        // Asks for a letter and ends the game when the input is over.
        let Some(letter) = read_letter() else {
            break;
        };

        // Reacts to the guess. `match` forces us to handle every outcome.
        match hangman.guess(letter) {
            Guess::Correct => println!("> Correct :)"),
            Guess::Wrong => println!("> Wrong, you lose a life :("),
            Guess::AlreadyUsed => println!("> Careful, already used!"),
        }
    }
}

// Unit tests are only compiled when `cargo test` is run.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_letter_is_returned_in_upper_case() {
        assert_eq!(parse_letter("a"), Ok('A'));
        assert_eq!(parse_letter("Z"), Ok('Z'));
    }

    #[test]
    fn surrounding_whitespace_is_ignored() {
        assert_eq!(parse_letter("a\n"), Ok('A'));
        assert_eq!(parse_letter("  a  \r\n"), Ok('A'));
    }

    #[test]
    fn eszett_becomes_a_single_s() {
        assert_eq!(parse_letter("ß"), Ok('S'));
        assert_eq!(parse_letter("ß\n"), Ok('S'));
    }

    #[test]
    fn an_empty_input_is_rejected() {
        assert_eq!(parse_letter(""), Err("Please enter exactly one letter!"));
        assert_eq!(parse_letter(" \n"), Err("Please enter exactly one letter!"));
    }

    #[test]
    fn more_than_one_character_is_rejected() {
        assert_eq!(parse_letter("ab"), Err("Please enter exactly one letter!"));
        assert_eq!(parse_letter("a b"), Err("Please enter exactly one letter!"));
    }

    #[test]
    fn a_character_that_is_no_letter_is_rejected() {
        assert_eq!(parse_letter("4"), Err("Please use only the letters A-Z!"));
        assert_eq!(parse_letter("?"), Err("Please use only the letters A-Z!"));
    }

    #[test]
    fn an_umlaut_is_rejected_as_an_unusable_letter() {
        assert_eq!(parse_letter("ä"), Err("Please use only the letters A-Z!"));
        assert_eq!(parse_letter("Ü"), Err("Please use only the letters A-Z!"));
    }

    #[test]
    fn a_word_is_picked_in_upper_case() {
        assert_eq!(random_word(&["Dojo"]), "DOJO");
    }

    #[test]
    fn a_word_with_eszett_is_picked_in_upper_case() {
        assert_eq!(random_word(&["Süßigkeit"]), "SÜSSIGKEIT");
    }

    #[test]
    #[should_panic(expected = "The word list must not be empty!")]
    fn an_empty_word_list_is_rejected() {
        random_word(&[]);
    }
}
