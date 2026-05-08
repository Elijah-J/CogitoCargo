---
id: 027-loop-and-break
move: "write `loop { ... }` to repeat a block forever, and `break;` inside the body to exit it"
main_concept: "`loop { ... }` is Rust's unconditional-repetition construct: it runs the body, then runs it again, forever, with no built-in exit condition; the `break;` statement, used inside the loop body, terminates the loop and resumes execution at the next statement after the closing `}`; `loop` and `break;` are inseparable in practice — a `loop` without a `break` (or other early exit) never returns; together with `while` (lesson 017) and `for` (lesson 022), they form Rust's three loop constructs"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 005-let-binding
  - 006-mut-binding
  - 013-comparison-operators
  - 014-if-else
  - 017-while-loop
  - 019-type-annotation-i32
  - 022-for-range
  - 023-compound-add-assign
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "`break value;` carrying a value out of `loop` (loop-as-expression, fourth surface of lesson 024's expression rule)" moves
  - future "`continue;` skip to next pass" moves
  - future "loop labels `'name: loop { ... }`" moves
  - future "`while let` and `Option`-driven exits" moves
  - future "nested loops and inner-loop scope" moves
sources:
  - output/docs/rust/book/ch03-05-control-flow.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/027-loop-and-break.rs
status: accepted
---

# Repeat forever with `loop`, exit with `break;`

## The Move

Inside `fn main`, write the shape

```rust
loop {
    // body
}
```

The executable will run the body, then run it again, then run it again,
with no built-in stopping condition. To make it actually stop, write

```rust
break;
```

somewhere inside the body. When execution reaches `break;`, the loop
ends and the program continues with the next statement after the
closing `}` of the `loop` block. In practice you almost always pair
`break;` with an `if` (lesson 014) so the loop exits when some
condition becomes `true`.

## Mental Model Delta

- Before: "I have two loop shapes. `while condition { ... }` from
  lesson 017 checks a boolean before each pass and stops when it
  becomes `false`. `for var in 0..N { ... }` from lesson 022 walks a
  range and stops when the range is exhausted. Both have their stopping
  condition built in."
- After: "Rust has a third loop shape, `loop { ... }`, which has *no*
  built-in stop. It just repeats the body forever. To stop it, I use
  `break;` inside the body — typically guarded by an `if` so the loop
  runs until some condition I choose becomes `true`. `loop` plus
  `break;` is the third member of the trio: `while` (condition before
  each pass), `for` (auto-advanced range), and `loop` (manual exit on a
  chosen condition)."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`, silent on success.
  - Lesson 002: body of `fn main` runs when the executable launches.
  - Lesson 005: `let name: TYPE = value;` binds a name; `{name}` in
    `println!` substitutes the bound value.
  - Lesson 006 (load-bearing): `let mut name = value;` makes a binding
    reassignable. The probe binds `let mut counter: i32 = 0;` and then
    updates `counter` inside the loop body. Without `mut`, the update
    would not compile.
  - Lesson 013 (load-bearing): `==` produces a boolean. The probe uses
    `counter == 3` to decide when to break.
  - Lesson 014 (load-bearing): `if condition { ... } else { ... }` runs
    a block based on a boolean. The probe uses `if counter == 3 { break; }`
    *without* an `else` arm. Rust permits dropping the `else` block
    when you only need to act on `true`: this is the same machine from
    lesson 014 with the `else` block omitted, not a separate concept.
  - Lesson 017 (precursor): `while condition { ... }` is the first loop
    construct — condition checked before each pass.
  - Lesson 019: `name: TYPE` attaches a type; used here as
    `let mut counter: i32 = 0;` so the counter's kind is explicit.
  - Lesson 022 (precursor): `for var in 0..N { ... }` is the second
    loop construct — auto-advanced over a range.
  - Lesson 023 (load-bearing): `n += 1;` is shorthand for
    `n = n + 1;` (requires `mut`). The probe uses `counter += 1;` to
    update the counter on each pass.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let mut counter: i32 = 0;
    loop {
        counter += 1;
        if counter == 3 {
            break;
        }
    }
    println!("counter = {counter}");
}
```

The Book describes the `loop` shape directly:

> The `loop` keyword tells Rust to execute a block of code over and
> over again either forever or until you explicitly tell it to stop.

And on `break`:

> Rust also provides a way to break out of a loop using code. You can
> place the `break` keyword within the loop to tell the program when
> to stop executing the loop.

So in our program: `loop { ... }` repeats its body with no stopping
condition built in; the `break;` statement inside the body is what
"explicitly tells it to stop".

Compile and run:

```console
$ rustc demo.rs
$ ./demo
counter = 3
```

Walk through it pass by pass. `counter` starts at `0` (lesson 006
binds it `mut` so the body can update it; lesson 019 spells the kind
out as `i32`).

- *Pass 1.* The loop runs the body. `counter += 1;` makes `counter`
  equal `1` (lesson 023). The `if`: `counter == 3` is `1 == 3`, which
  is `false` (lesson 013); the inner `{ break; }` is *skipped* (lesson
  014, with the `else` block dropped because we only care about the
  `true` case). The body ends. The loop iterates: it runs the body
  again.
- *Pass 2.* `counter += 1;` makes `counter` equal `2`. `2 == 3` is
  `false`; the inner block is skipped. Iterate.
- *Pass 3.* `counter += 1;` makes `counter` equal `3`. `3 == 3` is
  `true`. The inner `{ break; }` runs. `break;` terminates the loop.
- Execution resumes at the next statement after the `loop { ... }`
  block: `println!("counter = {counter}");` prints `counter = 3`.

The structural difference from lessons 017 and 022 is that `loop`
itself has no condition and no range. The body chooses when to stop
by reaching a `break;`. Remove the `if counter == 3 { break; }` block
and the loop has no exit at all: `counter` would keep going up
forever, the program would never reach the `println!`, and you would
have to stop it with Control-C. (Do not actually run that program; the
property is calibrated in prose, not exercised in a probe.)

## What Changed

- You can write `loop { ... }` inside `fn main` and have the
  executable repeat the body with no built-in stopping condition.
- You can write `break;` inside the body to terminate the loop and
  continue with the next statement after the closing `}`.
- You know `loop` and `break;` are paired in practice: a `loop`
  without a `break` (or other early exit) never returns. The natural
  shape is a `loop` whose body contains an `if` that calls `break;`
  on the chosen exit condition.
- You can place Rust's three loop constructs against each other:
  `while` (lesson 017) checks a condition before each pass; `for`
  (lesson 022) auto-advances over a range; `loop` (this lesson) just
  repeats and relies on `break;` for exit. Each is the right tool for
  a different shape of repetition.
- You have a name for the no-`else` form used in the probe: it is
  the same `if` from lesson 014, with the `else` block dropped because
  there is no second case to run.

## Check Yourself

You write `count.rs` containing:

```rust
fn main() {
    let mut k: i32 = 0;
    loop {
        k += 2;
        if k == 6 {
            break;
        }
    }
    println!("k = {k}");
}
```

You run `rustc count.rs` and then `./count`.

(a) What does it print?

(b) How many times does the body of the `loop` run before the program
prints `k = 6`?

(c) If you delete the `if k == 6 { break; }` block from the body
(leaving everything else alone), what happens when you run `./count`?

(Answers: (a) `k = 6`. (b) Three times: `k` becomes `2`, then `4`,
then `6`; on the third pass `k == 6` is `true`, so `break;` runs.
(c) The loop has no exit. `k` would keep climbing — `8`, `10`, `12`,
... — and the program would never reach the `println!`. You would
have to stop it with Control-C.)

## What To Ignore For Now

This lesson installs only one idea: `loop { ... }` repeats forever,
and `break;` inside the body terminates it. Each of the following is
real but *not* part of this move:

- *`break value;` carrying a value out of the loop*. The Book's very
  next subsection ("Returning Values from Loops") makes the whole
  `loop { ... }` construct an expression whose value is whatever
  follows the `break`, applying lesson 024's expression rule to a
  fourth surface (after let-block, function body, and `if`/`else`
  arms). Deferred to a separate move.
- *`continue;`* — a sibling of `break;` that skips the rest of the
  current pass and starts the next pass. Same family, different
  statement; deferred.
- *Loop labels* like `'name: loop { ... break 'name; }`, which let
  `break` and `continue` target an outer loop instead of the
  innermost. The Book has its own subsection on this; deferred.
- *`while let` and `Option`-driven exits* — pattern-matching loop
  forms; deferred (they need pattern matching first).
- *Infinite-loop idioms* such as `loop { read_input(); ... }` for
  user-driven programs; the probe terminates by design.
- *The unit type `()`* and what `loop { ... }` evaluates to when used
  without `break value;`. Still deferred (calibrated in lessons
  021/024/025/026).
- All previously deferred items: shadowing, `&` references, generics,
  `cargo`, modules and `pub`, the broader format-string DSL, and
  `mut` scope details beyond what 006 / 023 installed.

## Evidence

### Sources

- `output/docs/rust/book/ch03-05-control-flow.md`, the
  "Repetition with Loops" framing (lines 257-264) and the
  "Repeating Code with `loop`" subsection (lines 266-320). Three
  load-bearing direct quotes:
  - Line 264: "Rust has three kinds of loops: `loop`, `while`, and
    `for`." This licenses the lesson's framing of `loop` + `break`
    as the *third* loop construct, alongside `while` (lesson 017) and
    `for` (lesson 022).
  - Lines 268-269: "The `loop` keyword tells Rust to execute a block
    of code over and over again either forever or until you
    explicitly tell it to stop." This is the corpus statement that
    licenses the lesson's main concept for `loop` (unconditional
    repetition, no built-in exit).
  - Lines 311-313: "Rust also provides a way to break out of a loop
    using code. You can place the `break` keyword within the loop to
    tell the program when to stop executing the loop." This grounds
    the `break;` half of the move: a statement, placed inside the
    loop body, that terminates the loop.

  Calibration:
  - The Book builds with `cargo run`; this lesson uses `rustc demo.rs`
    per lesson 001. Behavior is identical.
  - The Book's first `loop` example is `loop { println!("again!"); }`
    (lines 277-281) and *intentionally* runs forever to teach the
    "press Control-C to stop" reflex. This lesson uses a
    counter-driven `break;` (lessons 006 / 019 / 023 / 013 / 014) so
    the program terminates cleanly and can be captured with a probe
    transcript. The two framings agree: the Book's example shows
    `loop` *without* `break` (hangs); ours shows `loop` *with*
    `break;` (the canonical pair). The Book itself transitions from
    one framing to the other when it introduces `break` at line 311.
  - The next subsection in the Book ("Returning Values from Loops",
    lines 321-355) introduces the `break value;` form that makes
    `loop { ... }` an expression whose value can be bound with `let`.
    That is a separate move — the fourth surface of lesson 024's
    expression rule — and is explicitly deferred under What To Ignore
    For Now.
  - The Book mentions `continue` once in passing (lines 317-319) as a
    sibling of `break`. Also deferred under What To Ignore For Now.
  - The Book's later "Disambiguating with Loop Labels" subsection
    (lines 359+) is its own future move.
  - Lesson 014's official move was the with-`else` form
    `if cond { ... } else { ... }`. The probe drops the `else` block.
    This is not a new concept: it is the same `if` machine from lesson
    014 with the `else` block omitted because we only need to act on
    `true`. The lesson notes the omission in one sentence and does
    *not* install if-without-else as a separate move.

- The local probe (single working transcript), captured below.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/027-loop-and-break.rs`.
The committed file is the exact working program used in Try It.

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
    let mut counter: i32 = 0;
    loop {
        counter += 1;
        if counter == 3 {
            break;
        }
    }
    println!("counter = {counter}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
counter = 3
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (consistent with lesson 001).
- `./demo` prints exactly one line: `counter = 3`. This single line of
  output is jointly the load-bearing observation for both halves of
  the move:
  - The line printed *at all*: `loop` did not run forever — the
    `break;` inside the `if` block did terminate it. If `break;` did
    not exit the loop, the program would never reach the
    `println!("counter = {counter}");` statement that follows the
    closing `}`, and `./demo` would either hang or print nothing.
  - The line's contents (`counter = 3`): the loop ran the body
    exactly three times before exiting, with `counter` taking the
    values `1`, `2`, `3` in order. The exit happened on the pass
    where the inner `if` condition became `true`, not before.
- The body has no `while` and no `for in` — the only repetition
  machinery is the bare `loop` keyword. This is the load-bearing
  observation for the "third construct, distinct from `while` and
  `for`" framing.
- No broken-contrast probe is captured. A `loop {}` with no `break`
  hangs forever; the property is mentioned in prose but deliberately
  not exercised, so no transcript of a hanging program exists in this
  repository.
- Only the working source is committed under `observations/`. The
  temp dir was removed; no binaries are committed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `005-let-binding` (accepted) — `let name = value;` binds a name;
  `println!("... {name}")` substitutes the bound value at print time.
- `006-mut-binding` (accepted, load-bearing) — `let mut name = value;`
  makes a binding reassignable. Without `mut`, the body's update of
  `counter` would not compile.
- `013-comparison-operators` (accepted, load-bearing) — `==` produces
  a boolean. Used as the inner condition `counter == 3` that decides
  when to break.
- `014-if-else` (accepted, load-bearing) — `if condition { ... }
  else { ... }` runs a block based on a boolean. The probe drops the
  `else` block because the move only needs to act on `true`; this is
  the same machine, not a new concept.
- `017-while-loop` (accepted, precursor) — the first loop construct;
  cited in the trio framing.
- `019-type-annotation-i32` (accepted) — `name: TYPE` between the
  binding name and the `=`. Used as `let mut counter: i32 = 0;`.
- `022-for-range` (accepted, precursor) — the second loop construct;
  cited in the trio framing.
- `023-compound-add-assign` (accepted, load-bearing) — `n += 1;` is
  shorthand for `n = n + 1;` and requires `mut`. Used as
  `counter += 1;` to advance the counter on each pass.
