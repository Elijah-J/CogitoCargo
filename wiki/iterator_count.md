<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_utf8_chars_count/src/main.rs
  - https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
  - https://doc.rust-lang.org/stable/book/ch13-02-iterators.html
  - https://doc.rust-lang.org/stable/std/primitive.str.html
topic: rust-playground/iterator-count
---

# `Iterator::count`

`Iterator::count` consumes an iterator and returns how many items it produced.
In `hello_utf8_chars_count`, `.count()` counts the `char` values produced by
`word.chars()`.

## Shape I have used

```rust
let word: &str = "café";

let char_count = word.chars().count();
```

The program prints:

```console
chars: 4
```

`word.chars()` creates an iterator over `char` values. `.count()` consumes that
iterator and returns `4`.

## What count does

The standard library documents `count` as consuming the iterator, counting the
number of iterations, and returning that number. It does this by repeatedly
calling `next` until the iterator returns `None`.

The Rust Book uses the same idea when it describes consuming iterator methods:
they use up the iterator to produce a result.

## In the method chain

```rust
word.chars().count()
```

The chain runs left to right:

1. `word.chars()` creates a `Chars` iterator.
2. `.count()` consumes that iterator.
3. The expression evaluates to the number of `char` values.

## Useful guardrail

`.count()` counts iterator items. It does not decide what a string item should
be. In `hello_utf8_chars_count`, `chars()` decides that the items are Rust
`char` values.

## Corpus references

- [Rust std: `Iterator::count`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
- [The Rust Book: consuming iterators](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html)
- [Rust std: `str::chars`](https://doc.rust-lang.org/stable/std/primitive.str.html)

## Related wiki pages

- [Iterator](iterators.md)
- [`Iterator::next`](iterator_next.md)
- [`str::chars`](str_chars.md)
- [`char`](char.md)
- [`.len()` on `&str`](str_len.md)
- [Function calls](function_calls.md)
- [Concepts so far](concepts.md)
