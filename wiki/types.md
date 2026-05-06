<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_empty_array_len/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
  - https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html
  - https://doc.rust-lang.org/stable/book/ch04-03-slices.html
  - https://doc.rust-lang.org/stable/reference/types/boolean.html
  - https://doc.rust-lang.org/stable/reference/expressions/if-expr.html
  - https://doc.rust-lang.org/stable/reference/types/numeric.html
  - https://doc.rust-lang.org/stable/reference/types/str.html
  - https://doc.rust-lang.org/stable/rust-by-example/variable_bindings.html
  - experiments/hello_integer/src/main.rs
  - experiments/hello_addition/src/main.rs
  - experiments/hello_comparison/src/main.rs
  - experiments/hello_if_type_error/src/main.rs
  - experiments/hello_type_annotation/src/main.rs
  - experiments/hello_str_binding/src/main.rs
  - experiments/hello_static_str_return/src/main.rs
  - experiments/hello_array_for/src/main.rs
  - experiments/hello_array_last_index/src/main.rs
  - https://doc.rust-lang.org/stable/reference/types/array.html
  - https://doc.rust-lang.org/stable/std/primitive.slice.html
  - https://doc.rust-lang.org/stable/reference/expressions/array-expr.html
topic: rust-playground/types
---

# Types

A type tells Rust what kind of data a value is. Rust must know the types of all
variables at compile time, but the compiler can often infer the type from the
value and how the value is used.

## Shapes I have used

```rust
let name = "Eli";
let count = 3;
let apples = 3;
let oranges = 4;
let total = apples + oranges;
let more_apples = apples > oranges;
let count: i32 = 3;
let name: &str = "Eli";
fn static_message() -> &'static str { "Hello from a string literal" }
let numbers = [3, 4, 5];
let last_index: usize = numbers.len() - 1;
let empty: [i32; 0] = [];
```

`"Eli"` is text written as a string literal. `3` is a number written as an
integer literal. `apples + oranges` is an arithmetic expression that evaluates
to a number. `apples > oranges` is a comparison expression that evaluates to a
boolean value. `i32` is a written integer type annotation. These values can be
bound with `let`, but they are different kinds of values. `&str` is the string
type written in the `hello_str_binding` experiment. `[3, 4, 5]` is an
array value whose elements are integer values. `usize` is the integer type
used by the computed index in `hello_array_last_index`. `[i32; 0]` is an
array type with element type `i32` and length `0`.

## Type inference

The `hello_integer` experiment did not write a type annotation:

```rust
let count = 3;
```

Rust still accepted the program because the compiler could choose a type for
the integer literal. For this current model, the important point is that Rust
knows a type even when the source code does not spell it out.

## Type annotations

The `hello_type_annotation` experiment writes the type explicitly:

```rust
let count: i32 = 3;
let name: &str = "Eli";
```

Here, `: i32` and `: &str` are type annotations. The bindings still name
literal values, but the source now spells out the types.

`hello_static_str_return` writes a function return type:

```rust
fn static_message() -> &'static str {
    "Hello from a string literal"
}
```

Here, `&'static str` is the return type. It is still a type, not the string
literal value itself.

## Array type

`hello_array_for` introduces an array value:

```rust
let numbers = [3, 4, 5];
```

The source does not write the full array type. The Rust Reference writes the
array type shape as `[T; N]`, where `T` is the element type and `N` is the
number of elements.

`hello_empty_array_len` writes that shape directly:

```rust
let numbers: [i32; 0] = [];
```

This is an array type annotation. It gives the empty array expression an
element type and a length.

## `usize`

`hello_array_last_index` writes an index type explicitly:

```rust
let last_index: usize = numbers.len() - 1;
let last = numbers[last_index];
```

The standard-library slice `len` method returns `usize`, and the Rust
Reference says array and slice index expressions use an index expression of
type `usize`.

## Useful guardrail

Types are not the same thing as bindings. A binding gives a value a name; a
type tells Rust what kind of value it is.

An `if` expression that produces a value also needs one type. The
`hello_if_type_error` experiment produced [E0308](compiler_error_e0308.md) when
one branch produced a string literal and the other branch produced an integer
literal.

## Corpus references

- [The Rust Book: Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [The Rust Book: Control Flow](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html)
- [Rust Reference: Boolean type](https://doc.rust-lang.org/stable/reference/types/boolean.html)
- [Rust Reference: if expressions](https://doc.rust-lang.org/stable/reference/expressions/if-expr.html)
- [Rust Reference: numeric types](https://doc.rust-lang.org/stable/reference/types/numeric.html)
- [Rust Reference: Array types](https://doc.rust-lang.org/stable/reference/types/array.html)
- [Rust std: slice `len`](https://doc.rust-lang.org/stable/std/primitive.slice.html)
- [Rust Reference: Array indexing expressions](https://doc.rust-lang.org/stable/reference/expressions/array-expr.html)
- [Rust by Example: Variable Bindings](https://doc.rust-lang.org/stable/rust-by-example/variable_bindings.html)

## Related wiki pages

- [Bindings](bindings.md)
- [Type inference](type_inference.md)
- [Type annotations](type_annotations.md)
- [`i32`](i32.md)
- [`Option<T>` syntax](option_t_syntax.md)
- [`&`](ampersand.md)
- [`&str`](str.md)
- [`'static`](static_lifetime.md)
- [Literals](literals.md)
- [String literals](string_literals.md)
- [Integer literals](integer_literals.md)
- [Array](array.md)
- [Empty array](empty_array.md)
- [`usize`](usize.md)
- [Boolean values](boolean_values.md)
- [Comparison expressions](comparison_expressions.md)
- [Addition operator](addition_operator.md)
- [Arithmetic expressions](arithmetic_expressions.md)
- [`let`](let_binding.md)
- [`error[E0308]`](compiler_error_e0308.md)
- [Concepts so far](concepts.md)
