<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_str_len_syntax/src/main.rs
  - experiments/hello_utf8_len/src/main.rs
  - https://doc.rust-lang.org/stable/std/primitive.str.html
  - https://doc.rust-lang.org/stable/reference/paths.html
  - https://doc.rust-lang.org/stable/reference/expressions/path-expr.html
  - https://doc.rust-lang.org/stable/reference/expressions/method-call-expr.html
  - https://doc.rust-lang.org/stable/reference/items/associated-items.html
  - https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html
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

- [Rust std: `str::len`](https://doc.rust-lang.org/stable/std/primitive.str.html)
- [Rust Reference: paths](https://doc.rust-lang.org/stable/reference/paths.html)
- [Rust Reference: path expressions](https://doc.rust-lang.org/stable/reference/expressions/path-expr.html)
- [Rust Reference: method-call expressions](https://doc.rust-lang.org/stable/reference/expressions/method-call-expr.html)
- [Rust Reference: associated functions and methods](https://doc.rust-lang.org/stable/reference/items/associated-items.html)
- [The Rust Book: method syntax](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html)

## Related wiki pages

- [`.len()` on `&str`](str_len.md)
- [`&str`](str.md)
- [Function calls](function_calls.md)
- [Functions](functions.md)
- [`usize`](usize.md)
- [UTF-8](utf_8.md)
- [Concepts so far](concepts.md)
