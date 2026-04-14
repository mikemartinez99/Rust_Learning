# Rust_Learning
My Rust learning journey

To accompany [Rustlings](http://rustlings.rust-lang.org) questions, I have AI draft me problem sets to accompany the reading in [The Rust Programming Language Book](https://doc.rust-lang.org/stable/book/index.html). These are the problem sets and their solutions.

## Instructions

Each `.rs` file in `src/bin` is its own independent program with its own `fn main()` call. You run a specific one with 

```bash
cargo check --bin problem_set_1 # To check compilation
cargo run --bin problem_set_1 # To build and run binary
```

## Outstanding Questions as I go:

- Difference between string types (`&str`, `String`, &Static str`)
- Returning different data types from a function with `if/else` logic (i.e., return a character for base complements, but return a string for "non-canonical"
