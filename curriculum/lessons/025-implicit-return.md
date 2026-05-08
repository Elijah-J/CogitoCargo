---
id: 025-implicit-return
move: "drop `return` and the trailing `;` from a returning function body — write `fn add_one(n: i32) -> i32 { n + 1 }` instead of `fn add_one(n: i32) -> i32 { return n + 1; }`"
main_concept: "a function body is itself a block; lesson 024's rule applies — the block's value is the value of its tail expression (no `;`), and the `-> RTYPE` declaration makes that value the function's return value, so `return value;` and a bare `value` on the last line are equivalent in this position; the bare form is idiomatic Rust; adding a stray `;` to the tail line drops the block to `()` and rustc reports E0308 with `implicitly returns ()` as the body annotation"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 005-let-binding
  - 008-define-and-call-function
  - 009-arithmetic-on-integers
  - 019-type-annotation-i32
  - 020-function-with-parameter
  - 021-function-return-value
  - 024-statement-vs-expression
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "if/else as expression in function body" moves
  - future "match as function body expression" moves
  - future "unit type ()" moves
  - future "early return inside if/else arms" moves
  - future "mixing implicit return with explicit return" moves
sources:
  - output/docs/rust/book/ch03-03-how-functions-work.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/025-implicit-return.rs
status: accepted
---

# Drop `return` and the trailing `;` — let the body's tail expression *be* the return value

## The Move

Take lesson 021's `add_one`:

```rust
fn add_one(n: i32) -> i32 {
    return n + 1;
}
```

Delete `return` and the trailing `;`, leaving a bare `n + 1`:

```rust
fn add_one(n: i32) -> i32 {
    n + 1
}
```

Compile and run with the same call site as lesson 021. Same observable
behavior: the program prints `result = 6`. The reason it works: a
function body is itself a block, so lesson 024's rule applies — the
block's value is the value of its tail expression (the last line, no
`;`), and the `-> i32` declaration says that value *is* the function's
return value.

## Mental Model Delta

- Before: "A function returns a value with `return value;` inside its
  body (lesson 021). Those `{ ... }` are just where the body lives."
- After: "A function body *is* a block. Lesson 024's rule applies to
  it unchanged: the block's value is its tail expression (no `;`), and
  `-> RTYPE` says that value is the function's return value. So
  `return value;` and a bare `value` on the last line are two ways to
  say the same thing; the bare form is idiomatic Rust. A stray `;` on
  the tail drops the body to `()` and rustc annotates the function
  with `implicitly returns ()` (its body has no tail or `return`
  expression)."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs`, `./name`, silent on success;
    `fn main` runs when the executable launches.
  - Lesson 003 (load-bearing): rustc diagnostics = headline + `-->` +
    source excerpt with caret + optional `help:` / `note:` lines. The
    broken-contrast walkthrough parses E0308 using exactly this skill.
  - Lessons 005, 008, 009, 019, 020: `let name: TYPE = value;`,
    `fn name() { ... }` plus `name();`, `+` on integers, `name: TYPE`
    annotation form, and `fn name(p: i32) { ... }` with `name(value);`
    at the call.
  - Lesson 021 (load-bearing): `fn name(p: i32) -> RTYPE { return value; }`
    plus the call expression `name(args)` carrying the returned value.
    This lesson keeps that shape and replaces the body's
    `return value;` with a tail expression.
  - Lesson 024 (load-bearing): a block `{ ... }` is itself an
    expression; its value is its tail expression (no `;`); a `;` on
    the tail drops the block to `()`. **This is the rule this lesson
    applies to function bodies.**
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn add_one(n: i32) -> i32 {
    n + 1
}

fn main() {
    let result: i32 = add_one(5);
    println!("result = {result}");
}
```

This is lesson 021's `demo.rs` with one change: the body of `add_one`
went from `return n + 1;` to a bare `n + 1` (no `;`). The signature
and the call site are byte-for-byte identical.

Compile and run:

```console
$ rustc demo.rs
$ ./demo
result = 6
```

Walk it. The body of `add_one` is the block `{ n + 1 }`. Per lesson
024, the block's value is its tail expression's value — `n + 1` (no
`;`) evaluates to `6` with `n = 5`. The `-> i32` declaration makes
that block-value the function's return value. Per lesson 021, the
call expression `add_one(5)` carries that `6` to the right of `let`
in `main`, and `println!` prints `result = 6`. *Same printed line as
lesson 021* — the equivalence is the load-bearing observation.

The Book states the equivalence directly:

> Functions can return values to the code that calls them. We don't
> name return values, but we must declare their type after an arrow
> (`->`). In Rust, the return value of the function is synonymous with
> the value of the final expression in the block of the body of a
> function. You can return early from a function by using the `return`
> keyword and specifying a value, but most functions return the last
> expression implicitly.

So lesson 021's `return n + 1;` is one valid form; this lesson's bare
`n + 1` is the other, and the Book says "most functions" use it.

Now do the contrast. In the same directory, save a second file
`broken.rs` that is identical to `demo.rs` except for one extra `;`
after `n + 1`:

```rust
fn add_one(n: i32) -> i32 {
    n + 1;
}

fn main() {
    let result: i32 = add_one(5);
    println!("result = {result}");
}
```

Compile it. rustc rejects with `error[E0308]: mismatched types` — the
same E-code lesson 024's broken contrast emitted. The full transcript
lives in `## Evidence`; the load-bearing bits, read using lesson
003's order:

- *Source excerpt*: the caret `^^^` underlines the declared return
  type `i32` ("expected `i32`, found `()`"); dashes `-------`
  underline the function name `add_one`; under those dashes, a
  sub-annotation reads **implicitly returns `()` as its body has no
  tail or `return` expression** (rustc itself prints backticks around
  `()` and `return`; see the transcript). That sentence names this
  lesson's whole mechanism — rustc itself says it.
- *Help line*: `help: remove this semicolon to return this value`,
  with `-` under the stray `;` on the tail line. Same help text as
  lesson 024's broken contrast.

The `-> i32` says the body must produce an `i32`. The `;` made the
body's tail a statement (lesson 024), so the block evaluates to `()`.
That is the mismatch. Removing the `;` makes `n + 1` the tail
expression again — back to the working `demo.rs`.

## What Changed

- You can write a returning function without `return` and without a
  trailing `;` on the body's last line:
  `fn add_one(n: i32) -> i32 { n + 1 }`.
- The rule: a function body is a block; lesson 024's tail-expression
  rule applies, and `-> RTYPE` makes the block's value the return
  value. So `return value;` and a bare `value` are equivalent here.
- The bare form is idiomatic Rust; the Book says "most functions
  return the last expression implicitly."
- A stray `;` on the tail drops the body to `()`; rustc reports
  E0308, points the caret at the declared return type, and annotates
  the function with "implicitly returns `()` as its body has no tail
  or `return` expression", plus the same `help: remove this semicolon
  to return this value` line lesson 024 emitted.

## Check Yourself

You write `tiny.rs` containing:

```rust
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let answer: i32 = double(7);
    println!("answer = {answer}");
}
```

You run `rustc tiny.rs && ./tiny`.

- Does rustc accept the program, and what does it print?
- Why does the bare `n * 2` (no `;`) work as a return — which lesson
  states the rule, and what does `-> i32` add to it?
- If you added a `;` after `n * 2`, what headline would rustc emit,
  what would the sub-annotation under `double` literally say, and what
  would the `help:` line suggest?

(Answers: yes; prints `answer = 14`. The body of `double` is a block;
lesson 024 says the block's value is its tail expression's value
(`14` with `n = 7`), and `-> i32` makes that the return value. With a
trailing `;`: headline `error[E0308]: mismatched types`, sub-annotation
"implicitly returns `()` as its body has no tail or `return`
expression" under `double`, `help: remove this semicolon to return
this value` pointing at the stray `;`.)

## What To Ignore For Now

Real and deferred:

- *`return` for early exits inside `if`/`else` arms.* Early returns
  are still useful when a function must exit before its tail line.
  Future move.
- *Functions returning the unit type `()` explicitly* — `fn foo() -> () { ... }`,
  and the equivalence with no `->` (lessons 008 and 020). Carried
  forward from lesson 021's calibration. Future move.
- *Tuple, struct, enum, reference, and generic returns.* Each is its
  own later move.
- *`Result` and `Option` and the `?` operator* used to propagate
  them. Deferred.
- *`if`/`else`, `match`, and `loop` as expressions in function
  bodies.* The same lesson-024 rule applied to a different surface.
  Each is its own later move.
- *Mixing implicit return with `return`* in the same function body.
  Allowed by Rust; pedagogically a separate move. Deferred.
- All previously deferred items: `mut` scope and shadowing details
  beyond the installed lessons, the broader format-string DSL,
  references, generics, `cargo`, modules and `pub`.

## Evidence

### Sources

- `output/docs/rust/book/ch03-03-how-functions-work.md`, the
  "Functions with Return Values" section. Two load-bearing direct
  quotes:

  Lines 254-258 (the equivalence statement that makes implicit return
  canonical):

  > Functions can return values to the code that calls them. We don't
  > name return values, but we must declare their type after an arrow
  > (`->`). In Rust, the return value of the function is synonymous
  > with the value of the final expression in the block of the body of
  > a function. You can return early from a function by using the
  > `return` keyword and specifying a value, but most functions return
  > the last expression implicitly.

  Lines 302-304 (the Book naming the bare-tail-expression form
  directly, on the `fn five() -> i32 { 5 }` example):

  > Second, the `five` function has no parameters and defines the type
  > of the return value, but the body of the function is a lonely `5`
  > with no semicolon because it's an expression whose value we want
  > to return.

  The Book's `plus_one` example on lines 311-320 is the same shape as
  this lesson's `add_one` (parameter `x: i32`, body `x + 1`, return
  type `i32`); the Book then shows the broken-contrast (`x + 1;`) on
  lines 328-338 and prints rustc's diagnostic on lines 343-356.

  Calibration:
  - The Book builds with `cargo run`; this lesson uses `rustc demo.rs`
    per lesson 001. Behavior is identical; only the build command
    differs.
  - The Book uses the function name `plus_one` and parameter `x`. This
    lesson uses `add_one` and `n` — identical to lesson 021's working
    probe so the only delta the learner perceives between the two
    lessons is the body shape.
  - The Book's *printed* broken-contrast diagnostic (lines 343-356)
    differs cosmetically from current rustc 1.95.0's emission (rustc
    formatting evolves between releases). The Book's text supplies the
    quotable rule and the `plus_one` example; the *captured*
    transcript below is the load-bearing artifact for what current
    rustc actually prints.

### Probes

Two probes were captured. The working probe is committed at
`experimental/eduratchet2/runs/rust-moves/observations/025-implicit-return.rs`.
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
fn add_one(n: i32) -> i32 {
    n + 1
}

fn main() {
    let result: i32 = add_one(5);
    println!("result = {result}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
result = 6
exit=0
```

Notes:

- `rustc` exits 0 and is silent (consistent with lesson 001).
- The single output line is `result = 6` — *the same line* as lesson
  021's working probe, on the same call site `add_one(5)` with the
  same `let` slot. The body is the only thing that changed. That
  identical observable behavior is the load-bearing observation for
  this lesson: `return n + 1;` and a bare `n + 1` produce the same
  return value.
- The body of `add_one` is `{ n + 1 }`. Per lesson 024 the block's
  value is the value of `n + 1` (no `;`); that is `6` with `n = 5`.
  Per the `-> i32` declaration, that block-value is the function's
  return value, and lesson 021 carries it through `add_one(5)` to the
  right of `let`.

#### Broken-contrast probe

Same source as the working probe with one extra `;` after `n + 1`. Not
committed; the transcript below is the artifact.

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
broken.rs
--- cat broken.rs ---
fn add_one(n: i32) -> i32 {
    n + 1;
}

fn main() {
    let result: i32 = add_one(5);
    println!("result = {result}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:1:23
  |
1 | fn add_one(n: i32) -> i32 {
  |    -------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     n + 1;
  |          - help: remove this semicolon to return this value

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Notes:

- The headline `error[E0308]: mismatched types` carries the code
  `E0308` — same code as lesson 024's broken-contrast probe — and the
  optional `--explain E0308` trailer is also present.
- The `--> broken.rs:1:23` location points at column 23 of line 1 —
  that is the start of the declared return type `i32` in the
  signature.
- Inside the source excerpt, the caret `^^^` underlines the declared
  return type `i32`; the dashes `-------` underline the function name
  `add_one`. Below the dashes, the sub-annotation reads
  **implicitly returns `()` as its body has no tail or `return` expression**
  (rustc itself prints backticks around `()` and `return`; see the
  transcript verbatim above). That sentence is the corpus-level
  statement of this lesson's mechanism — rustc itself names it. It is
  the load-bearing diagnostic evidence for the lesson.
- The `help: remove this semicolon to return this value` line, with
  the dash `-` directly under the stray `;` on line 2, is the same
  help text lesson 024's broken contrast emitted. Same rule (`;` makes
  the block tailless, block becomes `()`), now applied inside a
  function body where the surrounding contract is `-> i32`.
- The type `()` named in `found ()` is the unit type. Calibrated as
  "nothing useful" return in lesson 021 and as "block has no tail" in
  lesson 024; deferred for a dedicated move.
- Exit code: 1. No executable was produced.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success. Used by both probes.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — diagnostics
  have a headline, a `-->` location, a source excerpt with caret, and
  optional `help:` / `note:` lines. The broken-contrast walkthrough
  parses E0308 using exactly this structure, including the
  sub-annotation under `add_one` and the `help:` line under the stray
  `;`.
- `005-let-binding` (accepted) — `let name: TYPE = value;` binds a
  name to a value; reused as the slot the call's returned value lands
  in.
- `008-define-and-call-function` (accepted) — define a second function
  with `fn name() { ... }` and call it from `main`.
- `009-arithmetic-on-integers` (accepted) — `+` between two integers
  produces a new integer; used inside `add_one` as `n + 1`.
- `019-type-annotation-i32` (accepted) — `name: TYPE` attaches a type;
  `-> i32` declares the function's return type.
- `020-function-with-parameter` (accepted) — `fn name(p: i32) { ... }`
  plus call shape `name(value);`. The signature surface this lesson
  builds on.
- `021-function-return-value` (accepted, load-bearing) —
  `fn name(p: i32) -> RTYPE { return value; }` plus the call expression
  `name(args)` carrying the returned value. This lesson keeps that
  exact shape and replaces the body's `return value;` with a tail
  expression. Both lessons end with `result = 6` printed on the same
  call site `add_one(5)`; that intentional identity is the
  load-bearing observation that the two body shapes are equivalent.
- `024-statement-vs-expression` (accepted, load-bearing) — a block
  `{ ... }` is itself an expression; its value is the value of its
  tail expression (no `;`); a `;` on the tail turns it into a
  statement and the block evaluates to `()`. **This lesson is the
  application of that rule to a function body.** Lesson 024 also
  emitted E0308 with the `help: remove this semicolon to return this
  value` line; this lesson emits the same E0308 with the additional
  body-context sub-annotation "implicitly returns `()` as its body has
  no tail or `return` expression".
