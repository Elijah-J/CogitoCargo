<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_early_return/src/main.rs
  - experiments/hello_static_str_return/src/main.rs
  - https://doc.rust-lang.org/stable/reference/expressions/literal-expr.html
  - https://doc.rust-lang.org/stable/book/ch04-03-slices.html
  - https://doc.rust-lang.org/stable/reference/types/str.html
  - https://doc.rust-lang.org/stable/error_codes/E0106.html
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

- [Rust Reference: literal expressions](https://doc.rust-lang.org/stable/reference/expressions/literal-expr.html)
- [The Rust Book: String slices](https://doc.rust-lang.org/stable/book/ch04-03-slices.html)
- [Rust Reference: str](https://doc.rust-lang.org/stable/reference/types/str.html)
- [Rust error code E0106](https://doc.rust-lang.org/stable/error_codes/E0106.html)

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
