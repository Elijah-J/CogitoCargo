---
id: 062-u32-unsigned-integer
status: accepted
evidence: ../evidence/062-u32-unsigned-integer.md
---

# Annotate a `let` binding with the type `u32`

## The Move

Inside `fn main`, write a `let` statement of the shape
`let name: u32 = value;`. This is cycle 019's
`let name: TYPE = value;` form with `u32` in the `TYPE` slot
instead of `i32`. The std page calls `u32` "the 32-bit unsigned
integer type." Same width as `i32`; same arithmetic, same `{n}`
printing. The one visible difference is at the literal:
`let n: u32 = 42;` compiles, `let n: u32 = -1;` does not.

## Mental Model Delta

- *Before:* "I know one integer type, `i32`. I have not seen any
  other type name in the `: TYPE` slot."
- *After:* "There is a sibling integer type, `u32`. The `i` in
  `i32` means *signed*; the `u` in `u32` means *unsigned*. Same
  32-bit width, same arithmetic, but `u32` only holds values from
  `0` upward (range `0` to `2^32 − 1`). I write
  `let n: u32 = 42;` to bind one; a negative literal there is
  rejected at compile time. The integer-literal default is still
  `i32`, so the annotation is required: `let n = 42;` infers
  `i32`, not `u32`."

## Prerequisites

- Installed concepts:
  - Cycles 001, 002, 005, 011: compile-and-run, `fn main`,
    `let name = value;`, the `{name}` placeholder.
  - Cycle 009 (cited): `+` between integer values produces a new
    integer value fit for the right side of `let`.
  - Cycle 019 (load-bearing): `let name: TYPE = value;` is a
    *type annotation*; integer literals default to `i32`. Today's
    lesson reuses the annotation syntax unchanged — only the
    `TYPE` slot changes to `u32`.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, Linux/macOS shell (same as cycle 001).

## Try It

Save this as `demo.rs`:

```rust
fn main() {
    let n: u32 = 42;
    let m: u32 = n + 1;
    println!("n = {n}, m = {m}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
n = 42, m = 43
```

One line of output. The cycle-019 annotation form, the cycle-009 `+`
operator, and the cycle-011 `{name}` placeholder all work on `u32`
without modification. The right side of `let m` has type `u32`
because both operands are `u32`, so the annotation matches.

Now the contrast. Predict what happens if you change line 2 to
`let n: u32 = -1;`. Save it as `broken.rs` and recompile:

```
error[E0600]: cannot apply unary operator `-` to type `u32`
 --> broken.rs:2:18
  |
2 |     let n: u32 = -1;
  |                  ^^ cannot apply unary operator `-`
  |
  = note: unsigned values cannot be negated
```

The headline names the operator (`-`) and the type (`u32`) that
disagree. The `= note:` line is the audience-level reason. The
E-code is new (`E0600`); you do not need to memorize it — the
`note:` line is the takeaway. Read it the way cycle 003 taught:
headline, location, `note:`, fix.

(Full transcripts: `../evidence/062-u32-unsigned-integer.md`.)

## What Changed

- The `: TYPE` slot from cycle 019 is reusable: write `: u32` to
  bind a 32-bit unsigned integer.
- One new spelling rule: leading `i` means *signed*, leading `u`
  means *unsigned*. `i32` and `u32` share the 32-bit row of the
  Book's Table 3-1.
- `u32` arithmetic and printing reuse cycles 009 and 011 unchanged.
- Negative literals do not fit a `u32` slot: `let n: u32 = -1;`
  fails with `error[E0600]: cannot apply unary operator -`, gloss
  *unsigned values cannot be negated*. The corresponding `: i32`
  binding would compile.
- The integer-literal default is still `i32`. To get a `u32` from a
  literal, the annotation is *required* — without `: u32`, the
  literal `42` infers to `i32`.

## Check Yourself

You write `pred.rs`:

```rust
fn main() {
    let a: u32 = 7;
    let b: u32 = 3;
    let c: u32 = a + b;
    println!("c = {c}");
}
```

(a) Does `rustc pred.rs` accept the program? What does `./pred`
print?

(b) If you replaced line 3 with `let b: u32 = -3;`, which E-code
would the headline carry, and what would the `= note:` line say?

(c) If you removed the `: u32` annotation from line 2 and wrote
`let a = 7;`, what type would rustc infer for `a`? (Hint: cycle
019's installed default.)

*(Answers: (a) Yes. Prints `c = 10`. (b) `error[E0600]: cannot
apply unary operator -` to type `u32`; `note: unsigned values
cannot be negated`. (c) `i32` — cycle 019's installed default,
since removing the annotation lets it stand.)*

## What To Ignore For Now

- *Other integer types*. Table 3-1 lists ten more: `i8`, `u8`,
  `i16`, `u16`, `i64`, `u64`, `i128`, `u128`, `isize`, `usize`.
  Deferred.
- *Integer overflow*. `u32::MAX + 1` panics in debug, wraps in
  release. Not exercised.
- *Type suffixes on literals* (`42u32`, `1_000_u32`). Alternative
  spellings; the lesson uses bare literals with annotations.
- *`u32::MIN` and `u32::MAX`* — the associated constants rustc's
  `help:` block suggested. Deferred.
- *`as` casts between integer types*. Cycle 034 installed
  `i32 as f64`; `i32 as u32` is deferred.
- *The `Neg` trait*. The structural reason `-1` fails on `u32` is
  that `u32` does not implement `std::ops::Neg`. Trait machinery
  has been deferred since cycle 040; today's lesson treats the
  failure as "negative literals don't fit," not as a trait-bound
  failure. Cycle 034's E0277 is the alternate diagnostic family
  that surfaces when the reason is reported through a trait bound;
  today it surfaces as E0600 instead.
- *Binary representation* of unsigned integers — not unpacked.
- *`From` / `Into` between integer widths*. Deferred.
- *Checked, wrapping, saturating arithmetic* (`u32::checked_sub`,
  etc.). Deferred.

## Evidence

See `../evidence/062-u32-unsigned-integer.md` for the corpus-quote
map, the toolchain string, the working and broken-contrast probe
transcripts, and the prerequisite-claim summary.
