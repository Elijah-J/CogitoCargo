---
id: 043-nested-module-paths
status: accepted
evidence: ../evidence/043-nested-module-paths.md
---

# Call a function in a nested module with `std::cmp::min(a, b)`

## The Move

Lessons 041 and 042 used the qualified path `Type::name(args)` with one
*type* segment in front of `::` — `i32::abs(n)`, `String::new()`. The
path syntax generalizes: the leading segments can be *modules*, and
there can be more than one. Concretely:

```rust
let smaller: i32 = std::cmp::min(3, 5);
```

Read the right-hand side left-to-right. `std` is Rust's standard
library; `::` separates each segment; `cmp` is a *submodule* of `std`
holding comparison-related items; `min` is a free function in that
submodule that takes two values of the same kind and returns the
smaller. The whole call expression produces `3`, so `smaller` binds to
`3` (lesson 021).

The same shape works for any number of `::`-separated segments — the
final one names the function; every earlier one names a module.

## Mental Model Delta

- Before: "The path syntax `A::B::name` puts a *type* in front of `::`
  to reach a function attached to that type."
- After: "The path syntax is more general. The leading segments can be
  *modules*, not just types — a module is a namespace that holds
  functions, types, constants, and other items. Each `::` steps
  deeper. `std::cmp::min` reads as 'in the `std` module, in the `cmp`
  submodule, the function `min`'."

## Prerequisites

- Installed concepts:
  - Lesson 005 (`let name = value;`), lesson 019 (`let name: TYPE = value;`),
    lesson 020 (typed parameters), lesson 021 (a call expression returns
    a value).
  - Lessons 001, 002, 003: `rustc file.rs` then `./name`; `fn main` is
    the entry point; diagnostics have headline + `-->` + source excerpt
    + caret + optional `help:`.
  - Lesson 041 (load-bearing): the qualified path `A::B(args)` with one
    segment in front of `::`. Today's lesson generalizes that to multiple
    segments.
  - Lesson 042 (load-bearing): `A::B(args)` can have an empty argument
    list and no receiver — it just calls a function reached by path.
    Today's call site reuses that same shape with a longer path.
- Ordinary computer-use assumptions: terminal, plain-text editor, `rustc`
  on `PATH`, Linux/macOS shell where `./name` runs an executable (same
  as lesson 001).

## Try It

In a fresh empty directory, create `demo.rs`:

```rust
fn main() {
    let smaller: i32 = std::cmp::min(3, 5);
    let larger: i32 = std::cmp::max(3, 5);
    println!("smaller = {smaller}, larger = {larger}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
smaller = 3, larger = 5
```

Walk it. `std::cmp::min(3, 5)` has three `::`-separated segments —
`std`, `cmp`, `min` — then the argument list `(3, 5)`. The two arguments
are matched positionally (lesson 036), so the call returns the smaller
of the two: `3`. Line 3 reuses the same prefix `std::cmp::` with `max`
as the final segment and returns the larger: `5`. The shared prefix is
the point: `std::cmp::` is a namespace; `min` and `max` are two
different items inside it.

*Predict*: what if you drop the path and just write `min(3, 5)`? Edit
line 2 to `let smaller: i32 = min(3, 5);` and recompile. rustc emits:

```
error[E0425]: cannot find function `min` in this scope
 --> broken.rs:2:24
  |
2 |     let smaller: i32 = min(3, 5);
  |                        ^^^ not found in this scope
  |
help: consider importing this function
  |
1 + use std::cmp::min;
  |
```

Same E-code as lessons 005, 008, 040, and 042 — *cannot find this name*.
The `help:` block suggests `use std::cmp::min;` at the top of the file.
That `use` declaration is a future move and is **deliberately not
installed by this lesson**. The same broken program is also fixed by
restoring the full path `std::cmp::min(3, 5)` — the move this lesson
teaches. Both fixes work; you are learning the full-path one.

(Full transcripts are in `../evidence/043-nested-module-paths.md`.)

## What Changed

- You can call a free function in a nested module by writing the full
  path `module::submodule::name(args)`. Read left-to-right with `::`
  between segments; the final segment names the function.
- You know one new word, *module*: a namespace that holds functions,
  types, constants, and other items. `std` is the standard library's
  root module; `std::cmp` is one of its submodules.
- You know two concrete functions: `std::cmp::min(a, b)` returns the
  smaller of two same-kind values; `std::cmp::max(a, b)` returns the
  larger.
- You know the failure mode: writing `min(3, 5)` without the path fires
  E0425 — the same E-code family as lessons 005, 008, 040, and 042. The
  `help:` block names `use std::cmp::min;` (a future move); the full
  path is the second valid fix.

## Check Yourself

You write `pred.rs` containing:

```rust
fn main() {
    let small: i32 = std::cmp::min(10, 4);
    let big: i32 = std::cmp::max(10, 4);
    println!("small = {small}, big = {big}");
}
```

(a) Does rustc accept the program?

(b) What single line does `./pred` print?

(c) If you replaced `std::cmp::min(10, 4)` with `min(10, 4)` and
recompiled, which E-code would the headline carry?

(Answers: (a) Yes — same shape, different arguments. (b)
`small = 4, big = 10`. (c) E0425 "cannot find function `min` in this
scope"; the `help:` again suggests `use std::cmp::min;`, but
`std::cmp::min(10, 4)` is the fix this lesson teaches.)

## What To Ignore For Now

This lesson installs only the full-path call form
`module::submodule::name(args)`. Deferred:

- *`use` declarations* — `use std::cmp::min;` lets you write `min(3, 5)`
  afterwards. The `help:` block in the broken probe points at this
  future move.
- *Defining your own modules* — `mod`, inline `{ ... }` modules, and
  modules in separate files. This lesson only *consumes* an existing
  module path.
- *Visibility and `pub`* — items in a module are private by default;
  reaching them from outside needs `pub`. `std::cmp::min` is already
  `pub`.
- *Path roots* — `crate::name`, `self::name`, `super::name`, leading
  `::std::...`.
- *The standard prelude* — a small set of names (`String`, `Vec`,
  `i32`, `println!`, ...) is pre-imported into every Rust program,
  which is why you have written `String` bare. Mechanism deferred.
- *Why `min` works on integers* — `std::cmp::min` is *generic* and uses
  a trait called `Ord`. For now, treat it as a function on two values
  of the same kind.
- *External crates and Cargo dependencies* — third-party libraries also
  use `::` paths, with the crate name as the leading segment.
- All previously deferred items.

## Evidence

See `../evidence/043-nested-module-paths.md` for the corpus-quote map,
the rustc / system toolchain string, the working probe transcript, the
broken-contrast E0425 transcript including the captured `use`-suggesting
`help:` block, and the prerequisite-claim summary.
