<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_more_comparisons/src/main.rs
  - output/docs/rust/reference/expressions/operator-expr.md
  - output/docs/rust/reference/types/boolean.md
topic: rust-playground/greater-than-or-equal-operator
---

# Greater-Than-Or-Equal Operator

`>=` is the greater-than-or-equal comparison operator. It evaluates to `true`
when the left operand is greater than or equal to the right operand.

## Shape I have used

```rust
let apples = 7;
let oranges = 5;
let at_least_as_many = apples >= oranges;
println!("At least as many: {at_least_as_many}");
```

`apples` is the left operand. `oranges` is the right operand. `>=` is the
operator between them.

## Result value

In `hello_more_comparisons`, `7 >= 5` evaluates to `true`.

```console
At least as many: true
```

The result is bound to `at_least_as_many`.

## Useful guardrail

`>=` includes equality. The `hello_more_comparisons` result is true because
`7` is greater than `5`; an equal pair of operands would also make `>=` true.

## Corpus references

- [Rust Reference: Operator expressions](../../output/docs/rust/reference/expressions/operator-expr.md)
- [Rust Reference: Boolean type](../../output/docs/rust/reference/types/boolean.md)

## Related wiki pages

- [Comparison expressions](comparison_expressions.md)
- [Boolean values](boolean_values.md)
- [Greater-than operator](greater_than_operator.md)
- [Less-than-or-equal operator](less_than_or_equal_operator.md)
- [Operands](operands.md)
- [Integer literals](integer_literals.md)
- [Concepts so far](concepts.md)
