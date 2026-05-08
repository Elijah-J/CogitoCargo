---
id: 014-if-else
move: "write `if condition { ... } else { ... }` to run one of two blocks based on a boolean condition"
main_concept: "`if condition { ... } else { ... }` evaluates the condition, which must produce a `true` or `false`; if `true`, the first block runs and the `else` block is skipped; if `false`, the `else` block runs and the first block is skipped"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
  - 005-let-binding
  - 012-bool-literals
  - 013-comparison-operators
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "else if chains" moves
  - future "if as expression in let" moves
  - future "loops (loop/while/for)" moves
  - future "match expressions" moves
  - future "if let" moves
sources:
  - output/docs/rust/book/ch03-05-control-flow.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/014-if-else.rs
status: accepted
---

# Run one of two blocks with `if` / `else`

## The Move

Inside `fn main`, write the shape

```rust
if condition {
    // first block
} else {
    // second block
}
```

where `condition` is anything that produces a `true` or `false` —
typically a comparison from lesson 013 (`n > 5`, `a == b`, ...) or a
bound boolean from lesson 012. When the executable runs and reaches
this construct, exactly one of the two `{ ... }` blocks runs: the
first if `condition` is `true`, the second if `condition` is `false`.

## Mental Model Delta

- Before: "Statements in `fn main` run top to bottom (lesson 004), and
  some of them produce booleans (lessons 012, 013). I have no way to
  let the executable *decide* between two different things to do based
  on a boolean."
- After: "Rust has a two-armed `if condition { A } else { B }` shape.
  When the executable reaches it, the condition is evaluated to a
  single `true` or `false`. On `true`, block `A` runs and block `B` is
  skipped. On `false`, block `A` is skipped and block `B` runs.
  Exactly one of the two blocks runs each time. The condition has to
  *be* a boolean; Rust does not pretend a number or a string is one."

## Prerequisites

- Installed concepts:
  - Lesson 001 (`001-rustc-compile-and-run`): `rustc file.rs` produces
    an executable next to the source; run it with `./name`. `rustc` is
    silent on success.
  - Lesson 002 (`002-fn-main-entry-point`): the body inside
    `fn main() { ... }` is what runs when the executable launches.
  - Lesson 004 (`004-statements-in-order`): the body of `fn main` is a
    sequence of statements that run top to bottom in source order. The
    whole `if` / `else` construct is itself one such step in that
    sequence; only its *interior* gets the conditional skip.
  - Lesson 005 (`005-let-binding`, load-bearing): `let name = value;`
    binds a name; `println!("... {name}")` substitutes the bound value.
    The probe binds `n` and uses it inside the condition.
  - Lesson 012 (`012-bool-literals`, load-bearing): `true` and `false`
    are Rust's two boolean literal values. The condition must produce
    one of these.
  - Lesson 013 (`013-comparison-operators`, load-bearing): the six
    comparison operators take two values of the same kind and produce
    a boolean. `n > 5` is the natural shape we use as a condition.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let n = 7;
    if n > 5 {
        println!("big");
    } else {
        println!("small");
    }
}
```

The Book describes this shape directly:

> An `if` expression allows you to branch your code depending on
> conditions. You provide a condition and then state, "If this
> condition is met, run this block of code. If the condition is not
> met, do not run this block of code."

And on the optional second arm:

> Optionally, we can also include an `else` expression, which we chose
> to do here, to give the program an alternative block of code to
> execute should the condition evaluate to `false`.

So in our program: `n > 5` is the condition (it produces a boolean,
lesson 013); `{ println!("big"); }` is the block to run if the
condition is `true`; `{ println!("small"); }` is the alternative block
to run if it is `false`.

Compile and run:

```console
$ rustc demo.rs
$ ./demo
big
```

Why `big`? Because `n` is bound to `7`, the condition `n > 5` evaluates
to `true`, and on `true` the first block runs and the `else` block is
skipped.

Now the contrast. **Before you edit the file, predict**: if you change
`let n = 7;` to `let n = 4;`, leaving everything else alone, what does
`./demo` print after recompiling?

Make that one edit:

```rust
fn main() {
    let n = 4;
    if n > 5 {
        println!("big");
    } else {
        println!("small");
    }
}
```

Recompile and run:

```console
$ rustc demo.rs
$ ./demo
small
```

Walk through it once: `n` is `4`, so `n > 5` evaluates to `false`. On
`false`, the first block is skipped and the `else` block runs. Only
`small` is printed; `big` never appears.

The general rule: the condition is evaluated to a single `true` or
`false`, exactly one of the two blocks runs, and the other is skipped
entirely.

One Rust-specific note worth flagging up front: the condition has to
*be* a boolean. `if n > 5 { ... }` is fine because `n > 5` is a
comparison and produces a boolean. Writing `if n { ... }` with a bare
integer is a *compile error*. Unlike some languages (the Book names
Ruby and JavaScript), Rust does not silently treat numbers, strings,
or other values as "truthy" or "falsy". If you want to test "is `n`
non-zero?", you write that out: `if n != 0 { ... }`. We do not
exercise the failure here; just know it exists.

## What Changed

- You can write a two-armed `if condition { ... } else { ... }` inside
  `fn main` and have the executable run exactly one of the two blocks.
- You know what the condition has to be: any expression that produces
  a `true` or `false` — typically a comparison (lesson 013) or a bound
  boolean (lesson 012). Rust does not auto-convert non-booleans.
- You know what gets skipped: the block whose side of the `if` does
  not match the condition's value. The skipped block does not run at
  all; its `println!` does not fire.
- You can change a single binding (`let n = 7;` to `let n = 4;`) and
  predict the output flip from `big` to `small` without re-reading the
  whole program.

## Check Yourself

You write `pick.rs` containing:

```rust
fn main() {
    let score = 80;
    if score >= 60 {
        println!("pass");
    } else {
        println!("fail");
    }
}
```

You run `rustc pick.rs` and then `./pick`.

(a) What does it print?

(b) You change `let score = 80;` to `let score = 50;`, leaving every
other line alone. After recompiling, what does `./pick` print?

(c) In the original `score = 80` version, does the `println!("fail");`
line ever run?

(Answers: (a) `pass`, because `80 >= 60` is `true`, so the first block
runs. (b) `fail`, because `50 >= 60` is `false`, so the `else` block
runs. (c) No. When the condition is `true`, the `else` block is
skipped entirely; the program prints only `pass`.)

## What To Ignore For Now

This lesson installs only one idea: a two-armed `if` / `else` runs
exactly one of two blocks based on a boolean condition. Each of the
following is real and will be taught later, but is *not* part of this
move:

- `else if` *chains*, e.g. `if a { } else if b { } else { }`. The
  Book's very next subsection teaches them; deferred to a future cycle.
- `if` *as an expression in `let` position*, e.g.
  `let x = if cond { 1 } else { 2 };`. Same chapter; deferred.
- *Truthy / falsy non-bool conditions*. `if 5 { ... }` is a compile
  error in Rust — the condition must be a `bool`. Briefly mentioned
  above; the failure is not exercised here.
- *Loops* — `loop`, `while`, `for`. Same chapter of the Book; out of
  scope for this cycle.
- `match` *expressions*. A different control-flow shape; deferred.
- `if let` *patterns*. Different shape again; deferred.
- *Logical operators* `&&`, `||`, `!` for combining or negating
  booleans inside the condition. Still deferred from lesson 013.
- The word *expression* (the `if` form is technically an expression in
  Rust, hence the section title in the Book). The full
  statement-vs-expression distinction is still deferred from lesson
  004; for now we are using `if` / `else` as a self-contained step
  inside `fn main`.
- Everything previously deferred: `mut`, shadowing, type annotations
  on `let`, comments, defining your own functions, function parameters
  and return values, the broader format-string DSL, and `cargo`.

## Evidence

### Sources

- `output/docs/rust/book/ch03-05-control-flow.md`, the
  "`if` Expressions" subsection (lines 1-75). Two load-bearing direct
  quotes:
  - Lines 12-14: "An `if` expression allows you to branch your code
    depending on conditions. You provide a condition and then state,
    'If this condition is met, run this block of code. If the
    condition is not met, do not run this block of code.'" This is
    the corpus statement that licenses the lesson's main concept (the
    two-armed branch).
  - Lines 41-45: "Optionally, we can also include an `else`
    expression, which we chose to do here, to give the program an
    alternative block of code to execute should the condition evaluate
    to `false`. If you don't provide an `else` expression and the
    condition is `false`, the program will just skip the `if` block
    and move on to the next bit of code." This grounds the `else` arm
    and the "skip the block" rule.

  The same subsection (lines 82-117) also notes that "the condition in
  this code *must* be a `bool`. If the condition isn't a `bool`, we'll
  get an error" and contrasts with Ruby/JavaScript-style coercion.
  This is the source for the brief Rust-specific note in Try It; the
  failure is not exercised in the probe.

  Calibration:
  - The Book's example builds with `cargo run`; this lesson uses
    `rustc demo.rs` directly, consistent with lesson 001's two-step
    workflow. The behavior under test (which block runs) is unchanged
    by the build-driver choice.
  - The same chapter continues into "Handling Multiple Conditions
    with `else if`" at line 135, then `if` *as an expression in `let`*,
    then loops. All three are explicitly deferred under What To Ignore
    For Now.
  - The Book also calls `if` an "*expression*" and the inner blocks
    "*arms*". This lesson uses the plainer "the first / second block"
    language and defers the formal expression vocabulary; both
    framings agree on the behavior taught here.

- The local probe (single working transcript), captured below.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/014-if-else.rs`.
The committed file is the *original* `n = 7` version. The `n = 4`
variant is documented as a second run inside this Evidence section,
not as a separate `.rs` file.

Probe transcript, both runs in the same temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64

=== RUN 1: n = 7 ===
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let n = 7;
    if n > 5 {
        println!("big");
    } else {
        println!("small");
    }
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
big
exit=0

=== RUN 2: change n = 7 to n = 4 ===
--- cat demo.rs ---
fn main() {
    let n = 4;
    if n > 5 {
        println!("big");
    } else {
        println!("small");
    }
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
small
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent in both runs (consistent with
  lesson 001).
- Run 1 (`n = 7`): the condition `n > 5` is `true`, and `./demo`
  prints exactly `big`. The `else` block does not fire — `small` does
  not appear.
- Run 2 (`n = 4`): only the binding changed; the `if` / `else`
  structure is identical. The condition `n > 5` is now `false`, and
  `./demo` prints exactly `small`. The first block does not fire —
  `big` does not appear.
- Comparing the two runs side by side is the load-bearing observation
  for the lesson's main concept: the *same* two blocks of code are
  written in the *same* order, and changing only the condition's value
  changes which block runs and which is skipped.
- Each run prints exactly one line, never both. There is no execution
  path through this construct that runs both blocks, and no path
  (with this `else` arm present) that runs neither.
- Only the working `n = 7` source is committed under `observations/`;
  the `n = 4` variant exists only inside this transcript. The temp
  dir was removed; no binaries are committed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `004-statements-in-order` (accepted) — the body of `fn main` runs
  top to bottom in source order; the `if` / `else` is itself one step
  in that sequence.
- `005-let-binding` (accepted, load-bearing) — `let name = value;`
  binds a name. The probe binds `n` and uses it inside the condition.
- `012-bool-literals` (accepted, load-bearing) — `true` and `false`
  are Rust's two boolean values. The condition must produce one of
  these; the lesson reuses lesson 012's "must be a boolean" framing.
- `013-comparison-operators` (accepted, load-bearing) — comparisons
  like `n > 5` produce a boolean. This lesson uses one comparison as
  the natural condition.
