# RustPlayground Wiki Philosophy

This wiki is a living reflection of current Rust knowledge, not a compressed
copy of the Rust documentation.

The learning style is bottom-up. A page should start from something actually
used in the playground, then add only enough explanation to make the current
mental model accurate.

## Page Standard

Each wiki page should answer:

- What did I use or see?
- What is my current model of it?
- What does it connect to?
- What is the one guardrail that prevents a misleading simplification?

## What To Keep

Keep details that explain the `hello_world` and `hello_cargo` experiments:

- the direct `rustc main.rs` flow
- running `./main`
- the Cargo package created by `cargo new hello_cargo`
- `Cargo.toml`, `Cargo.lock`, and `src/main.rs`
- `cargo run` and `cargo check`

Runnable experiments live under `RustPlayground/experiments/`. The wiki should
stay close to that experiment set.

Keep one next-layer fact when it protects the model. For example,
`Cargo.toml` should be more than "a file produced by `cargo new`" because it is
the manifest Cargo reads to understand the package.

## What To Defer

Defer reference-manual material until an experiment needs it:

- workspaces
- multiple binaries
- examples, benches, and tests
- release profiles
- feature flags
- dependency update commands
- module-tree details

Those topics can be added when they become part of the lived path.

## Editing Rule

When a page starts to feel like a full reference, trim it back to the current
experiment and leave only one or two signposts at the edge of current
knowledge.
