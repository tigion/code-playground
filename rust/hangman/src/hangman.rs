//! The state and the rules of a single hangman round.

/// The number of life points a player starts with.
pub const MAX_LIFE_POINTS: u32 = 10;

/// The character that is shown instead of a letter that is still hidden.
const HIDDEN_MARKER: char = '_';

/// The outcome of a single guess.
///
/// An `enum` lists all possible outcomes. Because `match` has to cover every
/// variant, the compiler makes it impossible to forget one of the cases.
#[derive(Debug, PartialEq, Eq)]
pub enum Guess {
    /// The letter occurs in the word, at least one position was revealed.
    Correct,
    /// The letter does not occur in the word, one life point is lost.
    Wrong,
    /// The letter was already guessed before, so nothing happens.
    AlreadyUsed,
}

/// Holds everything that belongs to one round of the game.
pub struct Hangman {
    /// The secret word, split into single characters.
    word: Vec<char>,
    /// One flag per character of the word: `true` if it is already revealed.
    revealed: Vec<bool>,
    /// Every letter that was guessed so far, sorted alphabetically.
    used_letters: Vec<char>,
    /// The remaining life points.
    life_points: u32,
}

impl Hangman {
    /// Creates a new round for the given word.
    ///
    /// # Arguments
    ///
    /// * `word` - The word to guess. It has to contain at least one letter.
    ///
    /// # Returns
    ///
    /// * `Ok(hangman)` - A round that is ready to be played.
    /// * `Err(message)` - The word contains no letter at all.
    pub fn new(word: &str) -> Result<Self, &'static str> {
        // A `String` cannot be indexed by position, because it is UTF-8
        // encoded. A `Vec<char>` can, which is needed to reveal single letters.
        let word: Vec<char> = word.chars().collect();

        // `any` returns true as soon as one character satisfies the condition.
        if !word.iter().any(|character| character.is_alphabetic()) {
            return Err("The word has to contain at least one letter!");
        }

        // Characters that are no ASCII letters (like spaces, punctuation or digits)
        // can never be guessed. They are revealed right from the start,
        // otherwise such a word could never be completed.
        let revealed = word
            .iter()
            .map(|character| !character.is_ascii_alphabetic())
            .collect();

        Ok(Self {
            word,
            revealed,
            used_letters: Vec::new(),
            life_points: MAX_LIFE_POINTS,
        })
    }

    /// Returns the number of characters of the word.
    pub fn word_length(&self) -> usize {
        self.word.len()
    }

    /// Returns the remaining life points.
    ///
    /// Rust naming conventions omit a `get_` prefix for such accessors.
    pub fn life_points(&self) -> u32 {
        self.life_points
    }

    /// Returns the secret word.
    pub fn word(&self) -> String {
        self.word.iter().collect()
    }

    /// Guesses a letter and updates the state of the round.
    ///
    /// # Arguments
    ///
    /// * `letter` - The guessed letter in upper case.
    ///
    /// # Returns
    ///
    /// What the guess did: [`Guess::Correct`], [`Guess::Wrong`] or
    /// [`Guess::AlreadyUsed`].
    pub fn guess(&mut self, letter: char) -> Guess {
        // A letter may only be used once, so a repeated guess is free.
        if self.used_letters.contains(&letter) {
            return Guess::AlreadyUsed;
        }

        // Remember the letter and keeps the list sorted for a tidy output.
        self.used_letters.push(letter);
        self.used_letters.sort_unstable();

        // Reveal every position where the letter occurs. A letter can appear
        // more than once, so the whole word is checked.
        let mut found = false;
        for (index, &character) in self.word.iter().enumerate() {
            if character == letter {
                self.revealed[index] = true;
                found = true;
            }
        }

        if found {
            Guess::Correct
        } else {
            // `saturating_sub` stops at zero. A plain `- 1` on an unsigned
            // integer would panic if the value was already zero.
            self.life_points = self.life_points.saturating_sub(1);
            Guess::Wrong
        }
    }

    /// Returns `true` if every letter of the word is revealed.
    pub fn is_word_complete(&self) -> bool {
        // `all` returns true only if the condition holds for every element.
        self.revealed.iter().all(|&revealed| revealed)
    }

    /// Returns `true` if there are no life points left.
    pub fn is_player_dead(&self) -> bool {
        self.life_points == 0
    }

    /// Returns the word with hidden letters, for example `_ _ A _ _`.
    pub fn formatted_word(&self) -> String {
        let mut output = String::new();

        // `zip` walk both vectors in parallel and yield pairs.
        for (index, (&character, &revealed)) in self.word.iter().zip(&self.revealed).enumerate() {
            // Separate the characters with a space, but not before the first.
            if index > 0 {
                output.push(' ');
            }

            output.push(if revealed { character } else { HIDDEN_MARKER });
        }

        output
    }

    /// Returns the used letters separated by commas, for example `A,L`.
    pub fn formatted_used_letters(&self) -> String {
        // `join` needs strings, so every character becomes a short `String`.
        self.used_letters
            .iter()
            .map(|letter| letter.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

// Unit tests are only compiled when `cargo test` is run.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_round_hides_every_letter() {
        let hangman = Hangman::new("HASE").unwrap();

        assert_eq!(hangman.formatted_word(), "_ _ _ _");
        assert_eq!(hangman.formatted_used_letters(), "");
        assert_eq!(hangman.life_points(), MAX_LIFE_POINTS);
        assert!(!hangman.is_word_complete());
        assert!(!hangman.is_player_dead());
    }

    #[test]
    fn a_correct_guess_reveals_every_occurrence() {
        let mut hangman = Hangman::new("HANGAR").unwrap();

        assert_eq!(hangman.guess('A'), Guess::Correct);
        assert_eq!(hangman.formatted_word(), "_ A _ _ A _");
        // A correct guess must not cost a life point.
        assert_eq!(hangman.life_points(), MAX_LIFE_POINTS);
    }

    #[test]
    fn a_wrong_guess_costs_one_life_point() {
        let mut hangman = Hangman::new("HASE").unwrap();

        assert_eq!(hangman.guess('X'), Guess::Wrong);
        assert_eq!(hangman.life_points(), MAX_LIFE_POINTS - 1);
        assert_eq!(hangman.formatted_word(), "_ _ _ _");
    }

    #[test]
    fn a_repeated_guess_changes_nothing() {
        let mut hangman = Hangman::new("HASE").unwrap();
        hangman.guess('X');

        assert_eq!(hangman.guess('X'), Guess::AlreadyUsed);
        // Still only one life point lost and one letter used.
        assert_eq!(hangman.life_points(), MAX_LIFE_POINTS - 1);
        assert_eq!(hangman.formatted_used_letters(), "X");
    }

    #[test]
    fn used_letters_are_sorted() {
        let mut hangman = Hangman::new("HASE").unwrap();

        hangman.guess('S');
        hangman.guess('A');
        hangman.guess('X');

        assert_eq!(hangman.formatted_used_letters(), "A,S,X");
    }

    #[test]
    fn a_word_is_complete_when_all_letters_are_found() {
        let mut hangman = Hangman::new("AB").unwrap();

        hangman.guess('A');
        assert!(!hangman.is_word_complete());
        hangman.guess('B');
        assert!(hangman.is_word_complete());
    }

    #[test]
    fn the_player_dies_after_ten_wrong_guesses() {
        let mut hangman = Hangman::new("A").unwrap();

        for letter in ['B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K'] {
            assert_eq!(hangman.guess(letter), Guess::Wrong);
        }

        assert!(hangman.is_player_dead());
        assert_eq!(hangman.life_points(), 0);
    }

    #[test]
    fn life_points_never_drop_below_zero() {
        let mut hangman = Hangman::new("A").unwrap();

        // Twelve wrong guesses, but only ten life points.
        for letter in ['B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M'] {
            hangman.guess(letter);
        }

        assert_eq!(hangman.life_points(), 0);
    }

    #[test]
    fn a_word_without_letters_is_rejected() {
        assert!(Hangman::new("").is_err());
        assert!(Hangman::new("   ").is_err());
        assert!(Hangman::new("42").is_err());
    }

    #[test]
    fn a_word_with_non_ascii_letters_is_working() {
        // Chars like ` Ü()` cannot be typed by the player,
        // so they are revealed from the start and do not count as letters.
        let mut hangman = Hangman::new("AB CD (SÜSS)").unwrap();

        assert_eq!(hangman.formatted_word(), "_ _   _ _   ( _ Ü _ _ )");

        for letter in ['A', 'B', 'C', 'D', 'S'] {
            assert_eq!(hangman.guess(letter), Guess::Correct);
        }

        assert!(hangman.is_word_complete());
        assert_eq!(hangman.formatted_word(), "A B   C D   ( S Ü S S )");
        assert_eq!(hangman.life_points(), MAX_LIFE_POINTS);
    }
}
