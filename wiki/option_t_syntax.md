<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_chars_next/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html
  - https://doc.rust-lang.org/stable/std/option/enum.Option.html
topic: rust-playground/option-t-syntax
---

# `Option<T>` Syntax

`Option<T>` is the type syntax for the `Option` enum with a contained type
plugged into `T`. In `hello_chars_next`, `chars.next()` returns
`Option<char>`, so `Some(...)` can contain a `char`.

## Shape

The standard library defines `Option` as:

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

The Rust Book calls `<T>` a generic type parameter. In this page, `T` is a
placeholder for the type that `Some` can contain.

## In `hello_chars_next`

`word.chars()` creates an iterator whose item type is `char`. Because
`Iterator::next` returns `Option<Self::Item>`, the calls in `hello_chars_next`
return `Option<char>`.

```rust
let mut chars = word.chars();

println!("first: {:?}", chars.next());
println!("done: {:?}", chars.next());
```

The output includes:

```console
first: Some('c')
done: None
```

For this case, read `Option<char>` as: either `Some(char)` or `None`.

## Different from the inner type

`Option<T>` and `T` are different types. In `hello_chars_next`,
`Option<char>` is different from `char`.

That is why `Some('c')` is not just `'c'`. It is an `Option<char>` value whose
variant is `Some` and whose contained `char` is `'c'`.

## Useful guardrail

This page is only the part of generic syntax needed for `Option<T>`. Writing
custom generic types is a later topic.

## Corpus references

- [The Rust Book: The `Option` Enum](https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html)
- [Rust std: `Option`](https://doc.rust-lang.org/stable/std/option/enum.Option.html)

## Related wiki pages

- [`Option`](option.md)
- [`enum`](enum.md)
- [`Some`](some.md)
- [`None`](none.md)
- [`char`](char.md)
- [Types](types.md)
- [Type annotations](type_annotations.md)
- [`Iterator::next`](iterator_next.md)
- [Concepts so far](concepts.md)
