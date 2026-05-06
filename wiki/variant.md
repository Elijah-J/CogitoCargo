<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_enum/src/main.rs
  - experiments/hello_chars_next/src/main.rs
  - experiments/hello_match/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html
  - https://doc.rust-lang.org/stable/reference/items/enumerations.html
topic: rust-playground/variant
---

# Variant

A variant is one possible form of a value of an enum type. The Rust Book says
we can *enumerate* all possible variants, which is where enumeration gets its
name. A value of an enum can be one of its variants, but not both at the same
time.

## Variants I have used

Three experiments have used enum variants:

In `hello_enum`, `Direction` has two variants with no carried data:

```rust
enum Direction {
    Left,
    Right,
}
```

`Direction::Left` and `Direction::Right` are both `Direction` values. The
variant names which kind of `Direction` was chosen.

In `hello_chars_next`, `Option<char>` has two variants:

```console
first: Some('c')
done: None
```

`Some('c')` carries data — the `char` value `'c'`. `None` carries no data.

In `hello_match`, a function branches on the variant:

```rust
fn describe(turn: Direction) -> &'static str {
    match turn {
        Direction::Left => "going left",
        Direction::Right => "going right",
    }
}
```

## With and without data

The Rust Reference says enum constructors can have named or unnamed fields.
In the current experiments, the two shapes are:

- **No data:** `Left`, `Right`, `None` — the variant itself is the entire
  value.
- **Unnamed field:** `Some(T)` — the variant carries one value inside it.
  `Some('c')` carries the `char` value `'c'`.

## Constructing a variant

Variants are constructed with a path: the enum name, `::`, and the variant
name.

```rust
Direction::Left
Direction::Right
```

For `Option`, the variants `Some` and `None` are used without the `Option::`
prefix because the standard library makes them available directly.

## Useful guardrail

A variant is not a type. `Direction::Left` is a value of type `Direction`, not
a type called `Left`. The Rust Reference says each enum defines one type; the
variants are the possible forms values of that type can take.

## Corpus references

- [The Rust Book: Defining an Enum](https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html)
- [Rust Reference: Enumerations](https://doc.rust-lang.org/stable/reference/items/enumerations.html)

## Related wiki pages

- [`enum`](enum.md)
- [`match`](match.md)
- [`Some`](some.md)
- [`None`](none.md)
- [`Option`](option.md)
- [Concepts so far](concepts.md)
