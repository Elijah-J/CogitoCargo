<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_array_sum/src/main.rs
  - RustPlayground/experiments/hello_plus_equals/src/main.rs
  - RustPlayground/experiments/hello_array_for/src/main.rs
  - RustPlayground/experiments/hello_mutability/src/main.rs
  - RustPlayground/experiments/hello_addition/src/main.rs
  - output/docs/rust/book/ch03-01-variables-and-mutability.md
  - output/docs/rust/book/ch03-02-data-types.md
  - output/docs/rust/book/ch03-05-control-flow.md
  - output/docs/rust/reference/expressions/operator-expr.md
  - output/docs/rust/reference/expressions/loop-expr.md
topic: rust-playground/accumulator
---

# Accumulator

An accumulator is a binding that keeps a result while repeated work happens.
In `hello_array_sum`, `total` keeps a running sum while a `for` loop walks
through an array. `hello_plus_equals` keeps the same accumulator but updates it
with `+=`.

## Shape I have used

```rust
fn main() {
    let numbers = [3, 4, 5];
    let mut total = 0;

    for number in numbers {
        total = total + number;
        println!("running total: {total}");
    }

    println!("final total: {total}");
}
```

The program prints:

```console
running total: 3
running total: 7
running total: 12
final total: 12
```

## Starting value

```rust
let mut total = 0;
```

`total` starts at `0` before the loop. It uses `mut` because the loop will
assign a new value to the same binding.

## Updating inside the loop

```rust
total = total + number;
```

The right side reads the current `total` and the current array element. The
`+` expression produces the next total, and the assignment stores that value
back in `total`.

## Updating with `+=`

`hello_plus_equals` uses compound assignment:

```rust
total += number;
```

For this experiment, read that as "add `number` into `total`." The running
totals stay the same as the explicit `total = total + number;` version.

## After the loop

`total` is created before the loop, so it is still available after the loop
finishes. In `hello_array_sum`, the final print uses the sum of all array
elements.

## Useful guardrail

The loop variable and the accumulator are different bindings. `number` is one
array element for the current pass. `total` is the running value that survives
from one pass to the next.

`+=` still updates an existing mutable binding. It does not create a new
binding.

## Corpus references

- [The Rust Book: Variables and Mutability](../../output/docs/rust/book/ch03-01-variables-and-mutability.md)
- [The Rust Book: Data Types](../../output/docs/rust/book/ch03-02-data-types.md)
- [The Rust Book: Control Flow](../../output/docs/rust/book/ch03-05-control-flow.md)
- [Rust Reference: Operator expressions](../../output/docs/rust/reference/expressions/operator-expr.md)
- [Rust Reference: `for` loops](../../output/docs/rust/reference/expressions/loop-expr.md)

## Related wiki pages

- [Array](array.md)
- [`for` loops](for_loops.md)
- [`let mut`](mutable_binding.md)
- [Assignment](assignment.md)
- [Addition operator](addition_operator.md)
- [`+=`](plus_equals.md)
- [Arithmetic expressions](arithmetic_expressions.md)
- [Bindings](bindings.md)
- [Concepts so far](concepts.md)
