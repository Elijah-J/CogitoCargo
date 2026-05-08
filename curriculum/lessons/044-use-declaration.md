---
id: 044-use-declaration
status: accepted
evidence: ../evidence/044-use-declaration.md
---

# Shorten a path with `use std::cmp::min;`

## The Move

Lesson 043 reached `std::cmp::min` by writing the full path at every
call site. There is a second way to reach the same function: write a
*`use` declaration* at the top of the file once, and then call the
function by its final segment alone everywhere below.

```rust
use std::cmp::min;

fn main() {
    let full: i32 = std::cmp::min(3, 5);
    let short: i32 = min(3, 5);
    println!("full = {full}, short = {short}");
}
```

Both `full` and `short` bind to `3`. The `use std::cmp::min;` line at
the top brings the name `min` into the file's scope, so the bare call
`min(3, 5)` resolves to the same `std::cmp::min` function the full
path reaches. The full path still works after the `use`; the `use`
*adds* the short alias, it does not replace the long form.

## Mental Model Delta

- Before: "To call a free function in a nested module I have to write
  the full path `module::submodule::name(args)` every time."
- After: "I can write `use module::submodule::name;` once at the top of
  the file. The bare final segment then resolves to the same function
  anywhere below. The full path still works too — `use` adds a short
  alias, it does not remove the long form."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 003, 005, 019: compile/run shape, `fn main`,
    diagnostic shape, `let name: i32 = value;`.
  - Lesson 043 (load-bearing): the full path call form
    `std::cmp::min(a, b)`, with `std` as the standard library's root
    module, `cmp` as a submodule, and `min` as a free function in it.
    Today's lesson is "shorten *exactly that* path."
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH` (same as lesson 001).

## Try It

In a fresh empty directory, create `demo.rs`:

```rust
use std::cmp::min;

fn main() {
    let full: i32 = std::cmp::min(3, 5);
    let short: i32 = min(3, 5);
    println!("full = {full}, short = {short}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
full = 3, short = 3
```

Walk it. Line 1 is the new shape: the keyword `use`, then a path
identical in form to lesson 043's call paths, then a trailing `;`. It
sits at the top of the file, outside `fn main`. Line 4 is the
lesson-043 full-path call, unchanged. Line 5 is the new call: just the
final segment `min` followed by `(3, 5)`. Both produce `3`.

*Predict*: what if you keep `use std::cmp::min;` but call a *different*
sibling function from the same module by its bare name? Edit `demo.rs`
to read:

```rust
use std::cmp::min;

fn main() {
    let smaller: i32 = min(3, 5);
    let larger: i32 = max(3, 5);
    println!("smaller = {smaller}, larger = {larger}");
}
```

and recompile. rustc emits:

```
error[E0425]: cannot find function `max` in this scope
 --> broken.rs:5:23
  |
5 |     let larger: i32 = max(3, 5);
  |                       ^^^ not found in this scope
  |
help: consider importing this function
  |
1 + use std::cmp::max;
  |
```

Same E-code as lessons 005, 008, 040, 042, 043. The headline names
`max`, not `min`. `use std::cmp::min;` brought in *only* `min`; `max`
is still unreachable as a bare name. rustc suggests a *separate*
`use std::cmp::max;` line — one `use` per item. The lesson-043 fix
(full path `std::cmp::max(3, 5)`) also works.

(Full transcripts are in `../evidence/044-use-declaration.md`.)

## What Changed

- You can write `use module::submodule::name;` at the top of a `.rs`
  file: a one-line item of its own ending in `;`, outside `fn main`.
- After the `use` line, the final segment `name` resolves to the same
  function the full path reached. `min(3, 5)` calls `std::cmp::min`.
- The full path keeps working too. `use` adds an alias; it does not
  remove the long form.
- A `use` line brings in *only the items it names*. `use std::cmp::min;`
  does not also bring in sibling `max` — calling bare `max(3, 5)`
  afterward fires E0425 with `help:` suggesting a separate
  `use std::cmp::max;`.

## Check Yourself

You write `pred.rs` containing:

```rust
use std::cmp::max;

fn main() {
    let big: i32 = max(10, 4);
    let small: i32 = std::cmp::min(10, 4);
    println!("big = {big}, small = {small}");
}
```

(a) Does rustc accept the program?

(b) What single line does `./pred` print?

(c) If you replaced line 4 with `let big: i32 = min(10, 4);` and kept
line 1 as `use std::cmp::max;`, which E-code would the headline carry,
and what would the `help:` block suggest?

(Answers: (a) Yes — `use std::cmp::max;` brings `max` into scope, and
the lesson-043 full path `std::cmp::min(10, 4)` works regardless of
imports. (b) `big = 10, small = 4`. (c) E0425 "cannot find function
`min` in this scope"; `help:` suggests `use std::cmp::min;` —
`use std::cmp::max;` does not pull in sibling items.)

## What To Ignore For Now

This lesson installs only one idea: a top-level `use Path::final;`
line brings the final segment into the file's scope. Deferred:

- *`use ... as alias;`* — renaming on import. Future move.
- *Glob imports `use std::cmp::*;`* — wildcard import. Future move.
- *Nested-group imports `use std::{io, cmp::min};`* — multiple names
  in one `use` line. Future move.
- *`use crate::`, `use self::`, `use super::`* — non-absolute path
  roots. Today's only path root is `std::`.
- *`pub use` re-exports*, *import shadowing and conflicts*. Future
  moves alongside visibility.
- *The standard prelude* — a small set of names (`String`, `Vec`,
  `i32`, `println!`, ...) is pre-imported into every Rust program,
  which is why `String` and `i32` work bare. Mechanism deferred.
- *`use` inside functions or modules* — `use` can appear at any
  item-position scope, not just the file top level. Today's surface
  is only the file-top-level form.
- All previously deferred items.

## Evidence

See `../evidence/044-use-declaration.md` for the corpus-quote map, the
rustc / system toolchain string, the working probe transcript, the
broken-contrast E0425 transcript, and the prerequisite-claim summary.
