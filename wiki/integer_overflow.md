<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_array_last_index_if/src/main.rs
  - experiments/hello_empty_array_last_index_error/src/main.rs
  - experiments/hello_empty_array_len/src/main.rs
  - https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html
  - https://doc.rust-lang.org/stable/reference/behavior-not-considered-unsafe.html
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
  - https://doc.rust-lang.org/stable/std/primitive.usize.html
topic: rust-playground/integer-overflow
---

# Integer Overflow

Integer overflow is Rust's umbrella term for an integer operation that tries to
produce a value outside the range of the integer type. In
`hello_empty_array_last_index_error`, `0usize - 1` is the underflow case: it
goes below the minimum value because `usize` cannot represent negative values.

## Shape I have used

```rust
let numbers: [i32; 0] = [];
let last_index: usize = numbers.len() - 1;
```

The program fails in debug mode with:

```console
attempt to subtract with overflow
```

## Why `0 - 1` underflows for `usize`

`numbers.len()` returns a `usize`. The `usize` type is unsigned, so its minimum
value is `0`.

For an empty array, `numbers.len()` returns `0`. Subtracting `1` would require
the result `-1`, but that value is below the minimum value for `usize`. Rust's
panic message still says `overflow` because the language groups out-of-range
integer arithmetic under overflow.

## Debug-mode behavior

The Rust Reference says integer operators panic when they overflow in debug
mode. The Rust Book says debug builds include overflow checks that cause a
runtime panic when overflow occurs.

That is why `cargo check` accepts `hello_empty_array_last_index_error`, while
`cargo run` fails when the subtraction executes.

## Useful guardrail

Overflow is an arithmetic problem, not an array problem by itself. In
`hello_empty_array_last_index_error`, the program fails before any array index
expression runs.

The standard library has checked arithmetic methods such as `checked_sub`,
which returns `None` when subtraction would overflow. The current experiment
only records the failure; checked arithmetic is a later repair shape.

`hello_array_last_index_if` repairs the empty-array case with a branch:

```rust
if numbers.len() > 0 {
    let last_index: usize = numbers.len() - 1;
    println!("last index: {last_index}");
} else {
    println!("empty array");
}
```

For an empty array, the subtraction does not run.

## Corpus references

- [Rust Reference: Operator overflow](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html)
- [Rust Reference: Integer overflow](https://doc.rust-lang.org/stable/reference/behavior-not-considered-unsafe.html)
- [The Rust Book: Integer Overflow](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [Rust std: `usize::checked_sub`](https://doc.rust-lang.org/stable/std/primitive.usize.html)

## Related wiki pages

- [Empty array last index error](empty_array_last_index_error.md)
- [Array last index with `if`](array_last_index_if.md)
- [`usize`](usize.md)
- [Subtraction operator](subtraction_operator.md)
- [Empty array](empty_array.md)
- [Array last index](array_last_index.md)
- [Concepts so far](concepts.md)
