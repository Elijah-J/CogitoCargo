<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - output/docs/rust/cargo/reference/manifest.md
  - output/docs/rust/book/ch01-03-hello-cargo.md
  - output/docs/rust/cargo/guide/cargo-toml-vs-cargo-lock.md
topic: rust-playground/cargo-toml
---

# `Cargo.toml`

`Cargo.toml` is Cargo's manifest file for a package. In `hello_cargo`, it was
the package-level file Cargo read to know what it was building.

## Starter manifest

`cargo new hello_cargo` created this kind of starter file:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

`[package]` contains basic package metadata. In this starter file, that means
the package name, package version, and Rust edition. `[dependencies]` is where
external crates will be listed later.

## Where it lives

`Cargo.toml` lives at the package root:

```text
hello_cargo/
|-- Cargo.toml
`-- src/
    `-- main.rs
```

The source code lives under `src/`. The manifest sits above it and describes
the package.

## Useful guardrail

`Cargo.toml` is not just "a file produced by `cargo new`." It is the file that
makes the directory a Cargo package. Cargo commands such as `cargo run` and
`cargo check` use it to understand the package.

## `Cargo.toml` versus `Cargo.lock`

`Cargo.toml` is the file a developer edits to describe the package and its
dependencies. `Cargo.lock` is maintained by Cargo and records exact dependency
versions after Cargo resolves them.

## Corpus references

- [Cargo Reference: The Manifest Format](../../output/docs/rust/cargo/reference/manifest.md)
- [The Rust Book: Hello, Cargo!](../../output/docs/rust/book/ch01-03-hello-cargo.md)
- [Cargo Guide: Cargo.toml vs Cargo.lock](../../output/docs/rust/cargo/guide/cargo-toml-vs-cargo-lock.md)

## Related wiki pages

- [`cargo new <project_name>`](cargo_new.md)
- [`Cargo.lock`](cargo_lock.md)
- [`src/main.rs`](src_main_rs.md)
- [`cargo run`](cargo_run.md)
- [`cargo check`](cargo_check.md)
- [Concepts so far](concepts.md)
