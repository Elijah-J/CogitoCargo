<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_else_if/src/main.rs
  - RustPlayground/experiments/hello_else_if_chain/src/main.rs
  - output/docs/rust/book/ch03-05-control-flow.md
  - output/docs/rust/reference/expressions/if-expr.md
  - output/docs/rust/reference/types/boolean.md
topic: rust-playground/else-if-expressions
---

# `else if` Expressions

`else if` adds another checked condition after an earlier `if` condition is
false. It lets an `if` expression choose among more than two branches.

## Shape I have used

```rust
if apples > oranges {
    println!("More apples");
} else if apples < oranges {
    println!("Fewer apples");
} else {
    println!("Same amount");
}
```

`apples > oranges` is the first condition. `apples < oranges` is the second
condition. The final `else` has no condition of its own.

## Branch order

Rust checks the branches in order. When it finds the first condition that
evaluates to `true`, it runs that branch and skips the later branches.

In `hello_else_if`, `apples` is `5` and `oranges` is `7`:

1. `apples > oranges` evaluates to `false`, so the first branch is skipped.
2. `apples < oranges` evaluates to `true`, so the `else if` branch runs.
3. The final `else` branch is skipped.

## Program output

`hello_else_if` prints one line:

```console
Fewer apples
```

No `Same amount` line appears because the `else if` branch was the first true
branch.

## Chain shape

The Rust Reference allows any number of `else if` conditions and blocks. The
`hello_else_if_chain` uses two `else if` branches:

```rust
if number % 5 == 0 {
    println!("Divisible by 5");
} else if number % 3 == 0 {
    println!("Divisible by 3");
} else if number % 2 == 0 {
    println!("Divisible by 2");
} else {
    println!("No small divisor");
}
```

The same branch-order rule applies: the first true branch runs, and later
branches are skipped.

## Useful guardrail

`else if` has a condition. Plain `else` does not. The plain `else` branch is the
fallback when the earlier `if` and `else if` conditions are false.

## Corpus references

- [The Rust Book: Control Flow](../../output/docs/rust/book/ch03-05-control-flow.md)
- [Rust Reference: if expressions](../../output/docs/rust/reference/expressions/if-expr.md)
- [Rust Reference: Boolean type](../../output/docs/rust/reference/types/boolean.md)

## Related wiki pages

- [`if` expressions](if_expressions.md)
- [`else if` chains](else_if_chains.md)
- [`else` blocks](else_blocks.md)
- [Conditions](conditions.md)
- [Boolean values](boolean_values.md)
- [Comparison expressions](comparison_expressions.md)
- [Less-than operator](less_than_operator.md)
- [Greater-than operator](greater_than_operator.md)
- [Block scope](block_scope.md)
- [Rust `println!` macro](println_macro.md)
- [Concepts so far](concepts.md)
