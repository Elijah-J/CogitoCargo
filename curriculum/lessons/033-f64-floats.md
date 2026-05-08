---
id: 033-f64-floats
move: "bind a floating-point number with `let x: f64 = 5.0 / 3.0;` instead of an integer; the result is `1.6666666666666667`, not `1` -- float division does not truncate the way integer division does"
main_concept: "`f64` is Rust's *floating-point* type -- numbers with decimal points; a *float literal* is written with a `.` (e.g. `5.0`, `3.14`, `1.0 / 2.0`); the default float type for a literal like `5.0` is `f64`; lesson 009's `+`, `-`, `*`, `/` operators all work on floats too, but `/` does NOT truncate when both operands are floats -- `5.0 / 3.0` evaluates to roughly `1.6666666666666667`; float and integer types are distinct, and rustc rejects `let x: f64 = 3;` (integer literal in `f64` slot) with E0308 plus a literal `help: use a float literal` source-diff suggesting `3.0`"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 005-let-binding
  - 009-arithmetic-on-integers
  - 019-type-annotation-i32
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "`f32` (32-bit float)" moves
  - future "`as` casts between numeric types" moves
  - future "other integer types `i64`/`u32`/`usize`" moves
  - future "IEEE 754 / NaN / infinity" moves
  - future "float comparison gotchas" moves
  - future "format specifiers like `{:.2}`" moves
  - future "float literal suffixes / scientific notation" moves
sources:
  - output/docs/rust/book/ch03-02-data-types.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/033-f64-floats.rs
  - broken-contrast probe transcript inline in lesson `## Evidence` (not committed)
status: accepted
---

# Bind a floating-point number with `let x: f64 = 5.0 / 3.0;`

## The Move

Inside `fn main`, write a `let` of the lesson-019 shape, but replace
`i32` with `f64` and replace the integer literals on the right with
*float literals* -- numbers written with a `.`:

```rust
let x: f64 = 5.0 / 3.0;
```

Print it with `println!("x = {x}");`. The output is
`x = 1.6666666666666667`, not `x = 1`. Same operator `/`, different
type, different result.

## Mental Model Delta

- Before: "Numbers in Rust are integers (`i32` from lesson 019). `/`
  between integers truncates toward zero (lesson 009: `5 / 3` is `1`).
  That is the only kind of number I have."
- After: "Rust has a second, separate kind of number: *floating-point*
  numbers, written with a `.` (`5.0`, `3.14`). The default
  floating-point type is `f64`. Lesson 009's `+`, `-`, `*`, `/`
  operators all work on floats, but `/` does *not* truncate -- `5.0 /
  3.0` is `1.6666666666666667`. Integer types and float types are
  distinct: rustc will reject `let x: f64 = 3;` because `3` is an
  integer literal, not a float literal."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`, silent on success.
  - Lesson 002: body of `fn main` runs when the executable launches.
  - Lesson 003 (load-bearing): rustc diagnostics have a headline +
    `-->` location + source excerpt with caret + optional `help:`
    lines. The broken-contrast walk below decodes E0308 with a
    `help: use a float literal` source-diff using exactly this skill.
  - Lesson 005: `let name: TYPE = value;` binds a name; reused as the
    slot the float value lands in.
  - Lesson 009 (load-bearing): `+`, `-`, `*`, `/` on integers; the
    truncation rule "`5 / 3` is `1`" is exactly what this lesson
    contrasts against. The probe shows `5 / 3 = 1` and `5.0 / 3.0 =
    1.6666666666666667` side by side.
  - Lesson 019 (load-bearing): `name: TYPE` attaches a type. The probe
    annotates `let int_div: i32 = ...;` and `let float_div: f64 =
    ...;`. The broken-contrast diagnostic uses the annotation
    `let pi: f64` to phrase the error.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let int_div: i32 = 5 / 3;
    let float_div: f64 = 5.0 / 3.0;
    println!("int_div = {int_div}");
    println!("float_div = {float_div}");
}
```

Two `let` statements. Both share the lesson-019 shape `let name: TYPE =
value;` and both put `5 / 3`-shaped arithmetic on the right. The only
real differences are the type (`i32` vs `f64`) and how the literals are
spelled (`5` vs `5.0`).

Compile and run:

```console
$ rustc demo.rs
$ ./demo
int_div = 1
float_div = 1.6666666666666667
```

Walk the two lines.

- `let int_div: i32 = 5 / 3;` is lesson 009 with a lesson 019
  annotation. Both operands are integer literals, so `/` is integer
  division and truncates toward zero. `5 / 3` would be `1.66...` in
  ordinary arithmetic; the bound value is `1`.
- `let float_div: f64 = 5.0 / 3.0;` is the new shape. The literals
  `5.0` and `3.0` are *float literals* -- numbers written with a `.`.
  Their default type is `f64`. The `/` operator works on floats too,
  but on floats it does *not* truncate. The bound value prints as
  `1.6666666666666667`.

Same operator `/`, same arithmetic shape `5 / 3`, two different types,
two different results. That is the load-bearing contrast.

The exact digits `1.6666666666666667` are an artifact of `f64`'s
64-bit binary representation and Rust's default float formatting. The
real value of 5 divided by 3 has infinitely many digits; `f64` stores
a binary approximation of it, and `println!` prints however many of
those digits the default formatter chose. Treat the printed value as
"`1.66...`-ish" for now; the precise binary-format details are
deferred.

Now do the contrast. In the same directory, save a second file
`broken.rs` that *omits* the `.0` on the right -- an integer literal in
an `f64` slot:

```rust
fn main() {
    let pi: f64 = 3;
    println!("pi = {pi}");
}
```

Compile it. The full transcript and a part-by-part walk live in
`## Evidence`; reading it with lesson 003's order:

- *Headline*: `error[E0308]: mismatched types`. Same E-code seen in
  lessons 024-031 whenever a type expectation clashes with what was
  actually written.
- *`-->` location*: `broken.rs:2:19` -- column 19 of line 2, the
  literal `3` itself.
- *Source excerpt*: dashes `---` underline `f64` with the
  sub-annotation `expected due to this`; carets `^` underline `3` with
  the trailing annotation `expected f64, found integer`. So rustc says
  the *annotation* `: f64` set the expectation and the *literal* `3`
  did not match.
- *`help:` block*: `help: use a float literal`, followed by a literal
  source diff showing the line rewritten as `let pi: f64 = 3.0;` with
  `++` under the new `.0`.

That `help:` line is rustc encoding exactly what this lesson installs:
to land in an `f64` slot, write the literal with a `.`. The literal
`3` is an integer; the literal `3.0` is a float. The diagnostic
*states* the rule (`use a float literal`) and *shows* the smallest
fix as a one-character source diff.

Same `help:`-with-source-diff shape as lesson 028's `break 42;`
suggestion and lessons 030/031's `false => todo!()` arm-fillers. Same
skill, new instance.

## What Changed

- You can write `let name: f64 = value;` to bind a *floating-point*
  number. The `f64` slot accepts *float literals* -- numbers written
  with a `.`, like `5.0` or `3.14`.
- Lesson 009's operators `+`, `-`, `*`, `/` all work on floats too,
  with one big difference: `/` between two floats does *not* truncate.
  `5.0 / 3.0` evaluates to roughly `1.6666666666666667`, not `1`.
- Integer types (`i32`) and float types (`f64`) are distinct. An
  integer literal like `3` does not fit in an `f64` slot; rustc
  rejects it with `error[E0308]: mismatched types` and a
  `help: use a float literal` source-diff suggesting `3.0`.
- For ordinary fractional values like `5.0`, `3.14`, or `1.0 / 2.0`,
  `f64` is the default. Rust also has a 32-bit float type called
  `f32`; this lesson does not install it. See What To Ignore For Now.
- The printed value `1.6666666666666667` reflects `f64`'s 64-bit
  binary precision and Rust's default float formatting. The exact
  digits depend on IEEE 754 rounding; treat them as approximate for
  now.

## Check Yourself

You write `frac.rs` containing:

```rust
fn main() {
    let a: i32 = 7 / 2;
    let b: f64 = 7.0 / 2.0;
    println!("a = {a}");
    println!("b = {b}");
}
```

You run `rustc frac.rs && ./frac`.

(a) What two lines does the executable print, in order?

(b) Why is the first line not `a = 3.5`?

(c) If you change `let b: f64 = 7.0 / 2.0;` to `let b: f64 = 7 / 2;`
(omitting the `.0` on both literals) -- which line does rustc's `-->`
location point at, and what does the `help:` line literally say?

(Answers: (a) `a = 3` then `b = 3.5`. (b) `a` is annotated `i32`, both
operands are integer literals, so `/` is integer division and
truncates toward zero -- `7 / 2` would be `3.5`, so truncating gives
`3`. (c) The `-->` location points at the right-hand side of the
broken `let` (the `7 / 2` expression). The `help:` line says
`help: use a float literal`, with a source-diff suggestion adding
`.0` to one or both literals so the right-hand side has the float
type the `: f64` slot expects.)

## What To Ignore For Now

This lesson installs only one new type (`f64` with float literals
written using a `.`) and one operator-behavior contrast (`/` on floats
does not truncate). Each of the following is real but *not* part of
this move:

- *`f32`*, the 32-bit float type. Rust's other primitive
  floating-point type, mentioned alongside `f64` in the corpus
  (`output/docs/rust/book/ch03-02-data-types.md` lines 147-151). This
  lesson installs only `f64`, the default; `f32` is its own future
  move.
- *Mixing integer and float arithmetic* like `let x: f64 = 1 + 2.0;`.
  Rust does not implicitly convert between numeric types; rustc
  rejects the mix with its own diagnostic shape. Future move.
- *Numeric type conversion via `as` casts* (`5 as f64`, `3.14 as
  i32`). The standard way to bridge between integer and float values
  when you actually want to mix them. Future move.
- *Other integer types* (`i64`, `u32`, `usize`, etc.). Deferred from
  lesson 019.
- *Float arithmetic peculiarities* -- IEEE 754 rounding, the special
  values `NaN` and `infinity`, what `0.0 / 0.0` produces, and why
  `NaN == NaN` is `false`. Real and important; their own teaching
  surface; future move.
- *Integer overflow* (`i32::MAX + 1` panics in debug, wraps in
  release). Floats don't overflow in this way -- they go to `infinity`
  instead. Different model; future move.
- *Format specifiers for floats* like `{:.2}` for "two decimal
  places". The default-formatted output `1.6666666666666667` is what
  this lesson uses; controlling how many digits print is its own
  future move.
- *Float literal suffixes* like `2.0_f32`, scientific notation like
  `3.14e2`, hex floats. Future moves.
- All previously deferred items: shadowing, `&` references,
  closures, generics, modules and `pub`, the broader format-string
  DSL.

## Evidence

### Sources

- `output/docs/rust/book/ch03-02-data-types.md`, the
  *Floating-Point Types* subsection (lines 145-165). Two load-bearing
  direct quotes:

  Lines 147-151 (the canonical introduction):

  > Rust also has two primitive types for *floating-point numbers*,
  > which are numbers with decimal points. Rust's floating-point
  > types are `f32` and `f64`, which are 32 bits and 64 bits in size,
  > respectively. The default type is `f64` because on modern CPUs,
  > it's roughly the same speed as `f32` but is capable of more
  > precision. All floating-point types are signed.

  This grounds: the *floating-point* name, the "numbers with decimal
  points" framing, the existence of `f32` and `f64`, and the claim
  that `f64` is the default float type.

  Lines 158-162 (the Book's example):

  > ```rust
  > fn main() {
  >     let x = 2.0; // f64
  >
  >     let y: f32 = 3.0; // f32
  > }
  > ```

  This grounds the claim that an unsuffixed float literal like `2.0`
  has default type `f64` (so `let x = 2.0;` infers `f64`), and that
  `f32` requires an explicit annotation. This lesson's probe sticks
  to `f64` and uses an explicit `: f64` annotation for symmetry with
  lesson 019's `: i32` annotation; the underlying default-type rule
  is the Book's.

  Plus the existing lesson 009 quote (Book lines 169-171, "Numeric
  Operations"): "Rust supports the basic mathematical operations
  you'd expect for all the number types: addition, subtraction,
  multiplication, division, and remainder. Integer division
  truncates toward zero to the nearest integer." The phrase "for all
  the number types" is what licenses applying `+`, `-`, `*`, `/` to
  floats too; the qualifier "Integer division truncates toward zero"
  -- explicit about *integer* division -- implies the qualifier does
  not apply to float division. The probe directly confirms this:
  `5.0 / 3.0` does not truncate.

  Calibration:

  - The Book introduces both `f32` and `f64` together. This lesson
    installs `f64` only (the default and most common); `f32` is
    explicitly mentioned and deferred under What To Ignore For Now.
  - The Book's example uses untyped `let x = 2.0;` and lets type
    inference pick `f64`. This lesson uses explicit
    `let float_div: f64 = ...;` annotation for symmetry with lesson
    019's `i32` annotation pattern. Both work; the explicit
    annotation makes the type-check verifiable in the probe and lets
    the broken-contrast diagnostic phrase the error in terms of the
    annotation.
  - The printed value `1.6666666666666667` reflects `f64`'s 64-bit
    binary precision (IEEE 754 double precision, mentioned at Book
    line 165: "Floating-point numbers are represented according to
    the IEEE-754 standard") and Rust's default float formatting. The
    exact decimal digits printed depend on IEEE 754 rounding; this
    lesson mentions this in passing and defers the detail.

- The local probes (working + broken-contrast), captured below.

### Probes

Two probes were captured on rustc 1.95.0 (59807616e 2026-04-14) on
Darwin x86_64. The working probe is committed at
`experimental/eduratchet2/runs/rust-moves/observations/033-f64-floats.rs`.
The broken-contrast probe is *not* committed under `observations/`;
its transcript is reproduced verbatim below.

Both probes were run in temp directories created with `mktemp -d` and
removed at the end.

#### Working probe

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let int_div: i32 = 5 / 3;
    let float_div: f64 = 5.0 / 3.0;
    println!("int_div = {int_div}");
    println!("float_div = {float_div}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
int_div = 1
float_div = 1.6666666666666667
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (consistent with lesson 001).
- The first output line `int_div = 1` reproduces lesson 009's
  integer-division-truncates result: `5 / 3` would be `1.66...`,
  truncating to `1`.
- The second output line `float_div = 1.6666666666666667` is the
  *new* observation: same arithmetic shape, different type, no
  truncation. The exact digits `1.6666666666666667` are determined
  by `f64`'s 64-bit binary precision and Rust's default float
  formatting; treat them as `1.66...`-ish.
- Only the working source is committed under `observations/`. No
  binaries are committed. The temp dir was removed.

#### Broken-contrast probe

`broken.rs` is identical in shape to a single annotated `let` but
puts an *integer* literal in an `f64` slot. Not committed; the
transcript below is the artifact.

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
broken.rs
--- cat broken.rs ---
fn main() {
    let pi: f64 = 3;
    println!("pi = {pi}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:2:19
  |
2 |     let pi: f64 = 3;
  |             ---   ^ expected `f64`, found integer
  |             |
  |             expected due to this
  |
help: use a float literal
  |
2 |     let pi: f64 = 3.0;
  |                    ++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
--- temp dir removed ---
```

Notes:

- The headline `error[E0308]: mismatched types` is the generic E0308
  form (same E-code as lessons 024, 025, 028). The `--explain E0308`
  trailer is also present, consistent with lesson 003.
- The `--> broken.rs:2:19` location points at column 19 of line 2 --
  the literal `3` on the right of the `=`.
- The source excerpt traces the type expectation across the *one*
  line: `---` underlines `f64` with the sub-annotation `expected due
  to this`; `^` underlines `3` with the trailing annotation
  `expected f64, found integer`. Reading this with lesson 003: the
  annotation `: f64` set the expectation; the integer literal `3`
  is what failed to match.
- The `help:` block uses the same shape as lesson 028's
  `give the break a value of the expected type` and lessons
  030/031's arm-filler suggestions:
  - The headline-style line `help: use a float literal` states the
    fix in English.
  - Below it, indented under the `|`, rustc shows a literal source
    diff: `2 |     let pi: f64 = 3.0;` with `++` under the new `.0`,
    showing exactly what to add.
- The named *kind* `integer` (in `expected f64, found integer`) is
  rustc's umbrella label for an unconstrained integer literal whose
  specific type it has not yet pinned down. The broken `let` here
  did not give rustc enough information to pick a concrete integer
  type, so it reports the kind rather than a specific name like
  `i32`. Treat it as "the `3` was an integer, and the slot wanted a
  float". A future move will install the kind/type distinction.
- Exit code: 1. No executable was produced.
- The pedagogical point: rustc itself encodes the lesson's rule. An
  integer literal does not fit in an `f64` slot, and the suggested
  fix is exactly the syntactic move this lesson installs -- write
  the literal with a `.` so it is a float literal.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) -- `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) -- body of `fn main` runs
  when the executable launches.
- `003-read-rustc-diagnostic` (accepted, load-bearing) -- diagnostics
  have a headline + `-->` location + source excerpt with caret +
  optional `help:` lines. The broken-contrast walk above decodes the
  E0308 diagnostic, including its source-diff `help:` block, using
  exactly this skill.
- `005-let-binding` (accepted) -- `let name: TYPE = value;` binds a
  name; reused as the slot the float value lands in.
- `009-arithmetic-on-integers` (accepted, load-bearing) -- `+`, `-`,
  `*`, `/` on integers; *integer division truncates toward zero so
  `5 / 3` is `1`*. The probe's first line directly reproduces this
  result; the lesson's contrast is "the same operator does *not*
  truncate on floats".
- `019-type-annotation-i32` (accepted, load-bearing) -- `name: TYPE`
  attaches a type. The probe annotates two `let` lines, one with
  `: i32` and one with `: f64`, side by side. The broken-contrast
  diagnostic uses the annotation `let pi: f64` to phrase the error
  (`expected due to this` underlining `f64`).
- Older lessons (mention only): lesson 011 (`println!` positional
  args; the probe uses one named placeholder per `println!` for
  clarity), lessons 023, 026, 028, 030, 031 (no comparisons, compound
  assigns, or `match` arms appear in this lesson's probe; the
  *`help:` source-diff* shape used by the broken-contrast diagnostic
  follows the same pattern as 028's `break 42;` and 030/031's
  `false => todo!()`). Cited as familiar-shape, not load-bearing on
  their own.
