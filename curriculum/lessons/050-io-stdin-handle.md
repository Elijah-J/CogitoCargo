---
id: 050-io-stdin-handle
status: accepted
evidence: ../evidence/050-io-stdin-handle.md
---

# Obtain a standard-input handle with `std::io::stdin()`

## The Move

Lesson 042 reached a function that takes no receiver: `String::new()` —
type, `::`, name, empty parens. Lesson 043 generalized the path syntax
so the leading segments could be *modules* rather than a single type,
with `std::cmp::min(3, 5)` as the example. Today's move composes those
two ideas. The path `std::io::stdin` has three segments — two of them
modules — and the final segment names a no-receiver free function. The
call returns a value of a type spelled `std::io::Stdin`, called a
*standard-input handle*:

```rust
let _stdin = std::io::stdin();
```

Read the right-hand side left to right. `std` is the standard
library's root module (already named in lesson 043). `std::io` is its
input/output submodule. `std::io::stdin` is a free function inside
that submodule. The empty parens `()` call it. The function takes no
arguments and produces a `std::io::Stdin` value — a handle through
which a future move will read input.

This lesson installs only that the function and the type exist and
the path resolves. Every method on `Stdin` — including the next
natural step `.read_line(&mut buf)` — is deferred.

## Mental Model Delta

- Before: "The path syntax `module::submodule::name(args)` reaches a
  free function in a nested module — example, `std::cmp::min(3, 5)`."
- After: "The same path syntax also reaches the standard library's
  *input/output* corner. `std::io::stdin()` is the no-receiver free
  function `stdin` in the `std::io` submodule; the call returns a
  *handle* — a value of type `std::io::Stdin` — that later moves will
  read input through."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 005: `rustc file.rs` then `./name`; `fn main`
    is the entry point; `let name = value;` binds a name and
    `println!("...")` prints a literal line.
  - Lesson 042 (load-bearing): the no-receiver call shape with empty
    parens and no value-side dot form. Today reuses the shape with a
    free function in a module instead of an associated function on a
    type.
  - Lesson 043 (load-bearing): the nested-module-path form
    `module::submodule::name(args)`. `std::cmp::min` introduced the
    three-segment shape and named `std` as the standard library's
    root module. Today reuses the shape with a different submodule
    (`io`) and a different final function (`stdin`).
  - Lesson 029 (gloss only): the leading `_` on a binding name tells
    rustc "intentionally unused." Same gloss as `let _name: () = ();`.
    Not formally installed there or here.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

In a fresh empty directory, create `demo.rs`:

```rust
fn main() {
    let _stdin = std::io::stdin();
    println!("got stdin handle");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
got stdin handle
```

Walk it. `std::io::stdin()` is the lesson-043 three-segment path
with an empty argument list (the lesson-042 no-receiver shape). The
call returns a value of type `std::io::Stdin`. The `let` binds it to
`_stdin`; the leading underscore is the lesson-029 gloss for
"intentionally unused." Without it the program still compiles but
rustc emits an unused-variable warning. Nothing is read from the
handle — creating it is a separate step from reading through it,
and today's move stops at the creation step.

(Full transcript and toolchain string are in
`../evidence/050-io-stdin-handle.md`.)

## What Changed

- You can write the path `std::io::stdin` to reach a free function
  named `stdin` in the `std::io` submodule. Same lesson-043 path
  shape; new submodule, new function name.
- You know one new submodule, `std::io` — the standard library's
  input/output corner.
- You know one new function, `std::io::stdin()`, which takes no
  arguments and produces a `std::io::Stdin` value.
- You know one new type, `std::io::Stdin`, called a *standard-input
  handle*. The lesson does not call any method on it; that is the
  natural next move.
- You know what the underscore on `_stdin` does, in one sentence:
  it signals "intentionally unused" so rustc does not warn. Same
  gloss as lesson 029.

## Check Yourself

You write `pred.rs`:

```rust
fn main() {
    let _h = std::io::stdin();
    println!("ok");
}
```

(a) Does rustc accept the program? Does it emit any diagnostic?

(b) What single line does `./pred` print?

(c) In the path `std::io::stdin`, which segments name modules and
which segment names the function?

(Answers: (a) Yes, with no diagnostic — the program compiles silently
and the binary is produced. (b) `ok`. (c) `std` and `io` are modules;
`stdin` is the free function in the `io` submodule.)

## What To Ignore For Now

This lesson installs only that `std::io::stdin()` exists, returns a
value of type `std::io::Stdin`, and that the path resolves. Deferred:

- *Every method on `Stdin`* — `.read_line(&mut buf)`, `.lock()`,
  `.lines()`, etc. The natural next move (`.read_line(&mut buf)`)
  is the immediate successor cycle.
- *Reading from stdin* — the lesson does not actually read input;
  the program runs straight through.
- *The `Read` and `BufRead` traits* — `Stdin` implements both.
  Trait machinery still deferred from lessons 040, 041.
- *Threading, the internal mutex, and stdin singleton-ness* — heavy
  deferral; not installed.
- *The rest of the `io` module* — `stdout()`, `stderr()`, `Stdout`,
  `Stderr`, `BufReader`, `BufWriter`, `Error`. Defer.
- *The `io::Result<T>` type alias* — `Result` itself is uninstalled.
  Defer.
- *Type-path syntax* `: std::io::Stdin` — type inference is used
  today, so the lesson-019 annotation surface is not extended here.
- *The `_` binding-name convention* — glossed via lesson 029; formal
  install is still a future move.
- *`use std::io;` so the call shortens to `io::stdin()`* — parent-
  module use form, deferred from cycle 044. The Book chapter 2
  source uses that form; today uses the full path so no `use` is
  required.
- *The standard prelude* and *external crates* — carrying over from
  cycle 043.
- All previously deferred items.

## Evidence

See `../evidence/050-io-stdin-handle.md` for the corpus-quote map,
the rustc / system toolchain string, the working probe transcript,
the contrastive-probe-omission justification, and the prerequisite-
claim summary.
