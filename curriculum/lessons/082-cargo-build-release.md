---
id: 082-cargo-build-release
status: accepted
evidence: ../evidence/082-cargo-build-release.md
---

# Build the optimized version: `cargo build --release`

## The Move

From inside a Cargo package directory (the same setup as lesson 064:
`Cargo.toml` and `src/`, scaffolded by `cargo new`), run
`cargo build --release`. Cargo compiles `src/main.rs` with the
optimizing settings turned on and writes the resulting executable to
`target/release/<name>` instead of `target/debug/<name>`. The
`Finished` line says `release` profile and `[optimized]` instead of
`dev` profile and `[unoptimized + debuginfo]`. Run the optimized
binary yourself with `./target/release/<name>` — same `./executable`
shape as lesson 001, just at the `release/` path instead of `debug/`.

## Mental Model Delta

- *Before:* "I run `cargo build` and get one binary at
  `target/debug/<name>`."
- *After:* "Cargo has two build profiles. The default profile, `dev`,
  optimizes for fast rebuilds during iteration; its binary lands at
  `target/debug/<name>`. The `release` profile, selected by
  `cargo build --release`, applies the compiler's optimizations; its
  binary lands at `target/release/<name>`. Two parallel directories,
  two different binaries from the same source. The `release` build is
  what to ship and what to benchmark; the `dev` build is what to use
  while iterating."

## Prerequisites

- Installed concepts:
  - Lesson 064 (`064-cargo-build-standalone`): `cargo build` from
    inside a package compiles to `target/debug/<name>` and prints
    ``Finished `dev` profile [unoptimized + debuginfo] target(s) in
    <time>s``. Today extends 064 with the `--release` flag.
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
$ cargo new --vcs none hello_release
    Creating binary (application) `hello_release` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd hello_release
```

First, run the plain `cargo build` from lesson 064 so you have the
debug binary to compare against:

```console
$ cargo build
   Compiling hello_release v0.1.0 (/path/to/hello_release)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.25s
$ ./target/debug/hello_release
Hello, world!
```

Now run the same command with `--release`:

```console
$ cargo build --release
   Compiling hello_release v0.1.0 (/path/to/hello_release)
    Finished `release` profile [optimized] target(s) in 1.72s
```

Two things changed in the second `Finished` line: the profile name
flipped from `dev` to `release`, and the bracketed phrase flipped
from `[unoptimized + debuginfo]` to `[optimized]`. List `target/`:

```console
$ ls target/
CACHEDIR.TAG    debug    release
```

There are now two subdirectories, `debug/` and `release/`. Each holds
its own copy of the executable:

```console
$ ./target/release/hello_release
Hello, world!
$ ./target/debug/hello_release
Hello, world!
```

Same output — the program is too small for the optimizer to make a
visible difference, and timing the print would be too short to
measure anyway. The runtime difference would only show on real work
(the Book's reason for the two profiles). What you should believe
from this probe is the path and command shapes, not a speed claim.

For the parallel-directories observation, edit `src/main.rs` to print
something else and rebuild only the release profile:

```rust
fn main() {
    println!("Built optimized!");
}
```

```console
$ cargo build --release
   Compiling hello_release v0.1.0 (/path/to/hello_release)
    Finished `release` profile [optimized] target(s) in 1.38s
$ ./target/release/hello_release
Built optimized!
$ ./target/debug/hello_release
Hello, world!
```

The release binary picked up the edit; the debug binary did not,
because `cargo build --release` only rebuilt into `target/release/`.
Two parallel binaries, two parallel directories, one source file.

## What Changed

- `cargo build --release` is the optimized build. Output:
  `target/release/<name>`. `Finished` line: ``Finished `release`
  profile [optimized] target(s) in <time>s``.
- `cargo build` (no flag) remains the unoptimized dev build from
  lesson 064. Output: `target/debug/<name>`. `Finished` line:
  ``Finished `dev` profile [unoptimized + debuginfo] target(s) in
  <time>s``.
- The two profiles produce two parallel binaries from one source.
  The Book's framing: `dev` is for iteration (rebuild quickly and
  often); `release` is for the final program you give to a user
  (won't be rebuilt repeatedly, runs as fast as possible).
- If you want to measure how fast Rust code runs, the Book is
  explicit: build with `cargo build --release` and run the binary
  out of `target/release/`. Timing the dev binary measures the
  unoptimized build.

## Check Yourself

You are inside a fresh `greeter/` package from `cargo new greeter`.

- After `cargo build --release`, what is the path of the executable?
- What command (no `cargo`) runs that executable from inside
  `greeter/`?
- The `Finished` line names the profile and an optimization phrase
  in brackets. For `cargo build --release`, what are they?

(Answers: `target/release/greeter`. `./target/release/greeter`.
The profile is `release`; the bracketed phrase is `[optimized]`.)

## What To Ignore For Now

- The actual optimization level (`opt-level = 3`), the specific
  optimizations rustc applies (inlining, constant folding, dead-code
  elimination), and the `[profile.dev]` / `[profile.release]`
  manifest sections in `Cargo.toml` that let you tweak them. Treat
  the bracketed `[optimized]` and `[unoptimized + debuginfo]` as
  literal output text for now.
- Custom profiles (`[profile.bench]`, `[profile.test]`, user-defined
  profiles), `--profile <name>`, and the `cargo` reference's profiles
  page. Future move.
- `cargo run --release` and `cargo test --release`. The same
  `--release` flag composes onto other Cargo subcommands; today only
  installs the build form.
- The internals of `target/release/` beyond the executable itself
  (`deps/`, `.fingerprint/`, `build/`). Same deferral as lesson 064.
- `cargo bench`. Built-in benchmarking subcommand; out of scope for
  Ch1-3.
- Stripping symbols, link-time optimization (LTO), codegen-units, and
  other release-tuning knobs. Deferred.
- The `--release` shorthand `-r`. Cargo accepts it; minor, deferred.

## Evidence

See `../evidence/082-cargo-build-release.md`.
