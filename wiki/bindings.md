<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - https://doc.rust-lang.org/stable/rust-by-example/variable_bindings.html
  - https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/mut.html
  - https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/scope.html
  - https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html
topic: rust-playground/bindings
---

# Bindings

A binding is the connection Rust creates between a name and a value. In
`hello_variables`, `let name = "Eli";` creates a binding named `name` for the
value `"Eli"`.

## Shape I have used

```rust
let name = "Eli";
println!("Hello, {name}!");
```

`name` is the name introduced by the binding. `"Eli"` is the value bound to
that name. Later code can use `name`, as in the `println!` format string.

## Immutable by default

Rust bindings are immutable by default. Once `let name = "Eli";` binds the
value to `name`, assigning a new value with `name = "Rust";` is rejected unless
the binding was declared with `mut`.

```rust
let mut name = "Eli";
name = "Rust";
```

`mut` changes what the binding permits: the same name can be assigned a new
value later.

## Shadowing creates a new binding

Repeating `let` with the same name creates a new binding:

```rust
let name = "Eli";
let name = "Rust";
```

The second binding shadows the first. This is different from `name = "Rust";`,
which tries to assign to an existing binding.

## Scope

Rust by Example says variable bindings have a scope and are constrained to a
block. In `hello_variables`, `hello_mutability`, `hello_shadowing`, and
`hello_scope`, the bindings all live inside the block of `fn main()`.

## Corpus references

- [Rust by Example: Variable Bindings](https://doc.rust-lang.org/stable/rust-by-example/variable_bindings.html)
- [Rust by Example: Mutability](https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/mut.html)
- [Rust by Example: Scope and Shadowing](https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/scope.html)
- [The Rust Book: Variables and Mutability](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)

## Related wiki pages

- [`let`](let_binding.md)
- [Assignment](assignment.md)
- [Types](types.md)
- [Integer literals](integer_literals.md)
- [Boolean values](boolean_values.md)
- [Comparison expressions](comparison_expressions.md)
- [Addition operator](addition_operator.md)
- [Arithmetic expressions](arithmetic_expressions.md)
- [String literals](string_literals.md)
- [Block scope](block_scope.md)
- [`error[E0425]`](compiler_error_e0425.md)
- [`let mut`](mutable_binding.md)
- [Shadowing](shadowing.md)
- [`error[E0384]`](compiler_error_e0384.md)
- [Rust `println!` macro](println_macro.md)
- [Concepts so far](concepts.md)
