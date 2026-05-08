---
id: 088-f32-floating-point
status: accepted
evidence: ../evidence/088-f32-floating-point.md
---

# Bind a 32-bit float with `let small: f32 = 3.0;`

## The Move

Lesson 033 installed *one* floating-point type: `f64`. The Book says
Rust has *two*. Today names the second.

Inside `fn main`, write the lesson-033 binding shape but with `f32`
where the annotation said `f64`:

```rust
let small: f32 = 3.0;
```

Same `: TYPE` annotation slot from lesson 019. Same float literal
shape (`3.0`) from lesson 033. The only edit is the type name —
`f32` instead of `f64`. The literal `3.0` lands at `f32` because
the annotation pins it there.

## Mental Model Delta

- *Before:* "There is one floating-point type in Rust: `f64`. A
  literal like `2.0` is `f64`. That's the only float type."
- *After:* "There are *two* floating-point types: `f32` (32 bits)
  and `f64` (64 bits). The unsuffixed float literal default is still
  `f64` (lesson 033 — that's why `let x = 2.0;` is `f64`). To bind a
  literal at `f32` instead, use the `: f32` annotation. Both types
  are signed. `f64` has more precision; `f32` exists for cases where
  32 bits is enough or memory matters. Both follow the IEEE-754
  standard."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs` then `./name`; rustc silent on
    success.
  - Lesson 003 (cited): the four-part diagnostic map; the contrast
    probe is read with it.
  - Lesson 005 (cited): `let name = value;`.
  - Lesson 011 (cited): `println!("{}", expr)` positional printing.
  - Lesson 019 (load-bearing): `: TYPE` is the *type annotation*
    slot. Today plugs `f32` into the same slot the way 033 plugged
    `f64`.
  - Lesson 033 (load-bearing): `f64` as the floating-point type;
    *float literal* written with a `.`; the unsuffixed-float-literal
    default is `f64`. Today extends 033 from one float type to the
    family of two, default rule unchanged. Lesson 033's *What To
    Ignore For Now* named "*`f32`*, the 32-bit float type" verbatim
    as deferred; today closes that line.
  - Lesson 080 (cited): the integer family is twelve names by
    sign-and-width. The float family is the same shape, smaller —
    two names by bit width, both signed.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

In a fresh empty directory, save `demo.rs`:

```rust
fn main() {
    let big = 2.0;        // f64 by default
    let small: f32 = 3.0; // f32 via annotation
    println!("big = {}", big);
    println!("small = {}", small);
}
```

This is the Book's canonical example for this section, verbatim.
Two `let` statements: the first has *no* annotation, so rustc
infers `f64` from the unsuffixed float literal default (lesson
033). The second has `: f32`, which pins the literal at `f32`.

Compile and run:

```console
$ rustc demo.rs
$ ./demo
big = 2
small = 3
```

Both bindings compile (no errors, no warnings) and the program
prints two lines. The default `{}` formatter prints whole-valued
floats without a trailing decimal — `2.0` prints as `2`, `3.0`
prints as `3`. The displayed digits happen to look integer-shaped
here, but the *types* are `f64` and `f32`. The contrast probe below
confirms: rustc rejects an integer literal in the `f32` slot just
the way it rejected one in the `f64` slot for lesson 033.

Now the contrast. In the same directory, save `broken.rs` that
*omits* the `.0` on the right — an integer literal in an `f32`
slot:

```rust
fn main() {
    let small: f32 = 3;
    println!("small = {}", small);
}
```

Read with the lesson 003 diagnostic map; full transcript in the
evidence appendix:

```text
error[E0308]: mismatched types
 --> broken.rs:2:22
  |
2 |     let small: f32 = 3;
  |                ---   ^ expected `f32`, found integer
  |                |
  |                expected due to this
  |
help: use a float literal
  |
2 |     let small: f32 = 3.0;
  |                       ++
```

Identical shape to lesson 033's `f64 = 3` diagnostic, except
`f32` replaces `f64`. The `help: use a float literal` line is
rustc itself encoding the rule: to land in *either* float slot, the
literal must be a *float* literal (written with a `.`). The
lesson-033 rule generalizes to the float family.

## What Changed

- Rust has *two* floating-point primitives: `f32` (32 bits) and
  `f64` (64 bits). Both are signed (Book lines 147-151).
- The unsuffixed float literal default is still `f64` (lesson 033
  unchanged). To bind a literal at `f32`, write the `: f32`
  annotation: `let small: f32 = 3.0;` is the canonical form.
- `f64` is the default because on modern CPUs it's "roughly the
  same speed as `f32` but is capable of more precision" (Book lines
  149-150, restated as a rule — the lesson does not measure speed).
  The precision difference is observable; the appendix witnesses it.
- Rust's floating-point types follow the *IEEE-754* standard (Book
  line 165). Name installed; mechanics stay deferred from lesson 033.
- An integer literal in an `f32` slot is rejected at compile time
  with the same `error[E0308]: mismatched types` plus `help: use a
  float literal` source-diff lesson 033 saw on `f64 = 3`. Same rule,
  both float types.

## Check Yourself

Predict, do not run:

(a) What are the two floating-point primitives in Rust?

(b) Without an annotation, what type does rustc infer for
`let z = 7.5;`?

(c) How many bits does an `f32` value occupy?

(d) `let n: f32 = 5;` — does this compile? If not, what does the
`help:` line literally say?

(Answers: (a) `f32` and `f64`. (b) `f64` — the unsuffixed float
literal default from lesson 033. (c) 32. (d) Reject. Same `error
[E0308]: mismatched types` shape as the *Try It* contrast probe;
the `help:` line says `help: use a float literal` and shows the
source-diff suggestion `let n: f32 = 5.0;` with `++` under the new
`.0`.)

## What To Ignore For Now

Today installs only the *second name* of the floating-point family
and the rule that the lesson-019 annotation slot accepts `f32` as
well as `f64`. Each of the following is real and deferred:

- *Float type-suffix forms* `2.0_f32`, `3.0_f64` — float parallel
  to lesson 081's integer suffix `57u8`. The auxiliary
  precision-witness probe in the evidence appendix uses one such
  suffix; it is not installed as a centered concept today.
- *IEEE-754 mechanics* — `NaN`, `infinity`, `0.0 / 0.0`, signed
  zero, denormals, why `NaN == NaN` is `false`. Lesson 033 deferred
  these; today only names the standard.
- *Float comparison gotchas* like `0.1 + 0.2 != 0.3`.
- *Float methods* `.sqrt()`, `.floor()`, `.abs()`, etc. Real on
  both `f32` and `f64`; deferred since lesson 040.
- *Float literals with exponents* `1.5e10`, `2.5E-3` — alternate
  spellings; separate move.
- *Float `as` casts* — float-to-integer (`3.14 as i32`),
  integer-to-`f32` (`5_i32 as f32`), and `f32`/`f64` cross-casts.
  Lesson 034 installed `i32 as f64` only.
- *Format specifiers for floats* like `{:.2}`, `{:e}`. Today's
  probe uses default `{}`, which prints whole-valued floats without
  a trailing decimal (`2.0` prints as `2`).
- *Mixing `f32` and `f64`* in arithmetic without an `as` cast.
- *When to pick `f32` vs `f64`* operationally. Today only states
  the Book's default rule.

## Evidence

See `../evidence/088-f32-floating-point.md`.
