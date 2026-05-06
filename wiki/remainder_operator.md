<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_division/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
  - https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html
topic: rust-playground/remainder-operator
---

# Remainder Operator

`%` is the remainder operator. With integer operands, it gives the remainder
left after integer division.

## Shape I have used

```rust
let total = 10;
let groups = 3;
let leftover = total % groups;
println!("Leftover: {leftover}");
```

`total` is the left operand. `groups` is the right operand. `%` is the operator
between them.

## Remainder after division

The `hello_division` experiment pairs `/` and `%` with the same operands:

```rust
let each = total / groups;
let leftover = total % groups;
```

`10 / 3` evaluates to `3`, and `10 % 3` evaluates to `1`.

```console
Each: 3
Leftover: 1
```

## Useful guardrail

The Rust Reference defines remainder with truncating division.
`hello_division` only uses positive integers, so the result is the ordinary
leftover amount.

## Corpus references

- [The Rust Book: Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [Rust Reference: Operator expressions](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html)

## Related wiki pages

- [Division operator](division_operator.md)
- [Integer division](integer_division.md)
- [Operands](operands.md)
- [Arithmetic expressions](arithmetic_expressions.md)
- [Integer literals](integer_literals.md)
- [`else if` chains](else_if_chains.md)
- [Concepts so far](concepts.md)
