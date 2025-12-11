# first_word

A tiny Rust command-line program that demonstrates how to find the first word in a string using string slices and borrowed references.

## What it does

The program constructs a `String`, calls a `first_word` function that returns a `&str` slice for the first word (text up to the first space), and prints the result.

## Files

- `src/main.rs`: contains the `main` function and `first_word` implementation.

## Build & Run

From the project root (where `Cargo.toml` lives) run:

```bash
cargo build
cargo run
```

Or to build and run in release mode:

```bash
cargo build --release
./target/release/first_word
```

## Example output

```
The first word is: Hello
```

## `first_word` function (summary)

The function signature in `src/main.rs` is:

```rust
fn first_word(s: &str) -> &str
```

- It converts the string to bytes with `as_bytes()` and iterates with `enumerate()`.
- When it encounters a space (`b' '`), it returns a slice from the start to that index: `&s[0..i]`.
- If no space is found, it returns the entire string slice: `&s[..]`.

This demonstrates Rust's borrowing and slicing: `first_word` returns a slice referencing the original string without taking ownership.

## Customizing

Change the string value assigned to `my_s` in `src/main.rs` to try different inputs.

## License

Unlicensed — use as you like.
