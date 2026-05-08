---
id: 091-range-reversal-rev
status: accepted
evidence: ../evidence/091-range-reversal-rev.md
---

# Reverse a range with `(start..end).rev()` for a countdown

## The Move

Lesson 022 installed `for var in 0..N { ... }`, which walks the range
in *ascending* order: `0, 1, ..., N-1`. To walk a range in
*descending* order, append `.rev()` to a parenthesized range:

```rust
for number in (1..4).rev() {
    println!("{}!", number);
}
```

This runs the body three times with `number` bound to `3`, then `2`,
then `1` — the same three values lesson 022's `1..4` would produce,
but in reversed order. Everything else about the `for` loop is
unchanged.

The parentheses around `1..4` are required. Without them, `4.rev()`
would parse first as a method call on the integer `4`, and rustc
would reject. The parens force the range to be the value `.rev()` is
called on.

## Mental Model Delta

- *Before:* "`for var in 0..N { ... }` (lesson 022) iterates the
  range in ascending order. To count *down* from `N-1` to `0`, I
  need a `while` loop with a `mut` counter (lesson 017) or to
  contrive arithmetic on the loop variable."
- *After:* "Append `.rev()` to a parenthesized range to reverse
  the iteration order. `for number in (1..4).rev() { ... }` walks
  `3, 2, 1`. Same `for ... in ...` shape (lesson 022), one method
  (lesson 040's method-call syntax) tacked onto the range value.
  This is the canonical Rust countdown shape."

## Prerequisites

- Installed concepts:
  - Lesson 022 (load-bearing): `for var in 0..N { ... }` repeats
    the body once per number in the exclusive range `0..N`,
    auto-binding `var`. Today swaps the bare range for
    `(start..end).rev()` and observes the values arrive reversed.
  - Lesson 040 (load-bearing): the method-call form
    `value.method(args)` — receiver, dot, method name,
    parenthesized argument list. Today applies that form to a
    range expression with empty arguments.
  - Lessons 002, 011, 003 (cited): `fn main`, `println!`, and the
    diagnostic map for the contrast probe.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

This is the Book's countdown example at Ch3-5 lines 538-544
(verbatim modulo the format string `"{number}!"` -> `"{}!", number`,
the positional shape from lesson 011).

Compile and run:

```console
$ rustc demo.rs
$ ./demo
3!
2!
1!
LIFTOFF!!!
```

The piece `(1..4).rev()` is a range value with `.rev()` called on
it. Lesson 022's `1..4` would produce `1, 2, 3`; `.rev()` reverses
that, so the loop binds `number` to `3`, then `2`, then `1` —
printing `3!`, `2!`, `1!` — and after the range is exhausted the
next statement prints `LIFTOFF!!!`.

The Book introduces this exact program as "the countdown would look
like using a `for` loop and another method we've not yet talked
about, `rev`, to reverse the range."

Now the contrast. *Predict*: what happens if you drop the
parentheses and write `for number in 1..4.rev() { ... }`? Edit
`demo.rs` so the `for` line reads `for number in 1..4.rev() {` and
recompile. rustc emits:

```
error[E0689]: can't call method `rev` on type `{integer}`
 --> broken.rs:2:24
  |
2 |     for number in 1..4.rev() {
  |                        ^^^ can't call method `rev` on type `{integer}`
  |
help: you must surround the range in parentheses to call its `rev` function
  |
2 |     for number in (1..4).rev() {
  |                   +    +
```

Without parens, rustc parses `4.rev()` first — a method call on the
integer `4` — and there is no `rev` method on `{integer}`. The
`help:` line states the fix in this lesson's exact words: surround
the range in parentheses. With parens, `(1..4)` evaluates to a range
*first*, then `.rev()` is called on the range.

(Full transcripts in `../evidence/091-range-reversal-rev.md`.)

## What Changed

- You can write `(start..end).rev()` to walk the range from
  `end - 1` down to `start`, the reverse of lesson 022's order.
- You know `.rev()` is a method (lesson 040) attached to the range
  value, with empty argument list `()` — same shape as `n.abs()`.
- You know the parentheses are syntactically required. Without
  them, rustc parses `4.rev()` first and rejects with E0689 plus a
  `help:` line that names the parens fix directly.
- You know one canonical Rust countdown shape:
  `for n in (1..stop).rev() { ... }`, the preferred alternative to
  lesson 017's `while`-with-`mut`-counter countdown.
- The exclusive-upper-bound rule from lesson 022 is unchanged:
  `(1..4).rev()` produces `3, 2, 1` (not `4, 3, 2, 1`). Reversing
  changes the order, not the membership.

## Check Yourself

(a) What does `for n in (1..4).rev() { println!("{}", n); }` print,
in order?

(b) What three values does `for n in (5..8).rev() { ... }` bind to
`n`, in order?

(c) Why are the parentheses required in `(1..4).rev()`? What error
fires without them?

(Answers: (a) Three lines: `3`, `2`, `1`. (b) `7`, then `6`, then
`5`. The range `5..8` produces `5, 6, 7`; `.rev()` reverses that
order. `8` is not iterated — excluded both before and after
reversal. (c) Without parens, rustc parses `4.rev()` first and
emits `error[E0689]: can't call method `rev` on type `{integer}``
with `help: you must surround the range in parentheses to call its
`rev` function`. The parens force the range to be `.rev()`'s
receiver.)

## What To Ignore For Now

This lesson installs only one idea: appending `.rev()` to a
parenthesized range reverses the iteration order. Deferred:

- *The `Iterator` trait* and *`DoubleEndedIterator`* — the trait
  machinery that supplies `.rev()`. Today names `.rev()` as a
  method without unpacking it. The Book itself defers ("another
  method we've not yet talked about").
- *Other iterator methods* — `.map()`, `.filter()`, `.collect()`,
  `.take()`, `.skip()`, `.enumerate()`, `.sum()`, etc.
- *`.rev()` on collections* like `Vec`, `String`, arrays, slices.
  Same method, works because they are iterable; today applies it
  only to ranges.
- *`(start..=end).rev()`* — lesson 039's inclusive range also
  accepts `.rev()`. Today shows only the exclusive form.
- *`(0..N).step_by(2)`* and other range adapters.
- *The precise precedence rule* that makes the parens required —
  the method-call dot binds tighter than the `..` range operator.
  Mechanism, not learner-facing rule.
- *`break` and `continue` inside a reversed `for`*. Both work
  unchanged; each is its own composition.
- All previously deferred items.

## Evidence

See `../evidence/091-range-reversal-rev.md` for the corpus-quote
map, the rustc / system toolchain string, the working probe
transcript, the no-parens E0689 contrast transcript, the auxiliary
`(5..8).rev()` and `(0..3).rev()` probe transcripts, and the
prerequisite-claim summary.
