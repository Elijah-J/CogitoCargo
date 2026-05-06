<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - https://doc.rust-lang.org/stable/cargo/commands/cargo-run.html
  - https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html
  - https://doc.rust-lang.org/stable/cargo/commands/cargo-build.html
topic: rust-playground/cargo-run
---

# `cargo run`

`cargo run` builds and runs the current Cargo package. In `hello_cargo`, it
compiled `src/main.rs` and then ran the resulting program.

## Shape I have used

From inside the package directory:

```console
$ cargo run
```

For the starter project, the important result is the program output:

```console
Hello, world!
```

Cargo also prints its own build status around that output.

## Relationship to `cargo build`

The Book shows that `cargo build` creates an executable under `target/debug/`.
`cargo run` is the convenient form that builds and then runs that executable in
one command.

If nothing has changed, Cargo can reuse the existing build output. If a source
file changed, Cargo rebuilds before running.

That build output lives under `target/`.

## Useful guardrail

`cargo run` runs the package, not an arbitrary loose `.rs` file. It depends on
the Cargo package structure, especially `Cargo.toml` and `src/main.rs`.

## Corpus references

- [Cargo Book: cargo-run](https://doc.rust-lang.org/stable/cargo/commands/cargo-run.html)
- [The Rust Book: Hello, Cargo!](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html)
- [Cargo Book: cargo-build](https://doc.rust-lang.org/stable/cargo/commands/cargo-build.html)

## Related wiki pages

- [`cargo check`](cargo_check.md)
- [`cargo new <project_name>`](cargo_new.md)
- [`Cargo.toml`](cargo_toml.md)
- [`src/main.rs`](src_main_rs.md)
- [`let`](let_binding.md)
- [`target/`](target_directory.md)
- [Running a compiled binary](run_compiled_binary.md)
- [Concepts so far](concepts.md)
