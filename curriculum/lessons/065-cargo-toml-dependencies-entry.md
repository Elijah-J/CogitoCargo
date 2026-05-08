---
id: 065-cargo-toml-dependencies-entry
status: accepted
evidence: ../evidence/065-cargo-toml-dependencies-entry.md
---

# List an external crate under `[dependencies]`

## The Move

Open the `Cargo.toml` your package was scaffolded with (lesson 032).
Under the `[dependencies]` section header — the empty section
lessons 032 and 064 named but never filled — add one line of the
shape `<crate-name> = "<version-string>"`. Today's exact line, copied
from the Book's guessing-game tutorial, is:

```toml
[dependencies]
rand = "0.8.5"
```

Save the file. Run `cargo build`. Cargo contacts crates.io (the public
registry of Rust crates), figures out which crates are needed,
downloads them, compiles them, then compiles your package on top.
Your `src/main.rs` is unchanged today; we are not yet calling anything
from `rand`. The cycle is about wiring the entry and watching the
resolver run.

## Mental Model Delta

- *Before:* "Lessons 032 and 064 named the `[dependencies]` section
  but kept it empty. My package depends on no external code."
- *After:* "I can list someone else's Rust code under `[dependencies]`
  as `<crate-name> = \"<version-string>\"`. The next `cargo build`
  fetches that crate (and any crates *it* depends on) from crates.io,
  builds them, then builds my package on top. The dependency is now
  *resolved and built* — importing and calling it is a separate move."

## Prerequisites

- Installed concepts:
  - Lesson 032 (`032-cargo-new-and-run`): `cargo new <name>` scaffolds
    a package with `Cargo.toml` (containing `[package]` and an empty
    `[dependencies]` section) and `src/main.rs`. Today fills the
    empty section.
  - Lesson 064 (`064-cargo-build-standalone`): `cargo build` from
    inside a package directory compiles the package and stops, with
    output `Compiling <name> v0.1.0 (...)` then `Finished \`dev\`
    profile ... in X.XXs`. Today extends that output shape to
    include the dependency-resolution lines.
  - Lessons 001, 002, 011 (rustc compile-and-run; `fn main`;
    `println!`): so the default `Hello, world!` program from
    `cargo new` still makes sense as a runnable program.
- Ordinary computer-use assumptions: terminal; plain-text editor that
  saves UTF-8; `cargo` on `PATH`; `cd`, `ls`, `cat`; **internet
  access** — Cargo's first build with a new dependency must reach
  crates.io to download.

## Try It

In a directory you can write to:

```console
$ cargo new --vcs none hello_dep
    Creating binary (application) `hello_dep` package
$ cd hello_dep
$ cat Cargo.toml
[package]
name = "hello_dep"
version = "0.1.0"
edition = "2024"

[dependencies]
```

Edit `Cargo.toml` so the bottom looks like:

```toml
[dependencies]
rand = "0.8.5"
```

Leave `src/main.rs` alone — it should still be the default
`fn main() { println!("Hello, world!"); }`. Run `cargo build`:

```console
$ cargo build
    Updating crates.io index
     Locking 14 packages to latest Rust 1.95.0 compatible versions
      Adding rand v0.8.6 (available: v0.10.1)
 Downloading crates ...
  Downloaded rand v0.8.6
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.4
  Downloaded getrandom v0.2.17
  ... (more crates)
   Compiling libc v0.2.186
   Compiling cfg-if v1.0.4
   ... (more crates)
   Compiling rand v0.8.6
   Compiling hello_dep v0.1.0 (/path/to/hello_dep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 11.20s
```

(Full transcript is in the evidence appendix.) Three things to notice:

1. `Updating crates.io index` and `Downloading crates ...` show up on
   this first build with a new dependency. They were absent in lesson
   064's no-dependency build.
2. Cargo compiles **more than just `rand`**. The crates `libc`,
   `cfg-if`, `getrandom`, `rand_core`, `rand_chacha`, etc. are
   `rand`'s own dependencies. You only listed `rand`; Cargo handled
   the rest.
3. After all dependencies are built, Cargo finally runs the familiar
   `Compiling hello_dep v0.1.0 (...)` + `Finished` pair from lesson
   064.

Look at the directory now:

```console
$ ls
Cargo.lock  Cargo.toml  src  target
```

A new file, `Cargo.lock`, appeared next to `Cargo.toml`. Lesson 032
flagged this file and deferred it; today, just notice that Cargo
created it on the resolving build to record the exact versions it
chose. (Treat the file as opaque for now.)

Now run `cargo build` a second time without changing anything:

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
```

No `Updating`, no `Downloading`, no `Compiling`. Same cache-hit shape
as lesson 064: nothing changed, so Cargo did nothing. As a sanity
check, `cargo run` (lesson 032) still prints `Hello, world!` —
because `src/main.rs` is unchanged.

Note: the Book quotes `rand v0.8.5`, but the transcript above shows
Cargo picking `rand v0.8.6`. Your output may differ similarly. Cargo
picked a version *compatible* with what `Cargo.toml` requested; the
rule for how it does that (SemVer) is a future move. For today,
copy the exact string `rand = "0.8.5"` and trust Cargo's choice.

## What Changed

- You can list an external crate under `[dependencies]` in
  `Cargo.toml` as `<crate-name> = "<version-string>"`.
- You know the next `cargo build` will (a) hit crates.io, (b)
  download the crate and any crates it depends on, (c) build all of
  them, (d) then build your package. After that, a no-change rebuild
  is back to the lesson-064 cache-hit shape.
- You know `Cargo.lock` is created on that resolving build. Treat it
  as Cargo's record of the exact versions it picked; details are a
  future move.
- You have NOT yet *used* the crate. `src/main.rs` still prints
  `Hello, world!`. Importing `rand` and calling its API is the next
  move.

## Check Yourself

You start with a fresh `cargo new playground` and an empty
`[dependencies]`. You add one line, `serde = "1.0"`, and save.

- What new lines do you expect on the *next* `cargo build` that you
  did NOT see in a no-dependency build?
- After that build finishes, what new file do you expect next to
  `Cargo.toml`?
- You run `cargo build` a second time without editing anything. What
  does the output look like?

(Answers: a `Updating crates.io index` line, possibly a `Locking ...
packages` line, `Downloading` lines for `serde` plus its own
dependencies, and `Compiling` lines for each crate before the final
`Compiling playground v0.1.0 (...)`. New file: `Cargo.lock`. Second
build: `Finished` only — Cargo did nothing.)

## What To Ignore For Now

- *What the version string `"0.8.5"` actually means as a specifier
  (SemVer).* Future move; the body note above already names this.
- *`Cargo.lock`'s contents and TOML structure.* Lesson 032 deferred
  this; today just notice the file appears on the resolving build.
  Reproducible-build semantics are a future move.
- *`cargo update`* for upgrading dependency versions. Future move.
- *`cargo add <crate>`*: Cargo also has a subcommand that edits
  `Cargo.toml` for you. We added the line by hand to make the
  manifest edit visible. Future move.
- *Path and git dependencies* (`{ path = "..." }`,
  `{ git = "..." }`); *`[dev-dependencies]`* and
  *`[build-dependencies]`* sections; *feature flags*
  (`features = [...]`); *workspace dependencies*; *alternate
  registries*; *vendoring*; *`cargo build --offline`*. All future.
- *crates.io's account/publishing/ownership model.* Today: "Cargo's
  default place to download public Rust crates."
- *Importing `rand` with `use rand::...;` and calling `rand`'s API.*
  Strictly the next move — today's program doesn't touch `rand`.

## Evidence

See `../evidence/065-cargo-toml-dependencies-entry.md`.
