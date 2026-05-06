<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_comparison/src/main.rs
  - output/docs/rust/reference/expressions/operator-expr.md
  - output/docs/rust/reference/types/boolean.md
topic: rust-playground/equality-operator
---

# Equality Operator

`==` is the equality comparison operator. It evaluates to `true` when the two
operands are equal and `false` when they are not equal.

## Shape I have used

```rust
let apples = 7;
let oranges = 5;
let same_amount = apples == oranges;
println!("Same amount: {same_amount}");
```

`apples` is the left operand. `oranges` is the right operand. `==` is the
operator between them.

## Result value

In `hello_comparison`, `7 == 5` evaluates to `false`.

```console
Same amount: false
```

The result is bound to `same_amount`.

## Useful guardrail

`==` compares two operands. It is different from the `=` used in a `let`
statement, as in `let same_amount = apples == oranges;`.

## Corpus references

- [Rust Reference: Operator expressions](../../output/docs/rust/reference/expressions/operator-expr.md)
- [Rust Reference: Boolean type](../../output/docs/rust/reference/types/boolean.md)

## Related wiki pages

- [Comparison expressions](comparison_expressions.md)
- [Boolean values](boolean_values.md)
- [Greater-than operator](greater_than_operator.md)
- [Not-equal operator](not_equal_operator.md)
- [Less-than operator](less_than_operator.md)
- [Operands](operands.md)
- [`let`](let_binding.md)
- [Integer literals](integer_literals.md)
- [`else if` chains](else_if_chains.md)
- [Concepts so far](concepts.md)
