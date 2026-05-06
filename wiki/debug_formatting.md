<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_enum/src/main.rs
  - RustPlayground/experiments/hello_chars_next/src/main.rs
  - output/docs/rust/std/macro.println.md
  - output/docs/rust/std/macro.format.md
  - output/docs/rust/std/fmt/trait.Debug.md
  - output/docs/rust/std/option/enum.Option.md
topic: rust-playground/debug-formatting
---

# Debug Formatting

`{:?}` asks Rust formatting to use `Debug`. In `hello_chars_next`,
it prints the `Option<char>` values returned by `chars.next()`. In
`hello_enum`, it prints custom `Direction` values after `Direction` derives
`Debug`.

## Shape I have used

```rust
println!("first: {:?}", chars.next());
```

The output is:

```console
first: Some('c')
```

## Why `{}` was not used

`hello_variables` used `{name}`, and earlier formatting examples used `{}` for
values printed with the usual display formatting. The `chars.next()` expression
returns `Option<char>`, and `hello_chars_next` prints that wrapper directly.

The standard-library `println!` docs say `println!` uses the same syntax as
`format!`. The `Debug` docs describe `?` formatting as programmer-facing
debugging output.

## What got printed

The standard library implements `Debug` for `char` and for `Option<T>` when
`T` implements `Debug`. That is enough for `{:?}` to print an `Option<char>`.

In `hello_chars_next`:

```console
first: Some('c')
done: None
```

`Some('c')` and `None` are debug-formatted `Option<char>` values.

In `hello_enum`, the custom enum is marked with `#[derive(Debug)]`:

```rust
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
```

That lets this code use `{:?}` with `Direction` values:

```rust
println!("First turn: {:?}", first_turn);
println!("Second turn: {:?}", second_turn);
```

The output is:

```console
First turn: Left
Second turn: Right
```

The derived debug format uses the variant name for these fieldless variants.

## Useful guardrail

Debug output is useful for inspection. The standard library docs say `Debug`
formats are not stable, so this page uses `{:?}` as a learning tool rather
than as a promise about long-term text output.

## Corpus references

- [Rust std: `println!`](../../output/docs/rust/std/macro.println.md)
- [Rust std: `format!`](../../output/docs/rust/std/macro.format.md)
- [Rust std: `Debug`](../../output/docs/rust/std/fmt/trait.Debug.md)
- [Rust std: `Option`](../../output/docs/rust/std/option/enum.Option.md)

## Related wiki pages

- [Rust `println!` macro](println_macro.md)
- [`#[derive(Debug)]`](derive_debug.md)
- [`enum`](enum.md)
- [`Option`](option.md)
- [`Iterator::next`](iterator_next.md)
- [`char`](char.md)
- [`Some`](some.md)
- [`None`](none.md)
- [Concepts so far](concepts.md)
