---
id: 070-rustc-explain
status: accepted
evidence: ../evidence/070-rustc-explain.md
---

# Follow the `rustc --explain ECODE` trailer

## The Move

When a `rustc` diagnostic ends with the trailer

```text
For more information about this error, try `rustc --explain E0425`.
```

run that command exactly. `rustc --explain E0425` prints a multi-paragraph
explainer for the error code: a one-line summary, examples that *cause*
the error, and at least one example that *fixes* it. The explainer is
the same content as the corpus page
`output/docs/rust/error_codes/E0425.md`, so once you have run it you
have already met the page.

## Mental Model Delta

- Before: "Lesson 003 said the `For more information ... rustc
  --explain ECODE` trailer was an optional extra at the bottom of
  some diagnostics. I have been ignoring it."
- After: "That trailer is a *runnable instruction*. Run the command
  and `rustc` prints the canonical longer-form explainer for that
  error code: what it means, what causes it, and at least one fix.
  The text is the same as `output/docs/rust/error_codes/E####.md` —
  `rustc` and the corpus draw from one source. The command is a
  documentation lookup; no source file is needed and nothing is
  compiled."

## Prerequisites

- Installed concepts:
  - Lesson 001: you can invoke `rustc` from a terminal.
  - Lesson 003 (load-bearing): you have already seen the trailer
    line `For more information about this error, try \`rustc
    --explain ECODE\`.` at the bottom of a coded error block.
    Lesson 003 named this trailer and deferred its interactive use;
    this lesson closes that loop.
  - Lesson 005: supplies the canonical E0425 case
    (`error[E0425]: cannot find value \`x\` in this scope` plus its
    `--explain E0425` trailer, captured in lesson 005's appendix).
- Ordinary computer-use assumptions: terminal, `rustc` on `PATH`.
  No network access required — `--explain` is a local lookup
  against data baked into the `rustc` binary.

## Try It

Open any terminal, in any directory (an empty scratch dir works fine),
and run:

```console
$ rustc --explain E0425
```

Notice what does *not* happen: there is no `file.rs`, no compile,
and no executable produced. `rustc` prints text and exits 0. Then
read the printed text. The output begins:

````text
An unresolved name was used.

Erroneous code examples:

```
something_that_doesnt_exist::foo;
// error: unresolved name `something_that_doesnt_exist::foo`
...
```
````

Three parts to observe:

1. **Headline summary** — the first line states what the error
   means: `An unresolved name was used.`
2. **Erroneous code examples** — under `Erroneous code examples:`,
   shapes that trigger the error. The third one,
   `let x = unknown_variable;`, is the pattern lesson 005 broke
   deliberately.
3. **Fix examples** — under `Please verify ... Example:` and the
   `Or:` connectives, shapes that compile. The third fix,
   `let unknown_variable = 12u32; let x = unknown_variable; // ok!`,
   is the fix shape lesson 005 used.

Now open `output/docs/rust/error_codes/E0425.md`. The prose, the
erroneous examples, the fix examples, and the `Or:` connectives are
the same. The corpus page adds a small markdown header and wraps
each inline code block with `#![allow(unused)]` / `fn main()` for
its automated compile-tests, but every paragraph and every example
line `rustc` printed is in the file. Diff captured in the appendix.

That is the equivalence: `rustc --explain ECODE` and
`output/docs/rust/error_codes/ECODE.md` are two surfaces of one
explainer.

For another shape, run it on lesson 002's code:

````console
$ rustc --explain E0601
No `main` function was found in a binary crate.

To fix this error, add a `main` function:

```
fn main() {
    // Your program will start here.
    println!("Hello world!");
}
```
...
````

Same pattern, shorter content: headline summary, then a fix
example. Full transcript in the appendix.

## What Changed

- The `For more information ... rustc --explain ECODE` line in a
  coded diagnostic is now a step you can take, not just decoration.
- Running `rustc --explain ECODE` on any `E####` you encounter
  produces a multi-paragraph explainer with the shape "headline
  summary + erroneous examples + fix examples."
- That explainer is the same content as
  `output/docs/rust/error_codes/ECODE.md` in the corpus, so the
  command is also a way to navigate to a known artifact.
- `--explain` works without any source file or project; it is a
  documentation lookup, not a compilation.

## Check Yourself

You see the following at the bottom of a `rustc` block:

```text
For more information about this error, try `rustc --explain E0601`.
```

(a) What command do you type next?

(b) Do you need a `.rs` file open to run that command?

(c) Roughly what shape does the printed output have? (Two or three
parts — name them.)

(d) Where in the corpus is the same content?

(Answers: (a) `rustc --explain E0601`. (b) No; `--explain` is a
documentation lookup, no source needed. (c) A one-line headline
summary (`No \`main\` function was found in a binary crate.`),
followed by a fix example shown in a code block. The fuller E0425
case adds a separate `Erroneous code examples:` section before the
fix examples. (d) `output/docs/rust/error_codes/E0601.md`.)

## What To Ignore For Now

- Other `rustc` flags (`--emit`, `--edition`, `-C`, `-W`, `-D`).
  `--version` is fine as a sanity-check, not a concept here.
- The full enumeration of `E####` codes. The corpus index
  `output/docs/rust/error_codes/index.md` lists them all.
- The *content* of the E0425 explainer as a Rust lesson; lesson 005
  already taught the let-binding fix. Today installs only "follow
  the trailer to see the canonical explanation."
- `cargo`'s wrapping of rustc diagnostics. Same trailer; same fix.
- The JSON diagnostic format (`--error-format=json`) and the HTML
  `error-index.html` variant. Same prose, different surface.
- `--explain` for *lints*. Warnings are emitted by lint name
  (lesson 069), not by an `E####` code.

## Evidence

See `../evidence/070-rustc-explain.md`.
