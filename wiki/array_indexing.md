<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_array_last_index/src/main.rs
  - RustPlayground/experiments/hello_array_index/src/main.rs
  - RustPlayground/experiments/hello_array_index_error/src/main.rs
  - RustPlayground/experiments/hello_array_for/src/main.rs
  - RustPlayground/experiments/hello_array_len/src/main.rs
  - output/docs/rust/std/primitive.slice.md
  - output/docs/rust/reference/types/numeric.md
  - output/docs/rust/book/ch03-02-data-types.md
  - output/docs/rust/reference/expressions/array-expr.md
topic: rust-playground/array-indexing
---

# Array Indexing

Array indexing reads one element from an array by position. In
`hello_array_index`, `numbers[0]` reads the first element and `numbers[1]`
reads the second element.

## Shape I have used

```rust
fn main() {
    let numbers = [3, 4, 5];

    let first = numbers[0];
    let second = numbers[1];

    println!("first: {first}");
    println!("second: {second}");
}
```

The program prints:

```console
first: 3
second: 4
```

## Index syntax

```rust
numbers[0]
```

The array name comes first. The index expression goes inside square brackets.
The Rust Book uses the same shape to access array elements.

## Zero-based positions

The Rust Reference says array indexes are zero-based. That means the first
element is at index `0`, not index `1`.

For this array:

```rust
let numbers = [3, 4, 5];
```

The indexed values are:

```rust
numbers[0] // 3
numbers[1] // 4
```

## Computed index

`hello_array_last_index` uses a binding as the index expression:

```rust
let last_index: usize = numbers.len() - 1;
let last = numbers[last_index];
```

The Rust Reference says array and slice indexes use a square-bracket-enclosed
expression of type `usize`. `last_index` has type `usize`, so it can go inside
`[]`.

For `[3, 4, 5]`, `last_index` is `2`, so `numbers[last_index]` reads `5`.

## Useful guardrail

This experiment only reads indexes that exist in the array. What happens for
an index outside the array is covered by `hello_array_index_error`.

## Corpus references

- [The Rust Book: Array Element Access](../../output/docs/rust/book/ch03-02-data-types.md)
- [Rust Reference: Array indexing expressions](../../output/docs/rust/reference/expressions/array-expr.md)
- [Rust std: slice `len`](../../output/docs/rust/std/primitive.slice.md)
- [Rust Reference: numeric types](../../output/docs/rust/reference/types/numeric.md)

## Related wiki pages

- [Array](array.md)
- [Array index bounds](array_index_bounds.md)
- [Array length](array_len.md)
- [Array last index](array_last_index.md)
- [`usize`](usize.md)
- [Sequence](sequence.md)
- [Integer literals](integer_literals.md)
- [Bindings](bindings.md)
- [Concepts so far](concepts.md)
