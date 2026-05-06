<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_subtract_multiply/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
  - https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html
topic: rust-playground/multiplication-operator
---

# Multiplication Operator

`*` is the multiplication operator when it appears between two numeric
operands. It multiplies the operands and evaluates to the product.

## Shape I have used

```rust
let starting = 12;
let removed = 5;
let multiplier = 3;
let difference = starting - removed;
let product = difference * multiplier;
println!("Product: {product}");
```

`difference` is the left operand. `multiplier` is the right operand. `*` is the
operator between them.

## Result value

The Rust Book says each numeric operation expression evaluates to a single
value. In `hello_subtract_multiply`, `7 * 3` evaluates to `21`.

```console
Product: 21
```

The `let product = ...;` statement binds that result to `product`.

## Useful guardrail

This page covers `*` as a binary multiplication operator. The Rust Reference
also uses `*` for dereference expressions. `hello_subtract_multiply` has not
used dereferencing.

## Corpus references

- [The Rust Book: Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [Rust Reference: Operator expressions](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html)

## Related wiki pages

- [Arithmetic expressions](arithmetic_expressions.md)
- [Subtraction operator](subtraction_operator.md)
- [Addition operator](addition_operator.md)
- [Division operator](division_operator.md)
- [Remainder operator](remainder_operator.md)
- [Operands](operands.md)
- [Integer literals](integer_literals.md)
- [Concepts so far](concepts.md)
