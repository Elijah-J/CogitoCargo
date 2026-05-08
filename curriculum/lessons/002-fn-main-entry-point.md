---
id: 002-fn-main-entry-point
move: "recognize fn main() as the entry point of a tiny Rust program"
main_concept: "every tiny rustc-compiled Rust program needs fn main() { ... }; the body of main runs when the executable is launched, and if there is no main, rustc refuses to build an executable (E0601)"
depends_on:
  - 001-rustc-compile-and-run
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - 003-read-rustc-diagnostic
  - future let-binding moves
  - future "define your own function" moves
sources:
  - output/docs/rust/book/ch01-02-hello-world.md
  - output/docs/rust/reference/crates-and-source-files.md
  - output/docs/rust/error_codes/E0601.md
  - output/docs/rust/rust-by-example/hello.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/002-fn-main-entry-point.rs
status: accepted
---

# `fn main` is the entry point

## The Move

Look at the tiny Rust program from lesson 001. The first line says
`fn main()`. Recognize that line as the *entry point*: when you run
the executable that `rustc` produced, the code inside the `{ ... }`
attached to `fn main` is what actually runs. Rename `main` to anything
else and `rustc` will refuse to build an executable at all.

## Mental Model Delta

- Before: "`fn main() { ... }` is required boilerplate around the
  interesting line. I do not know why it is there or what would happen
  without it."
- After: "`main` is a *specific name* with a specific job. The body
  attached to `fn main` is the code that runs when I launch the
  executable. The shape `fn name() { ... }` is what the corpus calls
  a *function*; for now I can read it as a named block of code. The
  body of `main` is special only because the name is `main`. Without
  a `main`, `rustc` will not produce an executable; it errors out
  with `error[E0601]: \`main\` function not found in crate \`...\``."

## Prerequisites

- Installed concepts:
  - From lesson 001 (`001-rustc-compile-and-run`): you can save a
    tiny Rust program to a `.rs` file, run `rustc file.rs` to produce
    an executable next to the source, and run that executable with
    `./name`. You also know that `rustc` is silent on success.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Use the same program from lesson 001. Make a fresh empty directory,
`cd` into it, and create `hello.rs` containing exactly:

```rust
fn main() {
    println!("hello from rustc");
}
```

Compile and run, just like lesson 001:

```console
$ rustc hello.rs
$ ./hello
hello from rustc
```

Now the contrast. Edit `hello.rs` and rename `main` to `start`,
changing nothing else:

```rust
fn start() {
    println!("hello from rustc");
}
```

Compile again:

```console
$ rustc hello.rs
error[E0601]: `main` function not found in crate `hello`
 --> hello.rs:3:2
  |
3 | }
  |  ^ consider adding a `main` function to `hello.rs`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0601`.
```

The line that matters for this lesson is the first one:

> `error[E0601]: \`main\` function not found in crate \`hello\``

Do not try to read the rest of the diagnostic in detail yet; that is
its own lesson. For now, just register: `rustc` looked for something
named `main` in your file, did not find it, and gave up.

Rename `start` back to `main`, recompile, and `./hello` works again.

## What Changed

- You can point at the line `fn main() { ... }` in any tiny Rust
  program and say: "this is the entry point. The body inside the
  curly braces is what runs."
- You know `main` is a *specific name*, not a placeholder. Other
  names in that slot (`start`, `run`, `begin`, your own initials) do
  not work for a normal executable.
- You can recognize the rustc error message that appears when `main`
  is missing: `error[E0601]: \`main\` function not found in crate ...`.
- You have a name for the `fn name() { ... }` shape: the corpus
  calls it a *function*. For now you can read it as a named block
  of code; functions as a concept come in a later lesson.

## Check Yourself

You write `greet.rs` containing:

```rust
fn greet() {
    println!("hi");
}
```

You run `rustc greet.rs`.

- Does an executable named `greet` appear?
- What does `rustc` print, roughly?
- What is the smallest edit to `greet.rs` that would make
  `rustc greet.rs` succeed and `./greet` print `hi`?

(Answers: no executable appears; `rustc` prints an `error[E0601]`
diagnostic that mentions `\`main\` function not found in crate
\`greet\``; rename `fn greet` to `fn main`, leaving the body alone.)

## What To Ignore For Now

This lesson installs only one idea: `main` is the entry point, and
without it `rustc` will not build an executable. Each of the following
is real and will be taught later, but is *not* part of this move:

- Defining your own functions. Any `fn name() { ... }` other than
  `fn main` is out of scope here. We are only learning to *recognize*
  the shape attached to `main`. (covered in lesson 008.)
- What goes inside the `()` after `main`. For now, treat the empty
  `()` as part of the required shape. Function parameters are a
  separate later move. (parameters covered in lesson 020; multiple parameters in lesson 036.)
- Function return values, the `->` arrow, `main() -> Result<...>`,
  and the `Termination` trait that the Rust Reference mentions next
  to `main`. The empty body shape `fn main() { ... }` is enough for
  every program in the next several lessons. (the `->` arrow and ordinary return values are covered in lesson 021; `main() -> Result<...>` and the `Termination` trait remain deferred — they require trait machinery, deferred since lesson 040.)
- Calling functions. We are not calling `main` from anywhere; the
  runtime arranges for its body to run. We are also not calling other
  functions from inside `main` yet. `println!` is technically a macro
  invocation, not a function call, and is still deferred from lesson
  001. (covered in lesson 008.)
- The `#![no_main]` attribute and any "main can be imported" exotica
  the Reference mentions. Ignore them; they exist for advanced cases. (out of scope; advanced feature.)
- What the operating system and Rust runtime do *before* control
  reaches `main`. Treat that machinery as someone else's problem for
  now; just trust that the body of `main` is where your code starts. (out of scope; not Rust-specific.)
- Reading the rustc diagnostic line by line. The next lesson
  (`003-read-rustc-diagnostic`) will teach how to actually parse a
  diagnostic. For this lesson, only the phrase `\`main\` function
  not found in crate \`...\`` matters. (covered in lesson 003.)
- `println!`, the `!`, the trailing `;`, and the string literal are
  still deferred from lesson 001. (trailing `;` covered in lesson 004; `println!` operational use covered in lesson 011; string-literal type `&str` covered incidentally in lesson 055; the `!`/macro concept remains deferred.)
- Cargo. Still deferred; we are only using `rustc` directly. (covered starting in lesson 032.)

## Evidence

### Sources

- `output/docs/rust/book/ch01-02-hello-world.md` — "The Anatomy of a
  Rust Program" walkthrough. Direct quote: "These lines define a
  function named `main`. The `main` function is special: It is always
  the first code that runs in every executable Rust program." Also:
  "the first line declares a function named `main` that has no
  parameters and returns nothing", and "The function body is wrapped
  in `{}`. Rust requires curly brackets around all function bodies."
- `output/docs/rust/reference/crates-and-source-files.md`, section
  `## Main functions`. Direct quote: "A crate that contains a `main`
  function can be compiled to an executable." The same section lists
  the formal restrictions on `main` (no arguments, no trait or
  lifetime bounds, no where clauses, return type must implement
  `Termination`); for this lesson only the existence-and-shape claim
  is load-bearing, and the restrictions are explicitly deferred under
  `## What To Ignore For Now`.
- `output/docs/rust/error_codes/E0601.md` — Direct quote: "No `main`
  function was found in a binary crate. To fix this error, add a
  `main` function". This is the canonical description of the error
  the contrast probe reproduces.
- `output/docs/rust/rust-by-example/hello.md` — Annotates the same
  Hello World program with the comments `// This is the main
  function.` and `// Statements here are executed when the compiled
  binary is called.` This is the most direct corpus statement that
  the body of `main` is what runs when you launch the executable.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/002-fn-main-entry-point.rs`.
The committed file is the *broken* version (`fn start`), so that
re-running `rustc` on it reproduces the E0601 diagnostic. The header
comment in the file documents the rename and the working version.

Probe transcript, run in a clean temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64

--- ls before compile ---
hello.rs
--- rustc hello.rs (working: fn main) ---
exit=0
--- ls after compile ---
hello
hello.rs
--- ./hello ---
hello from rustc
exit=0

--- edit: rename fn main to fn start, body unchanged ---
hello.rs now contains:
fn start() {
    println!("hello from rustc");
}

--- rustc hello.rs (broken: fn start) ---
error[E0601]: `main` function not found in crate `hello`
 --> hello.rs:3:2
  |
3 | }
  |  ^ consider adding a `main` function to `hello.rs`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0601`.
exit=1

--- ls after broken compile ---
hello
hello.rs

--- edit: rename fn start back to fn main ---
--- rustc hello.rs (back to main) ---
exit=0
--- ./hello ---
hello from rustc
exit=0
```

Notes from the transcript:

- Working `fn main`: `rustc` exits 0, silent, produces `hello`
  alongside `hello.rs`, and `./hello` prints `hello from rustc`.
  This re-confirms the lesson 001 workflow before the contrast.
- Broken `fn start`: `rustc` exits 1 and prints an `error[E0601]`
  diagnostic whose first line is exactly
  `` error[E0601]: `main` function not found in crate `hello` ``.
  The crate name `hello` matches the source file name `hello.rs`.
- The `hello` executable in `ls after broken compile` is the *old*
  one from the previous successful build. `rustc` did not produce a
  new executable on this failed run. (This is consistent with lesson
  001's claim that compilation and running are two separate steps:
  failed compilation leaves whatever was previously built untouched
  and just refuses to make a new one.)
- Renaming back to `fn main` and recompiling restores the working
  state.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — installs the
  rustc-compile-and-run workflow that this lesson assumes: `.rs`
  source compiled with `rustc file.rs` produces an executable next
  to the source, run with `./name`, and `rustc` is silent on success.
