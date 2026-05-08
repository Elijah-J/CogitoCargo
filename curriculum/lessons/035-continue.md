---
id: 035-continue
move: "write `continue;` inside a loop body to skip the rest of the current pass and immediately start the next pass; the loop's stop condition still applies"
main_concept: "`continue;` is a sibling of lesson 027's `break;` — both are loop-control statements that interrupt the body, but they do opposite things; `break;` *terminates* the loop and resumes execution after the closing `}`; `continue;` *skips the rest of the current body* and returns to the loop head, where the loop continues running; in a `for var in 0..N` loop (lesson 022) the head advances `var` to the next value in the range; in a `while condition` loop (lesson 017) the head re-checks the condition; in a `loop { ... }` (lesson 027) the head re-runs the body from the top; the natural shape is `if condition { continue; }` — skip the rest when some predicate is true; together with `break;` (lesson 027) and `break value;` (lesson 028), `continue;` completes the loop-control trio; unlike `break value;`, there is no `continue value;` form"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 005-let-binding
  - 006-mut-binding
  - 013-comparison-operators
  - 014-if-else
  - 019-type-annotation-i32
  - 022-for-range
  - 023-compound-add-assign
  - 027-loop-and-break
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "labeled `continue 'name;` for nested loops" moves
  - future "`continue;` outside a loop body — diagnostic E0268" moves
  - future "`while let` and `Option`-driven exits" moves
  - future "iterator adapter `.filter(|x| ...)` as an alternative to `if cond { continue; }`" moves
  - future "nested loops and inner-loop scope" moves
sources:
  - output/docs/rust/book/ch03-05-control-flow.md
  - output/docs/rust/reference/expressions/loop-expr.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/035-continue.rs
status: accepted
---

# Skip the rest of a loop pass with `continue;`

## The Move

Inside a loop body — `for`, `while`, or `loop` — write

```rust
continue;
```

When execution reaches `continue;`, the rest of the current pass through
the body is skipped, and the loop returns to its head to start the next
pass. The loop itself does *not* terminate: the stop condition that
was already in place (the range exhausting in `for`, the condition
becoming `false` in `while`, the eventual `break;` in `loop`) still
governs when the loop ends. `continue;` only shortcuts one pass. The
natural shape is `if condition { continue; }` — skip the rest of the
body when some predicate is true.

## Mental Model Delta

- Before: "I have one statement that interrupts a loop body: `break;`
  (lesson 027), which terminates the loop and resumes after the closing
  `}`. `break value;` (lesson 028) does the same thing and also carries
  a value out. Both end the loop."
- After: "There is a second loop-control statement, `continue;`, that
  also interrupts the body — but it does the opposite of `break;`.
  `break;` ends the loop. `continue;` skips the rest of *this* pass
  and lets the loop keep running. The loop's normal stop condition is
  unchanged. So the trio is now: `break;` (terminate), `break value;`
  (terminate and produce a value), `continue;` (skip the rest of this
  pass, keep looping)."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`, silent on success.
  - Lesson 002: body of `fn main` runs when the executable launches.
  - Lesson 005: `let name: TYPE = value;` binds a name.
  - Lesson 006 (load-bearing): `let mut name = value;` makes a binding
    reassignable. The probe binds `let mut count: i32 = 0;` and
    updates `count` inside the loop body. Without `mut`, the update
    would not compile.
  - Lesson 013 (load-bearing): `==` produces a boolean. The probe uses
    `n == 2` as the predicate that gates the `continue;`.
  - Lesson 014 (load-bearing): `if condition { ... }` runs a block based
    on a boolean. The probe uses `if`-without-`else` (same
    micro-extension grounded in lessons 027 and 028 — same `if` machine
    with the `else` block omitted, not a separate concept).
  - Lesson 019: `name: TYPE` attaches a type; `let mut count: i32 = 0;`.
  - Lesson 022 (load-bearing): `for var in 0..N { ... }` runs the body
    once for each integer in `0..N`, with `var` bound to the current
    integer. The probe uses `for n in 0..5`.
  - Lesson 023: `n += 1;` is shorthand for `n = n + 1;` (requires
    `mut`). Used as `count += 1;`.
  - Lesson 027 (load-bearing): `break;` is a loop-control statement
    that terminates the loop. **This lesson installs `continue;` as
    `break;`'s sibling — same loop-interrupt mechanism, opposite
    effect.**
- Background but not load-bearing: lesson 028 (`break value;`) is named
  in the trio framing; lessons 030/031 (`match`) and 033/034 (numeric
  types) are unrelated.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let mut count: i32 = 0;
    for n in 0..5 {
        if n == 2 {
            continue;
        }
        count += 1;
    }
    println!("count = {count}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
count = 4
```

Walk it pass by pass. `count` starts at `0` (lessons 005, 006, 019). The
`for n in 0..5` loop runs the body once for each value of `n` taking
`0`, `1`, `2`, `3`, `4` in order (lesson 022).

- *Pass `n = 0`*: `if n == 2` is `false` (lesson 013). The inner block
  is skipped (lesson 014, with the `else` block dropped). `count += 1;`
  runs (lesson 023). `count` is `1`.
- *Pass `n = 1`*: same path. `count` becomes `2`.
- *Pass `n = 2`*: `if n == 2` is `true`. The inner block runs. The
  body of the inner block is `continue;`. The rest of the loop body —
  specifically, the `count += 1;` line that comes *after* the closing
  `}` of the `if` — is skipped. The loop returns to its head and
  advances `n` to the next value in the range. `count` is still `2`.
- *Pass `n = 3`*: `if n == 2` is `false` again. `count += 1;` runs.
  `count` is `3`.
- *Pass `n = 4`*: same. `count` is `4`.
- The range `0..5` is now exhausted. The loop exits normally (the
  same way it would have exited without any `continue;`). Execution
  resumes with the next statement after the closing `}` of the loop:
  `println!("count = {count}");` prints `count = 4`.

Now do the contrast in your head against lesson 027's `break;`. Suppose
the inner block had been `break;` instead of `continue;`. Then on
`n = 2`, the loop would have *terminated* — `n` would not advance to
`3` and `4`, and the trailing `println!` would print `count = 2` (only
the `n = 0` and `n = 1` passes incremented before the break). With
`continue;` the loop instead keeps going: `n` does advance to `3` and
`4`, and `count` ends at `4`. Same surface (`if n == 2 { ... }`),
opposite effect on the loop.

The Reference states the rule directly:

> When `continue` is encountered, the current iteration of the
> associated loop body is immediately terminated, returning control
> to the loop head.

And it specifies what "the head" means for the three loop shapes you
already have: in a `while` loop, the head is the condition operands
(re-checked); in a `for` loop, the head is the call-expression that
yields the next value (next item taken from the range); in a plain
`loop`, the head is just the start of the body again (the Reference
implies this through the `loop`-as-base-case framing — `while` and
`for` both desugar to `loop` plus a `match`, see lines 131-152 and
242-271 of the corpus file). So `continue;` works in all three of the
loop constructs you already know.

## What Changed

- You can now write `continue;` inside a `for`, `while`, or `loop`
  body. It skips the rest of the current pass and returns to the loop
  head. The loop keeps running; only this one pass is shortened.
- You can place `continue;` against `break;` (lesson 027): both
  interrupt the loop body, but they do opposite things. `break;`
  terminates the loop. `continue;` skips one pass.
- The natural shape is `if condition { continue; }` — same
  no-`else` shape as lessons 027 and 028, used to gate the
  `continue;` on a predicate.
- The loop-control trio is now complete: `break;` (terminate),
  `break value;` (terminate with a value, lesson 028), `continue;`
  (skip the rest of this pass).
- Asymmetry to remember: `break value;` exists, `continue value;` does
  *not*. `continue;` only ever takes a label (future move). There is
  no value to carry out, because the loop is being resumed, not
  ended.

## Check Yourself

You write `skip.rs` containing:

```rust
fn main() {
    let mut total: i32 = 0;
    for n in 0..6 {
        if n == 0 {
            continue;
        }
        total += n;
    }
    println!("total = {total}");
}
```

You run `rustc skip.rs && ./skip`.

(a) What does it print?

(b) Which value of `n` is *not* added into `total`?

(c) If you replace `continue;` with `break;` (leaving everything else
alone), what does the program print?

(Answers: (a) `total = 15`. The loop runs for `n` taking `0`, `1`, `2`,
`3`, `4`, `5`. On `n = 0`, `if n == 0` is `true`, so `continue;` runs
and `total += n;` is skipped on that pass. On the other five passes
`total += n;` runs, adding `1 + 2 + 3 + 4 + 5 = 15`. (b) `0`. (c) With
`break;`, the loop terminates on `n = 0` before `total += n;` ever
runs on any pass; `total` is still `0`, so the program prints
`total = 0`.)

## What To Ignore For Now

This lesson installs only one idea: `continue;` skips the rest of the
current pass and returns to the loop head. Each of the following is
real but *not* part of this move:

- *Labeled `continue 'name;`* — for nested loops, lets `continue` apply
  to an outer loop instead of the innermost. Pairs with labeled
  `break 'name;` and the Book's "Disambiguating with Loop Labels"
  subsection (line 359+). Future move.
- *`continue;` outside a loop body* — rejected by rustc with
  `error[E0268]: continue outside of a loop` and the message
  `cannot continue outside of a loop`. Mentioned for completeness;
  a separate diagnostic-walking move could exercise it.
- *`continue value;`* — does *not* exist in Rust, unlike lesson 028's
  `break value;`. The Reference's continue grammar
  (`continue LIFETIME_OR_LABEL?`) shows no expression slot. Asymmetry
  named in `## What Changed`; nothing further to install.
- *Interaction with `break value;` in the same loop* — both can appear
  in the same `loop` body, with `continue;` skipping passes and
  `break value;` exiting with a value. Composition of two installed
  moves; no new concept.
- *Skipping with `if`/`else` instead of `continue;`* — sometimes more
  readable as `if !skip { count += 1; }`. The lesson installs
  `continue;` as one valid shape; it does not lobby for one over the
  other.
- *Iterator adapters like `.filter(|x| ...)`* — a higher-level way to
  skip elements without writing `continue;`. Needs iterator and
  closure moves first.
- *`while let` and `Option`-driven loops* — deferred (need pattern
  matching).
- All previously deferred items: shadowing, `&` references, generics,
  `cargo` (lesson 032 is installed but this lesson uses `rustc` per
  lesson 001), the broader format-string DSL.

## Evidence

### Sources

- `output/docs/rust/book/ch03-05-control-flow.md`, lines 317-319 (the
  one sentence the Book devotes to `continue` in this chapter):

  > We also used `continue` in the guessing game, which in a loop
  > tells the program to skip over any remaining code in this
  > iteration of the loop and go to the next iteration.

  This is the Book's plain-English statement of the move: skip the
  rest of *this* iteration, go to the *next* iteration. It licenses
  the lesson's main concept in everyday phrasing. The Book introduces
  `continue` in passing, referring back to its earlier guessing-game
  chapter (Chapter 2), which is *not* visited by this run; we use the
  Book quote for the plain-English statement and the Reference for the
  formal definition.

- `output/docs/rust/reference/expressions/loop-expr.md`, the
  `continue` expressions section (lines 441-473). Two load-bearing
  excerpts:

  Lines 451-453 (the canonical formal definition):

  > When `continue` is encountered, the current iteration of the
  > associated loop body is immediately terminated, returning control
  > to the loop *head*.

  Lines 459-465 (what "the head" means for two of the three loop
  shapes already installed in this run):

  > In the case of a `while` loop, the head is the conditional
  > operands controlling the loop.
  >
  > In the case of a `for` loop, the head is the call-expression
  > controlling the loop.

  The Reference does not give an explicit "head" definition for the
  bare `loop` construct. It is implied through two facts: (1) the
  Reference's framing in lines 20-25 lists `loop`, `while`, `for` as
  three loop expressions all of which support `continue` (line 33
  states `All except labeled block expressions support continue
  expressions`), and (2) the Reference shows that `for` desugars to
  `loop` plus a `match` (lines 242-271) and `while` desugars to
  `loop` plus a `match` (lines 131-152). In the bare `loop` shape the
  body simply restarts from the top, since there is no condition or
  iterator to consult. This composition reading is what the lesson
  asserts about `loop`'s head, and it is how the Reference uses the
  three constructs together.

  Line 473 also licenses the deferral note about `continue;` outside a
  loop:

  > A `continue` expression is only permitted in the body of a loop.

  Calibration:
  - `continue;` is also a *labeled* expression: the grammar at
    line 449 is `continue LIFETIME_OR_LABEL?`, meaning `continue;` and
    `continue 'name;` are both syntactically valid. This lesson
    installs only the unlabeled form. Labeled `continue` is a future
    move.
  - The Reference (line 457) also calls `continue` "diverging" with
    type `!`, the never type. Same calibration as `break` in lesson
    027 — the type-theoretic name is deferred; for now, treat
    `continue;` as a statement that interrupts control flow.
  - The Book builds with `cargo run`; this lesson uses `rustc demo.rs`
    per lesson 001. Behavior is identical.
  - Lesson 014's official move was the with-`else` form
    `if cond { ... } else { ... }`. The probe drops the `else` block,
    the same micro-extension grounded in lessons 027 and 028 — same
    `if` machine, not a new concept, not added to the graph.

- The local probe (single working transcript), captured below.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/035-continue.rs`.
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
    let mut count: i32 = 0;
    for n in 0..5 {
        if n == 2 {
            continue;
        }
        count += 1;
    }
    println!("count = {count}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
count = 4
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (consistent with lesson 001).
- `./demo` prints exactly one line: `count = 4`. That single value is
  the load-bearing observation for both halves of the move:
  - The line printed *at all*: the loop ran to completion. If
    `continue;` had terminated the loop (the way `break;` does), the
    range would not have exhausted; but it did, and the program
    reached the trailing `println!`.
  - The contents `count = 4`: out of five passes (`n` taking `0`,
    `1`, `2`, `3`, `4`), `count += 1;` ran on four of them. The pass
    that did *not* increment is exactly the one where `if n == 2` was
    `true` and `continue;` ran, skipping the trailing `count += 1;`
    line on that single pass. This is the *new* observation for this
    lesson — `continue;` skips the rest of *one* pass, not the whole
    loop.
- The probe's `for` body has the form
  `if condition { continue; } REST_OF_BODY`. The `continue;` is the
  *only* statement inside the `if` block, so all of `REST_OF_BODY`
  (just `count += 1;` here) is what gets skipped on the `n = 2` pass.
- No broken-contrast probe is captured under `observations/`. The
  prose calibration in `## Try It` notes that swapping `continue;` for
  `break;` would print `count = 2`; that property was confirmed
  locally (`rustc` on the swapped source compiled and `./demo` printed
  `count = 2`), but the swapped source is not committed and no
  transcript is reproduced — the affirmative behavior is the load-
  bearing observation, and the contrast is in prose only.
- Likewise, the deferral note about `continue;` outside a loop
  (rejected with `error[E0268]: continue outside of a loop`) was
  confirmed locally on a tiny `fn main() { continue; }` source but is
  not committed; that diagnostic is its own future move.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `005-let-binding` (accepted) — `let name: TYPE = value;` binds a
  name.
- `006-mut-binding` (accepted, load-bearing) — `let mut name = value;`
  makes a binding reassignable. Without `mut`, `count += 1;` in the
  loop body would not compile.
- `013-comparison-operators` (accepted, load-bearing) — `==` produces
  a boolean. Used as the predicate `n == 2` that gates the
  `continue;`.
- `014-if-else` (accepted, load-bearing) — `if condition { ... }` runs
  a block based on a boolean. The probe drops the `else` block, same
  micro-extension grounded in lessons 027 and 028.
- `019-type-annotation-i32` (accepted) — `name: TYPE`. Used as
  `let mut count: i32 = 0;`.
- `022-for-range` (accepted, load-bearing) — `for var in 0..N { ... }`
  runs the body once for each integer in `0..N`. The probe uses
  `for n in 0..5`. The "head" of a `for` loop (where `continue;`
  returns control) is the call-expression that yields the next value
  from the range — this is the Reference's wording, lines 463-465.
- `023-compound-add-assign` (accepted) — `n += 1;` is shorthand for
  `n = n + 1;` (requires `mut`). Used as `count += 1;`.
- `027-loop-and-break` (accepted, load-bearing) — `break;` is a
  loop-control statement inside a loop body that terminates the loop.
  **This lesson installs `continue;` as `break;`'s sibling: same
  loop-interrupt mechanism, opposite effect — `break;` ends the loop,
  `continue;` skips one pass.**
- Older lessons in the trio framing (mention only):
  - `017-while-loop` — first loop construct; named in the
    "what is the head" section (the head is the condition).
  - `028-break-value` — closes the loop-control trio with `break;`,
    `break value;`, `continue;`. Asymmetry: `continue value;` does
    not exist.
