---
id: 032-cargo-new-and-run
move: "create a Rust package with `cargo new <name>` and run its program with `cargo run`"
main_concept: "cargo is Rust's package manager and build tool; `cargo new <name>` scaffolds a directory <name>/ containing a `Cargo.toml` manifest and a `src/main.rs` source file (with a default `fn main` that prints `Hello, world!`); `cargo run` from inside that directory compiles `src/main.rs` and runs the resulting executable in one command, replacing lesson 001's two-step `rustc file.rs` then `./file` workflow"
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, Linux/macOS shell)
  - cargo is installed and on your PATH (it ships with every official Rust install via rustup)
unlocks:
  - future "Cargo.toml manifest format" moves
  - future "cargo build (just build, no run)" moves
  - future "cargo check (typecheck only)" moves
  - future "cargo test" moves
  - future "cargo add (manage dependencies)" moves
  - future "cargo new --lib (library packages)" moves
  - future "editions" moves
  - future "crates.io and dependency resolution" moves
  - future "release vs dev profiles (--release)" moves
sources:
  - output/docs/rust/book/ch01-03-hello-cargo.md
  - output/docs/rust/cargo/getting-started/first-steps.md
  - output/docs/rust/cargo/commands/cargo-new.md
  - output/docs/rust/cargo/commands/cargo-run.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/032-cargo-new-and-run.rs
status: accepted
---

# Create a package with `cargo new`, run it with `cargo run`

## The Move

Run `cargo new hello_cargo` in any directory. A new folder
`hello_cargo/` appears, pre-populated with a small set of files
(`Cargo.toml`, `src/main.rs`, and a git scaffold). `cd` into that
folder and run `cargo run`. Cargo compiles the program and prints
`Hello, world!` to your terminal — all in one command. You have just
used Rust's standard project-management workflow for the first time.

## Mental Model Delta

- Before: "to build and run a Rust program I write a `.rs` file, run
  `rustc file.rs` to make an executable, then run `./file` (lesson
  001). That's two commands and they only handle one source file."
- After: "for any Rust project bigger than a single tutorial file, the
  standard workflow is *cargo*, Rust's package manager and build tool.
  `cargo new <name>` scaffolds a small directory with conventional
  layout: a `Cargo.toml` *manifest* describing the package (name,
  version, edition, dependencies) and a `src/main.rs` source file with
  a default `fn main` already in it. `cargo run` from inside the
  package directory compiles that `src/main.rs` and runs the result in
  one command. The `fn main`-runs-when-the-executable-launches rule
  from lesson 002 still applies; cargo is just a higher-level wrapper
  around rustc plus a project layout convention."

## Prerequisites

- Installed concepts:
  - From lesson 001 (`001-rustc-compile-and-run`): a `.rs` file is a
    Rust source file; compiling Rust produces an executable; compiling
    and running are conceptually distinct steps. This lesson contrasts
    cargo's one-step `cargo run` against that two-step workflow.
  - From lesson 002 (`002-fn-main-entry-point`): `fn main() { ... }`
    is the entry point — the body inside the curly braces is what runs
    when the executable launches. The `src/main.rs` file cargo
    generates contains exactly that shape, and `cargo run` runs it the
    same way.
- Ordinary computer-use assumptions:
  - same terminal/editor/shell assumptions as lesson 001;
  - `cargo` is installed and on your `PATH`. If you installed Rust via
    the official `rustup` installer (the recommended path), cargo came
    with it. Check by running `cargo --version`; if it prints a version
    number you are set.

## Try It

Pick any directory you can write to (your home directory works). Open
a terminal there and run:

```console
$ cargo new hello_cargo
    Creating binary (application) `hello_cargo` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

A new directory `hello_cargo/` now exists. Look inside it:

```console
$ ls -F hello_cargo/
.git/        .gitignore   Cargo.toml   src/
```

Two of those entries matter for this lesson:

- **`Cargo.toml`** — the *manifest*. A small text file describing the
  package: its name, version, Rust edition, and (eventually) any
  external libraries it depends on. The default contents are:

  ```toml
  [package]
  name = "hello_cargo"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  ```

  You will edit this file later when you want to add dependencies. For
  now just notice it exists.

- **`src/main.rs`** — the actual source file, with a default
  `fn main` already written for you:

  ```rust
  fn main() {
      println!("Hello, world!");
  }
  ```

  Cargo expects your source files to live inside `src/`; the top-level
  package directory is for the manifest, README files, and the like.

(`.git/` and `.gitignore` are git's bookkeeping. Cargo initializes a
git repository by default. You can ignore those for this lesson.)

Now run the program:

```console
$ cd hello_cargo
$ cargo run
   Compiling hello_cargo v0.1.0 (/path/to/hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.74s
     Running `target/debug/hello_cargo`
Hello, world!
```

The first three lines are cargo narrating its progress: it compiled
the package (using rustc internally), finished, then ran the resulting
binary. The fourth line, `Hello, world!`, is your program's actual
output — the `println!` from `src/main.rs`.

Cargo also created a new sibling directory called `target/`. That is
cargo's build cache; the compiled executable lives at
`target/debug/hello_cargo`. You normally do not look inside `target/`
or run that path directly — `cargo run` handles it. Cargo also creates
a `Cargo.lock` file alongside `Cargo.toml` on first build; it records
the exact versions of dependencies for reproducible builds. You do not
edit `Cargo.lock` by hand — cargo manages it for you.

## What Changed

- You can scaffold a fresh Rust project with one command:
  `cargo new <name>`.
- You can recognize cargo's standard layout: `<name>/Cargo.toml` (the
  manifest) and `<name>/src/main.rs` (the entry-point source).
- You can build and run a cargo project in one step with `cargo run`,
  instead of lesson 001's two-step `rustc` + `./executable`.
- You have a name for the tool (*cargo*) and the file format (*manifest*,
  in TOML) and the layout (`src/` for sources, `target/` for build
  artifacts).
- You know cargo is the standard workflow for any non-trivial Rust
  code; lesson 001's direct `rustc` is fine for one-file experiments,
  but cargo is what you use otherwise.

## Check Yourself

You run `cargo new greeter` in your home directory.

- What new directory appears, and what are the two files inside it
  that this lesson named?
- What is the shape of the `fn main` that cargo wrote into
  `src/main.rs`?
- What single command, run from inside `greeter/`, will compile that
  program and print its output?
- If you wanted to compile and run the same program with only `rustc`
  (lesson 001's workflow), which file would you point `rustc` at?

(Answers: `greeter/`, containing `Cargo.toml` (the manifest) and
`src/main.rs` (the source). `fn main() { println!("Hello, world!"); }`.
`cargo run`. `rustc src/main.rs` — though you would usually
`cd src/` first or pass an output path; the simpler answer is "use
`cargo run` instead.")

## What To Ignore For Now

This lesson installs only the cargo workflow shape: `cargo new <name>`
to scaffold, `cargo run` from inside to build-and-run. Each of the
following is real and will be taught later, but is *not* part of this
move:

- The detailed shape of `Cargo.toml`. The `[package]` and
  `[dependencies]` headings, the `name`/`version`/`edition` keys, the
  TOML format itself — all noted at high level only. Future moves will
  install them properly.
- Adding dependencies (with `cargo add <crate>` or by editing
  `Cargo.toml`'s `[dependencies]` section). The whole point of cargo
  beyond simple builds; future move.
- `cargo build` (just build, no run), `cargo check` (typecheck only,
  no codegen), `cargo test`, `cargo doc`, `cargo bench`. Mentioned in
  the corpus alongside `cargo run`; deferred as their own moves.
- `cargo new --lib` for a library package (`src/lib.rs` instead of
  `src/main.rs`). Future move.
- *Editions* (the `edition = "2024"` line in `Cargo.toml`). A versioning
  mechanism for the Rust language itself; deferred until libraries
  matter.
- Build profiles. The `dev` profile (default, unoptimized, with debug
  info) versus the `release` profile (`cargo run --release`,
  optimized). Mentioned in the cargo output but not installed.
- The internal layout of `target/`. Implementation detail of cargo's
  build cache.
- *Crates*. The corpus uses the word *crate* for "a Rust compilation
  unit / package of code"; this lesson uses *package* and avoids
  *crate* as a separate concept until it earns its keep.
- `crates.io` and the broader ecosystem of public Rust libraries.
  Future move.
- Workspaces (multiple packages sharing one build). Future move.
- Git itself. `cargo new` creates `.git/` and `.gitignore` because
  cargo defaults to initializing a git repository, but git is its own
  toolchain and is not a Rust concept.
- Windows shell specifics. The cargo commands themselves are
  cross-platform (the corpus explicitly says so), but the path
  separators in shown output are macOS/Linux style.

## Evidence

### Sources

- `output/docs/rust/book/ch01-03-hello-cargo.md` — the Book's
  "Hello, Cargo!" chapter, the canonical pedagogical introduction.
  Direct quote: "Cargo is Rust's build system and package manager."
  Direct quote on `cargo new`: "The first command creates a new
  directory and project called *hello_cargo*. We've named our project
  *hello_cargo*, and Cargo creates its files in a directory of the
  same name." Direct quote on the layout: "Cargo has generated two
  files and one directory for us: a *Cargo.toml* file and a *src*
  directory with a *main.rs* file inside. It has also initialized a
  new Git repository along with a *.gitignore* file." Direct quote on
  `cargo run`: "we can also use `cargo run` to compile the code and
  then run the resultant executable all in one command." Direct quote
  on `src/main.rs`: shows the exact `fn main() { println!("Hello, world!"); }`
  template this lesson references.
- `output/docs/rust/cargo/getting-started/first-steps.md` — the cargo
  book's own tutorial. Direct quote: "To start a new package with
  Cargo, use `cargo new`". Direct quote naming the manifest: "Let's
  check out `Cargo.toml`: ... This is called a [***manifest***], and
  it contains all of the metadata that Cargo needs to compile your
  package." This is the source for the term *manifest* the lesson
  uses, and it is the cargo book's own authoritative naming.
- `output/docs/rust/cargo/commands/cargo-new.md` — the man-page for
  `cargo new`. Direct quote in DESCRIPTION: "This command will create
  a new Cargo package in the given directory. This includes a simple
  template with a `Cargo.toml` manifest, sample source file, and a
  VCS ignore file. If the directory is not already in a VCS
  repository, then a new repository is created". Direct quote on the
  default: "Create a package with a binary target (`src/main.rs`).
  This is the default behavior." This is the source for the lesson's
  claim that the default is a binary package with `src/main.rs`.
- `output/docs/rust/cargo/commands/cargo-run.md` — the man-page for
  `cargo run`. Direct quote in DESCRIPTION: "Run a binary or example
  of the local package." Direct quote on default behavior: "When no
  target selection options are given, `cargo run` will run the binary
  target." This is the source for the lesson's claim that bare
  `cargo run` builds and runs the package's `src/main.rs` binary.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/032-cargo-new-and-run.rs`.
The committed file is the exact `src/main.rs` contents that
`cargo new hello_cargo` generates — a verbatim Hello-world template
plus comments documenting the workflow. The lesson's actual probe is
a shell session, captured in the transcript below.

Probe transcript, run in a clean temp directory created with
`mktemp -d`:

```text
--- cargo --version ---
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64

--- workdir: /var/folders/.../tmp.mvhqUkJElV ---
--- cargo new hello_cargo ---
    Creating binary (application) `hello_cargo` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

--- ls -la hello_cargo/ ---
total 16
drwxr-xr-x  6 eli  staff  192 May  6 23:51 .
drwx------  3 eli  staff   96 May  6 23:51 ..
drwxr-xr-x  9 eli  staff  288 May  6 23:51 .git
-rw-r--r--  1 eli  staff    8 May  6 23:51 .gitignore
-rw-r--r--  1 eli  staff   82 May  6 23:51 Cargo.toml
drwxr-xr-x  3 eli  staff   96 May  6 23:51 src

--- cat hello_cargo/Cargo.toml ---
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]

--- cat hello_cargo/src/main.rs ---
fn main() {
    println!("Hello, world!");
}

--- cd hello_cargo && cargo run ---
   Compiling hello_cargo v0.1.0 (/private/var/folders/.../tmp.mvhqUkJElV/hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.74s
     Running `target/debug/hello_cargo`
Hello, world!
exit=0
```

Notes from the transcript:

- `cargo new hello_cargo` creates exactly what the corpus describes:
  a `Cargo.toml`, a `src/` directory, and a git scaffold (`.git/`,
  `.gitignore`). The `Cargo.toml` contents match the cargo book's
  example verbatim — `[package]` with `name`/`version`/`edition`,
  then an empty `[dependencies]` section.
- The default `src/main.rs` template is `fn main() { println!("Hello, world!"); }`,
  the same Hello-world program as lesson 001's probe (lesson 001 used
  the variant string `"hello from rustc"` to disambiguate; the cargo
  default uses the canonical `"Hello, world!"`).
- `cargo run` from inside `hello_cargo/` produces three cargo-narration
  lines (`Compiling`, `Finished`, `Running`) followed by the program's
  own output `Hello, world!`. Exit code is 0.
- The compile time (`0.74s` here) is environment-dependent and will
  vary; the rest of the transcript is reproducible. The absolute path
  printed in `Compiling` and `Running` lines reflects the temp
  directory and is also environment-dependent.
- The compiled executable lives at
  `hello_cargo/target/debug/hello_cargo` (per the cargo book and the
  `Running \`target/debug/hello_cargo\`` line in the transcript).
  Lesson does not install this path as a working surface; cargo run
  handles it.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — installs the two-step
  `rustc file.rs` then `./file` workflow that this lesson contrasts
  against. Specifically: a `.rs` file is a Rust source file, compiling
  produces an executable, and running the executable is a separate
  step. Cargo collapses those into one `cargo run` command with a
  conventional layout.
- `002-fn-main-entry-point` (accepted) — installs `fn main() { ... }`
  as the entry point: the body runs when the executable launches.
  The default `src/main.rs` template cargo writes is exactly that
  shape, so when `cargo run` invokes the compiled binary, control
  enters `main`'s body just as in lesson 002.
