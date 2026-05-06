<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_str_binding/src/main.rs
  - RustPlayground/experiments/hello_static_str_return/src/main.rs
  - output/docs/rust/reference/types/pointer.md
  - output/docs/rust/reference/types/str.md
  - output/docs/rust/reference/expressions/literal-expr.md
  - output/docs/rust/book/ch10-03-lifetime-syntax.md
topic: rust-playground/ampersand
---

# `&`

`&` marks a reference in the Rust type shapes used so far. In `&str`, the
ampersand means the type is a reference to `str` text data, not bare `str`.

## Shapes I have used

```rust
let name: &str = "Eli";

fn static_message() -> &'static str {
    "Hello from a string literal"
}
```

In `&str`, `&` starts the reference type. In `&'static str`, `&` still starts
the reference type, and `'static` is the explicit lifetime written after it.

## Why `str` appears with `&`

The Rust Reference says `str` represents a sequence of characters, but a `str`
value can only be used through a pointer type such as `&str`.

That matches `hello_str_binding`. The source writes this:

```rust
let name: &str = "Eli";
```

It does not write a bare `str` binding type.

## Shared reference type

The Rust Reference describes `&` as the shared reference form. In the current
model, read that as: the reference points at existing data, but the reference
itself is not the data.

The type is written as `&type`, or as `&'a type` when an explicit lifetime is
needed.

`hello_str_binding` and `hello_static_str_return` have used these two concrete
forms:

```rust
&str
&'static str
```

`&mut` also exists, but it has not appeared in `hello_str_binding` or
`hello_static_str_return`.

## Ownership language is later

The official docs say a shared reference points to memory owned by some other
value. That wording belongs to Rust's ownership model. The current wiki has
not introduced ownership yet, so the useful translation is narrower:

- `&str` is a reference to `str` text data.
- `&'static str` is a reference to `str` text data with an explicit
  `'static` lifetime.
- The reference is not the text data itself.

`&` also appears in expressions to create references, but that form has not
appeared in any experiment yet.

## Corpus references

- [Rust Reference: Pointer types](../../output/docs/rust/reference/types/pointer.md)
- [Rust Reference: String slice type](../../output/docs/rust/reference/types/str.md)
- [Rust Reference: String literal expressions](../../output/docs/rust/reference/expressions/literal-expr.md)
- [The Rust Book: Lifetime annotation syntax](../../output/docs/rust/book/ch10-03-lifetime-syntax.md)

## Related wiki pages

- [`&str`](str.md)
- [`'static`](static_lifetime.md)
- [String literal return type](string_literal_return_type.md)
- [Types](types.md)
- [Type annotations](type_annotations.md)
- [`error[E0106]`](compiler_error_e0106.md)
- [Concepts so far](concepts.md)
