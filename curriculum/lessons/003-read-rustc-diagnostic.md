---
id: 003-read-rustc-diagnostic
move: "read a single rustc diagnostic by locating the headline, the --> location, the source excerpt with caret, and any help/note lines"
main_concept: "rustc diagnostics have a consistent structure; the optional E#### code and --explain trailer vary, but the headline, the --> location, and the source excerpt with caret are always there"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future let-binding moves
  - future "fix the diagnostic" moves
  - future cargo moves
sources:
  - output/docs/rust/rustc/command-line-arguments.md
  - output/docs/rust/error_codes/index.md
  - output/docs/rust/error_codes/E0601.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/003-read-rustc-diagnostic.rs
status: accepted
---

# Read a `rustc` diagnostic by its parts

## The Move

When `rustc` rejects your program, do not read the output as a wall of
text. Look for four labelled parts: the **headline**, the **location**,
the **source excerpt with caret**, and any **help / note** lines. Once
you can point at each of those, you can read any `rustc` diagnostic
without panicking, even when the message is unfamiliar.

## Mental Model Delta

- Before: "When `rustc` prints an error, it prints a confusing block
  of text. Sometimes there is a code like `E0601`, sometimes there
  isn't. Sometimes a fix is suggested, sometimes not. I have to guess
  what to look at first."
- After: "Every `rustc` error has the same skeleton. The first line is
  the headline. A line starting with `-->` says exactly which file,
  line, and column. A bordered block underneath shows the offending
  code with `^` characters underlining the span. Lines starting with
  `help:` or `= note:` add context or suggest a fix. The `[E####]`
  code and the `For more information ... rustc --explain` trailer are
  optional extras that show up on some errors and not others."

## Prerequisites

- Installed concepts:
  - From lesson 001 (`001-rustc-compile-and-run`): you can save a
    tiny Rust program to a `.rs` file and run `rustc file.rs`. You
    know that `rustc` is silent on success and produces an executable
    next to the source.
  - From lesson 002 (`002-fn-main-entry-point`): you have already
    seen one rustc diagnostic, the `error[E0601]` one that appears
    when `main` is missing. Its full transcript is in lesson 002's
    `## Evidence` section. This lesson uses that transcript as the
    second example without re-capturing it.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `prntln.rs`
containing exactly this (note the deliberate typo `prntln` for
`println`):

```rust
fn main() {
    prntln!("hello from rustc");
}
```

Compile:

```console
$ rustc prntln.rs
error: cannot find macro `prntln` in this scope
 --> prntln.rs:2:5
  |
2 |     prntln!("hello from rustc");
  |     ^^^^^^
  |
 --> /rustc/<hash>/library/std/src/macros.rs:138:0
  |
  = note: similarly named macro `println` defined here
help: a macro with a similar name exists
  |
2 |     println!("hello from rustc");
  |       +

error: aborting due to 1 previous error
```

Now walk through it by parts. Do not try to understand every word;
just point at each part in turn.

1. **Headline** — the first line:

   ```text
   error: cannot find macro `prntln` in this scope
   ```

   It starts with the word `error:` and a short message. There is no
   `[E####]` code on this one. That is allowed; not every error has
   a code.

2. **Location** — the line that starts with `-->`:

   ```text
    --> prntln.rs:2:5
   ```

   Read this as `file:line:column`. The error is in `prntln.rs`, on
   line 2, starting at column 5. That is exactly where `rustc` is
   pointing. If a diagnostic shows more than one `-->` line, the first
   one is your bug location; later `-->` lines belong to `note:`
   context and point at related code somewhere else (here, where the
   similarly-named `println` macro is defined inside the standard
   library).

3. **Source excerpt with caret** — the bordered block:

   ```text
     |
   2 |     prntln!("hello from rustc");
     |     ^^^^^^
   ```

   `rustc` reprints the offending line of your file (with its line
   number on the left) and underlines the exact span it is complaining
   about with `^` characters. Here, the carets sit under `prntln`.
   That span *is* the problem.

4. **Help and note** — lines that start with `help:` or `= note:`:

   ```text
     = note: similarly named macro `println` defined here
   help: a macro with a similar name exists
     |
   2 |     println!("hello from rustc");
     |       +
   ```

   `note:` adds context (`println` exists nearby). `help:` proposes
   a concrete change, often with a second source excerpt showing the
   suggested edit. You do not have to follow the suggestion; you
   just have to recognize what those lines are for.

There is one more line in the output:

```text
error: aborting due to 1 previous error
```

This is a trailer that says how many errors fired. Skip it for now.

Now compare what you just read with the diagnostic from lesson 002
(the `fn start` case, captured in lesson 002's `## Evidence`):

```text
error[E0601]: `main` function not found in crate `hello`
 --> hello.rs:3:2
  |
3 | }
  |  ^ consider adding a `main` function to `hello.rs`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0601`.
```

The same parts are there:

- **Headline**: `error[E0601]: \`main\` function not found in crate \`hello\``.
  This one has an `[E0601]` code wedged into the headline.
- **Location**: ` --> hello.rs:3:2`.
- **Source excerpt with caret**: the bordered block under it, with
  `^` pointing at the column.
- **Help / note**: this one folds the suggestion (`consider adding a
  \`main\` function to \`hello.rs\``) onto the same line as the caret
  rather than on a separate `help:` line. Same role, different layout.

One extra line appears that the `prntln` case did not have:

```text
For more information about this error, try `rustc --explain E0601`.
```

This trailer only appears for errors that *have* an `E####` code.
Errors without one (like `prntln`) do not get a `--explain` trailer.

That is the structural point. Headline, location, and source excerpt
with caret are always there. The `E####` code and the `--explain`
trailer are optional extras.

## What Changed

- You can take any `rustc` error block and point at its headline,
  its location, its source excerpt with caret, and any help/note
  lines, even if you have never seen the specific error before.
- You know that the `[E####]` part of the headline is *optional*.
  Some errors carry a code; some do not.
- You know that the `For more information ... rustc --explain ECODE`
  line only appears when there is an `[E####]` code to explain.
- You have a fixed reading order to fall back on, which means an
  unfamiliar diagnostic is no longer a wall of text.

## Check Yourself

Suppose `rustc foo.rs` prints the following (do not actually run this;
just read it):

```text
error[E0425]: cannot find value `x` in this scope
 --> foo.rs:5:20
  |
5 |     println!("{}", x);
  |                    ^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

- What is the **headline**? Does it carry an `E####` code?
- Which file, line, and column does the **location** point at?
- Which token in the source excerpt is underlined by the caret?
- Is there a `help:` line, a `= note:` line, both, or neither?
- Does this error have a `--explain` trailer?

(Answers: headline is `error[E0425]: cannot find value \`x\` in this scope`,
yes it carries the code `E0425`; location is `foo.rs`, line 5, column 20;
the caret underlines `x`; neither `help:` nor `= note:` appears here, only
an inline `not found in this scope` annotation next to the caret; yes,
the `--explain E0425` trailer is present because there is an `E####` code.)

## What To Ignore For Now

This lesson installs only the *map* of a rustc diagnostic. Each of
the following is real and will be taught (or used) in a later lesson,
but is *not* part of this move:

- What a *macro* is. The `prntln` diagnostic uses the word "macro";
  read it as "the thing called with `name!(...)`" and move on.
  Lesson 001 already deferred this.
- What "in this scope" means. *Scope* is a concept for a later lesson. (covered in lesson 068.)
- How to actually *fix* the error. The lesson teaches **reading**;
  fixing is the obvious follow-on, but not the main concept here. (no separate "how to fix" lesson is planned; fixing is taught operationally throughout — every contrast probe in subsequent lessons shows a fix.)
- Type-error diagnostics (`E0277`, trait bounds, lifetime errors,
  and friends). They follow the same skeleton, but the messages and
  notes are much harder to read. Out of scope for now. (no exhaustive type-error walkthrough is planned; specific type errors are introduced operationally in the lessons that surface them.)
- Multi-error diagnostics (several errors in one run, "previous error"
  cascades). This lesson sticks to single errors. (out of scope for this run.)
- *Warnings* (`warning:` instead of `error:`) and the `rustc` lint
  system. Same skeleton, different category. Deferred.
- ANSI color codes / terminal coloring of diagnostics. Real terminals
  paint these in color; this lesson ignores color and works from the
  plain text. (out of scope; this run treats output as plain text.)
- The JSON diagnostic format (`--error-format=json`). A separate
  machine-readable view, not for human reading. (out of scope; not for human reading.)
- Using `rustc --explain ECODE` interactively. Note only that it
  exists for errors that carry an `E####` code; using it is a later
  move.
- The exact wording of any specific diagnostic. Different `rustc`
  versions print slightly different text. The *structure* persists.
- `cargo`'s richer diagnostic surfacing. Still deferred; we are
  reading raw `rustc` output. (cargo's surfacing is operationally observed in lessons 032, 064, 065; no separate diagnostic-surfacing-vs-rustc lesson is planned.)
- `println!`, the `!` after a name, the trailing `;`, and the string
  literal in `"hello from rustc"`. Still deferred from lessons
  001/002. (trailing `;` covered in lesson 004; `println!` operational use covered in lesson 011; string-literal type `&str` covered incidentally in lesson 055; the `!`/macro concept remains deferred.)

## Evidence

### Sources

- `output/docs/rust/rustc/command-line-arguments.md`, the section
  `## --explain: provide a detailed explanation of an error message`.
  Direct quote: "Each error of `rustc`'s comes with an error code;
  this will print out a longer explanation of a given error." This
  is the corpus claim that error codes exist and that `--explain`
  acts on them. Calibration: the captured `prntln` probe shows an
  `error:` *without* an `[E####]` code, so this corpus sentence is
  mildly idealized — most errors that get a dedicated explainer page
  carry a code, but parse-style and macro-resolution errors are
  often emitted without one. The lesson presents the `E####` code
  and the `--explain` trailer as *optional* parts of the diagnostic
  to match observed behavior.
- `output/docs/rust/error_codes/index.md`. Direct quote: "This page
  lists all the error codes emitted by the Rust compiler." Confirms
  that `E####` codes are a defined, enumerable set with one explainer
  page per code (the page lists hundreds of `E####` entries, each
  linking to its own file).
- `output/docs/rust/error_codes/E0601.md`. The canonical explainer
  for the `E0601` error that lesson 002 reproduced. Direct quote:
  "No `main` function was found in a binary crate. To fix this
  error, add a `main` function". This lesson uses E0601 as the
  *coded* example to contrast with the *uncoded* `prntln` example.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/003-read-rustc-diagnostic.rs`.
The committed file is the *broken* version (typo `prntln`), so that
re-running `rustc` on it reproduces the diagnostic this lesson walks
through. The header comment in the file documents the typo and the
working fix.

Probe transcript, run in a clean temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls ---
prntln.rs
--- rustc prntln.rs (capturing stderr) ---
error: cannot find macro `prntln` in this scope
 --> prntln.rs:2:5
  |
2 |     prntln!("hello from rustc");
  |     ^^^^^^
  |
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/macros.rs:138:0
  |
  = note: similarly named macro `println` defined here
help: a macro with a similar name exists
  |
2 |     println!("hello from rustc");
  |       +

error: aborting due to 1 previous error

exit=1
--- ls after ---
prntln.rs
```

Notes from the transcript:

- The headline `error: cannot find macro \`prntln\` in this scope`
  has *no* `[E####]` code. This is the load-bearing observation for
  the lesson's claim that the code is optional.
- The `-->` location `prntln.rs:2:5` points at line 2, column 5 of
  the source.
- The bordered block reprints line 2 of the source and underlines
  `prntln` (six `^` characters under a six-character identifier).
- A second `-->` line points into `library/std/src/macros.rs:138:0`,
  which is where the *similarly named* `println` macro is defined.
  This is the diagnostic showing rustc's "did you mean?" context;
  the lesson treats it as an extra `note:` reference and does not
  unpack it further.
- One `= note:` line (`similarly named macro \`println\` defined
  here`) and one `help:` line (`a macro with a similar name exists`)
  follow, with the help block including a small source excerpt that
  shows the proposed fix (`println!`).
- Trailer: `error: aborting due to 1 previous error`. No
  `For more information ... rustc --explain` line follows, because
  there is no `E####` code to explain. This is the contrast with
  lesson 002's E0601 transcript.
- Exit code: 1. No executable was produced (`ls after` shows only
  `prntln.rs`).

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — installs the
  rustc-compile-and-run workflow this lesson assumes (`.rs` source
  compiled with `rustc file.rs`, silent on success, executable next
  to source).
- `002-fn-main-entry-point` (accepted) — captured the second
  diagnostic this lesson references, the `error[E0601]: \`main\`
  function not found in crate \`hello\`` block. Its full transcript
  lives in that lesson's `## Evidence` section. This lesson cites
  it by id rather than re-capturing it; the load-bearing claim from
  lesson 002 is just that an `error[E####]` headline plus a
  `For more information ... rustc --explain ECODE` trailer is
  observed in the wild on the same machine and same `rustc` version.
