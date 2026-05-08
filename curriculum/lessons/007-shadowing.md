---
id: 007-shadowing
move: "rebind an existing name with a second `let name = ...;` to shadow the old binding"
main_concept: "a second `let` with the same name does not reassign the existing binding — it creates a new binding that shadows the old one; from that statement onward the name refers to the new value; shadowing requires repeating `let` and does NOT require `mut` (different mechanism from lesson 006's reassignment)"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
  - 005-let-binding
  - 006-mut-binding
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "scope" moves
  - future "type annotations on let" moves
  - future "arithmetic / operators" moves
  - future "type-changing shadowing" moves
sources:
  - output/docs/rust/book/ch03-01-variables-and-mutability.md
  - output/docs/rust/error_codes/E0384.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/007-shadowing.rs
status: accepted
---

# Shadow a binding with a second `let`

## The Move

Inside `fn main`, write `let x = 5;`, then later in the same body
write `let x = 10;`. Both statements start with `let`, both use the
same name. The second `let` does not change the first binding; it
creates a *new* binding under the same name. From that statement
forward, the name `x` refers to the new value. No `mut` is involved.

## Mental Model Delta

- Before: "Lesson 006 said plain `let` is immutable. The only way I
  know to make `x` refer to a new value is `let mut x = 5;` plus
  `x = 10;`."
- After: "There is a second way: write a *second* `let x = ...;`.
  That is *shadowing*. The Book says 'the first variable is
  *shadowed* by the second, which means that the second variable
  is what the compiler will see when you use the name of the
  variable.' Shadowing does not reassign the old binding — it
  creates a fresh binding under the same name. That is why it
  does not need `mut`."

## Prerequisites

- Installed concepts:
  - Lessons 001 and 002: `rustc file.rs` produces an executable
    next to the source; the body of `fn main` runs.
  - Lesson 004 (load-bearing): `;`-terminated statements in
    `fn main` run top to bottom — what makes "first `println!`
    shows `5`, then a second `let` runs, then second `println!`
    shows `10`" concrete.
  - Lesson 005 (load-bearing): `let name = value;` binds a name;
    `println!("... {name} ...")` substitutes the bound value. This
    lesson reuses that exact statement shape — twice.
  - Lesson 006 (load-bearing for the *contrast*): plain `let` is
    immutable; bare `name = new_value;` (no `let`) after a plain
    `let` fails with `error[E0384]: cannot assign twice to
    immutable variable`. Its full transcript is in lesson 006's
    `## Evidence`; this lesson references it by id rather than
    re-capturing it.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let x = 5;
    println!("first: x = {x}");
    let x = 10;
    println!("second: x = {x}");
}
```

Four `;`-terminated statements live in `fn main`. The first and
third are *both* `let` statements binding the same name `x`. No
`mut`, no bare `x = 10;` reassignment.

Compile and run:

```console
$ rustc demo.rs
$ ./demo
first: x = 5
second: x = 10
```

By lesson 004's source-order rule, `let x = 5;` runs first and the
first `println!` substitutes that value (`5`). Then `let x = 10;`
runs — a *second* `let` with the same name. The Book's phrasing:
"We can shadow a variable by using the same variable's name and
repeating the use of the `let` keyword." A new binding is created
under the name `x`; from that statement forward, uses of `x` see
the new binding, so the second `println!` prints `10`.

Contrast with lesson 006. The nearby program

```rust
fn main() {
    let x = 5;
    x = 10;     // no second `let` — reassignment, requires `mut`
}
```

does *not* compile: bare `x = 10;` (no `let`) is *reassignment*,
and reassignment of an immutable binding fails with `error[E0384]:
cannot assign twice to immutable variable \`x\``. That diagnostic
is in lesson 006's `## Evidence`; do not re-run it. This lesson's
program is different — every line that mentions `x` starts with
`let` — and the Book draws the distinction explicitly: "Shadowing
is different from marking a variable as `mut` because we'll get a
compile-time error if we accidentally try to reassign to this
variable without using the `let` keyword."

## What Changed

- You can rebind a name to a new value with a second
  `let name = ...;`. No `mut` needed.
- You have a name for it: *shadowing*. The first binding is
  *shadowed* by the second; uses of the name after the second `let`
  see the new binding.
- You can tell shadowing apart from lesson 006's reassignment by
  looking at the line: if it starts with `let`, it is a new binding
  (shadowing); if it is bare `name = value;`, it is reassignment
  (and requires `mut`).
- One-line definition: shadowing creates a new binding under the
  same name rather than mutating the old one.

## Check Yourself

You write `count.rs` containing:

```rust
fn main() {
    let n = 1;
    println!("a: n = {n}");
    let n = 7;
    println!("b: n = {n}");
}
```

(a) You run `rustc count.rs` and then `./count`. Does it compile? If
yes, what does it print, in order, and what is the exit code?

(b) You then edit line 4, replacing `let n = 7;` with bare
`n = 7;` (delete the keyword `let`, change nothing else). You run
`rustc count.rs` again. Does it still compile? If not, which
`E####` code do you expect, and what is the smallest one-keyword
edit that would fix it without going back to shadowing?

(Answers: (a) it compiles; `./count` prints `a: n = 1` then
`b: n = 7`, exit 0; the second `let` shadows the first.
(b) it does not compile; `rustc` exits 1 with `error[E0384]:
cannot assign twice to immutable variable \`n\`` (same shape as
lesson 006's contrast probe). The smallest one-keyword fix is to
add `mut` on line 2 (`let mut n = 1;`), which is lesson 006's
move; restoring `let` on line 4 would put you back to shadowing.)

## What To Ignore For Now

This lesson installs only one idea: a second `let` with the same
name creates a new binding that shadows the old one. Each of the
following is real and will be taught later, but is *not* part of
this move:

- *Inner-scope shadowing*. The Book also shows shadowing inside
  `{ ... }` braces nested in the body, where the shadow ends when
  the inner braces close. That requires teaching what a *scope* is.
  Deferred. This lesson uses only two top-level `let` statements at
  the same level inside `fn main`.
- *Scope* as a general concept. The Book says a shadow lasts "until
  either it itself is shadowed or the scope ends"; this lesson uses
  only the "shadowed-again" half (no nested braces).
- *Arithmetic in the right-hand side of `let`*. The Book uses
  `let x = x + 1;` to shadow with a computed value. We have not
  introduced operators; deferred. Both `let`s here use plain
  literals.
- *Type-changing shadowing*. The Book argues shadowing's main
  practical advantage is that the new binding can hold a different
  *kind* of value (e.g. `let spaces = "   ";` then
  `let spaces = spaces.len();`). That depends on types and methods,
  both deferred. Both `let`s here bind the same kind of value.
- The `mut` rule itself. Lesson 006 already installed it; this
  lesson only contrasts with it.
- Shadowing is *not* equivalent to `mut`. Different mechanism (new
  binding vs. reassign), different syntax requirement (no `mut` vs.
  `mut`). The practical extras (type-changing, post-shadow
  immutability) are deferred along with types.
- All previously-deferred items: constants, type annotations,
  `&mut` references, interior mutability, the broader format-string
  DSL beyond `{name}`, comments, `cargo`, defining your own
  functions, and calling functions other than `println!`.

## Evidence

### Sources

- `output/docs/rust/book/ch03-01-variables-and-mutability.md`, the
  "Shadowing" section. Three direct quotes are load-bearing:
  - Lines 162-165 (definition): "you can declare a new variable
    with the same name as a previous variable. Rustaceans say that
    the first variable is *shadowed* by the second, which means
    that the second variable is what the compiler will see when you
    use the name of the variable." This is the corpus's name and
    one-line definition for the move.
  - Lines 168-169 (mechanism): "We can shadow a variable by using
    the same variable's name and repeating the use of the `let`
    keyword." This licences the exact syntactic move (a second
    `let` with the same name).
  - Lines 205-207 (shadowing vs `mut`): "Shadowing is different
    from marking a variable as `mut` because we'll get a
    compile-time error if we accidentally try to reassign to this
    variable without using the `let` keyword." This licences the
    contrast that *defines* the concept against lesson 006.

  Calibration: the same Book section also shows (a) inner-scope
  shadowing inside nested braces, (b) shadowing with arithmetic
  (`let x = x + 1;`, `let x = x * 2;`), and (c) type-changing
  shadowing (`let spaces = "   "; let spaces = spaces.len();`).
  This lesson uses *none* of those examples. It stays on the
  simplest case: two top-level `let` statements with the same
  name in the same `fn main` body, both binding the same kind of
  value (an integer literal). All three richer cases are listed
  under `## What To Ignore For Now`.

- `output/docs/rust/error_codes/E0384.md`, lines 25-26. The
  explainer for the reassignment error from lesson 006 cross-
  references this lesson's move: "Alternatively, you might
  consider initializing a new variable: either with a new bound
  name or (by [shadowing](...)) with the bound name of your
  existing variable." This is independent corpus evidence that the
  Rust documentation itself treats shadowing as the alternative to
  `mut` for this exact situation.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/007-shadowing.rs`.
The committed file is the working program — there is no broken
contrast probe, because the contrast (bare `x = 10;` without
`mut`) is exactly lesson 006's E0384 transcript, referenced by id.

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
    let x = 5;
    println!("first: x = {x}");
    let x = 10;
    println!("second: x = {x}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
first: x = 5
second: x = 10
exit=0
```

Notes from the transcript:

- `rustc` exits 0 silently. The program with two `let x = ...;`
  statements compiles with no error and no warning. In particular,
  `rustc` does *not* require `mut` for the second `let` — that is
  the load-bearing observation that distinguishes this move from
  lesson 006.
- `./demo` prints exactly two lines: `first: x = 5` then
  `second: x = 10`, exit 0. The first `println!` ran *before* the
  second `let`, so it saw the binding `x = 5`. The second
  `println!` ran *after* the second `let`, so it saw the new
  binding `x = 10`. That is lesson 004's source-order rule applied
  across a shadow.
- The contrast (bare `x = 10;` without `mut`) is *not* re-captured
  here. That diagnostic is in lesson 006's `## Evidence` (the
  E0384 block). This lesson references it by id; the same pattern
  lesson 003 used to reference lesson 002's E0601 transcript.
- The temp dir was removed; `git status` shows only the working
  `.rs` file added. No binaries committed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs
  when the executable launches.
- `004-statements-in-order` (accepted, load-bearing) —
  `;`-terminated statements in `fn main` execute top to bottom in
  source order. That is what makes "the first `println!` sees
  `x = 5`, then the second `let` runs, then the second `println!`
  sees `x = 10`" concrete.
- `005-let-binding` (accepted, load-bearing) — the statement
  `let name = value;` binds a name to a value; `println!("...
  {name} ...")` substitutes the bound value at print time. This
  lesson reuses that exact statement shape — twice — and the only
  new thing is the observation that two `let`s with the same name
  is allowed.
- `006-mut-binding` (accepted, load-bearing for the contrast) —
  plain `let` is immutable; bare `name = new_value;` requires
  `mut` or fails with E0384. This lesson defines its move *against*
  that one: shadowing is the other way to make a name refer to a
  new value, and it is mechanically different. The E0384 transcript
  is in lesson 006's `## Evidence`; it is referenced here, not
  replayed.
