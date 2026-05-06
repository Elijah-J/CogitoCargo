<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_if_value/src/main.rs
  - RustPlayground/experiments/hello_function/src/main.rs
  - RustPlayground/experiments/hello_return/src/main.rs
  - output/docs/rust/book/ch01-02-hello-world.md
  - output/docs/rust/book/ch03-03-how-functions-work.md
  - output/docs/rust/reference/expressions/block-expr.md
  - output/docs/rust/reference/statements.md
topic: rust-playground/semicolons
---

# Semicolons

A semicolon usually marks the end of a Rust statement. In `hello_if_value`,
`hello_function`, and `hello_return`, semicolons end `let` statements and
`println!` statements.

## Statement endings

```rust
let apples = 5;
let oranges = 7;
println!("{message}");
```

Each semicolon ends a statement. The Rust Book introduces this with
`println!("Hello, world!");`, where the semicolon indicates that the expression
is over and the next one can begin.

## Expression values

The Rust Book also uses semicolons to distinguish expressions from statements:
adding a semicolon to the end of an expression turns it into a statement, and
then it does not return a value.

That matters in the `hello_if_value` experiment:

```rust
let message = if apples > oranges {
    "More apples"
} else {
    "Not more apples"
};
```

The string literals inside the branch blocks do not end with semicolons. They
are the values produced by those blocks. The semicolon after the final `}` ends
the whole `let message = ...;` statement.

The same rule appears in `hello_function`:

```rust
fn add_one(number: i32) -> i32 {
    number + 1
}
```

`number + 1` does not end with a semicolon because it is the value returned by
the function.

`hello_return` uses a different shape:

```rust
fn add_one(number: i32) -> i32 {
    return number + 1;
}
```

Here the line ends with a semicolon. The `return` keyword is what sends the
value back to the caller.

## Extra semicolons

The Rust Reference says extra semicolons between statements are allowed and do
not affect semantics. `hello_if_value`, `hello_function`, and `hello_return`
still use one semicolon where a statement needs to end, because that shape
keeps the source easy to read.

## Useful guardrail

In the current model, use semicolons after ordinary statements like `let ...;`
and `println!(...);`. Leave the final value expression inside a value-producing
block without a semicolon.

## Corpus references

- [The Rust Book: Hello, World!](../../output/docs/rust/book/ch01-02-hello-world.md)
- [The Rust Book: Functions](../../output/docs/rust/book/ch03-03-how-functions-work.md)
- [Rust Reference: block expressions](../../output/docs/rust/reference/expressions/block-expr.md)
- [Rust Reference: statements](../../output/docs/rust/reference/statements.md)

## Related wiki pages

- [Statements](statements.md)
- [Function return values](function_return_values.md)
- [Functions](functions.md)
- [`return`](return_keyword.md)
- [`if` expression results](if_expression_results.md)
- [`if` expressions](if_expressions.md)
- [`let`](let_binding.md)
- [Rust `println!` macro](println_macro.md)
- [String literals](string_literals.md)
- [Block scope](block_scope.md)
- [Rust `main` function](main_function.md)
- [Concepts so far](concepts.md)
