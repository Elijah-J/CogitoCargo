<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_if_value/src/main.rs
  - RustPlayground/experiments/hello_if_type_error/src/main.rs
  - output/docs/rust/book/ch03-05-control-flow.md
  - output/docs/rust/book/ch03-03-how-functions-work.md
  - output/docs/rust/reference/expressions/if-expr.md
  - output/docs/rust/reference/expressions/block-expr.md
topic: rust-playground/if-expression-results
---

# `if` Expression Results

An `if` expression can produce a value. When it appears on the right side of a
`let` statement, the value from the branch that runs becomes the value bound to
the name.

## Shape I have used

```rust
let message = if apples > oranges {
    "More apples"
} else {
    "Not more apples"
};
```

`message` is bound to the result of the whole `if` expression. In
`hello_if_value`, `apples > oranges` evaluates to `false`, so the `else` branch
runs and the result is `"Not more apples"`.

## Branch values

The Rust Book says blocks evaluate to the last expression in them. In
`hello_if_value`, each branch block ends with a string literal expression:

```rust
{
    "More apples"
}

{
    "Not more apples"
}
```

Those string literals do not have semicolons after them because they are the
values of their branch blocks.

## Statement ending

The semicolon after the closing brace ends the `let` statement:

```rust
let message = if apples > oranges {
    "More apples"
} else {
    "Not more apples"
};
```

The semicolon belongs to the whole `let message = ...;` statement, not to the
string literal expressions inside the branch blocks.

## Useful guardrail

The Rust Book says the possible results from each arm of an `if` must be the
same type. `hello_if_value` uses a string literal in both branches. A mismatch
between branch value types produced [E0308](compiler_error_e0308.md) in
`hello_if_type_error`.

## Corpus references

- [The Rust Book: Control Flow](../../output/docs/rust/book/ch03-05-control-flow.md)
- [The Rust Book: Functions](../../output/docs/rust/book/ch03-03-how-functions-work.md)
- [Rust Reference: if expressions](../../output/docs/rust/reference/expressions/if-expr.md)
- [Rust Reference: block expressions](../../output/docs/rust/reference/expressions/block-expr.md)

## Related wiki pages

- [`if` expressions](if_expressions.md)
- [`else` blocks](else_blocks.md)
- [Conditions](conditions.md)
- [Statements](statements.md)
- [Semicolons](semicolons.md)
- [String literals](string_literals.md)
- [`error[E0308]`](compiler_error_e0308.md)
- [`let`](let_binding.md)
- [Block scope](block_scope.md)
- [Concepts so far](concepts.md)
