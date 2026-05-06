<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_variables/src/main.rs
  - experiments/hello_if_value/src/main.rs
  - experiments/hello_type_annotation/src/main.rs
  - experiments/hello_return/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html
  - https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
  - https://doc.rust-lang.org/stable/reference/statements.html
  - https://doc.rust-lang.org/stable/reference/expressions/block-expr.html
topic: rust-playground/statements
---

# Statements

Statements are pieces of code that perform an action and do not return a value.
In `hello_variables`, `hello_if_value`, `hello_type_annotation`, and
`hello_return`, statements appear inside the `fn main()` block and run in
order.

## Shapes I have used

```rust
let apples = 5;
let oranges = 7;
let count: i32 = 3;

let message = if apples > oranges {
    "More apples"
} else {
    "Not more apples"
};

println!("{message}");
```

These are statements in the function body. The `let` statements introduce
bindings. A `let` statement can include a type annotation, as in
`let count: i32 = 3;`. The `println!` statement prints output.

## `let` statements

The Rust Reference says a `let` statement introduces variables. In
`hello_variables` and `hello_if_value`, each `let` statement binds a name to
the value from an expression:

```rust
let apples = 5;
let message = if apples > oranges {
    "More apples"
} else {
    "Not more apples"
};
```

`5` is an expression. The whole `if ... else` form is also an expression. The
`let` statement uses the expression value to create a binding, but the `let`
statement itself does not return a value.

## Expression statements

The Rust Reference says an expression statement evaluates an expression and
ignores its result, usually to trigger the expression's effects. In
`hello_variables` and `hello_if_value`, `println!` is used this way:

```rust
println!("{message}");
```

The macro call is an expression, and the statement form is used for its effect:
printing to standard output.

## Statement or branch value

The branch string literals in the `hello_if_value` experiment are not
statements:

```rust
let message = if apples > oranges {
    "More apples"
} else {
    "Not more apples"
};
```

`"More apples"` and `"Not more apples"` are expression values. They do not end
with semicolons because they are the values produced by their branch blocks.
The semicolon after the final `}` ends the surrounding `let` statement.

## Useful guardrail

In the current model, statements do work and expressions produce values.
Statements can contain expressions, as in `let apples = 5;`, but the statement
itself is not the value.

The `hello_return` experiment adds this distinct function-body line:

```rust
return number + 1;
```

That line uses `return` to send a value back to the caller. It is different
from leaving `number + 1` as the final expression of the function body.

## Corpus references

- [The Rust Book: Hello, World!](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html)
- [The Rust Book: Functions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)
- [Rust Reference: statements](https://doc.rust-lang.org/stable/reference/statements.html)
- [Rust Reference: block expressions](https://doc.rust-lang.org/stable/reference/expressions/block-expr.html)

## Related wiki pages

- [Semicolons](semicolons.md)
- [Function return values](function_return_values.md)
- [`return`](return_keyword.md)
- [Function calls](function_calls.md)
- [Type annotations](type_annotations.md)
- [`if` expression results](if_expression_results.md)
- [`let`](let_binding.md)
- [Rust `println!` macro](println_macro.md)
- [String literals](string_literals.md)
- [Block scope](block_scope.md)
- [Rust `main` function](main_function.md)
- [Concepts so far](concepts.md)
