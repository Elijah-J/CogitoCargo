<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_str_len_syntax/src/main.rs
  - RustPlayground/experiments/hello_utf8_len/src/main.rs
  - output/docs/rust/std/primitive.str.md
  - output/docs/rust/reference/paths.md
  - output/docs/rust/reference/expressions/path-expr.md
  - output/docs/rust/reference/expressions/method-call-expr.md
  - output/docs/rust/reference/items/associated-items.md
  - output/docs/rust/book/ch05-03-method-syntax.md
topic: rust-playground/str-len-syntax
---

# `str::len` Syntax

`str::len(ascii_word)` calls the same standard-library `len` method that
`ascii_word.len()` calls. In the current model, `str::len` is a path to the
`len` method associated with `str`.

## Shape I have used

```rust
let ascii_word: &str = "cafe";

let method_call_length = ascii_word.len();
let path_call_length = str::len(ascii_word);

println!("method call: {method_call_length}");
println!("path call: {path_call_length}");
```

The program prints:

```console
method call: 4
path call: 4
```

Both calls use the standard-library `str::len` method, so both return the byte
length of `"cafe"`.

## The method being called

The Rust standard library documents `len` under `str` with this signature:

```rust
pub const fn len(&self) -> usize
```

This introduces a standard-library method, not a language keyword. The `&self`
parameter means the method reads a `str` value through a shared reference. In
`hello_str_len_syntax`, `ascii_word` has type `&str`, so it can be the value
read by `len`.

## Dot syntax

```rust
let method_call_length = ascii_word.len();
```

The Rust Reference describes a method call as a receiver expression, a dot, a
method name, and parentheses. In `ascii_word.len()`, `ascii_word` is the
receiver: it is the value the method is called on.

## Path syntax

```rust
let path_call_length = str::len(ascii_word);
```

The Rust Reference says a path is made from path segments separated by `::`.
It also says a path expression can denote an item. In `str::len`, `str` and
`len` are the path segments, and the path names the `len` method associated
with `str`.

The Rust Reference also says methods can be invoked with the method-call
operator, as in `x.foo()`, and with usual function call notation. In
`hello_str_len_syntax`, `str::len(ascii_word)` is that function-call form.

The parentheses after `str::len` make this a call. The argument
`ascii_word` supplies the value that dot syntax put before the dot.

## Useful guardrail

`str::len` explains where the method is found. It does not change what length
means. The length rule still comes from `str::len`: it returns bytes, not
visible letters.

Modules and traits also use `::`, but they are separate concepts not yet
introduced in this wiki.

## Corpus references

- [Rust std: `str::len`](../../output/docs/rust/std/primitive.str.md)
- [Rust Reference: paths](../../output/docs/rust/reference/paths.md)
- [Rust Reference: path expressions](../../output/docs/rust/reference/expressions/path-expr.md)
- [Rust Reference: method-call expressions](../../output/docs/rust/reference/expressions/method-call-expr.md)
- [Rust Reference: associated functions and methods](../../output/docs/rust/reference/items/associated-items.md)
- [The Rust Book: method syntax](../../output/docs/rust/book/ch05-03-method-syntax.md)

## Related wiki pages

- [`.len()` on `&str`](str_len.md)
- [`&str`](str.md)
- [Function calls](function_calls.md)
- [Functions](functions.md)
- [`usize`](usize.md)
- [UTF-8](utf_8.md)
- [Concepts so far](concepts.md)
