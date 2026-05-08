---
id: 071-macro-invocation-syntax
status: accepted
evidence: ../evidence/071-macro-invocation-syntax.md
---

# Macro invocation `name!(...)` vs. function call `name(...)`

## The Move

In the same `.rs` file as `fn main`, define a tiny function `fn greet() { println!("hi"); }` (lesson 008's shape). From inside `main`, call it as `greet()` — that compiles and runs. Now swap the marks: change the call to `greet!()` (add a `!`); rustc rejects with `error: cannot find macro \`greet\` in this scope`. Restore `greet()` and drop the `!` from the next line, making it `println("from a macro")`; rustc rejects with `error[E0423]: expected function, found macro \`println\``. The `!` is the syntactic mark that distinguishes a macro invocation from an ordinary function call.

## Mental Model Delta

- Before: "I have been writing `println!("...");` since lesson 001 and treating the trailing `!` as opaque required punctuation."
- After: "There are two call shapes. `name(...)` is a *function call*. `name!(...)` is a *macro invocation*. The `!` is the syntactic mark that tells rustc which shape this is. The two shapes look up the name in different places: a function `greet` is not findable as a macro, and a macro `println` is not findable as a function. `println!` has been a macro all along; today names it."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`. Lesson 001 deferred the `!` after `println` ("means something specific in Rust; ignore it for this lesson"); today closes that loop.
  - Lesson 002: body of `fn main` runs when the executable launches.
  - Lesson 003 (load-bearing): the four-part diagnostic map (headline, `-->`, source excerpt with caret, help/note). Both contrast diagnostics today are read with that map; no new diagnostic vocabulary is installed.
  - Lesson 008 (load-bearing): the function-call form `name();` and the `fn greet() { ... }` shape. Today contrasts macro invocation against exactly that form.
  - Lesson 011: operational fluency with `println!`. Lesson 011 deferred "what 'macro' actually means and what the trailing `!` after `println` signifies"; today closes that loop *for the syntax only*.
- Ordinary computer-use assumptions: same as lesson 001 (terminal, plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs` containing exactly:

```rust
fn greet() {
    println!("hi");
}

fn main() {
    greet();
    println!("from a macro");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
hi
from a macro
```

Two calls happen inside `main`. `greet()` is a *function call* (lesson 008's form): name followed directly by `()`. `println!("from a macro")` is a *macro invocation*: name followed by `!`, then `("...")`. The Book where it first introduces `println!` puts the rule plainly: "If it had called a function instead, it would be entered as `println` (without the `!`). [...] using a `!` means that you're calling a macro instead of a normal function." The Reference is more compact: macros "are given names, and invoked through a consistent syntax: `some_extension!(...)`."

Now two contrast probes that surface what rustc does when the marks are swapped. Full transcripts are in the evidence appendix; the headlines are below.

**Contrast A** — keep `greet` as a function, but call it with `!`. Edit `main` to `greet!();` and recompile. Rustc rejects:

```text
error: cannot find macro `greet` in this scope
 --> demo.rs:6:5
  |
6 |     greet!();
  |     ^^^^^
  |
  = note: `greet` is in scope, but it is a function, not a macro
```

The `note:` line states the namespace separation in plain prose: `greet` exists, but `name!(...)` looks up `name` in a different table than `name(...)` does, and the macro table does not contain a function.

**Contrast B** — restore `greet()`, then call `println` *without* the `!`. Change the second-to-last line of `main` to `println("from a macro");` and recompile. Rustc rejects:

```text
error[E0423]: expected function, found macro `println`
 --> demo.rs:7:5
  |
7 |     println("from a macro");
  |     ^^^^^^^ not a function
  |
help: use `!` to invoke the macro
  |
7 |     println!("from a macro");
  |            +
```

The mirror of Contrast A: `println` exists, but the function table does not contain a macro. The `help:` block proposes the exact fix — add the `!`. Together the two probes witness that the `!` is what rustc keys on when deciding which table to search.

## What Changed

- The `!` after a name in `name!(...)` is no longer opaque punctuation. It is the syntactic mark of a *macro invocation*; it tells rustc to look the name up as a macro rather than as a function.
- Rust has two call shapes: function-call `name(...)` and macro-invocation `name!(...)`. For a given name, rustc accepts one or the other, never both.
- `println!` has been a macro since lesson 001. Today gives that observation a name. Other macros (e.g. `vec!`, `format!`, `assert_eq!`, `panic!`) will be picked up case-by-case in later lessons.
- Two new diagnostic shapes, both readable with lesson 003's map: `cannot find macro \`X\` in this scope` (wrote `X!(...)` for something that is not a macro), and `error[E0423]: expected function, found macro \`X\`` (wrote `X(...)` for something that is a macro).

## Check Yourself

You write `tiny.rs` containing:

```rust
fn shout() {
    println!("HEY");
}

fn main() {
    shout!();
}
```

You run `rustc tiny.rs`.

(a) Does the executable build?

(b) What word follows `cannot find` in the headline, and what does the inline `note:` say?

(c) What is the smallest single-character edit to `main`'s body that makes the program compile?

(Answers: (a) No. (b) `macro`; the headline reads `error: cannot find macro \`shout\` in this scope`, and the `note:` says `\`shout\` is in scope, but it is a function, not a macro`. (c) Delete the `!` from `shout!();` so the line reads `shout();` — lesson 008's function-call form.)

## What To Ignore For Now

This lesson installs only the *syntactic* distinction between `name(...)` and `name!(...)`. The following are real and deferred:

- *What macro expansion actually does at compile time.* Macros expand into other Rust code before rustc finishes compiling; the mechanism is out of scope. The Book itself defers it to Chapter 20.
- *Defining* a macro: `macro_rules!`, procedural macros (function-like, attribute, derive), token trees, capture variables (`$x:expr`), hygiene. Today only *uses* macros someone else defined.
- *The full set of std macros* (`vec!`, `format!`, `assert!`, `assert_eq!`, `panic!`, `dbg!`, `eprintln!`, `write!`, `writeln!`, `todo!`, `unimplemented!`). `format!` will likely be the next macro picked up. Each new macro is its own future move.
- *Macros in positions other than statements/expressions* and *macro paths* (`crate::mod::name!(...)`). This lesson uses only the bare-name statement form `name!(...);`.
- *The bracketing alternatives* `name![...]` and `name!{...}`. The Reference allows them; this run has only seen `name!(...)`.
- *The `!` type* (the "never" type in function signatures like `fn f() -> !`). Same character, unrelated to macro invocation. Flagged here only so you do not confuse the two when you meet `!` in another role later.
- *Trait machinery.* Some macros desugar through traits at expansion time, but no trait concepts are installed today; the prerequisite list deliberately cites no trait-related lesson.

## Evidence

See `../evidence/071-macro-invocation-syntax.md`.
