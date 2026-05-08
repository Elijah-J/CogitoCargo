---
id: 001-rustc-compile-and-run
move: "compile a tiny Rust source file with rustc and run the produced executable"
main_concept: "rustc turns one .rs source file into a separate executable file in the same directory; running that executable is a second, distinct step"
depends_on: []
assumptions:
  - a working terminal where you can cd into a directory and run commands
  - ability to create and edit a plain text file with any editor
  - rustc is already installed and on your PATH
  - Linux or macOS shell, where ./name runs an executable named name in the current directory
unlocks:
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - future let-binding moves
  - future cargo moves
sources:
  - output/docs/rust/book/ch01-02-hello-world.md
  - output/docs/rust/rustc/what-is-rustc.md
  - output/docs/rust/rust-by-example/hello.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/001-rustc-compile-and-run.rs
status: accepted
---

# Compile a Rust source file with `rustc`, then run it

## The Move

Save a tiny Rust program to a file ending in `.rs`. Run `rustc` on that
file. A new file appears next to the source: the executable. Run the
executable by name. You have just compiled and run your first Rust
program as two separate commands.

## Mental Model Delta

- Before: "to run a program, you point an interpreter at the source
  file." (Or: no model yet.)
- After: "Rust source is compiled ahead of time. `rustc hello.rs`
  produces a separate file named `hello` in the current directory.
  That file is the program. Running it is a second step:
  `./hello`. The `.rs` file and the executable are two different
  files, and only the executable runs."

## Prerequisites

- Installed concepts: none. This is lesson 001.
- Ordinary computer-use assumptions:
  - a terminal where you can `cd` into a directory and run commands;
  - you can create and edit a plain text file with any editor;
  - `rustc` is already installed and on your `PATH` (this run does
    not cover installation);
  - you are on Linux or macOS, where `./name` runs an executable
    named `name` in the current directory.

## Try It

1. Make a new directory and `cd` into it.
2. Create a file called `hello.rs` with exactly this contents:

   ```rust
   fn main() {
       println!("hello from rustc");
   }
   ```

   You are not expected to understand the inside of this program
   yet. Treat it as a complete, opaque "tiny Rust source file" for
   now.
3. Compile it:

   ```console
   $ rustc hello.rs
   ```

   `rustc` prints nothing on success. After it finishes, list the
   directory:

   ```console
   $ ls
   hello  hello.rs
   ```

   There are now two files: the source you wrote (`hello.rs`) and a
   new executable (`hello`).
4. Run the executable:

   ```console
   $ ./hello
   hello from rustc
   ```

The `./` part means "the file named `hello` in this current
directory." That step is how you ask the shell to run that
executable.

## What Changed

- You can take any tiny Rust program someone hands you, save it as
  `something.rs`, and turn it into an executable with one command.
- You know that compiling and running Rust are two separate steps,
  not one.
- You know that after `rustc hello.rs`, the file you actually run is
  `hello`, not `hello.rs`.
- You know that on success, `rustc` is silent. No output is the
  expected output.
- You have a name for the tool: `rustc`, the Rust compiler.

## Check Yourself

You wrote `greet.rs`. You ran `rustc greet.rs`. It printed nothing,
and exited.

- What new file should now exist alongside `greet.rs`?
- What command runs that new file and prints its output?
- If you change `greet.rs` and want the new behavior, what do you
  have to do before running again?

(Answers: a file called `greet`; `./greet`; run `rustc greet.rs`
again to rebuild the executable, then run `./greet`.)

A small contrast worth noticing: the `.rs` file is *not* something
you run. If you try `./hello.rs` after a successful compile, the
shell will refuse it ("permission denied") because the source file is
not an executable. Only the file `rustc` produced is. Source and
executable are two different objects on disk.

## What To Ignore For Now

This lesson installs only the compile-and-run workflow. Each of the
following is real and will be taught in a later lesson, but is *not*
part of this move:

- The shape `fn main() { ... }`. Treat it as required boilerplate
  for now. (covered in lesson 002.)
- `println!` and the `!` after the name. The `!` means something
  specific in Rust; ignore it for this lesson.
- The trailing `;` at the end of the `println!` line. (covered in lesson 004.)
- What `"hello from rustc"` is, as a kind of value. (string-literal type `&str` covered incidentally in lesson 055.)
- What "compilation" actually does inside `rustc` (parsing, code
  generation, linking, etc.). For this lesson, "compile" means
  exactly "run `rustc file.rs` and get an executable next to the
  source." (out of scope for this run; not Rust-specific.)
- Cargo, the higher-level Rust build tool. Cargo eventually replaces
  most direct uses of `rustc`, but learning `rustc` first makes the
  two-file picture (source vs. executable) concrete. (covered starting in lesson 032; built on in lessons 064, 065, 066.)
- Windows shells. The exact run command differs there
  (`.\hello.exe` instead of `./hello`). This lesson is shaped for
  Linux and macOS. (out of scope; this run targets Linux/macOS shells.)

## Evidence

### Sources

- `output/docs/rust/book/ch01-02-hello-world.md` — the canonical
  Hello World walkthrough. Names Rust as an "ahead-of-time compiled"
  language; explicitly shows the two-step flow `$ rustc main.rs` then
  `$ ./main`; explicitly shows that after compile, `ls` lists two
  files (`main` and `main.rs`); explicitly notes that on Windows the
  executable is `main.exe` and the run command is `.\main`.
- `output/docs/rust/rustc/what-is-rustc.md` — defines `rustc` as
  "the compiler for the Rust programming language" and shows the
  same `rustc hello.rs` then `./hello` (or `.\hello.exe` on Windows)
  flow on a `hello.rs` source file.
- `output/docs/rust/rust-by-example/hello.md` — states that "a
  binary can be generated using the Rust compiler: `rustc`" and
  shows `rustc hello.rs` followed by `./hello` printing the output.

### Probe

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/001-rustc-compile-and-run.rs`.

Source compiled (verbatim, comments included for context only):

```rust
fn main() {
    println!("hello from rustc");
}
```

Probe transcript, run in a clean temp directory containing only the
copied `hello.rs`:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
hello.rs
--- rustc hello.rs ---
--- ls after compile ---
hello
hello.rs
--- ./hello ---
hello from rustc
--- file hello hello.rs ---
hello:    Mach-O 64-bit executable x86_64
hello.rs: c program text, ASCII text
```

Notes from the transcript:

- `rustc hello.rs` produced no output and exited successfully.
- After compile, `ls` shows two files: `hello.rs` (the source) and
  `hello` (the new executable).
- `./hello` printed exactly `hello from rustc`.
- `file(1)` confirms `hello` is a Mach-O executable while `hello.rs`
  is plain text. They are distinct files of different kinds.

Contrast probe, same temp dir, after compile:

```text
--- attempt: ./hello.rs ---
(eval):1: permission denied: ./hello.rs
exit=126
```

The shell refuses to run `hello.rs`. Only the file `rustc` produced
is runnable. This is the contrast the lesson teaches: source and
executable are two different objects.

### Prior lessons

None. This is the first lesson in the run.
