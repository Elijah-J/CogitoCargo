<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_variables/src/main.rs
  - RustPlayground/experiments/hello_integer/src/main.rs
  - RustPlayground/experiments/hello_comparison/src/main.rs
  - RustPlayground/experiments/hello_array_for/src/main.rs
  - output/docs/rust/reference/expressions/literal-expr.md
  - output/docs/rust/reference/expressions/array-expr.md
  - output/docs/rust/book/ch03-02-data-types.md
  - output/docs/rust/rust-by-example/types/literals.md
topic: rust-playground/literals
---

# Literals

A literal is source code that directly writes a value. `hello_variables`,
`hello_integer`, and `hello_comparison` introduce string literals, integer
literals, and boolean values.

## Shapes I have used

```rust
let name = "Eli";
let count = 3;
let more_apples = apples > oranges;
```

`"Eli"` is a string literal. `3` is an integer literal. The comparison
expression `apples > oranges` evaluates to a boolean value; when boolean
values are written directly, they are `true` and `false`.

## Literal expressions

The Rust Reference describes a literal expression as a single token that
directly denotes the value it evaluates to, instead of referring to the value
by name or another evaluation rule.

```rust
"Eli";
3;
true;
false;
```

These are direct values in source code. A binding name is different:

```rust
let name = "Eli";
println!("Hello, {name}!");
```

`"Eli"` is the literal. `name` is the binding that refers to the value later.

## Useful guardrail

Not every expression is a literal. In `apples + oranges`, the values are
computed by the `+` operator. The integer literals are the numbers that were
bound earlier, such as `let apples = 3;` and `let oranges = 4;`.

`hello_array_for` adds another boundary:

```rust
let numbers = [3, 4, 5];
```

The `3`, `4`, and `5` are integer literals. The whole bracketed form is an
array expression, not a single-token literal expression.

## Corpus references

- [Rust Reference: literal expressions](../../output/docs/rust/reference/expressions/literal-expr.md)
- [Rust Reference: array expressions](../../output/docs/rust/reference/expressions/array-expr.md)
- [The Rust Book: Data Types](../../output/docs/rust/book/ch03-02-data-types.md)
- [Rust by Example: Literals](../../output/docs/rust/rust-by-example/types/literals.md)

## Related wiki pages

- [String literals](string_literals.md)
- [Integer literals](integer_literals.md)
- [Array](array.md)
- [Boolean values](boolean_values.md)
- [Types](types.md)
- [Type inference](type_inference.md)
- [Bindings](bindings.md)
- [Arithmetic expressions](arithmetic_expressions.md)
- [Comparison expressions](comparison_expressions.md)
- [Concepts so far](concepts.md)
