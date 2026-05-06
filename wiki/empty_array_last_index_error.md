<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_array_last_index_if/src/main.rs
  - experiments/hello_empty_array_last_index_error/src/main.rs
  - experiments/hello_empty_array_len/src/main.rs
  - experiments/hello_array_last_index/src/main.rs
  - https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html
  - https://doc.rust-lang.org/stable/reference/behavior-not-considered-unsafe.html
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
  - https://doc.rust-lang.org/stable/std/primitive.usize.html
  - https://doc.rust-lang.org/stable/std/primitive.slice.html
topic: rust-playground/empty-array-last-index-error
---

# Empty Array Last Index Error

An empty array has no last index. In `hello_empty_array_last_index_error`,
`numbers.len() - 1` tries to subtract `1` from the `usize` value `0`.

## Shape I have used

```rust
fn main() {
    let numbers: [i32; 0] = [];

    // Empty arrays have length 0, so subtracting 1 from len underflows.
    let last_index: usize = numbers.len() - 1;

    println!("last index: {last_index}");
}
```

`cargo check` accepts this program, but `cargo run` fails in debug mode:

```console
thread 'main' panicked at src/main.rs:5:29:
attempt to subtract with overflow
```

The program does not reach `println!`.

## Why the subtraction fails

`numbers.len()` returns `0` because `numbers` is an empty array. The return type
of `len` is `usize`.

`usize` is unsigned, so it cannot represent `-1`. The expression below
underflows by trying to produce that value:

```rust
numbers.len() - 1
```

The Rust Reference uses the term overflow for integer results outside the
stored range, including values below the minimum. In ordinary arithmetic
language, this specific case is underflow. Rust reports it with the panic
message `attempt to subtract with overflow`.

## Not an indexing failure yet

This experiment does not index the array:

```rust
let last_index: usize = numbers.len() - 1;
```

The panic happens while computing `last_index`. The program fails before any
`numbers[last_index]` expression exists.

## Useful guardrail

`len() - 1` is only valid after the code has proved the length is greater than
`0`. Empty arrays need a branch or another checked shape before computing a
last index.

`hello_array_last_index_if` uses a branch:

```rust
if numbers.len() > 0 {
    let last_index: usize = numbers.len() - 1;
    println!("last index: {last_index}");
} else {
    println!("empty array");
}
```

For an empty array, the condition is false, so the subtraction does not run.

## Corpus references

- [Rust Reference: Operator overflow](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html)
- [Rust Reference: Integer overflow](https://doc.rust-lang.org/stable/reference/behavior-not-considered-unsafe.html)
- [The Rust Book: Integer Overflow](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [Rust std: `usize`](https://doc.rust-lang.org/stable/std/primitive.usize.html)
- [Rust std: slice `len`](https://doc.rust-lang.org/stable/std/primitive.slice.html)

## Related wiki pages

- [Empty array](empty_array.md)
- [Array last index with `if`](array_last_index_if.md)
- [Array last index](array_last_index.md)
- [Array length](array_len.md)
- [Array index bounds](array_index_bounds.md)
- [`usize`](usize.md)
- [Integer overflow](integer_overflow.md)
- [Subtraction operator](subtraction_operator.md)
- [Concepts so far](concepts.md)
