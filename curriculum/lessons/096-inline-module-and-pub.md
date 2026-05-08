---
id: 096-inline-module-and-pub
status: accepted
evidence: ../evidence/096-inline-module-and-pub.md
---

# Author your own module with `mod foo { ... }` and expose one item with `pub`

## The Move

Lessons 043 and 044 reached `std::cmp::min` by walking a path the
standard library had set up. Today you *author a module yourself*. The
move composes two inseparable pieces:

```rust
mod foo {
    pub fn hi() {
        println!("hi from foo");
    }
}

fn main() {
    foo::hi();
}
```

1. **Declare a submodule inline**: `mod foo { ... }`. The keyword `mod`,
   a name, then a brace-enclosed body. Items inside the braces (`fn`,
   `struct`, more `mod` blocks) live in a new namespace called `foo`.
2. **Expose one item**: write `pub` before `fn hi`. Items inside a
   module are *private by default*; `pub` lets code outside reach the
   item via `foo::item`.

Compile silently. `./demo` prints `hi from foo`. The call site
`foo::hi()` reuses the lesson-043 `path::name(args)` shape — except
*you* declared the module on the left of `::`.

(This unlocks reading the rmp target's `pub mod bigint;`, its
`pub use basic::BigUInt;` re-export, and `pub fn zero()` items.
`pub mod`, `pub use`, file-based `mod foo;`, `pub(super)`, and `pub`
on struct fields are all deferred.)

## Mental Model Delta

- *Before*: "Modules are namespaces the standard library hands me. I
  reach in with `std::cmp::min(...)`. Items inside `std` already work
  — I never thought about whether they could have been hidden."
- *After*: "I can declare a module myself with `mod foo { ... }`. Items
  inside the braces are *private by default* — code outside the module
  cannot reach them, even though I wrote them in the same file. Adding
  `pub` is what makes one item reachable from outside via `foo::item`.
  The standard library did this same thing for me long ago; that is why
  `std::cmp::min` was already callable."

## Prerequisites

- Installed concepts:
  - Lesson 002 (`fn main`): the body of `fn main` runs when the
    executable launches.
  - Lesson 008 (define and call a function): `fn hi() { ... }` defines
    one. The definition shape is unchanged today; the call site reaches
    it via `foo::hi()` instead of bare `hi()`.
  - Lesson 011 (`println!` positional `{}`): the body of `hi`.
  - Lesson 043 (*load-bearing*): the call form `module::name(args)`
    reaches a function in a module. Today's `foo::hi` is that shape with
    *your* module on the left of `::`.
  - Lesson 044 (`use` declaration): named only as the close cousin
    already seen. The E0603 contrast probe's `help:` block proposes
    `use foo::hi;` — that is lesson 044's mechanism on your module, not
    a new move.
  - Lesson 003 (rustc diagnostic four-part map): used to read E0603.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Save the program above as `demo.rs` in a fresh directory:

```console
$ rustc demo.rs
$ ./demo
hi from foo
```

`mod foo { ... }` is one *item* — same item position `fn` lives in.
Inside it sits `pub fn hi() { ... }`. From `fn main`, `foo::hi();`
reaches across the module boundary with the lesson-043 shape.

*Now the contrast — the centered teaching moment.* Save `broken.rs`
with `pub` deleted from `fn hi`:

```rust
mod foo {
    fn hi() {
        println!("hi from foo");
    }
}

fn main() {
    foo::hi();
}
```

Compile:

```
error[E0603]: function `hi` is private
 --> broken.rs:8:10
  |
8 |     foo::hi();
  |          ^^ private function
  |
note: the function `hi` is defined here
 --> broken.rs:2:5
  |
2 |     fn hi() {
  |     ^^^^^^^

error: aborting due to 1 previous error
```

Read it with the lesson 003 map. Headline carries `E0603`; location
sits at the *use site* (`broken.rs:8:10`); caret under `hi`; a `note:`
block points back at the definition with a second `-->`. The
diagnostic states today's rule: without `pub`, the function exists, the
path resolves to it, but crossing the module boundary is rejected at
compile time. Restore `pub` and the program prints the same line.

(Today's `mod foo { ... }` is the *inline* form. Rust also has a
*file-based* form `mod foo;` where the body lives in `foo.rs`. Same
keyword, same privacy rule, separate move.)

## What Changed

- You can declare your own submodule with `mod foo { ... }` at module
  scope. The body lives in braces; the name becomes a new namespace.
- Items inside a module are *private to the module by default*. Code
  outside the module cannot reach them, even by their full path.
- Adding `pub` to an item declaration makes that item accessible from
  outside via `module::item`. Only the keyword changes; the function's
  body and the call site stay the same.
- Without `pub`, the diagnostic is `error[E0603]: ... is private` — a
  new E-code in your collection. The caret sits at the *call site*, and
  a `note:` block points back at the definition.

## Check Yourself

You write `tiny.rs`:

```rust
mod bar {
    pub fn shout() {
        println!("BAR");
    }

    fn whisper() {
        println!("bar");
    }
}

fn main() {
    bar::shout();
}
```

(a) Does `rustc tiny.rs` accept the program?

(b) What single line does `./tiny` print?

(c) If you change line 12 to `bar::whisper();`, which E-code appears in
the headline, and which identifier does the caret underline?

(d) Why is `bar::shout()` reachable from `fn main` but `bar::whisper()`
is not?

(*Answers: (a) Yes. (b) `BAR`. (c) E0603, with the caret under
`whisper` at the call site and a `note:` pointing at `fn whisper` inside
`mod bar`. (d) Only `shout` is `pub`. `whisper` is private to `mod bar`,
so the path resolves but the privacy check at the module boundary
fails.*)

## What To Ignore For Now

This lesson installs only two coupled pieces: declare an inline
submodule, expose one item with `pub`. Every nearby module/visibility
feature below is real and deferred:

- *File-based modules* — `mod foo;` (semicolon-terminated, no braces)
  resolves to `foo.rs` or `foo/mod.rs`. Same keyword, same privacy
  rule, body in a separate file. The follow-on for lesson 097.
- *Restricted visibility* — `pub(super)`, `pub(crate)`, `pub(in path)`,
  `pub(self)`. The rmp target uses `pub(super)` heavily.
- *Visibility on the `mod` declaration itself* — `pub mod foo { ... }`.
  Today's `mod foo { ... }` is private at the crate root; making the
  module itself public is a different rule. The rmp target's `lib.rs`
  uses `pub mod bigint;`.
- *`pub` on struct fields* — `struct Foo { pub x: i32 }`. Today only
  puts `pub` on the function, not on a field.
- *`pub` on enum variants* — enums are not yet installed.
- *`pub use` re-exports* — `pub use basic::BigUInt;` in the rmp
  target. Composes today's `pub` with lesson 044's `use`.
- *`pub trait` and `pub impl`* — trait machinery is blocked.
- *`crate::`, `super::`, `self::` path prefixes* — alternative path
  roots inside modules. Today uses only `foo::hi`.
- *Nested submodules* — `mod foo { mod bar { ... } }`.
- *Privacy-by-default at the crate root* — a `fn` defined directly in
  the crate root is also private-by-default, but in a single-binary
  `rustc` build with no other crates you cannot tell. That is *why*
  today required a `mod` block to make the rule observable.
- *The `Visibility?` grammar slot on every item kind* — today only
  exercises it on a function.
- All previously deferred items.

## Evidence

See `../evidence/096-inline-module-and-pub.md` for the corpus-quote
map, the rustc / system toolchain string, the working probe transcript,
the E0603 missing-`pub` contrast, the auxiliary E0425 bare-call probe
that grounds the "module is a namespace" framing, and the
prerequisite-claim summary.
