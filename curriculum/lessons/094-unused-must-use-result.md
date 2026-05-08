---
id: 094-unused-must-use-result
status: accepted
evidence: ../evidence/094-unused-must-use-result.md
---

# `unused_must_use` — discarding a `Result` is a warn-by-default lint

## The Move

Lesson 053 chained `.expect("msg")` after every `Result`-returning
call. That advice had a quiet companion: if you *don't* chain
`.expect(...)` (or do anything else with the `Result`), `rustc`
notices and prints a warning. Today's move is recognizing that
warning and naming its escape hatch.

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf);
    println!("got: {}", buf);
}
```

This is lesson 054's program with the `.expect("Failed to read line")`
call removed. `read_line(&mut buf)` still returns a `Result<usize,
io::Error>`, but nothing on line 5 binds, matches, or chains a
method onto that `Result`. The semicolon throws it away.

Compile it:

```console
$ rustc demo.rs
warning: unused `Result` that must be used
 --> demo.rs:5:5
  |
5 |     io::stdin().read_line(&mut buf);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this `Result` may be an `Err` variant, which should be handled
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
  |
5 |     let _ = io::stdin().read_line(&mut buf);
  |     +++++++

warning: 1 warning emitted

```

Read it with the lesson 003 map. **Headline**: `warning: unused
\`Result\` that must be used`. First word `warning:`, so this is
lesson 069's category: an executable still appears, `rustc` exits 0,
and `echo "hello" | ./demo` prints `got: hello`. The program *runs*.
The warning was a nudge.

The two `= note:` lines explain *why* and name the lint. First: the
discarded `Result` *might* have been an `Err` — a failure your code
never noticed. Second: the lint is `unused_must_use`, on by default
as part of the `unused` lint group.

The `help:` line names the deliberate-discard form:

```rust
let _ = io::stdin().read_line(&mut buf);
```

One-line edit, rebuild, warning gone. `let _ = ...` is the way you
tell `rustc` "I see the value and I am choosing to drop it."

## Mental Model Delta

- *Before:* "Lesson 053 said chain `.expect("msg")` after a
  `Result`-returning call. If I forget, rustc compiles silently."
- *After:* "If I call a function returning `Result<T, E>` and don't
  do anything with the value, `rustc` fires the `unused_must_use`
  warning at compile time. It's a *warning*, not an error: the
  program still compiles and runs (lesson 069's category). The
  discarded value could have been an `Err`, which is why the lint
  exists. `let _ = expr;` is the documented escape hatch: a
  deliberate discard that silences the warning."

## Prerequisites

- Installed concepts:
  - Lesson 069 (load-bearing): `warning:` is a separate category
    from `error:`; warnings do not abort the build, exit is 0, an
    executable is produced. Today's diagnostic is exactly that
    category.
  - Lesson 052 (load-bearing): `Result<T, E>` as a two-variant
    enum (`Ok(T)`, `Err(E)`). The lint's name is "unused `Result`"
    because today's discarded value is a `Result`.
  - Lesson 053 (load-bearing): `.expect("msg")` consumes a `Result`
    by panicking on `Err`. Today's program is lesson 053's chain
    *without* `.expect(...)` — that omission is what triggers the
    lint.
  - Lesson 054 (load-bearing): `io::stdin().read_line(&mut buf)`
    returns a `Result<usize, io::Error>`. The probe is lesson 054's
    program with the `.expect(...)` call removed.
  - Lessons 050, 044, 003, 011, 002, 005 (cited): `std::io::stdin()`,
    `use std::io;`, the four-part diagnostic map, `println!`,
    `fn main`, `let name = value;`.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`; plus shell-piping `echo "hello" | ./demo`
  (assumption used since lesson 054).

## Try It

Save the source above as `demo.rs` and compile with `rustc demo.rs`.
You should see the bordered warning block reproduced above, then
`warning: 1 warning emitted`. The trailer is *not*
`error: aborting due to N previous error(s)` — the word "aborting"
is absent (lesson 069's signal that the build did not abort).

`demo` is there. Run it with `echo "hello" | ./demo`; it prints
`got: hello` and exits 0. The `Result` from `read_line` was thrown
away — exactly what the warning warned about — but the call
succeeded, so the discarded `Result` was an `Ok(...)` and the
program got away with it.

*Predict.* Edit line 5 to `let _ = io::stdin().read_line(&mut buf);`
and recompile. `rustc` is silent (lesson 001 — silent on a clean
build). `let _ = ...` consumed the value, and the lint no longer
fires. Verbatim transcripts in the appendix.

## What Changed

- The `unused_must_use` lint fires when a `Result<T, E>` value (or
  any `#[must_use]` type) is the value of an expression statement
  that nothing else uses. Today's trigger is the most common one in
  early Rust: calling a `Result`-returning function and ending the
  line with a semicolon.
- The diagnostic is a *warning*, not an error. The program compiles
  and runs (lesson 069's category). The lint exists because a
  discarded `Result` is usually a bug — the call could have been an
  `Err` and you threw the failure information away.
- `let _ = expr;` is the documented escape hatch. It silences
  `unused_must_use` for a single call site by *deliberately* ignoring
  the value. Today names this exact shape; broader uses of `_` on the
  left of `let` are a future move.
- The note line ``#[warn(unused_must_use)]` (part of `#[warn(unused)]`)
  on by default`` says the lint is on by default — you do not need
  to opt in.

## Check Yourself

You write `tiny.rs`:

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf);
    println!("done");
}
```

(a) Does `rustc tiny.rs` succeed? What is the first word of the
diagnostic block it prints?

(b) Does a `tiny` executable appear next to `tiny.rs`? Does
`echo "x" | ./tiny` run successfully?

(c) The `help:` line in the diagnostic suggests one form to silence
the lint. Write that form for line 5.

(d) Why does this lint exist? In one sentence.

(*Answers: (a) Yes; `rustc` exits 0. First word is `warning:`, not
`error:`. (b) Yes; the program compiles, prints `done`, exits 0.
(c) `let _ = io::stdin().read_line(&mut buf);`. (d) Because a
discarded `Result` could have been an `Err`, so silently throwing
it away usually loses failure information.*)

## What To Ignore For Now

- *The `_` token in general.* Today installs only the exact shape
  `let _ = expr;` as the `unused_must_use` escape hatch. `_`'s
  broader role on the left of `let`, in `match` arms, and in tuple
  destructures is a future move.
- *Authoring `#[must_use]` on your own types or functions.* Today
  only names the attribute as the source of the lint's name.
- *Other `#[must_use]` types in `std`.* `Option<T>` is *not*
  `#[must_use]` — discarding an `Option`-returning call does not
  fire this lint. Other types like `MutexGuard` *are*. Today
  installs the rule for `Result` only.
- *Lint configuration.* `#[allow(unused_must_use)]`,
  `#[deny(unused_must_use)]`, the `-A`/`-D`/`-W`/`-F` flags, and the
  lint level system. All deferred since lesson 069.
- *The `?` operator* — another short way to handle a `Result`
  besides `.expect` (lesson 053). Future move.
- *Other consumers of a `Result`* that also silence the lint:
  `match`, `if let Ok(_) = ...`, binding to a real name. Not
  today's centered move.
- *`rustc --explain unused_must_use`.* `--explain` (lesson 070) is
  for `E####` codes only; lints have no `E####`. The corpus listing
  `output/docs/rust/rustc/lints/listing/warn-by-default.md` is the
  canonical reference instead.

## Evidence

See `../evidence/094-unused-must-use-result.md`.
