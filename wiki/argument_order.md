<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_argument_order/src/main.rs
  - output/docs/rust/book/ch03-03-how-functions-work.md
  - output/docs/rust/book/ch03-02-data-types.md
  - output/docs/rust/reference/items/functions.md
  - output/docs/rust/reference/expressions/operator-expr.md
topic: rust-playground/argument-order
---

# Argument Order

Argument order is the left-to-right position of values in a function call. In
`hello_argument_order`, reversing the two arguments changes which value becomes
`left` and which value becomes `right`.

## Shape I have used

```rust
let remaining = subtract(starting, removed);
let reversed = subtract(removed, starting);

fn subtract(left: i32, right: i32) -> i32 {
    left - right
}
```

In the first call, `starting` is passed to `left`, and `removed` is passed to
`right`. In the second call, `removed` is passed to `left`, and `starting` is
passed to `right`.

## Why the output changes

The helper function subtracts the right operand from the left operand:

```rust
fn subtract(left: i32, right: i32) -> i32 {
    left - right
}
```

With `starting = 10` and `removed = 3`, the two calls produce different
results:

```console
Remaining: 7
Reversed: -7
```

The first call evaluates `10 - 3`. The second call evaluates `3 - 10`.

## Useful guardrail

Argument names at the call site do not rename the parameters. Position is what
matters here: first argument to first parameter, second argument to second
parameter.

## Corpus references

- [The Rust Book: Functions](../../output/docs/rust/book/ch03-03-how-functions-work.md)
- [The Rust Book: Data Types](../../output/docs/rust/book/ch03-02-data-types.md)
- [Rust Reference: functions](../../output/docs/rust/reference/items/functions.md)
- [Rust Reference: operator expressions](../../output/docs/rust/reference/expressions/operator-expr.md)

## Related wiki pages

- [Multiple function parameters](multiple_function_parameters.md)
- [Function calls](function_calls.md)
- [Function parameters](function_parameters.md)
- [Subtraction operator](subtraction_operator.md)
- [Operands](operands.md)
- [`i32`](i32.md)
- [Concepts so far](concepts.md)
