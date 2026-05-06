<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_array_last_index_if/src/main.rs
  - experiments/hello_comparison/src/main.rs
  - https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html
  - https://doc.rust-lang.org/stable/reference/types/boolean.html
topic: rust-playground/greater-than-operator
---

# Greater-Than Operator

`>` is the greater-than comparison operator. It compares the left operand with
the right operand.

## Shape I have used

```rust
let apples = 7;
let oranges = 5;
let more_apples = apples > oranges;
println!("More apples: {more_apples}");
```

`apples` is the left operand. `oranges` is the right operand. `>` is the
operator between them.

## Result value

In `hello_comparison`, `7 > 5` evaluates to `true`.

```console
More apples: true
```

The result is bound to `more_apples`.

`hello_array_last_index_if` uses `>` as an `if` condition:

```rust
if numbers.len() > 0 {
    let last_index: usize = numbers.len() - 1;
    println!("last index: {last_index}");
}
```

For an empty array, `numbers.len()` is `0`, so `numbers.len() > 0` evaluates
to `false`.

## Useful guardrail

`>` is not the same as `>=`. `hello_comparison` only asks whether the left
operand is greater than the right operand.

## Corpus references

- [Rust Reference: Operator expressions](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html)
- [Rust Reference: Boolean type](https://doc.rust-lang.org/stable/reference/types/boolean.html)

## Related wiki pages

- [Comparison expressions](comparison_expressions.md)
- [Boolean values](boolean_values.md)
- [Array last index with `if`](array_last_index_if.md)
- [Equality operator](equality_operator.md)
- [Greater-than-or-equal operator](greater_than_or_equal_operator.md)
- [Less-than operator](less_than_operator.md)
- [Operands](operands.md)
- [Integer literals](integer_literals.md)
- [Concepts so far](concepts.md)
