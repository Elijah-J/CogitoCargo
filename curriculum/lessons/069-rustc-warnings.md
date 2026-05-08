---
id: 069-rustc-warnings
status: accepted
evidence: ../evidence/069-rustc-warnings.md
---

# `rustc` warnings are a separate category from errors

## The Move

Compile a tiny program with an unused `let` binding. `rustc` prints
a diagnostic that starts with the word `warning:` instead of
`error:`. Unlike every error you have seen so far, `rustc` still
exits 0, an executable still appears next to the source, and you
can run it.

## Mental Model Delta

- Before: "When `rustc` prints a diagnostic block (lesson 003's
  shape), the build is broken: no executable, exit 1."
- After: "`rustc` prints diagnostics in two categories. **Errors**
  (`error:` headline) abort the build: no executable, non-zero
  exit. **Warnings** (`warning:` headline) flag something `rustc`
  wants you to notice, but they do not stop the build by themselves.
  An executable is still produced and exits 0. The diagnostic block
  underneath the headline has the same shape in both cases."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` on success is silent and produces
    an executable next to the source; running it is a separate step.
  - Lesson 002: a program needs `fn main() { ... }`.
  - Lesson 003 (load-bearing): a `rustc` diagnostic has four
    labelled parts — **headline**, `-->` **location**, **source
    excerpt with caret**, and any **help / note** lines — plus an
    optional `error: aborting due to N previous error(s)` trailer.
    So far you have only seen this skeleton with `error:` headlines.
  - Lesson 005: the form `let name = value;`.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
with exactly this body:

```rust
fn main() {
    let x = 5;
}
```

Compile it:

```console
$ rustc demo.rs
warning: unused variable: `x`
 --> demo.rs:2:9
  |
2 |     let x = 5;
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: 1 warning emitted

```

Read the diagnostic with the lesson 003 map. **Headline**:
`warning: unused variable: \`x\``. The first word is `warning:`,
not `error:` — that single word change is the whole point of this
lesson. **Location**, **source excerpt with caret**, and the
`= note:` line all have the same shape as the error case. The
`help:` text is folded onto the caret line, the same layout
lesson 003 saw on the E0601 example. The `= note:` line names a
lint (`unused_variables`) and uses attribute syntax (`#[warn(...)]`)
— treat both as opaque for now.

The **trailer** is different. Errors trail `error: aborting due
to N previous error(s)`; this run trails `warning: 1 warning
emitted`. Notice the word "aborting" is absent. That absence is
rustc telling you the warnings did not abort anything.

After `rustc` finishes, list the directory:

```console
$ ls
demo  demo.rs
```

The executable is there. Run it:

```console
$ ./demo
```

Nothing printed (the body of `main` has no output statement) and
the program exits cleanly. That is the load-bearing observation:
your program ran. With every `error:` you have seen so far
(lessons 002, 003, 005, 068), `rustc` exited 1 and produced no
executable. With this warning, `rustc` exited 0 and produced one.

## What Changed

- `rustc` diagnostics come in two categories: `error:` and
  `warning:`. The headline word tells you which.
- An `error:` aborts the build. A `warning:` does not. Warnings
  are *advisory*: rustc wants you to notice something, but it
  still hands you a working executable.
- Recognizing the category is now mechanical: read the first word
  of the headline.
- The lesson 003 diagnostic map (headline / location / source
  excerpt with caret / help / note) works the same on warnings
  as on errors.
- The "X warning(s) emitted" / "aborting due to N previous
  error(s)" trailers are different sentences, and only one of them
  means "no executable was built."

## Check Yourself

You write `tmp.rs`:

```rust
fn main() {
    let n = 42;
}
```

(a) When you run `rustc tmp.rs`, what is the first word of the
headline `rustc` prints?

(b) After `rustc tmp.rs` finishes, does a file called `tmp` exist
next to `tmp.rs`?

(c) Does `./tmp` run, and what is its exit code?

(d) Now imagine you instead wrote:

```rust
fn main() {
    let n = m;
}
```

(`m` was never bound, so this is the lesson 005 / E0425 case).
The first word `rustc` prints on this one is `error:`, not
`warning:`. After `rustc` finishes, does `tmp` exist? Does
`./tmp` run?

(Answers: (a) `warning:`. (b) Yes. (c) Yes; exits 0; prints
nothing. (d) No `tmp` is produced. `./tmp` cannot run because
there is no such file. The category of the diagnostic — error vs
warning — is exactly what determines whether step (b) and step (c)
work.)

## What To Ignore For Now

This lesson installs only the *category* distinction. Each of the
following is real and deferred:

- The full `rustc` lint *system*: lints, lint groups, lint
  *levels* (`allow` / `warn` / `deny` / `forbid` / `force-warn`
  / `expect`), the flags `-A` / `-W` / `-D` / `-F`, and the
  attributes `#[allow(...)]` / `#[warn(...)]` / `#[deny(...)]` /
  `#[forbid(...)]` (and their crate-level `#![...]` cousins).
  You saw the word `warn` inside `#[warn(...)]` in the `= note:`
  line; treat that as opaque for now.
- The `#![deny(warnings)]` pattern that promotes every warning
  into a hard error. Deferred.
- `cargo fix`, which can apply some warnings' suggested fixes.
  Deferred.
- The full lint taxonomy: `dead_code`, `unused_imports`,
  `unused_mut`, and many others. `unused_variables` is the
  example here only because it is the smallest reachable warning.
- *Fixing* this warning by renaming `x` to `_x` or by actually
  using `x`. The lesson teaches the category, not the fix; the
  `help:` line in the diagnostic shows the rename when you are
  ready.
- Clippy, the third-party lint tool. Deferred.
- Future-incompatible warnings (warnings that will become errors
  in a later `rustc` release). Same category mechanics; deferred.

## Evidence

See `../evidence/069-rustc-warnings.md`.
