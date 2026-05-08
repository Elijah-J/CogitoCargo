---
id: 104-super-and-crate-paths
status: accepted
evidence: ../evidence/104-super-and-crate-paths.md
---

# Walk the module tree with `super::` and `crate::`

## The Move

Lessons 043 and 044 reached items via paths the standard library handed
you: `std::cmp::min`, `std::io::stdin`. Those paths are *absolute* —
they start with a crate name and walk down. Today installs *relative*
paths that start where the calling code lives:

- `super::name` — walks up one level, to the *parent* module, and
  resolves `name` from there.
- `crate::name` — jumps to the *crate root* (the file `rustc` was
  given), regardless of how deeply nested the call site is.

The Reference's path grammar lists both as path qualifiers (`super`,
`crate`, plus `self` and `Self`); each "can only be used as the first
segment" of a path. Today centers these two together because they are
the two relative-anchor keywords real code uses most.

```rust
fn at_root() -> u32 { 1 }

mod outer {
    pub fn at_outer() -> u32 { 2 }
    pub mod inner {
        pub fn use_super() -> u32 {
            super::at_outer()
        }
        pub fn use_crate() -> u32 {
            crate::at_root()
        }
    }
}

fn main() {
    println!("super = {}, crate = {}",
        outer::inner::use_super(),
        outer::inner::use_crate());
}
```

`./demo` prints `super = 2, crate = 1`. From inside `inner`,
`super::at_outer` walks one level up to `outer` and finds `at_outer`
there; `crate::at_root` jumps to the crate root and finds `at_root`
there. Both forms appear in the *leftmost* segment of the path; the
rest of the path (`::name(args)`) is still the lesson-043 call form.

## Mental Model Delta

- *Before*: "Paths start with a crate name and walk down. Inside a
  module body I have not navigated to a sibling module or back to the
  crate root."
- *After*: "There is a second style: *relative* paths anchored by a
  keyword in the leftmost segment. `super::` means *the parent module*
  (one level up, like `..` in a filesystem path). `crate::` jumps to
  *the crate root*. Both sit alongside `self::` (the current module)
  and the absolute form beginning with a crate name."

## Prerequisites

- Installed concepts:
  - Lesson 096 (*load-bearing*): `mod foo { ... }`, items inside a
    module live in a new namespace. Today's probe nests a module inside
    another module to make `super::` observable.
  - Lesson 097: file-based modules. The same `super::` and `crate::`
    keywords work unchanged across file boundaries.
  - Lesson 043 (*load-bearing*): the call form `module::name(args)`.
    Today's prefixes go in the *leftmost* segment of that path; the
    rest (`::name(args)`) is unchanged.
  - Lesson 044: `use` declarations bring a path into scope. Today's
    prefixes also work inside `use` (`use super::Item;`,
    `use crate::module::Item;`); that *composition* is deferred to a
    follow-on move.
  - Lesson 003: the diagnostic four-part map applied to E0433.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
super = 2, crate = 1
```

*Now the contrast.* The crate root has *no parent module*, so a
`super::` from there has nowhere to go. Save as `too_many_supers.rs`:

```rust
fn main() {
    super::missing();
}
```

Compile:

```
error[E0433]: too many leading `super` keywords
 --> too_many_supers.rs:2:5
  |
2 |     super::missing();
  |     ^^^^^ there are too many leading `super` keywords

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0433`.
```

E0433 is the "failed to resolve" family. The
diagnostic phrase *too many leading `super` keywords* says exactly what
went wrong: each `super::` walks up one level, and you cannot walk up
past the crate root.

`crate::` does not have this failure mode. From anywhere — the crate
root, an inline `mod`, or a deeply nested submodule — `crate::name`
resolves `name` from the crate root, full stop.

## What Changed

- Two new keywords in leftmost position: `super::` and `crate::`.
- Both are *relative*: they anchor where the calling code lives, not
  at an external crate name.
- `super::` walks up exactly one module level. (This unlocks reading
  the rmp target's many `super::` lines across `biguint/*.rs`.)
- `crate::` resets to *this* crate's root from any depth. It is the
  Rust 2018+ form for absolute paths inside the current crate.
- A walk above the crate root fires `E0433: too many leading \`super\`
  keywords`.
- Position summary (today's two centered, neighbors named):

| Leftmost prefix | Resolves from                       |
|-----------------|-------------------------------------|
| `crate::`       | the crate root                      |
| `super::`       | the parent of the current module    |
| `self::`        | the current module (often elidable) |
| `std::`, etc.   | an external crate (absolute)        |

## Check Yourself

You write `quiz.rs`:

```rust
fn at_root() -> u32 { 7 }

mod a {
    pub fn at_a() -> u32 { 8 }
    pub mod b {
        pub fn from_b() -> u32 {
            super::at_a() + crate::at_root()
        }
    }
}

fn main() {
    println!("{}", a::b::from_b());
}
```

(a) What does `./quiz` print, and why?

(b) If you change `super::at_a()` to `super::super::super::at_a()`,
    what happens? What E-code fires?

(*Answers: (a) `15` — `super::at_a` walks from `b` up to `a` and finds
`at_a` (8); `crate::at_root` jumps to the crate root and finds
`at_root` (7); `8 + 7 = 15`. (b) From `b` you can walk up at most
*two* levels (to `a`, then to the crate root). A *third* `super::`
fires `error[E0433]: too many leading \`super\` keywords`. Probe 4 in
the evidence appendix captures the same E0433 from a one-level-deep
context.*)

## What To Ignore For Now

- *`self::name`* — the third relative qualifier; means "the current
  module," equivalent to bare `name` in most positions. Named in the
  Reference grammar but rarely used.
- *`Self`* (capital S) — the type-level qualifier from lesson 100; a
  different keyword in a different namespace.
- *Multi-level `super::super::...`* — chains walk up multiple levels;
  today only exercises one. Behaviour at the limit is the same E0433.
- *Path-prefix in `use`* — `use super::Item;` and
  `use crate::module::Item;` compose today's prefixes with lesson 044.
  Natural follow-on, deferred.
- *Edition differences* — Rust 2015 used a leading `::` for the crate
  root; Rust 2018 introduced `crate::`. This run uses edition 2024.
- *`$crate`* — a macro-only qualifier; blocked on macros.
- *Cross-crate paths* — depending on another crate by name; blocked
  on the binary-and-library-crate move.

## Evidence

See `../evidence/104-super-and-crate-paths.md` for the corpus-quote
map, the toolchain string, the working probe transcript, the centered
E0433 contrast, the operational `super`-versus-`crate` swap probe, the
multi-level `super::super::` probe, and the prerequisite-claim summary.
