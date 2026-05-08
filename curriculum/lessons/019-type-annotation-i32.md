---
id: 019-type-annotation-i32
move: "write `let name: i32 = value;` to annotate a binding with the type `i32`"
main_concept: "every Rust value has a *type* that says what kind of value it is; Rust is statically typed, so rustc must know each binding's type at compile time, but it usually *infers* the type from the value (integer literals default to `i32`); when you want to be explicit, you write `let name: TYPE = value;` -- a *type annotation* between the name and the `=`"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
  - 005-let-binding
  - 009-arithmetic-on-integers
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "other integer types i8/u32/etc" moves
  - future "floating-point types f32/f64" moves
  - future "type errors E0308" moves
  - future "function parameters with types" moves
  - future "function return types" moves
sources:
  - output/docs/rust/book/ch03-02-data-types.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/019-type-annotation-i32.rs
status: accepted
---

# Annotate a `let` binding with the type `i32`

## The Move

Inside `fn main`, write a `let` statement of the shape
`let name: i32 = value;`. The piece between `name` and `=` -- a colon
followed by `i32` -- is a *type annotation*. It tells `rustc` exactly
what kind of value the name holds. For ordinary integer literals like
`5`, the annotation is optional, because `rustc` already infers `i32`
from the value; but you can write it for clarity, and you have to write
it whenever the inferred type would be wrong or ambiguous.

## Mental Model Delta

- Before: "A `let` line names a value. I have not had to say *what kind*
  of value it is. The integer `5` is just `5`."
- After: "Every value in Rust has a *type* that says what kind of value
  it is. Rust is *statically typed*, so `rustc` must know each
  binding's type at compile time. Most of the time it figures the type
  out from the value -- an integer literal like `5` defaults to `i32`.
  When I want to be explicit I add a *type annotation* between the
  name and the `=`: `let name: i32 = 5;`. That is added information
  attached to the lesson-005 binding form, not a new mechanism."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`; `rustc` is silent on
    success.
  - Lesson 002: the body of `fn main` runs when the executable
    launches.
  - Lesson 004: statements in `fn main` run top to bottom.
  - Lesson 005 (load-bearing): `let name = value;` binds a name to a
    value, and `println!("... {name} ...");` substitutes the bound
    value at print time. This lesson adds an optional `: TYPE` slot
    between `name` and `=`; everything else about `let` is unchanged.
  - Lesson 009 (cited): `+` between two integer values produces a new
    integer value that fits on the right of `let`. The probe uses
    `x + y` on the right of one annotated `let`.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let x: i32 = 5;
    let y = 10;
    let sum: i32 = x + y;
    println!("x = {x}, y = {y}, sum = {sum}");
}
```

Three `let` statements live inside `fn main`:

- `let x: i32 = 5;` is the new shape. The `: i32` between the name and
  the `=` is the *type annotation*. It says `x` holds a value of type
  `i32`.
- `let y = 10;` is the bare lesson-005 shape, with no annotation. It
  still works: the Book says the compiler "can usually infer what type
  we want to use based on the value and how we use it", and an integer
  literal like `10` defaults to `i32`.
- `let sum: i32 = x + y;` puts the annotation on a `let` whose right
  side is the arithmetic expression `x + y` from lesson 009. Because
  `x` and `y` are both `i32`, `x + y` is `i32`, so the annotation
  matches what `rustc` would have inferred. It compiles and runs the
  same as `let sum = x + y;`.

Compile and run:

```console
$ rustc demo.rs
$ ./demo
x = 5, y = 10, sum = 15
```

One line of output. The contrast lives *inside this one program*:
`x` and `sum` are annotated, `y` is not, and all three end up as
`i32`. The annotation is added information, not a new mechanism.

## What Changed

- You can attach a type to a `let` binding by writing
  `let name: TYPE = value;`. For ordinary whole-number values the
  `TYPE` slot is `i32`.
- You have a working noun: every value has a *type*. Rust is
  *statically typed*; `rustc` must know each binding's type at compile
  time.
- You know one default: integer literals like `5` and `10` default to
  `i32`, so `let n = 5;` and `let n: i32 = 5;` produce the same
  binding -- one infers, the other states it explicitly.
- The bare lesson-005 form `let name = value;` still works exactly as
  before; the annotation is an optional addition.

## Check Yourself

You write `tiny.rs` containing:

```rust
fn main() {
    let a: i32 = 7;
    let b = 4;
    let total: i32 = a + b;
    println!("total = {total}");
}
```

You run `rustc tiny.rs` and then `./tiny`.

- Does `rustc` reject the program for any of the three `let` lines?
- What does the executable print?
- Which `let` lines carry an explicit annotation, and which lets
  `rustc` infer the type?

(Answers: `rustc` accepts the program, exits 0, and is silent. The
executable prints `total = 11`. The first and third lines carry the
`: i32` annotation; `let b = 4;` has none and lets `rustc` infer
`i32`.)

## What To Ignore For Now

This lesson installs only one idea: every value has a type, `rustc`
usually infers it, and you can write it explicitly with
`let name: TYPE = value;`; for integer literals the type name is
`i32`. Each of the following is real and will be taught later, but is
*not* part of this move:

- *Other integer types*. The Book lists a whole table: `i8`, `u8`,
  `i16`, `u16`, `i64`, `u64`, `i128`, `u128`, `isize`, `usize`. They
  differ in size and in whether they can hold negative values. This
  lesson only uses `i32`; the rest are deferred.
- *Floating-point types* `f32` and `f64`, for numbers with a
  fractional part. Different family; deferred.
- *Other types*. `bool` (`true` / `false`, lesson 012), `char`,
  strings, tuples, arrays, structs, enums, references. Each has its
  own type and will get its own move.
- *Generic types* and *type parameters* (`Vec<T>`, `Option<T>`, etc.).
  Deferred.
- *Type errors*. A wrong annotation -- for example binding an integer
  literal to a name annotated as a non-integer type -- makes `rustc`
  refuse the program with a diagnostic such as
  `error[E0308]: mismatched types`. The probe here only uses correct
  annotations; the diagnostic shape is deferred.
- *The internals of type inference vs type checking*. We say "`rustc`
  figures out the type from the value" without unpacking the
  algorithm.
- All previously deferred items: `mut`, shadowing, the broader
  format-string DSL, defining your own functions with parameters and
  return types, `cargo`, etc.

## Evidence

### Sources

- `output/docs/rust/book/ch03-02-data-types.md`. Three load-bearing
  passages:
  - Lines 4-6: "Every value in Rust is of a certain *data type*, which
    tells Rust what kind of data is being specified so that it knows
    how to work with that data." This is the corpus source for the
    lesson's "every value has a type" framing.
  - Lines 8-14: "Rust is a *statically typed* language, which means
    that it must know the types of all variables at compile time. The
    compiler can usually infer what type we want to use based on the
    value and how we use it. In cases when many types are possible,
    such as when we converted a `String` to a numeric type using
    `parse` ..., we must add a type annotation, like this:" followed
    by `let guess: u32 = "42".parse().expect("Not a number!");`. This
    is the corpus source for "statically typed", "the compiler can
    usually infer", and the `let name: TYPE = value;` annotation form.
  - Lines 56-72: the "Integer Types" introduction and Table 3-1.
    Direct quote from line 56: "An *integer* is a number without a
    fractional component." The table lists `i32` as the 32-bit signed
    integer type; that is the type name this lesson uses. Direct
    quote from line 110: "Integer types default to `i32`." This is the
    corpus source for "for plain integer literals like `5`, the
    inferred type is `i32`."

  Calibration: the Book motivates annotations with
  `let guess: u32 = "42".parse().expect("Not a number!");`, a case
  where the compiler genuinely cannot infer the type because `parse`
  can return many numeric types. This lesson uses simpler integer
  literals where the inferred type is determinate (`i32` by default),
  so the annotation is presented as *explicitness/documentation*
  rather than a parse-disambiguation necessity. The motivation
  difference is honest: the syntax form `let name: TYPE = value;` is
  identical to the Book's, only the surrounding "why annotate" story
  is simpler here.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/019-type-annotation-i32.rs`.
The committed file is the working program. There is no separate
broken-contrast file; the load-bearing observation is that an
annotated and an unannotated `let` line coexist in the same compiled
program with identical runtime behavior.

Probe transcript, run in a temp directory created with `mktemp -d`
and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let x: i32 = 5;
    let y = 10;
    let sum: i32 = x + y;
    println!("x = {x}, y = {y}, sum = {sum}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
x = 5, y = 10, sum = 15
exit=0
```

Notes:

- `rustc` exits 0 and is silent (consistent with lesson 001).
- The single output line is `x = 5, y = 10, sum = 15`. The values come
  from the three bindings: `x` (annotated `: i32`), `y` (unannotated,
  inferred `i32`), and `sum` (annotated `: i32`, right-side
  expression `x + y` from lesson 009).
- That all three bindings compile under one `rustc` invocation, with
  no diagnostic on the unannotated `y`, is the load-bearing
  observation: the annotation is added information, not a replacement.
- Only the working source is committed under `observations/`. No
  binaries are committed. The temp dir was removed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) -- the `rustc file.rs` then
  `./name` workflow used by the probe.
- `002-fn-main-entry-point` (accepted) -- body of `fn main` runs when
  the executable launches.
- `004-statements-in-order` (accepted) -- the body of `fn main` is a
  sequence of `;`-terminated statements that run top to bottom; this
  is what makes "the `println!` on the last line sees all three
  bindings made above" concrete.
- `005-let-binding` (accepted, load-bearing) -- `let name = value;`
  binds a name to a value, and `println!("... {name} ...");`
  substitutes the bound value at print time. This lesson adds an
  optional `: TYPE` slot between `name` and `=`; the rest of the
  binding form is unchanged.
- `009-arithmetic-on-integers` (accepted, cited) -- `+` between two
  integer values produces a new integer value that fits on the right
  of `let`. The probe uses `x + y` on the right of an annotated
  `let sum: i32 = x + y;`.
