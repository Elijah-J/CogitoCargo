---
id: 072-tuple-type-and-index
status: accepted
evidence: ../evidence/072-tuple-type-and-index.md
---

# Tuple types with two or more fields, and field access by `.0`, `.1`

## The Move

A *tuple* is a fixed-length bundle of values written between
parentheses with commas: `(3, 7)` is a 2-tuple of integers; `(5, 2.5)`
is a 2-tuple whose two fields have different types. The *type* is
written with the same shape: `(i32, i32)` and `(i32, f64)`. Bind a
tuple with lesson 005's form plus lesson 019's annotation slot:
`let pair: (i32, i32) = (3, 7);`. Read a field by writing the bound
name, a `.`, and the field's *number*: `pair.0` is the first field,
`pair.1` is the second. The first index is `0`. Asking for a field
number that does not exist fails at compile time with
`error[E0609]: no field \`N\` on type \`(...)\``.

## Mental Model Delta

- Before: "A `let` binding holds one value of one type. Lesson 029
  named `()` as the unit type and flagged non-zero-arity tuples as
  a future move; I have not yet built or read any tuple with
  actual fields."
- After: "A single `let` binding can hold a fixed-arity bundle of
  values whose element types do not have to match. The type is
  `(T1, T2, ...)`; the value is `(v1, v2, ...)`. Each field has a
  numeric *name* matching its position; I read a field with
  `expr.N`. Asking for a field that does not exist is a compile-
  time error — tuples have fixed length and rustc knows it from
  the type."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs` then `./name`; `fn main`
    body runs when the executable launches; rustc silent on success.
  - Lesson 003 (load-bearing): rustc diagnostics have a headline +
    `-->` location + source excerpt with caret + optional
    `help:` / `= note:` lines. The broken-contrast walk decodes
    E0609 with that map.
  - Lesson 005 (load-bearing): `let name = value;` binds a name to
    a value. Today puts a tuple value on the right of that form.
  - Lesson 019 (load-bearing): `name: TYPE` attaches a type
    annotation. Today the `TYPE` slot is filled by `(i32, i32)`
    and `(i32, f64)` — the annotation form generalizes from
    `: i32` to `: (T1, T2, ...)` with no new `let` mechanism.
  - Lesson 029 (load-bearing): named `()` as the unit type, the
    0-arity tuple, and flagged "non-zero-arity tuple types
    (`(i32, i32)`, `(f64, String)` etc.)" as a deferred future
    move. Today is exactly that move.
  - Lessons 011, 033 (cited): `println!("{}", ...)` reused as-is.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let pair: (i32, i32) = (3, 7);
    let triple = (10, 20, 30);
    let mixed: (i32, f64) = (5, 2.5);

    let first = pair.0;
    let second = pair.1;

    println!("pair = ({}, {})", first, second);
    println!("triple.2 = {}", triple.2);
    println!("mixed = ({}, {})", mixed.0, mixed.1);
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
pair = (3, 7)
triple.2 = 30
mixed = (5, 2.5)
```

Three pieces are new. The right-hand side of each `let` is a *tuple
expression* — parenthesized comma-separated values. The annotations
`: (i32, i32)` and `: (i32, f64)` fill lesson 019's `TYPE` slot with
a *tuple type* (same parens-and-commas shape as the value); `triple`
omits the annotation, so rustc infers `(i32, i32, i32)` from three
integer literals; `mixed` shows element types do not have to match
(the Book calls this *heterogeneous*). `pair.0`, `pair.1`,
`triple.2`, `mixed.0`, `mixed.1` are *tuple indexing expressions*:
bound name, `.`, plain decimal number — no leading zero, no
underscore, no type suffix. Counting starts at `0`.

Now the contrast. In the same directory, save `broken.rs`:

```rust
fn main() {
    let pair: (i32, i32) = (3, 7);
    let bad = pair.2;
    println!("{}", bad);
}
```

Compile it. The full transcript is in `## Evidence`; the headline is
`error[E0609]: no field \`2\` on type \`(i32, i32)\``, the caret
underlines the `2` with the inline annotation `unknown field`, and a
`= note:` line says `available fields are: \`0\`, \`1\``. rustc
*enumerates* the legal field names — the audience-level statement
that the tuple has fixed length: the type `(i32, i32)` has exactly
two fields, named `0` and `1`, and rustc knows both at compile time.
The trailer `For more information about this error, try \`rustc
--explain E0609\`.` follows the coded-headline pattern of lesson 070.

The contrast is the lesson's "with X works, without X fails"
witness: with a field number that exists on the tuple's type, the
program builds; with one that does not, rustc rejects at compile
time and lists the legal names.

## What Changed

- A `let` binding can hold a tuple — a fixed-arity bundle of values
  whose element types do not have to match. Type: `(T1, T2, ...)`.
  Value: `(v1, v2, ...)`. Lesson 029's `()` is the 0-arity case.
- Read a field by position with `.0`, `.1`, `.2`, ... The index is
  a plain decimal number; counting starts at `0`.
- Asking for a field that does not exist on the tuple's type fails
  at compile time with `error[E0609]: no field \`N\` on type
  \`(...)\``, with a `= note:` line listing the legal field names.
  rustc knows the names from the type itself.

## Check Yourself

You write `tiny.rs` containing:

```rust
fn main() {
    let t: (i32, i32, f64) = (1, 2, 3.5);
    println!("{} {} {}", t.0, t.1, t.2);
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) Now change the `println!` to `println!("{}", t.3);`. Predict the
rustc headline and the `= note:` line without running anything.

(c) Why is part (b)'s problem caught by `rustc` rather than by the
running program?

(Answers: (a) Yes; prints `1 2 3.5`. (b) `error[E0609]: no field
\`3\` on type \`(i32, i32, f64)\``, with `= note: available fields
are: \`0\`, \`1\`, \`2\``. (c) The tuple type has fixed length `3`
and rustc knows that length at compile time from the type itself,
so `t.3` is rejected before producing an executable.)

## What To Ignore For Now

- *Pattern destructuring on the left of `let`* — `let (a, b) =
  pair;` binds `a` to `pair.0` and `b` to `pair.1`. This is the
  explicit next move (deferred-queue Q06). The Book's
  `ch03-02-data-types.md` lines 277-285 show the form; today does
  *not* install it. Today only installs construction with `(...)`
  and access with `.N`.
- *1-ary tuples* and the trailing-comma rule — the Reference says
  `(5)` is a parenthesized expression of type `i32`, not a 1-tuple,
  while `(5,)` of type `(i32,)` is the 1-tuple form. Today's probe
  uses 2-tuples and 3-tuples to sidestep this; the rule is real and
  deferred.
- *Tuple structs* — `struct Point(f32, f32);` plus `Point(1.0, 2.0)`
  as a constructor. Same `.N` access shape, different declaration
  with its own rules. Future move.
- *Tuple patterns in `match`* — `match pair { (0, _) => ..., ... }`.
  Lessons 030, 031, 058 used `match` with simple patterns; tuple
  patterns are a separate future move.
- *Tuple indexing as a place expression* — `pair.0 = 9;` on a
  `mut` binding, and nested field access like `pairs.0.1`. Today
  only reads fields, never writes them.
- *Tuples larger than three fields*, and the upper-arity-limit
  policy — std implements certain traits only up to a limited
  arity. Trait machinery is not installed in this run; deferred.
- *Whether `pair.0 + pair.1` works* — it does (the two fields are
  `i32`; lesson 009 installed `+` on integers). Today's probe
  binds the fields to `first` and `second` first to keep one move
  per lesson.
- *Method calls on tuples* — none useful at audience level. No
  `.len()`, for example, because the length is baked into the
  type. Not installed.

## Evidence

See `../evidence/072-tuple-type-and-index.md`.
