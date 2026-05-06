<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_plus_equals/src/main.rs
  - experiments/hello_array_sum/src/main.rs
  - https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html
  - https://doc.rust-lang.org/stable/book/appendix-02-operators.html
topic: rust-playground/plus-equals
---

# `+=`

`+=` adds a value into an existing mutable binding. In `hello_plus_equals`, it
updates the running total inside a `for` loop.

## Shape I have used

```rust
fn main() {
    let numbers = [3, 4, 5];
    let mut total = 0;

    for number in numbers {
        total += number;
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

## Compared with explicit addition

`hello_array_sum` used this explicit form:

```rust
total = total + number;
```

`hello_plus_equals` uses this shorter form:

```rust
total += number;
```

For the current model, both update the existing `total` binding by adding the
current `number` into it.

## Compound assignment

The Rust Reference calls this a compound assignment expression. The operator
is written as one token with no space between `+` and `=`.

The Rust Book operator appendix describes `+=` as arithmetic addition and
assignment.

## Useful guardrail

`+=` updates an existing place. In this experiment, that place is the mutable
binding `total`. It does not create a new binding, so the line does not use
`let`.

## Corpus references

- [Rust Reference: Compound assignment expressions](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html)
- [The Rust Book: Operators appendix](https://doc.rust-lang.org/stable/book/appendix-02-operators.html)

## Related wiki pages

- [Accumulator](accumulator.md)
- [Addition operator](addition_operator.md)
- [Assignment](assignment.md)
- [Arithmetic expressions](arithmetic_expressions.md)
- [`let mut`](mutable_binding.md)
- [Bindings](bindings.md)
- [Concepts so far](concepts.md)
