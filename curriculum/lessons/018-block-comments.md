---
id: 018-block-comments
move: "write a block comment with `/* ... */` that rustc ignores; the comment can span multiple lines or sit inline within a single line of code"
main_concept: "a block comment opens with `/*` and runs until the matching `*/`; everything between is ignored by rustc; unlike `//` line comments, a block comment does not have to fit on one physical line and can also sit inline or trailing within a line that also contains code; block comments are the second non-doc comment form alongside lesson 010's `//` line comments"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 004-statements-in-order
  - 010-line-comments
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "doc comments ///" moves
  - future "doc block comments /** */" moves
  - future "nested block comments" moves
sources:
  - output/docs/rust/reference/comments.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/018-block-comments.rs
status: accepted
---

# Block comments with `/* ... */`

## The Move

Type `/*` in a Rust source file. Type `*/` later. Everything between
is a *block comment*: `rustc` ignores it. Unlike a `//` line comment,
a block comment does not stop at end of line — it runs until the
matching `*/`, which can be on the same line or many lines below.
You can also drop a small `/* ... */` *inside* a line, between code.

## Mental Model Delta

- Before: "Rust has one comment form, `//`, which ends at the end of
  the physical line. A multi-line note means prefixing every line,
  and a comment in the middle of a code line ends that line."
- After: "Rust has two non-doc comment forms. `//` ends at end of
  line. `/* ... */` ends at the matching `*/`. Block comments are
  range-delimited, not line-delimited, so I can wrap many lines or
  drop a tiny inline note between tokens. Both forms install the
  same rule — `rustc` treats the marked text as if it were not there."

## Prerequisites

- Installed concepts:
  - From lesson 001: `rustc file.rs` produces an executable next to
    the source; `./name` runs it.
  - From lesson 002: the body inside `fn main() { ... }` is the code
    that runs when the executable is launched.
  - From lesson 004: the body of `fn main` is a sequence of
    `;`-terminated statements that run top to bottom.
  - From lesson 010 (load-bearing): `//` makes `rustc` ignore from
    `//` to end of line. Block comments install the same "rustc
    ignores this text" idea, with a different boundary: `/*` opens,
    `*/` closes.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create
`018-block-comments.rs` containing exactly:

```rust
fn main() {
    /* this is a block comment;
       it spans multiple lines until the closing marker */
    println!("hello");
    /* inline */ println!("world"); /* trailing */
}
```

Three block comments inside `fn main`: the first spans two lines (a
*multi-line* block comment); `/* inline */` sits before
`println!("world");` on a single line; `/* trailing */` sits after
the same `;`-terminated `println!`.

Compile and run:

```console
$ rustc 018-block-comments.rs
$ ./018-block-comments
hello
world
```

What `rustc` saw:

- It skipped the first block comment from `/*` to the matching `*/`,
  then ran `println!("hello");`.
- On the next line `/* inline */` opens and closes before any code;
  `rustc` skipped it, then ran `println!("world");`.
- `/* trailing */` opens and closes after the `;`; `rustc` skipped it.

Source order from lesson 004 still holds: the two `println!`s are
the only real statements, and they print in that order.

## What Changed

- You can write a multi-line comment without prefixing every line
  with `//`, by wrapping it in `/* ... */`.
- You can drop a small comment *between* code on a single line.
  `//` cannot do this: it would consume the rest of the line.
- You have a name for the new form: the Reference calls it a *block
  comment* and lists it alongside `//` as one of two non-doc
  comment forms.

## Check Yourself

You write `notes.rs`:

```rust
fn main() {
    println!("alpha"); /* tag */
    /* println!("beta"); */
    println!("gamma");
}
```

You run `rustc notes.rs` then `./notes`. How many lines does it
print, and which ones?

(Answers: two lines; `alpha`, then `gamma`. `/* tag */` is ignored
but does not stop the `println!("alpha");` before it. The middle
line wraps the entire `println!("beta");` in `/* ... */`, so `rustc`
skips it like a commented-out line from lesson 010, and that output
line drops out.)

## What To Ignore For Now

This lesson installs only one rule: `/* ... */` makes `rustc` ignore
everything between the markers. The following are real and will be
taught later, but are *not* part of this move:

- *Documentation comments* `///`, `//!`, `/** ... */`, `/*! ... */`.
  Already deferred from lesson 010; remain deferred. The Reference
  describes them as a special syntax for `doc` attributes.
- *Nested block comments*. The Reference says "Nested block comments
  are supported," so `/* outer /* inner */ outer */` is valid Rust —
  the closing `*/` matches the most recent unmatched `/*`. The probe
  uses only flat block comments; nested cases are deferred.
- Inline-attribute syntax `#[...]` and `#![...]`. These start with
  `#`, are not comments, and are not used here.
- All previously-deferred items from earlier lessons.

## Evidence

### Sources

- `output/docs/rust/reference/comments.md`, section
  `## Non-doc comments` (lines 59–66). The load-bearing quote
  (line 61):

  > "Comments follow the general C++ style of line (`//`) and block
  > (`/* ... */`) comment forms. Nested block comments are supported."

  This is the corpus statement that licenses two claims in the
  lesson: (a) Rust has two non-doc comment forms, line and block;
  (b) the block form is written `/* ... */`. Line 65 also says:
  "Non-doc comments are interpreted as a form of whitespace." This
  is the Reference's way of saying `rustc` ignores them — the same
  rule lesson 010 installed for `//`, now extended to `/* ... */`.

  Calibration: the same file's `## Doc comments` section (lines
  67–98) describes `///`, `//!`, `/** ... */`, and `/*! ... */` as a
  separate category that is "interpreted as a special syntax for
  `doc` attributes." This lesson defers all of those under
  `## What To Ignore For Now` and uses only the non-doc form.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/018-block-comments.rs`.

Probe transcript, run in a clean temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
018-block-comments.rs
--- cat 018-block-comments.rs ---
fn main() {
    /* this is a block comment;
       it spans multiple lines until the closing marker */
    println!("hello");
    /* inline */ println!("world"); /* trailing */
}
--- rustc 018-block-comments.rs ---
exit=0
--- ls after compile ---
018-block-comments
018-block-comments.rs
--- ./018-block-comments ---
hello
world
exit=0
```

Notes from the transcript:

- `rustc 018-block-comments.rs` exited 0 and was silent. None of the
  three block comments produced a syntax error: a multi-line block
  comment, an inline block comment between code, and a trailing
  block comment after a `;`-terminated statement are all accepted by
  `rustc`.
- `./018-block-comments` printed exactly two lines: `hello` then
  `world`. The block comments contributed nothing to the output;
  only the two `println!` statements ran. This matches the
  Reference's "form of whitespace" framing — for the purposes of
  what the executable does, the comment text might as well not
  exist.
- The executable was produced as a separate file alongside the
  source, consistent with lesson 001's compile-then-run two-step.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — the `rustc file.rs` then
  `./name` workflow used by the probe.
- `002-fn-main-entry-point` (accepted) — the body inside `fn main()
  { ... }` is what runs when the executable is launched.
- `004-statements-in-order` (accepted) — the body of `fn main` is a
  sequence of `;`-terminated statements that run top-to-bottom; the
  two `println!` statements outside the block comments print in that
  order.
- `010-line-comments` (accepted, load-bearing) — installs the
  general "rustc ignores this text" framing for the `//` form. This
  lesson reuses that framing for a second syntactic form
  (`/* ... */`) without re-deriving it. The contrast that matters
  here is the boundary rule: `//` ends at end of line, `/* ... */`
  ends at the matching `*/`, which is what makes block comments
  span multiple lines or sit inline.
