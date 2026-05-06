<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - https://doc.rust-lang.org/stable/cargo/commands/cargo-new.html
  - https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html
  - https://doc.rust-lang.org/stable/cargo/guide/project-layout.html
topic: rust-playground/cargo-new
---

# `cargo new <project_name>`

`cargo new <project_name>` creates a new Cargo package in a new directory. In
`hello_cargo`, it created the `hello_cargo` folder and the starter files Cargo
needs.

## Shape I have used

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

The command created a package directory named `hello_cargo`.

## Files it created

For the starter executable package, the important generated shape was:

```text
hello_cargo/
|-- Cargo.toml
`-- src/
    `-- main.rs
```

`Cargo.toml` is the manifest Cargo reads. `src/main.rs` is the starter Rust
source file containing the hello-world program.

## Useful guardrail

`cargo new` creates a package, not just a loose `.rs` file. That is why later
commands such as `cargo run` and `cargo check` work from inside the package
directory.

## Corpus references

- [Cargo Book: cargo-new](https://doc.rust-lang.org/stable/cargo/commands/cargo-new.html)
- [The Rust Book: Hello, Cargo!](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html)
- [Cargo Book: Package Layout](https://doc.rust-lang.org/stable/cargo/guide/project-layout.html)

## Related wiki pages

- [`Cargo.toml`](cargo_toml.md)
- [`src/main.rs`](src_main_rs.md)
- [`cargo run`](cargo_run.md)
- [`cargo check`](cargo_check.md)
- [`Cargo.lock`](cargo_lock.md)
- [Concepts so far](concepts.md)
