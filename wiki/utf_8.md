<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_str_binding/src/main.rs
  - RustPlayground/experiments/hello_utf8_literal/src/main.rs
  - RustPlayground/experiments/hello_utf8_len/src/main.rs
  - RustPlayground/experiments/hello_utf8_chars_count/src/main.rs
  - output/docs/mdn-glossary/UTF-8.md
  - output/docs/mdn-glossary/Code_point.md
  - output/docs/rust/std/primitive.str.md
  - output/docs/rust/std/primitive.char.md
  - output/docs/rust/book/ch08-02-strings.md
  - output/docs/rust/reference/types/str.md
  - output/docs/rust/reference/input-format.md
  - output/docs/rust/reference/expressions/literal-expr.md
topic: rust-playground/utf-8
---

# UTF-8

UTF-8 is a character encoding: it represents text as bytes. In the current
Rust model, it matters because `str` text data is valid UTF-8.

## Shape I have used

```rust
let name: &str = "Eli";
let utf8_word: &str = "café";
```

`"Eli"` is an ASCII string literal. `"café"` is a non-ASCII string literal.
Both types can be written as `&str`, and Rust's docs say `str` methods assume
the data contains valid UTF-8.

## Characters and bytes

MDN describes UTF-8 as a character encoding where each character is represented
by one to four bytes. MDN's code point page describes a code point as a number
assigned to represent an abstract character, and says encoding forms such as
UTF-8 determine how a code point becomes a sequence of bytes.

For the current model, that means:

- text has characters
- UTF-8 represents those characters as bytes
- `str` is Rust text data that must be valid UTF-8

## ASCII text still fits

MDN says the first 128 UTF-8 characters match the first 128 ASCII characters,
so existing ASCII text is already valid UTF-8.

That explains why early string literal examples like `hello_variables` and
`hello_static_str_return` can stay simple:

```rust
"Eli"
"Hello from a string literal"
```

These literals use ordinary ASCII characters, and ASCII text is valid UTF-8.

## Non-ASCII text can still fit

`hello_utf8_literal` adds one non-ASCII character:

```rust
let ascii_word: &str = "cafe";
let utf8_word: &str = "café";
```

The `é` in `"café"` is not ASCII. The program still compiles and runs because
Rust source files are UTF-8, and `str` text data is valid UTF-8.

## Byte length

`hello_utf8_len` calls `.len()` on both words:

```rust
println!("ASCII bytes: {}", ascii_word.len());
println!("UTF-8 bytes: {}", utf8_word.len());
```

It prints `4` for `"cafe"` and `5` for `"café"`. Rust's `str::len` returns
byte length, so non-ASCII UTF-8 text can be longer in bytes than it looks in
visible letters.

## `char` count

`hello_utf8_chars_count` compares byte length with `char` count:

```rust
let byte_count = word.len();
let char_count = word.chars().count();
```

For `"café"`, the byte count is `5` and the `char` count is `4`. The `é`
still fits in one Rust `char`, but its UTF-8 encoding uses more than one byte.

## Rust-specific connections

Rust source files are interpreted as Unicode characters encoded in UTF-8, and a
file that is not valid UTF-8 is an error.

Rust string literal expressions also connect directly to UTF-8. The Rust
Reference says a string literal expression has type `&'static str`, and its
value is a reference to a statically allocated `str` containing the UTF-8
encoding of the represented string.

## Useful guardrail

UTF-8 is the encoding of the text data. It is not the same thing as `&str`.
`&str` is the Rust reference type currently used to refer to valid UTF-8 text
data.

## Corpus references

- [MDN Glossary: UTF-8](../../output/docs/mdn-glossary/UTF-8.md)
- [MDN Glossary: Code point](../../output/docs/mdn-glossary/Code_point.md)
- [Rust std: `str`](../../output/docs/rust/std/primitive.str.md)
- [Rust std: `char`](../../output/docs/rust/std/primitive.char.md)
- [The Rust Book: bytes, scalar values, and grapheme clusters](../../output/docs/rust/book/ch08-02-strings.md)
- [Rust Reference: str](../../output/docs/rust/reference/types/str.md)
- [Rust Reference: source encoding](../../output/docs/rust/reference/input-format.md)
- [Rust Reference: string literal expressions](../../output/docs/rust/reference/expressions/literal-expr.md)

## Related wiki pages

- [`&str`](str.md)
- [ASCII](ascii.md)
- [String literals](string_literals.md)
- [`.len()` on `&str`](str_len.md)
- [`str::chars`](str_chars.md)
- [`char`](char.md)
- [Iterator](iterators.md)
- [`&`](ampersand.md)
- [Types](types.md)
- [Literals](literals.md)
- [Concepts so far](concepts.md)
