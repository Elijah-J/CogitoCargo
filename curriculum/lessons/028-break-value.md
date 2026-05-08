---
id: 028-break-value
move: "add a value after `break` — `break value;` instead of lesson 027's bare `break;` — so the whole `loop { ... }` becomes an expression that produces `value`, bindable by `let`"
main_concept: "`break value;` does two things at once: it terminates the loop exactly as `break;` does in lesson 027, and it carries `value` out of the loop so the whole `loop { ... }` construct itself is an expression whose value is the `value` passed to whichever `break` exited it; this is the *fourth surface* lesson 024's tail-expression rule applies to (after let-block in 024, function body in 025, and `if`/`else` arms in 026); without `break VALUE` (or another value-producing exit), a `loop { ... }` cannot be used as a value-producing expression — rustc rejects it with E0308 mismatched types, naming the unit type `()` as what the loop produces and emitting `help: give the break a value of the expected type` with a literal `break 42;` suggestion"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 005-let-binding
  - 006-mut-binding
  - 009-arithmetic-on-integers
  - 013-comparison-operators
  - 014-if-else
  - 019-type-annotation-i32
  - 023-compound-add-assign
  - 024-statement-vs-expression
  - 027-loop-and-break
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "`continue;` skip-to-next-pass" moves
  - future "loop labels `'name: loop { ... break 'name value; }`" moves
  - future "`return` from inside a loop body (function exit, not just loop exit)" moves
  - future "`while let` and `Option`-driven exits" moves
  - future "`match` as expression" moves
  - future "the unit type `()`" moves
sources:
  - output/docs/rust/book/ch03-05-control-flow.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/028-break-value.rs
status: accepted
---

# Carry a value out of a `loop` with `break value;`

## The Move

Inside `fn main`, write a `loop { ... }` whose body uses

```rust
break counter * 10;
```

(or any expression of the right type) instead of lesson 027's bare
`break;`. Then put the whole `loop { ... }` on the right of a `let`:

```rust
let result: i32 = loop {
    // ...
    break counter * 10;
};
```

The `loop` still terminates exactly the way it did in lesson 027 — by
reaching a `break`. The new piece is that the value following `break`
is also the value the whole `loop { ... }` produces, so `let result:
i32 = loop { ...; break some_i32; };` binds `result` to that `i32`.

## Mental Model Delta

- Before: "`loop { ... }` repeats the body forever; `break;` inside the
  body terminates it (lesson 027). Separately, blocks (lesson 024),
  function bodies (lesson 025), and `if`/`else` arms (lesson 026) are
  expressions — they produce a value. `loop` is a control-flow
  construct, not an expression."
- After: "`loop` is also an expression. The value comes out through
  `break`: writing `break value;` instead of `break;` carries `value`
  out as the loop's value. The whole `loop { ... }` then sits anywhere
  a value can — most directly on the right of `let`. Lesson 024's rule
  has now been applied to four surfaces: let-block, function body,
  `if`/`else` arms, and now `loop`. The carrier is different (a tail
  expression for blocks, a value after `break` for `loop`) but the
  *role* — produce a value the surrounding code can bind — is the same."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`, silent on success.
  - Lesson 002: body of `fn main` runs when the executable launches.
  - Lesson 003 (load-bearing): rustc diagnostics have a headline +
    `-->` location + source excerpt with caret + optional `help:`
    lines. The broken-contrast walkthrough below decodes E0308 with a
    `help:` line that includes a literal source-diff suggestion using
    exactly this skill.
  - Lesson 005 (load-bearing): `let name: TYPE = value;` binds a name;
    reused as the slot the loop's value lands in.
  - Lesson 006: `let mut name = value;` makes a binding reassignable,
    so the body can update `counter`.
  - Lesson 009: integer values and `*`; the probe uses `counter * 10`.
  - Lesson 013: `==` produces a boolean; the probe uses
    `if counter == 5` to pick the exit pass.
  - Lesson 014: `if condition { ... } else { ... }` runs a block based
    on a boolean. The probe drops the `else` block, the same
    micro-extension grounded in lesson 027 — same `if` machine, no new
    concept.
  - Lesson 019 (load-bearing): `name: TYPE` attaches a type. The
    annotation `let result: i32` is what pins the loop's expected type.
    The broken-contrast diagnostic explicitly traces from the `let
    result: i32 = loop {` line back to the `break;` site.
  - Lesson 023: `n += 1;` is shorthand for `n = n + 1;`; used as
    `counter += 1;` to advance the counter.
  - Lesson 024 (load-bearing): a block `{ ... }` is itself an
    expression. **This lesson applies the same rule on a *fourth*
    surface — `loop { ... }` itself as an expression carrying a value
    via `break`.**
  - Lesson 027 (load-bearing): `loop { ... }` repeats forever; `break;`
    inside the body terminates it. **This lesson extends `break` from
    the bare statement form to a value-carrying form.**
  - Older precursors (mention only): lesson 025 (function-body
    surface), lesson 026 (`if`/`else`-arm surface). Cited in the
    four-surface framing; not load-bearing on their own.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 5 {
            break counter * 10;
        }
    };
    println!("result = {result}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
result = 50
```

Walk it. `counter` starts at `0` (lessons 005, 006, 019). The `loop`
runs the body. Each pass, `counter += 1;` (lesson 023) advances the
counter, then `if counter == 5 { ... }` (lesson 014, no `else`) checks
whether to exit on this pass.

- Passes 1-4: `counter` becomes `1`, `2`, `3`, `4`. `counter == 5` is
  `false` (lesson 013). The inner block is skipped. The body ends; the
  loop iterates.
- Pass 5: `counter` becomes `5`. `counter == 5` is `true`. The inner
  block runs. The new piece is the statement inside it,
  `break counter * 10;`. rustc evaluates `counter * 10` to `50`
  (lesson 009 `*`), terminates the loop the same way lesson 027's
  `break;` did, *and* carries the `50` out as the loop's value.

Two things happen at once. First, the loop terminates. Second, the
whole `loop { ... }` evaluates to `50`. The outer `let result: i32 =
...;` binds `result` to that `50` (lesson 005). `println!` prints
`result = 50`.

The Book licenses this directly, in the subsection "Returning Values
from Loops":

> you can add the value you want returned after the `break` expression
> you use to stop the loop; that value will be returned out of the
> loop so that you can use it

So `break counter * 10;` is exactly what the Book describes: the value
`counter * 10` follows the `break` and is the value returned out of
the loop. The Book's own example (lines 330-343 of the source) is the
same shape with different numbers (`counter == 10`, `break counter * 2;`,
prints `result = 20`); ours uses `5` and `* 10` and prints `result = 50`.

Now do the contrast. In the same directory, save a second file
`broken.rs` that is identical except the `break counter * 10;` line is
replaced with a bare `break;`:

```rust
fn main() {
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 5 {
            break;
        }
    };
    println!("result = {result}");
}
```

Compile it. The full transcript and a part-by-part walk live in `##
Evidence`; reading it with lesson 003's order:

- *Headline*: `error[E0308]: mismatched types`. Same E-code as lessons
  024, 025, and 026, with the *generic* "mismatched types" headline (not
  the specialized "if and else have incompatible types" form).
- *`-->` location*: `broken.rs:6:13` — column 13 of line 6, the `break;`
  site itself.
- *Source excerpt*: this diagnostic spans **two locations**. On line
  3, dashes `------` underline `result` with `expected because of
  this assignment`, and dashes `----` underline `loop` with `this loop
  is expected to be of type i32`. On line 6, carets `^^^^^` underline
  `break;` with `expected i32, found ()`.
- *`help:` block*: `help: give the break a value of the expected type`,
  followed by a *source diff* showing line 6 rewritten as `break 42;`
  with `++` under the new `42`.

That diagnostic is rustc encoding this lesson's whole point. The type
flow it traces is: the annotation `let result: i32` (lesson 019)
*expects* the loop to produce an `i32`; the bare `break;` causes the
loop to produce `()` instead; the help line says, literally, "give the
break a value" and shows what that looks like. As in lessons 021, 024,
025, 026, and 027, read `()` as "nothing useful / no value" and defer
the proper move.

## What Changed

- You can now write `break value;` inside a `loop` body. The loop
  still terminates (same as lesson 027); the new piece is that
  `value` becomes the value the whole `loop { ... }` produces.
- The whole `loop { ... }` is now an expression. It sits anywhere a
  value of `value`'s type can — most directly on the right of `let`,
  as in `let result: i32 = loop { ...; break some_i32; };`.
- Closing of the expression-rule arc 024 → 025 → 026 → 027 → 028.
  Lesson 024 named the rule (block-as-expression with a tail
  expression). Lesson 025 applied it to a function body. Lesson 026
  applied it to `if`/`else` arms. Lesson 027 introduced `loop` and
  `break;` as control flow. This lesson closes the arc by making
  `loop` the *fourth* surface — value-producing via `break value;`.
- Without `break VALUE` (or another value-producing exit), a `loop {
  ... }` cannot be bound by `let`. rustc reports it with the generic
  `error[E0308]: mismatched types`, naming `()` as what the loop
  produced, and the `help:` line literally says
  `help: give the break a value of the expected type` with a
  `break 42;` suggestion.
- Lesson 027's `break;` is still valid — it is the right form when the
  loop is being used as a *statement*. The choice between `break;` and
  `break value;` depends on whether you are using the loop as a
  statement or as an expression. Same loop, two `break` shapes for two
  uses.

## Check Yourself

You write `count.rs` containing:

```rust
fn main() {
    let mut k: i32 = 0;
    let answer: i32 = loop {
        k += 2;
        if k == 6 {
            break k + 1;
        }
    };
    println!("answer = {answer}");
}
```

You run `rustc count.rs && ./count`.

(a) What does it print?

(b) The body of the `loop` runs how many times before the program
prints `answer = 7`?

(c) If you change `break k + 1;` back to a bare `break;` (no value),
leaving the rest alone — which line will rustc's `-->` location point
at, and what does the `help:` line literally suggest?

(Answers: (a) `answer = 7`. (b) Three times: `k` becomes `2`, then
`4`, then `6`; on the third pass `k == 6` is `true`, so
`break k + 1;` runs. `k + 1` is `7`; the loop terminates and produces
`7`; `let answer: i32` binds `answer` to `7`. (c) `-->` points at the
`break;` site (column 13 of the line containing `break;`). The `help:`
line says `help: give the break a value of the expected type`, with a
source diff showing the line rewritten as `break 42;` and `++` under
the literal `42`.)

## What To Ignore For Now

This lesson installs only one idea: `break value;` makes the loop an
expression whose value is `value`. Each of the following is real but
*not* part of this move:

- *`return` inside a loop body* — exits the surrounding function, not
  just the loop. Mentioned in passing by the Book at lines 354-355
  ("`break` only exits the current loop, `return` always exits the
  current function"). It composes lessons 021/025 with this lesson's
  loop-as-expression rule but is its own future move.
- *Loop labels* like `'name: loop { ... break 'name value; }`, which
  let `break value;` target an outer loop instead of the innermost.
  The Book's "Disambiguating with Loop Labels" subsection (lines 359+).
  Future move; would build on this lesson.
- *Multiple `break value;` sites in the same loop*. Rust allows
  several exits provided every `break value;` site carries the same
  type. The probe has only one exit; the constraint is named in prose
  but not exercised. A specialized E0308 diagnostic for two
  type-disagreeing `break` sites is the natural future move here.
- *`break;` (no value) inside a `loop` used as a statement* — lesson
  027's form. Still valid. This lesson does not invalidate it; it adds
  a second form for the second use case.
- *`continue;`* — sibling of `break;` that skips the rest of the
  current pass and starts the next. Still deferred from lesson 027.
- *`while let` / `if let` / pattern-matching loops* — deferred (need
  pattern matching).
- *The unit type `()`*. Named again by rustc in the broken-contrast
  diagnostic; calibrated as "nothing useful / no value" since lesson
  021. Still deferred.
- All previously deferred items: shadowing, `&` references, generics,
  `cargo`, modules and `pub`, the broader format-string DSL.

## Evidence

### Sources

- `output/docs/rust/book/ch03-05-control-flow.md`, the subsection
  "Returning Values from Loops" (approximately lines 321-355). Two
  load-bearing direct quotes:

  Lines 323-328 (the corpus statement that licenses the move):

  > One of the uses of a `loop` is to retry an operation you know
  > might fail, such as checking whether a thread has completed its
  > job. You might also need to pass the result of that operation out
  > of the loop to the rest of your code. To do this, you can add the
  > value you want returned after the `break` expression you use to
  > stop the loop; that value will be returned out of the loop so
  > that you can use it

  Lines 350-352 (the Book's walk of its own example, confirming
  `result` is bound to the value `break` carried out):

  > After the loop, we use a semicolon to end the statement that
  > assigns the value to `result`. Finally, we print the value in
  > `result`, which in this case is `20`.

  Calibration:
  - The Book's example (lines 330-343) is `if counter == 10 { break
    counter * 2; }`, which prints `result = 20`. This lesson's probe
    uses `5` and `* 10`, printing `result = 50`. Structurally
    identical; faster termination; the literal numbers `2` and `10`
    are kept off the page so they do not collide with the `0..N` range
    framing from lesson 022.
  - The Book builds with `cargo run`; this lesson uses `rustc demo.rs`
    per lesson 001. Behavior identical.
  - The Book mentions briefly (lines 354-355): "You can also `return`
    from inside a loop. While `break` only exits the current loop,
    `return` always exits the current function." Mentioned under
    What To Ignore For Now; future move.
  - The Book's next subsection ("Disambiguating with Loop Labels",
    line 359+) is its own future move.
  - Lesson 014 framing: this lesson's probe uses `if`-without-`else`
    for the `break counter * 10;` arm. Same micro-extension grounded
    in lesson 027 — same `if` machine, not a new concept, not added to
    the graph.
  - Lesson 024 framing: this lesson is the *fourth* surface for the
    expression rule, closing the arc 024 (let-block) → 025 (function
    body) → 026 (`if`/`else` arms) → 027 (`loop` + `break;` as
    control flow) → 028 (`loop` + `break value;` as expression).

- The local probes (working + broken-contrast), captured below.

### Probes

Two probes were captured on rustc 1.95.0 (59807616e 2026-04-14) on
Darwin x86_64. The working probe is committed at
`experimental/eduratchet2/runs/rust-moves/observations/028-break-value.rs`.
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
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 5 {
            break counter * 10;
        }
    };
    println!("result = {result}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
result = 50
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (consistent with lesson 001).
- The single output line is `result = 50`. That single value reflects
  *both* halves of the move:
  - The line printed *at all*: the loop terminated. If `break counter
    * 10;` did not exit, `./demo` would never reach the `println!`.
    (Same observation as lesson 027 for the termination half.)
  - The contents `result = 50`: `result` is bound to the value the
    `loop` produced, which is the value `break` carried out
    (`counter * 10` evaluated on pass 5 with `counter = 5`, giving
    `50`). This is the *new* observation for this lesson — the loop
    is bindable by `let`, with the bound value equal to what `break`
    carried out.
- The annotation `let result: i32` is required for this probe: it
  pins the loop's expected type so the broken-contrast diagnostic can
  trace the type-mismatch from the annotation site to the `break;`
  site (see broken-contrast probe).

#### Broken-contrast probe

Same `main`, with `break;` instead of `break counter * 10;`. Not
committed; the transcript below is the artifact.

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
broken.rs
--- cat broken.rs ---
fn main() {
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 5 {
            break;
        }
    };
    println!("result = {result}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:6:13
  |
3 |     let result: i32 = loop {
  |         ------        ---- this loop is expected to be of type `i32`
  |         |
  |         expected because of this assignment
...
6 |             break;
  |             ^^^^^ expected `i32`, found `()`
  |
help: give the `break` a value of the expected type
  |
6 |             break 42;
  |                   ++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Notes:

- The headline `error[E0308]: mismatched types` is the **generic**
  E0308 form (same as lessons 024 and 025). Lesson 026's `if`/`else`
  case had a *specialized* headline; this case does not — same E-code,
  generic phrasing. The `--explain E0308` trailer is also present,
  consistent with lesson 003.
- The `--> broken.rs:6:13` location points at column 13 of line 6 —
  the `break;` site itself.
- The source excerpt traces type flow across **two locations**, joined
  by `...`:
  - *Line 3*: `------` underlines `result` with the sub-annotation
    `expected because of this assignment`. `----` underlines `loop`
    with `this loop is expected to be of type i32`. So the binding
    `let result: i32` is what set the expectation, and rustc explicitly
    says so on the loop itself.
  - *Line 6*: `^^^^^` underlines the bare `break;` with the trailing
    annotation `expected i32, found ()`. This is the actual type
    mismatch site — the loop produced `()` because `break;` carried no
    value.
- The `help:` block goes further than lesson 024's or 025's:
  - The headline-style line `help: give the break a value of the
    expected type` states the *fix in English*.
  - Below it, indented under the `|`, rustc shows a literal source
    diff: `6 |             break 42;` with `++` under the new `42`,
    showing exactly what to add. The literal value `42` is rustc's
    placeholder, not a meaningful answer.
- The named type `()` is the unit type. Calibrated as "nothing
  useful / no value" since lesson 021; explicit move deferred.
- Exit code: 1. No executable was produced.
- The pedagogical point: rustc itself encodes the lesson's whole rule.
  A bare `break;` produces `()`; to make the loop expression-bindable,
  give `break` a value. The diagnostic *names* the rule, *traces* the
  type flow, and *suggests* the fix.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — diagnostics
  have a headline + `-->` location + source excerpt with caret +
  optional `help:` lines. The broken-contrast walk decodes this
  diagnostic, including its two-location source excerpt and its
  literal-source-diff `help:` block, using exactly this skill.
- `005-let-binding` (accepted, load-bearing) — `let name: TYPE =
  value;` binds a name; reused as the slot the loop's value lands in.
- `006-mut-binding` (accepted) — `let mut name = value;` makes a
  binding reassignable. Used as `let mut counter: i32 = 0;` so the
  body can update `counter`.
- `009-arithmetic-on-integers` (accepted) — `*` between two integers
  produces a new integer. Used as `counter * 10` in the working probe.
- `013-comparison-operators` (accepted) — `==` produces a boolean.
  Used as the inner condition `counter == 5` to pick the exit pass.
- `014-if-else` (accepted) — `if condition { ... }` runs a block based
  on a boolean. The probe drops the `else` block, the same
  micro-extension grounded in lesson 027 — same `if` machine, no new
  concept.
- `019-type-annotation-i32` (accepted, load-bearing) — `name: TYPE`
  attaches a type. The annotation `let result: i32` is what pins the
  loop's expected type; rustc's broken-contrast diagnostic traces the
  expected-type from this annotation back to the `break;` site
  (`expected because of this assignment`).
- `023-compound-add-assign` (accepted) — `n += 1;` is shorthand for
  `n = n + 1;` and requires `mut`. Used as `counter += 1;`.
- `024-statement-vs-expression` (accepted, load-bearing) — a block
  `{ ... }` is itself an expression; its value is its tail expression
  (no `;`). **This lesson applies the same rule to a *fourth* surface
  — `loop { ... }` as an expression carrying its value via `break`.**
  Lessons 025 (function body) and 026 (`if`/`else` arms) are the
  second and third surfaces; mentioned in the four-surface framing
  but not load-bearing on their own here.
- `027-loop-and-break` (accepted, load-bearing) — `loop { ... }`
  repeats forever; `break;` inside the body terminates it. **This
  lesson extends `break` from the bare statement form to a
  value-carrying form (`break value;`), keeping the termination
  behavior and adding value-out-of-loop behavior.**
