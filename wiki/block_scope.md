<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_scope/src/main.rs
  - experiments/hello_if/src/main.rs
  - https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/scope.html
  - https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html
  - https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html
  - https://doc.rust-lang.org/stable/reference/names/scopes.html
topic: rust-playground/block-scope
---

# Block Scope

Scope is the part of source code where a binding's name can be used. In
`hello_scope`, an inner `{}` block creates a smaller scope inside `fn main()`.

## Shape I have used

```rust
let name = "outer";
println!("Before block: {name}");

{
    let name = "inner";
    println!("Inside block: {name}");
}

println!("After block: {name}");
```

The outer `name` binding lives in the main function's block. The inner `name`
binding lives only inside the inner `{}` block.

## Program output

```console
Before block: outer
Inside block: inner
After block: outer
```

Inside the block, `let name = "inner";` shadows the outer `name`. After the
block ends, that inner binding is gone, so `name` refers to the outer binding
again.

## Useful guardrail

Rust by Example describes a block as statements enclosed by braces. A binding
created inside a block is constrained to that block, so code after the closing
brace cannot use that inner binding.

If code tries to use only that inner binding after the block, `cargo check`
reports [E0425](compiler_error_e0425.md).

The `hello_if` experiment uses another kind of block:

```rust
if apples > oranges {
    println!("More apples");
}
```

The braces contain the code that runs when the condition is true.

## Corpus references

- [Rust by Example: Scope and Shadowing](https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/scope.html)
- [The Rust Book: Variables and Mutability](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)
- [The Rust Book: Control Flow](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html)
- [Rust Reference: Scopes](https://doc.rust-lang.org/stable/reference/names/scopes.html)

## Related wiki pages

- [Bindings](bindings.md)
- [`if` expressions](if_expressions.md)
- [`if` expression results](if_expression_results.md)
- [Statements](statements.md)
- [Semicolons](semicolons.md)
- [Conditions](conditions.md)
- [`error[E0425]`](compiler_error_e0425.md)
- [Shadowing](shadowing.md)
- [`let`](let_binding.md)
- [`//` comments](line_comments.md)
- [`cargo check`](cargo_check.md)
- [`cargo run`](cargo_run.md)
- [Rust `main` function](main_function.md)
- [Concepts so far](concepts.md)
