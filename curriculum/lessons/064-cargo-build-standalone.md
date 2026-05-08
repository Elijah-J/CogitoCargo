---
id: 064-cargo-build-standalone
status: accepted
evidence: ../evidence/064-cargo-build-standalone.md
---

# Build a package without running it: `cargo build`

## The Move

From inside a Cargo package directory (the directory containing
`Cargo.toml` and `src/`, scaffolded by `cargo new`), run `cargo build`.
Cargo compiles `src/main.rs` and writes the resulting executable to
`target/debug/<name>`, where `<name>` is the package name from
`Cargo.toml`'s `[package] name = "..."` line. Cargo does *not* run
the program. To run it, invoke the binary yourself with
`./target/debug/<name>` — the same `./executable` shape as lesson 001,
just at a deeper path.

## Mental Model Delta

- *Before:* "`cargo run` from lesson 032 compiles and runs in one
  step. Cargo handles everything."
- *After:* "`cargo build` is the build half alone. It produces the
  executable at `target/debug/<name>` and stops; running it is a
  separate step I do myself with `./target/debug/<name>`. This is
  lesson 001's two-step `rustc file.rs` + `./file` workflow at the
  Cargo layer. `cargo run` is the convenience wrapper that does both
  in one command."

## Prerequisites

- Installed concepts:
  - Lesson 001 (`001-rustc-compile-and-run`): the two-step *compile,
    then run* shape; running an executable (`./executable`) is a
    separate command after compilation.
  - Lesson 002 (`002-fn-main-entry-point`): `fn main() { ... }` runs
    when the executable launches.
  - Lesson 011 (`011-println-positional-args`): `println!` prints to
    stdout. The probe edits the default greeting to distinguish the
    rebuilt binary's output from the original's.
  - Lesson 032 (`032-cargo-new-and-run`): `cargo new <name>` scaffolds
    a package with `Cargo.toml` and `src/main.rs`; `cargo run`
    compiles and runs in one step. Today is the build half of that.
- Ordinary computer-use assumptions: same terminal/editor/shell setup
  as lesson 032; `cargo` on `PATH`; `cd` and `ls`.

## Try It

Pick any directory you can write to:

```console
$ cargo new --vcs none hello_build
    Creating binary (application) `hello_build` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd hello_build
$ cargo build
   Compiling hello_build v0.1.0 (/path/to/hello_build)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.70s
```

Three things to notice:

1. `Compiling hello_build v0.1.0 (...)` — Cargo narrating the compile.
2. `Finished `dev` profile [unoptimized + debuginfo] target(s) in
   0.70s` — Cargo announces it finished. (`dev` is the default
   profile name; treat this as the literal output line for now.)
3. **There is no `Running` line.** With `cargo run` (lesson 032), a
   third line `Running \`target/debug/hello_build\`` would appear,
   followed by the program's output. `cargo build` stops after
   `Finished`.

Run the executable yourself:

```console
$ ./target/debug/hello_build
Hello, world!
```

That is lesson 001's `./file` shape at a deeper path. The default
`fn main` printed `Hello, world!` (lesson 032).

Now edit `src/main.rs` to print something else and rebuild:

```rust
fn main() {
    println!("Built but not run!");
}
```

```console
$ cargo build
   Compiling hello_build v0.1.0 (/path/to/hello_build)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
$ ./target/debug/hello_build
Built but not run!
```

Cargo recompiled (the source changed) and overwrote the binary at
the same path. Running it shows the new output.

Run `cargo build` a third time without editing anything:

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
```

No `Compiling` line this time — Cargo saw the source was unchanged
and skipped the work. (Lesson 032 noted the same caching for
`cargo run`.)

## What Changed

- You can compile a Cargo package without running it: `cargo build`
  from inside the package directory.
- You know the produced binary's path — `target/debug/<name>` with
  `<name>` from `Cargo.toml` — and you can run it yourself with
  `./target/debug/<name>`. That pair is the two-step cousin of
  lesson 001's `rustc file.rs && ./file`.
- You can read three shapes of build output: fresh build (`Compiling`
  + `Finished`), rebuilt after source edit (same shape), no source
  change since last build (just `Finished`).
- `cargo run` is roughly `cargo build` then run the binary. Reach for
  `cargo build` when you want the executable but do not want it to
  start running yet — to ship it, time the compile, or run it later
  with input you will provide separately.

## Check Yourself

You are inside a fresh `greeter/` package from `cargo new greeter`.

- After `cargo build`, what is the path of the executable?
- What command (no `cargo`) runs that executable from inside
  `greeter/`?
- You run `cargo build` twice with no edits between. One line from
  the first run is missing in the second. Which line, and why?

(Answers: `target/debug/greeter`. `./target/debug/greeter`. The
`Compiling greeter v0.1.0 (...)` line; Cargo sees the source has not
changed and skips the compile, printing only `Finished`.)

## What To Ignore For Now

- The meaning of `dev` in `Finished \`dev\` profile`. `dev` is the
  default *profile* name. Build profiles (the contrast with
  `release`, `cargo build --release`, optimization levels, the
  `[profile.*]` manifest sections) are a future move. Treat `dev`
  as part of the literal output line.
- `cargo check`. The Book pairs it with `cargo build` (it typechecks
  without producing an executable). Future move.
- The internals of `target/`. The `target/debug/` listing also
  contains `hello_build.d`, `deps/`, `incremental/`, etc.; only
  `target/debug/<name>` is part of today's lesson.
- `Cargo.lock`. Appears beside `Cargo.toml` after the first build;
  lesson 032's deferral stands.
- The `[dependencies]` section. Named in lesson 032; with
  dependencies, `cargo build` would also fetch and build them.
  Future move.
- `cargo new --lib`, library packages, multi-file projects,
  workspaces, modules. `cargo doc`, `cargo test`, `cargo clean`,
  `cargo fmt`, `cargo clippy`. Crates.io and the public ecosystem.
  All deferred.
- Windows path separators. Cargo commands are cross-platform;
  shown paths use macOS/Linux style.

## Evidence

See `../evidence/064-cargo-build-standalone.md`.
