<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_str_binding/src/main.rs
  - RustPlayground/experiments/hello_static_str_return/src/main.rs
  - RustPlayground/experiments/hello_utf8_literal/src/main.rs
  - RustPlayground/experiments/hello_utf8_len/src/main.rs
  - RustPlayground/experiments/hello_utf8_chars_count/src/main.rs
  - RustPlayground/experiments/hello_variables/src/main.rs
  - output/docs/rust/std/primitive.str.md
  - output/docs/rust/book/ch04-03-slices.md
  - output/docs/rust/reference/types/str.md
  - output/docs/rust/reference/expressions/literal-expr.md
  - output/docs/rust/error_codes/E0106.md
topic: rust-playground/str
---

# `&str`

`&str` is the type used for string literal bindings.
`hello_str_binding` writes the type explicitly for the first time.

## Shape I have used

```rust
let name: &str = "Eli";
println!("Name: {name}");
```

`name` is the binding. `&str` is the written type annotation. `"Eli"` is the
string literal value.

## Compared with inferred string literal type

`hello_variables` did not write a type annotation:

```rust
let name = "Eli";
println!("Hello, {name}!");
```

Rust accepted that program because it could infer the type. `hello_str_binding`
writes the type in source:

```rust
let name: &str = "Eli";
```

## What the docs call this

The Rust Book describes string literals as "string slices." The Rust Reference
says `str` represents a sequence of characters and can be used through a
pointer type such as `&str`. The word "slice" has not been grounded by an
experiment yet; for now, `&str` is the type shape to write when a binding names
a string literal.

## What `str` refers to

`str` is text data. The Rust Reference says `str` represents a sequence of
characters and is stored as a sequence of 8-bit bytes. Rust's standard
library assumes that `str` contains valid UTF-8.

`&str` is a reference to that text data. For a string literal, the Rust Book
says the text is stored in the program binary, and the `&str` value points to
that part of the binary.

`hello_str_binding` and `hello_utf8_literal` use these shapes:

```rust
let name: &str = "Eli";
let utf8_word: &str = "café";
```

`"Eli"` and `"café"` are string literal values. `&str` is the type of the
reference to the text data.

`hello_utf8_len` calls `.len()` on `&str` bindings. Rust's standard
library docs say `str::len` returns byte length, so `"cafe"` reports `4` and
`"café"` reports `5`.

`hello_utf8_chars_count` calls `.chars()` on a `&str` binding. Rust's
standard library docs say `str::chars` returns an iterator over `char` values.

## When the lifetime has to be written

`hello_static_str_return` records a failed first signature:

```rust
fn static_message() -> &str {
    "Hello from a string literal"
}
```

For this no-argument function return, `&str` is too vague. The fixed version
writes the lifetime:

```rust
fn static_message() -> &'static str {
    "Hello from a string literal"
}
```

The shorter `&str` shape is enough for `hello_str_binding`. The longer
`&'static str` shape is needed when the function return type has to say how
long the returned reference is valid.

## Useful guardrail

`&str` is a type, not a value. `"Eli"` is the value in
`let name: &str = "Eli";`.

## Corpus references

- [Rust std: `str`](../../output/docs/rust/std/primitive.str.md)
- [The Rust Book: String slices](../../output/docs/rust/book/ch04-03-slices.md)
- [Rust Reference: str](../../output/docs/rust/reference/types/str.md)
- [Rust Reference: literal expressions](../../output/docs/rust/reference/expressions/literal-expr.md)
- [Rust error code E0106](../../output/docs/rust/error_codes/E0106.md)

## Related wiki pages

- [String literals](string_literals.md)
- [UTF-8](utf_8.md)
- [`.len()` on `&str`](str_len.md)
- [`str::len` syntax](str_len_syntax.md)
- [`str::chars`](str_chars.md)
- [`char`](char.md)
- [`&`](ampersand.md)
- [Type annotations](type_annotations.md)
- [Type inference](type_inference.md)
- [Types](types.md)
- [`'static`](static_lifetime.md)
- [String literal return type](string_literal_return_type.md)
- [`error[E0106]`](compiler_error_e0106.md)
- [Bindings](bindings.md)
- [Concepts so far](concepts.md)
