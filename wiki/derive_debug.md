<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_enum/src/main.rs
  - output/docs/rust/std/fmt/trait.Debug.md
  - output/docs/rust/book/appendix-03-derivable-traits.md
  - output/docs/rust/book/appendix-02-operators.md
topic: rust-playground/derive-debug
---

# `#[derive(Debug)]`

`#[derive(Debug)]` is an attribute placed before a type definition. In
`hello_enum`, it appears before the `Direction` enum:

```rust
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
```

## What `derive` does here

The Rust Book appendix says the `derive` attribute can be applied to a struct
or enum definition. It generates code that gives the annotated type a standard
capability.

For `hello_enum`, the capability is `Debug`. That generated code is what lets
`println!` use `{:?}` with `Direction` values:

```rust
println!("First turn: {:?}", first_turn);
println!("Second turn: {:?}", second_turn);
```

## What got printed

The output from `hello_enum` is:

```console
First turn: Left
Second turn: Right
```

The standard-library `Debug` docs say derived `Debug` for enums uses the
variant name, plus field values when a variant has fields. `Direction::Left`
and `Direction::Right` have no fields, so the debug output is just `Left` and
`Right`.

## Useful guardrail

`Debug` is for programmer-facing inspection. The standard-library docs say
derived `Debug` formats are not stable, so `hello_enum` uses this as a learning
and debugging tool, not as a stable text format.

## Corpus references

- [Rust std: `Debug`](../../output/docs/rust/std/fmt/trait.Debug.md)
- [The Rust Book: Derivable Traits](../../output/docs/rust/book/appendix-03-derivable-traits.md)
- [The Rust Reference operator appendix: attributes](../../output/docs/rust/book/appendix-02-operators.md)

## Related wiki pages

- [`enum`](enum.md)
- [Debug formatting](debug_formatting.md)
- [Rust `println!` macro](println_macro.md)
- [Concepts so far](concepts.md)
