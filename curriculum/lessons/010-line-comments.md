---
id: 010-line-comments
move: "add `//` somewhere on a line so rustc ignores everything from `//` to the end of the line"
main_concept: "`//` starts a line comment; rustc ignores everything from `//` to end of line; works at the start of a line, trailing after code, or to comment-out a `;`-terminated statement so it does not run"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "doc comments ///" moves
  - future "block comments /* */" moves
  - future "annotated probes use //" moves
sources:
  - output/docs/rust/book/ch03-04-comments.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/010-line-comments.rs
status: accepted
---

# Line comments with `//`

## The Move

Type `//` somewhere on a line in a Rust source file. Everything from
`//` to the end of that line is a *comment*: `rustc` ignores it. Use
`//` to leave a note for human readers, or to temporarily disable a
line of code by prefixing it with `//` so that line stops being part of
the compiled program.

## Mental Model Delta

- Before: "Every character in my `.rs` file matters to `rustc`. If I
  want to write a note for myself, I have nowhere to put it that the
  compiler won't choke on."
- After: "`//` is a *cut here* mark for the compiler. From `//` to the
  end of that physical line, `rustc` acts as if the text isn't there.
  The same mark works in three places on the same rule: at the start of
  its own line, after some code on a line that mixes code and a
  trailing comment, and in front of a whole statement to remove that
  statement from the build."

## Prerequisites

- Installed concepts:
  - From lesson 001 (`001-rustc-compile-and-run`): you can save a
    `.rs` file, run `rustc file.rs`, and run the resulting executable
    with `./name`.
  - From lesson 002 (`002-fn-main-entry-point`): the body inside
    `fn main() { ... }` is the code that runs when you launch the
    executable.
  - From lesson 004 (`004-statements-in-order`): the body of `fn main`
    is a sequence of `;`-terminated statements that run top to bottom
    in source order. Removing a statement removes its effect from the
    output.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create
`010-line-comments.rs` containing exactly:

```rust
fn main() {
    println!("first");
    // this line is a comment; rustc ignores it
    println!("second"); // trailing comment
    // println!("third");
}
```

There are five lines inside the body of `fn main`. Three of them use
`//` in three different ways:

- the third line is a free-standing comment on its own line;
- the fourth line ends with `// trailing comment` after the
  `;`-terminated `println!`;
- the fifth line is a whole `println!("third");` statement that has
  been commented out by prefixing it with `//`.

Compile and run:

```console
$ rustc 010-line-comments.rs
$ ./010-line-comments
first
second
```

Two lines of output, not three. The free-standing comment line
produced nothing because `rustc` ignored it. The trailing
`// trailing comment` did not affect the compile; the `println!`
before the `//` still ran. The third `println!` is gone from the
program because `//` at the start of that line told `rustc` to ignore
the entire `println!("third");` text. That dropped statement is the
load-bearing observation: a commented-out `;`-terminated statement
no longer participates in the source-order sequence from lesson 004,
so the output drops a line.

## What Changed

- You can put notes for human readers anywhere in a Rust source file,
  by starting the note with `//` and finishing it at the end of the
  line.
- You can leave a comment after code on the same line; only the part
  from `//` onward is ignored, so the code before it still compiles
  and runs.
- You can disable a single statement during testing or debugging by
  prefixing it with `//`; the compiled program then behaves as if
  that statement were not in the file at all.
- You have a name for what `//` introduces: the corpus calls it a
  *comment*.

## Check Yourself

You write `notes.rs` containing:

```rust
fn main() {
    println!("alpha");
    // println!("beta");
    println!("gamma"); // a label
    println!("delta");
}
```

You run `rustc notes.rs` and then `./notes`.

- How many lines does the executable print?
- What are they, in order?

(Answers: three lines; `alpha`, `gamma`, `delta`. The `// println!("beta");`
line is a commented-out statement and does not run. The trailing
`// a label` after `println!("gamma");` is ignored by `rustc` but does
not stop the `println!` before it from running.)

## What To Ignore For Now

This lesson installs only the rule: `//` makes `rustc` ignore from
`//` to the end of the line. Each of the following is real and will
be taught later, but is *not* part of this move:

- *Documentation comments* `///` and `//!`. The Book mentions these
  and defers to its Chapter 14; we do the same.
- *Block comments* `/* ... */`. The Rust Reference covers them; we
  use only `//` here.
- Inline-attribute syntax `#[...]` and `#![...]`. These start with
  `#`, are not comments, and are not used in this lesson.
- All previously-deferred items from earlier lessons: `mut`,
  shadowing, types, type annotations, constants, `&mut`, the broader
  format-string DSL, `cargo`, function parameters, function return
  values, the full statement-vs-expression distinction, modules,
  traits, generics, lifetimes, recursion.

## Evidence

### Sources

- `output/docs/rust/book/ch03-04-comments.md`. Two direct quotes are
  load-bearing here:
  - Lines 4-7: "programmers leave *comments* in their source code
    that the compiler will ignore but that people reading the source
    code may find useful." This licenses the lesson's framing of `//`
    as a marker for text that `rustc` ignores.
  - Lines 18-20: "In Rust, the idiomatic comment style starts a
    comment with two slashes, and the comment continues until the end
    of the line." This is the rule the lesson installs: from `//` to
    end of line.
  The same chapter shows two of the three uses the lesson exercises:
  a comment on its own line above a `let` (lines 47-50) and a
  trailing comment after a `let` (lines 36-39). The third use,
  commenting out a whole `;`-terminated statement, is the same rule
  applied to a `println!` line; the load-bearing source-order
  observation for that case is in lesson 004.
  Calibration: the same chapter also mentions a separate
  *documentation comments* variant, deferred to Chapter 14. This
  lesson defers documentation comments and block comments under
  `## What To Ignore For Now`.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/010-line-comments.rs`.

Probe transcript, run in a clean temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
010-line-comments.rs
--- cat 010-line-comments.rs ---
fn main() {
    println!("first");
    // this line is a comment; rustc ignores it
    println!("second"); // trailing comment
    // println!("third");
}
--- rustc 010-line-comments.rs ---
exit=0
--- ls after compile ---
010-line-comments
010-line-comments.rs
--- ./010-line-comments ---
first
second
exit=0
```

Notes from the transcript:

- `rustc 010-line-comments.rs` exited 0 and was silent. The
  trailing `// trailing comment` after `println!("second");` did not
  produce a syntax error, confirming that the trailing-comment use of
  `//` is accepted by `rustc`.
- `./010-line-comments` printed exactly two lines: `first` and
  `second`. The third `println!`, which would have printed `third`,
  is commented out and did not run. This is the load-bearing
  observation that ties this lesson to lesson 004's source-order
  rule: a commented-out statement disappears from the sequence, so
  the output drops a line.
- The executable was produced as a separate file alongside the
  source, consistent with lesson 001's compile-then-run two-step.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — the `rustc file.rs` then
  `./name` workflow used by the probe.
- `002-fn-main-entry-point` (accepted) — the body inside `fn main()
  { ... }` is what runs when the executable is launched.
- `004-statements-in-order` (accepted) — load-bearing for the
  "commented-out statement does not run" observation. The body of
  `fn main` is a sequence of `;`-terminated statements that run in
  source order; removing one (by prefixing with `//`) removes its
  effect from the output, which is exactly what the probe's missing
  third line shows.
