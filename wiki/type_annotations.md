<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_empty_array_len/src/main.rs
  - RustPlayground/experiments/hello_array_last_index/src/main.rs
  - RustPlayground/experiments/hello_type_annotation/src/main.rs
  - RustPlayground/experiments/hello_str_binding/src/main.rs
  - RustPlayground/experiments/hello_function/src/main.rs
  - RustPlayground/experiments/hello_static_str_return/src/main.rs
  - output/docs/rust/book/ch03-02-data-types.md
  - output/docs/rust/book/ch03-03-how-functions-work.md
  - output/docs/rust/book/ch04-03-slices.md
  - output/docs/rust/reference/statements.md
  - output/docs/rust/reference/types/numeric.md
  - output/docs/rust/reference/types/array.md
  - output/docs/rust/reference/expressions/array-expr.md
  - output/docs/rust/std/primitive.slice.md
  - output/docs/rust/reference/types/str.md
  - output/docs/rust/rust-by-example/variable_bindings.md
topic: rust-playground/type-annotations
---

# Type Annotations

A type annotation writes the type of a binding in the source code. In
`hello_type_annotation`, `: i32` tells Rust that `count` has the `i32` integer
type.

## Shape I have used

```rust
let count: i32 = 3;
println!("Count: {count}");
```

`count` is the binding name. `i32` is the written type. `3` is the integer
literal used as the value.

## Annotation position

The type annotation goes after the binding name and before the `=`:

```rust
let name: Type = value;
```

The Rust Reference describes this type annotation as an optional part of a
`let` statement. When no type annotation is given, the compiler infers the
type if it has enough information.

## `i32`

`i32` is one of Rust's signed integer types. The Rust Book lists `i32` as the
signed 32-bit integer type and says integer types default to `i32` when there
is no stronger type information.

`hello_integer` used inference:

```rust
let count = 3;
```

`hello_type_annotation` writes the type explicitly:

```rust
let count: i32 = 3;
```

Both versions compile. The second version makes the type visible in the source.

## `&str`

The `hello_str_binding` experiment writes a string literal binding type:

```rust
let name: &str = "Eli";
```

Here, `: &str` is the type annotation. The binding names the string literal
`"Eli"`, but the source now spells out the type.

## `usize`

`hello_array_last_index` writes the type of a computed array index:

```rust
let last_index: usize = numbers.len() - 1;
```

Here, `: usize` is the type annotation. The standard-library slice `len`
method returns `usize`, and array indexing uses a `usize` index expression.

## Array type annotation

`hello_empty_array_len` writes a full array type:

```rust
let numbers: [i32; 0] = [];
```

The annotation says `numbers` has array type `[i32; 0]`. The `i32` part is the
element type, and the `0` part is the array length.

## Useful guardrail

A type annotation is not a new binding by itself. It is part of the `let`
statement that creates the binding.

Function signatures also write types:

```rust
fn add_one(number: i32) -> i32 {
    number + 1
}
```

`number: i32` writes the parameter type. `-> i32` writes the return type.

`hello_static_str_return` writes a string literal return type:

```rust
fn static_message() -> &'static str {
    "Hello from a string literal"
}
```

Here, `-> &'static str` writes the return type.

## Corpus references

- [The Rust Book: Data Types](../../output/docs/rust/book/ch03-02-data-types.md)
- [Rust Reference: statements](../../output/docs/rust/reference/statements.md)
- [Rust Reference: numeric types](../../output/docs/rust/reference/types/numeric.md)
- [Rust Reference: Array types](../../output/docs/rust/reference/types/array.md)
- [Rust Reference: Array indexing expressions](../../output/docs/rust/reference/expressions/array-expr.md)
- [Rust std: slice `len`](../../output/docs/rust/std/primitive.slice.md)
- [Rust by Example: Variable Bindings](../../output/docs/rust/rust-by-example/variable_bindings.md)

## Related wiki pages

- [Types](types.md)
- [Type inference](type_inference.md)
- [Integer literals](integer_literals.md)
- [`i32`](i32.md)
- [`usize`](usize.md)
- [Empty array](empty_array.md)
- [`&`](ampersand.md)
- [`&str`](str.md)
- [`'static`](static_lifetime.md)
- [Function parameters](function_parameters.md)
- [Function return values](function_return_values.md)
- [`let`](let_binding.md)
- [Statements](statements.md)
- [`cargo check`](cargo_check.md)
- [Concepts so far](concepts.md)
