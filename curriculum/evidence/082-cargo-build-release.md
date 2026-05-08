# Evidence — 082-cargo-build-release

Audit appendix for `lessons/082-cargo-build-release.md`. The lesson
teaches one move: `cargo build --release` compiles a package with
optimizations, writes the binary to `target/release/<name>` (parallel
to `target/debug/<name>` from lesson 064), and changes the `Finished`
line to read `release` profile and `[optimized]`. The lesson installs
one new concept: Cargo has two build profiles — `dev` (default,
unoptimized, fast rebuilds) and `release` (`--release` flag,
optimized, for shipping or benchmarking) — producing two parallel
binaries from the same source.

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
directory: `/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.E8ugZBTUpA/`.

## Project setup

The committed observation directory
`observations/082-cargo-build-release/` contains the post-probe
files:

- `Cargo.toml` — bit-exact match to what `cargo new --vcs none
  hello_release` writes:

  ```toml
  [package]
  name = "hello_release"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  ```

- `src/main.rs` — the *post-edit* program (the program that prints
  `Built optimized!`):

  ```rust
  fn main() {
      println!("Built optimized!");
  }
  ```

- `.gitignore` — listing `/target` and `Cargo.lock`. Build artifacts
  are not committed.

The original `cargo new`-generated `src/main.rs` (the canonical
`println!("Hello, world!")` template, captured in cycle 032 and 064)
is not committed; the post-edit form is the one load-bearing for the
"release rebuilt, debug did not" observation in transcript step 7.

## Probe transcript

All steps run from
`/private/var/folders/vc/cf1c1_d13nng8d7v388jh7380000gn/T/tmp.E8ugZBTUpA/`.

### Step 1: Toolchain capture and scaffold

```text
$ cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -sm
Darwin x86_64

$ cargo new --vcs none hello_release
    Creating binary (application) `hello_release` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

Bit-identical (modulo package name) to lesson 064 step 1. The
`--vcs none` form is the same one lesson 064's appendix
documents — it skips git scaffolding to keep the directory listing
minimal.

### Step 2: Inspect the generated package

```text
$ cd hello_release
$ ls -la
total 8
drwxr-xr-x  4 eli  staff  128 May  7 18:49 .
drwx------  3 eli  staff   96 May  7 18:49 ..
-rw-r--r--  1 eli  staff   84 May  7 18:49 Cargo.toml
drwxr-xr-x  3 eli  staff   96 May  7 18:49 src

$ cat Cargo.toml
[package]
name = "hello_release"
version = "0.1.0"
edition = "2024"

[dependencies]

$ cat src/main.rs
fn main() {
    println!("Hello, world!");
}
```

Bit-identical to lesson 064 step 2-3.

### Step 3: First `cargo build` (debug baseline)

```text
$ cargo build
   Compiling hello_release v0.1.0 (/private/var/folders/.../tmp.E8ugZBTUpA/hello_release)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.25s
exit=0
```

Lesson 064's central observation, reproduced unchanged on this host
and toolchain. The lesson uses this output as the contrast baseline
for step 4's `--release` build.

### Step 4: First `cargo build --release`

```text
$ cargo build --release
   Compiling hello_release v0.1.0 (/private/var/folders/.../tmp.E8ugZBTUpA/hello_release)
    Finished `release` profile [optimized] target(s) in 1.72s
exit=0
```

**Two changes from step 3's `Finished` line, in the same character
positions:**

| Field        | step 3 (`cargo build`)        | step 4 (`cargo build --release`) |
| ------------ | ----------------------------- | -------------------------------- |
| profile name | `` `dev` ``                   | `` `release` ``                  |
| bracketed    | `[unoptimized + debuginfo]`   | `[optimized]`                    |

Both are the literal output strings the lesson tells the learner to
expect. The `Compiling` line is identical in shape; only the
`Finished` line changes.

### Step 5: Inspect `target/`

```text
$ ls target/
CACHEDIR.TAG
debug
release

$ ls target/debug/
build
deps
examples
hello_release
hello_release.d
incremental

$ ls target/release/
build
deps
examples
hello_release
hello_release.d
incremental
```

Two parallel directories, `debug/` and `release/`, each containing
the same six entries. The lesson names only `target/release/<name>`
(the executable) and treats the rest as deferred (same deferral as
lesson 064 for `target/debug/`). `CACHEDIR.TAG` is a Cargo-wide
cache-system marker; not load-bearing for the lesson and not
mentioned.

### Step 6: Run the release binary

```text
$ ./target/release/hello_release
Hello, world!
exit=0
```

Same `./executable` shape lesson 064 already installed for
`./target/debug/hello_release`, applied at the parallel `release/`
path. Output is identical to the debug binary's because the program
is trivial — the lesson is explicit that this probe does not
demonstrate a runtime speed difference, only path and output-line
shapes.

### Step 7: Edit + release-only rebuild

`src/main.rs` was edited to the post-edit form (committed in the
observation directory):

```rust
fn main() {
    println!("Built optimized!");
}
```

```text
$ cargo build --release
   Compiling hello_release v0.1.0 (/private/var/folders/.../tmp.E8ugZBTUpA/hello_release)
    Finished `release` profile [optimized] target(s) in 1.38s
exit=0

$ ./target/release/hello_release
Built optimized!
exit=0

$ ./target/debug/hello_release
Hello, world!
exit=0
```

**Load-bearing observation for the "two parallel directories" claim:**
the release binary picked up the edit (`Built optimized!`); the debug
binary did not (still prints `Hello, world!`). `cargo build --release`
rebuilt only into `target/release/`; it did not touch
`target/debug/hello_release`. This is the empirical witness that the
two profiles produce two independent binaries from one source.

### Step 8: Cache-hit `cargo build --release`

Run between step 4 and step 7 (no source change since step 4):

```text
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.07s
exit=0
```

No `Compiling` line — same caching mechanism lesson 064 captured for
`cargo build`, applied here to `cargo build --release`. Not
load-bearing for today's lesson body (the lesson does not reteach
caching) but documented here for completeness.

## Corpus quote map

### `output/docs/rust/book/ch01-03-hello-cargo.md`

The Book's "Hello, Cargo!" chapter. The dedicated subsection
**"Building for Release"** is the canonical pedagogical source for
`cargo build --release`.

**Lines 203-213** (load-bearing — the entire section):

> ### [Building for Release](#building-for-release)
>
> When your project is finally ready for release, you can use
> `cargo build --release` to compile it with optimizations. This
> command will create an executable in *target/release* instead of
> *target/debug*. The optimizations make your Rust code run faster,
> but turning them on lengthens the time it takes for your program
> to compile. This is why there are two different profiles: one for
> development, when you want to rebuild quickly and often, and
> another for building the final program you'll give to a user that
> won't be rebuilt repeatedly and that will run as fast as possible.
> If you're benchmarking your code's running time, be sure to run
> `cargo build --release` and benchmark with the executable in
> *target/release*.

This single paragraph grounds every substantive Book-attributable
claim in the lesson:

- "`cargo build --release` ... compile it with optimizations" →
  *The Move* and *What Changed* bullet 1.
- "create an executable in *target/release* instead of *target/debug*"
  → *The Move*'s `target/release/<name>` path; *What Changed*
  bullet 1; the *Try It* `ls target/` and `./target/release/...`
  steps.
- "two different profiles: one for development ... another for
  building the final program" → *Mental Model Delta*'s "two profiles"
  framing; *What Changed* bullet 3.
- "rebuild quickly and often" / "won't be rebuilt repeatedly and ...
  run as fast as possible" → *What Changed* bullet 3's wording
  ("iteration" / "shipping").
- "benchmarking your code's running time, be sure to run
  `cargo build --release` and benchmark with the executable in
  *target/release*" → *What Changed* bullet 4 (the benchmarking
  rule).

The "optimizations make your Rust code run faster, but ... lengthens
the time it takes for your program to compile" sentence is the Book's
*reason* for the two profiles; the lesson restates it inside the
"`dev` for iteration / `release` for shipping" mental model. The
lesson does not claim a measured speed improvement on its own probe
because the trivial `println!` program does not exercise any
optimizable code; the lesson is explicit about this.

### `output/docs/rust/cargo/commands/cargo-build.md`

The reference man-page for `cargo build`. Used as a corroborating
source for the `--release` flag's documented effect.

**Lines 165-167** (corroborating):

> [`--release`](#option-cargo-build---release)
> :   Build optimized artifacts with the `release` profile.
>     See also the `--profile` option for choosing a specific profile by name.

The lesson does not cite this directly; it appears here as a second
witness that "`--release` selects the `release` profile and produces
optimized artifacts" is documented in two independent corpus files
(the Book chapter and the man-page), not just one.

**Lines 169-171** (deferred — the `--profile <name>` flag):

> [`--profile` *name*](#option-cargo-build---profile)
> :   Build with the given profile.
>     See [the reference](../reference/profiles.md) for more details on profiles.

The general `--profile <name>` mechanism (and custom profiles) is
named in *What To Ignore* and is the natural unlock for the
profiles-deep-dive future move.

## Prerequisite-claim summary

### From lesson 064 (`064-cargo-build-standalone`) — *direct, load-bearing*

- `cargo build` from inside a Cargo package directory compiles
  `src/main.rs` and writes the executable to `target/debug/<name>`,
  printing two lines: `Compiling <name> v0.1.0 (<path>)` and
  ``Finished `dev` profile [unoptimized + debuginfo] target(s) in
  <time>s``. Today's `cargo build --release` produces the same
  `Compiling` line shape; the `Finished` line's profile name and
  bracketed phrase are the two characters-in-place changes.
- The binary at `target/debug/<name>` is run with
  `./target/debug/<name>` — the same `./executable` shape as lesson
  001 at a deeper path. Today's `./target/release/<name>` is the
  parallel application at the parallel path.
- Lesson 064's *What To Ignore* explicitly defers ``the meaning of
  `dev` in `Finished `dev` profile``, naming "the contrast with
  `release`, `cargo build --release`, optimization levels, the
  `[profile.*]` manifest sections" as a future move. Today's lesson
  is exactly that future move (the orchestrator's queue item J),
  scoped to the `--release` flag and the `target/release/` path
  only — the deeper details (`opt-level`, `[profile.*]` syntax,
  custom profiles, `--profile <name>`) remain deferred.

### From lesson 032 (`032-cargo-new-and-run`) — *supporting*

Mentioned by number/title only. Cycle 032 grounds `cargo new <name>`
scaffolding the package with `Cargo.toml` and `src/main.rs`. Today's
probe uses `cargo new --vcs none hello_release`; the `--vcs none`
form is documented in lesson 064's appendix.

### From lesson 001 (`001-rustc-compile-and-run`) — *supporting*

Mentioned by number/title only. Lesson 001 grounds the
`./executable` shape; lesson 064 already restates the load-bearing
form (`./target/debug/<name>`). Today's `./target/release/<name>` is
the same shape with a different middle directory.

### From lessons 002 and 011 — *supporting*

Mentioned by number/title only. Lesson 002 grounds `fn main()`
running when the executable launches; lesson 011 grounds `println!`
printing to stdout. Both are restated by lesson 064's prerequisites
and not load-bearing in any way not already covered there.

## Contrast-probe coverage

The lesson's central contrastive claim has two parts:

1. **`Finished` line shape:** `dev` / `[unoptimized + debuginfo]`
   versus `release` / `[optimized]`. Steps 3 and 4 sit
   side-by-side as the empirical witness — same package, same
   source, same fresh-cache state, only the `--release` flag
   differs. Both are valid commands producing valid output; this is
   a contrast between two working forms, not a working-vs-broken
   contrast, so no negative probe is needed.

2. **Two parallel directories from one source:** `cargo build`
   writes to `target/debug/<name>`; `cargo build --release` writes
   to `target/release/<name>`. Step 7 is the empirical witness: the
   release rebuild after a source edit changed only
   `target/release/hello_release` (now prints `Built optimized!`)
   while leaving `target/debug/hello_release` untouched (still
   prints `Hello, world!`). The "untouched" debug binary is the
   negative-side observation that strengthens the
   parallel-directories claim — it would be weaker if the lesson
   only showed both paths existing without showing they are
   independent.

The lesson does not make a runtime-speed claim on this probe (the
`println!` program is too small for the optimizer to make a visible
difference, and the Book's whole point is that timing matters only
for non-trivial code). The Book's "make your Rust code run faster"
sentence is restated in the *What Changed* mental model, not as a
probe observation.

## What is *not* in this probe

The probe deliberately does not exercise:

- The `[profile.dev]` / `[profile.release]` manifest sections.
  Profile-tuning is deferred; the bracketed phrases `[optimized]`
  and `[unoptimized + debuginfo]` are treated as literal output
  strings.
- The `opt-level` setting and the specific optimizations rustc
  applies. Compiler-internal; deferred.
- `cargo run --release`, `cargo test --release`. Same `--release`
  flag composing onto other subcommands; explicitly deferred.
- `cargo bench`, custom profiles, `--profile <name>`,
  user-defined profiles. Deferred.
- A measured speed comparison between `target/debug/` and
  `target/release/` binaries. The probe's program is too trivial;
  the Book's framing (timing is meaningful from `target/release/`
  only) is restated as a *rule*, not demonstrated as a measurement.
- LTO, codegen-units, symbol stripping, and other release-tuning
  knobs.
- The `-r` shorthand for `--release`.
- Windows path separators (cycle 064's deferral preserved).

## Files committed for this cycle

- `lessons/082-cargo-build-release.md` (this lesson)
- `evidence/082-cargo-build-release.md` (this appendix)
- `observations/082-cargo-build-release/Cargo.toml`
- `observations/082-cargo-build-release/src/main.rs`
- `observations/082-cargo-build-release/.gitignore`
- updated `graph.md` (a new draft node block under `## Draft Nodes`)
