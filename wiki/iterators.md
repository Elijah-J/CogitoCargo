<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_chars_next/src/main.rs
  - experiments/hello_utf8_chars_count/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch13-02-iterators.html
  - https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
  - https://doc.rust-lang.org/stable/std/option/enum.Option.html
  - https://doc.rust-lang.org/stable/std/primitive.str.html
  - https://doc.rust-lang.org/stable/std/str/struct.Chars.html
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

- [The Rust Book: iterators](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html)
- [Rust std: `Iterator`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
- [Rust std: `Option`](https://doc.rust-lang.org/stable/std/option/enum.Option.html)
- [Rust std: `str::chars`](https://doc.rust-lang.org/stable/std/primitive.str.html)
- [Rust std: `Chars`](https://doc.rust-lang.org/stable/std/str/struct.Chars.html)

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
