---
id: 105-pub-use-re-export
status: accepted
evidence: ../evidence/105-pub-use-re-export.md
---

# Re-export an item with `pub use Path::Item;`

## The Move

Lesson 044 installed `use Path::Item;` — bring a name into the *current
file's* (or *current module's*) scope. From outside that scope, the
short name is invisible; callers still need the original path. Today's
move adds one keyword in front:

```rust
mod inner {
    pub mod hidden {
        pub fn value() -> u32 {
            42
        }
    }
    pub use self::hidden::value;
}

fn main() {
    println!("via re-export: {}", inner::value());
    println!("via original:  {}", inner::hidden::value());
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
via re-export: 42
via original:  42
```

The `pub use self::hidden::value;` line *does two things at once*. It
brings `value` into `inner`'s local scope, the same as a plain
lesson-044 `use` would. *And* it makes `value` reachable from *outside*
`inner` under the path `inner::value`. So `fn main` — outside `inner` —
can write `inner::value()` directly, even though the function `value`
lives one level deeper at `inner::hidden::value`.

The original path `inner::hidden::value` keeps working. `pub use` adds a
second public path to the same function; it does not move or remove the
first.

The Reference calls this *re-exporting*: a `use` declaration "qualified
by the `pub` keyword" serves to "*re-export* a name."

(This unlocks reading the rmp target's `pub use basic::BigUInt;` in
`biguint/mod.rs` — exactly today's shape. The pattern lets callers say
`bignum::biguint::BigUInt` instead of the longer
`bignum::biguint::basic::BigUInt`.)

## Mental Model Delta

- *Before*: "`use module::Item;` brings `Item` into the current
  module's *local* scope. From outside, callers still need the original
  path `module::Item`."
- *After*: "`pub use module::Item;` does *both* — brings `Item` into
  local scope *and* re-exports it from the current module. After this,
  external callers can write either the original path *or*
  `current_module::Item` (the new re-export). Either way, the same
  function is called. The `pub` is the same `pub` from lesson 096:
  *items* without `pub` are private to the module; `use` declarations
  are *items* and follow that same rule."

## Prerequisites

- Installed concepts:
  - Lesson 044 (*load-bearing*): `use Path::Item;` brings the final
    segment into scope. Today is "the same `use`, but with `pub` added
    in front."
  - Lesson 096 (*load-bearing*): `mod foo { ... }` and `pub` on a
    function item; the privacy-by-default rule for items inside a
    module. Today applies the *same* `pub` keyword to a *different*
    item kind: a `use` declaration. The contrast probe's E0603
    ("private function import") is the same E-code lesson 096
    installed.
  - Lesson 043: `module::name(args)` call form. Today's
    `inner::value()` and `inner::hidden::value()` are both this shape.
  - Lesson 003: the diagnostic four-part map applied to E0603.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs` and run it; the transcript matches
the working probe in *The Move*. (The leading `self::` in
`pub use self::hidden::value;` is just lesson 044's `use` machinery —
the Reference's own re-export example uses the same
`pub use self::foo::{bar, baz};` shape. Today's centered move is the
`pub` on the front.)

*Now the contrast — drop the `pub`.* Save `no_pub.rs`:

```rust
mod inner {
    pub mod hidden {
        pub fn value() -> u32 {
            42
        }
    }
    use self::hidden::value;
}

fn main() {
    println!("via re-export: {}", inner::value());
}
```

Compile:

```
error[E0603]: function import `value` is private
  --> no_pub.rs:11:42
   |
11 |     println!("via re-export: {}", inner::value());
   |                                          ^^^^^ private function import
   |
note: the function import `value` is defined here...
  --> no_pub.rs:7:9
   |
 7 |     use self::hidden::value;
   |         ^^^^^^^^^^^^^^^^^^^
note: ...and refers to the function `value` which is defined here
  --> no_pub.rs:3:9
   |
 3 |         pub fn value() -> u32 {
   |         ^^^^^^^^^^^^^^^^^^^^^ you could import this directly
help: consider importing this function instead
   |
11 |     println!("via re-export: {}", inner::hidden::value());
   |                                          ++++++++

error: aborting due to 1 previous error
```

(rustc emits a second `help:` block — abbreviated here — proposing a
local `use` import; the appendix has the full transcript.)

Same E0603 lesson 096 installed, with extra phrasing: *"private
function import"*. The `use` itself is now an item, and like any other
item it is private by default. The `note:` block points back at the
`use` line, *and* a second `note:` chases the chain one step further to
the original `pub fn value()` — rustc shows you the redirect that the
private `use` was hiding. Restore `pub` and the program prints both
lines.

## What Changed

- A new item shape: `pub use Path::Item;`. Same syntactic position as a
  plain `use`, with `pub` in front (same `pub` from lesson 096).
- Effect: `Item` is brought into the current module's local scope *and*
  re-exported under the current module's path. Two effects from one
  line.
- The original path keeps working. `pub use` adds a second public path;
  it does not remove the first.
- Without `pub`, the `use` is private — the bare name works *inside*
  the module but is invisible from outside, firing E0603 ("private
  function import") if a caller tries.

## Check Yourself

You write `quiz.rs`:

```rust
mod outer {
    pub mod deep {
        pub fn answer() -> u32 { 7 }
    }
    pub use self::deep::answer;
}

fn main() {
    let a = outer::answer();
    let b = outer::deep::answer();
    println!("a = {a}, b = {b}");
}
```

(a) What does `./quiz` print?

(b) If you change `pub use self::deep::answer;` to plain
    `use self::deep::answer;`, which line of `fn main` fires an error,
    and what E-code?

(*Answers: (a) `a = 7, b = 7` — both call paths reach the same function;
the `pub use` adds `outer::answer` as a second public path, the
original `outer::deep::answer` keeps working. (b) The
`outer::answer()` call on line 9 fires `error[E0603]: function import
\`answer\` is private`; the `outer::deep::answer()` call on line 10
still resolves because `pub mod deep` and `pub fn answer` make the
original path public. The contrast probe in the evidence appendix
captures this exact shape.*)

## What To Ignore For Now

- *`pub(crate) use ...` and `pub(super) use ...`* — restricted-visibility
  re-exports; composes today with lesson 103. Future move.
- *`pub use Path::Item as Alias;`* — re-export under a *different* name.
  Future move.
- *Glob re-exports `pub use module::*;`* — re-export *every* public
  item. Future move.
- *Re-export of an entire module* — `pub use module;` (no final
  segment). Future move.
- *Re-exporting trait items* — blocked on trait machinery.
- *Cycle detection in re-export chains* — Reference
  use-declarations.md line 78 names this as a compile-time error. Out
  of scope today.
- *Path-resolution rules for the `use`-path itself* — why
  `pub use self::hidden::value;` works but `pub use hidden::value;`
  (no `self::`) fails under `rustc demo.rs`. Edition-dependent and
  orthogonal to today's `pub` move; the appendix records the
  observation.

## Evidence

See `../evidence/105-pub-use-re-export.md` for the corpus-quote map,
the toolchain string, the working probe transcript, the centered E0603
contrast, the auxiliary plain-`use`-from-inside witness, and the
prerequisite-claim summary.
