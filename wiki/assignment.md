<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_mutability/src/main.rs
  - experiments/hello_array_sum/src/main.rs
  - experiments/hello_plus_equals/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html
  - https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html
topic: rust-playground/assignment
---

# Assignment

Assignment changes the value stored in an existing mutable binding. In
`hello_mutability`, the assignment is `name = "Rust";`. In `hello_array_sum`,
the assignment is `total = total + number;`.

## Shape I have used

```rust
let mut name = "Eli";
name = "Rust";
```

The first line creates the binding. The second line assigns a new value to the
existing binding.

`hello_array_sum` uses the same assignment shape with an arithmetic expression
on the right side:

```rust
let mut total = 0;
total = total + number;
```

## Left and right side

In `total = total + number;`, the left side is the existing binding being
changed. The right side is the value expression used to compute the new value.

The Rust Reference says an assignment expression uses an equals sign between a
mutable assignee and an assigned value. The current examples keep that simple:
the assignee is a mutable binding name.

## Compound assignment

`hello_plus_equals` introduces this shorter form:

```rust
total += number;
```

That is compound assignment: it combines addition with assignment. The current
model can read it as "add `number` into `total`."

## Useful guardrail

Assignment is not the same thing as a `let` binding. `let mut total = 0;`
creates the binding. `total = total + number;` and `total += number;` update
the existing binding, so they do not repeat `let`.

## Corpus references

- [The Rust Book: Variables and Mutability](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)
- [Rust Reference: Assignment expressions](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html)
- [Rust Reference: Compound assignment expressions](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html)

## Related wiki pages

- [`let mut`](mutable_binding.md)
- [Bindings](bindings.md)
- [Accumulator](accumulator.md)
- [`+=`](plus_equals.md)
- [Addition operator](addition_operator.md)
- [`error[E0384]`](compiler_error_e0384.md)
- [Concepts so far](concepts.md)
