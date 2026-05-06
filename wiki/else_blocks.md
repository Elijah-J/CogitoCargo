<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_array_last_index_if/src/main.rs
  - experiments/hello_else/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html
  - https://doc.rust-lang.org/stable/reference/expressions/if-expr.html
  - https://doc.rust-lang.org/stable/reference/types/boolean.html
topic: rust-playground/else-blocks
---

# `else` Blocks

`else` supplies the alternative block of an `if` expression. When the `if`
condition is false, Rust skips the first block and runs the `else` block.

## Shape I have used

```rust
if apples > oranges {
    println!("More apples");
} else {
    println!("Not more apples");
}
```

`apples > oranges` is the condition. In `hello_else`, `apples` is `5` and
`oranges` is `7`, so the condition evaluates to `false`.

## Branches

A branch is one possible block an `if` expression can execute. In this shape,
the first branch is the block after the condition, and the second branch is the
`else` block.

Only one branch runs. If the condition is true, the first block runs and the
`else` block is skipped. If the condition is false, the first block is skipped
and the `else` block runs.

## Program output

`hello_else` prints one line:

```console
Not more apples
```

No `More apples` line appears because that branch did not run.

`hello_array_last_index_if` also runs the `else` block:

```rust
if numbers.len() > 0 {
    let last_index: usize = numbers.len() - 1;
    println!("last index: {last_index}");
} else {
    println!("empty array");
}
```

For an empty array, the condition is false, so the program prints:

```console
empty array
```

## Useful guardrail

Plain `else` has no condition of its own. Additional checked conditions use
`else if`, which is a separate shape.

## Corpus references

- [The Rust Book: Control Flow](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html)
- [Rust Reference: if expressions](https://doc.rust-lang.org/stable/reference/expressions/if-expr.html)
- [Rust Reference: Boolean type](https://doc.rust-lang.org/stable/reference/types/boolean.html)

## Related wiki pages

- [`if` expressions](if_expressions.md)
- [Array last index with `if`](array_last_index_if.md)
- [`else if` expressions](else_if_expressions.md)
- [Conditions](conditions.md)
- [Boolean values](boolean_values.md)
- [Comparison expressions](comparison_expressions.md)
- [Greater-than operator](greater_than_operator.md)
- [Block scope](block_scope.md)
- [Rust `println!` macro](println_macro.md)
- [Concepts so far](concepts.md)
