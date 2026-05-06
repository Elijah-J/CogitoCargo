<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_variables/src/main.rs
  - experiments/hello_addition/src/main.rs
  - experiments/hello_if_value/src/main.rs
  - experiments/hello_type_annotation/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html
  - https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
  - https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html
  - https://doc.rust-lang.org/stable/reference/statements.html
topic: rust-playground/let-binding
---

# `let`

`let` creates a variable binding. In the `hello_variables` experiment, it bound
the name `name` to the string `"Eli"`.

## Shape I have used

```rust
let name = "Eli";
```

`name` is the variable name. `"Eli"` is the value bound to that name. The
semicolon ends the statement.

The value being bound can also come from an expression:

```rust
let total = apples + oranges;
```

Here, `apples + oranges` evaluates to a value, and `let` binds that value to
`total`.

The value can also come from an `if` expression:

```rust
let message = if apples > oranges {
    "More apples"
} else {
    "Not more apples"
};
```

A `let` statement can also write the binding's type:

```rust
let count: i32 = 3;
```

## Using the name

The next line used that variable in `println!`:

```rust
println!("Hello, {name}!");
```

When the program runs, `{name}` is replaced with the value bound to `name`.

## Useful guardrail

The Rust Book says variables are immutable by default. In `hello_variables`,
`let name = "Eli";` creates a name for a value, but the program does not try to
change that value afterward.

To change the value later, the binding needs `mut`, as in
`let mut name = "Eli";`.

Changing an existing mutable binding is assignment, not another `let`
statement.

## Corpus references

- [The Rust Book: Variables and Mutability](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)
- [The Rust Book: Functions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)
- [The Rust Book: Control Flow](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html)
- [Rust Reference: statements](https://doc.rust-lang.org/stable/reference/statements.html)

## Related wiki pages

- [Bindings](bindings.md)
- [Assignment](assignment.md)
- [Types](types.md)
- [Type annotations](type_annotations.md)
- [Addition operator](addition_operator.md)
- [Arithmetic expressions](arithmetic_expressions.md)
- [`if` expression results](if_expression_results.md)
- [Statements](statements.md)
- [Semicolons](semicolons.md)
- [Block scope](block_scope.md)
- [Rust `println!` macro](println_macro.md)
- [Shadowing](shadowing.md)
- [`let mut`](mutable_binding.md)
- [`error[E0384]`](compiler_error_e0384.md)
- [`//` comments](line_comments.md)
- [`src/main.rs`](src_main_rs.md)
- [`cargo check`](cargo_check.md)
- [`cargo run`](cargo_run.md)
- [Concepts so far](concepts.md)
