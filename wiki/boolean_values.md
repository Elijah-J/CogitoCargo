<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_array_last_index_if/src/main.rs
  - RustPlayground/experiments/hello_comparison/src/main.rs
  - RustPlayground/experiments/hello_more_comparisons/src/main.rs
  - RustPlayground/experiments/hello_if/src/main.rs
  - output/docs/rust/book/ch03-02-data-types.md
  - output/docs/rust/book/ch03-05-control-flow.md
  - output/docs/rust/reference/types/boolean.md
  - output/docs/rust/rust-by-example/primitives.md
topic: rust-playground/boolean-values
---

# Boolean Values

A boolean value is either `true` or `false`. Rust names the boolean type
`bool`.

## Shape I have used

```rust
let more_apples = apples > oranges;
let same_amount = apples == oranges;
let fewer_apples = apples < oranges;
let different_amount = apples != oranges;
let at_least_as_many = apples >= oranges;
let at_most_as_many = apples <= oranges;
println!("More apples: {more_apples}");
println!("Same amount: {same_amount}");
```

The comparison expressions produce boolean values. Across `hello_comparison`
and `hello_more_comparisons`, `more_apples`, `different_amount`, and
`at_least_as_many` are `true`, while `same_amount`, `fewer_apples`, and
`at_most_as_many` are `false`.

```console
More apples: true
Same amount: false
Fewer apples: false
Different amount: true
At least as many: true
At most as many: false
```

## Type

The Rust Book says booleans have two possible values, `true` and `false`, and
that the boolean type is written as `bool`. `hello_comparison` and
`hello_more_comparisons` do not write `: bool`; Rust can infer the type from
the comparison expressions.

## Useful guardrail

`true` and `false` are values, not strings. `hello_comparison` and
`hello_more_comparisons` print them through `println!`, but the source does
not write `"true"` or `"false"`.

The `hello_if` experiment uses boolean values without binding them to names:

```rust
if apples > oranges {
    println!("More apples");
}
```

The comparison expression is the `if` condition.

`hello_array_last_index_if` uses another comparison as an `if` condition:

```rust
if numbers.len() > 0 {
    let last_index: usize = numbers.len() - 1;
    println!("last index: {last_index}");
} else {
    println!("empty array");
}
```

For an empty array, `numbers.len() > 0` evaluates to `false`.

## Corpus references

- [The Rust Book: Data Types](../../output/docs/rust/book/ch03-02-data-types.md)
- [The Rust Book: Control Flow](../../output/docs/rust/book/ch03-05-control-flow.md)
- [Rust Reference: Boolean type](../../output/docs/rust/reference/types/boolean.md)
- [Rust by Example: Primitives](../../output/docs/rust/rust-by-example/primitives.md)

## Related wiki pages

- [Literals](literals.md)
- [Comparison expressions](comparison_expressions.md)
- [`if` expressions](if_expressions.md)
- [Array last index with `if`](array_last_index_if.md)
- [Conditions](conditions.md)
- [Greater-than operator](greater_than_operator.md)
- [Equality operator](equality_operator.md)
- [Less-than operator](less_than_operator.md)
- [Not-equal operator](not_equal_operator.md)
- [Greater-than-or-equal operator](greater_than_or_equal_operator.md)
- [Less-than-or-equal operator](less_than_or_equal_operator.md)
- [Types](types.md)
- [Type inference](type_inference.md)
- [Bindings](bindings.md)
- [Concepts so far](concepts.md)
