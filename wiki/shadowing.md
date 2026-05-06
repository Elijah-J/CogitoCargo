<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_shadowing/src/main.rs
  - output/docs/rust/book/ch03-01-variables-and-mutability.md
  - output/docs/rust/rust-by-example/variable_bindings/scope.md
topic: rust-playground/shadowing
---

# Shadowing

Shadowing happens when a new `let` binding uses the same name as an earlier
binding. After the new binding appears, uses of that name refer to the newer
binding.

## Shape I have used

```rust
let name = "Eli";
println!("First: {name}");

let name = "Rust";
println!("Second: {name}");
```

The second `let name = "Rust";` creates a new binding named `name`. It does
not need `mut` because it is not assigning to the first binding.

## Program output

```console
First: Eli
Second: Rust
```

The first `println!` runs before the second binding exists, so it prints the
first value. The second `println!` runs after the new binding, so it prints the
second value.

## Difference from `mut`

`let mut name = "Eli";` creates one mutable binding, and `name = "Rust";`
assigns a new value to that binding.

`let name = "Eli";` followed by `let name = "Rust";` creates two bindings with
the same name. The newer binding shadows the older one.

## Useful guardrail

Shadowing repeats `let`. Reassignment does not. If the code leaves out the
second `let`, it is trying to assign to the existing binding instead.

## Corpus references

- [The Rust Book: Variables and Mutability](../../output/docs/rust/book/ch03-01-variables-and-mutability.md)
- [Rust by Example: Scope and Shadowing](../../output/docs/rust/rust-by-example/variable_bindings/scope.md)

## Related wiki pages

- [Bindings](bindings.md)
- [Assignment](assignment.md)
- [Block scope](block_scope.md)
- [`error[E0425]`](compiler_error_e0425.md)
- [`let`](let_binding.md)
- [`let mut`](mutable_binding.md)
- [`cargo check`](cargo_check.md)
- [`error[E0384]`](compiler_error_e0384.md)
- [`//` comments](line_comments.md)
- [Rust `println!` macro](println_macro.md)
- [Concepts so far](concepts.md)
