<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_array_last_index_if/src/main.rs
  - experiments/hello_if/src/main.rs
  - experiments/hello_else/src/main.rs
  - experiments/hello_else_if/src/main.rs
  - experiments/hello_if_value/src/main.rs
  - experiments/hello_early_return/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html
  - https://doc.rust-lang.org/stable/reference/expressions/if-expr.html
  - https://doc.rust-lang.org/stable/reference/types/boolean.html
topic: rust-playground/if-expressions
---

# `if` Expressions

An `if` expression uses a condition to decide which block of code runs.
`hello_if`, `hello_else`, `hello_else_if`, and `hello_if_value` use comparison
expressions as conditions.

## Shape I have used

```rust
if apples > oranges {
    println!("More apples");
}

if apples == oranges {
    println!("Same amount");
}
```

`apples > oranges` and `apples == oranges` are the conditions. Each condition
is followed by a block in braces.

## Without `else`

The first condition is true, so its block runs:

```console
More apples
```

The second condition is false, so its block is skipped. Because `hello_if` does
not use `else`, there is no alternative block to run.

## With `else`

```rust
if apples > oranges {
    println!("More apples");
} else {
    println!("Not more apples");
}
```

When an `else` block is present, a false condition skips the first block and
runs the `else` block instead.

## With `else if`

```rust
if apples > oranges {
    println!("More apples");
} else if apples < oranges {
    println!("Fewer apples");
} else {
    println!("Same amount");
}
```

`else if` adds another condition to check before the final `else` fallback.
Rust runs the first branch whose condition is true and skips the later
branches.

## As a value

```rust
let message = if apples > oranges {
    "More apples"
} else {
    "Not more apples"
};
```

Because `if` is an expression, the branch that runs can produce a value. In
this shape, that value is bound to `message`.

## With early return

```rust
if count == 0 {
    return "none";
}
```

The `hello_early_return` experiment uses an `if` block to return from a
function early. When the condition is true, the `return` line sends a value
back to the caller before the function reaches its final expression.

## As a guard

`hello_array_last_index_if` uses `if` to guard a calculation:

```rust
if numbers.len() > 0 {
    let last_index: usize = numbers.len() - 1;
    println!("last index: {last_index}");
} else {
    println!("empty array");
}
```

The subtraction only runs when the condition is true. For an empty array, the
condition is false and the `else` block runs.

## Useful guardrail

The Rust Book says an `if` condition must be a `bool`. `hello_if` gets a
boolean from a comparison expression.

## Corpus references

- [The Rust Book: Control Flow](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html)
- [Rust Reference: if expressions](https://doc.rust-lang.org/stable/reference/expressions/if-expr.html)
- [Rust Reference: Boolean type](https://doc.rust-lang.org/stable/reference/types/boolean.html)

## Related wiki pages

- [Conditions](conditions.md)
- [`else` blocks](else_blocks.md)
- [Array last index with `if`](array_last_index_if.md)
- [`else if` expressions](else_if_expressions.md)
- [`else if` chains](else_if_chains.md)
- [`if` expression results](if_expression_results.md)
- [Early return](early_return.md)
- [Boolean values](boolean_values.md)
- [Comparison expressions](comparison_expressions.md)
- [Greater-than operator](greater_than_operator.md)
- [Equality operator](equality_operator.md)
- [Block scope](block_scope.md)
- [Rust `println!` macro](println_macro.md)
- [Concepts so far](concepts.md)
