<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_early_return/src/main.rs
  - RustPlayground/experiments/hello_static_str_return/src/main.rs
  - output/docs/rust/reference/expressions/literal-expr.md
  - output/docs/rust/book/ch04-03-slices.md
  - output/docs/rust/reference/types/str.md
  - output/docs/rust/error_codes/E0106.md
topic: rust-playground/string-literal-return-type
---

# String Literal Return Type

`&'static str` is the return type used in `hello_early_return` and
`hello_static_str_return` for functions that return string literals. It
appears first with `"none"` and `"some"` returned from `describe_count`, then
becomes more explicit in `static_message`.

## Shape I have used

```rust
fn describe_count(count: i32) -> &'static str {
    if count == 0 {
        return "none";
    }

    "some"
}
```

The return type is written after `->`. Both returned values are string
literals.

`hello_static_str_return` uses a no-argument function:

```rust
fn static_message() -> &'static str {
    "Hello from a string literal"
}
```

## Why this type appears

The Rust Reference says a string literal expression has type `&'static str`.
The Rust Book introduces string literals as `&str` values stored in the
program binary.

The current useful model is narrow: when a function returns string literals,
writing `-> &'static str` gives the function a concrete return type.

The separate [`&str`](str.md) page covers the shorter type shape used for a
string literal binding. The [`'static`](static_lifetime.md) page covers why the
lifetime marker matters for a no-argument function return.

## Why `-> &str` was not enough

`hello_static_str_return` records the tempting first signature:

```rust
fn static_message() -> &str {
    "Hello from a string literal"
}
```

That form produced E0106 because the function has no input lifetime for Rust
to connect to the returned reference. Writing `-> &'static str` fits this
specific case because string literals have the `'static` lifetime.

## Useful guardrail

`&'static str` is a type, not a value. `"none"` and `"some"` are the values
returned by the function.

## Corpus references

- [Rust Reference: literal expressions](../../output/docs/rust/reference/expressions/literal-expr.md)
- [The Rust Book: String slices](../../output/docs/rust/book/ch04-03-slices.md)
- [Rust Reference: str](../../output/docs/rust/reference/types/str.md)
- [Rust error code E0106](../../output/docs/rust/error_codes/E0106.md)

## Related wiki pages

- [String literals](string_literals.md)
- [`&str`](str.md)
- [`&`](ampersand.md)
- [`'static`](static_lifetime.md)
- [`error[E0106]`](compiler_error_e0106.md)
- [Function return values](function_return_values.md)
- [Early return](early_return.md)
- [`return`](return_keyword.md)
- [Types](types.md)
- [Concepts so far](concepts.md)
