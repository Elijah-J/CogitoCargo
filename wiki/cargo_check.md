<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - output/docs/rust/cargo/commands/cargo-check.md
  - output/docs/rust/book/ch01-03-hello-cargo.md
  - output/docs/rust/cargo/commands/cargo-build.md
topic: rust-playground/cargo-check
---

# `cargo check`

`cargo check` checks whether the current Cargo package compiles without
producing the final executable. It is the fast feedback command for checking
work while editing.

## Shape I have used

From inside the package directory:

```console
$ cargo check
```

The Book's starter output looks like this:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

No `Hello, world!` appears because `cargo check` does not run the program.

## Relationship to `cargo build` and `cargo run`

`cargo build` produces an executable. `cargo run` builds and runs the
executable. `cargo check` stops earlier: Cargo's docs say it compiles without
the final code generation step.

That makes `cargo check` faster for repeated feedback while writing code.

## Useful guardrail

`cargo check` is not a final proof that the program can be run. Some diagnostics
only happen during final code generation, so use `cargo build` or `cargo run`
when you need the actual executable.

## Corpus references

- [Cargo Book: cargo-check](../../output/docs/rust/cargo/commands/cargo-check.md)
- [The Rust Book: Hello, Cargo!](../../output/docs/rust/book/ch01-03-hello-cargo.md)
- [Cargo Book: cargo-build](../../output/docs/rust/cargo/commands/cargo-build.md)

## Related wiki pages

- [`cargo run`](cargo_run.md)
- [`cargo new <project_name>`](cargo_new.md)
- [`Cargo.toml`](cargo_toml.md)
- [`src/main.rs`](src_main_rs.md)
- [Type inference](type_inference.md)
- [`let`](let_binding.md)
- [Shadowing](shadowing.md)
- [`let mut`](mutable_binding.md)
- [`error[E0384]`](compiler_error_e0384.md)
- [`error[E0425]`](compiler_error_e0425.md)
- [`error[E0308]`](compiler_error_e0308.md)
- [`target/`](target_directory.md)
- [Concepts so far](concepts.md)
