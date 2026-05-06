<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_division/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
  - https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html
topic: rust-playground/division-operator
---

# Division Operator

`/` is the division operator. With integer operands, Rust performs integer
division, so the result is the whole-number quotient.

## Shape I have used

```rust
let total = 10;
let groups = 3;
let each = total / groups;
println!("Each: {each}");
```

`total` is the left operand. `groups` is the right operand. `/` is the operator
between them.

## Integer result

The Rust Book says integer division truncates toward zero. In
`hello_division`, `10 / 3` evaluates to `3`.

```console
Each: 3
```

The leftover part is not included in `each`; the `%` operator gives the
remainder.

## Useful guardrail

The Rust Reference says integer division by zero panics. `hello_division` uses
`groups = 3`, so the right operand is not zero.

## Corpus references

- [The Rust Book: Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [Rust Reference: Operator expressions](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html)

## Related wiki pages

- [Integer division](integer_division.md)
- [Remainder operator](remainder_operator.md)
- [Operands](operands.md)
- [Arithmetic expressions](arithmetic_expressions.md)
- [Integer literals](integer_literals.md)
- [Concepts so far](concepts.md)
