<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_empty_array_len/src/main.rs
  - experiments/hello_integer/src/main.rs
  - experiments/hello_type_annotation/src/main.rs
  - experiments/hello_addition/src/main.rs
  - experiments/hello_division/src/main.rs
  - experiments/hello_subtract_multiply/src/main.rs
  - experiments/hello_array_for/src/main.rs
  - experiments/hello_array_index/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
  - https://doc.rust-lang.org/stable/rust-by-example/types/literals.html
  - https://doc.rust-lang.org/stable/reference/tokens.html
  - https://doc.rust-lang.org/stable/reference/expressions/literal-expr.html
topic: rust-playground/integer-literals
---

# Integer Literals

An integer literal is a whole number written directly in source code. In the
`hello_integer` experiment, `3` is the integer literal.

## Shape I have used

```rust
let count = 3;
println!("Count: {count}");
```

`count` is the binding name. `3` is the integer literal bound to that name.

## In arithmetic

The `hello_addition` experiment used two integer literals and then added the
bindings:

```rust
let apples = 3;
let oranges = 4;
let total = apples + oranges;
```

`3` and `4` are integer literals. `apples + oranges` evaluates to a new
integer value.

The `hello_division` experiment used integer literals with division and
remainder:

```rust
let total = 10;
let groups = 3;
let each = total / groups;
let leftover = total % groups;
```

The `hello_subtract_multiply` experiment used integer literals before
subtraction and multiplication:

```rust
let starting = 12;
let removed = 5;
let multiplier = 3;
let difference = starting - removed;
let product = difference * multiplier;
```

## In an array

`hello_array_for` uses three integer literals inside an array expression:

```rust
let numbers = [3, 4, 5];
```

`3`, `4`, and `5` are still integer literals. The square brackets make the
surrounding expression an array value.

## As array indexes

`hello_array_index` uses integer literals to choose array positions:

```rust
let first = numbers[0];
let second = numbers[1];
```

`0` and `1` are integer literals used as indexes. In this experiment, `0`
means the first array element and `1` means the second array element.

## As an array length

`hello_empty_array_len` uses `0` inside an array type:

```rust
let numbers: [i32; 0] = [];
```

Here, `0` is the length part of the array type `[i32; 0]`.

## Type inference

The source does not say which integer type `3` has:

```rust
let count = 3;
```

Rust by Example says unsuffixed numeric literals depend on how they are used,
and the Rust Reference says an under-constrained integer literal defaults to
`i32`. `hello_integer` does not need to use `i32` directly; it only relies on
Rust choosing a type for `3`.

The `hello_type_annotation` experiment writes `i32` directly:

```rust
let count: i32 = 3;
```

The value is still the integer literal `3`. The `: i32` part is the type
annotation on the binding.

## Useful guardrail

An integer literal is different from a string literal. `3` is a number literal;
`"3"` would be a string literal because it is inside double quotes.

## Corpus references

- [The Rust Book: Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [Rust by Example: Literals](https://doc.rust-lang.org/stable/rust-by-example/types/literals.html)
- [Rust Reference: Tokens](https://doc.rust-lang.org/stable/reference/tokens.html)
- [Rust Reference: Literal expressions](https://doc.rust-lang.org/stable/reference/expressions/literal-expr.html)

## Related wiki pages

- [Literals](literals.md)
- [Types](types.md)
- [Type inference](type_inference.md)
- [Type annotations](type_annotations.md)
- [`i32`](i32.md)
- [`usize`](usize.md)
- [Addition operator](addition_operator.md)
- [Subtraction operator](subtraction_operator.md)
- [Multiplication operator](multiplication_operator.md)
- [Division operator](division_operator.md)
- [Remainder operator](remainder_operator.md)
- [Arithmetic expressions](arithmetic_expressions.md)
- [Array](array.md)
- [Empty array](empty_array.md)
- [Array indexing](array_indexing.md)
- [String literals](string_literals.md)
- [`error[E0308]`](compiler_error_e0308.md)
- [Bindings](bindings.md)
- [`let`](let_binding.md)
- [Concepts so far](concepts.md)
