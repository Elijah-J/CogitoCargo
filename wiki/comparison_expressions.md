<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_array_last_index_if/src/main.rs
  - RustPlayground/experiments/hello_comparison/src/main.rs
  - RustPlayground/experiments/hello_more_comparisons/src/main.rs
  - RustPlayground/experiments/hello_if/src/main.rs
  - output/docs/rust/book/ch03-05-control-flow.md
  - output/docs/rust/reference/expressions/operator-expr.md
  - output/docs/rust/reference/types/boolean.md
topic: rust-playground/comparison-expressions
---

# Comparison Expressions

A comparison expression compares two operands and evaluates to a boolean value.
`hello_comparison` and `hello_more_comparisons` use `>`, `==`, `<`, `!=`,
`>=`, and `<=`.

## Shapes I have used

```rust
let apples = 7;
let oranges = 5;
let more_apples = apples > oranges;
let same_amount = apples == oranges;
let fewer_apples = apples < oranges;
let different_amount = apples != oranges;
let at_least_as_many = apples >= oranges;
let at_most_as_many = apples <= oranges;
```

`apples` is the left operand. `oranges` is the right operand. The comparison
operator sits between them.

## Result values

These comparisons produce boolean results:

```console
More apples: true
Same amount: false
Fewer apples: false
Different amount: true
At least as many: true
At most as many: false
```

`apples > oranges` is true because `7` is greater than `5`. `apples == oranges`
is false because `7` and `5` are not equal. The other comparison operators
follow the same two-operand shape and also produce boolean values.

## As an `if` condition

The `hello_if` experiment uses comparison expressions directly as conditions:

```rust
if apples > oranges {
    println!("More apples");
}

if apples == oranges {
    println!("Same amount");
}
```

`apples > oranges` is true, so its block runs. `apples == oranges` is false,
so its block is skipped.

`hello_array_last_index_if` uses a comparison expression to protect a
calculation:

```rust
if numbers.len() > 0 {
    let last_index: usize = numbers.len() - 1;
    println!("last index: {last_index}");
} else {
    println!("empty array");
}
```

`numbers.len() > 0` produces the boolean value used by `if`.

## Useful guardrail

The Rust Reference says chained comparisons need parentheses.
`hello_comparison` and `hello_more_comparisons` use one comparison operator at
a time.

## Corpus references

- [Rust Reference: Operator expressions](../../output/docs/rust/reference/expressions/operator-expr.md)
- [The Rust Book: Control Flow](../../output/docs/rust/book/ch03-05-control-flow.md)
- [Rust Reference: Boolean type](../../output/docs/rust/reference/types/boolean.md)

## Related wiki pages

- [Boolean values](boolean_values.md)
- [`if` expressions](if_expressions.md)
- [Array last index with `if`](array_last_index_if.md)
- [Conditions](conditions.md)
- [Greater-than operator](greater_than_operator.md)
- [Equality operator](equality_operator.md)
- [Less-than operator](less_than_operator.md)
- [Not-equal operator](not_equal_operator.md)
- [Greater-than-or-equal operator](greater_than_or_equal_operator.md)
- [Less-than-or-equal operator](less_than_or_equal_operator.md)
- [Operands](operands.md)
- [Types](types.md)
- [Type inference](type_inference.md)
- [Concepts so far](concepts.md)
