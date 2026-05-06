<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_array_last_index_if/src/main.rs
  - RustPlayground/experiments/hello_empty_array_last_index_error/src/main.rs
  - RustPlayground/experiments/hello_empty_array_len/src/main.rs
  - RustPlayground/experiments/hello_array_last_index/src/main.rs
  - RustPlayground/experiments/hello_array_len/src/main.rs
  - RustPlayground/experiments/hello_array_for/src/main.rs
  - RustPlayground/experiments/hello_array_index/src/main.rs
  - RustPlayground/experiments/hello_array_index_error/src/main.rs
  - RustPlayground/experiments/hello_utf8_len/src/main.rs
  - output/docs/rust/reference/expressions/array-expr.md
  - output/docs/rust/std/primitive.array.md
  - output/docs/rust/std/primitive.slice.md
  - output/docs/rust/book/ch03-02-data-types.md
topic: rust-playground/array-len
---

# Array Length

Array `.len()` returns how many elements the array stores. In
`hello_array_len`, the array has three integer elements:

```rust
let numbers = [3, 4, 5];

// Array len counts stored elements, not the highest valid index.
let length = numbers.len();

println!("length: {length}");
```

The program prints:

```console
length: 3
```

## Why the answer is `3`

The Rust standard library describes an array as a fixed-size value with shape
`[T; N]`, where `N` is the size. The standard library also says arrays can use
slice methods. The slice `len` method returns the number of elements.

For `[3, 4, 5]`, the stored elements are:

```text
3, 4, 5
```

That is three elements, so `numbers.len()` is `3`.

## Empty array length

`hello_empty_array_len` uses an empty array:

```rust
let numbers: [i32; 0] = [];

let length = numbers.len();
```

The program prints `length: 0`. The length is still the number of stored
elements; this array stores none.

## Connection to indexes

`hello_array_index_error` showed that `numbers[3]` is outside this array. The
length is `3`, but indexes start at `0`, so the valid indexes are:

```text
0, 1, 2
```

The last valid index is one less than the length.

`hello_array_last_index` computes that value:

```rust
let last_index: usize = numbers.len() - 1;
let last = numbers[last_index];
```

For `[3, 4, 5]`, `last_index` is `2` and `last` is `5`.

## Different from `&str` length

The same method name can mean the natural length for the receiver type. For an
array, `.len()` counts elements. For `&str`, `.len()` counts bytes.

That is why `[3, 4, 5].len()` is `3`, while `"café".len()` is `5`.

## Useful guardrail

Array length is the number of stored elements. It is not the last index, and it
is not the amount of memory in bytes.

For an empty array, `len()` returns `0`. Subtracting `1` from that value
underflows; Rust reports the debug-mode panic as integer overflow.

`hello_array_last_index_if` checks the length first:

```rust
if numbers.len() > 0 {
    let last_index: usize = numbers.len() - 1;
    println!("last index: {last_index}");
} else {
    println!("empty array");
}
```

## Corpus references

- [Rust std: array primitive](../../output/docs/rust/std/primitive.array.md)
- [Rust std: slice `len`](../../output/docs/rust/std/primitive.slice.md)
- [Rust Reference: Array indexing expressions](../../output/docs/rust/reference/expressions/array-expr.md)
- [The Rust Book: The Array Type](../../output/docs/rust/book/ch03-02-data-types.md)

## Related wiki pages

- [Array](array.md)
- [Empty array](empty_array.md)
- [Empty array last index error](empty_array_last_index_error.md)
- [Array last index with `if`](array_last_index_if.md)
- [Array last index](array_last_index.md)
- [Array indexing](array_indexing.md)
- [Array index bounds](array_index_bounds.md)
- [`usize`](usize.md)
- [Integer overflow](integer_overflow.md)
- [`&str` length](str_len.md)
- [Sequence](sequence.md)
- [Integer literals](integer_literals.md)
- [Concepts so far](concepts.md)
