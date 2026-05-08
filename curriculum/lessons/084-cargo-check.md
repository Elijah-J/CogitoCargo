---
id: 084-cargo-check
status: accepted
evidence: ../evidence/084-cargo-check.md
---

# Typecheck without producing the executable: `cargo check`

## The Move

From inside a Cargo package directory (the same setup as lesson 064:
`Cargo.toml` and `src/`, scaffolded by `cargo new`), run `cargo check`.
Cargo verifies that `src/main.rs` compiles, but it does *not* produce
an executable at `target/debug/<name>`. The output's leading verb is
`Checking <name> v0.1.0 (...)` instead of `Compiling`. The `Finished`
line still appears with the same `dev` profile shape as lesson 064.
Use `cargo check` as a fast iteration loop while editing; reach for
`cargo build` (lesson 064) only when you actually want the binary.

## Mental Model Delta

- *Before:* "I run `cargo build` to compile and `cargo run` to
  compile-and-run. Both produce an executable at
  `target/debug/<name>`."
- *After:* "There is a third Cargo verb. `cargo check` does the
  typechecking step of `cargo build` but stops short of producing
  the executable. The output's leading verb flips from `Compiling`
  to `Checking`; no executable lands at `target/debug/<name>`. Use
  it as a fast iteration loop: `cargo check` to know if the code
  still compiles, `cargo build` only when you want the binary."

## Prerequisites

- Installed concepts:
  - Lesson 064 (`064-cargo-build-standalone`): `cargo build` from
    inside a package compiles to `target/debug/<name>` and prints
    ``Compiling <name> v0.1.0 (...)`` then ``Finished `dev` profile
    [unoptimized + debuginfo] target(s) in <time>s``. Today contrasts
    the verb `Checking` with `Compiling` and the absence of the
    binary at `target/debug/<name>`.
  - Lesson 032 (`032-cargo-new-and-run`): `cargo new <name>`
    scaffolds the package directory.
  - Lesson 001 (`001-rustc-compile-and-run`): `./executable` runs an
    executable; running is a separate step after compilation.
  - Lesson 002 (`002-fn-main-entry-point`): `fn main() { ... }` runs
    when the executable launches.
  - Lesson 011 (`011-println-positional-args`): `println!` prints to
    stdout.
- Ordinary computer-use assumptions: same terminal/editor/shell as
  lesson 064; `cargo` on `PATH`; `cd` and `ls`.

## Try It

Pick any directory you can write to:

```console
$ cargo new --vcs none hello_check
    Creating binary (application) `hello_check` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd hello_check
```

Run `cargo check` first:

```console
$ cargo check
    Checking hello_check v0.1.0 (/path/to/hello_check)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
```

Two things to notice in the output:

1. `Checking hello_check v0.1.0 (...)` — Cargo's leading verb is
   `Checking`, not `Compiling`. With `cargo build` (lesson 064) the
   first line says `Compiling`; with `cargo check` it says
   `Checking`.
2. `` Finished `dev` profile [unoptimized + debuginfo] target(s) in
   <time>s `` — the same literal `Finished` line shape as lesson 064.
   The `dev` profile applies; only the leading verb of the work
   itself differs.

Confirm no executable was produced:

```console
$ ls target/debug/
build       deps        examples    incremental
$ ls target/debug/hello_check
ls: target/debug/hello_check: No such file or directory
```

`target/debug/` exists with its bookkeeping subdirectories
(`build/`, `deps/`, etc.), but the binary `hello_check` is *absent*.
The `ls target/debug/hello_check` command exits non-zero — there is
nothing to run.

Now run `cargo build` (lesson 064) to see the contrast:

```console
$ cargo build
   Compiling hello_check v0.1.0 (/path/to/hello_check)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.76s
$ ls target/debug/
build           deps            examples        hello_check     hello_check.d   incremental
$ ./target/debug/hello_check
Hello, world!
```

The leading verb is now `Compiling`. The listing now includes
`hello_check` (the executable) and `hello_check.d` (a Cargo
bookkeeping file). Running `./target/debug/hello_check` works because
the binary now exists.

## What Changed

- `cargo check` is the typecheck-only Cargo verb. Output:
  ``Checking <name> v0.1.0 (...)`` then ``Finished `dev` profile
  [unoptimized + debuginfo] target(s) in <time>s``. No executable
  is produced.
- `cargo build` (lesson 064) and `cargo run` (lesson 032) both
  produce the executable at `target/debug/<name>`; `cargo check`
  does not.
- Why prefer `cargo check` while editing? It is faster than
  `cargo build` because it skips the step of producing the
  executable (Book Ch1-3 lines 181-184).
- The Book's recommended workflow: run `cargo check` periodically
  while editing to confirm the code still compiles; run `cargo build`
  (or `cargo run`) when you actually want the binary.

## Check Yourself

You are inside a fresh `greeter/` package from `cargo new greeter`.

- After `cargo check` from inside `greeter/` (no other commands run
  first), can you run `./target/debug/greeter`?
- The leading verb in Cargo's output names the work it just did. For
  `cargo check`, what is that verb?
- One reason to prefer `cargo check` over `cargo build` while editing?

(Answers: No — the executable does not exist yet; `cargo check` does
not produce it. `Checking`. It is faster, because it skips the step
of producing the executable.)

## What To Ignore For Now

- The internals of `target/debug/` left behind by `cargo check`
  (`build/`, `deps/`, `examples/`, `incremental/` and the metadata
  files Cargo writes there). Same deferral as lesson 064.
- `cargo check --release`. The `--release` flag (lesson 082) composes
  onto `cargo check` like other Cargo subcommands; future move.
- `cargo clippy`. A separate, also-fast linting tool; separate move.
- The `--message-format=json` output mode and other Cargo verbosity
  controls. Deferred.
- IDE / editor integrations that run `cargo check` on save
  (rust-analyzer and friends). Operational sprawl; deferred.
- `cargo check --all-targets` / `--tests` / `--examples` — targets
  beyond the default binary. Deferred.
- The man-page note that "some diagnostics and errors are only
  emitted during code generation, so they inherently won't be
  reported with `cargo check`." True, but rare in practice; future
  move if it ever bites.

## Evidence

See `../evidence/084-cargo-check.md`.
