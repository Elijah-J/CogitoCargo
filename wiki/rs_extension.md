<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html
  - https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html
  - https://doc.rust-lang.org/stable/cargo/guide/project-layout.html
topic: rust-playground/rs-extension
---

# Rust Files Use the `.rs` Extension

Rust source files end with `.rs`. In `hello_world`, that showed up as
`main.rs`; in `hello_cargo`, it showed up as `src/main.rs`.

## Direct file

`hello_world` used a source file named `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

The file was compiled by name:

```console
$ rustc main.rs
```

The `.rs` file is source code. The compiled `main` file is the executable.

## Cargo file

`cargo new hello_cargo` put the Rust source file inside `src/`:

```text
hello_cargo/
|-- Cargo.toml
`-- src/
    `-- main.rs
```

Cargo keeps source code under `src`, while `Cargo.toml` stays at the package
root.

## Useful guardrail

The Book also notes the filename convention for multiple words:
`hello_world.rs`, not `helloworld.rs`. That convention is about readability;
the main rule for now is that Rust source files end in `.rs`.

## Corpus references

- [The Rust Book: Hello, World!](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html)
- [The Rust Book: Hello, Cargo!](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html)
- [Cargo Book: Package Layout](https://doc.rust-lang.org/stable/cargo/guide/project-layout.html)

## Related wiki pages

- [`main.rs`](main_rs.md)
- [`src/main.rs`](src_main_rs.md)
- [`rustc <filename>`](rustc_filename.md)
- [`cargo new <project_name>`](cargo_new.md)
- [Concepts so far](concepts.md)
