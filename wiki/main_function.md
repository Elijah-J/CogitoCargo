<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - output/docs/rust/book/ch01-02-hello-world.md
  - output/docs/rust/book/ch03-03-how-functions-work.md
topic: rust-playground/main-function
---

# Rust `main` Function

`fn main() {}` is the starting function for the executable Rust programs in
`hello_world` and `hello_cargo`. The code inside its braces runs when the
compiled program starts.

## Shape I have used

```rust
fn main() {
    println!("Hello, world!");
}
```

`fn` starts a function definition. `main` is the special function name for an
executable program's entry point. The empty parentheses mean this version of
`main` takes no arguments. The braces contain the function body.

## What happens when it runs

The statement inside `main` runs when the program runs:

```rust
println!("Hello, world!");
```

In `hello_world`, `rustc main.rs` compiled the file and `./main` ran the
compiled binary. In `hello_cargo`, `cargo run` compiled and ran the package
that contains `src/main.rs`.

## Useful guardrail

`main.rs` is the source file. `fn main()` is the function inside the source
file. They usually appear together in beginner executable programs, but they
are not the same thing.

## Corpus references

- [The Rust Book: Hello, World!](../../output/docs/rust/book/ch01-02-hello-world.md)
- [The Rust Book: Functions](../../output/docs/rust/book/ch03-03-how-functions-work.md)

## Related wiki pages

- [Functions](functions.md)
- [Function calls](function_calls.md)
- [Block scope](block_scope.md)
- [`main.rs`](main_rs.md)
- [`src/main.rs`](src_main_rs.md)
- [`rustc <filename>`](rustc_filename.md)
- [`cargo run`](cargo_run.md)
- [Rust `println!` macro](println_macro.md)
- [Concepts so far](concepts.md)
