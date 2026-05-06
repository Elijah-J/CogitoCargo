<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_array_last_index_if/src/main.rs
  - experiments/hello_if/src/main.rs
  - experiments/hello_else/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html
  - https://doc.rust-lang.org/stable/reference/expressions/if-expr.html
  - https://doc.rust-lang.org/stable/reference/types/boolean.html
topic: rust-playground/conditions
---

# Conditions

A condition is the expression an `if` checks before deciding whether to run its
block. `hello_if` and `hello_else` use comparison expressions as conditions.

## Shape I have used

```rust
if apples > oranges {
    println!("More apples");
}
```

`apples > oranges` is the condition. It evaluates to `true`, so the block runs.

```rust
if apples == oranges {
    println!("Same amount");
}
```

`apples == oranges` is also a condition. It evaluates to `false`, so the block
is skipped.

The condition is still the comparison expression when `else` is present:

```rust
if apples > oranges {
    println!("More apples");
} else {
    println!("Not more apples");
}
```

Here, `apples > oranges` is the condition. `else` provides the alternative
block; it does not add a second condition.

`hello_array_last_index_if` uses an array length comparison as a condition:

```rust
if numbers.len() > 0 {
    let last_index: usize = numbers.len() - 1;
    println!("last index: {last_index}");
} else {
    println!("empty array");
}
```

For an empty array, `numbers.len() > 0` evaluates to `false`.

## Boolean requirement

The Rust Book says Rust does not automatically convert non-boolean values to a
boolean. An `if` condition must be a `bool`. A comparison expression works as a
condition because it evaluates to a boolean value.

## Useful guardrail

The condition is not the block. In `if apples > oranges { ... }`, the condition
is `apples > oranges`; the block is the code inside `{ ... }`.

## Corpus references

- [The Rust Book: Control Flow](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html)
- [Rust Reference: if expressions](https://doc.rust-lang.org/stable/reference/expressions/if-expr.html)
- [Rust Reference: Boolean type](https://doc.rust-lang.org/stable/reference/types/boolean.html)

## Related wiki pages

- [`if` expressions](if_expressions.md)
- [`else` blocks](else_blocks.md)
- [Array last index with `if`](array_last_index_if.md)
- [`else if` expressions](else_if_expressions.md)
- [`else if` chains](else_if_chains.md)
- [Boolean values](boolean_values.md)
- [Comparison expressions](comparison_expressions.md)
- [Greater-than operator](greater_than_operator.md)
- [Equality operator](equality_operator.md)
- [Block scope](block_scope.md)
- [Concepts so far](concepts.md)
