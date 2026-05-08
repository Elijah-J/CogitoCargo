# Evidence — 064-cargo-build-standalone

Audit appendix for `lessons/064-cargo-build-standalone.md`. The lesson
teaches one move: `cargo build` compiles a package and writes the
binary to `target/debug/<name>` *without* executing it; the binary is
then run manually with `./target/debug/<name>`.

This appendix covers (a) toolchain and reproducibility, (b) corpus
quote map, (c) the verbatim probe transcript, (d) project setup and
the committed observation files, (e) the prerequisite-claim summary,
and (f) the calibration `cargo run` probe used to anchor the
contrast.

## Toolchain

```
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
binary: rustc
commit-hash: 59807616e1fa2540724bfbac14d7976d7e4a3860
commit-date: 2026-04-14
host: x86_64-apple-darwin
release: 1.95.0
LLVM version: 22.1.2
```

`uname -sm` -> `Darwin x86_64`. Probe ran in a fresh `mktemp -d`
directory.

## Project setup

The committed observation directory
`observations/064-cargo-build-standalone/` contains exactly the files
that are load-bearing for the lesson:

- `Cargo.toml` — bit-exact match to what `cargo new --vcs none
  hello_build` writes:

  ```toml
  [package]
  name = "hello_build"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  ```

- `src/main.rs` — the *post-edit* program (the program that prints
  `Built but not run!`):

  ```rust
  fn main() {
      println!("Built but not run!");
  }
  ```

- `.gitignore` — listing `target/` and `Cargo.lock`. These are build
  artifacts; the orchestrator directive explicitly limits the commit
  to `Cargo.toml` + `src/main.rs` + (optional) `.gitignore`.

The original `cargo new`-generated `src/main.rs` (the canonical
`println!("Hello, world!")` template, already captured in cycle
032's appendix) is not committed; the post-edit form is the one
load-bearing for the build-after-edit observation in transcript step
8 below.

## Probe transcript

Steps 1-12 from the orchestrator's probe shape, run in
`/private/var/folders/.../tmp.tDlrYxvzJv/`.

### Steps 1-3: Setup and toolchain capture

```text
$ cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)

$ cargo new --vcs none hello_build
    Creating binary (application) `hello_build` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

`cargo new --vcs none` skips the git scaffolding (`.git/`,
`.gitignore`) that lesson 032 captured; everything else is identical
to lesson 032's transcript. The orchestrator chose `--vcs none` for
this lesson to keep the directory listing minimal — the cargo book
documents the flag at `cargo-new.md` in the OPTIONS / Manifest
Options section.

### Steps 3-4: Verify the generated package

```text
$ ls -la hello_build/
total 8
drwxr-xr-x  4 eli  staff  128 May  7 12:22 .
drwx------  3 eli  staff   96 May  7 12:22 ..
-rw-r--r--  1 eli  staff   82 May  7 12:22 Cargo.toml
drwxr-xr-x  3 eli  staff   96 May  7 12:22 src

$ cat hello_build/Cargo.toml
[package]
name = "hello_build"
version = "0.1.0"
edition = "2024"

[dependencies]

$ cat hello_build/src/main.rs
fn main() {
    println!("Hello, world!");
}
```

Bit-identical to lesson 032's transcript modulo `--vcs none` (no
`.git/`, no `.gitignore`).

### Step 5: First `cargo build`

```text
$ cd hello_build
$ cargo build
   Compiling hello_build v0.1.0 (/private/var/folders/.../tmp.tDlrYxvzJv/hello_build)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.70s
exit=0
```

**Two output lines, not three.** The lesson's load-bearing
observation: there is no `Running \`target/debug/hello_build\`` line
that `cargo run` would produce. Cargo stops after `Finished`.

The first line `Compiling hello_build v0.1.0 (...)` is bit-identical
to lesson 032's `cargo run` first line. The second line `Finished
\`dev\` profile [unoptimized + debuginfo] target(s) in X.XXs` is
bit-identical to lesson 032's second line. The lesson does not
explain `dev` beyond naming it as the literal output line; deferred.

### Step 6: Inspect `target/debug/`

```text
$ ls target/debug/
build
deps
examples
hello_build
hello_build.d
incremental
```

The lesson names only `hello_build` (the executable). Everything
else (`build`, `deps`, `examples`, `hello_build.d`, `incremental`)
is Cargo's build cache and is deferred. The Book's load-bearing
sentence at ch01-03 line 129-130 is exactly: "This command creates
an executable file in *target/debug/hello_cargo* (or
*target\debug\hello_cargo.exe* on Windows) rather than in your
current directory."

### Step 7: Run the binary manually

```text
$ ./target/debug/hello_build
Hello, world!
exit=0
```

This is lesson 001's `./executable` shape applied at the deeper
path. The Book's load-bearing line at ch01-03 line 134-136: "You
can run the executable with this command: `$ ./target/debug/hello_cargo` ...
`Hello, world!`". Bit-identical to today's transcript.

### Steps 8-10: Edit + rebuild + run

`src/main.rs` was edited to:

```rust
fn main() {
    println!("Built but not run!");
}
```

(This is the version committed in the observation directory.)

```text
$ cargo build
   Compiling hello_build v0.1.0 (/private/var/folders/.../tmp.tDlrYxvzJv/hello_build)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
exit=0

$ ./target/debug/hello_build
Built but not run!
exit=0
```

Cargo recompiled (`Compiling` line present) and the binary's output
is the new string — empirical witness that `cargo build` overwrote
the binary at `target/debug/hello_build`. No `Running` line on the
build, just like step 5.

### Step 11: Cache-hit `cargo build`

```text
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
exit=0
```

**No `Compiling` line.** The lesson's third reading-shape
observation: when source has not changed since the last build,
Cargo prints only `Finished`. This matches the Book's wording at
ch01-03 lines 158-160: "Cargo figured out that the files hadn't
changed, so it didn't rebuild but just ran the binary." (The Book's
sentence is in the `cargo run` paragraph; today is its `cargo build`
counterpart.)

### Step 12 (Calibration): `cargo run` for contrast

```text
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello_build`
Built but not run!
exit=0
```

Two load-bearing observations:

1. The third line `Running \`target/debug/hello_build\`` is present
   under `cargo run` and absent under `cargo build`. Same package,
   same binary, same fresh-cache state — the only difference is the
   command. This is the empirical witness for the lesson's repeated
   "no `Running` line" claim.
2. The path `target/debug/hello_build` Cargo announces in the
   `Running` line is exactly the path the lesson tells the learner
   to invoke manually. Cargo's `Running` line names the same
   executable path that `./target/debug/hello_build` runs.

## Corpus quote map

### `output/docs/rust/book/ch01-03-hello-cargo.md`

The Book's "Hello, Cargo!" chapter. Used by lesson 032 and (now)
the canonical pedagogical source for `cargo build` itself.
Section: **"Building and Running a Cargo Project"** (header at line
117).

**Lines 119-127** — the `cargo build` invocation and its output
shape (load-bearing for transcript step 5):

> Now let's look at what's different when we build and run the
> "Hello, world!" program with Cargo! From your *hello_cargo*
> directory, build your project by entering the following command:
>
> ```console
> $ cargo build
>    Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
>     Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
> ```

The lesson's transcript reproduces this two-line shape (with a
post-rustc-1.95 `dev` profile spelling — `Finished \`dev\` profile`
in 2026 vs. `Finished dev` in the Book's older capture; the
substantive shape is identical: Compiling line, Finished line, no
Running line).

**Lines 129-131** — where the binary lives (load-bearing for the
lesson's `target/debug/<name>` claim):

> This command creates an executable file in *target/debug/hello_cargo*
> (or *target\debug\hello_cargo.exe* on Windows) rather than in
> your current directory. Because the default build is a debug
> build, Cargo puts the binary in a directory named *debug*.

The lesson uses the form `target/debug/<name>` rather than the
literal `target/debug/hello_cargo` to make the rule reusable; both
forms are equivalent.

**Lines 132-137** — the manual run step (load-bearing for the
lesson's `./target/debug/<name>` step):

> You can run the executable with this command:
>
> ```console
> $ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
> Hello, world!
> ```

This is the literal command the lesson tells the learner to type;
bit-identical except for `<name>` substitution.

**Lines 145-156** — the `cargo run` contrast and its three-line
output shape (load-bearing for the lesson's "no `Running` line"
claim being a contrast claim, not a free-standing one):

> We just built a project with `cargo build` and ran it with
> `./target/debug/hello_cargo`, but we can also use `cargo run` to
> compile the code and then run the resultant executable all in one
> command:
>
> ```console
> $ cargo run
>     Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
>      Running `target/debug/hello_cargo`
> Hello, world!
> ```

The Book's framing of `cargo run` as "all in one command" after the
two-step `cargo build` + `./target/debug/hello_cargo` walk is
exactly the lesson's mental-model delta: today is the build half;
`cargo run` is build + run.

**Lines 158-162** — the cache-hit observation (load-bearing for
transcript step 11):

> Notice that this time we didn't see output indicating that Cargo
> was compiling `hello_cargo`. Cargo figured out that the files
> hadn't changed, so it didn't rebuild but just ran the binary. If
> you had modified your source code, Cargo would have rebuilt the
> project before running it.

The Book's sentence is in the `cargo run` paragraph but applies
identically to `cargo build` (the cache mechanism is shared); the
probe's step-11 transcript is the empirical witness for the
`cargo build` case.

**Lines 189-197** — the recap that names the relationship between
`cargo build`, `cargo run`, `cargo check` (load-bearing for the
*Mental Model Delta* and *What Changed*):

> Let's recap what we've learned so far about Cargo:
>
> - We can create a project using `cargo new`.
> - We can build a project using `cargo build`.
> - We can build and run a project in one step using `cargo run`.
> - We can build a project without producing a binary to check for
>   errors using `cargo check`.
> - Instead of saving the result of the build in the same directory
>   as our code, Cargo stores it in the *target/debug* directory.

`cargo check` appears here in the Book; the lesson defers it to a
future move (per orchestrator directive).

**Lines 203-213** — the `--release` deferral (NOT used by the
lesson body except as a *What To Ignore*):

> When your project is finally ready for release, you can use
> `cargo build --release` to compile it with optimizations. This
> command will create an executable in *target/release* instead of
> *target/debug*. ... This is why there are two different profiles:
> one for development ... and another for building the final program
> you'll give to a user.

The lesson explicitly defers profiles, the `--release` flag, and
the `dev` vs `release` distinction. This corpus paragraph is the
source for the deferral — the lesson does not unpack it.

### `output/docs/rust/cargo/commands/cargo-build.md`

The reference man-page for `cargo build`. Two short quotes:

**Lines 6 and 13-14** (NAME and DESCRIPTION):

> cargo-build — Compile the current package
>
> ## DESCRIPTION
>
> Compile local packages and all of their dependencies.

This is the source for the lesson's claim that `cargo build`
compiles the current package and stops. The `and all of their
dependencies` clause is moot today (no dependencies in the probe);
referenced only as part of the *What To Ignore* deferral.

### `output/docs/rust/cargo/commands/cargo-run.md`

Already cited in lesson 032; today's lesson uses it only by
reference. The relevant fact (`cargo run` runs the binary after
building) is cycled forward from cycle 032 — the contrast that
makes `cargo build`'s "no `Running` line" observation meaningful.

## Prerequisite-claim summary (1-3 bullets each per direct prerequisite)

### From lesson 032 (`032-cargo-new-and-run`) — *direct, load-bearing*

- `cargo new <name>` scaffolds a package directory with `Cargo.toml`
  and `src/main.rs`. Today's probe uses `cargo new --vcs none
  hello_build`; everything else (the manifest contents, the default
  `src/main.rs`) is bit-identical to cycle 032's setup.
- `cargo run` from inside the package directory compiles `src/main.rs`
  and runs the resulting binary, printing three cargo-narration lines
  (`Compiling`, `Finished`, `Running`) followed by program output.
  Today's lesson is the build half: `cargo build` produces the same
  first two lines and *omits* the third.
- The compiled executable lives at `target/debug/<name>`. Cycle 032's
  evidence appendix already captured this path in the
  `Running \`target/debug/hello_cargo\`` line of its transcript;
  today's lesson elevates it from "cargo's internal detail" to "the
  path you yourself invoke."

### From lesson 001 (`001-rustc-compile-and-run`) — *direct,
load-bearing*

- After compilation produces an executable, running the executable
  is a separate command — `./executable` from the same directory.
  Today's lesson reuses that shape literally: `./target/debug/<name>`
  is the same `./<path-to-binary>` invocation, just with a deeper
  path.

### From lesson 002 (`002-fn-main-entry-point`) and lesson 011 (`011-println-positional-args`) — *supporting*

Mentioned by number/title only. Cycle 002 grounds the claim that
`fn main()`'s body runs when the executable launches; cycle 011
grounds `println!`'s print-to-stdout behavior. Neither claim is
load-bearing in a way not already restated by cycle 032's
prerequisite claims.

## Contrast-probe coverage

The lesson's central contrastive claim is "`cargo build` produces no
`Running` line, whereas `cargo run` does." Step 12 of the probe is
the calibration probe: same package, same binary, same fresh-cache
state, only the command differs. The two transcripts (step 5 and
step 12) sit side-by-side as the empirical witness:

- Step 5 output: `Compiling`, `Finished`. (No `Running`.)
- Step 12 output: `Finished`, `Running \`target/debug/hello_build\``,
  `Built but not run!`. (No `Compiling` because the cache is fresh.)

The third line absence-vs-presence is the contrastive observation;
no separate broken-contrast probe is needed because the contrast is
between two valid commands, not between a working and a broken form.

## Notes on caching

Step 11's "cache-hit" observation is identical in mechanism to cycle
032's "second `cargo run` skips the `Compiling` line" observation
(captured in cycle 032's appendix). Cycle 032 made the observation
about `cargo run`; today the same caching applies to `cargo build`.
The Book's wording (lines 158-162) attributes the behavior to "the
files hadn't changed" — the rule is about source files, not about
which build command invoked it.

The 0.20s rebuild time in step 8-9 (after editing one line of one
source file) is faster than the 0.70s first build but still does
real work — Cargo recompiles the changed source file and links a
new binary. The 0.00s in step 11 (no source change) is Cargo
deciding the binary is already up to date.

## What is *not* in this probe

The probe deliberately does not exercise:

- `cargo build --release`. Profile-related; deferred per orchestrator.
- `cargo check`. Sibling command; deferred.
- Multi-file packages, `lib.rs`, modules, dependencies. None of them
  are load-bearing for today's move.
- `cargo clean`. Would reset the cache and re-trigger a full
  `Compiling` line; the cache-hit observation in step 11 is the
  positive form of the same fact.
- Interactive vs piped stdin. The program prints to stdout and
  exits; no input.

## Files committed for this cycle

- `lessons/064-cargo-build-standalone.md` (this lesson)
- `evidence/064-cargo-build-standalone.md` (this appendix)
- `observations/064-cargo-build-standalone/Cargo.toml`
- `observations/064-cargo-build-standalone/src/main.rs`
- `observations/064-cargo-build-standalone/.gitignore`
- updated `graph.md` (a new draft node block under `## Draft Nodes`)
