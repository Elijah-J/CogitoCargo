<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_empty_array_last_index_error/src/main.rs
  - experiments/hello_empty_array_len/src/main.rs
  - experiments/hello_array_last_index/src/main.rs
  - experiments/hello_array_index_error/src/main.rs
  - experiments/hello_array_index/src/main.rs
  - experiments/hello_array_len/src/main.rs
  - https://doc.rust-lang.org/stable/std/primitive.array.html
  - https://doc.rust-lang.org/stable/std/primitive.slice.html
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
  - https://doc.rust-lang.org/stable/reference/expressions/array-expr.html
topic: rust-playground/array-index-bounds
---

# Array Index Bounds

Array indexes must name positions that exist in the array. In
`hello_array_index_error`, `numbers[3]` was outside the three-element array.

## Invalid shape tested

The experiment tried this first:

```rust
let numbers = [3, 4, 5];
let missing = numbers[3];
```

`cargo run` failed while compiling the program:

```console
error: this operation will panic at runtime
index out of bounds: the length is 3 but the index is 3
```

The diagnostic also said `#[deny(unconditional_panic)]` is on by default. The
program did not reach `println!`.

## Fixed shape

The fixed version reads the last valid element:

```rust
fn main() {
    let numbers = [3, 4, 5];

    let last = numbers[2];

    println!("last: {last}");
}
```

The program prints:

```console
last: 5
```

## Why index `3` is outside

The Rust Reference says array indexes are zero-based. For an array with three
elements, the valid indexes are:

```text
0, 1, 2
```

Index `3` would be the fourth position, but `[3, 4, 5]` has no fourth element.

## Length connection

`hello_array_len` makes the boundary visible with `numbers.len()`:

```rust
let numbers = [3, 4, 5];
let length = numbers.len();
```

The length is `3`. Because indexes start at `0`, the highest valid index is
`length - 1`, which is `2` for this array.

`hello_array_last_index` computes that boundary:

```rust
let last_index: usize = numbers.len() - 1;
let last = numbers[last_index];
```

The computed index is still checked against the array bounds. In this
experiment the value is `2`, so the index exists.

## Empty array boundary

`hello_empty_array_len` creates an array with length `0`:

```rust
let numbers: [i32; 0] = [];
```

An empty array has no element positions, so it has no valid indexes.

`hello_empty_array_last_index_error` fails before indexing. The expression
`numbers.len() - 1` underflows while trying to compute an index value; Rust
reports this as integer overflow.

## Useful guardrail

This page only covers the known index used in `hello_array_index_error`. The
Rust Book also shows that when the index is only known at runtime, an invalid
index can make the program panic at runtime.

## Corpus references

- [The Rust Book: Invalid Array Element Access](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [Rust std: array primitive](https://doc.rust-lang.org/stable/std/primitive.array.html)
- [Rust std: slice `len`](https://doc.rust-lang.org/stable/std/primitive.slice.html)
- [Rust Reference: Array indexing expressions](https://doc.rust-lang.org/stable/reference/expressions/array-expr.html)

## Related wiki pages

- [Array indexing](array_indexing.md)
- [Array](array.md)
- [Array length](array_len.md)
- [Array last index](array_last_index.md)
- [Empty array](empty_array.md)
- [Empty array last index error](empty_array_last_index_error.md)
- [`usize`](usize.md)
- [Integer overflow](integer_overflow.md)
- [Integer literals](integer_literals.md)
- [Concepts so far](concepts.md)
