---
id: 087-rustfmt
status: accepted
evidence: ../evidence/087-rustfmt.md
---

# Reformat the package to standard style: `cargo fmt`

## The Move

From inside a Cargo package directory (the same setup as lesson 064:
`Cargo.toml` and `src/`, scaffolded by `cargo new`), run `cargo fmt`.
Cargo invokes `rustfmt` on the package's Rust source files and rewrites
them to the *community standard* style — conventional indentation,
spacing, line breaks, brace placement. Two facts anchor today: the
underlying tool is bundled with the standard Rust toolchain (no separate
install — the same rustup that delivered `rustc` and `cargo` also
delivered `rustfmt` and `cargo-fmt`), and the rewrite changes only
*style* — what the program *does* is unchanged.

## Mental Model Delta

- *Before:* "I write Rust in whatever style I happened to type. The
  Book has named good style for one specific thing — for example,
  Ch1-2 line 112 says 'It's good style to place the opening curly
  bracket on the same line as the function declaration' — but I have
  to remember each rule by hand."
- *After:* "There is a standard formatter, `rustfmt`, bundled with the
  Rust toolchain. From inside a Cargo package directory I run
  `cargo fmt` and Cargo invokes `rustfmt` to rewrite the package's
  Rust files to the community standard style. The rewrite is
  style-only and never changes what the program does. It is the
  operational answer to *what is the right way to format this?*."

## Prerequisites

- Installed concepts:
  - Lesson 064 (`064-cargo-build-standalone`, load-bearing): `cargo`
    is on `PATH` and runs from inside a Cargo package directory
    (`Cargo.toml` plus `src/`). Today extends 064's set of Cargo
    subcommands (`build`, plus `run` from lesson 032 and `check` from
    lesson 084) with `fmt`.
  - Lesson 032 (`032-cargo-new-and-run`, cited): `cargo new <name>`
    scaffolds the package directory the lesson formats.
  - Lesson 085 (`085-toolchain-housekeeping`, cited): `rustup` is the
    program that installed Rust on this host. The same rustup that
    delivered `rustc` and `cargo` also delivered `rustfmt` and
    `cargo-fmt`; today reuses that framing.
  - Lesson 008 (`008-define-and-call-function`, cited): `fn name() {
    ... }` — the working probe's source is one such function body.
  - Lesson 011 (`011-println-positional-args`, cited): `println!`
    prints to stdout. Used by the probe to demonstrate that the
    program's output (its behavior) is unchanged across formatting.
- Ordinary computer-use assumptions: same terminal/editor/shell as
  lesson 064; `cargo` on `PATH`; `cd`; opening a text file before and
  after a change to *see* what changed (the lesson's central witness
  is a visible source-file diff).

## Try It

Pick any directory you can write to:

```console
$ cargo new --vcs none hello_fmt
    Creating binary (application) `hello_fmt` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd hello_fmt
```

Replace the default `src/main.rs` with a deliberately *ugly* but valid
program. The body is the same `fn main()` plus `println!` you have used
since lesson 008/011, but typed without spaces and run together onto
two lines:

```rust
fn main(){let x=42;
println!("x = {}",x);}
```

This is real Rust — rustc does not care about whitespace at the token
level — so it compiles and runs:

```console
$ cargo build
   Compiling hello_fmt v0.1.0 (/path/to/hello_fmt)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.57s
$ ./target/debug/hello_fmt
x = 42
```

Now run `cargo fmt`:

```console
$ cargo fmt
$
```

`cargo fmt` is *quiet*: nothing prints to the terminal, the prompt
returns, exit 0. The witness is not the terminal — it is the source
file. Open `src/main.rs` again:

```rust
fn main() {
    let x = 42;
    println!("x = {}", x);
}
```

The file has been rewritten in place. Spaces around `=`, a space after
`,`, a newline after `{`, a four-space indent for the body, and the
closing `}` on its own line. Rebuild and run to confirm the program
still does the same thing:

```console
$ cargo build
   Compiling hello_fmt v0.1.0 (/path/to/hello_fmt)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
$ ./target/debug/hello_fmt
x = 42
```

Same `x = 42`, exit 0. Style changed; behavior did not.

## What Changed

- `cargo fmt` from inside a Cargo package directory rewrites the
  package's Rust source files to the community standard style. Cargo
  invokes the underlying `rustfmt` tool on each file. (Book Appendix D
  *Automatic Formatting with `rustfmt`*: "To format any Cargo project,
  enter the following: `$ cargo fmt`. Running this command reformats
  all the Rust code in the current crate.")
- `rustfmt` is bundled with the standard Rust toolchain — no separate
  install. The Book Ch1-2 says it directly: "The Rust team has
  included this tool with the standard Rust distribution, as `rustc`
  is, so it should already be installed on your computer." (Lines
  118-120.) Same toolchain you got from `rustup` (lesson 085).
- The rewrite is *style-only*. The Book Appendix D says it explicitly:
  "This should only change the code style, not the code semantics."
  Indentation, spaces, line breaks, brace placement change; what the
  code *does* — its compiled behavior, its output — does not.
- Use `cargo fmt` to keep yourself in one consistent style without
  remembering each individual rule (line 112's "opening curly bracket
  on the same line as the function declaration" is one such rule out
  of many), and to stay on the same style as collaborators on a
  shared project.

## Check Yourself

(a) What kind of changes does `cargo fmt` make to your code? What kind
of changes does it *not* make?

(b) From which directory do you run `cargo fmt`?

(c) Does `rustfmt` need a separate install step beyond the standard
Rust toolchain?

(Answers: (a) Style changes — indentation, spacing, line breaks,
brace placement. It does not change what the program does (its
behavior or output). (b) Inside a Cargo package directory — the same
context as `cargo build` from lesson 064. (c) No. The Book says
"included with the standard Rust distribution, as `rustc` is";
`rustup` delivered it alongside `rustc` and `cargo` (lesson 085).)

## What To Ignore For Now

- `rustfmt` invoked directly on a single file (e.g.
  `rustfmt src/main.rs`). It works — the Book Appendix D pairs it with
  `cargo-fmt` ("`rustfmt` allows finer grained control and `cargo-fmt`
  understands conventions of a project that uses Cargo") — but today
  installs only the Cargo-aware form.
- `rustfmt.toml` / `.rustfmt.toml` configuration files for tweaking
  the standard style (changing the indent width, line length, and so
  on). Future move.
- The exact rules the standard style enforces (four-space indent, max
  line length, trailing-comma policy, blank-line conventions, etc.).
  The lesson installs *that there is a standard*; the rule list is
  separate territory.
- `cargo fmt -- --check`. The flag exits non-zero if the file *would*
  be reformatted — useful in CI to fail a build on unformatted code.
  Future move.
- `rustfix`. A sibling tool also named in Appendix D — it
  automatically applies compiler suggestions. Different tool,
  separate move.
- `cargo clippy`. A separate linting tool that checks for *suspicious
  patterns* rather than style. Lesson 084 named it as a future move.
- IDE / editor integrations that run `cargo fmt` on save. Operational
  layer above the command itself; deferred.

## Evidence

See `../evidence/087-rustfmt.md`.
