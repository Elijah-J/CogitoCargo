---
id: 081-integer-literal-forms
status: accepted
evidence: ../evidence/081-integer-literal-forms.md
---

# Write integer literals in five non-decimal forms plus the `_` separator

## The Move

Every integer literal so far has been *decimal* — `42`, `5`, `100`.
Today opens the rest of Book Ch3-2 Table 3-2: five more spellings for
integer values, plus two notational conventions.

The five forms, by example:

- *Hex* (base 16): `0xff` — `0x` prefix, then digits `0`-`9` and
  `a`-`f`. Value: `255`.
- *Octal* (base 8): `0o77` — `0o` prefix, digits `0`-`7`. Value: `63`.
- *Binary* (base 2): `0b1111_0000` — `0b` prefix, digits `0`-`1`.
  Value: `240`.
- *Type-suffix*: `57u8` — a literal followed *immediately* (no space)
  by one of the twelve integer type names from lesson 080. Equivalent
  to `let x: u8 = 57;`.
- *Byte literal*: `b'A'` — letter `b` then one ASCII character in
  single quotes. Value: `65` (the ASCII code of `A`), type: `u8`.

And two conventions:

- *Visual separator `_`*: `1_000` is the same value as `1000`. The
  underscore can appear between digits in any of the four base forms.
- *Default still `i32`*: an unsuffixed literal in any base defaults to
  `i32` when nothing else fixes the type — same rule as lessons 019
  and 080. The suffix form is one of the things that *does* fix it.

(*Hex*, *octal*, and *binary* are alternative notations for whole
numbers, using 16, 8, and 2 digit symbols. *ASCII* is the standard
7-bit code table where each basic Latin letter, digit, or punctuation
mark has a number from `0` to `127`; `'A'` is `65`.)

## Mental Model Delta

- *Before*: "I write integer literals as bare decimal digits like
  `42`. I have no picture of the rest of the literal grammar."
- *After*: "Decimal is one of several forms. Five more forms produce
  integer values: hex `0xff`, octal `0o77`, binary `0b1111_0000`,
  type-suffix `57u8`, and byte `b'A'`. An underscore between digits
  is a visual separator only. A byte literal is the only form whose
  printed value isn't the digits I typed — `b'A'` is a `u8` value
  `65`, the ASCII code of `A`, and only ASCII is allowed there."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs` then `./name`; rustc silent on
    success.
  - Lesson 003 (load-bearing): the four-part diagnostic map.
  - Lesson 005 (load-bearing): `let name = value;`.
  - Lesson 019 (load-bearing): integer literals default to `i32`.
    Today inherits this for the unsuffixed forms.
  - Lesson 062 (load-bearing): `u32` as a typed name, installed via
    the annotation form `let n: u32 = 42;`. Today's `57u8` suffix is
    the first time the run installs a literal-bound type *without* a
    separate `: TYPE` annotation — a one-token alternative to that
    annotation form, valid for any of the twelve integer types from
    lesson 080.
  - Lesson 080 (load-bearing): the twelve-name integer type family —
    the exact set today's suffix may name.
  - Lesson 074 (cited): `'A'` is a `char` literal; today's byte
    literal `b'A'` is a *different* form (`u8`, not `char`).
  - Lesson 011 (cited): positional `{}` printing.
- Ordinary computer-use assumptions: same as lesson 001. One added
  fact: ASCII is the standard 7-bit character code table; the lesson
  does not depend on it beyond the equality `b'A' == 65u8`. The
  working probe witnesses this by printing `byte = 65` for
  `let byte = b'A';`; Probe 5 in the evidence appendix shows the
  literal `==` comparison evaluating to `true`.

## Try It

In a fresh empty directory, save `demo.rs`:

```rust
fn main() {
    let dec = 1_000;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_0000;
    let suffix = 57u8;
    let byte = b'A';
    println!("dec = {}", dec);
    println!("hex = {}", hex);
    println!("oct = {}", oct);
    println!("bin = {}", bin);
    println!("suffix = {}", suffix);
    println!("byte = {}", byte);
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
dec = 1000
hex = 255
oct = 63
bin = 240
suffix = 57
byte = 65
```

`1_000` prints `1000` (separator was visual). `0xff`, `0o77`,
`0b1111_0000` print their values in plain decimal. `57u8` prints `57`
— the suffix is consumed at compile time. `b'A'` prints `65`, *not*
the letter, because the binding's type is `u8`.

Now the contrast. Save `broken.rs`:

```rust
fn main() {
    let byte = b'ℤ';
    println!("byte = {}", byte);
}
```

One character changed: ASCII `A` became non-ASCII `ℤ`. Read with the
lesson 003 map; full transcript in the evidence appendix:

```text
error: non-ASCII character in byte literal
 --> broken.rs:2:18
  |
2 |     let byte = b'ℤ';
  |                  ^
  |                  |
  |                  must be ASCII
  |                  this multibyte character does not fit into a single byte
```

The headline names the rule. The label lines under the caret spell it
out: `must be ASCII` and `this multibyte character does not fit into
a single byte`. With ASCII the byte literal compiles; with non-ASCII
it does not. (For non-ASCII single characters, lesson 074's `char`
literal `'ℤ'` is the right form — different prefix, different type.)

## What Changed

- Five new spellings for integer literals: hex `0xff`, octal `0o77`,
  binary `0b1111_0000`, type-suffix `57u8`, byte `b'A'`. Each is a
  literal expression and fits on the right of `let`.
- The `_` separator is purely visual; rustc treats `1_000_000` and
  `1000000` identically. It can sit between digits in any base form.
- A type suffix is a one-token alternative to a `: TYPE` annotation.
  Any of the twelve integer type names from lesson 080 may be the
  suffix; literal and suffix touch with no space.
- A byte literal `b'A'` produces a `u8` whose value is the ASCII code
  of the character. Only ASCII (codes `0`-`127`); non-ASCII fires
  `error: non-ASCII character in byte literal` at compile time. The
  `b` prefix is load-bearing: without it, `'A'` is a `char` (lesson
  074), a different type.

## Check Yourself

Predict, do not run. Give value (in plain decimal) and type:

(a) `let n = 0x10;`

(b) `let n = 0b1010;`

(c) `let n = 1_000_000_000_i64;` (suffix at the end).

(d) `let n = b'0';` (the *character* zero).

(e) `let c = '0';` (no `b`). Same value and type as (d)?

(Answers: (a) `16`, `i32` — `1*16 + 0`, no suffix so the default
applies. (b) `10`, `i32` — `8 + 2`. (c) `1000000000`, `i64` — suffix
pins it. (d) `48`, `u8` — ASCII code of digit `0`. (e) value displays
as the character `0`, not the number `48`; type is `char`, not `u8`.
The `b` prefix is what flips byte-vs-char.)

## What To Ignore For Now

- *Float literals* `3.14`, `1e10`, `2.0f32` — separate table; separate
  suffix set (`f32`, `f64`). Queue item O.
- *Char escape sequences* `'\n'`, `'\u{1F600}'` — lesson 074 deferred
  these; still deferred.
- *Byte string literals* `b"hello"`, *raw string literals* `r"..."` —
  same `b`/`r` prefix family but different types; separate moves.
- *Integer-to-integer `as` casts* — extends lesson 034's `i32 as f64`.
- *`MIN` / `MAX` associated constants* — deferred from lesson 080.
- *Integer overflow at arithmetic* — queue item I.
- *The full Reference lexer grammar* (`BIN_LITERAL`, `SUFFIX_NO_E`,
  etc.) — today follows the Book's audience-level shape.
- *The `overflowing_literals` lint* on suffixed literals (`256u8`) —
  same lint family lesson 080 saw.

## Evidence

See `../evidence/081-integer-literal-forms.md`.
