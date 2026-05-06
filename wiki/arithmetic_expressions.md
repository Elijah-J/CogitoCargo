<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_addition/src/main.rs
  - RustPlayground/experiments/hello_division/src/main.rs
  - RustPlayground/experiments/hello_subtract_multiply/src/main.rs
  - RustPlayground/experiments/hello_array_sum/src/main.rs
  - RustPlayground/experiments/hello_plus_equals/src/main.rs
  - output/docs/rust/book/ch03-02-data-types.md
  - output/docs/rust/book/ch03-03-how-functions-work.md
  - output/docs/rust/reference/expressions/operator-expr.md
topic: rust-playground/arithmetic-expressions
---

# Arithmetic Expressions

An arithmetic expression uses a mathematical operator and evaluates to a value.
`hello_addition`, `hello_division`, and `hello_subtract_multiply` use `+`,
`-`, `*`, `/`, and `%` in arithmetic expressions.

## Shapes I have used

```rust
let apples = 3;
let oranges = 4;
let total = apples + oranges;
```

```rust
let total = 10;
let groups = 3;
let each = total / groups;
let leftover = total % groups;
```

```rust
let difference = starting - removed;
let product = difference * multiplier;
```

```rust
total = total + number;
```

Each arithmetic expression has an operator between two operands. In
`total / groups`, `total` is the left operand, `/` is the operator, and
`groups` is the right operand.

## Binding the result

The expression evaluates first, and the result becomes the value bound by
`let`:

```rust
let total = apples + oranges;
let each = total / groups;
let leftover = total % groups;
let difference = starting - removed;
let product = difference * multiplier;
```

In `hello_subtract_multiply`, those values are printed as:

```console
Difference: 7
Product: 21
```

## Reassigning the result

`hello_array_sum` uses an arithmetic expression on the right side of an
assignment:

```rust
total = total + number;
```

The expression `total + number` evaluates first. The result is then assigned
back to the mutable `total` binding.

## Compound assignment boundary

`hello_plus_equals` changes the update line to:

```rust
total += number;
```

This is not written as a separate `total + number` expression. It is compound
assignment: addition and assignment in one operator.

## Useful guardrail

This page now covers arithmetic expressions used inside a `let` statement and
on the right side of a reassignment. The Rust Book also introduces expressions
more broadly, including function calls, macro calls, and blocks.

## Corpus references

- [The Rust Book: Data Types](../../output/docs/rust/book/ch03-02-data-types.md)
- [The Rust Book: Functions](../../output/docs/rust/book/ch03-03-how-functions-work.md)
- [Rust Reference: Operator expressions](../../output/docs/rust/reference/expressions/operator-expr.md)

## Related wiki pages

- [Addition operator](addition_operator.md)
- [Accumulator](accumulator.md)
- [Assignment](assignment.md)
- [`+=`](plus_equals.md)
- [Subtraction operator](subtraction_operator.md)
- [Multiplication operator](multiplication_operator.md)
- [Division operator](division_operator.md)
- [Remainder operator](remainder_operator.md)
- [Operands](operands.md)
- [Integer division](integer_division.md)
- [Integer literals](integer_literals.md)
- [Type inference](type_inference.md)
- [Types](types.md)
- [Bindings](bindings.md)
- [`let`](let_binding.md)
- [Concepts so far](concepts.md)
