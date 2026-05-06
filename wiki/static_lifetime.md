<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_static_str_return/src/main.rs
  - output/docs/rust/book/ch10-03-lifetime-syntax.md
  - output/docs/rust/reference/expressions/literal-expr.md
  - output/docs/rust/book/ch04-03-slices.md
  - output/docs/rust/error_codes/E0106.md
topic: rust-playground/static-lifetime
---

# `'static`

`'static` is a lifetime name for a reference that can live for the entire
duration of the program. In `hello_static_str_return`, it appears in the
return type `&'static str` because the function returns a string literal.

## Shape I have used

```rust
fn static_message() -> &'static str {
    "Hello from a string literal"
}
```

`static_message` returns a string literal. The return type says the returned
value is a `&str` with the `'static` lifetime.

## Why plain `&str` was not enough

The tempting first signature was:

```rust
fn static_message() -> &str {
    "Hello from a string literal"
}
```

That does not compile for a no-argument function. The return type says the
function returns a `&str`, but it does not say how long that
returned reference is valid.

The E0106 docs describe the limited cases where Rust can infer an omitted
output lifetime in a function signature. A no-argument function returning
`&str` has no input lifetime for Rust to connect to the output, so the
function needs a written lifetime.

## Why `'static` fits this example

The Rust Reference says a string literal expression has type `&'static str`.
The Rust Book says the text of a string literal is stored directly in the
program binary and is always available.

That is the narrow motivation for `hello_static_str_return`: `&'static str`
fits a function that returns a string literal because the referenced string
literal is available for the whole program.

## Useful guardrail

`'static` is not a way to make an ordinary short-lived reference last longer.
The Rust Book warns that `'static` suggestions can also come from dangling
references or lifetime mismatches. The current model only uses `'static` for
string literals.

## Corpus references

- [The Rust Book: The Static Lifetime](../../output/docs/rust/book/ch10-03-lifetime-syntax.md)
- [Rust Reference: String literal expressions](../../output/docs/rust/reference/expressions/literal-expr.md)
- [The Rust Book: String literals as slices](../../output/docs/rust/book/ch04-03-slices.md)
- [Rust error code E0106](../../output/docs/rust/error_codes/E0106.md)

## Related wiki pages

- [`&str`](str.md)
- [`&`](ampersand.md)
- [String literal return type](string_literal_return_type.md)
- [`error[E0106]`](compiler_error_e0106.md)
- [String literals](string_literals.md)
- [Function return values](function_return_values.md)
- [Types](types.md)
- [Concepts so far](concepts.md)
