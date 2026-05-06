<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_comments/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-04-comments.html
  - https://doc.rust-lang.org/stable/reference/comments.html
  - https://doc.rust-lang.org/stable/rust-by-example/hello/comment.html
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

- [The Rust Book: Comments](https://doc.rust-lang.org/stable/book/ch03-04-comments.html)
- [Rust Reference: Comments](https://doc.rust-lang.org/stable/reference/comments.html)
- [Rust by Example: Comments](https://doc.rust-lang.org/stable/rust-by-example/hello/comment.html)

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
