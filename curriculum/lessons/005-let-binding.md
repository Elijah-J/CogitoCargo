---
id: 005-let-binding
move: "bind a name to a value with `let name = value;`, then use the name in a later statement"
main_concept: "`let name = value;` is a statement that binds a name to a value; from that statement onward in the same body, the name refers to that value; without the binding, rustc errors with E0425 (`cannot find value in this scope`)"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 004-statements-in-order
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "mut / mutability" moves
  - future "type annotations on let" moves
  - future "shadowing" moves
  - future "format-string DSL" moves
sources:
  - output/docs/rust/book/ch03-01-variables-and-mutability.md
  - output/docs/rust/book/ch03-03-how-functions-work.md
  - output/docs/rust/error_codes/E0425.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/005-let-binding.rs
status: accepted
---

# Bind a name to a value with `let`

## The Move

Inside `fn main`, write a statement of the shape `let name = value;`.
That statement *binds* the name to the value. From the next statement
onward, you can write the name where the value would have gone, and
Rust treats it as that value. Delete the `let` line and `rustc` no
longer recognizes the name; it refuses to compile, citing
`error[E0425]: cannot find value \`name\` in this scope`.

## Mental Model Delta

- Before: "I can put `;`-terminated lines in `fn main` and they run
  in source order. But every line stands alone — there is no way for
  one line to refer to something an earlier line set up."
- After: "A `let name = value;` statement *creates an association*
  between a name and a value. Later statements in the same body can
  use that name as a stand-in for the value. The Book phrases this
  as 'once a value is bound to a name'. Without the `let`, the name
  is just an unknown identifier and `rustc` rejects the program."

## Prerequisites

- Installed concepts:
  - From lesson 001 (`001-rustc-compile-and-run`): `rustc file.rs`
    produces an executable next to the source; run it with
    `./name`. `rustc` is silent on success.
  - From lesson 002 (`002-fn-main-entry-point`): the body inside
    `fn main() { ... }` runs when the executable launches.
  - From lesson 003 (`003-read-rustc-diagnostic`): a rustc error has
    a headline, a `-->` location, a source excerpt with caret, and
    optional `help:` / `= note:` lines. The contrast probe below
    produces an `error[E0425]` that you can read using that map
    without re-teaching it.
  - From lesson 004 (`004-statements-in-order`): the body of
    `fn main` is a sequence of `;`-terminated statements that run
    top to bottom. That is what makes "later statements can use the
    bound name" concrete.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let x = 5;
    println!("x = {x}");
}
```

Two `;`-terminated statements live inside `fn main`. The first,
`let x = 5;`, binds the name `x` to the value `5`. (The Book also
calls bindings like this *variables*; this lesson sticks with
*binding* because nothing here ever changes after it is bound.)
Inside the string passed to `println!`, the form `{name}` substitutes
the value bound to `name` at print time; the broader format-string
DSL is deferred.

Compile and run:

```console
$ rustc demo.rs
$ ./demo
x = 5
```

The `5` came from the binding made one line earlier.

Now the contrast. Delete the `let x = 5;` line from `demo.rs`:

```rust
fn main() {
    println!("x = {x}");
}
```

Compile again:

```console
$ rustc demo.rs
error[E0425]: cannot find value `x` in this scope
 --> demo.rs:2:20
  |
2 |     println!("x = {x}");
  |                    ^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

Read it using the lesson 003 map: **headline** carries code `E0425`;
**location** `demo.rs:2:20`; **source excerpt** caret underlines the
`x` inside the format string; no `help:` / `= note:` lines, only an
inline `not found in this scope` annotation; the `--explain E0425`
trailer follows. Without the `let`, the name `x` in the next
statement has no meaning.

Restore `let x = 5;` and it compiles again. Change `5` to `7`,
recompile, rerun, and the output becomes `x = 7`.

## What Changed

- You can introduce a name and a value in one statement
  (`let name = value;`) and use that name in any later statement of
  the same body.
- You have a working noun for what `let` makes: a *binding*, an
  association between a name and a value.
- You know one specific way to print a bound value:
  `println!("... {name} ...");` substitutes the value at print time.
- You can read the rustc error you get when a name has no binding:
  `error[E0425]: cannot find value \`name\` in this scope`. The
  error is a structural signal that no `let` introduced that name.

## Check Yourself

(a) You write `tiny.rs` containing:

```rust
fn main() {
    let n = 3;
    println!("n is {n}");
}
```

You run `rustc tiny.rs` and then `./tiny`. What does it print, and
what is the exit code of `./tiny`?

(b) You then delete the `let n = 3;` line, leaving only the
`println!`. You run `rustc tiny.rs` again. Does an updated `tiny`
executable get produced? Roughly what does `rustc` print, and which
identifier will the caret point at?

(Answers: (a) it prints `n is 3` and exits 0. (b) No new executable
is produced; `rustc` exits 1 and prints an `error[E0425]: cannot
find value \`n\` in this scope` block, with the caret pointing at
the `n` inside the `{n}` placeholder; the `--explain E0425` trailer
follows.)

## What To Ignore For Now

This lesson installs only one idea: `let name = value;` binds a name
to a value, and later statements in the same body can use the name.
Each of the following is real and will be taught later, but is *not*
part of this move:

- `mut` and the rule that bindings are immutable by default. We
  never try to *reassign* `x` here, so the immutable default never
  fires; `mut` is a separate later move. (covered in lesson 006.)
- *Shadowing*: writing `let x = 5; let x = "hi";` to rebind the same
  name. Deferred. (covered in lesson 007; type-changing shadowing in lesson 057.)
- The word *type* and what kind of value `5` is. We just call `5`
  "the value `5`". Type annotations like `let x: i32 = 5;` are
  deferred. (the concept of *type* and the `i32` annotation form are covered in lesson 019; `f64` in lesson 033; `u32` in lesson 062.)
- Pattern destructuring on the left side of `let`, e.g.
  `let (a, b) = ...;`. Deferred.
- The full format-string DSL. We use only the named `{x}` form. An
  alternative form, `println!("{}", x)` with a positional `{}` and
  the value supplied as an extra argument, exists but is deferred. (positional `{}` covered in lesson 011.)
- *Scope* as a general concept. The contrast probe says `x` is
  "not found in this scope"; we do not unpack what scope is or
  where exactly a binding ends. (covered in lesson 068.)
- The full *statement vs expression* distinction. Still deferred
  from lesson 004. (covered in lesson 024.)
- Comments (`//`), `cargo`, defining your own functions, and
  calling functions other than `println!`. Still deferred. (line comments in lesson 010; block comments in lesson 018; cargo starting in lesson 032; defining and calling functions in lesson 008.)

## Evidence

### Sources

- `output/docs/rust/book/ch03-01-variables-and-mutability.md`,
  line 12. Direct quote: "When a variable is immutable, once a value
  is bound to a name, you can't change that value." This is the
  corpus source for "binds a name to a value." The chapter's example
  also reassigns `x` and triggers an immutability error; this lesson
  uses only the *first* `let` from that pattern and never attempts
  to reassign. The "can't change" half is deferred to a future `mut`
  lesson.
- `output/docs/rust/book/ch03-03-how-functions-work.md`, line 75:
  `println!("The value of x is: {x}");`. Corpus existence proof for
  the named-placeholder `{x}` form, separate from any `let`
  discussion.
- `output/docs/rust/error_codes/E0425.md`. Direct quote: "An
  unresolved name was used." The explainer page shows the canonical
  fix — adding the missing `let unknown_variable = 12u32;` before
  the use site — which is the structural shape this lesson teaches.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/005-let-binding.rs`.
The committed file is the *working* version (with `let x = 5;`).
The broken contrast (delete the `let` line) is documented as a
second run inside this Evidence section, not as a separate `.rs`
file.

Probe transcript, both runs in the same temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64

=== RUN 1: working program with let binding ===
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let x = 5;
    println!("x = {x}");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
x = 5
exit=0

=== RUN 2: broken program with no let ===
--- cat demo.rs ---
fn main() {
    println!("x = {x}");
}
--- rustc demo.rs (capturing stderr) ---
error[E0425]: cannot find value `x` in this scope
 --> demo.rs:2:20
  |
2 |     println!("x = {x}");
  |                    ^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
exit=1
--- ls after broken compile ---
demo
demo.rs
```

Notes:

- Run 1 (working): `rustc` exits 0, silent. `./demo` prints `x = 5`.
  The `5` printed is the value bound to `x` by the `let` above.
- Run 2 (no `let`): `rustc` exits 1 with `error[E0425]`. The caret
  at `demo.rs:2:20` lands on the `x` inside `{x}` — that is the
  unresolved use site. The `--explain E0425` trailer follows because
  the error has an `E####` code. The lesson 003 map covers this
  block with no new vocabulary.
- The `demo` listed in `ls after broken compile` is the executable
  from Run 1; Run 2 did not produce a new one (consistent with
  lesson 001's two-step picture).
- Only the working source is committed under `observations/`; the
  broken version exists only inside this transcript. The temp dir
  was removed; `git status` confirms only the working `.rs` was
  added.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs
  when the executable launches.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — the
  four-part map (headline, `-->` location, source excerpt with
  caret, help/note) is what the learner uses to read the contrast
  probe's E0425 block. This lesson cites that map and does not
  re-teach diagnostic reading.
- `004-statements-in-order` (accepted, load-bearing) —
  `;`-terminated statements in `fn main` execute top to bottom in
  source order. That is what makes "the `println!` on line 3 sees
  the binding made by the `let` on line 2" concrete.
