<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - experiments/hello_str_binding/src/main.rs
  - experiments/hello_utf8_literal/src/main.rs
  - experiments/hello_utf8_len/src/main.rs
  - https://developer.mozilla.org/en-US/docs/Glossary/ASCII
  - https://developer.mozilla.org/en-US/docs/Glossary/UTF-8
  - https://doc.rust-lang.org/stable/reference/types/str.html
  - https://doc.rust-lang.org/stable/reference/input-format.html
  - https://doc.rust-lang.org/stable/reference/tokens.html
  - https://doc.rust-lang.org/stable/reference/expressions/literal-expr.html
topic: rust-playground/ascii
---

# ASCII

ASCII is a 7-bit character encoding for 128 characters. In the current Rust
model, it matters because the first string literals use ASCII text, and ASCII
text is valid UTF-8.

## Shape I have used

```rust
let name: &str = "Eli";
let ascii_word: &str = "cafe";
```

`"Eli"` and `"cafe"` are string literals made from ordinary ASCII letters. The
binding type can be written as `&str` because this text is valid UTF-8.

## What ASCII includes

MDN describes ASCII as a character encoding that represents 128 characters.
The printable part includes digits, lowercase letters, uppercase letters, and
punctuation. The non-printing part includes control codes such as tab and line
feed.

That is enough for the current model:

- `E`, `l`, and `i` are ASCII letters.
- `c`, `a`, `f`, and `e` are ASCII letters.
- digits such as `0` through `9` are ASCII characters.
- punctuation such as `!` and `,` is part of ASCII.

## What ASCII is not

`hello_utf8_literal` contrasts an ASCII string literal with a
non-ASCII string literal:

```rust
let ascii_word: &str = "cafe";
let utf8_word: &str = "café";
```

`"cafe"` uses only ASCII letters. `"café"` contains `é`, which is not one of
the ASCII letters `a` through `z` or `A` through `Z`. The second literal is
not ASCII text, but it is still valid UTF-8 text.

`hello_utf8_len` makes the byte difference visible: `"cafe"` has
byte length `4`, while `"café"` has byte length `5`.

## Why ASCII fits UTF-8

MDN says the first 128 UTF-8 characters precisely match the first 128 ASCII
characters. That means existing ASCII text is already valid UTF-8.

For Rust, this connects back to `str`: Rust's docs say `str` data is assumed
to be valid UTF-8. ASCII string literals fit that rule without introducing any
new character examples. Non-ASCII string literals can also fit that rule when
their text is valid UTF-8.

## Rust string literal connection

The Rust Reference says a string literal can contain Unicode characters inside
double quotes. ASCII characters are the simple subset used in
`hello_str_binding`, `hello_utf8_literal`, and `hello_utf8_len`.

Rust also has a 7-bit code point escape form, written with `\x` and two hex
digits up to `0x7F`, for ASCII characters. That escape form is a signpost for
later; `hello_utf8_literal` and `hello_utf8_len` write the characters directly.

## Useful guardrail

ASCII is not all text. It is the small, old character set that fits inside the
first 128 UTF-8 characters. `hello_utf8_literal` shows that non-ASCII text can
still be valid UTF-8.

## Corpus references

- [MDN Glossary: ASCII](https://developer.mozilla.org/en-US/docs/Glossary/ASCII)
- [MDN Glossary: UTF-8](https://developer.mozilla.org/en-US/docs/Glossary/UTF-8)
- [Rust Reference: str](https://doc.rust-lang.org/stable/reference/types/str.html)
- [Rust Reference: source encoding](https://doc.rust-lang.org/stable/reference/input-format.html)
- [Rust Reference: tokens](https://doc.rust-lang.org/stable/reference/tokens.html)
- [Rust Reference: string literal expressions](https://doc.rust-lang.org/stable/reference/expressions/literal-expr.html)

## Related wiki pages

- [UTF-8](utf_8.md)
- [String literals](string_literals.md)
- [`&str`](str.md)
- [`.len()` on `&str`](str_len.md)
- [Literals](literals.md)
- [Concepts so far](concepts.md)
