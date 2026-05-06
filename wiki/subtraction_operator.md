<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_empty_array_last_index_error/src/main.rs
  - experiments/hello_subtract_multiply/src/main.rs
  - experiments/hello_argument_order/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
  - https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html
topic: rust-playground/subtraction-operator
---

# Subtraction Operator

`-` is the subtraction operator when it appears between two numeric operands.
It subtracts the right operand from the left operand.

## Shape I have used

```rust
let starting = 12;
let removed = 5;
let difference = starting - removed;
println!("Difference: {difference}");
```

`starting` is the left operand. `removed` is the right operand. `-` is the
operator between them.

## Result value

The Rust Book says each numeric operation expression evaluates to a single
value. In `hello_subtract_multiply`, `12 - 5` evaluates to `7`.

```console
Difference: 7
```

The `let difference = ...;` statement binds that result to `difference`.

## Argument order

The `hello_argument_order` experiment puts subtraction inside a function:

```rust
fn subtract(left: i32, right: i32) -> i32 {
    left - right
}
```

Calling the function with reversed arguments changes the result:

```console
Remaining: 7
Reversed: -7
```

The first call evaluates `10 - 3`. The second call evaluates `3 - 10`.

## Overflow

`hello_empty_array_last_index_error` subtracts from a `usize` value:

```rust
let last_index: usize = numbers.len() - 1;
```

For an empty array, `numbers.len()` is `0`. Because `usize` is unsigned,
subtracting `1` underflows. Rust reports this under integer overflow, and the
debug build panics.

## Useful guardrail

The Rust Reference also describes unary `-` for negation. The current
`hello_subtract_multiply` uses binary `-`, where the operator appears between
two operands.

## Corpus references

- [The Rust Book: Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [Rust Reference: Operator expressions](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html)

## Related wiki pages

- [Arithmetic expressions](arithmetic_expressions.md)
- [Multiplication operator](multiplication_operator.md)
- [Addition operator](addition_operator.md)
- [Division operator](division_operator.md)
- [Remainder operator](remainder_operator.md)
- [Integer overflow](integer_overflow.md)
- [Empty array last index error](empty_array_last_index_error.md)
- [`usize`](usize.md)
- [Operands](operands.md)
- [Argument order](argument_order.md)
- [Multiple function parameters](multiple_function_parameters.md)
- [Integer literals](integer_literals.md)
- [Concepts so far](concepts.md)
