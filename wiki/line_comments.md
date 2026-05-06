<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_comments/src/main.rs
  - output/docs/rust/book/ch03-04-comments.md
  - output/docs/rust/reference/comments.md
  - output/docs/rust/rust-by-example/hello/comment.md
topic: rust-playground/line-comments
---

# `//` Comments

`//` starts a line comment in Rust. The comment continues until the end of the
line, and the compiler ignores it when compiling the program.

## Shape I have used

```rust
// This line is for readers, not for the compiler.
let name = "Eli";

println!("Hello, {name}!"); // The program still prints one line.
```

A comment can sit on its own line above code. It can also sit after code on the
same line.

## Effect on the program

The comments do not print anything. The program output is still the output from
`println!`:

```console
Hello, Eli!
```

The Rust Reference describes non-doc comments as a form of whitespace. For the
current beginner model, that means they help the reader without changing what
the program does.

## Useful guardrail

For a comment that spans multiple lines, the Rust Book shows `//` at the start
of each commented line. Rust also has block comments and documentation
comments, but `hello_comments` only uses ordinary `//` comments.

## Corpus references

- [The Rust Book: Comments](../../output/docs/rust/book/ch03-04-comments.md)
- [Rust Reference: Comments](../../output/docs/rust/reference/comments.md)
- [Rust by Example: Comments](../../output/docs/rust/rust-by-example/hello/comment.md)

## Related wiki pages

- [`src/main.rs`](src_main_rs.md)
- [`let`](let_binding.md)
- [Block scope](block_scope.md)
- [Shadowing](shadowing.md)
- [`let mut`](mutable_binding.md)
- [Rust `println!` macro](println_macro.md)
- [`cargo check`](cargo_check.md)
- [`cargo run`](cargo_run.md)
- [Concepts so far](concepts.md)
