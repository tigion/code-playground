# Safe

A simple game in Rust. The goal is to guess a code and open the safe.

## How to build and run

1. Download the _safe_ folder to your computer.
2. Open a terminal and navigate to the _safe_ folder.
3. Build it with `cargo build` or `cargo build --release`.
4. Run it with `cargo run` (`cargo run --release`) or `./target/debug/safe`
   (`./target/release/safe`).
5. Follow the instructions.

Run the unit tests with `cargo test`.

## How to play

```txt
Can you open the safe?
The code is 5 digits long.

 6 > 38411
     *8*1?
 5 > 58619
     *8*1?
 4 > 98212
     ?8?1?
 3 > 28912
     2891?
 2 > 28918
     2891?
 1 > 28917
     2891*

You failed, the code was '28910'.
```
