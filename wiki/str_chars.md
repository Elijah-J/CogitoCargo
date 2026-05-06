<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_chars_next/src/main.rs
  - RustPlayground/experiments/hello_utf8_chars_count/src/main.rs
  - output/docs/rust/std/primitive.str.md
  - output/docs/rust/std/str/struct.Chars.md
  - output/docs/rust/std/primitive.char.md
  - output/docs/rust/std/iter/trait.Iterator.md
  - output/docs/rust/std/option/enum.Option.md
  - output/docs/rust/book/ch08-02-strings.md
topic: rust-playground/str-chars
---

# `str::chars`

`str::chars` is the standard-library method that returns an iterator over the
`char` values of a `&str`.

## Shape I have used

```rust
let word: &str = "café";

let char_count = word.chars().count();
```

`word.chars()` creates the iterator. `.count()` is then called on that
iterator.

## What it produces

The standard library says `chars` returns an iterator over the `char`s of a
`&str`. It can do this because `str` contains valid UTF-8.

For `hello_utf8_chars_count`:

```console
chars: 4
```

That count comes from the four `char` values in `"café"`: `c`, `a`, `f`, and
`é`.

## Compared with `.len()`

```rust
let byte_count = word.len();
let char_count = word.chars().count();
```

`word.len()` returns the byte length. `word.chars()` changes the view from
bytes to `char` values, and `.count()` counts those values.

For `"café"`, the byte count is `5` and the `char` count is `4`.

## With `next`

`hello_chars_next` stores the `Chars` iterator and advances it directly:

```rust
let mut chars = word.chars();

println!("first: {:?}", chars.next());
```

The first call returns `Some('c')`. Later calls return the remaining `char`
values, and the call after `é` returns `None`.

## Useful guardrail

`chars` does not return grapheme clusters. The standard library docs say
`char` is a Unicode scalar value and may not match a human idea of a character.
Grapheme-cluster iteration is not provided by Rust's standard library.

## Corpus references

- [Rust std: `str::chars`](../../output/docs/rust/std/primitive.str.md)
- [Rust std: `Chars`](../../output/docs/rust/std/str/struct.Chars.md)
- [Rust std: `char`](../../output/docs/rust/std/primitive.char.md)
- [Rust std: `Iterator::count`](../../output/docs/rust/std/iter/trait.Iterator.md)
- [Rust std: `Option`](../../output/docs/rust/std/option/enum.Option.md)
- [The Rust Book: iterating over strings](../../output/docs/rust/book/ch08-02-strings.md)

## Related wiki pages

- [`char`](char.md)
- [Iterator](iterators.md)
- [`Iterator::next`](iterator_next.md)
- [`Iterator::count`](iterator_count.md)
- [`Option`](option.md)
- [`.len()` on `&str`](str_len.md)
- [`&str`](str.md)
- [UTF-8](utf_8.md)
- [Concepts so far](concepts.md)
