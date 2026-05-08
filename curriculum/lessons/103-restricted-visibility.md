---
id: 103-restricted-visibility
status: accepted
evidence: ../evidence/103-restricted-visibility.md
---

# Restrict an item's reach with `pub(super)` and `pub(crate)`

## The Move

Lesson 096 installed plain `pub`: an item with `pub` is reachable from
outside its module. Today extends that single rule to a *family*. The
Reference's `Visibility` grammar admits five shapes — today centers the
two restricted shapes used most in real code:

```rust
mod inner {
    pub(super) fn for_super() -> u32 { 1 }
    pub(crate) fn for_crate() -> u32 { 2 }
}

fn main() {
    println!("super = {}, crate = {}", inner::for_super(), inner::for_crate());
}
```

`./demo` prints `super = 1, crate = 2`. The `fn main` call site lives
in the crate root, which *is* `inner`'s parent module, so both
restricted forms resolve.

The two new keywords-in-parens narrow `pub`'s reach:

- `pub(super)` — visible only to the *parent* module (the immediate
  enclosing `mod`). Code outside the parent's neighborhood cannot reach
  the item, even by full path.
- `pub(crate)` — visible *anywhere within this crate*, but not to any
  other crate that links against this one as a library.

Both diagnose with the same `E0603 ... is private` headline lesson 096
installed when the visibility check fails. The new content is *which*
locations count as "outside" for each modifier.

## Mental Model Delta

- *Before*: "Lesson 096 said `pub` opens an item to the outside world.
  Without `pub`, it's private to the module. Two states."
- *After*: "There's a *family* of `pub(...)` forms with different reach.
  Plain `pub` reaches everywhere, including library consumers.
  `pub(crate)` reaches everywhere *inside this crate* but stops at the
  crate boundary. `pub(super)` reaches only into the immediate parent
  module's namespace. Same E0603 fires when the check fails — only the
  *radius* of allowed callers changes."

## Prerequisites

- Installed concepts:
  - Lesson 096 (*load-bearing*): `mod foo { ... }`, `pub` on items, the
    privacy-by-default rule, `module::item` access, and E0603. Today
    extends 096 by adding two keywords-in-parens at the same position.
  - Lesson 003 (cited): the four-part diagnostic map applied to E0603.
  - Lessons 002, 005, 008, 011, 043 (cited): unchanged supporting
    machinery (`fn main`, `module::name(args)`, `println!`).
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
super = 1, crate = 2
```

*Now the contrast.* Add a third item inside `mod inner` with no
visibility marker, and try to call it from `main`. Save as
`private_call.rs`:

```rust
mod inner {
    pub(super) fn for_super() -> u32 { 1 }
    pub(crate) fn for_crate() -> u32 { 2 }
    fn private() -> u32 { 3 }
}

fn main() {
    println!("super = {}, crate = {}, private = {}",
        inner::for_super(),
        inner::for_crate(),
        inner::private());
}
```

Compile:

```
error[E0603]: function `private` is private
  --> private_call.rs:11:16
   |
11 |         inner::private());
   |                ^^^^^^^ private function
   |
note: the function `private` is defined here
  --> private_call.rs:4:5
   |
 4 |     fn private() -> u32 { 3 }
   |     ^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error
```

Same E0603 you saw in lesson 096: headline E-code, caret at the use
site, `note:` block at the definition. Only the unmarked item fails;
both `pub(...)` items in the same module resolve fine. Today's
modifiers *opt in* to the access rule, just like plain `pub` — without
some form of `pub`, the item is private.

## What Changed

- `pub(super)` and `pub(crate)` go in the same syntactic position as
  plain `pub` — in front of `fn`, `struct`, `const`, etc.
- The diagnostic on a failed visibility check is the same E0603 from
  lesson 096; only *which* call sites pass changes.
- Visibility radius summary:

| Modifier      | Reach                                       |
|---------------|---------------------------------------------|
| `pub`         | Everywhere, including library consumers     |
| `pub(crate)`  | Everywhere *within this crate*              |
| `pub(super)`  | Only the immediate parent module            |
| (no marker)   | Only the module the item is declared in     |

## Check Yourself

You write `quiz.rs`:

```rust
mod outer {
    pub mod inner {
        pub(super) fn for_super() -> u32 { 1 }
        pub(crate) fn for_crate() -> u32 { 2 }
    }
}

fn main() {
    println!("{}", outer::inner::for_crate());
    println!("{}", outer::inner::for_super());
}
```

(a) Does `for_crate` resolve from `fn main`? Why?

(b) Does `for_super` resolve from `fn main`? What E-code fires if not?

(*Answers: (a) Yes — `pub(crate)` reaches anywhere in the crate. (b)
No — `pub(super)` reaches only `outer` (the parent of `inner`); `fn
main` is outside `outer`, so the check fails with `error[E0603]:
function \`for_super\` is private`. Probe 3 in the evidence appendix
captures this transcript.*)

## What To Ignore For Now

Real and deferred:

- *`pub(self)`* — the fifth `Visibility` form per Reference
  visibility-and-privacy.md line 145; equivalent to no `pub` at all.
  Almost never used.
- *`pub(in path)`* — the most general form, takes a module path
  argument: `pub(in crate::foo::bar)`. Rare in practice.
- *Cross-crate visibility* — what plain `pub` actually means when
  another crate links against this one. Today's `pub(crate)` is the
  operational opposite of cross-crate-visible; the deeper rule waits
  on the binary-and-library-crate move.
- *Visibility on the `mod` declaration itself* — `pub(super) mod foo
  { ... }` versus `mod foo { pub(super) fn ... }`. Today's modifiers
  appear only in front of *items inside* a module, not on the module
  declaration.
- *Restricted visibility on struct fields* — `struct S { pub(super)
  limbs: Vec<u64> }`. Composes today's modifier with lesson 095's
  field declarations; the rmp target uses this on `BigUInt`'s `limbs`
  field; the natural next move.
- *Restricted `pub` on enum variants* — composes today with lesson
  098/099.
- *`pub(crate) use foo::Bar;` re-exports* — composes today with
  lesson 044's `use`.
- *Trait associated items* — blocked on trait machinery.
- *Edition-specific differences* — Rust 2015's `pub(in path)` syntax
  differs from 2018+; this run uses edition 2024 throughout.
- All previously deferred items.

## Evidence

See `../evidence/103-restricted-visibility.md` for the corpus-quote
map, the rustc / system toolchain string, the working probe transcript,
the centered E0603 contrast, the `pub(super)` out-of-reach auxiliary,
the `pub(crate)` reach-through auxiliary, and the prerequisite-claim
summary.
