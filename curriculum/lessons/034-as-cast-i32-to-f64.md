---
id: 034-as-cast-i32-to-f64
move: "convert an `i32` value to `f64` with `value as f64` so it can mix with float arithmetic, e.g. `let avg: f64 = (count as f64) / 2.0;` where `count: i32`"
main_concept: "Rust does not implicitly convert between numeric types -- an `i32` and an `f64` cannot share an arithmetic operator directly; the way to bridge them is an *explicit type cast*, written with the binary operator `as`: the expression `value as TARGET_TYPE` produces a new value of `TARGET_TYPE` with the same numeric value as `value`; this lesson installs only the `i32 as f64` direction (always produces the closest possible `f64`, exact for small integers); once converted, lesson 033's float arithmetic rules apply unchanged"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 005-let-binding
  - 009-arithmetic-on-integers
  - 019-type-annotation-i32
  - 033-f64-floats
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "`f64 as i32` (float-to-integer cast, truncates toward zero)" moves
  - future "integer-width casts `i32 as i64` / `i32 as u8`" moves
  - future "signed/unsigned reinterpretation `-1_i32 as u32`" moves
  - future "`as` on non-numeric types (raw pointers, function pointers, `char`)" moves
  - future "operator precedence (why `count as f64 / 2.0` parses the same as `(count as f64) / 2.0`)" moves
  - future "`From` / `Into` and `TryFrom` / `TryInto` traits as the safer-conversion alternative to `as`" moves
  - future "traits and generics" moves (the broken-contrast diagnostic mentions both)
sources:
  - output/docs/rust/reference/expressions/operator-expr.md
  - output/docs/rust/rust-by-example/types/cast.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/034-as-cast-i32-to-f64.rs
  - broken-contrast probe transcript inline in lesson `## Evidence` (not committed)
status: accepted
---

# Convert an `i32` to `f64` with `value as f64`

## The Move

When you have an `i32` value and you want to use it in float arithmetic
alongside an `f64`, write `value as f64`. The expression produces a new
`f64` value with the same numeric value as the original `i32`, and the
result fits anywhere an `f64` fits -- on the right of a `let: f64`, as an
operand of `/`, etc.

```rust
let count: i32 = 7;
let avg: f64 = (count as f64) / 2.0;
```

`count` is an `i32`; `count as f64` is the `f64` value `7.0`; the rest of
the right-hand side is lesson 033's float division `7.0 / 2.0` and
evaluates to `3.5`.

## Mental Model Delta

- Before: "I have `i32` (lesson 019) and `f64` (lesson 033). They are
  separate types. If I try to mix them in one expression -- like
  dividing an `i32` count by `2.0` to get a non-truncating average --
  rustc rejects it. There is no implicit conversion."
- After: "Right -- there is no *implicit* conversion. But there is an
  *explicit* one: the binary operator `as`. The expression
  `value as TARGET_TYPE` produces a new value of `TARGET_TYPE` with the
  same numeric value. So `count as f64` (where `count: i32`) is the
  `f64` value with the same number, ready to divide by `2.0` under
  lesson 033's float arithmetic. The Reference calls this a *type cast
  expression*."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`, silent on success.
  - Lesson 002: body of `fn main` runs when the executable launches.
  - Lesson 003 (load-bearing): rustc diagnostics have a headline +
    `-->` location + source excerpt with caret + optional `help:` /
    `note:` lines. The broken-contrast walk below decodes a new E-code
    (E0277) by reading the headline and explicitly *deferring* the
    trait-machinery body lines.
  - Lesson 005: `let name: TYPE = value;` binds a name; the slot the
    cast value lands in.
  - Lesson 009 (load-bearing): `+ - * /` on integers; `/` between
    integers truncates toward zero. The motivation for this lesson is
    that `count / 2` truncates while `(count as f64) / 2.0` does not.
  - Lesson 019 (load-bearing): `name: TYPE` attaches a type to a `let`
    binding. The probe's `let count: i32 = 7;` and `let avg: f64 = ...;`
    both rely on this shape.
  - Lesson 033 (load-bearing, just installed): `f64` is Rust's default
    floating-point type; `5.0`, `2.0`, `3.14` are float literals
    (numbers written with `.`); `+ - * /` work on floats too, and `/`
    does *not* truncate. **This lesson supplies the `as` cast that
    bridges integers and floats**, which lesson 033 explicitly deferred
    under What To Ignore For Now.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let count: i32 = 7;
    let avg: f64 = (count as f64) / 2.0;
    println!("count = {count}");
    println!("avg = {avg}");
}
```

Three things to notice in `let avg: f64 = (count as f64) / 2.0;`:

- `count as f64` is the new shape. The keyword `as` sits between an
  expression on the left (`count`, of type `i32`) and a type name on
  the right (`f64`). The whole expression `count as f64` is an `f64`
  value with the same numeric value as `count` -- so for `count = 7`,
  the cast produces the `f64` value `7.0`.
- The parentheses around `(count as f64)` are not strictly required.
  `as` binds tighter than `/`, so `count as f64 / 2.0` parses the same
  way as `(count as f64) / 2.0`. The probe uses parentheses anyway
  because they make the cast visible at a glance.
- Once the cast happens, the right-hand side is just `7.0 / 2.0`,
  which is float division from lesson 033. No truncation: the value
  is `3.5`, not `3`.

Compile and run:

```console
$ rustc demo.rs
$ ./demo
count = 7
avg = 3.5
```

Same `rustc file.rs` then `./name` workflow as lesson 001. `count`
prints as a plain integer; `avg` prints as `3.5`, the float-division
result lesson 033 would predict for `7.0 / 2.0`.

Now do the contrast. In a separate temp directory, save `broken.rs`
with the cast removed:

```rust
fn main() {
    let count: i32 = 7;
    let avg: f64 = count / 2.0;
    println!("count = {count}");
    println!("avg = {avg}");
}
```

Compile it. The full transcript is in `## Evidence`; reading it with
lesson 003's order:

- *Headline*: `error[E0277]: cannot divide \`i32\` by \`{float}\``.
  This is a **new E-code** -- different from E0308 (mismatched types,
  the workhorse code in lessons 024-029, 033) and from E0004
  (non-exhaustive match, lessons 030-031). The headline is plain
  English: rustc cannot divide an `i32` by a `{float}`. The label
  `{float}` is rustc's umbrella name for an unconstrained float
  literal, the float analogue of the `{integer}` label seen in
  lesson 033's broken-contrast.
- *`-->` location*: `broken.rs:3:26` -- column 26 of line 3, the `/`
  operator.
- *Source excerpt*: `^` underlines the `/` with the inline annotation
  `no implementation for \`i32 / {float}\``. So rustc says: there is
  no `/` defined that takes an `i32` on the left and a `{float}` on
  the right.

The diagnostic continues with several lines that begin with
`= help:` and `= note:` and reference *the trait `Div<Rhs>`*, types
like `Div<i32>`, `Div<&i32>`, `&i32 implements Div`, and a macro called
`div_impl_integer`. **All of those body lines refer to concepts this
run has not installed yet** -- specifically *traits* (the mechanism
the standard library uses to define which type pairs can be combined
with which operator) and *generics* (the `<Rhs>` part). Neither is
part of this lesson. Read the headline, note that no such combination
is implemented, and skip the rest. Lesson 003 already established
that `note:` and `help:` lines are optional context; this is the
first time in the run a broken-contrast diagnostic includes context
lines that point at concepts the lesson explicitly defers, and the
honest move is to walk past them.

The fix is what this lesson installs: cast the `i32` to `f64` first.
With `(count as f64) / 2.0`, the `/` now has an `f64` on the left
and an `f64` on the right, which is the same shape lesson 033's
`5.0 / 3.0` example used. Rustc accepts it and the program runs.

## What Changed

- You can convert an `i32` value to an `f64` value by writing
  `value as f64`. The keyword `as` is a binary operator: expression on
  the left, target type on the right.
- For an `i32` value, the cast is exact for small numbers -- the
  resulting `f64` has the same numeric value (`7_i32 as f64` is
  `7.0_f64`). The Reference puts it as: "Casting from an integer to
  float will produce the closest possible float."
- This unblocks mixing an `i32` count with `f64` arithmetic. Without a
  cast, `count / 2.0` is rejected. With the cast, `(count as f64) /
  2.0` is plain float division (lesson 033) and follows lesson 033's
  rules: `7.0 / 2.0` evaluates to `3.5`, no truncation.
- New E-code: **E0277** (`cannot divide \`i32\` by \`{float}\``)
  appears when you try to combine two distinct numeric types with an
  operator. Treat it as "you need an `as` cast on one side." The
  body of the diagnostic mentions traits and generics; ignore those
  body lines for now.

## Check Yourself

You write `half.rs` containing:

```rust
fn main() {
    let n: i32 = 5;
    let half: f64 = (n as f64) / 2.0;
    println!("half = {half}");
}
```

You run `rustc half.rs && ./half`.

(a) What single line does the executable print?

(b) If you remove the `as f64` so the right side reads `n / 2.0`,
which token does the `-->` location point at, and which E-code
appears in the headline?

(Answers: (a) `half = 2.5`. (b) The `-->` points at the `/` operator;
the E-code is `E0277`, the same one as the broken-contrast probe in
this lesson.)

## What To Ignore For Now

This lesson installs only one direction of one cast (`i32 as f64`)
and the headline-level read of one new E-code (`E0277`). Each of the
following is real and will be taught later, but is *not* part of
this move:

- *`f64 as i32` -- the reverse direction*. Casting a float to an
  integer truncates toward zero (the Reference: "Casting from a float
  to an integer will round the float towards zero"), with extra rules
  for `NaN` and overflow. Future move.
- *Integer-width casts*: `i32 as i64`, `i32 as u8`, `i64 as i32`, and
  so on. Different sizes have their own rules (truncation when
  shrinking, zero/sign-extension when growing). Future moves.
- *Signed/unsigned reinterpretation* like `-1_i32 as u32` (which
  produces `4294967295`). Future move.
- *`as` on non-numeric types*. The Reference grammar allows `as` for
  raw pointers, function pointers, enum-to-integer, `bool`/`char` to
  integer, and `u8` to `char`. All deferred.
- *Operator precedence in detail*. The probe uses parentheses so
  precedence does not need to be installed yet; the only fact this
  lesson uses in passing is that `as` binds tighter than `/`.
- *Coercions* (the few implicit conversions rustc *does* allow in
  specific positions). The Reference distinguishes coercions from
  casts. The rust-by-example phrasing "Rust provides no implicit
  type conversion (coercion) between primitive types" is a useful
  first approximation for primitive numeric types but oversimplifies
  in general. Future move.
- *Traits and generics*. The broken-contrast diagnostic mentions
  *the trait `Div<Rhs>`*, `Div<i32>`, `Div<&i32>`, and so on. These
  are the mechanism by which the standard library defines which
  operators work on which type pairs, and they involve *generics*
  (the `<Rhs>` part). Both are major future moves.
- *`From` / `Into` / `TryFrom` / `TryInto`*. These are the trait-based
  alternative to `as` for many conversions, often safer for cases
  where the conversion can fail. They depend on traits and generics.
  Future moves.
- *`{integer}` and `{float}` placeholders in diagnostics* as a formal
  concept. The label appeared in lesson 033's broken-contrast and
  again here; the run treats it informally as "the literal whose
  exact type rustc has not pinned down yet." A future move will
  install the kind/type distinction properly.
- *Float literal suffixes* like `7.0_f64`, `2_i32`, `2.0_f32`.
  Mentioned in passing; deferred.
- All previously deferred items: shadowing, `&` references, closures,
  modules and `pub`, the broader format-string DSL, IEEE 754
  peculiarities, integer overflow semantics.

## Evidence

### Sources

- `output/docs/rust/reference/expressions/operator-expr.md`, the
  *Type cast expressions* section starting at line 576. Three
  load-bearing direct quotes:

  Line 582 (the grammar):

  > TypeCastExpression -> Expression as TypeNoBounds

  This is the corpus source for the shape `expression as type`. The
  grammar's `TypeNoBounds` is more general than just numeric types
  (it covers raw pointers and other cases listed in the table at
  lines 611-627). This lesson restricts the cast to the
  *Integer or Float type -> Integer or Float type* row of that
  table, which the Reference calls a *Numeric cast*.

  Line 586:

  > A type cast expression is denoted with the binary operator `as`.

  Grounds the name *type cast expression* and the framing of `as` as
  a *binary operator* (left-hand expression, right-hand type).

  Line 590:

  > Executing an `as` expression casts the value on the left-hand
  > side to the type on the right-hand side.

  Grounds the operational claim that the expression *produces* a
  new value of the right-hand-side type from the left-hand-side
  value.

  And the load-bearing quote for the i32->f64 direction
  specifically, line 706:

  > Casting from an integer to float will produce the closest
  > possible float

  The Reference's Numeric cast subsection at line 633 enumerates the
  cases. The `int-as-float` bullet (line 704) gives the specific
  rule for this lesson's direction. For an `i32` value of `7`, the
  closest possible `f64` is exactly `7.0`, so the cast is exact.
  The Reference's `assert_eq!(1337i32 as f32, 1337f32);` example at
  line 715 confirms exactness for small integers in the `f32`
  variant of the same rule.

  Calibration:

  - The Reference grammar `Expression as TypeNoBounds` is more
    general than this lesson uses. The table at lines 611-627 lists
    eight cast directions; this lesson installs only the first
    (Integer or Float type -> Integer or Float type), and within
    that, only the `i32 as f64` direction. All other directions are
    deferred.
  - The Reference's example at lines 599-603 (the `average`
    function) uses `len(values) as f64` exactly as this lesson uses
    `count as f64`. That example also relies on functions returning
    typed values, which is lesson 021. Functions are not load-bearing
    here -- the probe uses `let count: i32 = 7;` directly -- but
    confirms the `as f64` shape is idiomatic.
  - The phrasing "the closest possible float" leaves room for
    rounding when the integer is too big to represent exactly in
    `f64`. For an `i32` value, the integer fits exactly in `f64`'s
    52-bit significand for any value of magnitude up to 2^53, which
    covers all of `i32`'s range (i32 max is about 2.15 * 10^9, well
    under 2^53 ~ 9 * 10^15). So `i32 as f64` is *always* exact in
    practice; the qualifier is real in general but not for this
    direction. Worth noting; not load-bearing for the lesson.

- `output/docs/rust/rust-by-example/types/cast.md`. Direct quote from
  lines 4-5:

  > Rust provides no implicit type conversion (coercion) between
  > primitive types. But, explicit type conversion (casting) can be
  > performed using the `as` keyword.

  Grounds the load-bearing claim "no implicit conversion between
  primitive types; the way to bridge them is `as`." Calibration: the
  parenthetical equation "implicit type conversion (coercion)" is a
  slight oversimplification -- Rust *does* permit some coercions
  between primitive numeric types in specific positions (e.g. an
  integer literal can fit in an `f64` slot), and the Reference
  separates *coercions* from *casts* even though `as` is involved
  in both. For the purposes of this lesson the simpler "i32 and f64
  cannot share an arithmetic operator directly; use `as`" framing is
  exactly right; the more precise coercion-vs-cast distinction is a
  future move.

### Probes

Two probes were captured on rustc 1.95.0 (59807616e 2026-04-14) on
Darwin x86_64. The working probe is committed at
`experimental/eduratchet2/runs/rust-moves/observations/034-as-cast-i32-to-f64.rs`.
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
    let count: i32 = 7;
    let avg: f64 = (count as f64) / 2.0;
    println!("count = {count}");
    println!("avg = {avg}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
count = 7
avg = 3.5
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (consistent with lesson 001).
- `count = 7` prints the underlying `i32` unchanged.
- `avg = 3.5` is the load-bearing observation: a `7_i32` cast to
  `f64`, divided by `2.0`, produces `3.5` -- no truncation, exactly
  the result lesson 033's `7.0 / 2.0` would produce. Same `/`
  operator, both operands now `f64`, no truncation.
- A separate side-check (run in another temp dir, transcript not
  reproduced) confirmed that the parens-free form
  `let avg: f64 = count as f64 / 2.0;` accepts and prints the same
  `avg = 3.5`. This grounds the lesson's in-passing claim that `as`
  binds tighter than `/`.
- Only the working source is committed under `observations/`. No
  binaries are committed. The temp dir was removed.

#### Broken-contrast probe

`broken.rs` is identical except the `as f64` cast is removed, so the
`/` operator faces an `i32` on the left and an `f64` literal on the
right. Not committed; the transcript below is the artifact.

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
broken.rs
--- cat broken.rs ---
fn main() {
    let count: i32 = 7;
    let avg: f64 = count / 2.0;
    println!("count = {count}");
    println!("avg = {avg}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0277]: cannot divide `i32` by `{float}`
 --> broken.rs:3:26
  |
3 |     let avg: f64 = count / 2.0;
  |                          ^ no implementation for `i32 / {float}`
  |
  = help: the trait `Div<{float}>` is not implemented for `i32`
help: the following other types implement trait `Div<Rhs>`
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/ops/arith.rs:490:8
  |
  = note: `i32` implements `Div`
 ::: /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/ops/arith.rs:507:1
  |
  = note: in this macro invocation
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/internal_macros.rs:22:8
  |
  = note: `&i32` implements `Div<i32>`
 ::: /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/internal_macros.rs:33:8
  |
  = note: `i32` implements `Div<&i32>`
 ::: /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/internal_macros.rs:44:8
  |
  = note: `&i32` implements `Div`
  = note: this error originates in the macro `div_impl_integer` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
exit=1
--- ls after ---
broken.rs
--- temp dir removed ---
```

Notes:

- The headline `error[E0277]: cannot divide \`i32\` by \`{float}\``
  is a **new E-code** in this run -- distinct from E0308 (lessons
  024-029, 033) and E0004 (lessons 030-031). The headline is plain
  English; the lesson reads it directly.
- The `-->` location `broken.rs:3:26` points at column 26 of line 3,
  which is the `/` operator. Lesson 003's structure: headline, `-->`
  line, source excerpt with caret, optional notes/help.
- The source excerpt underlines `/` with `^` and the inline
  annotation `no implementation for \`i32 / {float}\``. That sentence
  carries the load-bearing diagnostic content for the lesson: there
  is no built-in way to apply `/` between an `i32` and a `{float}`.
- The label `{float}` is rustc's umbrella for an unconstrained float
  literal whose specific type it has not pinned down. Same family of
  label as the `integer` in lesson 033's `expected f64, found
  integer`. The lesson treats it informally; a future move will
  install the kind/type distinction properly.
- *Body lines deferred*: the `= help:` and `= note:` block names
  *the trait `Div<Rhs>`*, lists multiple "X implements Div<Y>" rows,
  and points at standard-library file paths. None of these concepts
  -- *trait*, *generic*, *implementation*, *macro invocation* -- are
  installed in this run. Reading them with lesson 003's structural
  map still works ("these are `note:` lines, optional context"), but
  the *content* of the notes is deferred. The lesson states this
  explicitly: read the headline, accept "no `/` for this pair," and
  walk past the rest until traits and generics are installed.
- Trailer: `For more information about this error, try \`rustc
  --explain E0277\`.` Confirms lesson 003's claim that the
  `--explain` trailer follows any error that has an `E####` code.
- Exit code: 1. No executable was produced.
- The pedagogical point: the diagnostic correctly identifies *what*
  is wrong (`/` does not work between `i32` and `{float}`) without
  this lesson having to install how the standard library expresses
  that fact. The fix is to bring the operands to the same type with
  `as` -- exactly what this lesson installs.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) -- `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) -- body of `fn main` runs when
  the executable launches.
- `003-read-rustc-diagnostic` (accepted, load-bearing) -- diagnostics
  have a headline + `-->` location + source excerpt with caret +
  optional `help:` / `note:` lines. The broken-contrast walk above
  decodes the headline of E0277 and explicitly defers the
  trait-machinery body lines, which is exactly the kind of selective
  reading lesson 003 enables.
- `005-let-binding` (accepted) -- `let name: TYPE = value;` binds a
  name; reused as the slot the cast value lands in.
- `009-arithmetic-on-integers` (accepted, load-bearing) -- `+ - * /`
  on integers; `/` between integers truncates toward zero so `5 / 2`
  is `2`. The motivation for casting is precisely that integer
  division truncates: `count / 2` (both `i32`) loses the fractional
  part, and the lesson's job is to bridge to lesson 033's
  non-truncating float division.
- `019-type-annotation-i32` (accepted, load-bearing) -- `name: TYPE`
  attaches a type. Used twice in the probe (`let count: i32` and
  `let avg: f64`) and named on the right of `as` (the `f64` in
  `count as f64`).
- `033-f64-floats` (accepted, load-bearing) -- `f64` is Rust's
  default floating-point type; float literals are written with `.`;
  `+ - * /` work on floats and `/` does *not* truncate. **This
  lesson supplies the `as` cast that bridges integer and float
  values, which lesson 033 explicitly deferred under What To Ignore
  For Now.** Once `count as f64` produces `7.0`, the rest of the
  right-hand side `(7.0) / 2.0` is plain lesson-033 float division.
- Older lessons (mention only): lesson 011 (`println!` named
  placeholders); lessons 023, 026, 028, 030, 031 (no compound
  assigns, `if`-as-expression, `break value`, or `match` arms appear
  in this lesson's probe). Cited as familiar shapes; not load-bearing.
