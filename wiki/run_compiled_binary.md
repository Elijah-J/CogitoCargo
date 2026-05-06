<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - output/docs/rust/book/ch01-02-hello-world.md
  - output/docs/rust/book/ch01-03-hello-cargo.md
  - output/docs/rust/cargo/commands/cargo-run.md
topic: rust-playground/run-compiled-binary
---

# Running a Compiled Binary

Running a compiled binary executes the program produced from Rust source code.
In `hello_world`, compiling with `rustc` and running the binary were separate
commands.

## Direct `rustc` workflow

Compile the source file:

```console
$ rustc main.rs
```

Run the produced binary:

```console
$ ./main
Hello, world!
```

`./main` means "run the executable named `main` in the current directory." It
does not run `main.rs` directly.

## Cargo workflow

Cargo can compile and run the starter package in one command:

```console
$ cargo run
Hello, world!
```

The Book also shows the longer path: `cargo build` writes the debug executable
under `target/debug/`, and that executable can be run directly.

## Useful guardrail

Source files and executables are different artifacts. `main.rs` and
`src/main.rs` are source files. `main` and `target/debug/hello_cargo` are
compiled binaries.

## Corpus references

- [The Rust Book: Hello, World!](../../output/docs/rust/book/ch01-02-hello-world.md)
- [The Rust Book: Hello, Cargo!](../../output/docs/rust/book/ch01-03-hello-cargo.md)
- [Cargo Book: cargo-run](../../output/docs/rust/cargo/commands/cargo-run.md)

## Related wiki pages

- [`rustc <filename>`](rustc_filename.md)
- [`cargo run`](cargo_run.md)
- [`target/`](target_directory.md)
- [`main.rs`](main_rs.md)
- [`src/main.rs`](src_main_rs.md)
- [Concepts so far](concepts.md)
