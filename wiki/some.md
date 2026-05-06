<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_chars_next/src/main.rs
  - output/docs/rust/std/option/enum.Option.md
  - output/docs/rust/std/iter/trait.Iterator.md
topic: rust-playground/some
---

# `Some`

`Some(T)` is the `Option` variant that contains a value. In
`hello_chars_next`, `Some('c')` means `next` produced the `char` value `'c'`.

## Shape I have used

```console
first: Some('c')
second: Some('a')
third: Some('f')
fourth: Some('é')
```

Each line comes from a call to:

```rust
chars.next()
```

## Contained value

The standard library describes `Some(T)` as some value of type `T`. The `T`
means the contained value can have different types in different uses of
`Option`.

In `hello_chars_next`, `chars.next()` returns `Option<char>`, so each `Some`
contains a `char`.

## Useful guardrail

`Some('c')` is not just `'c'`. It is an `Option` value saying that a `char` is
present.

## Corpus references

- [Rust std: `Option`](../../output/docs/rust/std/option/enum.Option.md)
- [Rust std: `Iterator::next`](../../output/docs/rust/std/iter/trait.Iterator.md)

## Related wiki pages

- [`Option`](option.md)
- [`match`](match.md)
- [`enum`](enum.md)
- [`Option<T>` syntax](option_t_syntax.md)
- [`None`](none.md)
- [`Iterator::next`](iterator_next.md)
- [`char`](char.md)
- [Debug formatting](debug_formatting.md)
- [Concepts so far](concepts.md)
