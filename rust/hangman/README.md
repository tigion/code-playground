# Hangman (Galgenraten)

A simple game in Rust. The goal is to guess a word.

## How to build and run

1. Download the _hangman_ folder to your computer.
2. Open a terminal and navigate to the _hangman_ folder.
3. Build it with `cargo build` or `cargo build --release`.
4. Run it with `cargo run` (`cargo run --release`) or `./target/debug/hangman`
   (`./target/release/hangman`).
5. Follow the instructions.

Run the unit tests with `cargo test`.

## How it is structured

| File              | Content                                                |
| ----------------- | ------------------------------------------------------ |
| _src/main.rs_     | The user interaction: input, output and the main loop. |
| _src/hangman.rs_  | The `Hangman` struct with the state and the rules.     |
| _src/wordlist.rs_ | The list of words to guess.                            |

## How to play

You have 10 life points. Every letter that does not occur in the word costs one
of them. Guessing a letter twice is free, but it does not help either.

```txt
Welcome to Hangman!

Find a word with 5 letters.

_ _ _ _ _

10/10 lives
Used:
Which letter is in the word? a
> Richtig :)

_ _ A _ _

10/10 lives
Used: A
Which letter is in the word? l
> Wrong, you lose a life :(

_ _ A _ _

9/10 lives
Used: A,L
Which letter is in the word?

...

P H A S E

9/10 lives
Used: A,E,H,L,P,S

You won!
You guessed 'PHASE' correctly.
```
