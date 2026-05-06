<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_if_type_error/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html
  - https://doc.rust-lang.org/stable/reference/expressions/if-expr.html
topic: rust-playground/compiler-error-e0308
---

# `error[E0308]`

`error[E0308]: if and else have incompatible types` is the compiler error
`hello_if_type_error` produced when one branch returned a string literal and
the other branch returned an integer literal.

## Shape that triggered it

```rust
let message = if apples > oranges {
    "More apples"
} else {
    0
};
```

The `if` branch evaluates to `"More apples"`, a string literal. The `else`
branch evaluates to `0`, an integer literal. Rust rejected the expression
because the whole `if` expression needs one type.

## Compiler output

`cargo check` reported:

```console
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:5:64
  |
5 |     let message = if apples > oranges { "More apples" } else { 0 };
  |                                         -------------          ^ expected `&str`, found integer
  |                                         |
  |                                         expected because of this
```

The first branch established the expected type. The integer in the `else`
branch did not match it.

## Fix used in `hello_if_type_error`

```rust
let message = if apples > oranges {
    "More apples"
} else {
    "Not more apples"
};
```

The fixed version makes both branch values string literals. The branch that
runs still chooses the value, but both possible values have the same type.

## Program output after the fix

```console
Not more apples
```

## Useful guardrail

The Rust Book says values that might be results from each arm of an `if` must
be the same type. The Rust Reference states the same rule as: an `if`
expression must have the same type in all situations.

## Corpus references

- [The Rust Book: Control Flow](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html)
- [Rust Reference: if expressions](https://doc.rust-lang.org/stable/reference/expressions/if-expr.html)

## Related wiki pages

- [`if` expression results](if_expression_results.md)
- [Types](types.md)
- [`&str`](str.md)
- [String literals](string_literals.md)
- [Integer literals](integer_literals.md)
- [`cargo check`](cargo_check.md)
- [Concepts so far](concepts.md)
