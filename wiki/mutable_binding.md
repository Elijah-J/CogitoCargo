<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_mutability/src/main.rs
  - RustPlayground/experiments/hello_array_sum/src/main.rs
  - RustPlayground/experiments/hello_plus_equals/src/main.rs
  - output/docs/rust/book/ch03-01-variables-and-mutability.md
topic: rust-playground/mutable-binding
---

# `let mut`

`let mut` creates a variable binding whose value can be changed later. Rust
variables are immutable by default, so the `mut` marks the binding as one that
the program is allowed to reassign.

## Shape I have used

```rust
let mut name = "Eli";
println!("Before: {name}");

name = "Rust";
println!("After: {name}");
```

`let mut name = "Eli";` creates the binding. The first `println!` reads the
initial value. `name = "Rust";` assigns a new value to that same binding, and
the second `println!` reads the changed value.

## What `cargo check` caught

Before adding `mut`, `hello_mutability` tried this shape:

```rust
let name = "Eli";
name = "Rust";
```

`cargo check` rejected it with [E0384](compiler_error_e0384.md), because
`name` was immutable and the second line tried to assign to it again. The
compiler suggested making the binding mutable by writing
`let mut name = "Eli";`.

## Program output

After adding `mut`, the program prints the new value:

```console
Before: Eli
After: Rust
```

## As a running total

`hello_array_sum` uses a mutable binding for an integer total:

```rust
let mut total = 0;

for number in numbers {
    total = total + number;
}
```

Each loop pass assigns a new value to `total`. The binding must be mutable
because the program changes it more than once.

`hello_plus_equals` uses a shorter update form:

```rust
total += number;
```

This still changes the existing `total` binding, so `total` still needs `mut`.

## Useful guardrail

`mut` belongs on the binding that will be reassigned. The reassignment line
uses the name directly; it does not repeat `let`.

## Corpus references

- [The Rust Book: Variables and Mutability](../../output/docs/rust/book/ch03-01-variables-and-mutability.md)

## Related wiki pages

- [Bindings](bindings.md)
- [`let`](let_binding.md)
- [Shadowing](shadowing.md)
- [Accumulator](accumulator.md)
- [Assignment](assignment.md)
- [`+=`](plus_equals.md)
- [`error[E0384]`](compiler_error_e0384.md)
- [`cargo check`](cargo_check.md)
- [`//` comments](line_comments.md)
- [Rust `println!` macro](println_macro.md)
- [Concepts so far](concepts.md)
