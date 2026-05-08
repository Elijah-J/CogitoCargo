---
id: 016-else-if-chain
move: "extend an `if`/`else` with one or more `else if condition { ... }` arms between the `if` and the final `else`; Rust checks the conditions in source order and runs the first block whose condition is `true`, skipping the rest"
main_concept: "`else if` arms add extra branches to lesson 014's two-armed shape; Rust checks conditions in source order from top to bottom and runs the first block whose condition evaluates to `true`, skipping every later arm; the optional final `else` runs only when every earlier condition was `false`"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
  - 005-let-binding
  - 012-bool-literals
  - 013-comparison-operators
  - 014-if-else
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "match expressions" moves
  - future "if as expression in let" moves
  - future "no-else-arm chains" moves
sources:
  - output/docs/rust/book/ch03-05-control-flow.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/016-else-if-chain.rs
status: accepted
---

# Add `else if` arms; first matching condition wins

## The Move

Take lesson 014's two-armed `if condition { ... } else { ... }` shape
and slot one or more `else if condition { ... }` arms between the
opening `if` and the final `else`. When the executable reaches this
construct, Rust checks each arm's condition in source order, top to
bottom. The first arm whose condition is `true` runs. Every later arm
is skipped — its condition is not even checked. The final `else` (if
present) runs only when every earlier condition was `false`.

## Mental Model Delta

- Before: "I have a two-armed `if`/`else` (lesson 014). It runs one
  of exactly two blocks. To choose among three or four cases, I do
  not yet know how."
- After: "I can insert extra `else if condition { ... }` arms between
  the opening `if` and the final `else`. Rust checks the conditions
  in source order; the first `true` wins, its block runs, and the
  rest of the chain is skipped — even later conditions that *would
  also* be `true` are not evaluated. The final `else` runs only when
  every earlier condition was `false`."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 004 — `rustc file.rs` makes an executable;
    `fn main` body runs top-to-bottom. The chain extends source
    order to "conditions top to bottom."
  - Lesson 005 (load-bearing) — `let name = value;` binds a name;
    the probe binds `n` and reuses it inside three conditions.
  - Lesson 012 — `true` / `false` are the two boolean values.
  - Lesson 013 (load-bearing) — comparisons like `n > 10`, `n > 0`,
    `n == 0` produce booleans.
  - Lesson 014 (load-bearing) — the two-armed
    `if condition { ... } else { ... }` shape; condition must be
    boolean, exactly one block runs. This lesson extends that shape.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let n = 5;
    if n > 10 {
        println!("very big");
    } else if n > 0 {
        println!("positive");
    } else if n == 0 {
        println!("zero");
    } else {
        println!("negative");
    }
}
```

The Book describes this shape directly: "You can use multiple
conditions by combining `if` and `else` in an `else if` expression."

Compile and run:

```console
$ rustc demo.rs
$ ./demo
positive
```

Walk through *why* `positive`. `n` is `5`. Rust checks arms in source
order: `n > 10` is `false` (skip); `n > 0` is `true` (run, prints
`positive`). Done. `n == 0` and the final `else` are **not even
checked**.

The Book states the rule:

> When this program executes, it checks each `if` expression in turn
> and executes the first body for which the condition evaluates to
> `true`. ... Rust only executes the block for the first `true`
> condition, and once it finds one, it doesn't even check the rest.

That last clause is load-bearing. Once arm 2 wins, the rest is
skipped wholesale.

Predict three other values *without* recompiling:

- **`n = 15`**: arm 1 wins, prints `very big`. Arm 2 (`15 > 0`)
  would also be `true`, but Rust stops at the first match.
- **`n = 0`**: arms 1 and 2 are `false`; arm 3 (`0 == 0`) wins,
  prints `zero`.
- **`n = -3`**: every condition is `false`, so the final `else`
  runs and prints `negative`.

The chain has four paths; on any single run exactly one arm runs.

## What Changed

- You can write a multi-arm `if` / `else if` / ... / `else` and have
  the executable run exactly one of the arms.
- You know the order rule: top to bottom in source order. Reordering
  arms can change which one wins, even with no condition change.
- You know the first-match rule: once a condition is `true`, the rest
  of the chain is skipped — later conditions are not evaluated, even
  ones that would also be `true`.
- You know the final `else` is the catch-all and runs only when every
  earlier condition was `false`.

## Check Yourself

You write `grade.rs` containing:

```rust
fn main() {
    let score = 72;
    if score >= 90 {
        println!("A");
    } else if score >= 80 {
        println!("B");
    } else if score >= 70 {
        println!("C");
    } else {
        println!("F");
    }
}
```

You run `rustc grade.rs` and then `./grade`.

(a) What does it print, and how many conditions does Rust evaluate?

(b) For `score = 95`, conditions 1, 2, and 3 would all be `true`.
Why does the program never print both `A` and `B`?

(c) You change `let score = 72;` to `let score = 50;`. What does
`./grade` print after recompiling?

(Answers: (a) `C`; three conditions, arm 3 wins. (b) Rust stops at
the first `true`; for `score = 95`, arm 1 wins and the rest is
skipped. (c) `F`; all three conditions are `false`, so the final
`else` runs.)

## What To Ignore For Now

This lesson installs one idea: first matching condition wins, rest
skipped, final `else` is the catch-all. Deferred:

- `match` *expressions*. The Book warns, right after the `else if`
  example: "Using too many `else if` expressions can clutter your
  code, so if you have more than one, you might want to refactor
  your code. Chapter 6 describes a powerful Rust branching construct
  called `match` for these cases." Its own cycle.
- `if` *as an expression in `let` position*. Same deferral as
  lesson 014.
- *Pattern matching* and `if let`. Distinct shape; deferred.
- *No final `else`*. Allowed: if every condition is `false` and
  there is no `else`, the executable falls past the construct
  without printing anything. The probe here always keeps a final
  `else`; the no-`else` form is flagged and deferred.
- The modulus `%` operator (the Book's `else if` example uses it).
  Deferred from lesson 009; this lesson uses simpler comparisons.
- *Logical operators* `&&`, `||`, `!`. Deferred from lesson 015.
- *Loops* (`loop`, `while`, `for`), the *expression* vocabulary,
  and everything previously deferred (`mut`, shadowing, type
  annotations, comments, user-defined functions, the broader
  format-string DSL, `cargo`).

## Evidence

### Sources

- `output/docs/rust/book/ch03-05-control-flow.md`, the
  "Handling Multiple Conditions with `else if`" subsection (lines
  135-178). Two load-bearing direct quotes:
  - Lines 137-138: "You can use multiple conditions by combining `if`
    and `else` in an `else if` expression." This is the corpus
    statement that licenses extending lesson 014's two-armed shape
    with extra arms.
  - Lines 169-174: "When this program executes, it checks each `if`
    expression in turn and executes the first body for which the
    condition evaluates to `true`. ... Rust only executes the block
    for the first `true` condition, and once it finds one, it doesn't
    even check the rest." This is the corpus statement of the
    source-order, first-match-wins, skip-the-rest rule that is the
    main concept of this lesson.

  Lines 176-178 add: "Using too many `else if` expressions can clutter
  your code, so if you have more than one, you might want to refactor
  your code. Chapter 6 describes a powerful Rust branching construct
  called `match` for these cases." This is the corpus's own forward
  pointer, used as the basis for the `match` deferral in What To
  Ignore For Now.

  Calibration:
  - The Book's example uses the modulus operator `%`
    (`number % 4 == 0`, etc.), which is deferred from lesson 009.
    This lesson uses simpler comparison operators (`>`, `==`) from
    lesson 013 to avoid introducing `%`. The behavior under test —
    source-order checking, first-match-wins, rest-skipped — is
    determined by the chain shape itself, not by what each condition
    happens to compute, so the substitution is safe.
  - The Book's example builds with `cargo run`; this lesson uses
    `rustc demo.rs` directly, consistent with lesson 001's two-step
    workflow.
  - The Book uses the word *expression* for `if`; lesson 014 already
    deferred that vocabulary, and this lesson follows suit.

- The local probe (single working transcript), captured below.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/016-else-if-chain.rs`.
The committed file is the exact working program shown in Try It.

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
    let n = 5;
    if n > 10 {
        println!("very big");
    } else if n > 0 {
        println!("positive");
    } else if n == 0 {
        println!("zero");
    } else {
        println!("negative");
    }
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
positive
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent (consistent with lesson 001).
- After compile, `ls` shows two files: `demo.rs` and the new
  executable `demo`.
- `./demo` prints exactly one line, `positive`. None of the other
  three strings (`very big`, `zero`, `negative`) appear in the output.
  This is the load-bearing observation that exactly one arm of a
  four-armed `if`/`else if`/`else` chain fires per pass.
- Walkthrough: `n = 5`. Arm 1's condition `n > 10` is `false`, so the
  first block does not run. Arm 2's condition `n > 0` is `true`, so
  the second block runs and prints `positive`. Per the Book quote
  above, "once it finds one, it doesn't even check the rest" — arms
  3 and 4 are not evaluated. The fact that `n == 0` is `false` for
  `n = 5` is true but unobservable from this run, because Rust never
  asked.
- The predict-and-verify table for `n = 15`, `n = 0`, `n = -3` in
  the Try It section is *predictions*, not separately captured
  probes. The same chain shape and the same first-match-wins rule
  determine those outcomes; only the captured `n = 5` run is
  load-bearing for grounding.
- Only the working source is committed under `observations/`; no
  binaries are committed. The temp dir was removed at the end of the
  run.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `004-statements-in-order` (accepted) — statements in `fn main` run
  top to bottom in source order. This lesson extends source-order
  logic to multi-arm conditional checking inside the chain.
- `005-let-binding` (accepted, load-bearing) — `let name = value;`
  binds a name; `println!("... {name}")` substitutes the bound value.
  The probe binds `n` and reuses it inside three conditions.
- `012-bool-literals` (accepted) — `true` and `false` are Rust's two
  boolean values; each condition in the chain evaluates to one of
  these.
- `013-comparison-operators` (accepted, load-bearing) — comparisons
  like `n > 10`, `n > 0`, `n == 0` produce booleans. The chain uses
  three of these directly as conditions.
- `014-if-else` (accepted, load-bearing) — the two-armed
  `if condition { ... } else { ... }` shape; condition must be a
  boolean; exactly one of the two blocks runs. This lesson extends
  that shape by inserting `else if condition { ... }` arms between
  the opening `if` and the final `else`, generalizing "exactly one
  of two" to "exactly one of N, the first whose condition is `true`."
