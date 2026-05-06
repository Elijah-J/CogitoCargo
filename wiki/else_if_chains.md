<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_else_if_chain/src/main.rs
  - output/docs/rust/book/ch03-05-control-flow.md
  - output/docs/rust/reference/expressions/if-expr.md
topic: rust-playground/else-if-chains
---

# `else if` Chains

An `else if` chain is an `if` expression with more than one `else if` branch.
Rust checks the chain in order and runs the first branch whose condition is
true.

## Shape I have used

```rust
if number % 5 == 0 {
    println!("Divisible by 5");
} else if number % 3 == 0 {
    println!("Divisible by 3");
} else if number % 2 == 0 {
    println!("Divisible by 2");
} else {
    println!("No small divisor");
}
```

The first branch starts with `if`. The next two branches start with `else if`.
The final `else` is the fallback branch.

## First true branch

In `hello_else_if_chain`, `number` is `6`.

1. `number % 5 == 0` evaluates to `false`, so Rust skips the first branch.
2. `number % 3 == 0` evaluates to `true`, so Rust runs that branch.
3. The later `number % 2 == 0` branch is skipped, even though it would also be
   true.
4. The final `else` branch is skipped.

The Rust Book describes this behavior as executing the first body whose
condition evaluates to `true` and not checking the rest after that.

## Program output

`hello_else_if_chain` prints one line:

```console
Divisible by 3
```

No `Divisible by 2` line appears because the earlier `% 3` branch was already
the first true branch.

## Useful guardrail

Order matters in an `else if` chain. If more than one condition can be true,
the earlier true branch wins.

## Corpus references

- [The Rust Book: Control Flow](../../output/docs/rust/book/ch03-05-control-flow.md)
- [Rust Reference: if expressions](../../output/docs/rust/reference/expressions/if-expr.md)

## Related wiki pages

- [`else if` expressions](else_if_expressions.md)
- [`if` expressions](if_expressions.md)
- [`else` blocks](else_blocks.md)
- [Conditions](conditions.md)
- [Remainder operator](remainder_operator.md)
- [Equality operator](equality_operator.md)
- [Boolean values](boolean_values.md)
- [Concepts so far](concepts.md)
