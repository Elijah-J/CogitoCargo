<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - output/docs/rust/book/ch01-03-hello-cargo.md
  - output/docs/rust/cargo/reference/build-cache.md
topic: rust-playground/target-directory
---

# `target/`

`target/` is Cargo's build-output directory. In the `hello_cargo` experiment,
Cargo created it after build-related commands and put the compiled program
under `target/debug/`.

## Where I saw it

The Cargo project now has this build-output path:

```text
hello_cargo/
`-- target/
    `-- debug/
        `-- hello_cargo
```

The Rust Book shows the same path when running the built executable directly:

```console
$ ./target/debug/hello_cargo
Hello, world!
```

## What belongs there

Cargo's build-cache docs say Cargo stores build output in `target`. For the
current beginner model, the important file is the compiled executable:

```text
target/debug/hello_cargo
```

The source file is still `src/main.rs`. The compiled binary is build output
inside `target/`.

## Useful guardrail

`target/` is generated build output, not source code. It can be rebuilt by
Cargo, so it is not part of the hand-written `hello_cargo` source.

## Corpus references

- [The Rust Book: Hello, Cargo!](../../output/docs/rust/book/ch01-03-hello-cargo.md)
- [Cargo Reference: Build cache](../../output/docs/rust/cargo/reference/build-cache.md)

## Related wiki pages

- [`cargo run`](cargo_run.md)
- [`cargo check`](cargo_check.md)
- [Running a compiled binary](run_compiled_binary.md)
- [`src/main.rs`](src_main_rs.md)
- [`Cargo.toml`](cargo_toml.md)
- [Concepts so far](concepts.md)
