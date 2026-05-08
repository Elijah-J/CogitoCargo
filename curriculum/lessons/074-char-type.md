---
id: 074-char-type
status: accepted
evidence: ../evidence/074-char-type.md
---

# Bind a single character with the `char` type and single quotes

## The Move

Inside `fn main`, write a `let` statement whose right side is a
character between *single* quotes: `let c = 'z';`. The single-quote
literal makes the value a `char` — Rust's primitive type for one
character. The lesson-019 type-annotation slot accepts `char`:
`let letter: char = 'A';` is the same `let name: TYPE = value;`
shape with `char` in the `TYPE` slot.

The single quotes are load-bearing. `'z'` produces a `char`. `"z"`
(double quotes) produces a string-literal value of a *different*
type — lesson 055 calls that type `&str`. Putting a double-quoted
literal in a `: char` slot fails at compile time with
`error[E0308]: mismatched types` and `expected \`char\`, found \`&str\``.

## Mental Model Delta

- *Before:* "I know typed names for whole numbers (`i32`, `u32`),
  fractional numbers (`f64`), and truth values (`bool`). I have not
  seen a primitive type for a single character. I have seen text in
  double quotes, but no Rust syntax for one character on its own."
- *After:* "There is one more primitive scalar type: `char`. A
  `char` literal is one character between *single* quotes — `'z'`,
  `'A'`, `'ℤ'`. I plug `char` into the lesson-019 `: TYPE` slot to
  annotate it. Quote shape matters: single → `char`; double → a
  *different* type (`&str`, gloss installed by lesson 055). The
  Book frames `char` as a *Unicode scalar value*, broad enough to
  cover accented letters, CJK characters, and emoji."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs` then `./name`; `fn main`
    body runs when the executable launches.
  - Lesson 003 (load-bearing): the four-part diagnostic map. The
    contrast probe is read with that map.
  - Lesson 005 (load-bearing): `let name = value;`. Today's bare
    form `let c = 'z';` reuses the shape unchanged; only the
    literal form is new.
  - Lesson 019 (load-bearing): `let name: TYPE = value;` is a
    *type annotation*. Today extends `TYPE` to `char`, just as
    lesson 062 extended it to `u32` and lesson 033 to `f64`.
  - Lesson 011 (cited): positional `{}` printing. The probe prints
    three `char` values with `{}`.
- Ordinary computer-use assumptions: same as lesson 001. The
  source file should be saved as UTF-8 if you include a non-ASCII
  literal — every modern editor does this by default.

## Try It

In a fresh empty directory, save `demo.rs`:

```rust
fn main() {
    let c = 'z';
    let letter: char = 'A';
    let math: char = 'ℤ';
    println!("c = {}", c);
    println!("letter = {}", letter);
    println!("math = {}", math);
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
c = z
letter = A
math = ℤ
```

Three uses of the new shape. `let c = 'z';` has no annotation — the
single-quote literal `'z'` is what tells rustc the value is a
`char`, so rustc infers the type. `let letter: char = 'A';` plugs
`char` into the lesson-019 annotation slot. `let math: char = 'ℤ';`
witnesses the Book's "lot more than just ASCII" claim.

Now the contrast. Save `broken.rs`:

```rust
fn main() {
    let c: char = "z";
    println!("c = {}", c);
}
```

The only change is `'z'` → `"z"` — single quotes to double. Compile
it. Read the headline with the lesson 003 map; full transcript in
`## Evidence`:

```text
error[E0308]: mismatched types
 --> broken.rs:2:19
  |
2 |     let c: char = "z";
  |            ----   ^^^ expected `char`, found `&str`
  |            |
  |            expected due to this
  |
help: if you meant to write a `char` literal, use single quotes
```

The headline is `error[E0308]: mismatched types`. The caret
underlines `"z"`; the inline annotation says `expected \`char\`,
found \`&str\``. The `help:` line is rustc telling the learner
exactly what this lesson installs: "if you meant to write a `char`
literal, use single quotes." With single quotes, `: char` binds;
with double quotes, the literal is a different type and rustc
rejects the program at compile time.

## What Changed

- One more primitive scalar type is named: `char`. The Book lists
  `char` alongside `i32`, `u32`, `f64`, and `bool` as Rust's
  primitive scalars.
- A `char` literal is one character between *single* quotes:
  `'z'`, `'A'`, `'ℤ'`. The single quotes do the typing work — a
  bare `let c = 'z';` infers `char` without any annotation.
- The lesson-019 `: TYPE` slot accepts `char`:
  `let letter: char = 'A';`. Shape unchanged; only the type name
  changes.
- Single quotes and double quotes are *not* interchangeable. `'z'`
  is a `char`; `"z"` is a string literal of the type lesson 055
  calls `&str`. Putting a double-quoted literal in a `: char` slot
  fails with `error[E0308]: mismatched types`, caret label
  `expected \`char\`, found \`&str\``, plus a `help:` line literally
  suggesting single quotes.
- Background facts, not load-bearing for any compile-or-run claim:
  a `char` is "4 bytes in size" (Book line 237) and represents
  "a Unicode scalar value" (Book and Reference). The Reference
  adds: same size and alignment as `u32`. These corroborate that
  `char` is a fixed-size scalar type.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let a: char = 'X';
    let b = '7';
    println!("a = {}, b = {}", a, b);
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) What is the type of `b`?

(c) If you changed line 3 to `let b = "7";` (annotation on `a`
unchanged), would `rustc tiny.rs` still accept the program?

(Answers: (a) Yes; prints `a = X, b = 7`. The literal `'7'` is a
`char`, not the integer `7`. (b) `char` — single-quote literal
infers to `char`. (c) Yes; `b` has no annotation, so the binding
takes the right side's type. The mismatch fires *only* when a
`char`-shaped slot meets a non-`char` value, as in `broken.rs`.)

## What To Ignore For Now

Today installs only the `char` type and the single-quoted character
literal. Each of the following is real and deferred:

- *Byte literals `b'A'`*. Table 3-2 lists `b'A'` as a separate
  integer-literal form: a `u8`, not a `char`. Item H in the Book
  Ch1-3 closure queue; separate move.
- *`&str` as a typed name*. Lesson 055 named `&str` incidentally as
  the return type of `String::trim` and the type of string literals.
  Today reuses that gloss for the contrast diagnostic; not
  re-installed as a centered concept.
- *Escape sequences inside `char` literals* — `'\n'`, `'\t'`,
  `'\u{1F600}'`. Each is a real `char` value; the escape-sequence
  vocabulary is its own move.
- *`char` methods* — `.is_alphabetic()`, `.to_lowercase()`. Trait
  machinery; a later move.
- *Integer-`char` casting* — `'A' as u32`, `97u8 as char`. Builds
  on lesson 034's `as` cast; separate move.
- *The `chars()` iterator on strings* — iterators are not installed.
- *The exact Unicode-scalar-value range* — `U+0000` to `U+D7FF` and
  `U+E000` to `U+10FFFF`, surrogate codepoints excluded. Today only
  observes that one ASCII letter and one non-ASCII letter work.
- *UTF-8 encoding details*. The Book defers UTF-8 to chapter 8.
- *The "4 bytes" size as load-bearing*. Today observes the Book's
  claim and the Reference's "same size and alignment as `u32`"
  corroboration; not used in any compile-or-run claim.

## Evidence

See `../evidence/074-char-type.md`.
