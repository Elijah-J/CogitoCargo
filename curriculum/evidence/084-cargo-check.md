# Evidence — 084-cargo-check

Audit appendix for `lessons/084-cargo-check.md`. The lesson teaches
one move: `cargo check` from inside a Cargo package directory
verifies that the source compiles but does *not* produce an
executable at `target/debug/<name>`. The output's leading verb is
`Checking <name> v0.1.0 (...)` instead of `Compiling`. The `Finished`
line is the same `dev` profile shape as lesson 064. The lesson
installs one new concept: a third Cargo verb (alongside `cargo build`
from lesson 064 and `cargo run` from lesson 032) whose pedagogical
purpose is fast iteration — typecheck without paying to produce the
binary.

This appendix covers (a) toolchain and reproducibility, (b) corpus
quote map, (c) verbatim probe transcript, (d) the committed
observation files, (e) prerequisite-claim summary, and (f)
contrast-probe coverage.

## Toolchain

```
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
host: x86_64-apple-darwin
```

`uname -sm` -> `Darwin x86_64`. Probe ran in a fresh `mktemp -d`
directory: `/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.LsVsDLXIjM/`.
A second corroborating probe ran in
`/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.kDa5ladYm5/`
(used only for the cache-hit observation in step 6 below — not
load-bearing for the lesson).

## Project setup

The committed observation directory `observations/084-cargo-check/`
contains the post-probe files:

- `Cargo.toml` — bit-exact match to what `cargo new --vcs none
  hello_check` writes:

  ```toml
  [package]
  name = "hello_check"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  ```

- `src/main.rs` — the canonical `cargo new`-generated program (no
  edits — the lesson does not need an edit-driven contrast):

  ```rust
  fn main() {
      println!("Hello, world!");
  }
  ```

- `.gitignore` — listing `/target` and `Cargo.lock`. Build artifacts
  are not committed.

## Probe transcript

All steps run from
`/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.LsVsDLXIjM/`.

### Step 1: Toolchain capture and scaffold

```text
$ cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -sm
Darwin x86_64

$ cargo new --vcs none hello_check
    Creating binary (application) `hello_check` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

Bit-identical (modulo package name) to lesson 064 step 1.

### Step 2: Inspect the generated package

```text
$ cd hello_check
$ ls -la
total 8
drwxr-xr-x  4 eli  staff  128 May  7 19:22 .
drwx------  3 eli  staff   96 May  7 19:22 ..
-rw-r--r--  1 eli  staff   82 May  7 19:22 Cargo.toml
drwxr-xr-x  3 eli  staff   96 May  7 19:22 src

$ cat Cargo.toml
[package]
name = "hello_check"
version = "0.1.0"
edition = "2024"

[dependencies]

$ cat src/main.rs
fn main() {
    println!("Hello, world!");
}
```

Bit-identical to lesson 064 step 2-3.

### Step 3: First `cargo check`

```text
$ cargo check
    Checking hello_check v0.1.0 (/private/var/folders/.../tmp.LsVsDLXIjM/hello_check)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
exit=0
```

**Load-bearing observation 1 — the leading verb is `Checking`, not
`Compiling`.** This is the central contrast with lesson 064 step 3,
which on the same toolchain prints
``Compiling hello_release v0.1.0 (...)``. Same package layout, same
profile, same `Finished` line shape — only the verb of the work
itself flips from `Compiling` to `Checking`.

**Load-bearing observation 2 — the `Finished` line is identical to
lesson 064's.** Both runs print
`` Finished `dev` profile [unoptimized + debuginfo] target(s) in
<time>s ``. The lesson treats this as a known shape from lesson 064
and does not reteach it.

### Step 4: Confirm no executable was produced

```text
$ ls target/
CACHEDIR.TAG
debug

$ ls target/debug/
build
deps
examples
incremental

$ ls target/debug/hello_check
ls: target/debug/hello_check: No such file or directory
exit=1
```

**Load-bearing observation 3 — the binary at `target/debug/<name>`
is absent.** The `target/debug/` directory exists with its
bookkeeping subdirectories (`build/`, `deps/`, `examples/`,
`incremental/`), but the named executable `hello_check` is *not*
there. `ls target/debug/hello_check` exits 1 with a
`No such file or directory` message.

This is the empirical witness for the Book's claim that `cargo check`
"doesn't produce an executable" (lines 172-173). The directory
listing is also strictly smaller than the post-`cargo build` listing
in step 5: it is missing both `hello_check` (the binary) and
`hello_check.d` (the Cargo dep file).

### Step 5: Now run `cargo build` for the contrast

```text
$ cargo build
   Compiling hello_check v0.1.0 (/private/var/folders/.../tmp.LsVsDLXIjM/hello_check)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.76s
exit=0

$ ls target/debug/
build
deps
examples
hello_check
hello_check.d
incremental

$ ./target/debug/hello_check
Hello, world!
exit=0
```

**Load-bearing observation 4 — `cargo build` flips the verb to
`Compiling` and produces the binary.** After `cargo build`, the
listing includes both `hello_check` (the executable) and
`hello_check.d` (the Cargo dep file); the bookkeeping subdirectories
are unchanged. `./target/debug/hello_check` runs and prints
`Hello, world!` exit 0 — lesson 064's central observation, reproduced
here as the contrast for `cargo check`.

This is the side-by-side contrast the lesson rests on. Same package,
same source, same toolchain. Only the Cargo verb differs:

| Cargo verb     | leading line of work                | binary at `target/debug/hello_check`? |
| -------------- | ----------------------------------- | ------------------------------------- |
| `cargo check`  | `Checking hello_check v0.1.0 (...)` | absent                                |
| `cargo build`  | `Compiling hello_check v0.1.0 (...)`| present                               |

The `Finished` line is shape-identical for both; only the verb in
the line above it (and the existence of the `target/debug/<name>`
file) differ.

### Step 6: Cache-hit `cargo check` (corroborating, not load-bearing)

Probe in a separate fresh scratch directory
(`tmp.kDa5ladYm5/hello_check2`):

```text
$ cargo check
    Checking hello_check2 v0.1.0 (/private/var/folders/.../tmp.kDa5ladYm5/hello_check2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
exit=0

$ cargo check
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
exit=0
```

Same caching mechanism lesson 064 captured for `cargo build`, applied
here to `cargo check`. The second invocation skips the `Checking`
line (no source change) and prints only `Finished`. The lesson does
not reteach this — caching is already installed by lesson 064 — but
the observation is documented here for completeness and to show that
`cargo check`'s on-disk metadata is reused identically.

The man-page sentence "The compiler will save metadata files to disk
so that future runs will reuse them if the source has not been
modified" (corpus, lines 16-18) corroborates this caching behavior.

## Corpus quote map

### `output/docs/rust/book/ch01-03-hello-cargo.md`

The Book's "Hello, Cargo!" chapter. The dedicated subsection
introducing `cargo check` is the canonical pedagogical source.

**Lines 172-179** (load-bearing — the introduction and output
shape):

> Cargo also provides a command called `cargo check`. This command
> quickly checks your code to make sure it compiles but doesn't
> produce an executable:
>
> ```console
> $ cargo check
>    Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
>     Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
> ```

This grounds the lesson's central claims:

- "checks your code to make sure it compiles but doesn't produce an
  executable" -> *The Move*'s "verifies that `src/main.rs` compiles,
  but it does *not* produce an executable" and *What Changed*
  bullet 1.
- ``Checking hello_cargo v0.1.0 (...)`` -> *The Move*'s leading-verb
  claim and *Try It* step 3's transcript.
- ``Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs``
  -> *The Move*'s "the `Finished` line still appears with the same
  `dev` profile shape as lesson 064." Note: the Book's printed line
  uses unquoted `dev` and "secs"; the rustc 1.95.0 output uses
  backticked `` `dev` `` and "s". The probe transcript is the
  authoritative wording for *this* toolchain; the Book's wording
  reflects an earlier release. Lesson 064's appendix already
  documents the same drift for the `cargo build` `Finished` line, so
  the lesson reuses lesson 064's literal line shape (with backticks)
  rather than the Book's older unquoted form.

**Lines 181-187** (load-bearing — the *why* and the workflow):

> Why would you not want an executable? Often, `cargo check` is much
> faster than `cargo build` because it skips the step of producing
> an executable. If you're continually checking your work while
> writing the code, using `cargo check` will speed up the process of
> letting you know if your project is still compiling! As such, many
> Rustaceans run `cargo check` periodically as they write their
> program to make sure it compiles. Then, they run `cargo build`
> when they're ready to use the executable.

This grounds the lesson's pedagogical purpose:

- "much faster than `cargo build` because it skips the step of
  producing an executable" -> *What Changed* bullet 3 (the *why*
  for preferring `cargo check` while editing).
- "many Rustaceans run `cargo check` periodically as they write
  their program to make sure it compiles. Then, they run
  `cargo build` when they're ready to use the executable" ->
  *Mental Model Delta*'s "fast iteration loop" framing and
  *What Changed* bullet 4.

**Lines 189-195** (corroborating recap — not directly cited):

> Let's recap what we've learned so far about Cargo:
>
> - We can create a project using `cargo new`.
> - We can build a project using `cargo build`.
> - We can build and run a project in one step using `cargo run`.
> - We can build a project without producing a binary to check for
>   errors using `cargo check`.

The Book's own recap names exactly the four-verb framing the lesson
positions today's move within: `cargo new` (lesson 032), `cargo build`
(lesson 064), `cargo run` (lesson 032), and `cargo check` (today).

The lesson does not claim a measured speed difference on its own
probe — the trivial `println!` program is too small to exhibit a
visible one, and the `0.46s` (check) versus `0.76s` (build) numbers
in the transcript are not load-bearing as a speed claim. The Book's
"much faster" sentence is restated as a *rule* in *What Changed*
bullet 3, not demonstrated as a measurement.

### `output/docs/rust/cargo/commands/cargo-check.md`

The reference man-page for `cargo check`. Used as a corroborating
second-corpus witness.

**Lines 4-6** (corroborating — the one-line description):

> NAME
>
> cargo-check — Check the current package

**Lines 14-19** (corroborating — the technical description):

> Check a local package and all of its dependencies for errors. This
> will essentially compile the packages without performing the final
> step of code generation, which is faster than running `cargo build`.
> The compiler will save metadata files to disk so that future runs
> will reuse them if the source has not been modified. Some
> diagnostics and errors are only emitted during code generation, so
> they inherently won't be reported with `cargo check`.

This corroborates two lesson claims:

- "compile the packages without performing the final step of code
  generation" + "faster than running `cargo build`" -> *What Changed*
  bullet 3 (faster because it skips producing the executable). Two
  independent corpus sources (Book + man-page) state this rule.
- "save metadata files to disk so that future runs will reuse them
  if the source has not been modified" -> step 6's cache-hit
  observation. Lesson 064 already installed Cargo's caching, so the
  lesson body does not reteach it.

The "Some diagnostics and errors are only emitted during code
generation" caveat is named in *What To Ignore For Now* as a future
move; it is rare in practice for the size of programs the audience
is writing today.

## Prerequisite-claim summary

### From lesson 064 (`064-cargo-build-standalone`) — *direct, load-bearing*

- `cargo build` from inside a Cargo package directory compiles
  `src/main.rs` and writes the executable to `target/debug/<name>`,
  printing two lines: ``Compiling <name> v0.1.0 (<path>)`` and
  ``Finished `dev` profile [unoptimized + debuginfo] target(s) in
  <time>s``. Today's `cargo check` produces the same `Finished` line
  shape; the leading verb flips from `Compiling` to `Checking`, and
  the executable at `target/debug/<name>` is *absent* instead of
  present.
- The binary at `target/debug/<name>` is run with
  `./target/debug/<name>` — same `./executable` shape as lesson 001.
  Today's lesson uses this only as the contrast (after `cargo build`
  the binary exists and runs; after `cargo check` it does not exist
  to run).
- Lesson 064's *What To Ignore* explicitly defers `cargo check`
  ("The Book pairs it with `cargo build` (it typechecks without
  producing an executable). Future move."). Today is exactly that
  future move.

### From lesson 032 (`032-cargo-new-and-run`) — *supporting*

Mentioned by number/title only. Cycle 032 grounds `cargo new <name>`
scaffolding the package with `Cargo.toml` and `src/main.rs`. Today's
probe uses `cargo new --vcs none hello_check`; the `--vcs none` form
is documented in lesson 064's appendix.

### From lesson 001 (`001-rustc-compile-and-run`) — *supporting*

Mentioned by number/title only. Lesson 001 grounds the
`./executable` shape; lesson 064 already restates the load-bearing
form (`./target/debug/<name>`). Today's lesson invokes this shape
only in step 5's contrast (after `cargo build`, `./target/debug/...`
runs).

### From lessons 002 and 011 — *supporting*

Mentioned by number/title only. Lesson 002 grounds `fn main()`
running when the executable launches; lesson 011 grounds `println!`
printing to stdout. Both are restated by lesson 064's prerequisites
and not load-bearing in any way not already covered there.

## Contrast-probe coverage

The lesson's central contrastive claim is the side-by-side run of
`cargo check` (step 3) and `cargo build` (step 5) on the same package:

1. **Leading verb of Cargo's output:** `Checking` versus `Compiling`.
   Steps 3 and 5 are the empirical witness — same package, same
   source, same toolchain, same fresh-cache state at each command's
   first invocation. The `Finished` line is identical in shape; only
   the verb above it differs. Both commands succeed (exit 0) — this
   is a contrast between two valid-but-different working forms, not
   a working-vs-broken contrast, so no negative probe is needed.

2. **Presence of the binary at `target/debug/<name>`:** absent after
   `cargo check`, present after `cargo build`. Step 4 (the
   `ls target/debug/hello_check: No such file or directory` exit
   1) is the empirical witness for the absence claim, and step 5's
   `ls target/debug/` listing including `hello_check` (and
   `./target/debug/hello_check` running successfully) is the
   counterpart for the presence claim. The `target/debug/` directory
   contents differ by exactly two entries: `hello_check` and
   `hello_check.d`. The bookkeeping subdirectories (`build/`, `deps/`,
   `examples/`, `incremental/`) are present in both.

The lesson does not make a measured-speed claim on this probe (the
`println!` program is too small for the optimizer to make a visible
timing difference, and one-shot wall-clock numbers from a single host
are noisy). The Book's "much faster" sentence is restated as a
*rule*, not demonstrated as a measurement; a second corpus source
(the man-page) corroborates the rule.

## What is *not* in this probe

The probe deliberately does not exercise:

- `cargo check --release`. The `--release` flag (lesson 082) composes
  onto `cargo check` like other Cargo subcommands; explicitly
  deferred in *What To Ignore*.
- `cargo clippy`. A separate, also-fast linting tool; separate move.
- `--message-format=json` and other Cargo verbosity controls.
  Deferred.
- IDE / editor integrations that run `cargo check` on save
  (rust-analyzer and friends). Operational sprawl; deferred.
- `cargo check --all-targets` / `--tests` / `--examples`. Targets
  beyond the default binary; deferred.
- The man-page's "some diagnostics and errors are only emitted
  during code generation, so they inherently won't be reported with
  `cargo check`" caveat. Named in *What To Ignore* as a future
  move; not exercised because constructing a probe that triggers
  one of these rare codegen-only diagnostics requires machinery the
  audience does not yet have.
- A measured speed comparison between `cargo check` and `cargo build`.
  The probe's program is too trivial; the Book's framing of "much
  faster" is restated as a rule, not demonstrated as a measurement.
- Windows path separators (lesson 064's deferral preserved).

## Files committed for this cycle

- `lessons/084-cargo-check.md` (this lesson)
- `evidence/084-cargo-check.md` (this appendix)
- `observations/084-cargo-check/Cargo.toml`
- `observations/084-cargo-check/src/main.rs`
- `observations/084-cargo-check/.gitignore`
- updated `graph.md` (a new draft node block under `## Draft Nodes`)
