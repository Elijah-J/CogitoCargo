<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_chars_next/src/main.rs
  - RustPlayground/experiments/hello_utf8_chars_count/src/main.rs
  - RustPlayground/experiments/hello_utf8_len/src/main.rs
  - output/docs/rust/std/primitive.char.md
  - output/docs/rust/std/primitive.str.md
  - output/docs/rust/std/option/enum.Option.md
  - output/docs/rust/reference/types/char.md
  - output/docs/rust/book/ch08-02-strings.md
topic: rust-playground/char
---

# `char`

`char` is Rust's type for a Unicode scalar value. In `hello_utf8_chars_count`
and `hello_chars_next`, `char` appears through `word.chars()`, which walks
through a `&str` as `char` values.

## Shape I have used

```rust
let word: &str = "café";

let byte_count = word.len();
let char_count = word.chars().count();
```

The program prints:

```console
bytes: 5
chars: 4
```

`word.len()` counts bytes. `word.chars().count()` counts the `char` values
produced by the `chars` iterator.

## Unicode scalar value

The Rust standard library says `char` is a Unicode scalar value. The Rust Book
uses the same model when it distinguishes bytes, scalar values, and grapheme
clusters.

For `"café"`, the visible text is simple enough that the `char` count is `4`:
`c`, `a`, `f`, and `é`. The byte count is `5` because `é` takes more than one
byte in UTF-8.

`hello_chars_next` makes those `char` values visible one at a time:

```console
Some('c')
Some('a')
Some('f')
Some('é')
```

## Useful guardrail

Rust `char` is not the same thing as "whatever a human would call one letter"
in every language. The standard library docs for `str::chars` warn that
`char` values are Unicode scalar values and may not match a human idea of a
character. Grapheme clusters are a separate topic, and Rust's standard library
does not provide grapheme-cluster iteration.

## Corpus references

- [Rust std: `char`](../../output/docs/rust/std/primitive.char.md)
- [Rust Reference: `char`](../../output/docs/rust/reference/types/char.md)
- [Rust std: `str::chars`](../../output/docs/rust/std/primitive.str.md)
- [Rust std: `Option`](../../output/docs/rust/std/option/enum.Option.md)
- [The Rust Book: bytes, scalar values, and grapheme clusters](../../output/docs/rust/book/ch08-02-strings.md)

## Related wiki pages

- [`str::chars`](str_chars.md)
- [Iterator](iterators.md)
- [`Iterator::next`](iterator_next.md)
- [`Iterator::count`](iterator_count.md)
- [`Option`](option.md)
- [`.len()` on `&str`](str_len.md)
- [UTF-8](utf_8.md)
- [`&str`](str.md)
- [Concepts so far](concepts.md)
