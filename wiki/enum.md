<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_enum/src/main.rs
  - RustPlayground/experiments/hello_chars_next/src/main.rs
  - output/docs/rust/book/ch06-00-enums.md
  - output/docs/rust/std/option/enum.Option.md
topic: rust-playground/enum
---

# `enum`

An `enum` defines one type by listing the variants that values of that type can
have. In `hello_enum`, `Direction` is a custom enum with `Left` and `Right`
variants. In `hello_chars_next`, `Option` is the standard-library enum and
`Some(...)` and `None` are its variants.

## Custom enum in `hello_enum`

```rust
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
```

`Direction` is the enum type. `Left` and `Right` are the variants. Values are
constructed with paths:

```rust
let first_turn = Direction::Left;
let second_turn = Direction::Right;
```

Both values have the same type, `Direction`. This enum's variants do not carry
extra data; they only name which `Direction` value was chosen.

## Shape I have seen

The standard library defines `Option` with this shape:

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

The `hello_chars_next` experiment printed both variants:

```console
first: Some('c')
done: None
```

Both printed values have the same enum type, `Option<char>`. They are different
variants of that type.

## Variants

A value of an enum type can be one of its variants, but not both at the same
time. For `Option`, the variants are:

```rust
None
Some(T)
```

`None` carries no value. `Some(T)` carries one value of type `T`.

## Data in a variant

Some enum variants can carry data. In the current model, `Some('c')` is the
important example: the variant is `Some`, and the contained data is the `char`
value `'c'`.

`None` is still a variant of `Option<char>`, but it has no `char` inside it.
`Direction::Left` and `Direction::Right` are also variants with no carried
data.

## Useful guardrail

An enum is the type. A variant is one possible form of a value of that type.
`Option<char>` is not the same type as `char`; it is an enum type that can be
either `Some(char)` or `None`.

## Corpus references

- [The Rust Book: Enums and Pattern Matching](../../output/docs/rust/book/ch06-00-enums.md)
- [Rust std: `Option`](../../output/docs/rust/std/option/enum.Option.md)

## Related wiki pages

- [Variant](variant.md)
- [`match`](match.md)
- [`Option`](option.md)
- [`Option<T>` syntax](option_t_syntax.md)
- [`#[derive(Debug)]`](derive_debug.md)
- [`Some`](some.md)
- [`None`](none.md)
- [`Iterator::next`](iterator_next.md)
- [Types](types.md)
- [Concepts so far](concepts.md)
