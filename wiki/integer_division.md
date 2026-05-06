<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_division/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
  - https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html
topic: rust-playground/integer-division
---

# Integer Division

Integer division is division where the operands are integer values. Rust keeps
the whole-number quotient and drops the fractional part by truncating toward
zero.

## Shape I have used

```rust
let total = 10;
let groups = 3;
let each = total / groups;
```

`total / groups` evaluates to `3`, not `3.333...`, because `total` and
`groups` are integer values.

## Paired with remainder

The remainder can be calculated separately with `%`:

```rust
let each = total / groups;
let leftover = total % groups;
```

For `10` split into groups of `3`, the quotient is `3` and the remainder is
`1`.

## Useful guardrail

The Rust Book also shows `-5 / 3` evaluating to `-1`; that is the "toward zero"
part of truncation. `hello_division` only uses positive integers.

## Corpus references

- [The Rust Book: Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [Rust Reference: Operator expressions](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html)

## Related wiki pages

- [Division operator](division_operator.md)
- [Remainder operator](remainder_operator.md)
- [Operands](operands.md)
- [Arithmetic expressions](arithmetic_expressions.md)
- [Integer literals](integer_literals.md)
- [Concepts so far](concepts.md)
