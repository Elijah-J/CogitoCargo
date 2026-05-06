<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - output/docs/rust/book/ch03-01-variables-and-mutability.md
  - RustPlayground/experiments/hello_mutability/src/main.rs
topic: rust-playground/compiler-error-e0384
---

# `error[E0384]`

`error[E0384]: cannot assign twice to immutable variable` is the compiler error
Rust reports when code tries to assign a new value to a binding that was not
declared with `mut`.

## Shape that triggers it

```rust
let name = "Eli";
name = "Rust";
```

The first line creates an immutable binding. The second line tries to assign a
new value to that binding, so `cargo check` rejects it.

## Fix used in `hello_mutability`

```rust
let mut name = "Eli";
println!("Before: {name}");

name = "Rust";
println!("After: {name}");
```

Adding `mut` to the binding permits the later reassignment. The reassignment
line uses the existing name directly; it does not repeat `let`.

## Program output after the fix

```console
Before: Eli
After: Rust
```

## Useful guardrail

The error is about assigning again to an immutable binding. It is not about
`println!`, comments, or Cargo itself; `cargo check` is the command that
reported the compiler diagnostic during `hello_mutability`.

## Corpus references

- [The Rust Book: Variables and Mutability](../../output/docs/rust/book/ch03-01-variables-and-mutability.md)

## Related wiki pages

- [Bindings](bindings.md)
- [Assignment](assignment.md)
- [`let mut`](mutable_binding.md)
- [`let`](let_binding.md)
- [Shadowing](shadowing.md)
- [`cargo check`](cargo_check.md)
- [Rust `println!` macro](println_macro.md)
- [Concepts so far](concepts.md)
