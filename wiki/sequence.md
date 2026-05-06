<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_chars_next/src/main.rs
  - experiments/hello_array_for/src/main.rs
  - experiments/hello_array_index/src/main.rs
  - experiments/hello_array_len/src/main.rs
  - https://doc.rust-lang.org/stable/std/primitive.array.html
  - https://doc.rust-lang.org/stable/std/primitive.slice.html
  - https://doc.rust-lang.org/stable/book/ch13-02-iterators.html
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
  - https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
  - https://doc.rust-lang.org/stable/std/str/struct.Chars.html
  - https://doc.rust-lang.org/stable/std/option/enum.Option.html
  - https://doc.rust-lang.org/stable/reference/types/array.html
  - https://doc.rust-lang.org/stable/reference/expressions/array-expr.html
topic: rust-playground/sequence
---

# Sequence

A sequence is multiple items in an order. In `hello_chars_next`,
`word.chars()` walks through the ordered `char` values in `"café"`. In
`hello_array_for`, `[3, 4, 5]` is an array that stores integer elements in
order.

## Shape I have used

```rust
let word: &str = "café";
let mut chars = word.chars();
```

Repeated calls to `next` produce:

```console
Some('c')
Some('a')
Some('f')
Some('é')
None
```

The sequence of `char` items is `c`, then `a`, then `f`, then `é`.

## Array sequence

`hello_array_for` uses an array expression:

```rust
let numbers = [3, 4, 5];
```

The array elements are `3`, then `4`, then `5`. A `for` loop can walk those
elements in order:

```rust
for number in numbers {
    println!("number: {number}");
}
```

`hello_array_index` uses indexes to choose positions from the same ordered
array:

```rust
let first = numbers[0];
let second = numbers[1];
```

Index `0` is the first position, and index `1` is the second position.

`hello_array_len` asks how many items are in that stored sequence:

```rust
let length = numbers.len();
```

For `[3, 4, 5]`, the answer is `3`.

## Iterator connection

The Rust Book describes iterators as operating on a sequence of items in turn.
The iterator owns the logic for moving from one item to the next and knowing
when the sequence has finished.

In `hello_chars_next`, `chars` is the iterator and each item is a `char`. In
`hello_array_for`, the array is a stored sequence and each loop item is one
integer element.

## Useful guardrail

Sequence is a broader idea than array. In `hello_chars_next`, the sequence is
the ordered stream of `char` values produced by `word.chars()`. In
`hello_array_for`, the sequence is the fixed set of elements stored by the
array.

## Corpus references

- [The Rust Book: iterators](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html)
- [The Rust Book: Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [Rust std: array primitive](https://doc.rust-lang.org/stable/std/primitive.array.html)
- [Rust std: slice `len`](https://doc.rust-lang.org/stable/std/primitive.slice.html)
- [Rust std: `Iterator`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
- [Rust std: `Chars`](https://doc.rust-lang.org/stable/std/str/struct.Chars.html)
- [Rust std: `Option`](https://doc.rust-lang.org/stable/std/option/enum.Option.html)
- [Rust Reference: Array types](https://doc.rust-lang.org/stable/reference/types/array.html)
- [Rust Reference: Array indexing expressions](https://doc.rust-lang.org/stable/reference/expressions/array-expr.html)

## Related wiki pages

- [Iterator](iterators.md)
- [`Iterator::next`](iterator_next.md)
- [`str::chars`](str_chars.md)
- [Array](array.md)
- [Array indexing](array_indexing.md)
- [Array length](array_len.md)
- [`char`](char.md)
- [`Option`](option.md)
- [Concepts so far](concepts.md)
