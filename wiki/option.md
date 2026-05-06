<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_chars_next/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html
  - https://doc.rust-lang.org/stable/std/option/enum.Option.html
  - https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
  - https://doc.rust-lang.org/stable/book/ch13-02-iterators.html
topic: rust-playground/option
---

# `Option`

`Option<T>` is a standard-library enum for a value that may be present or
absent. Its variants are `Some(T)` and `None`.

## Shape I have used

```rust
let mut chars = word.chars();

println!("first: {:?}", chars.next());
println!("done: {:?}", chars.next());
```

The printed values include both variants:

```console
first: Some('c')
done: None
```

## Variants

The standard library defines the shape as:

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

`Some(T)` holds a value of type `T`. `None` means there is no value.

`Option` is an enum. `Some(T)` and `None` are its variants.

## In `hello_chars_next`

`chars.next()` returns `Option<char>`.

When another `char` exists, `next` returns `Some(...)`:

```console
first: Some('c')
```

When no next `char` exists, `next` returns `None`:

```console
done: None
```

## Useful guardrail

`Option` is the wrapper shape. It is not the `char` itself. In `Some('c')`,
`'c'` is the contained `char`; `Some(...)` is the variant that says a value is
present.

## Corpus references

- [Rust std: `Option`](https://doc.rust-lang.org/stable/std/option/enum.Option.html)
- [The Rust Book: The `Option` Enum](https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html)
- [Rust std: `Iterator::next`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
- [The Rust Book: the `Iterator` trait and the `next` method](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html)

## Related wiki pages

- [`match`](match.md)
- [`Some`](some.md)
- [`None`](none.md)
- [`enum`](enum.md)
- [`Option<T>` syntax](option_t_syntax.md)
- [`Iterator::next`](iterator_next.md)
- [Iterator](iterators.md)
- [`char`](char.md)
- [Debug formatting](debug_formatting.md)
- [Concepts so far](concepts.md)
