---
id: 093-standard-library-prelude
status: accepted
evidence: ../evidence/093-standard-library-prelude.md
---

# The standard library prelude — names that are auto-imported

## The Move

You have already used several standard-library names without writing a
`use` line: `String` (lesson 042) and `Result`/`Ok`/`Err` (lesson 052).
You have also used names that *did* require `use std::io;` (lesson
044) or a full path `std::io::stdin()` (lesson 050). Today's move
names the rule that splits the two groups: the *standard library
prelude*.

A *prelude* is "a collection of names that are automatically brought
into scope of every module in a crate" (Reference, names/preludes).
The Rust compiler implicitly imports the standard library prelude into
every module of every crate you build. Names inside it can be written
bare; names outside it cannot.

```rust
fn main() {
    let s: String = String::new();
    let v: Vec<i32> = Vec::new();
    let r: Result<i32, String> = Ok(42);
    let opt: Option<i32> = Some(7);
    println!("s = {:?}", s);
    println!("v = {:?}", v);
    println!("r = {:?}", r);
    println!("opt = {:?}", opt);
}
```

Every name on the right of an `=` here — `String`, `Vec`, `Ok`, `Some`
— is a standard-library name reached with no `use` line and no
`std::` prefix. The prelude is why. (`{:?}` is another `println!`
slot, like lesson 011's `{}`; it prints the *Debug* representation of
a value. Treat it as a different formatter for now and defer the
`Display`-vs-`Debug` distinction.)

## Mental Model Delta

- Before: "Lesson 042's `String::new()` worked bare; lesson 050's
  `std::io::stdin()` needed the full path; lesson 044's `use std::io;`
  was a third option. I have been pattern-matching which names need a
  `use` and which don't, but I do not have the rule."
- After: "There is one rule. The standard library prelude is the set
  of standard-library names automatically in scope of every module.
  Names in the prelude (`String`, `Vec`, `Option`/`Some`/`None`,
  `Result`/`Ok`/`Err`, ...) you write bare. Names outside the prelude
  (`std::io::stdin`, `std::collections::HashMap`, ...) you reach with
  the full path or with a `use` declaration."

## Prerequisites

- Installed concepts:
  - Lesson 042 (load-bearing): `String::new()` works bare. Today says
    *why*: `String` is in the prelude.
  - Lesson 044 (load-bearing): `use module::name;` brings the final
    segment of a path into the file's scope. Today's role: the
    mechanism for non-prelude names.
  - Lesson 050 (load-bearing): `std::io::stdin()` written as a full
    path. Today says *why* it needs one: `std::io::stdin` is not in
    the prelude.
  - Lesson 052 (load-bearing): `Result<T, E>` with `Ok`/`Err` worked
    bare; that lesson noted in passing that "`Result` (along with
    `Ok` and `Err`) is in the *prelude*." Today centers that framing.
  - Lessons 002, 005, 011: `fn main`, `let name: TYPE = value;`,
    `println!` with a positional `{}` slot. Today extends only by
    adding the `{:?}` formatter, glossed in one sentence.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

In a fresh empty directory, create `demo.rs` with the source above.
Compile and run:

```console
$ rustc demo.rs
$ ./demo
s = ""
v = []
r = Ok(42)
opt = Some(7)
```

Four prelude names exercised in one program. `String::new()` produces
an empty `String` (Debug-printed as `""`). `Vec::new()` produces an
empty `Vec` (Debug-printed as `[]`). `Ok(42)` and `Some(7)` are
constructor calls (lesson 052) on `Result` and `Option`. Nowhere in
this file is there a `use` declaration, and nowhere is there a
`std::...` prefix. rustc resolves all of these names because the
prelude has already pulled them into scope.

*Predict.* What if you try the same shape with a name that is *not*
in the prelude? Save `broken.rs`:

```rust
fn main() {
    let m = HashMap::new();
}
```

`HashMap` lives at `std::collections::HashMap` and is not in the
prelude. Compile:

```
error[E0433]: cannot find type `HashMap` in this scope
 --> broken.rs:2:13
  |
2 |     let m = HashMap::new();
  |             ^^^^^^^ use of undeclared type `HashMap`
  |
help: consider importing this struct
  |
1 + use std::collections::HashMap;
  |
```

The compiler's `help:` line literally states the rule today installs:
because `HashMap` is not in the prelude, you must bring it into scope
explicitly — either with the `use` declaration the diagnostic
suggests (lesson 044's mechanism), or with the full path
`std::collections::HashMap::new()` (lesson 050's mechanism).

(Full transcripts in `../evidence/093-standard-library-prelude.md`.)

## What Changed

- The *standard library prelude* is the named language feature that
  explains why `String`, `Vec`, `Option`/`Some`/`None`, and
  `Result`/`Ok`/`Err` work as bare names. Every Rust module you
  write has the prelude implicitly in scope.
- The split is binary. A standard-library name is either *in* the
  prelude (write it bare, like `String::new()`) or *not* (use the
  full path like `std::io::stdin()`, lesson 050, or a `use`
  declaration like `use std::io;`, lesson 044). Names outside the
  prelude have no third option.
- The `cargo new` default edition is 2024, so the prelude in scope
  is `std::prelude::rust_2024`. Different editions select slightly
  different prelude modules — today only names the rule and a few
  members, not the full enumeration.
- The compiler's E0433 `help:` line for an unimported standard-library
  name is a direct witness: rustc knows the name's path and offers a
  `use` line that would put it in scope. That line exists *because*
  the name is not in the prelude.

## Check Yourself

- Why does `let s = String::new();` compile with no `use` and no
  `std::` prefix?

- `let stdin = std::io::stdin();` (lesson 050) needs the full path,
  but `let s = String::new();` does not. State the one-sentence rule
  that explains the difference.

- Compile `fn main() { let m = HashMap::new(); }` and the compiler
  emits E0433 with `help: consider importing this struct` plus
  `use std::collections::HashMap;`. What does the *existence* of that
  `help:` suggestion tell you about `HashMap` and the prelude?

(Answers: (1) `String` is in the standard library prelude — implicitly
in scope of every module, no `use` needed. (2) `String` is in the
prelude; `std::io::stdin` is not. The prelude split decides whether a
standard-library name needs `use`/full-path or works bare. (3) That
`HashMap` is *not* in the prelude. The compiler is suggesting the
exact `use` declaration that would bring it into scope, which is only
necessary because the implicit prelude does not already include it.)

## What To Ignore For Now

- *The full list of names in the prelude.* Reference's
  `std::prelude::rust_2024` page enumerates them; today names the
  rule and a few representative members.
- *Edition-specific prelude differences.* The 2021/2024 preludes
  added the `TryInto`/`TryFrom`/`FromIterator` traits over earlier
  editions; the trait machinery is deferred.
- *The four other preludes named in the Reference.* The extern
  prelude, language prelude, `macro_use` prelude, and tool prelude
  exist and are listed alongside the standard library prelude.
  Today only the standard library prelude is in scope.
- *`#![no_std]`*, *`#![no_implicit_prelude]`*, and re-exporting
  `std::prelude::*` from a custom module — crate-level configuration
  and module-system depth.
- *Glob imports `use std::io::*;`* and how they interact with the
  prelude.
- *The `Display` vs `Debug` formatting traits.* `{:?}` is named here
  as another `println!` formatter that prints any value implementing
  `Debug`; the trait machinery is deferred.
- *The `Box` type from `std::boxed`* (named in the Reference's
  prelude discussion) — not yet in the audience's vocabulary.
- All previously deferred items.

## Evidence

See `../evidence/093-standard-library-prelude.md` for the corpus-quote
map, the rustc / system toolchain string, the working-probe transcript,
the contrast-probe E0433 transcript, and the prerequisite-claim
summary.
