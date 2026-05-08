---
id: 012-bool-literals
move: "bind a name to `true` or `false` and print it"
main_concept: "Rust has two boolean literal values, `true` and `false`, written as bare keywords (no quotes); you can bind one to a name with `let` and print it with `{name}`; the printed text is the word `true` or `false`"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
  - 005-let-binding
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "comparison operators" moves
  - future "if/else expressions" moves
  - future "logical operators && || !" moves
  - future "type annotations on let" moves
sources:
  - output/docs/rust/book/ch03-02-data-types.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/012-bool-literals.rs
status: accepted
---

# Bind a name to `true` or `false`

## The Move

Inside `fn main`, write `let yes = true;` and `let no = false;`. Print
each with `println!("... {name}")`. The executable prints the word
`true` and the word `false`. You now have a way to write down a Rust
value whose only job is to mean "yes" or "no".

## Mental Model Delta

- Before: "I can bind names to numbers like `5` with `let`, and I can
  put text like `"hello"` inside `println!`. I do not have a way to
  write down a yes-or-no value as itself."
- After: "Rust has exactly two *boolean literal* values, `true` and
  `false`. I write them as bare keywords — no quotes — on the right
  of a `let`. I can print them with the same `{name}` placeholder I
  use for numbers, and the output is the word `true` or `false`."

## Prerequisites

- Installed concepts:
  - From lesson 001 (`001-rustc-compile-and-run`): `rustc file.rs`
    produces an executable next to the source; run it with `./name`.
    `rustc` is silent on success.
  - From lesson 002 (`002-fn-main-entry-point`): the body inside
    `fn main() { ... }` is what runs when the executable launches.
  - From lesson 004 (`004-statements-in-order`): the body of `fn main`
    is a sequence of `;`-terminated statements that run top to bottom.
  - From lesson 005 (`005-let-binding`, load-bearing): `let name = value;`
    binds a name to a value; later statements in the same body can use
    the name; `println!("... {name}")` substitutes the bound value at
    print time.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `bools.rs`
containing exactly:

```rust
fn main() {
    let yes = true;
    let no = false;
    println!("yes = {yes}");
    println!("no = {no}");
}
```

Two new things to notice:

- `true` and `false` are *bare keywords*. They are not surrounded by
  `"..."` the way a string like `"hello"` is. If you type `"true"`
  with quotes, that is a string of four letters, not the boolean
  value the Book is talking about.
- The right side of `let yes = true;` is just `true`. The Book also
  shows a longer form — `let f: bool = false;` — that adds an explicit
  *type annotation*. Type annotations are a separate later move; the
  bare form here is enough.

Compile and run, like lesson 001:

```console
$ rustc bools.rs
$ ./bools
yes = true
no = false
```

Two lines of output, in source order (lesson 004): `yes = true`, then
`no = false`. The text `true` in the output came from the value bound
to `yes`; the text `false` came from the value bound to `no`. Rust's
`println!` already knows how to render a boolean as the word `true`
or the word `false`, with no extra setup on your part.

## What Changed

- You can write a yes/no value in Rust source: the literal `true` or
  the literal `false`, as bare keywords with no quotes.
- You can bind one to a name with `let` and print it with the same
  `{name}` placeholder you already use for numbers.
- You know what the printed output looks like: the bare word `true`
  or the bare word `false`.
- You have a name from the Book for what these values are: a *Boolean
  type in Rust has two possible values: `true` and `false`*. This
  lesson installs the two literal values and how to print them; the
  formal type called `bool` is a later move.

## Check Yourself

You write `flags.rs` containing:

```rust
fn main() {
    let on = true;
    let off = false;
    println!("on={on} off={off}");
}
```

You run `rustc flags.rs` and then `./flags`.

- How many lines does it print?
- What exact text appears between `on=` and the space?
- What exact text appears after `off=`?

(Answers: one line; the text `true`; the text `false`. The full output
line is `on=true off=false`.)

## What To Ignore For Now

This lesson installs only one idea: Rust has two boolean literal
values, `true` and `false`, and you can bind and print them. Each of
the following is real and will be taught later, but is *not* part of
this move:

- The formal type called `bool`. The Book says "The Boolean type in
  Rust is specified using `bool`" and shows the longer form
  `let f: bool = false;` with an explicit type annotation. The
  vocabulary of *types* and the syntax of `let name: type = value;`
  are deferred together.
- Comparison operators `==`, `!=`, `<`, `>`, `<=`, `>=`. In real code,
  most boolean values come from comparisons, not from typing `true`
  or `false` directly. Comparisons are their own later move.
- `if` and `else` expressions that consume a boolean. The Book itself
  forwards: "The main way to use Boolean values is through
  conditionals, such as an `if` expression." That is a later cycle.
- Logical operators `&&`, `||`, `!` that combine or negate booleans.
  Deferred.
- Boolean methods like `true.then(...)` or `then_some`. Out of scope.
- Booleans as keys, parts of tuples, or fields of bigger values. Out
  of scope.
- Everything previously deferred from earlier lessons: `mut`,
  shadowing, type annotations on `let`, defining your own functions,
  the full format-string DSL, comments, and `cargo`.

## Evidence

### Sources

- `output/docs/rust/book/ch03-02-data-types.md`, the
  "The Boolean Type" subsection (lines 201-219). Two load-bearing
  direct quotes:
  - Lines 203-204: "a Boolean type in Rust has two possible values:
    `true` and `false`." This is the corpus statement that licenses
    the lesson's main concept (two literal values).
  - Lines 217-219: "The main way to use Boolean values is through
    conditionals, such as an `if` expression. We'll cover how `if`
    expressions work in Rust in the 'Control Flow' section." This is
    the corpus's own forward-pointer, used as the basis for the
    `if`/`else` deferral in this lesson.

  Calibration: the same Book section also says "The Boolean type in
  Rust is specified using `bool`" and shows `let f: bool = false;`
  with an explicit type annotation. This lesson uses only the bare
  `let yes = true;` form and does not introduce the formal type
  vocabulary or the `let name: type = value;` syntax. Both are
  flagged as deferred.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/012-bool-literals.rs`.
The committed file is the exact working program below.

Probe transcript, run in a clean temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
bools.rs
--- cat bools.rs ---
fn main() {
    let yes = true;
    let no = false;
    println!("yes = {yes}");
    println!("no = {no}");
}
--- rustc bools.rs ---
exit=0
--- ls after compile ---
bools
bools.rs
--- ./bools ---
yes = true
no = false
exit=0
```

Notes:

- `rustc bools.rs` exits 0 and is silent (consistent with lesson 001).
- After compile, `ls` shows two files: `bools.rs` and the new
  executable `bools`.
- `./bools` prints exactly two lines, `yes = true` and `no = false`,
  in source order. The text after `=` is the word `true` and the word
  `false` respectively — that is, the two boolean literal values
  rendered as bare words by `println!`'s default handling.
- Only the working source is committed under `observations/`; no
  binaries are committed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `004-statements-in-order` (accepted) — `;`-terminated statements
  in `fn main` run top to bottom in source order; the two `println!`
  lines therefore print in the order written.
- `005-let-binding` (accepted, load-bearing) — `let name = value;`
  binds a name to a value; `println!("... {name}")` substitutes the
  bound value at print time. This lesson reuses both shapes verbatim
  with `true` / `false` on the right of `let`.
