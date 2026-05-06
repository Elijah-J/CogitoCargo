<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_more_comparisons/src/main.rs
  - output/docs/rust/reference/expressions/operator-expr.md
  - output/docs/rust/reference/types/boolean.md
topic: rust-playground/less-than-operator
---

# Less-Than Operator

`<` is the less-than comparison operator. It evaluates to `true` when the left
operand is less than the right operand.

## Shape I have used

```rust
let apples = 7;
let oranges = 5;
let fewer_apples = apples < oranges;
println!("Fewer apples: {fewer_apples}");
```

`apples` is the left operand. `oranges` is the right operand. `<` is the
operator between them.

## Result value

In `hello_more_comparisons`, `7 < 5` evaluates to `false`.

```console
Fewer apples: false
```

The result is bound to `fewer_apples`.

## Useful guardrail

`<` points the opposite direction from `>`. `hello_more_comparisons` uses the
same operands as `hello_comparison`, so the result flips.

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
