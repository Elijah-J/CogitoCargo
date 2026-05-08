# Evidence — 087-rustfmt

Audit appendix for `lessons/087-rustfmt.md`. The lesson installs one
Cargo subcommand — `cargo fmt` — that invokes `rustfmt` on the
package's Rust source files and rewrites them to the community
standard style. Two centered claims today: (1) the rewrite is
style-only — what the program *does* is unchanged; (2) `rustfmt` is
bundled with the standard Rust toolchain (same rustup as lessons 001,
032, 085), no separate install. This is closure item T of the Ch1-3
closure queue.

This appendix covers (a) toolchain and host context, (b) verbatim
probe transcripts (`cargo new`, the deliberately-ugly `src/main.rs`
*before*, `cargo build` + run *before*, `cargo fmt`, `src/main.rs`
*after*, `cargo build` + run *after*, plus an explicit `diff -u`
between the two states), (c) corpus quote map, (d) prerequisite-claim
summary, and (e) a note on contrast probes and why no working-vs-
broken contrast is needed today.

## Toolchain

```
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
rustfmt 1.9.0-stable (59807616e1 2026-04-14)
host: x86_64-apple-darwin
```

`rustup component list --installed` confirms `rustfmt-x86_64-apple-darwin`
is one of the installed components alongside `cargo`, `rustc`, and
`rust-std`. The `cargo-fmt` and `rustfmt` binaries are both present at
`/Users/eli/.cargo/bin/`, both as rustup-managed shims (symlinks to
`rustup`); this is the operational witness that they were installed
*by rustup*, not separately.

The probes ran on `2026-05-07` from a fresh `mktemp -d` scratch
directory.

## Project setup

```
$ cargo new --vcs none hello_fmt
    Creating binary (application) `hello_fmt` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd hello_fmt
$ ls
Cargo.toml  src
$ cat Cargo.toml
[package]
name = "hello_fmt"
version = "0.1.0"
edition = "2024"

[dependencies]
```

This is the same `cargo new` shape lessons 032 and 064 already
installed. The `--vcs none` flag suppresses the default `git init`;
behavior of `cargo fmt` does not depend on whether the directory is a
git repository.

## Probe transcripts

### Step 1: replace `src/main.rs` with deliberately ugly source

```text
$ printf 'fn main(){let x=42;\nprintln!("x = {}",x);}\n' > src/main.rs
$ cat src/main.rs
fn main(){let x=42;
println!("x = {}",x);}
```

Two valid Rust lines, no spaces around `=` or after `,`, no newline
after `{`, the closing `}` on the same line as the last statement.
This is the lesson's centered "ugly source" state, recorded byte-for-
byte at `observations/087-rustfmt/src/main.rs` so the audit can
reproduce the exact starting point.

Word-of-warning on `printf` reproducibility: the `\n` escape in the
format string produces literal newlines (POSIX `printf`), so the
recorded `src/main.rs` is exactly two LF-terminated lines (43 bytes
including the final newline).

### Step 2: `cargo build` + run *before* formatting (witness compiles and runs)

```text
$ cargo build
   Compiling hello_fmt v0.1.0 (/private/var/folders/.../hello_fmt)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.57s
$ ./target/debug/hello_fmt
x = 42
$ echo $?
0
```

Load-bearing observations:

- The deliberately-ugly source compiles. rustc does not care about
  whitespace or line breaks at the token level — those are convenience
  for human readers, and the lesson's *Try It* uses this fact to set up
  the contrast.
- The program prints `x = 42` and exits 0. This is the *before* output
  the lesson will compare against the *after* output to ground "the
  program's behavior is unchanged".

Grounds the lesson body's "rustc does not care about whitespace at the
token level — so it compiles and runs" sentence and the *before*
`cargo build` / run transcript.

### Step 3: `cargo fmt` (the centered move)

```text
$ cargo fmt
$ echo $?
0
```

Load-bearing observations:

- `cargo fmt` prints *nothing* on success. The prompt returns, exit
  code is 0. The terminal is not the witness here — the source file
  is. The lesson body says this explicitly ("`cargo fmt` is *quiet*:
  nothing prints to the terminal").
- Running `cargo fmt` a second time on the now-formatted source is
  also a no-op exit 0 with no output (verified separately during this
  audit). The standard style is a *fixed point*: once a file is in
  standard style, `cargo fmt` does nothing on it.

Grounds *Try It* steps "Now run `cargo fmt`" and "the prompt returns,
exit 0".

### Step 4: `src/main.rs` *after* — the source-file diff is the witness

```text
$ cat src/main.rs
fn main() {
    let x = 42;
    println!("x = {}", x);
}
```

Load-bearing observations:

- Spaces appeared around `=` and after `,` and between `)` and `{`.
- The body of `fn main` is on its own indented lines (four-space
  indent).
- The closing `}` is on its own line.
- The opening `{` of `fn main` is on the *same line* as the function
  declaration, with one space between — exactly the rule Book Ch1-2
  line 112 names: "It's good style to place the opening curly bracket
  on the same line as the function declaration, adding one space in
  between." rustfmt enforces this rule in the standard style.

Explicit `diff -u` (captured separately during this audit by copying
`src/main.rs` to `/tmp/before.rs` before `cargo fmt` and `/tmp/after.rs`
after):

```text
$ diff -u /tmp/before.rs /tmp/after.rs
--- /tmp/before.rs
+++ /tmp/after.rs
@@ -1,2 +1,4 @@
-fn main(){let x=42;
-println!("x = {}",x);}
+fn main() {
+    let x = 42;
+    println!("x = {}", x);
+}
```

The diff is the lesson's central empirical witness. Two lines removed,
four lines added; every change is whitespace, indentation, or line
break — no token added or removed.

`wc -c` reports 43 bytes before and 57 bytes after; the byte change is
entirely inserted whitespace.

Grounds the lesson body's *Try It* "Open `src/main.rs` again" reformat
display and *What Changed* bullet 1 ("rewrites the package's Rust
source files to the community standard style").

### Step 5: `cargo build` + run *after* (witness behavior unchanged)

```text
$ cargo build
   Compiling hello_fmt v0.1.0 (/private/var/folders/.../hello_fmt)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
$ ./target/debug/hello_fmt
x = 42
$ echo $?
0
```

Load-bearing observations:

- The reformatted source still compiles. (Cargo recompiled because
  the source file changed; the lesson does not center this.)
- The program still prints `x = 42` exit 0 — *bit-identical* to Step
  2's output. Same stdout content (8 bytes: `x = 42\n` plus the trailing
  newline `println!` adds), same exit code.
- This is the operational witness for the lesson's centered claim
  "the rewrite is style-only — what the program *does* is unchanged."

Grounds *Try It*'s closing `cargo build` / run transcript and *What
Changed* bullet 3 ("This should only change the code style, not the
code semantics").

### Step 6 (auxiliary, not centered): `cargo run` before and after

A separate scratch directory (`semantics_witness`) repeated the
ugly→`cargo fmt`→pretty cycle but used `cargo run` for the before/after
behavior witness:

```text
[BEFORE cargo fmt]
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/semantics_witness`
x = 42

$ cargo fmt
[AFTER cargo fmt]
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/semantics_witness`
x = 42
```

Behavior is identical. Documented for completeness; the lesson body
uses `cargo build` + `./target/debug/hello_fmt` (lesson 064's two-step
shape) rather than `cargo run` (lesson 032) so that the binary path
itself (`target/debug/<name>`) is reused as a sanity check.

### Step 7 (auxiliary, not centered): `cargo fmt` is the same on a multi-file crate

A third scratch directory (`multi_file2`) created two files in the
module tree — `src/main.rs` declaring `mod helper;` and `src/helper.rs`
defining `pub fn compute() -> i32 { ... }` — both deliberately ugly.
After one `cargo fmt`, *both* files were rewritten in standard style:

```text
$ ls src/
helper.rs  main.rs

[BEFORE: both ugly]
$ cat src/main.rs
mod helper;
fn main(){let x=helper::compute();
println!("x = {}",x);}
$ cat src/helper.rs
pub fn compute(  )->i32{let n=1+2;n}

$ cargo fmt

[AFTER: both formatted]
$ cat src/main.rs
mod helper;
fn main() {
    let x = helper::compute();
    println!("x = {}", x);
}
$ cat src/helper.rs
pub fn compute() -> i32 {
    let n = 1 + 2;
    n
}
```

Load-bearing observations:

- `cargo fmt` formats *every* file reachable through the module tree
  in one invocation — both `main.rs` and `helper.rs` here. This
  grounds the Book Appendix D claim "reformats all the Rust code in
  the current crate" (line 24).
- A separate scratch witnessed the inverse case: an *orphan* file
  `src/extra.rs` that is *not* declared in any `mod` was left
  unchanged by `cargo fmt`. The lesson body does not surface this
  edge case (the audience has no `mod` declaration vocabulary yet —
  multi-file crates and `mod` are deferred), but the audit notes it
  here so the "all the Rust code in the current crate" framing is
  understood as "all files in the crate's module tree", not "every
  `.rs` file in `src/`". The *centered* probe (Steps 1-5 above) uses
  a one-file crate, where the distinction does not bite.

Documented for completeness; the lesson stays single-file.

### Step 8 (auxiliary, not centered): `cargo fmt -- --check` exists

A fourth scratch directory (`check_demo`) ran `cargo fmt -- --check`
on the same ugly source. The flag exits non-zero and prints a colored
diff to stdout but does *not* modify the file:

```text
$ cargo fmt -- --check
Diff in /private/.../check_demo/src/main.rs:1:
-fn main(){let x=42;
-println!("x = {}",x);}
+fn main() {
+    let x = 42;
+    println!("x = {}", x);
+}
$ echo $?
1
```

Documented for completeness. The lesson defers `--check` to *What To
Ignore* — its CI use case sits above the audience's current
operational ceiling.

## Corpus quote map

### `output/docs/rust/book/ch01-02-hello-world.md`

The Book's "Hello, World!" chapter, mid-chapter `Note:` block. This
is the in-Ch1-3-scope source naming `rustfmt` and is the canonical
introductory mention.

**Lines 115-120** (load-bearing):

> Note: If you want to stick to a standard style across Rust projects, you can
> use an automatic formatter tool called `rustfmt` to format your code in a
> particular style (more on `rustfmt` in
> [Appendix D](appendix-04-useful-development-tools.md)). The Rust team has included this tool
> with the standard Rust distribution, as `rustc` is, so it should already be
> installed on your computer!

This grounds three lesson sentences:

- "an automatic formatter tool called `rustfmt`" → *The Move*
  ("Cargo invokes `rustfmt`"), *Mental Model Delta* "After"
  ("There is a standard formatter, `rustfmt`"), *What Changed*
  bullet 1 ("Cargo invokes the underlying `rustfmt` tool").
- "format your code in a particular style" → *The Move* ("rewrites
  them to the *community standard* style"), *What Changed* bullet 1
  ("the community standard style").
- "The Rust team has included this tool with the standard Rust
  distribution, as `rustc` is, so it should already be installed on
  your computer!" → *What Changed* bullet 2 (verbatim quote) and
  *Mental Model Delta* "After" ("bundled with the Rust toolchain")
  and *Check Yourself* (c) sample answer.

**Line 112** (load-bearing for the *Mental Model Delta* "Before"):

> It's good style to place the opening curly bracket on the same
> line as the function declaration, adding one space in between.

This grounds the *Mental Model Delta* "Before" paragraph's named
example — the kind of style rule the learner currently has to
remember by hand. The *after* state of Step 4 above shows rustfmt
applying exactly this rule.

### `output/docs/rust/book/appendix-04-useful-development-tools.md`

Appendix D, the *Automatic Formatting with `rustfmt`* subsection.
This is the corroborating-and-deepening source for today's claims; the
Book Ch1-2 note explicitly forwards to it ("more on `rustfmt` in
Appendix D").

**Lines 8-26** (load-bearing — the entire subsection):

> ### [Automatic Formatting with `rustfmt`](#automatic-formatting-with-rustfmt)
>
> The `rustfmt` tool reformats your code according to the community code style.
> Many collaborative projects use `rustfmt` to prevent arguments about which
> style to use when writing Rust: Everyone formats their code using the tool.
>
> Rust installations include `rustfmt` by default, so you should already have the
> programs `rustfmt` and `cargo-fmt` on your system. These two commands are
> analogous to `rustc` and `cargo` in that `rustfmt` allows finer grained control
> and `cargo-fmt` understands conventions of a project that uses Cargo. To format
> any Cargo project, enter the following:
>
> ```console
> $ cargo fmt
> ```
>
> Running this command reformats all the Rust code in the current crate. This
> should only change the code style, not the code semantics. For more information
> on `rustfmt`, see [its documentation](https://github.com/rust-lang/rustfmt).

This grounds five lesson sentences:

- "reformats your code according to the community code style" →
  *The Move* ("rewrites them to the *community standard* style"),
  *Mental Model Delta* "After" ("the community standard style").
- "Many collaborative projects use `rustfmt`" / "Everyone formats
  their code using the tool" → *What Changed* bullet 4 ("to stay on
  the same style as collaborators on a shared project").
- "Rust installations include `rustfmt` by default" / "you should
  already have the programs `rustfmt` and `cargo-fmt` on your
  system" → *The Move* ("the same rustup that delivered `rustc`
  and `cargo` also delivered `rustfmt` and `cargo-fmt`"), *Mental
  Model Delta* "After" ("bundled with the Rust toolchain"), and
  *Check Yourself* (c). Step 0 (`which cargo-fmt`, `which rustfmt`,
  both rustup-managed shims) is the operational witness.
- "To format any Cargo project, enter the following: `$ cargo fmt`.
  Running this command reformats all the Rust code in the current
  crate." → *The Move* ("From inside a Cargo package directory ...
  run `cargo fmt`"), *What Changed* bullet 1 (verbatim quote of the
  centered claim).
- "This should only change the code style, not the code semantics." →
  *What Changed* bullet 3 (verbatim quote). Step 5 above and Step 6
  auxiliary are the operational witnesses (`x = 42` before and after,
  bit-identical stdout).

The Appendix's `rustfmt` vs `cargo-fmt` analogy ("`rustfmt` allows
finer grained control and `cargo-fmt` understands conventions of a
project that uses Cargo") grounds the *What To Ignore* bullet on
"`rustfmt` invoked directly on a single file" — the lesson installs
only the Cargo-aware form today.

## Prerequisite-claim summary

### From lesson 064 (`064-cargo-build-standalone`) — *direct, load-bearing*

- `cargo` is on `PATH` and runs from inside a Cargo package directory.
  Lesson 064 establishes this with the same `cargo new --vcs none` /
  `cd <name>` / `cargo build` flow today reuses (Steps 0 and 2 of this
  appendix). Today extends 064's set of Cargo subcommands with `fmt`.
- The package directory has the shape `Cargo.toml` plus `src/`. Today
  reuses this without re-installing it; the package today writes to
  `src/main.rs`.

### From lesson 032 (`032-cargo-new-and-run`) — *cited, supporting*

- `cargo new <name>` scaffolds a binary package directory with
  `Cargo.toml` and `src/main.rs`. Today's *Try It* literal transcript
  reuses lesson 032's `Creating binary (application) ... package`
  shape verbatim.

### From lesson 085 (`085-toolchain-housekeeping`) — *cited, supporting*

- `rustup` is the program that installed Rust on this host, and it
  installed `rustc` and `cargo` together. Today extends that framing:
  the same rustup also installed `rustfmt` and `cargo-fmt`. The Step
  0 evidence (`rustup component list --installed` showing
  `rustfmt-x86_64-apple-darwin`; `/Users/eli/.cargo/bin/rustfmt` and
  `cargo-fmt` both as rustup-managed shims) is the operational
  witness; the Book Ch1-2 line 118 quote ("included this tool with
  the standard Rust distribution, as `rustc` is") is the corpus
  witness.

### From lesson 008 (`008-define-and-call-function`) — *cited, supporting*

- `fn name() { ... }` is a function definition. Today's working
  probe is one such function (`fn main`).

### From lesson 011 (`011-println-positional-args`) — *cited, supporting*

- `println!("x = {}", x)` prints `x` formatted with `{}`. Today's
  probe uses this to print `x = 42` and to demonstrate that the
  output is unchanged across formatting.

### Ordinary computer-use assumptions (named in the lesson body)

- The learner can open a text file, see its content, edit it, and
  open it again to see what changed. The lesson's central witness is
  a *visible source-file diff* — `src/main.rs` before vs after — so
  the ability to view a file is load-bearing in a way it has not
  been for previous Cargo lessons. Step 4 of this appendix records
  the explicit `diff -u` output so the audit does not depend on the
  learner re-running the command.

## Contrast-probe coverage

The lesson's central contrast is *style change* vs *behavior change*.
The probe witnesses both halves:

- **Style change witnessed:** Step 4's `diff -u` shows the source
  rewritten — every change is whitespace/indentation/line-break, no
  tokens added or removed.
- **Behavior unchanged witnessed:** Steps 2 and 5 both produce
  `x = 42` exit 0 — bit-identical stdout, identical exit code, before
  and after `cargo fmt`. Step 6 auxiliary repeats the same comparison
  with `cargo run`.

There is no working-vs-broken contrast probe. `cargo fmt` succeeds by
design on any valid Rust source in a Cargo package; the only failure
modes (invalid Rust source, no `Cargo.toml`) belong to other lessons'
diagnostic territory and would not produce a useful contrast for
*today's* claim. This matches lesson 086's contrast-coverage decision
for `rustup doc` (toolchain commands succeed by design on a healthy
install).

The implicit `cargo fmt -- --check` contrast (Step 8) is documented
for completeness but deferred to *What To Ignore*.

## What is *not* in this probe

The probe deliberately does not exercise:

- `rustfmt` invoked directly on a single file (`rustfmt src/main.rs`).
  Works on this host (the binary is at `/Users/eli/.cargo/bin/rustfmt`),
  but the lesson installs only the Cargo-aware `cargo fmt` form.
- `rustfmt.toml` / `.rustfmt.toml` configuration. Centered configuration
  files are a future move.
- The exact rule set the standard style enforces (four-space indent,
  100-character line length, etc.). The lesson installs *that there is
  a standard* and shows the rules' effect on the probe; the rule list
  is separate territory.
- `cargo fmt -- --check`. Step 8 auxiliary captured the transcript;
  *What To Ignore* defers it.
- `rustfix` — sibling tool, separate move.
- `cargo clippy` — separate linting tool, named in lesson 084's
  unlocks.
- IDE / on-save format integration — operational layer above the
  command.
- The full content of `cargo fmt --help` and `rustfmt --help`. Help
  text not load-bearing.
- Multi-file crates with `mod` declarations. Step 7 auxiliary captured
  the multi-file case for completeness, but the lesson body stays
  single-file because `mod` is not yet installed for the audience.
- The orphan-`.rs`-file edge case (a `src/extra.rs` not declared in
  any `mod` is left unformatted by `cargo fmt`). Documented in Step
  7's load-bearing observations as a clarification of "all the Rust
  code in the current crate", not centered in the lesson.

## Files committed for this cycle

- `lessons/087-rustfmt.md` (this lesson)
- `evidence/087-rustfmt.md` (this appendix)
- `observations/087-rustfmt/Cargo.toml` (the working probe's package
  manifest; bit-identical to `cargo new --vcs none hello_fmt`'s
  default minus a trailing blank line)
- `observations/087-rustfmt/src/main.rs` (the *deliberately ugly*
  before-state — two LF-terminated lines, 43 bytes; recorded so the
  audit can re-run `cargo fmt` and verify the diff against the
  recorded starting point)
- `observations/087-rustfmt/.gitignore` (ignores `/target` and
  `Cargo.lock` — same as lessons 064/082/084)
- updated `graph.md` (a new draft node block under `## Draft Nodes`)
