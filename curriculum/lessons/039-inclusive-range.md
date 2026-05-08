---
id: 039-inclusive-range
move: "write `0..=N` (with `=` between the dots and `N`) instead of lesson 022's `0..N` to make a `for` loop also iterate the upper bound `N`"
main_concept: "Rust has two range syntaxes: lesson 022's `0..N` is the *exclusive* range, producing `0, 1, ..., N-1`; the new `0..=N` is the *inclusive* range, producing `0, 1, ..., N-1, N`. Both fit the same `for var in range { ... }` shape from lesson 022 — the only syntactic delta is the `=` after the `..`. The inclusive form runs the body exactly one more time (the extra pass binds `var` to `N` itself)."
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 005-let-binding
  - 006-mut-binding
  - 019-type-annotation-i32
  - 022-for-range
  - 023-compound-add-assign
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "range patterns in `match` arms (`1..=5 => ...`)" moves
  - future "half-open ranges `..N`, `0..`, `..`" moves
  - future "range methods `.rev()` / `.step_by()`" moves
  - future "`RangeInclusive` vs `Range` as distinct std::ops types" moves
  - future "`gen_range(1..=100)` and other range arguments to library functions" moves
sources:
  - output/docs/rust/reference/expressions/range-expr.md
  - output/docs/rust/std/ops/struct.RangeInclusive.md
  - output/docs/rust/book/appendix-02-operators.md
  - output/docs/rust/book/ch02-00-guessing-game-tutorial.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/039-inclusive-range.rs
status: accepted
---

# Include the upper bound with `0..=N`

## The Move

Inside `fn main`, write the shape

```rust
for n in 0..=5 {
    // body
}
```

The body runs *six* times: once with `n` bound to `0`, then `1`, `2`,
`3`, `4`, and finally `5`. The piece `0..=5` is a *range* with `=`
glued to the dots, called an *inclusive* range. It produces the same
numbers as lesson 022's `0..5` plus one more — the upper bound `5`
itself.

The `for var in range { ... }` shape from lesson 022 is unchanged.
The only edit is the `=` after the `..`.

## Mental Model Delta

- Before: "`for var in 0..N { ... }` from lesson 022 runs the body N
  times, binding `var` to `0, 1, ..., N-1`. The upper bound `N` is
  exclusive — never included. To touch every value from `0` up to and
  including `N`, I would have to write `0..(N+1)` or contrive
  something."
- After: "Rust has *two* range syntaxes that fit the same
  `for var in range { ... }` shape. `0..N` (lesson 022) is exclusive
  — it stops before `N`. The new form `0..=N`, with `=` between the
  dots and `N`, is *inclusive* — it stops *after* `N`. The inclusive
  form runs the body one more time. To pick the right one, ask: do I
  want my loop variable to ever take the value `N`?"

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`, silent on success.
  - Lesson 002: body of `fn main` runs when the executable launches.
  - Lesson 005: `let name = value;` binds a name.
  - Lesson 006: `let mut name = value;` makes the binding reassignable.
  - Lesson 019: `let name: TYPE = value;` annotates the binding's type;
    `i32` is the default integer type.
  - Lesson 022 (load-bearing): `for var in 0..N { ... }` repeats the
    body N times with `var` auto-advancing through the *exclusive*
    range `0..N` (i.e. `0, 1, ..., N-1`). This lesson swaps `..` for
    `..=` and observes one extra iteration.
  - Lesson 023: `n += value;` is shorthand for `n = n + value;`,
    requiring `let mut`. Used here to accumulate each pass's value.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let mut sum_excl: i32 = 0;
    for n in 0..5 {
        sum_excl += n;
    }
    let mut sum_incl: i32 = 0;
    for n in 0..=5 {
        sum_incl += n;
    }
    println!("0..5 sum = {sum_excl}");
    println!("0..=5 sum = {sum_incl}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
0..5 sum = 10
0..=5 sum = 15
```

Walk through it loop by loop.

- *First loop, `for n in 0..5`* (lesson 022's exclusive form). The
  range produces `0, 1, 2, 3, 4` — five values, stopping *before* `5`.
  Each pass adds `n` to `sum_excl` via `+=`. Final value:
  `0 + 1 + 2 + 3 + 4 = 10`.
- *Second loop, `for n in 0..=5`* (the new inclusive form). The range
  produces `0, 1, 2, 3, 4, 5` — six values, including `5`. Final
  value: `0 + 1 + 2 + 3 + 4 + 5 = 15`.
- *The difference, `15 − 10 = 5`*, is exactly the upper bound that
  the inclusive form added and the exclusive form skipped. That is
  the load-bearing observation: switching `..` to `..=` adds one
  more pass, and that pass binds `n` to `N`.

The Reference's range-expressions table summarizes both forms in a
single row each:

> `start..end` → `std::ops::Range` → `start ≤ x < end`
>
> `start..=end` → `std::ops::RangeInclusive` → `start ≤ x ≤ end`

The `<` versus `≤` on the right edge is the whole story.

## What Changed

- You can write `0..=N` (with `=` between the dots and `N`) inside the
  `for var in range { ... }` shape from lesson 022 to repeat the body
  exactly `N + 1` times, with `var` taking each value from `0` through
  `N` inclusive.
- You know Rust has *two* range forms for this shape: `..` (exclusive,
  lesson 022) and `..=` (inclusive, this lesson). The bound on the
  *left* of the `..` is always inclusive in both forms; the `=`
  controls only the *right* edge.
- You can pick the right form by asking whether the loop variable
  should ever take the value of the upper bound. If yes, use `..=`;
  if no, use `..`.
- You know the difference is exactly one extra pass. Two loops with
  the same `N`, one `..` and one `..=`, differ by one body
  execution.

## Check Yourself

You write `pred.rs` containing:

```rust
fn main() {
    let mut count: i32 = 0;
    for k in 0..=3 {
        count += 1;
        println!("k = {k}");
    }
    println!("count = {count}");
}
```

You run `rustc pred.rs` and then `./pred`.

(a) How many `k = ...` lines does the program print?

(b) What is the first such line, and what is the last?

(c) What value does the final `count = ...` line print?

(Answers: (a) Four. (b) First is `k = 0`; last is `k = 3`. (c) `4`.
The range `0..=3` is inclusive, so it produces `0, 1, 2, 3` — four
values, including the upper bound `3`. The body runs once per value,
so `count` accumulates four `+= 1` steps.)

## What To Ignore For Now

This lesson installs only one idea: writing `..=` instead of `..` in
a `for var in range { ... }` loop makes the upper bound inclusive,
adding exactly one pass. Deferred:

- *Range patterns in `match`* — `match n { 1..=5 => ... }` matches
  `n` if it is in the closed interval `[1, 5]`. Same `..=` syntax,
  different role (pattern position, not iterator position). The Book
  introduces this form in chapter 19. Mentioned but explicitly
  deferred from lessons 030 and 031.
- *Half-open ranges* `..N` (end-only, exclusive), `0..` (start-only,
  unbounded above), and the bare `..` (full). These are valid Rust
  and the Reference lists them in the same table, but they are not
  iterable on their own (the unbounded ones produce infinite or
  undefined iteration), so they have a separate set of uses.
- *Reverse iteration* — `(0..5).rev()` produces `4, 3, 2, 1, 0`. The
  Book's countdown example in chapter 3.5 uses this form. Method-call
  syntax; future move.
- *Step ranges* — `(0..10).step_by(2)` produces `0, 2, 4, 6, 8`.
  Future move.
- *Float ranges* — `0.0..1.0` exists as a value but is not iterable
  (no defined step for floats). Defer.
- *`RangeInclusive` as a type vs `Range` as a type*. The Reference's
  table shows `0..N` and `0..=N` produce values of *different* types
  (`std::ops::Range` versus `std::ops::RangeInclusive`). For `for`
  loops this distinction is invisible; for explicit type annotations
  on a range value it is not. Defer until iterators and traits.
- *Negative bounds* like `-3..=3`. Valid for signed integers; the
  probe uses positive bounds only.
- *Range arguments to library functions* — e.g. the Book's
  `gen_range(1..=100)` from the guessing-game tutorial. Defer until
  external crates and method calls.
- All previously deferred items.

## Evidence

### Sources

- `output/docs/rust/reference/expressions/range-expr.md`, the
  authoritative table at `[[expr.range.behavior]]`. Two load-bearing
  rows, with the `Range` column on the right giving the canonical
  inclusion conditions:

  > `start..end` → `std::ops::Range` → `start ≤ x < end`
  >
  > `start..=end` → `std::ops::RangeInclusive` →
  > `start ≤ x ≤ end`

  This is the corpus statement that `0..N` excludes `N` (`x < end`)
  while `0..=N` includes `N` (`x ≤ end`). It also shows the `=` is
  the only syntactic delta between the two forms. The same page's
  `[[expr.range.syntax]]` grammar gives both productions side by
  side: `RangeExpr → Expression .. Expression` and
  `RangeInclusiveExpr → Expression ..= Expression`.

- `output/docs/rust/std/ops/struct.RangeInclusive.md`, the standard
  library page for `RangeInclusive`. Direct quote (lines 12-15):

  > A range bounded inclusively below and above (`start..=end`).
  >
  > The `RangeInclusive` `start..=end` contains all values with
  > `x >= start` and `x <= end`. It is empty unless `start <= end`.

  The page also gives a concrete numeric example confirming the
  iteration set (lines 26-28):
  `assert_eq!(3 + 4 + 5, (3..=5).sum());`. So `3..=5` iterates
  exactly `3, 4, 5` (three values), and their sum is `12`. This
  matches the lesson's claim that an inclusive range over `[a, b]`
  produces `b - a + 1` values, both endpoints included.

- `output/docs/rust/book/appendix-02-operators.md`, the operator
  table. Lines 44-45 list both forms next to each other:

  > `..` | `.., expr.., ..expr, expr..expr` | Right-exclusive range
  > literal | `PartialOrd`
  >
  > `..=` | `..=expr, expr..=expr` | Right-inclusive range literal
  > | `PartialOrd`

  This licenses the lesson's framing: `..` is *right-exclusive* and
  `..=` is *right-inclusive*; the *left* bound is always inclusive
  in both, the `=` toggles only the right edge.

- `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`, lines
  696-699. The Book's first prose introduction of the form, in the
  context of `gen_range`:

  > The `gen_range` method takes a range expression as an argument
  > and generates a random number in the range. The kind of range
  > expression we're using here takes the form `start..=end` and is
  > inclusive on the lower and upper bounds, so we need to specify
  > `1..=100` to request a number between 1 and 100.

  This is the Book's plain-English statement that `start..=end`
  includes both endpoints. The lesson uses the same shape but in a
  `for` loop instead of as a `gen_range` argument.

  Calibration:
  - The Book's chapter 3.5 (where lesson 022 sourced its for-loop
    quote) does not introduce `..=` at all — its inclusive-range-ish
    example is `(1..4).rev()`, which is exclusive `1..4` plus the
    `.rev()` method. So this lesson reaches outside chapter 3.5 to
    chapter 2 and the operator appendix for the prose introduction
    of `..=`, and to the Reference for the canonical semantics.
  - The Book's chapter 19.3 introduces `..=` again as a *pattern* in
    `match` arms ("`1..=5 => println!(...)`"). Same syntax, different
    role. Lessons 030 and 031 deferred this; this lesson defers it
    again.

- The local probe (single working transcript), captured below.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/039-inclusive-range.rs`.
The committed file is the exact working program used in Try It, plus
header comments documenting the run.

Probe transcript, run in a clean temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let mut sum_excl: i32 = 0;
    for n in 0..5 {
        sum_excl += n;
    }
    let mut sum_incl: i32 = 0;
    for n in 0..=5 {
        sum_incl += n;
    }
    println!("0..5 sum = {sum_excl}");
    println!("0..=5 sum = {sum_incl}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
0..5 sum = 10
0..=5 sum = 15
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent (consistent with lesson 001).
- `./demo` prints exactly two lines: `0..5 sum = 10` and
  `0..=5 sum = 15`.
- The exclusive loop summed `0 + 1 + 2 + 3 + 4 = 10` (five iterations,
  consistent with lesson 022's exclusive-upper-bound rule).
- The inclusive loop summed `0 + 1 + 2 + 3 + 4 + 5 = 15` (six
  iterations). The difference, `15 − 10 = 5`, is exactly the upper
  bound `5`, contributed by the inclusive form's extra pass. This is
  the load-bearing observation for the `..=` includes the upper
  bound claim.
- Both loops use the same `for var in range { ... }` shape from
  lesson 022. Inside each body, `+=` (lesson 023) accumulates onto a
  `let mut` accumulator (lesson 006) typed `i32` (lesson 019). No
  new mechanism beyond `..=` itself.
- No broken-contrast probe: the natural broken contrast for `..=` is
  a syntactically invalid range (e.g. `0..=` with no upper bound),
  not a useful pedagogical contrast. The lesson's contrast is
  `0..5` vs `0..=5`, both exercised in this single program.
- Only the working source is committed under `observations/`. The
  temp dir was removed; no binaries are committed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `005-let-binding` (accepted) — `let name = value;`.
- `006-mut-binding` (accepted) — `let mut name = value;` makes the
  binding reassignable. Used for the two `i32` accumulators.
- `019-type-annotation-i32` (accepted) — `let name: TYPE = value;`;
  `i32` for the accumulators.
- `022-for-range` (accepted, load-bearing) — `for var in 0..N { ... }`
  runs the body N times with `var` auto-advancing through the
  *exclusive* range `0..N`. This lesson swaps `..` for `..=` and
  observes the upper bound now also gets a pass.
- `023-compound-add-assign` (accepted) — `n += value;` shorthand on a
  `let mut` binding. Used to accumulate each pass's value.
- `030-match-on-bool` and `031-match-integer-and-wildcard` (accepted)
  — both lessons listed range patterns `1..=5` under What To Ignore
  For Now. This lesson installs `..=` in iterator position only and
  defers the pattern role again.
