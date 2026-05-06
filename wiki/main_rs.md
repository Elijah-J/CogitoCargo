<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - output/docs/rust/book/ch01-02-hello-world.md
  - output/docs/rust/book/ch01-03-hello-cargo.md
  - output/docs/rust/cargo/guide/project-layout.md
topic: rust-playground/main-rs
---

# `main.rs`

`main.rs` is a Rust source filename used for executable programs. In
`hello_world`, it was the file passed to `rustc`; in `hello_cargo`, the
corresponding file was `src/main.rs`.

## Direct `rustc` role

The first project had a file named `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Compiling it produced a separate executable:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

`main.rs` stayed source code. `main` was the compiled program.

## Cargo role

`cargo new hello_cargo` created `src/main.rs` instead of placing `main.rs` at
the package root:

```text
hello_cargo/
|-- Cargo.toml
`-- src/
    `-- main.rs
```

Cargo expects source files under `src`. For the starter executable package,
`src/main.rs` contains the same basic hello-world program.

## Useful guardrail

`main.rs` is a file path. `fn main()` is the function that starts the program.
The names line up in beginner examples, but one names a file and the other
names code inside the file.

## Corpus references

- [The Rust Book: Hello, World!](../../output/docs/rust/book/ch01-02-hello-world.md)
- [The Rust Book: Hello, Cargo!](../../output/docs/rust/book/ch01-03-hello-cargo.md)
- [Cargo Book: Package Layout](../../output/docs/rust/cargo/guide/project-layout.md)

## Related wiki pages

- [Rust `main` function](main_function.md)
- [`src/main.rs`](src_main_rs.md)
- [Rust files use the `.rs` extension](rs_extension.md)
- [`rustc <filename>`](rustc_filename.md)
- [`cargo new <project_name>`](cargo_new.md)
- [Concepts so far](concepts.md)
