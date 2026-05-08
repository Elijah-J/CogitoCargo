---
id: 017-while-loop
move: "write `while condition { ... }` to repeat a block as long as the condition is `true`"
main_concept: "`while condition { ... }` checks the condition before each pass; if `true`, the block runs, then the condition is checked again; if `false`, the loop exits and execution continues with the next statement; the body must update something the condition depends on, otherwise the loop never ends"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
  - 005-let-binding
  - 006-mut-binding
  - 009-arithmetic-on-integers
  - 013-comparison-operators
  - 014-if-else
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "loop / break / continue" moves
  - future "for loops" moves
  - future "while let" moves
  - future "loop labels" moves
  - future "compound assignment operators" moves
sources:
  - output/docs/rust/book/ch03-05-control-flow.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/017-while-loop.rs
status: accepted
---

# Repeat a block with `while condition { ... }`

## The Move

Inside `fn main`, write the shape

```rust
while condition {
    // body
}
```

where `condition` is anything that produces a `true` or `false` —
typically a comparison from lesson 013. When the executable reaches
this construct it checks the condition; if `true`, it runs the body
once, then checks the condition again; it keeps repeating that
check-then-run until the condition becomes `false`, at which point it
stops and moves on to whatever statement comes after the loop. For the
loop to ever stop, the body must change something the condition
depends on.

## Mental Model Delta

- Before: "I have `if condition { A } else { B }` (lesson 014), which
  checks a boolean and runs one block once. I have no way to repeat a
  block."
- After: "Rust has `while condition { ... }`, which *repeats* its block
  as long as the condition holds. Each pass: check the condition; if
  `true`, run the body, then check again; if `false`, exit and
  continue with the next statement. It is essentially an `if` whose
  block runs over and over until the condition flips. The body
  usually updates a `mut` binding so that the condition eventually
  becomes `false` and the loop stops."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`, silent on success.
  - Lesson 002: body of `fn main` runs when the executable launches.
  - Lesson 004: statements in `fn main` run top to bottom; the
    `while` is itself one such step, and the next statement after it
    runs once the condition becomes `false`.
  - Lesson 005: `let name = value;` binds a name; `{name}` in
    `println!` substitutes the bound value.
  - Lesson 006 (load-bearing): `let mut name = value;` makes a
    binding reassignable so a later `name = new_value;` works.
    Without `mut`, the body could not update the counter, and the
    loop would never make progress.
  - Lesson 009 (load-bearing): `+` between two integers fits on the
    right of `=`; used as `n + 1` to advance the counter.
  - Lesson 013 (load-bearing): comparisons like `n < 3` produce a
    boolean. Used as the loop condition.
  - Lesson 014 (precursor): the "check a boolean and run a block on
    `true`" pattern. `while` runs that check repeatedly, before each
    pass.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `main.rs`
containing exactly:

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

The Book describes this shape directly:

> A program will often need to evaluate a condition within a loop.
> While the condition is `true`, the loop runs.

So in our program: `n < 3` is the condition; the two-line body inside
`{ ... }` is what gets repeated; `println!("done");` is the statement
*after* the loop, which only runs once the loop exits.

Compile and run:

```console
$ rustc main.rs
$ ./main
n = 0
n = 1
n = 2
done
```

Walk through it pass by pass. `n` starts at `0` (lesson 006 binds it
mutable so we can update it later).

- *Check 1.* Condition `n < 3` is `0 < 3`, which is `true`. Run the
  body: print `n = 0`, then `n = n + 1;` updates `n` to `1`.
- *Check 2.* Condition `n < 3` is `1 < 3`, `true`. Print `n = 1`, then
  `n` becomes `2`.
- *Check 3.* Condition `n < 3` is `2 < 3`, `true`. Print `n = 2`, then
  `n` becomes `3`.
- *Check 4.* Condition `n < 3` is `3 < 3`, which is `false`. Exit the
  loop. Move on to the next statement: `println!("done");`. That prints
  `done`.

Two things make this terminate. `mut` (lesson 006): without it,
`n = n + 1;` would not compile. And the arithmetic `n + 1` (lesson
009): the body has to actually change something the condition looks
at. If the body printed `n` but never updated it, `n < 3` would stay
`true` forever, the body would keep running, and the program would
hang. (You would have to stop it with Control-C.) The Book restates
the rule at the end of the section:

> While a condition evaluates to `true`, the code runs; otherwise, it
> exits the loop.

The Book's own example uses `number -= 1;` (a *compound assignment*
operator) and counts down. This lesson uses `n = n + 1;` because
compound assignment is a separate later move.

## What Changed

- You can write `while condition { ... }` inside `fn main` and have
  the executable repeat the body as long as the condition is `true`.
- You know the per-pass rhythm: check, run if `true`, check again.
  When the check returns `false`, the loop exits and execution
  continues with the next statement.
- You know the load-bearing rule for termination: the body must change
  something the condition depends on. With a plain counter, that
  usually means a `mut` binding (lesson 006) updated by arithmetic
  (lesson 009).
- You can read `while` as a generalization of lesson 014's `if`:
  instead of checking once and running one block, it checks repeatedly
  and runs the same block again and again.

## Check Yourself

You write `count.rs` containing:

```rust
fn main() {
    let mut k = 0;
    while k < 2 {
        println!("k = {k}");
        k = k + 1;
    }
    println!("after");
}
```

You run `rustc count.rs` and then `./count`.

(a) How many lines does it print, in what order?

(b) What is the value of `k` at the moment the program prints `after`?
(You do not see it printed, but you can reason about it from the
condition.)

(c) If you delete `k = k + 1;` from the body, what happens when you
run `./count`?

(Answers: (a) Three lines: `k = 0`, `k = 1`, `after`. (b) `k` is `2`;
the loop exits the first time `k < 2` is `false`, which is when `k`
becomes `2`. (c) The loop never exits: `k` stays `0`, `k < 2` stays
`true`, the body keeps printing `k = 0`, and the program hangs until
you stop it with Control-C.)

## What To Ignore For Now

This lesson installs only one idea: `while condition { ... }` repeats
its body as long as the condition is `true`, checking before each
pass. Deferred:

- The unconditional `loop { ... }` form, plus `break` and `continue`.
  The Book introduces `loop` first; this lesson skips ahead to `while`.
- *`loop` as an expression that returns a value via `break value;`*.
- *`for` loops* over collections or ranges, e.g. `for i in 0..3 { ... }`.
  The Book's very next subsection.
- *`while let`* patterns; a pattern-matching loop variant.
- *Loop labels* like `'name: loop { ... }`.
- *Compound assignment operators* `+=`, `-=`, `*=`, `/=`. The Book's
  `while` example uses `number -= 1;`; this lesson writes `n = n + 1;`
  because compound assignment is a separate later move.
- *Infinite loops* where the condition never becomes `false`. The
  probe terminates by design; the failure mode is mentioned in passing
  but not exercised.
- *Iterating over a collection with `while`* (the Book's
  `while index < 5` array example).
- All previously deferred items: shadowing, type annotations,
  the broader format-string DSL, defining your own functions, function
  parameters and return values, `cargo`, `match`, `if let`, logical
  operators inside conditions.

## Evidence

### Sources

- `output/docs/rust/book/ch03-05-control-flow.md`, the "Streamlining
  Conditional Loops with `while`" subsection (lines 413-444). Two
  load-bearing direct quotes:
  - Lines 415-417: "A program will often need to evaluate a condition
    within a loop. While the condition is `true`, the loop runs. When
    the condition ceases to be `true`, the program calls `break`,
    stopping the loop." This is the corpus statement that licenses the
    lesson's main concept (check-then-run, repeat until false). The
    "calls `break`" phrase here describes how you would *implement* the
    same pattern with `loop` plus `if`/`break`; the next sentence in
    the corpus introduces `while` as the built-in shorthand for that
    pattern, which is what this lesson teaches.
  - Lines 442-444: "While a condition evaluates to `true`, the code
    runs; otherwise, it exits the loop." Independent corpus
    restatement at the end of the subsection. This is the version
    quoted under Try It as the "wrap-up" framing.

  Calibration:
  - The Book's example program (lines 426-438) uses
    `let mut number = 3;`, the condition `number != 0`, the body
    `println!("{number}!"); number -= 1;`, and a closing
    `println!("LIFTOFF!!!");`. It counts *down* from `3` to `1`,
    exits when `number` reaches `0`, and uses the compound assignment
    operator `-=`. This lesson's probe uses `let mut n = 0;`, the
    condition `n < 3`, the body `println!("n = {n}"); n = n + 1;`, and
    a closing `println!("done");`. It counts *up* from `0` to `2`,
    exits when `n` reaches `3`, and uses plain reassignment plus
    arithmetic. Differences and reasons:
    - `n = n + 1;` instead of `number -= 1;`: compound assignment
      operators are listed in What To Ignore For Now and reserved for
      a later move. Plain `name = name + 1;` only requires lesson 006
      (`mut` reassignment) and lesson 009 (integer `+`), both of which
      are already accepted.
    - `n < 3` instead of `number != 0`: `<` is one of lesson 013's six
      comparison operators and is already accepted; `!=` is too, but
      `<` makes the bound (`3`) easier to read off as the stop point
      and matches the natural counting-up direction.
    - Counts up rather than down for the same reason: with `n` starting
      at `0` and stopping at `3`, the body runs the same number of
      passes (three) and the printed values stay positive.
    - Three passes (`n = 0`, `n = 1`, `n = 2`) match the Book's three
      passes (`3!`, `2!`, `1!`); the per-pass behavior the lesson
      teaches is the same.

  The same chapter continues into "Looping Through a Collection with
  `for`" at line 446. That, plus the prior `loop`/`break` and
  `loop`-returning-a-value subsections, are explicitly deferred under
  What To Ignore For Now.

- The local probe (single working transcript), captured below.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/017-while-loop.rs`.
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
    let mut n = 0;
    while n < 3 {
        println!("n = {n}");
        n = n + 1;
    }
    println!("done");
}
--- rustc main.rs ---
exit=0
--- ls after compile ---
main
main.rs
--- ./main ---
n = 0
n = 1
n = 2
done
exit=0
```

Notes:

- `rustc main.rs` exits 0 and is silent (consistent with lesson 001).
- `./main` prints exactly four lines: `n = 0`, `n = 1`, `n = 2`,
  `done`. The first three come from the loop body (one print per pass,
  with `n` taking values `0`, `1`, `2` in order), and the fourth comes
  from the `println!("done");` *after* the loop, which only fires once
  the condition has become `false`.
- The number of passes is determined by the condition. `n` starts at
  `0` and the loop exits the first time `n < 3` is `false`, which
  happens when `n` reaches `3` (i.e., right after the third pass). The
  body never prints `n = 3`, because the condition is checked *before*
  each pass; on the fourth check, `3 < 3` is `false`, the body is
  skipped, and execution continues with `done`. This is the
  load-bearing observation for the lesson's main concept: the check
  happens before the body runs, and exit is at the next failed check.
- `mut` and `n + 1` are jointly load-bearing for termination. With
  either removed, the loop would not progress: drop `mut` and `rustc`
  rejects the program with E0384 (lesson 006); keep `mut` but drop the
  `n = n + 1;` line and the loop runs forever (mentioned but not
  exercised).
- Only the working source is committed under `observations/`. The temp
  dir was removed; no binaries are committed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `004-statements-in-order` (accepted) — the body of `fn main` runs
  top to bottom in source order. The `while` is itself one step in
  that sequence; the next statement after the loop runs once the
  condition becomes `false`.
- `005-let-binding` (accepted) — `let name = value;` binds a name;
  `println!("... {name}")` substitutes the bound value.
- `006-mut-binding` (accepted, load-bearing) — `let mut name = value;`
  makes a binding reassignable; a bare `name = new_value;` statement
  reassigns it. Without `mut`, the body's `n = n + 1;` would not
  compile.
- `009-arithmetic-on-integers` (accepted, load-bearing) — `+` between
  two integers produces a new integer that fits on the right of `=`.
  Used as `n + 1` to advance the counter.
- `013-comparison-operators` (accepted, load-bearing) — comparisons
  like `n < 3` produce a boolean. Used as the loop condition.
- `014-if-else` (accepted, precursor) — established the "check a
  boolean condition and run a block when it is `true`" pattern.
  `while` is the repeated version of that check.
