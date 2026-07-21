# Number

A simple game in Rust. The goal is to guess a number.

## How to build and run

1. Download the _number_ folder to your computer.
2. Open a terminal and navigate to the _number_ folder.
3. Build it with `cargo build` or `cargo build --release`.
4. Run it with `cargo run` (`cargo run --release`) or `./target/debug/number`
   (`./target/release/number`).

## How to play

```txt
Guess my number between 1 and 100!

Your guess: 50
Too low!
Your guess: 75
Too high!
Your guess: 62
Too high!
Your guess: 56
Too high!
Your guess: 53
Too low!
Your guess: 54
Too low!
Your guess: 55

You win, the number was 55!
You had 7 attempts.
```
