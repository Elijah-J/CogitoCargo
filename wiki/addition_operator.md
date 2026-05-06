<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_addition/src/main.rs
  - experiments/hello_array_sum/src/main.rs
  - experiments/hello_plus_equals/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
  - https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html
topic: rust-playground/addition-operator
---

# Addition Operator

`+` is the addition operator for numeric values. In `hello_addition`, it adds
two integer bindings and produces the value that gets bound to `total`.

## Shape I have used

```rust
let apples = 3;
let oranges = 4;
let total = apples + oranges;
println!("Total: {total}");
```

`apples + oranges` is written between two expressions. The Rust Reference calls
this infix notation: the operator sits between the left side and the right
side.

## The result is a value

The Rust Book says each numeric operation expression evaluates to a single
value. In `hello_addition`, `apples + oranges` evaluates to `7`.

```console
Total: 7
```

The `let total = ...;` statement then binds that value to the name `total`.

## Adding into a running total

`hello_array_sum` uses `+` inside a reassignment:

```rust
total = total + number;
```

The right side still evaluates to one value. That new value is assigned back
to the mutable `total` binding.

## Addition and assignment

`hello_plus_equals` uses `+=`:

```rust
total += number;
```

This updates `total` by adding `number` into it. It is a compound assignment
form, so it needs a mutable binding on the left side.

## Useful guardrail

The Rust Book lists addition, subtraction, multiplication, division, and
remainder as numeric operations. `hello_addition` only covers addition with
`+`.

## Corpus references

- [The Rust Book: Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [Rust Reference: Operator expressions](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html)

## Related wiki pages

- [Arithmetic expressions](arithmetic_expressions.md)
- [Accumulator](accumulator.md)
- [Assignment](assignment.md)
- [`+=`](plus_equals.md)
- [Operands](operands.md)
- [Subtraction operator](subtraction_operator.md)
- [Multiplication operator](multiplication_operator.md)
- [Integer literals](integer_literals.md)
- [Type inference](type_inference.md)
- [Types](types.md)
- [Bindings](bindings.md)
- [`let`](let_binding.md)
- [Concepts so far](concepts.md)
