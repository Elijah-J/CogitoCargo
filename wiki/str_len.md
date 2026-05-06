<!-- source: wiki (compiled) -->
---
method: llm-compiled
compiled: 2026-05-06
sources:
  - RustPlayground/experiments/hello_array_len/src/main.rs
  - RustPlayground/experiments/hello_str_len_syntax/src/main.rs
  - RustPlayground/experiments/hello_utf8_chars_count/src/main.rs
  - RustPlayground/experiments/hello_utf8_len/src/main.rs
  - RustPlayground/experiments/hello_utf8_literal/src/main.rs
  - output/docs/rust/std/primitive.array.md
  - output/docs/rust/std/primitive.slice.md
  - output/docs/rust/std/primitive.str.md
  - output/docs/rust/std/primitive.char.md
  - output/docs/rust/std/iter/trait.Iterator.md
  - output/docs/rust/reference/types/str.md
  - output/docs/rust/book/ch08-02-strings.md
  - output/docs/mdn-glossary/ASCII.md
  - output/docs/mdn-glossary/UTF-8.md
topic: rust-playground/str-len
---

# `.len()` on `&str`

`.len()` on `&str` returns the length in bytes. It does not return the number
of visible letters.

## Shape I have used

```rust
let ascii_word: &str = "cafe";
let utf8_word: &str = "café";

println!("ASCII bytes: {}", ascii_word.len());
println!("UTF-8 bytes: {}", utf8_word.len());
```

This calls the `len` method on each `&str` binding.

The program prints:

```console
ASCII bytes: 4
UTF-8 bytes: 5
```

## Why the numbers differ

`"cafe"` uses only ASCII letters. ASCII characters fit into one byte in UTF-8,
so its byte length is `4`.

`"café"` has four visible letters, but `é` is not ASCII. UTF-8 can represent a
character with more than one byte, so this string's byte length is `5`.

## Rust-specific connection

The Rust standard library docs for `str::len` say the returned length is in
bytes, not `char`s or graphemes. The Rust Reference also says `str` is
stored as a sequence of 8-bit bytes, while `str` methods assume valid
UTF-8.

That matches `hello_utf8_len`: both values are valid `&str`, but `.len()` counts
their bytes.

`hello_str_len_syntax` also calls the same method as `str::len(ascii_word)`.
That call form changes how the method is named, not what length means.

`hello_utf8_chars_count` contrasts `.len()` with
`word.chars().count()`. `.len()` still reports bytes; `chars().count()` counts
Rust `char` values.

## Contrast with array `.len()`

`hello_array_len` uses the same method spelling on an array:

```rust
let numbers = [3, 4, 5];
let length = numbers.len();
```

For an array, `.len()` counts stored elements, so this length is `3`. For
`&str`, `.len()` counts bytes. The receiver type decides which `len` method is
being called.

## Useful guardrail

`.len()` answers "how many bytes?" for `&str`. To count Rust `char` values in
`hello_utf8_chars_count`, use `word.chars().count()`. Grapheme clusters and
iterating through raw bytes are still separate topics.

## Corpus references

- [Rust std: array primitive](../../output/docs/rust/std/primitive.array.md)
- [Rust std: slice `len`](../../output/docs/rust/std/primitive.slice.md)
- [Rust std: `str::len`](../../output/docs/rust/std/primitive.str.md)
- [Rust std: `char`](../../output/docs/rust/std/primitive.char.md)
- [Rust std: `Iterator::count`](../../output/docs/rust/std/iter/trait.Iterator.md)
- [Rust Reference: `str`](../../output/docs/rust/reference/types/str.md)
- [The Rust Book: bytes and UTF-8 strings](../../output/docs/rust/book/ch08-02-strings.md)
- [MDN Glossary: ASCII](../../output/docs/mdn-glossary/ASCII.md)
- [MDN Glossary: UTF-8](../../output/docs/mdn-glossary/UTF-8.md)

## Related wiki pages

- [`&str`](str.md)
- [`str::len` syntax](str_len_syntax.md)
- [`str::chars`](str_chars.md)
- [`Iterator::count`](iterator_count.md)
- [`char`](char.md)
- [UTF-8](utf_8.md)
- [ASCII](ascii.md)
- [Array length](array_len.md)
- [String literals](string_literals.md)
- [Literals](literals.md)
- [Concepts so far](concepts.md)
