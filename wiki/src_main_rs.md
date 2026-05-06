<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - output/docs/rust/book/ch01-03-hello-cargo.md
  - output/docs/rust/cargo/guide/project-layout.md
  - output/docs/rust/cargo/reference/cargo-targets.md
  - output/docs/rust/book/ch01-02-hello-world.md
topic: rust-playground/src-main-rs
---

# `src/main.rs`

`src/main.rs` is the starter Rust source file in a default executable Cargo
package. In `hello_cargo`, it contained the same hello-world program as the
direct `main.rs` file in `hello_world`.

## Where it lives

`cargo new hello_cargo` created this shape:

```text
hello_cargo/
|-- Cargo.toml
`-- src/
    `-- main.rs
```

Cargo expects source files to live inside `src/`. The package manifest,
`Cargo.toml`, stays at the package root.

## What it contains right now

The starter file contains:

```rust
fn main() {
    println!("Hello, world!");
}
```

`cargo run` builds and runs this program. `cargo check` checks whether it
compiles without running it.

## Useful guardrail

Plain `main.rs` and Cargo's `src/main.rs` are the same kind of Rust source
file, but they sit in different workflows. `rustc main.rs` names the file
directly. Cargo uses the package layout and `Cargo.toml` to find `src/main.rs`.

## Corpus references

- [The Rust Book: Hello, Cargo!](../../output/docs/rust/book/ch01-03-hello-cargo.md)
- [Cargo Book: Package Layout](../../output/docs/rust/cargo/guide/project-layout.md)
- [Cargo Book: Cargo Targets](../../output/docs/rust/cargo/reference/cargo-targets.md)
- [The Rust Book: Hello, World!](../../output/docs/rust/book/ch01-02-hello-world.md)

## Related wiki pages

- [`main.rs`](main_rs.md)
- [`Cargo.toml`](cargo_toml.md)
- [`cargo new <project_name>`](cargo_new.md)
- [`cargo run`](cargo_run.md)
- [`cargo check`](cargo_check.md)
- [`//` comments](line_comments.md)
- [Concepts so far](concepts.md)
