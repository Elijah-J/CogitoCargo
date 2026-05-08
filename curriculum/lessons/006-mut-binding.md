---
id: 006-mut-binding
move: "write `let mut name = value;` to make a binding reassignable, then assign a new value to it in a later statement"
main_concept: "bindings are immutable by default; adding `mut` after `let` makes them reassignable, so a later `name = new_value;` works; without `mut`, that reassignment fails with E0384 `cannot assign twice to immutable variable`"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 004-statements-in-order
  - 005-let-binding
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "shadowing" moves
  - future "type annotations on let" moves
  - future "constants" moves
  - future "&mut references" moves
  - future "format-string DSL" moves
sources:
  - output/docs/rust/book/ch03-01-variables-and-mutability.md
  - output/docs/rust/error_codes/E0384.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/006-mut-binding.rs
status: accepted
---

# Make a binding reassignable with `let mut`

## The Move

Inside `fn main`, write `let mut x = 5;` instead of `let x = 5;`.
The keyword `mut` sits between `let` and the name. From a later
statement, write `x = 6;` to assign a new value to the same binding.
Two `println!` lines around the reassignment show the value before
and after. Drop `mut` and the same program no longer compiles:
`rustc` emits `error[E0384]: cannot assign twice to immutable
variable \`x\``.

## Mental Model Delta

- Before: "A `let name = value;` binds a name to a value. I do not
  know whether — or how — the value behind that name can change."
- After: "Bindings made with plain `let` are *immutable*: a later
  `x = 6;` is a compile error. To make a binding *reassignable*, I
  write `let mut x = 5;`; the `mut` sits between `let` and the name.
  After that, `x = new_value;` is allowed and updates what `x`
  refers to. `mut` toggles a binding from the immutable default to
  mutable; that is the whole rule."

## Prerequisites

- Installed concepts:
  - Lessons 001 and 002: `rustc file.rs` produces an executable
    next to the source; the body of `fn main` is what runs.
  - Lesson 003 (load-bearing): the four-part diagnostic map
    (headline, `-->` location, source excerpt with caret,
    help/note). The contrast probe produces an `error[E0384]`
    block you read with that map.
  - Lesson 004 (load-bearing): `;`-terminated statements in
    `fn main` run top to bottom. That makes "first `println!`
    shows `5`, then `x = 6;` runs, then second `println!` shows
    `6`" concrete.
  - Lesson 005 (load-bearing): `let name = value;` binds a name,
    and `println!("... {name} ...")` substitutes the bound value
    at print time. The new keyword `mut` slots between `let` and
    `name`.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");
}
```

Four `;`-terminated statements live in `fn main`. The first,
`let mut x = 5;`, is lesson 005's binding statement plus the new
keyword `mut`. The third, `x = 6;`, is a *reassignment*: no `let`,
just the bound name, an `=`, and a new value. (The corpus also
calls bindings *variables*; this lesson keeps *binding* for
consistency with lesson 005.)

Compile and run:

```console
$ rustc demo.rs
$ ./demo
x = 5
x = 6
```

The first `println!` runs before the reassignment, the second
after; that is lesson 004's source-order rule.

Now the contrast. Before editing, *predict*: if you remove `mut`
from line 2, leaving `let x = 5;`, what will `rustc` do?

Edit `demo.rs` so line 2 is `let x = 5;` (delete `mut`, change
nothing else):

```rust
fn main() {
    let x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");
}
```

Compile again:

```console
$ rustc demo.rs
error[E0384]: cannot assign twice to immutable variable `x`
 --> demo.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("x = {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0384`.
```

Read it using the lesson 003 map. **Headline** carries code
`E0384`. **Location** is `demo.rs:4:5`, where `x = 6;` starts.
**Source excerpt** is unusually rich: two annotations in one
bordered block — a `-` under the `x` on line 2 (`first assignment
to \`x\``) and `^^^^^` under `x = 6` on line 4 (`cannot assign twice
to immutable variable`). **Help** literally suggests the fix:
`consider making this binding mutable`, with a second excerpt and
`+++` marking where to insert `mut`. The `--explain E0384` trailer
follows.

The diagnostic is doing more than usual; treat it as a helpful
specimen, not the typical shape. The four-part map still covers it.

Restore `mut`, recompile, rerun: `x = 5` then `x = 6` again. One
keyword separates the program that compiles from the one that does
not.

## What Changed

- You can make a `let` binding reassignable by writing `mut`
  between `let` and the name.
- The rule is one keyword wide: plain `let` makes an *immutable*
  binding; `let mut` makes a *mutable* one. *Reassign* is the verb
  for writing `name = new_value;` after a binding exists.
- You can recognize `error[E0384]: cannot assign twice to immutable
  variable \`name\``. Its `help:` line tells you the fix out loud.
- Immutability is the *default*. The Book discusses why
  (correctness, easier reasoning); rationale deferred.

## Check Yourself

You write `count.rs` containing:

```rust
fn main() {
    let mut n = 1;
    println!("n = {n}");
    n = 2;
    println!("n = {n}");
}
```

(a) You run `rustc count.rs` and then `./count`. What does it print,
in order, and what is the exit code?

(b) You edit line 2 to remove `mut` (leaving `let n = 1;`) and run
`rustc count.rs` again. Is a new `count` produced? Which `E####`
code appears, and which line will the `-->` location point at?

(Answers: (a) `n = 1` then `n = 2`, exit 0. (b) No new executable;
`rustc` exits 1 with `error[E0384]: cannot assign twice to immutable
variable \`n\``. The `-->` location points at `count.rs:4:5`, where
`n = 2;` starts. The `help:` line says `consider making this binding
mutable`.)

## What To Ignore For Now

This lesson installs only one idea: `mut` toggles a `let` binding
from the immutable default to mutable. Each of the following is
real and will be taught later, but is *not* part of this move:

- *Shadowing*: `let x = 5; let x = 6;` rebinds the same name with
  a second `let` (no `mut`). The E0384 explainer page mentions
  this as an alternative. Different mechanism; deferred.
- The deeper *why* of immutability-by-default. The Book motivates
  it at length; rationale deferred.
- Constants: `const NAME: T = value;`. Different keyword, requires
  type annotation. Out of scope.
- Type annotations on `let`, e.g. `let mut x: i32 = 5;`. Deferred.
- The *kind* of value `5` is. Still deferred.
- Whether `let mut x = 5; x = "hi";` works. It does not — `mut`
  does not let you change a binding to a *different kind of
  value*. That is a *type* rule, and types are deferred.
- Patterns on the left of `let`, e.g. `let (mut a, b) = ...;`.
  Deferred.
- `&mut` references. A different use of `mut` from the
  binding-level one here. Out of scope.
- *Interior mutability* (`Cell`, `RefCell`, etc.). Out of scope.
- The full format-string DSL. Still deferred from lesson 005; this
  lesson uses only `{x}`.
- Comments (`//`), `cargo`, defining your own functions, and
  calling functions other than `println!`. Still deferred.

## Evidence

### Sources

- `output/docs/rust/book/ch03-01-variables-and-mutability.md`,
  lines 1-105 (the immutable-by-default → `mut` walkthrough; the
  chapter's "Constants" section starting at line 109 is out of
  scope). Two direct quotes are load-bearing:
  - Lines 4-6: "by default, variables are immutable." This is the
    canonical statement of the default the lesson teaches.
  - Lines 73-75: "Although variables are immutable by default, you
    can make them mutable by adding `mut` in front of the variable
    name." This licences the move's keyword placement.
  The chapter walks through exactly the same `let x = 5; ... x = 6;`
  → E0384 → add `mut` sequence the lesson uses, with the same
  printed output (`x = 5` then `x = 6`). Calibration: the Book
  builds the program inside `cargo new variables` and uses
  `cargo run`; this lesson uses `rustc demo.rs` then `./demo`
  directly, consistent with the lesson 001 toolchain. The behavior
  observed (E0384 on missing `mut`, success with `mut`) is the
  same.
- `output/docs/rust/error_codes/E0384.md`. Three direct quotes are
  load-bearing:
  - Line 4: "An immutable variable was reassigned." The
    one-sentence statement of the error.
  - Line 15: "By default, variables in Rust are immutable."
    Independent corpus restatement of the default.
  - Lines 15-16: "To fix this error, add the keyword `mut` after
    the keyword `let` when declaring the variable." This is the
    canonical fix the `help:` line on the captured probe also
    suggests. The explainer also mentions shadowing (`let x = 3;
    let x = 5;`) as an alternative; this lesson explicitly defers
    shadowing.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/006-mut-binding.rs`.
The committed file is the *working* version (with `let mut x = 5;`).
The broken contrast (remove `mut`) is documented as a second run
inside this Evidence section, not as a separate `.rs` file.

Probe transcript, both runs in the same temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64

=== RUN 1: working program with let mut ===
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
x = 5
x = 6
exit=0

=== RUN 2: broken program — let without mut, attempt reassign ===
--- cat demo.rs ---
fn main() {
    let x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");
}
--- rustc demo.rs (capturing stderr) ---
error[E0384]: cannot assign twice to immutable variable `x`
 --> demo.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("x = {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0384`.
exit=1
--- ls after broken compile ---
demo
demo.rs
```

Notes from the transcript:

- Run 1 (working `let mut x = 5;` then `x = 6;`): `rustc` exits 0,
  silent. `./demo` prints `x = 5` then `x = 6` and exits 0. This
  load-bearing observation is that the *first* `println!` sees the
  pre-reassignment value (`5`) and the *second* sees the
  post-reassignment value (`6`), which is just lesson 004's
  source-order rule applied across a reassignment.
- Run 2 (drop `mut`): `rustc` exits 1 with `error[E0384]`. The
  source-excerpt block contains *two* annotations: a short `-`
  under the `x` on line 2 labelled `first assignment to \`x\``, and
  five `^` characters under `x = 6` on line 4 labelled `cannot
  assign twice to immutable variable`. A `help:` line follows
  saying `consider making this binding mutable`, with a small
  source excerpt showing `let mut x = 5;` and `+++` indicating
  where to insert `mut`. The `--explain E0384` trailer follows
  because the error has an `E####` code. The lesson 003 map covers
  this block — headline, location, source excerpt, help — without
  new vocabulary.
- The `demo` listed in `ls after broken compile` is the executable
  from Run 1; Run 2 did not produce a new one (consistent with
  lesson 001's two-step picture).
- Only the working source is committed under `observations/`; the
  broken version exists only inside this transcript. The temp dir
  was removed; `git status` shows only the working `.rs` file
  added.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs
  when the executable launches.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — the
  four-part map (headline, `-->` location, source excerpt with
  caret, help/note) is what the learner uses to read the contrast
  probe's E0384 block. This lesson cites that map and does not
  re-teach diagnostic reading.
- `004-statements-in-order` (accepted, load-bearing) —
  `;`-terminated statements in `fn main` execute top to bottom in
  source order. That is what makes "first `println!` shows `5`,
  then `x = 6;` runs, then second `println!` shows `6`" concrete.
- `005-let-binding` (accepted, load-bearing) — the statement
  `let name = value;` binds a name to a value; `println!("...
  {name} ...")` substitutes the bound value at print time. This
  lesson adds exactly one keyword (`mut`) between `let` and `name`
  and one new statement shape (`name = new_value;`) that
  reassigns.
