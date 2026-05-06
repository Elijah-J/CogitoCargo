<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_chars_next/src/main.rs
  - RustPlayground/experiments/hello_utf8_chars_count/src/main.rs
  - output/docs/rust/book/ch13-02-iterators.md
  - output/docs/rust/std/iter/trait.Iterator.md
  - output/docs/rust/std/option/enum.Option.md
  - output/docs/rust/std/primitive.str.md
  - output/docs/rust/std/str/struct.Chars.md
topic: rust-playground/iterators
---

# Iterator

An iterator is a value that produces items from a sequence one at a time. In
`hello_utf8_chars_count`, `word.chars()` creates an iterator over the `char`
values in a `&str`.

## Shape I have used

```rust
let word: &str = "café";

let char_count = word.chars().count();
```

`word.chars()` creates the iterator. `.count()` consumes that iterator and
returns how many items it produced.

## `Iterator` and `next`

The Rust Book says all iterators implement the standard-library `Iterator`
interface. The standard library shows the required method:

```rust
fn next(&mut self) -> Option<Self::Item>;
```

`hello_utf8_chars_count` does not call `next` directly. It uses `.count()`,
which the standard library documents as repeatedly calling `next` until the
iterator is finished.

## Laziness

The Rust Book says iterators are lazy: creating an iterator by itself does not
use it up. A consuming method is what drives the iterator.

In `hello_utf8_chars_count`, `.count()` is the consuming method:

```rust
let char_count = word.chars().count();
```

The result is an integer count, not another iterator.

## One item at a time

The `hello_chars_next` experiment calls `next` directly:

```rust
let mut chars = word.chars();

println!("first: {:?}", chars.next());
println!("done: {:?}", chars.next());
```

Each call advances the iterator. While a next item exists, `next` returns
`Some(item)`. When no next item exists, it returns `None`.

## Useful guardrail

`Iterator` is a standard-library interface. This page covers the `Chars` iterator
created by `str::chars`, used in `hello_utf8_chars_count`, `hello_chars_next`,
and `hello_for`. Other iterator methods and custom iterator types are later
topics.

## Corpus references

- [The Rust Book: iterators](../../output/docs/rust/book/ch13-02-iterators.md)
- [Rust std: `Iterator`](../../output/docs/rust/std/iter/trait.Iterator.md)
- [Rust std: `Option`](../../output/docs/rust/std/option/enum.Option.md)
- [Rust std: `str::chars`](../../output/docs/rust/std/primitive.str.md)
- [Rust std: `Chars`](../../output/docs/rust/std/str/struct.Chars.md)

## Related wiki pages

- [`for` loops](for_loops.md)
- [`Iterator::next`](iterator_next.md)
- [`str::chars`](str_chars.md)
- [`Iterator::count`](iterator_count.md)
- [Sequence](sequence.md)
- [`Option`](option.md)
- [`char`](char.md)
- [`str::len` syntax](str_len_syntax.md)
- [Function calls](function_calls.md)
- [Concepts so far](concepts.md)
