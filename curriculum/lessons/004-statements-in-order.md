---
id: 004-statements-in-order
move: "write more than one `;`-terminated line in `fn main` and observe that the executable runs them in source order"
main_concept: "the body of `fn main` is a sequence of statements (lines ending in `;`); when the executable runs, the statements execute in the order they appear in the source, top to bottom"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - 005-let-binding
  - future "expression vs statement" moves
  - future "comments" moves
  - future function-call moves
sources:
  - output/docs/rust/book/ch01-02-hello-world.md
  - output/docs/rust/book/ch03-03-how-functions-work.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/004-statements-in-order.rs
status: accepted
---

# Statements in `fn main` run in source order

## The Move

Put more than one `;`-terminated line inside `fn main`. Compile and
run. The executable prints output from each line, in the order the
lines appear in your source. Reorder the lines, recompile, rerun: the
output reorders to match.

## Mental Model Delta

- Before: "I have a `fn main` body with one `println!` line in it. I
  do not know what happens if I add more lines, or in what order they
  would run."
- After: "The body of `fn main` is a *sequence* of lines. Each line
  that ends with `;` is what the corpus calls a *statement*: an
  instruction that does something. When the executable runs, the
  statements execute top to bottom, in exactly the order they appear
  in the source. If I swap two statements in the source and recompile,
  the program's output reorders to match."

## Prerequisites

- Installed concepts:
  - From lesson 001 (`001-rustc-compile-and-run`): you can save a
    tiny Rust program to a `.rs` file, compile it with `rustc file.rs`,
    and run the resulting executable with `./name`. `rustc` is silent
    on success.
  - From lesson 002 (`002-fn-main-entry-point`): the body inside
    `fn main() { ... }` is the code that runs when you launch the
    executable.
  - Lesson 003 (`003-read-rustc-diagnostic`) is already installed but
    not load-bearing here: this lesson never produces a compile error,
    so you do not need to read a diagnostic.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `main.rs`
containing exactly:

```rust
fn main() {
    println!("first line");
    println!("second line");
    println!("third line");
}
```

There are three `println!` lines, each ending in `;`. The corpus
calls each of those lines a *statement*; for now, a statement is a
line of Rust that does something and ends in `;`.

Compile and run:

```console
$ rustc main.rs
$ ./main
first line
second line
third line
```

Three lines of output, in source order: `first line`, then
`second line`, then `third line`.

Now the contrast. Before you edit the file, *predict*: if you swap
the second and third `println!` lines in the source, what will the
executable print after you recompile and rerun?

Edit `main.rs` so that the order of the second and third statements
is swapped:

```rust
fn main() {
    println!("first line");
    println!("third line");
    println!("second line");
}
```

Recompile and rerun:

```console
$ rustc main.rs
$ ./main
first line
third line
second line
```

The output reordered to match the new source order. That is the
whole concept: top to bottom, in order, no surprises.

## What Changed

- You can extend a `fn main` body to more than one line just by
  adding more `;`-terminated lines.
- You have a name for those lines: the corpus calls each `;`-terminated
  line in a function body a *statement*. A statement is "a line of
  Rust that does something" for now.
- You know the execution order: top to bottom, in source order. Move
  a line up or down and the program's behavior shifts to match.
- You know what the trailing `;` is doing in a structural sense: it
  marks the end of one statement so the next one can begin. Lessons
  001-003 deferred this; you no longer have to treat `;` as
  unexplained punctuation.

## Check Yourself

You write `greet.rs` containing:

```rust
fn main() {
    println!("good");
    println!("morning");
    println!("friend");
}
```

You run `rustc greet.rs` and then `./greet`.

- How many lines does the executable print?
- What are they, in order?
- If you swap the first and last `println!` lines in the source and
  recompile, what does `./greet` print, in order?

(Answers: three lines; `good`, `morning`, `friend` in that order;
after the swap, `friend`, `morning`, `good`.)

## What To Ignore For Now

This lesson installs only one idea: statements in `fn main` run top
to bottom in source order. Each of the following is real and will be
taught later, but is *not* part of this move:

- The full *statement vs expression* distinction. The corpus also
  defines *expressions* and notes that "function bodies are made up
  of a series of statements optionally ending in an expression."
  This lesson uses only the "series of statements" half. Expressions
  are a later move. (covered in lesson 024.)
- Why some Rust code lines do *not* end in `;`. You will sometimes
  see Rust code where the last line of a body has no trailing `;`
  (a final-expression "return" value), or `if`-style forms whose
  inner lines lack `;`. Those cases are deferred. For now, every
  statement we write ends in `;`, and the rule "statements run in
  source order" is enough. (covered in lessons 024 and 025.)
- `let`. Naming a value with `let` is the next lesson (cycle 005). (covered in lesson 005.)
- Defining your own functions other than `main`. Still deferred from
  lesson 002. (covered in lesson 008.)
- Calling functions. The Book's chapter 3 example uses a call to a
  user-defined `another_function()` to make the same "lines in order"
  point; this lesson uses only multiple `println!` invocations
  instead. (covered in lesson 008.)
- `println!` macro semantics, the `!`, the string literal, and any
  format-string placeholders. Still deferred from lesson 001. (`println!` positional formatting covered in lesson 011; string-literal type `&str` covered incidentally in lesson 055; the `!`/macro concept remains deferred.)
- Comments (`//`). Not used in this lesson and not introduced. (line comments covered in lesson 010; block comments in lesson 018.)
- `cargo`. Still deferred; we are using `rustc` directly. (covered starting in lesson 032.)
- Concurrency or parallel execution. The "lines run in order" claim
  is about a single straight-line program. We are not writing
  any code that asks for parallel execution. (out of scope for this run.)

## Evidence

### Sources

- `output/docs/rust/book/ch01-02-hello-world.md`, "The Anatomy of a
  Rust Program" walkthrough. Direct quote (lines 144-146): "we end
  the line with a semicolon (`;`), which indicates that this
  expression is over, and the next one is ready to begin. Most lines
  of Rust code end with a semicolon." This is the corpus's structural
  description of what `;` does between lines, and it is what licences
  the lesson's framing of `;` as a per-statement terminator that
  separates one statement from the next. The Book here calls the
  thing before the `;` an "expression" rather than a "statement";
  see the calibration note below.
- `output/docs/rust/book/ch03-03-how-functions-work.md`. Two direct
  quotes are load-bearing here:
  - Line 51, on a `fn main` body that contains a `println!` followed
    by a call to `another_function`: "The lines execute in the order
    in which they appear in the `main` function." This is the corpus
    statement of the source-order execution rule that this lesson
    teaches.
  - Lines 144-145, the bare definition: "*Statements* are
    instructions that perform some action and do not return a value."
    This is the corpus definition of *statement* the lesson uses as
    its informal "a line of Rust that does something and ends in `;`."
  Calibration: the same chapter goes on to also define *expressions*
  ("Expressions evaluate to a resultant value") and notes that
  "function bodies are made up of a series of statements optionally
  ending in an expression." Lesson 004 deliberately uses only the
  "series of statements" half and defers the full statement-vs-expression
  distinction. The mismatch with the ch01-02 wording (which calls the
  pre-`;` thing an "expression") is exactly the nuance being deferred:
  for the three `println!(...);` lines this lesson writes, both
  framings agree that source-order execution holds, so the deferral
  is safe.
- The local probe (both transcripts), captured below.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/004-statements-in-order.rs`.
The committed file is the *original* ordering. The swapped ordering
is documented as a second run inside this Evidence section, not as a
separate `.rs` file.

Probe transcript, both runs in the same temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64

=== RUN 1: original ordering ===
--- ls before compile ---
main.rs
--- cat main.rs ---
fn main() {
    println!("first line");
    println!("second line");
    println!("third line");
}
--- rustc main.rs ---
exit=0
--- ls after compile ---
main
main.rs
--- ./main ---
first line
second line
third line
exit=0

=== RUN 2: swap second and third statements ===
--- cat main.rs ---
fn main() {
    println!("first line");
    println!("third line");
    println!("second line");
}
--- rustc main.rs ---
exit=0
--- ls after compile ---
main
main.rs
--- ./main ---
first line
third line
second line
exit=0
```

Notes from the transcript:

- Run 1 (original ordering): `rustc` exits 0, silent. `./main`
  prints three lines: `first line`, `second line`, `third line`,
  matching the source order.
- Run 2 (swap second and third statements): `rustc` exits 0, silent
  again — the swap is not a syntax change, just a reorder. `./main`
  prints `first line`, `third line`, `second line`. The output
  reordering exactly tracks the source reordering. This is the
  load-bearing observation for the lesson's main concept.
- Both runs use the same temp directory. The `main` executable is
  rebuilt by the second `rustc main.rs` and overwrites the first
  one, consistent with lesson 001's compile-then-run two-step.
- Only the original-ordering source is committed under
  `observations/`; the swapped version exists only inside this
  transcript. No binaries are committed.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — the `rustc file.rs` then
  `./name` workflow used by both probe runs.
- `002-fn-main-entry-point` (accepted) — the body inside
  `fn main() { ... }` is what runs when the executable is launched;
  this lesson is about what happens when that body has more than one
  line.
