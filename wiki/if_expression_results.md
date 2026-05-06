<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_if_value/src/main.rs
  - experiments/hello_if_type_error/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html
  - https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
  - https://doc.rust-lang.org/stable/reference/expressions/if-expr.html
  - https://doc.rust-lang.org/stable/reference/expressions/block-expr.html
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

- [The Rust Book: Control Flow](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html)
- [The Rust Book: Functions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)
- [Rust Reference: if expressions](https://doc.rust-lang.org/stable/reference/expressions/if-expr.html)
- [Rust Reference: block expressions](https://doc.rust-lang.org/stable/reference/expressions/block-expr.html)

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
