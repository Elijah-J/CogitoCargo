<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_empty_array_last_index_error/src/main.rs
  - RustPlayground/experiments/hello_array_last_index/src/main.rs
  - RustPlayground/experiments/hello_array_len/src/main.rs
  - output/docs/rust/std/primitive.usize.md
  - output/docs/rust/std/primitive.slice.md
  - output/docs/rust/reference/expressions/array-expr.md
  - output/docs/rust/reference/types/numeric.md
  - output/docs/rust/book/ch03-02-data-types.md
  - output/docs/rust/reference/expressions/operator-expr.md
topic: rust-playground/usize
---

# `usize`

`usize` is Rust's pointer-sized unsigned integer type. In the current
experiments, it is the type that naturally appears when asking for an array
length and then using that value as an array index.

## Shape I have used

```rust
let numbers = [3, 4, 5];

let last_index: usize = numbers.len() - 1;
let last = numbers[last_index];
```

`last_index` is a binding with an explicit `usize` type annotation.

## Used for lengths and indexes

The standard-library slice `len` method returns `usize`:

```rust
pub const fn len(&self) -> usize
```

Arrays can use slice methods, so `numbers.len()` returns `usize`.

The Rust Reference also says array and slice index expressions use an index
expression of type `usize`:

```rust
numbers[last_index]
```

That is why the value returned from `numbers.len()` is already in the right
integer type family for indexing.

## Different from `i32`

Earlier integer examples used `i32`, either by default inference or with
`let count: i32 = 3;`. The Rust Book says the main situation for `usize` is
indexing some sort of collection.

In `hello_array_last_index`, the source writes `usize` because the value is an
index, not just a small counting number.

## Useful guardrail

`usize` is unsigned, so it does not represent negative numbers. It is also not
always 64 bits: the Rust Book and Reference describe it as architecture- or
pointer-size dependent.

`hello_empty_array_last_index_error` shows the consequence for subtraction:

```rust
let last_index: usize = numbers.len() - 1;
```

When `numbers.len()` is `0`, subtracting `1` would require a negative result.
That underflows for `usize`; Rust reports the arithmetic failure as overflow
and panics in debug mode.

## Corpus references

- [Rust std: `usize`](../../output/docs/rust/std/primitive.usize.md)
- [Rust std: slice `len`](../../output/docs/rust/std/primitive.slice.md)
- [Rust Reference: Array indexing expressions](../../output/docs/rust/reference/expressions/array-expr.md)
- [Rust Reference: numeric types](../../output/docs/rust/reference/types/numeric.md)
- [The Rust Book: Data Types](../../output/docs/rust/book/ch03-02-data-types.md)
- [Rust Reference: Operator overflow](../../output/docs/rust/reference/expressions/operator-expr.md)

## Related wiki pages

- [Types](types.md)
- [Type annotations](type_annotations.md)
- [`i32`](i32.md)
- [Integer literals](integer_literals.md)
- [Array length](array_len.md)
- [Array indexing](array_indexing.md)
- [Array last index](array_last_index.md)
- [Empty array last index error](empty_array_last_index_error.md)
- [Integer overflow](integer_overflow.md)
- [Concepts so far](concepts.md)
