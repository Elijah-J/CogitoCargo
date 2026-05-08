---
id: 011-println-positional-args
move: "use println! with one or more `{}` positional placeholders in the format string and one extra argument per `{}`; arguments fill the placeholders in left-to-right order"
main_concept: "each `{}` in a println! format string is a positional placeholder consumed by one extra argument from the comma-separated list after the format string; placeholders match arguments left-to-right; you can pass an expression like `a + b` as an argument directly without binding it first"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
  - 005-let-binding
  - 009-arithmetic-on-integers
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "format specifiers" moves
  - future "Debug formatting {:?}" moves
  - future "format! and eprintln!" moves
  - future "named + positional mixing" moves
sources:
  - output/docs/rust/std/macro.println.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/011-println-positional-args.rs
status: accepted
---

# Print multiple values with `{}` positional placeholders

## The Move

Inside `fn main`, write a `println!` whose format string contains one
or more bare `{}` placeholders, then list one extra argument per `{}`,
separated by commas, after the format string. The arguments fill the
placeholders in left-to-right order. An argument can be a bound name
(`a`), a literal, or an expression like `a + b` that does not need a
`let` of its own.

## Mental Model Delta

- Before: "to print a bound name, I write `println!("... {name} ...");`
  with the name inside the braces (lesson 005). I do not have a way to
  print an expression like `a + b` without first writing
  `let sum = a + b;`."
- After: "`println!` accepts another form: a bare `{}` in the format
  string is a *positional placeholder*. After the format string I can
  list extra arguments separated by commas, one per `{}`. The first
  `{}` is filled by the first extra argument, the second by the second,
  and so on, left to right. Each extra argument is an ordinary
  expression, so `a + b` can sit there directly."

## Prerequisites

- Installed concepts:
  - Lesson 001 (`001-rustc-compile-and-run`): `rustc file.rs` produces
    an executable next to the source; `./name` runs it; `rustc` is
    silent on success.
  - Lesson 002 (`002-fn-main-entry-point`): the body of
    `fn main() { ... }` runs when the executable launches.
  - Lesson 004 (`004-statements-in-order`): the body of `fn main` is a
    sequence of `;`-terminated statements that run top to bottom.
  - Lesson 005 (`005-let-binding`, load-bearing): `let name = value;`
    binds a name to a value; later statements use the name as that
    value; one form of `println!` substitution is the named placeholder
    `println!("... {name} ...");`. This lesson teaches an alternative
    form for the same job, plus a strict generalization.
  - Lesson 009 (`009-arithmetic-on-integers`): `a + b` between two
    integers is itself a value, so it can sit anywhere a value can
    sit. We rely on that to put `a + b` directly into the argument
    list of `println!`.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `main.rs`
containing exactly:

```rust
fn main() {
    let a = 5;
    let b = 10;
    println!("{} + {} = {}", a, b, a + b);
    println!("first = {}, second = {}", a, b);
}
```

Compile and run:

```console
$ rustc main.rs
$ ./main
5 + 10 = 15
first = 5, second = 10
```

Walk through the first `println!`. The format string `"{} + {} = {}"`
has three `{}` placeholders. The three extra arguments after it fill
them left to right: `a` (5), `b` (10), `a + b` (15). The third
argument is the load-bearing new piece: `a + b` is an expression, so
it goes straight into the argument list with no separate
`let sum = a + b;` line.

The second `println!` is the same idea with two placeholders: `a` and
`b` fill the two `{}`s in order, giving `first = 5, second = 10`.

Contrast with lesson 005's named form. When a value is already bound
to a name, both forms print the same thing:

```rust
println!("first = {a}");      // named placeholder, lesson 005
println!("first = {}", a);    // positional placeholder, this lesson
```

The named form pulls `a` from the surrounding scope; the positional
form takes `a` as an extra argument supplied to `println!` directly.
The std documentation shows both forms back to back as its canonical
examples:

> `println!("format {} arguments", "some");`
> `let local_variable = "some";`
> `println!("format {local_variable} arguments");`

The first line is the positional form this lesson installs; the third
is the named form from lesson 005.

## What Changed

- You can write `println!("... {} ... {} ...", arg1, arg2);` with one
  `{}` per slot and one extra argument per `{}`, separated by commas.
- You know placeholders bind to arguments left-to-right: the first
  `{}` takes the first extra argument, the second `{}` the second,
  and so on.
- You can print an expression like `a + b` directly, without binding
  it to a name first; the expression goes into the argument list.
- You have two ways to print a bound value: the named form
  `println!("... {name} ...")` from lesson 005, or the positional form
  `println!("... {} ...", name)` from this lesson.

## Check Yourself

You write `tiny.rs` containing:

```rust
fn main() {
    let x = 4;
    let y = 6;
    println!("{} times {} is {}", x, y, x * y);
}
```

You run `rustc tiny.rs` and then `./tiny`.

- How many `{}` placeholders are in the format string, and how many
  extra arguments come after it?
- What single line does the executable print?

(Answers: three placeholders and three extra arguments; it prints
`4 times 6 is 24`.)

## What To Ignore For Now

This lesson installs only one idea: each `{}` is a positional
placeholder consumed by one extra argument, left-to-right. Each of
the following is real and will be taught later, but is *not* part of
this move:

- *Format specifiers* inside the braces, like `{:?}` (debug
  formatting), `{:5}` (width), `{:.2}` (precision), `{:>10}`
  (alignment), `{:#x}` (hex). The std `fmt` documentation covers
  these; deferred.
- *Debug formatting* (`{:?}`) for types that do not print directly.
  This lesson uses only integers, which print with bare `{}`.
- The macros `format!` and `eprintln!`. They share `println!`'s
  argument syntax but write somewhere different (a `String` and
  stderr respectively). Out of scope here.
- *Mixing positional `{}` and named `{name}` placeholders in one
  format string*. Modern Rust allows it, but the argument-counting
  rule shifts; deferred.
- *Compile errors from misused placeholders* — for example, listing
  too few or too many arguments for the number of `{}`s. `rustc`
  catches these at compile time. We do not exercise the failure here.
- *What "macro" actually means* and what the trailing `!` after
  `println` signifies. Still deferred from lesson 001; treat
  `println!` as an opaque tool with a documented argument syntax.
- All previously-deferred items: `mut`, shadowing, the broader
  format-string DSL, types and type annotations, `cargo`, function
  parameters, return values, and the full statement-vs-expression
  distinction.

## Evidence

### Sources

- `output/docs/rust/std/macro.println.md`, lines 1-55. Two
  load-bearing direct quotes:
  - Line 15: "Prints to the standard output, with a newline." This
    confirms the macro's job: each `println!` call produces one line
    of output on stdout.
  - Lines 50-54, the `## Examples` section, showing the canonical
    forms back to back:
    `println!("format {} arguments", "some");` (positional) and
    `println!("format {local_variable} arguments");` (named, after a
    `let local_variable = "some";`). This is the corpus's existence
    proof for the positional `{}` form alongside the lesson-005 named
    form.
  Calibration: the same page also discusses locking stdout for hot
  loops, panic conditions when stdout writes fail, and links to
  `std::fmt` for the broader macro argument syntax. All three are
  deferred under What To Ignore For Now.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/011-println-positional-args.rs`.
The committed file is the working program shown in Try It.

Probe transcript, run in a temp directory created with `mktemp -d`
and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
main.rs
--- cat main.rs ---
fn main() {
    let a = 5;
    let b = 10;
    println!("{} + {} = {}", a, b, a + b);
    println!("first = {}, second = {}", a, b);
}
--- rustc main.rs ---
exit=0
--- ls after compile ---
main
main.rs
--- ./main ---
5 + 10 = 15
first = 5, second = 10
exit=0
```

Notes:

- `rustc` exits 0 and is silent (consistent with lesson 001).
- The first `println!` line has three `{}` placeholders consumed by
  three extra arguments in left-to-right order: `a` (5), `b` (10),
  `a + b` (15). Output: `5 + 10 = 15`.
- The second `println!` line has two `{}` placeholders consumed by
  `a` (5) and `b` (10) in order. Output: `first = 5, second = 10`.
- The third argument to the first `println!` is the expression
  `a + b`, not a separately bound name. It fills the third `{}` with
  the value `15` directly.
- Only the working source is committed under `observations/`. No
  binaries are committed. The temp dir was removed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs when
  the executable launches.
- `004-statements-in-order` (accepted) — `;`-terminated statements
  in `fn main` run top to bottom.
- `005-let-binding` (accepted, load-bearing) — `let name = value;`
  binds a name to a value, and `println!("... {name} ...");`
  substitutes the bound value at print time. The new thing in this
  lesson is a different way to fill placeholder slots: bare `{}`s
  with extra arguments after the format string.
- `009-arithmetic-on-integers` (accepted) — `a + b` between two
  integers is an expression that produces a value, so it can sit
  directly in the argument list of `println!`.
