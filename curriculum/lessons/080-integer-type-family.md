---
id: 080-integer-type-family
status: accepted
evidence: ../evidence/080-integer-type-family.md
---

# Name the twelve integer types and their sign-and-width convention

## The Move

Lessons 019, 062, and 077 each centered one typed integer name —
`i32`, `u32`, `usize`. The Book's Table 3-1 lists *twelve*. Today
names the family. The naming rule has two axes:

- *Signed vs unsigned.* The first letter says whether the type can
  hold negative values: `i` for *signed*, `u` for *unsigned* (zero
  or positive only). Lesson 062 named this for `i32` vs `u32`.
- *Bit width.* The number says how many bits of storage one value
  takes: `8`, `16`, `32`, `64`, `128`, or the keyword `size`. The
  `size` row (`isize` and `usize`) is *architecture-dependent* —
  64 bits on a 64-bit machine, 32 bits on a 32-bit machine —
  installed by lesson 077.

Two axes, six widths, twelve names: `i8` `u8` `i16` `u16` `i32`
`u32` `i64` `u64` `i128` `u128` `isize` `usize`. Each plugs into
the lesson-019 `: TYPE` slot the way `i32` does.

Bit width determines the *range* — the set of values the type can
hold. A signed `iN` holds `-(2^(N-1))` through `2^(N-1) - 1`
inclusive; an unsigned `uN` holds `0` through `2^N - 1`. So `u8`
is `0..=255` and `i8` is `-128..=127`. The Book gives both examples
verbatim (lines 83-87).

One default from lesson 019 still applies: with no annotation and
no other constraint, an integer literal becomes `i32`. To bind a
literal as any other integer type, write the annotation.

## Mental Model Delta

- *Before:* "I know three typed integer names — `i32` (019), `u32`
  (062), `usize` (077). I have heard there are more, but no system
  for naming them."
- *After:* "There are twelve named integer types, and the names are
  systematic. The first letter is `i` (signed) or `u` (unsigned).
  The number is the bit width — `8`, `16`, `32`, `64`, `128`, or
  the architecture-dependent `size`. Bit width fixes the range
  (`u8` is `0..=255`, `i8` is `-128..=127`, etc.). Same `: TYPE`
  annotation slot for any of them."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs` then `./name`; rustc silent
    on success.
  - Lesson 003 (load-bearing): the four-part diagnostic map. The
    contrast probe is read with that map.
  - Lesson 005 (load-bearing): `let name = value;`. The probe binds
    four names with this form.
  - Lesson 019 (load-bearing): `let name: TYPE = value;` is a *type
    annotation*; integer literals default to `i32`. Today plugs
    four type names into the same slot.
  - Lesson 062 (load-bearing): `u32` as the unsigned sibling of
    `i32`. Today extends the `i`/`u` convention to the rest of the
    family.
  - Lesson 077 (load-bearing): `usize` as the
    architecture-dependent indexing type. Today reuses
    "architecture-dependent" for both `isize` and `usize` without
    re-installing it.
  - Lesson 011 (cited): positional `{}` printing.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

In a fresh empty directory, save `demo.rs`:

```rust
fn main() {
    let small: u8 = 250;
    let signed_small: i8 = -100;
    let big: i64 = 9_000_000_000;
    let architecture: usize = 42;
    println!("u8 small = {}", small);
    println!("i8 signed_small = {}", signed_small);
    println!("i64 big = {}", big);
    println!("usize architecture = {}", architecture);
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
u8 small = 250
i8 signed_small = -100
i64 big = 9000000000
usize architecture = 42
```

Four bindings exercise four corners of the table: `u8` (smallest
unsigned, range `0..=255`); `i8` (smallest signed, range
`-128..=127` — the leading `i` is what allows the negative literal
at all); `i64` (nine billion is past `i32`'s top, fits comfortably
here); `usize` (architecture-dependent, like lesson 077). The `_`
in `9_000_000_000` is a visual separator; the full notation table
is queue item H.

Now the contrast. Save `broken.rs`:

```rust
fn main() {
    let too_big: u8 = 256;
    println!("{}", too_big);
}
```

Only one digit changed: `250` → `256`. `u8`'s range is `0..=255`,
so `256` is one past the top. Read the headline with the lesson
003 map; full transcript in `## Evidence`:

```text
error: literal out of range for `u8`
 --> broken.rs:2:23
  |
2 |     let too_big: u8 = 256;
  |                       ^^^
  |
  = note: the literal `256` does not fit into the type `u8` whose range is `0..=255`
  = note: `#[deny(overflowing_literals)]` on by default
```

The headline names the rule directly: `literal out of range for
`u8``. The first `note:` line spells the range the lesson just
named: `0..=255`. The second `note:` is the lint that fires; you
can ignore the lint name today. With `255` the program compiles;
with `256` it does not. The range is enforced at compile time when
rustc can see the literal.

## What Changed

- Twelve typed integer names exist in Rust: `i8` `u8` `i16` `u16`
  `i32` `u32` `i64` `u64` `i128` `u128` `isize` `usize`. Each plugs
  into the lesson-019 `: TYPE` slot.
- The names are systematic. *Sign*: `i` (signed, can be negative)
  vs `u` (unsigned, zero or positive only). *Bit width*: `8`, `16`,
  `32`, `64`, `128`, or `size` (architecture-dependent — 64 bits on
  64-bit, 32 bits on 32-bit, as lesson 077 installed for `usize`).
- Bit width fixes the range. Signed `iN`: `-(2^(N-1))` through
  `2^(N-1) - 1`. Unsigned `uN`: `0` through `2^N - 1`. So `u8` is
  `0..=255` and `i8` is `-128..=127`. Both examples are verbatim
  from the Book.
- The integer-literal default is still `i32` (lesson 019). To bind
  a literal at any other integer type, the annotation is required.
- An out-of-range literal is rejected at compile time:
  `error: literal out of range for `u8`` with the gloss
  `note: the literal `256` does not fit into the type `u8` whose
  range is `0..=255``.

## Check Yourself

Predict, do not run:

(a) `let n: u8 = 200;` — accept or reject?

(b) `let n: u8 = -1;` — accept or reject? Which sibling type would
accept `-1`?

(c) `let n: i16 = 40_000;` — `i16`'s range is `-32_768..=32_767`.
Accept or reject?

(d) `let n = 5;` — no annotation. What type does rustc infer?

(Answers: (a) accept; `200` is in `0..=255`. (b) reject — unsigned
types do not accept negative literals (lesson 062 saw this as
`error[E0600]: cannot apply unary operator -`). Any signed sibling
— `i8`, `i16`, `i32`, `i64`, `i128`, `isize` — accepts `-1`.
(c) reject — `40_000` is past `i16`'s upper bound; rustc fires
`error: literal out of range for `i16``. (d) `i32`, the
integer-literal default from lesson 019.)

## What To Ignore For Now

Today installs the *family* — twelve names by sign and width — and
the range-from-bit-width rule. Each of the following is real and
deferred:

- *Integer literal forms* — the `_` separator, hex `0x`, octal `0o`,
  binary `0b`, type suffix `57u8`, byte literal `b'A'`. The probe
  uses one separator (`9_000_000_000`); the full notation table is
  queue item H.
- *Integer overflow at runtime* — what happens when *arithmetic*
  produces a value outside the type's range. Book Ch3-2 *Integer
  Overflow* names the four method families (`wrapping_*`,
  `checked_*`, `overflowing_*`, `saturating_*`). Queue item I.
- *`as` casts between integer types* (`i32 as u8`, `u8 as i64`).
  Lesson 034 installed `i32 as f64`; integer-to-integer is a
  separate move, with sign-extension rules of its own.
- *Two's complement representation* — Book line 81 names this as
  the storage scheme for signed integers. Not load-bearing today.
- *`i128` / `u128` arithmetic* and *operations on `isize`* — the
  variants are named as family members; no probe binds or operates
  on them today.
- *Floating-point types* `f32` / `f64` (lesson 033) — different
  family, different table.
- *The `Copy` trait* — the structural reason integers can be copied
  freely. Trait machinery; deferred since lesson 040.
- *`MIN` / `MAX` associated constants* — `u8::MAX == 255`,
  `i8::MIN == -128`, etc. Today only states ranges in prose.
- *The `overflowing_literals` lint* — the deny-by-default lint
  named in the contrast probe's second `note:` line. Lint
  configuration is deferred.
- *When to pick `i32` vs `i64` vs `usize`* — the Book's operational
  advice at lines 109-112; today only states the default rule.

## Evidence

See `../evidence/080-integer-type-family.md`.
