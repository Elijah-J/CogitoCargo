<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_chars_next/src/main.rs
  - https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
  - https://doc.rust-lang.org/stable/book/ch13-02-iterators.html
  - https://doc.rust-lang.org/stable/std/str/struct.Chars.html
  - https://doc.rust-lang.org/stable/std/option/enum.Option.html
topic: rust-playground/iterator-next
---

# `Iterator::next`

`Iterator::next` advances an iterator and returns the next item. The return
type is `Option`, so a call can return either `Some(item)` or `None`.

## Shape I have used

```rust
let word: &str = "café";

let mut chars = word.chars();

println!("first: {:?}", chars.next());
println!("second: {:?}", chars.next());
println!("third: {:?}", chars.next());
println!("fourth: {:?}", chars.next());
println!("done: {:?}", chars.next());
```

The program prints:

```console
first: Some('c')
second: Some('a')
third: Some('f')
fourth: Some('é')
done: None
```

## Return shape

The standard library defines `next` under `Iterator` like this:

```rust
fn next(&mut self) -> Option<Self::Item>;
```

For the `Chars` iterator created by `word.chars()`, the item type is `char`.
That makes the `hello_chars_next` calls return `Option<char>`.

## Mutable iterator binding

```rust
let mut chars = word.chars();
```

The `next` method takes `&mut self`. The Rust Book says calling `next` changes
internal iterator state so the iterator can track where it is in the sequence.
That is why `hello_chars_next` binds the iterator with `mut`.

## End of iteration

Each successful call returns `Some(...)` with the next `char`. After the last
`char`, the next call returns `None`.

In `hello_chars_next`, `None` means the `Chars` iterator has no next `char` left
to produce.

## Useful guardrail

`next` produces one item at a time. It does not count all items; that was
`Iterator::count`. It also does not decide what the item type is; `str::chars`
created an iterator whose item type is `char`.

## Corpus references

- [Rust std: `Iterator::next`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
- [The Rust Book: the `Iterator` trait and the `next` method](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html)
- [Rust std: `Chars`](https://doc.rust-lang.org/stable/std/str/struct.Chars.html)
- [Rust std: `Option`](https://doc.rust-lang.org/stable/std/option/enum.Option.html)

## Related wiki pages

- [Iterator](iterators.md)
- [Sequence](sequence.md)
- [`str::chars`](str_chars.md)
- [`Option`](option.md)
- [`Option<T>` syntax](option_t_syntax.md)
- [`Some`](some.md)
- [`None`](none.md)
- [`char`](char.md)
- [Debug formatting](debug_formatting.md)
- [Concepts so far](concepts.md)
