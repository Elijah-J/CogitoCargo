<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_array_last_index_if/src/main.rs
  - experiments/hello_empty_array_last_index_error/src/main.rs
  - experiments/hello_empty_array_len/src/main.rs
  - experiments/hello_array_last_index/src/main.rs
  - experiments/hello_array_len/src/main.rs
  - experiments/hello_array_index/src/main.rs
  - experiments/hello_array_index_error/src/main.rs
  - https://doc.rust-lang.org/stable/reference/expressions/array-expr.html
  - https://doc.rust-lang.org/stable/std/primitive.slice.html
  - https://doc.rust-lang.org/stable/std/primitive.array.html
  - https://doc.rust-lang.org/stable/reference/types/numeric.html
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
topic: rust-playground/array-last-index
---

# Array Last Index

The last valid array index is one less than the array length when the array is
not empty. `hello_array_last_index` computes that index instead of writing
`2` directly.

## Shape I have used

```rust
fn main() {
    let numbers = [3, 4, 5];

    // This works because the array has at least one element.
    let last_index: usize = numbers.len() - 1;

    // A computed usize index can go inside [] just like a literal index.
    let last = numbers[last_index];

    println!("last index: {last_index}");
    println!("last: {last}");
}
```

The program prints:

```console
last index: 2
last: 5
```

## Why the last index is `2`

`numbers.len()` returns `3` because `[3, 4, 5]` has three elements. Array
indexes are zero-based, so the valid indexes are:

```text
0, 1, 2
```

Subtracting one from the length gives the last valid index:

```rust
let last_index: usize = numbers.len() - 1;
```

The result is `2`, so `numbers[last_index]` reads `numbers[2]`.

## Why `usize` fits

The standard-library slice `len` method returns `usize`. The Rust Reference
says array and slice indexing uses an expression of type `usize` inside the
square brackets.

That means this binding can be used directly as an index:

```rust
let last_index: usize = numbers.len() - 1;
let last = numbers[last_index];
```

## Useful guardrail

`len() - 1` is only valid when the array has at least one element. If the
length is `0`, there is no last index to compute. Empty arrays need a different
shape.

`hello_empty_array_len` makes that boundary visible:

```rust
let numbers: [i32; 0] = [];
let length = numbers.len();
```

The length is `0`, so there is no valid last index.

`hello_empty_array_last_index_error` tries to compute the last index anyway:

```rust
let last_index: usize = numbers.len() - 1;
```

This underflows and panics in debug mode with
`attempt to subtract with overflow`.

`hello_array_last_index_if` guards the same calculation:

```rust
if numbers.len() > 0 {
    let last_index: usize = numbers.len() - 1;
    println!("last index: {last_index}");
} else {
    println!("empty array");
}
```

The subtraction only runs in the branch where the array length is greater than
`0`.

## Corpus references

- [Rust Reference: Array indexing expressions](https://doc.rust-lang.org/stable/reference/expressions/array-expr.html)
- [Rust std: slice `len`](https://doc.rust-lang.org/stable/std/primitive.slice.html)
- [Rust std: array primitive](https://doc.rust-lang.org/stable/std/primitive.array.html)
- [Rust Reference: numeric types](https://doc.rust-lang.org/stable/reference/types/numeric.html)
- [The Rust Book: Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)

## Related wiki pages

- [Array](array.md)
- [Empty array](empty_array.md)
- [Empty array last index error](empty_array_last_index_error.md)
- [Array last index with `if`](array_last_index_if.md)
- [Array length](array_len.md)
- [Array indexing](array_indexing.md)
- [Array index bounds](array_index_bounds.md)
- [`usize`](usize.md)
- [Integer overflow](integer_overflow.md)
- [Subtraction operator](subtraction_operator.md)
- [Type annotations](type_annotations.md)
- [Concepts so far](concepts.md)
