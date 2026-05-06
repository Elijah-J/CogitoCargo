<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_addition/src/main.rs
  - experiments/hello_division/src/main.rs
  - experiments/hello_subtract_multiply/src/main.rs
  - experiments/hello_argument_order/src/main.rs
  - experiments/hello_comparison/src/main.rs
  - experiments/hello_more_comparisons/src/main.rs
  - https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html
topic: rust-playground/operands
---

# Operands

An operand is an expression supplied to an operator. In a binary operator
expression, one operand is on the left side of the operator and one operand is
on the right side.

## Shapes I have used

```rust
let total = apples + oranges;
let each = total / groups;
let leftover = total % groups;
let difference = starting - removed;
let product = difference * multiplier;
let more_apples = apples > oranges;
let same_amount = apples == oranges;
let fewer_apples = apples < oranges;
let different_amount = apples != oranges;
let at_least_as_many = apples >= oranges;
let at_most_as_many = apples <= oranges;
```

In `total / groups`, `total` is the left operand and `groups` is the right
operand. `/` is the operator between them.

## Same operands, different operators

The `hello_division` experiment uses the same operands twice:

```rust
let each = total / groups;
let leftover = total % groups;
```

`total / groups` evaluates to the whole-number quotient. `total % groups`
evaluates to the remainder.

Subtraction and multiplication use the same operand pattern:

```rust
let difference = starting - removed;
let product = difference * multiplier;
```

`starting - removed` subtracts the right operand from the left operand.
`difference * multiplier` multiplies the two operands.

`hello_argument_order` uses parameter names as operands:

```rust
fn subtract(left: i32, right: i32) -> i32 {
    left - right
}
```

`left` is the left operand. `right` is the right operand.

Comparison expressions also use operands:

```rust
let more_apples = apples > oranges;
let same_amount = apples == oranges;
let fewer_apples = apples < oranges;
let different_amount = apples != oranges;
let at_least_as_many = apples >= oranges;
let at_most_as_many = apples <= oranges;
```

`apples` is the left operand in each expression. `oranges` is the right
operand.

## Useful guardrail

The operator is not an operand. In `apples + oranges`, `apples` and `oranges`
are operands, and `+` is the operator.

## Corpus references

- [Rust Reference: Operator expressions](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html)

## Related wiki pages

- [Arithmetic expressions](arithmetic_expressions.md)
- [Addition operator](addition_operator.md)
- [Subtraction operator](subtraction_operator.md)
- [Argument order](argument_order.md)
- [Multiplication operator](multiplication_operator.md)
- [Division operator](division_operator.md)
- [Remainder operator](remainder_operator.md)
- [Integer division](integer_division.md)
- [Comparison expressions](comparison_expressions.md)
- [Less-than operator](less_than_operator.md)
- [Not-equal operator](not_equal_operator.md)
- [Greater-than-or-equal operator](greater_than_or_equal_operator.md)
- [Less-than-or-equal operator](less_than_or_equal_operator.md)
- [Bindings](bindings.md)
- [Concepts so far](concepts.md)
