---
id: 022-for-range
move: "write `for var in 0..N { ... }` to repeat the body N times, binding `var` to each value in the range"
main_concept: "`for var in 0..N { ... }` runs the body once for each number in the exclusive range `0..N`; on each pass `var` is bound to that pass's number; the range `0..N` produces 0, 1, ..., N-1 and does NOT include N; unlike `while`, the loop variable is auto-advanced (no `mut` and no manual `+ 1` needed)"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
  - 005-let-binding
  - 017-while-loop
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "arrays and for-element-in-a" moves
  - future "inclusive ranges 0..=N" moves
  - future "range methods .rev() / .step_by()" moves
  - future "iterators and Iterator trait" moves
  - future "for with break/continue" moves
sources:
  - output/docs/rust/book/ch03-05-control-flow.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/022-for-range.rs
status: accepted
---

# Repeat a block N times with `for var in 0..N { ... }`

## The Move

Inside `fn main`, write the shape

```rust
for i in 0..3 {
    // body
}
```

The executable will run the body three times. On the first pass, the
name `i` is bound to `0`. On the second pass, `i` is bound to `1`. On
the third pass, `i` is bound to `2`. Then the loop is done and the
program continues with whatever statement comes after the closing `}`.

The piece `0..3` is a *range*. It produces the numbers `0`, `1`, `2`,
in order. The upper bound `3` is **exclusive** — `3` itself is *not*
in the range. To repeat the body N times, write `0..N`.

## Mental Model Delta

- Before: "I have `while condition { ... }` (lesson 017). To repeat a
  block N times, I have to declare a `mut` counter, write a comparison
  for the condition, and update the counter myself with `n = n + 1;`
  inside the body. If I forget the update, the loop runs forever."
- After: "Rust has a second loop shape: `for var in 0..N { ... }`. It
  repeats the body once for each number in the range `0..N`, and
  *automatically* binds `var` to that number on each pass. No `mut`,
  no comparison, no manual `+ 1`. The range `0..N` is exclusive, so it
  produces `0, 1, ..., N-1` — exactly N values. This is the natural
  shape for 'do this N times'."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`, silent on success.
  - Lesson 002: body of `fn main` runs when the executable launches.
  - Lesson 004: statements in `fn main` run top to bottom; the `for`
    is itself one such step, and the next statement after the loop
    runs once the range is exhausted.
  - Lesson 005 (load-bearing): `let name = value;` binds a name;
    `{name}` inside a `println!` format string substitutes the bound
    value at print time. Used here as `{i}`.
  - Lesson 017 (precursor): `while condition { ... }` repeats a block
    while a boolean condition holds. To repeat N times with `while`
    you need a `mut` counter (lesson 006), a comparison (lesson 013),
    and arithmetic (lesson 009). This lesson installs an alternative
    shape that needs none of those.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `main.rs`
containing exactly:

```rust
fn main() {
    for i in 0..3 {
        println!("i = {i}");
    }
    println!("done");
}
```

The Book describes the range piece directly:

> The way to do that would be to use a `Range`, provided by the
> standard library, which generates all numbers in sequence starting
> from one number and ending before another number.

So `0..3` produces `0`, `1`, `2`, in that order. *Ending before* `3`
is the load-bearing word: `3` is not in the range.

Compile and run:

```console
$ rustc main.rs
$ ./main
i = 0
i = 1
i = 2
done
```

Walk through it pass by pass.

- *Pass 1.* The loop takes the next number from the range, which is
  `0`. The name `i` is bound to `0` for this pass. The body runs once:
  `println!("i = {i}");` prints `i = 0`.
- *Pass 2.* The next number from the range is `1`. The name `i` is
  bound to `1`. The body prints `i = 1`.
- *Pass 3.* The next number from the range is `2`. The name `i` is
  bound to `2`. The body prints `i = 2`.
- *Range exhausted.* The next value would be `3`, but `3` is the
  exclusive upper bound, so the range is done. The loop exits.
  Execution continues with the next statement, `println!("done");`,
  which prints `done`.

That is four lines of output: three from inside the loop, one from
after.

Now contrast with lesson 017's `while` version of the same task:

```rust
fn main() {
    let mut n = 0;
    while n < 3 {
        println!("n = {n}");
        n = n + 1;
    }
    println!("done");
}
```

Both programs print the same four lines. But the `for` version has no
`let mut`, no `n < 3` condition, and no `n = n + 1;` line. The range
`0..3` carries the start, the stop, and the step all at once, and the
loop variable is advanced for you. Forgetting the manual update is
the classic `while`-loop mistake — the program would hang. The `for`
shape removes that whole failure mode.

## What Changed

- You can write `for var in 0..N { ... }` inside `fn main` and have
  the executable run the body exactly N times.
- You know how the loop variable behaves: on each pass, `var` is
  automatically bound to the next number in the range, starting at
  `0` and going up by `1` until the body has run N times.
- You know the range `0..N` is *exclusive* on the upper end: it
  produces `0, 1, ..., N-1`, never `N` itself. So `0..3` gives three
  numbers, not four.
- You can pick between two loop shapes for "do this N times": the
  `while` shape from lesson 017 (with `mut` plus arithmetic), or the
  `for var in 0..N` shape from this lesson (auto-advanced, no `mut`).
  The Book says: "most Rustaceans would use a `for` loop."

## Check Yourself

You write `count.rs` containing:

```rust
fn main() {
    for k in 0..5 {
        println!("k = {k}");
    }
    println!("after");
}
```

You run `rustc count.rs` and then `./count`.

(a) How many lines does it print?

(b) What is the first printed line, and what is the last printed line
*before* `after`?

(c) Does the program ever print `k = 5`? Why or why not?

(Answers: (a) Six lines: five from the loop body, one `after` from the
following statement. (b) First is `k = 0`; last before `after` is
`k = 4`. (c) No. The range `0..5` is exclusive on the upper end, so it
produces `0, 1, 2, 3, 4` and stops before `5`.)

## What To Ignore For Now

This lesson installs only one idea: `for var in 0..N { ... }` repeats
the body N times, with `var` bound to each value in the exclusive
range `0..N`. Deferred:

- *Iterating over arrays and other collections*, e.g. `for element in a`.
  The Book's main `for`-loop motivation is array iteration; this
  lesson uses only the integer-range form so we do not have to teach
  arrays first.
- *Inclusive ranges* `0..=N` (with `=`) that *do* include `N`. A
  distinct operator; deferred.
- *Range methods* like `.rev()` (which reverses the range) and
  `.step_by(2)` (which skips). The Book's countdown example uses
  `(1..4).rev()`; we use the bare `0..3` form so methods stay
  deferred.
- *Iterators and the `Iterator` trait*, the deeper machinery that
  makes `for` work on more than just ranges. Out of scope.
- *Other collection types* like `Vec`, `HashMap`, slices. Deferred.
- *The implicit `let` semantics of the loop variable*. Each pass binds
  `var` fresh; we describe this informally without unpacking scope or
  shadowing rules.
- *`break` and `continue` inside a `for` loop*. Both work, but each is
  its own move.
- All previously deferred items: shadowing, type annotations on
  bindings, the broader format-string DSL, defining your own
  functions, function parameters and return values, `cargo`, `match`,
  `if let`, compound assignment operators, infinite `loop { ... }`.

## Evidence

### Sources

- `output/docs/rust/book/ch03-05-control-flow.md`, the "Looping
  Through a Collection with `for`" subsection (lines 446-545). Two
  load-bearing direct quotes:
  - Lines 528-531: "The way to do that would be to use a `Range`,
    provided by the standard library, which generates all numbers in
    sequence starting from one number and ending before another
    number." This is the corpus definition of the range value
    `0..N` as a sequence of numbers, with the upper bound exclusive
    ("ending before"). It licenses the lesson's exclusive-upper-bound
    claim and the "produces 0, 1, ..., N-1" enumeration.
  - Lines 525-528: "The safety and conciseness of `for` loops make
    them the most commonly used loop construct in Rust. Even in
    situations in which you want to run some code a certain number of
    times, as in the countdown example that used a `while` loop in
    Listing 3-3, most Rustaceans would use a `for` loop." This is the
    corpus statement that licenses framing `for` + range as the
    preferred shape over the `while` + `mut` counter shape from
    lesson 017.

  Calibration:
  - The Book's main `for`-loop example (Listing 3-5, lines 502-509)
    iterates over an array literal: `let a = [10, 20, 30, 40, 50];
    for element in a { ... }`. This lesson uses the *range* form
    `0..3` rather than an array, so we do not have to teach array
    literals, indexing, or collection types in this cycle. Arrays
    and `for element in a` are explicitly listed under What To Ignore
    For Now.
  - The Book's range example (lines 538-543) is `for number in
    (1..4).rev()`, with `.rev()` reversing the range so the body
    counts `3, 2, 1`. This lesson uses the bare `0..3` form (no
    method call) so `.rev()`, `.step_by()`, and the broader iterator
    machinery stay deferred.
  - The Book's range starts at `1` and ends before `4`, producing
    `1, 2, 3`. This lesson's range starts at `0` and ends before `3`,
    producing `0, 1, 2`. Starting at `0` makes the
    "N values for `0..N`" rule visually obvious in the output and
    keeps the count-of-values arithmetic trivial.

- The local probe (single working transcript), captured below.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/022-for-range.rs`.
The committed file is the exact working program used in Try It.

Probe transcript, run in a clean temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
main.rs
--- cat main.rs ---
fn main() {
    for i in 0..3 {
        println!("i = {i}");
    }
    println!("done");
}
--- rustc main.rs ---
exit=0
--- ls after compile ---
main
main.rs
--- ./main ---
i = 0
i = 1
i = 2
done
exit=0
```

Notes:

- `rustc main.rs` exits 0 and is silent (consistent with lesson 001).
- `./main` prints exactly four lines: `i = 0`, `i = 1`, `i = 2`,
  `done`. The first three come from the loop body (one print per
  pass, with `i` taking values `0`, `1`, `2` in order), and the
  fourth comes from the `println!("done");` *after* the loop, which
  fires once the range is exhausted.
- The range `0..3` produced exactly three values: `0`, `1`, `2`.
  The body never prints `i = 3`. This is the load-bearing observation
  for the exclusive-upper-bound claim.
- The body of the program contains no `let mut`, no `n < 3`
  comparison, and no `n = n + 1;` statement. The loop variable `i` is
  bound on each pass without any of the lesson 006 / 009 / 013
  machinery. This is the load-bearing observation for the "auto-
  advanced, no `mut` needed" half of the main concept.
- Only the working source is committed under `observations/`. The
  temp dir was removed; no binaries are committed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `004-statements-in-order` (accepted) — the body of `fn main` runs
  top to bottom in source order. The `for` is itself one step in that
  sequence; the next statement after the loop runs once the range is
  exhausted.
- `005-let-binding` (accepted, load-bearing) — `let name = value;`
  binds a name; `println!("... {name}")` substitutes the bound value.
  Used here as `{i}` to print the loop variable on each pass.
- `017-while-loop` (accepted, precursor) — `while condition { ... }`
  is the previous loop shape, where the learner had to declare a
  `mut` counter, write a comparison, and update the counter manually.
  This lesson installs an alternative shape that needs none of that
  machinery.
