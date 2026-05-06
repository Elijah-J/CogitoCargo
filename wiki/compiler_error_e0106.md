<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_static_str_return/src/main.rs
  - https://doc.rust-lang.org/stable/error_codes/E0106.html
  - https://doc.rust-lang.org/stable/reference/lifetime-elision.html
  - https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html
topic: rust-playground/compiler-error-e0106
---

# `error[E0106]`

`error[E0106]: missing lifetime specifier` is the compiler error produced when
`hello_static_str_return` first tried to write a no-argument function returning
`&str`.

## Shape that triggered it

```rust
fn static_message() -> &str {
    "Hello from a string literal"
}
```

The function has no parameters, but its return type is a reference type. Rust
needs to know how long the returned reference is valid.

## Compiler output

`rustc` reported:

```console
error[E0106]: missing lifetime specifier
  |
1 | fn static_message() -> &str { "Hello from a string literal" }
  |                        ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
```

The compiler also suggested `&'static str` for this string literal shape.

## Fix used in `hello_static_str_return`

```rust
fn static_message() -> &'static str {
    "Hello from a string literal"
}
```

The fixed version writes the lifetime in the return type. This works here
because a string literal has the type `&'static str`.

## Useful guardrail

The official E0106 docs say lifetime elision for a function output needs
either one input lifetime, or a method receiver such as `&self` or `&mut self`.
`hello_static_str_return` has no input, so `-> &str` leaves Rust without enough
information.

## Corpus references

- [Rust error code E0106](https://doc.rust-lang.org/stable/error_codes/E0106.html)
- [Rust Reference: Lifetime elision](https://doc.rust-lang.org/stable/reference/lifetime-elision.html)
- [The Rust Book: The Static Lifetime](https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html)

## Related wiki pages

- [`'static`](static_lifetime.md)
- [`&`](ampersand.md)
- [`&str`](str.md)
- [String literal return type](string_literal_return_type.md)
- [Function return values](function_return_values.md)
- [`cargo check`](cargo_check.md)
- [Concepts so far](concepts.md)
