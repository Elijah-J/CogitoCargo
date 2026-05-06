<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_empty_array_len/src/main.rs
  - experiments/hello_variables/src/main.rs
  - experiments/hello_str_binding/src/main.rs
  - experiments/hello_static_str_return/src/main.rs
  - experiments/hello_integer/src/main.rs
  - experiments/hello_addition/src/main.rs
  - experiments/hello_comparison/src/main.rs
  - experiments/hello_type_annotation/src/main.rs
  - experiments/hello_array_for/src/main.rs
  - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
  - https://doc.rust-lang.org/stable/book/ch04-03-slices.html
  - https://doc.rust-lang.org/stable/reference/statements.html
  - https://doc.rust-lang.org/stable/reference/types/boolean.html
  - https://doc.rust-lang.org/stable/reference/types/str.html
  - https://doc.rust-lang.org/stable/reference/types/array.html
  - https://doc.rust-lang.org/stable/reference/expressions/array-expr.html
  - https://doc.rust-lang.org/stable/rust-by-example/variable_bindings.html
  - https://doc.rust-lang.org/stable/rust-by-example/types/literals.html
  - https://doc.rust-lang.org/stable/reference/expressions/literal-expr.html
  - https://doc.rust-lang.org/stable/error_codes/E0106.html
topic: rust-playground/type-inference
---

# Type Inference

Type inference is the compiler determining a type without the source code
writing that type explicitly. Rust still needs to know the type at compile
time; inference is how it can know without a type annotation in simple cases.

## Shape I have used

```rust
let count = 3;
println!("Count: {count}");
```

There is no `: i32` or other type annotation after `count`. Rust accepts the
program because it can infer a type for the binding.

The same shape appears in `hello_addition`:

```rust
let apples = 3;
let oranges = 4;
let total = apples + oranges;
```

The source does not write integer types for `apples`, `oranges`, or `total`.
Rust still knows their types at compile time.

`hello_comparison` also omits type annotations:

```rust
let more_apples = apples > oranges;
let same_amount = apples == oranges;
```

Rust accepts these bindings and treats the comparison results as boolean
values.

`hello_array_for` omits a type annotation too:

```rust
let numbers = [3, 4, 5];
```

Rust can infer the array type from the element values and the number of
elements.

`hello_empty_array_len` shows the nearby boundary:

```rust
let numbers: [i32; 0] = [];
```

The empty array expression has no element values, so the source writes the
array type.

## Integer literal default

For `let count = 3;`, the integer literal `3` has no suffix and no stronger
constraint from the surrounding code. Rust's docs say this defaults to `i32`.

The current beginner model does not need to write `i32` yet. The important
lesson is that the compiler still has a type in mind.

`hello_type_annotation` writes that type explicitly:

```rust
let count: i32 = 3;
```

This is not inference. It is the source code telling Rust the type.

## String literal inference

`hello_variables` did not write a string type:

```rust
let name = "Eli";
```

The `hello_str_binding` experiment writes that type explicitly:

```rust
let name: &str = "Eli";
```

Both versions compile. The second version makes the `&str` type visible
in the source.

## Inference boundary

Rust can infer the type of a simple string literal binding, but
`hello_static_str_return` shows a boundary:

```rust
fn static_message() -> &str {
    "Hello from a string literal"
}
```

That signature produced E0106. The issue is not the literal value; the issue is
that the return type leaves out the lifetime of the returned reference. For
this no-argument function, Rust has no input lifetime to connect to the output.
The fixed return type is `&'static str`.

## Useful guardrail

Inference is not guessing at runtime. The Rust Book says Rust is statically
typed and must know variable types at compile time.

## Corpus references

- [The Rust Book: Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [Rust Reference: statements](https://doc.rust-lang.org/stable/reference/statements.html)
- [Rust Reference: Boolean type](https://doc.rust-lang.org/stable/reference/types/boolean.html)
- [Rust Reference: Array types](https://doc.rust-lang.org/stable/reference/types/array.html)
- [Rust Reference: Array expressions](https://doc.rust-lang.org/stable/reference/expressions/array-expr.html)
- [Rust by Example: Variable Bindings](https://doc.rust-lang.org/stable/rust-by-example/variable_bindings.html)
- [Rust by Example: Literals](https://doc.rust-lang.org/stable/rust-by-example/types/literals.html)
- [Rust Reference: Literal expressions](https://doc.rust-lang.org/stable/reference/expressions/literal-expr.html)
- [Rust error code E0106](https://doc.rust-lang.org/stable/error_codes/E0106.html)

## Related wiki pages

- [Types](types.md)
- [Type annotations](type_annotations.md)
- [Boolean values](boolean_values.md)
- [Comparison expressions](comparison_expressions.md)
- [Integer literals](integer_literals.md)
- [Array](array.md)
- [Empty array](empty_array.md)
- [`i32`](i32.md)
- [`&`](ampersand.md)
- [`&str`](str.md)
- [`'static`](static_lifetime.md)
- [`error[E0106]`](compiler_error_e0106.md)
- [Addition operator](addition_operator.md)
- [Arithmetic expressions](arithmetic_expressions.md)
- [Bindings](bindings.md)
- [`let`](let_binding.md)
- [`cargo check`](cargo_check.md)
- [Concepts so far](concepts.md)
