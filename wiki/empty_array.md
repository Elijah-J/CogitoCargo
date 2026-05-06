<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_array_last_index_if/src/main.rs
  - RustPlayground/experiments/hello_empty_array_last_index_error/src/main.rs
  - RustPlayground/experiments/hello_empty_array_len/src/main.rs
  - RustPlayground/experiments/hello_array_len/src/main.rs
  - RustPlayground/experiments/hello_array_last_index/src/main.rs
  - output/docs/rust/reference/types/array.md
  - output/docs/rust/reference/expressions/array-expr.md
  - output/docs/rust/std/primitive.array.md
  - output/docs/rust/std/primitive.slice.md
topic: rust-playground/empty-array
---

# Empty Array

An empty array stores zero elements. In `hello_empty_array_len`, the binding
uses the full array type `[i32; 0]` so Rust knows both the element type and the
length.

## Shape I have used

```rust
fn main() {
    // The type annotation gives the empty array an element type and length.
    let numbers: [i32; 0] = [];

    let length = numbers.len();

    println!("length: {length}");
}
```

The program prints:

```console
length: 0
```

## The type says the length

The Rust Reference writes the array type as `[T; N]`, where `T` is the element
type and `N` is the number of elements. In this experiment:

```rust
[i32; 0]
```

`i32` is the element type and `0` is the length.

## The expression has no elements

The array expression is:

```rust
[]
```

The Rust Reference grammar allows an array expression with no `ArrayElements`.
Because there are no element expressions inside the brackets, this experiment
writes the type annotation on the binding.

## Length

Arrays can use slice methods, and the slice `len` method returns the number of
elements. An empty array has no elements, so `numbers.len()` returns `0`.

## Useful guardrail

An empty array has no valid indexes. `len() - 1` only works for a non-empty
array, so the last-index shape from `hello_array_last_index` does not apply to
this array.

`hello_empty_array_last_index_error` tries that shape anyway:

```rust
let last_index: usize = numbers.len() - 1;
```

In debug mode, the underflow is reported as
`attempt to subtract with overflow`.

`hello_array_last_index_if` avoids that subtraction for the empty array:

```rust
if numbers.len() > 0 {
    let last_index: usize = numbers.len() - 1;
    println!("last index: {last_index}");
} else {
    println!("empty array");
}
```

## Corpus references

- [Rust Reference: Array types](../../output/docs/rust/reference/types/array.md)
- [Rust Reference: Array expressions](../../output/docs/rust/reference/expressions/array-expr.md)
- [Rust std: array primitive](../../output/docs/rust/std/primitive.array.md)
- [Rust std: slice `len`](../../output/docs/rust/std/primitive.slice.md)

## Related wiki pages

- [Array](array.md)
- [Array length](array_len.md)
- [Array last index](array_last_index.md)
- [Empty array last index error](empty_array_last_index_error.md)
- [Array last index with `if`](array_last_index_if.md)
- [Array index bounds](array_index_bounds.md)
- [Integer overflow](integer_overflow.md)
- [Type annotations](type_annotations.md)
- [`i32`](i32.md)
- [`usize`](usize.md)
- [Concepts so far](concepts.md)
