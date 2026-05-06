<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_more_comparisons/src/main.rs
  - output/docs/rust/reference/expressions/operator-expr.md
  - output/docs/rust/reference/types/boolean.md
topic: rust-playground/not-equal-operator
---

# Not-Equal Operator

`!=` is the not-equal comparison operator. It evaluates to `true` when the two
operands are not equal.

## Shape I have used

```rust
let apples = 7;
let oranges = 5;
let different_amount = apples != oranges;
println!("Different amount: {different_amount}");
```

`apples` is the left operand. `oranges` is the right operand. `!=` is the
operator between them.

## Result value

In `hello_more_comparisons`, `7 != 5` evaluates to `true`.

```console
Different amount: true
```

The result is bound to `different_amount`.

## Useful guardrail

`!=` is the opposite equality question from `==`. In the same program,
`apples == oranges` is `false` and `apples != oranges` is `true`.

## Corpus references

- [Rust Reference: Operator expressions](../../output/docs/rust/reference/expressions/operator-expr.md)
- [Rust Reference: Boolean type](../../output/docs/rust/reference/types/boolean.md)

## Related wiki pages

- [Comparison expressions](comparison_expressions.md)
- [Boolean values](boolean_values.md)
- [Equality operator](equality_operator.md)
- [Operands](operands.md)
- [Integer literals](integer_literals.md)
- [Concepts so far](concepts.md)
