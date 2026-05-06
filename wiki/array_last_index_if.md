<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_array_last_index_if/src/main.rs
  - RustPlayground/experiments/hello_empty_array_last_index_error/src/main.rs
  - RustPlayground/experiments/hello_empty_array_len/src/main.rs
  - RustPlayground/experiments/hello_array_last_index/src/main.rs
  - output/docs/rust/reference/expressions/if-expr.md
  - output/docs/rust/reference/expressions/operator-expr.md
  - output/docs/rust/std/primitive.slice.md
  - output/docs/rust/reference/expressions/array-expr.md
topic: rust-playground/array-last-index-if
---

# Array Last Index With `if`

An `if` expression can guard the arithmetic needed to compute a last array
index. In `hello_array_last_index_if`, the code checks the length before
subtracting `1`.

## Shape I have used

```rust
fn main() {
    let numbers: [i32; 0] = [];

    if numbers.len() > 0 {
        // This branch only runs when subtracting 1 from len will not underflow.
        let last_index: usize = numbers.len() - 1;
        println!("last index: {last_index}");
    } else {
        println!("empty array");
    }
}
```

The program prints:

```console
empty array
```

## The condition

The condition is:

```rust
numbers.len() > 0
```

The Rust Reference says an `if` condition must be a boolean expression. The
greater-than comparison returns a boolean value. For the empty array,
`numbers.len()` is `0`, so `0 > 0` is false.

## The protected branch

The subtraction is inside the first branch:

```rust
let last_index: usize = numbers.len() - 1;
```

That branch only runs when the length is greater than `0`. Under that
condition, subtracting `1` from the length does not underflow.

## The empty branch

When the condition is false, Rust runs the `else` block:

```rust
println!("empty array");
```

For `let numbers: [i32; 0] = [];`, this is the branch that runs.

## Useful guardrail

This repair avoids computing a last index for an empty array. It does not yet
read the last element in the non-empty branch; that is a later experiment.

## Corpus references

- [Rust Reference: if expressions](../../output/docs/rust/reference/expressions/if-expr.md)
- [Rust Reference: comparison operators](../../output/docs/rust/reference/expressions/operator-expr.md)
- [Rust std: slice `len`](../../output/docs/rust/std/primitive.slice.md)
- [Rust Reference: Array indexing expressions](../../output/docs/rust/reference/expressions/array-expr.md)

## Related wiki pages

- [Empty array last index error](empty_array_last_index_error.md)
- [Array last index](array_last_index.md)
- [Empty array](empty_array.md)
- [`if` expressions](if_expressions.md)
- [Conditions](conditions.md)
- [Greater-than operator](greater_than_operator.md)
- [`else` blocks](else_blocks.md)
- [`usize`](usize.md)
- [Integer overflow](integer_overflow.md)
- [Concepts so far](concepts.md)
