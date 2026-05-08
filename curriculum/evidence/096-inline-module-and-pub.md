# Evidence — 096-inline-module-and-pub

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` -> `Darwin x86_64`
- Probes run in `mktemp -d` directories on this host. Same toolchain
  family as recent accepted lessons (082-095).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/096-inline-module-and-pub.rs`
is the working two-piece probe verbatim, with a header comment naming
the expected output and the contrast probes captured below.

## Sources

### `output/docs/rust/book/ch07-02-defining-modules-to-control-scope-and-privacy.md`

The Book's *Defining Modules to Control Scope and Privacy* chapter.
Three load-bearing passages:

#### Lines 41-44 — the private-by-default rule and the role of `pub`

> **Private vs. public**: Code within a module is private from its
> parent modules by default. To make a module public, declare it with
> `pub mod` instead of `mod`. To make items within a public module
> public as well, use `pub` before their declarations.

Corpus warrant for the lesson's centered claim "items inside a module
are *private to the module by default*; `pub` lets code outside reach
the item via `foo::item`." The Book's framing names two layers of
visibility (module-itself vs. items-inside); today's lesson only
installs the items-inside layer because the outer `mod foo { ... }`
sits at the crate root in a single-binary build, and `pub mod` is
listed as deferred.

#### Lines 127-152 — the inline `mod NAME { ... }` syntax (Listing 7-1)

> ```rust
> mod front_of_house {
>     mod hosting {
>         fn add_to_waitlist() {}
>
>         fn seat_at_table() {}
>     }
>
>     mod serving {
>         fn take_order() {}
>
>         fn serve_order() {}
>
>         fn take_payment() {}
>     }
> }
> ```
>
> *Listing 7-1: A `front_of_house` module containing other modules
> that then contain functions*
>
> We define a module with the `mod` keyword followed by the name of
> the module (in this case, `front_of_house`). The body of the module
> then goes inside curly brackets. Inside modules, we can place other
> modules, as in this case with the modules `hosting` and `serving`.
> Modules can also hold definitions for other items, such as structs,
> enums, constants, traits, and as in Listing 7-1, functions.

Corpus warrant for the lesson's piece (1) framing — the inline
`mod NAME { ... }` syntax. Today's `mod foo { ... }` is exactly this
shape with `foo` substituted and a single inner item.

### `output/docs/rust/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md`

The Book's *Paths for Referring to an Item in the Module Tree* chapter.
Three load-bearing passages on `pub` and the privacy check:

#### Lines 121-127 — privacy default and what `pub` is for

> The error messages say that module `hosting` is private. In other
> words, we have the correct paths for the `hosting` module and the
> `add_to_waitlist` function, but Rust won't let us use them because
> it doesn't have access to the private sections. In Rust, all items
> (functions, methods, structs, enums, modules, and constants) are
> private to parent modules by default. If you want to make an item
> like a function or struct private, you put it in a module.

Corpus warrant for the lesson's *path-resolves-but-privacy-check-fails*
framing. Today's E0603 contrast probe witnesses this exact behaviour:
the path `foo::hi` resolves (the function exists in the named module),
but the use site is rejected because `hi` is private.

#### Lines 142-156 — the `pub` keyword exposing items (Listing 7-5/7-7)

> Let's return to the error in Listing 7-4 that told us the `hosting`
> module is private. We want the `eat_at_restaurant` function in the
> parent module to have access to the `add_to_waitlist` function in
> the child module, so we mark the `hosting` module with the `pub`
> keyword [...]

The chapter walks through Listings 7-5 (`pub mod hosting`), 7-6 (still
fails because `add_to_waitlist` itself is not `pub`), and 7-7 (working
version with `pub mod hosting` AND `pub fn add_to_waitlist`). Today's
lesson collapses this into one move because the outer module is at the
crate root: only the *inner* `pub fn` matters for the contrast. The
Book does this same shape in Listing 7-3's stripped-down form before
the privacy walkthrough begins.

#### Lines 215-217 — the privacy rule covers function items

> The errors in Listing 7-6 say that the `add_to_waitlist` function is
> private. The privacy rules apply to structs, enums, functions, and
> methods as well as modules.

Corpus warrant for "the function-item position takes `pub` the same
way." Today only exercises the function-item position; the lesson's
*What To Ignore For Now* names struct fields, enum variants, methods,
and the `mod` declaration itself as separate cases.

### `output/docs/rust/reference/items/modules.md`

The Reference's formal grammar for `mod`. Lines 10-15 are load-bearing:

> [Module] →
>     unsafe? mod IDENTIFIER ;
>   | unsafe? mod IDENTIFIER {
>       InnerAttribute*
>       Item*
>     }

Reference warrant for the two `mod` shapes: the semicolon-terminated
file-based form and the brace-enclosed inline form. Today's lesson
exercises only the second. Lines 19, 23 also matter:

> A module is a container for zero or more items.
> A *module item* is a module, surrounded in braces, named, and
> prefixed with the keyword `mod`. A module item introduces a new,
> named module into the tree of modules making up a crate.

Reference warrant for "items inside live in a new namespace called
`foo`."

### `output/docs/rust/reference/visibility-and-privacy.md`

Lines 9-15 — the formal `Visibility` grammar:

> [Visibility] →
>     pub
>   | pub ( crate )
>   | pub ( self )
>   | pub ( super )
>   | pub ( in SimplePath )

Today exercises only the bare `pub` form. The four restricted forms
(`pub(crate)`, `pub(self)`, `pub(super)`, `pub(in path)`) are named in
*What To Ignore For Now* and deferred.

Lines 29-31 — the privacy default:

> By default, everything is *private*, with two exceptions: Associated
> items in a `pub` Trait are public by default; Enum variants in a
> `pub` enum are also public by default. When an item is declared as
> `pub`, it can be thought of as being accessible to the outside world.

Reference warrant for the lesson's "private by default" framing. The
two stated exceptions (`pub` Trait associated items, `pub` enum
variants) are not exercised today: traits are blocked, enums are not
yet installed.

Lines 52-56 — the access rules:

> With the notion of an item being either public or private, Rust
> allows item accesses in two cases:
>
> 1. If an item is public, then it can be accessed externally from
>    some module `m` if you can access all the item's ancestor modules
>    from `m`. You can also potentially be able to name the item
>    through re-exports. See below.
> 2. If an item is private, it may be accessed by the current module
>    and its descendants.

Reference warrant for "items inside the module are private to the
module *and its descendants*; outside the module they are not
accessible." The full chain-of-ancestors rule is named here but not
unpacked in the lesson — today's case has only one module level so the
chain has length one.

### `output/docs/rust/error_codes/E0603.md`

The error-code page for E0603 — the privacy violation today's contrast
probe witnesses:

> A private item was used outside its scope.
>
> Erroneous code example:
>
> ```rust
> mod foo {
>     const PRIVATE: u32 = 0x_a_bad_1dea_u32; // This const is private,
>                                             // so we can't use it
>                                             // outside of the `foo`
>                                             // module.
> }
>
> println!("const value: {}", foo::PRIVATE); // error: constant
>                                            //        `PRIVATE` is
>                                            //        private
> ```
>
> In order to fix this error, you need to make the item public by
> using the `pub` keyword. Example:
>
> ```rust
> mod foo {
>     pub const PRIVATE: u32 = 0x_a_bad_1dea_u32; // We set it public
>                                                 // by using the
>                                                 // `pub` keyword.
> }
>
> println!("const value: {}", foo::PRIVATE); // ok!
> ```

Direct corpus warrant: the lesson's E0603 contrast probe shape is
structurally bit-for-bit the corpus erroneous-code example with `const`
swapped for `fn` (the audience knows `fn` from lesson 008, not `const`
— `const` was installed at lesson 075 but is not load-bearing for
today). The corpus example uses `mod foo { ... }` inline, accesses the
inner item via `foo::PRIVATE`, and fixes the error by adding `pub`.
Today's probe substitutes `pub fn hi() { ... }` for `pub const ...`
and otherwise preserves the shape.

## Probes

### Probe 1 — working: inline module + `pub fn`

Source (`/tmp/.../demo.rs`, also at
`observations/096-inline-module-and-pub.rs`):

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

Compile transcript:

```
$ rustc demo.rs
(no output; exit 0)
$ ls
demo  demo.rs
$ ./demo
hi from foo
(exit 0)
```

Witnesses:

- **Inline module declaration** is accepted: `mod foo { ... }` at the
  crate root is a valid item. `rustc` is silent on success, consistent
  with lesson 001.
- **`pub fn` inside the module body** is accepted: `pub fn hi() { ... }`
  exposes `hi` for external access via `foo::item`.
- **Cross-boundary call** is accepted: `foo::hi()` from `fn main`
  reaches across the module boundary, runs the function body, and
  returns. The output line `hi from foo` is exactly the function body's
  `println!` (lesson 011).
- All pieces compile under one `rustc` invocation. There is no separate
  module-resolution step; `mod foo { ... }` is parsed and type-checked
  alongside the rest of the file.

### Probe 2 — contrast: `pub` removed (E0603)

Source (`/tmp/.../broken.rs`):

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

Compile transcript:

```
$ rustc broken.rs
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

For more information about this error, try `rustc --explain E0603`.
(exit 1)
```

Witnesses:

- *`error:` headline; build aborts.* `error[E0603]` is the headline
  E-code for "private item used outside its scope." Lesson 003's
  diagnostic-map vocabulary reads this byte-for-byte. The trailer
  `error: aborting due to 1 previous error` is lesson 069's contrast
  against `warning:`.
- *Caret at the use site, not the definition.* The `-->` location is
  `broken.rs:8:10` — column 10 of line 8, which is the `hi` token after
  `foo::`. This is load-bearing: the diagnostic fires *where the code
  tried to reach in*, not where the function was declared.
- *`note:` block points at the definition.* The second `-->` block
  (lines `2:5` underlining `fn hi()`) is rustc's standard "this is
  where the named item was declared" cross-reference (lesson 003's
  diagnostic-map admits this; lesson 003 said diagnostics may carry
  more than one `-->` line and that "the first one is your bug
  location"). Today the bug is at the use site.
- *Build does not produce an executable.* The contrast probe leaves
  the working probe's `demo` binary alone but does not produce a new
  `broken` binary.
- *The lesson's centered rule is exactly what the diagnostic states.*
  Without `pub`, the function exists inside `mod foo`, the path
  `foo::hi` resolves to it, but the privacy check fails at compile
  time. Adding `pub` to `fn hi` (restoring Probe 1's source) compiles
  silently and prints the same line.

The corpus E0603 page (cited above) shows the same shape with `const`
in place of `fn`. Today's `fn` form is the most economical because
`const` was last touched at lesson 075 and `fn` is the audience's
day-one item kind.

### Probe 3 — auxiliary: bare call site (E0425), grounding the namespace claim

Source (`/tmp/.../bare.rs`):

```rust
mod foo {
    pub fn hi() {
        println!("hi from foo");
    }
}

fn main() {
    hi();
}
```

Compile transcript:

```
$ rustc bare.rs
error[E0425]: cannot find function `hi` in this scope
 --> bare.rs:8:5
  |
8 |     hi();
  |     ^^ not found in this scope
  |
help: consider importing this function
  |
1 + use foo::hi;
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
(exit 1)
```

Witnesses:

- The framing "items inside `mod foo { ... }` live in a *new namespace
  called* `foo`" is grounded by an empirical contrast: bare `hi()` from
  outside the module fires E0425 — the same "cannot find … in this
  scope" family lessons 005, 008, 040, 042, 043, 044 already
  exercised. The function exists, but it lives in `foo`'s namespace,
  not the crate root's.
- The `help:` block proposes `use foo::hi;`, which is *exactly* the
  lesson-044 mechanism applied to the user's own module. The lesson
  body's *Prerequisites* and *What To Ignore* sections reference this
  observation explicitly: today does not install `use` again; lesson 044
  already did.
- This probe is *not* the centered teaching contrast (Probe 2 is). It
  is the "module = namespace" framing's empirical witness, kept in the
  appendix to give the red-team something to verify rather than
  treating "namespace" as an unverified vocabulary import from the Book.

### Probe 4 — calibration: privacy-by-default at the crate root is invisible

Source (`/tmp/.../nomod.rs`):

```rust
fn hi() {
    println!("hi");
}

fn main() {
    hi();
}
```

Compile transcript:

```
$ rustc nomod.rs
(no output; exit 0)
$ ./nomod
hi
(exit 0)
```

And with `pub fn hi()` instead of `fn hi()`:

```
$ rustc nomodpub.rs
(no output; exit 0)
$ ./nomodpub
hi
(exit 0)
```

Witnesses:

- Both programs compile silently and produce the same output. In a
  single-binary `rustc` build with no other crates, there is no
  observable difference between `pub fn hi()` and `fn hi()` at the
  crate root. The Reference's privacy default *does* apply — `fn hi`
  is private at the crate root — but with no second crate to look in,
  the rule has no observable consequence.
- This is the empirical justification for the lesson's *What To Ignore
  For Now* bullet on "privacy-by-default at the crate root is also
  in effect, but in a single-binary `rustc` build with no other crates
  you cannot tell." It is also the empirical justification for the
  brief's framing that today's move *requires* a `mod` block: a
  module boundary is the smallest scaffold that makes the rule
  observable in this run's compile-and-run setup.

This probe is calibration material, not a contrast for the centered
move. It is captured here so the red-team can verify that the brief's
framing "single-binary crate-root visibility has no observable
difference" is empirically true on this host, not just inferred from
the Reference.

## Prerequisite-claim summary

Direct prerequisites — each prerequisite's load-bearing claim used by
this lesson, summarized in 1-3 bullets per the run README:

- **Lesson 002 — `fn main`** (cited). `fn main()` is the entry-point
  function whose body runs when the executable launches. The probe
  uses one `fn main` block, unchanged.

- **Lesson 008 — define and call a function** (cited, supporting). The
  definition shape `fn name() { ... }` and the call form `name(args)`.
  Today the function definition shape is unchanged; the call site
  reaches across a module boundary via `foo::hi()` instead of bare
  `hi()`. The auxiliary E0425 probe witnesses what bare `hi()` does
  when the function lives inside a module.

- **Lesson 011 — `println!` positional `{}`** (cited). One `{}`
  consumed by no extra arguments — the body of `hi` is just
  `println!("hi from foo");`, the simplest possible body.

- **Lesson 043 — nested module paths** (load-bearing). The full-path
  call form `module::name(args)` reaches a function in a module.
  Lesson 043's path was `std::cmp::min`; today's path is `foo::hi`.
  Same shape, one segment, *your* module on the left of `::`. Lesson
  043's *What To Ignore* explicitly named "Defining your own
  modules — `mod`, inline `{ ... }` modules, and modules in separate
  files" and "Visibility and `pub`" as deferred future moves. Today
  is the first half of those (inline `mod` + bare `pub`); file-based
  modules and restricted visibility stay deferred.

- **Lesson 044 — `use std::cmp::min;`** (cited only). The shorthand
  `use Path::final;` that brings the final segment of a path into
  scope. Today does not install or re-install `use`; the contrast
  probe's `help:` block merely proposes `use foo::hi;` as one
  conceivable fix, and the lesson body says this is lesson 044's
  mechanism applied to the user's own module rather than a new move.

- **Lesson 003 — rustc diagnostic four-part map** (cited). Used to
  read Probe 2's E0603 transcript: headline `error[E0603]: function
  \`hi\` is private`, location `broken.rs:8:10`, source excerpt with
  caret under `hi`, and a follow-up `note:` block with a second `-->`
  pointing at the definition.

Older supporting lessons (cited only, no specific claim load-bearing):

- Lesson 001 (`rustc file.rs`; silent on success; produced
  executable). Used by the probe transcripts.
- Lesson 069 (`warning:` vs `error:` category). Probe 2's trailer is
  `error: aborting due to 1 previous error`, not
  `warning: 1 warning emitted` — categorization carries through
  unchanged.
- Lesson 095 (struct with named fields). Used in the lesson's
  *What To Ignore For Now* bullet "today only puts `pub` on the
  function, not on a field" — `pub`-on-struct-fields composes today's
  move with lesson 095 and is deferred.

## Contrast-probe coverage

The lesson's contrastive claim is "with `pub` it works; without `pub`
the same code at the same call site fails to compile." This is
witnessed empirically by Probe 2 (the E0603 contrast).

A second, supporting framing — "the inline module is a namespace; the
function only exists at `foo::hi`, not at bare `hi`" — is witnessed by
Probe 3 (the E0425 bare-call probe). The lesson body does not exercise
this contrast directly; it is appendix-only because (a) the centered
claim is `pub`'s effect, not the path-shape, and (b) lesson 043 already
installed the path-shape rule. Probe 3's role is to give the red-team
a concrete grounding for "namespace" beyond a vocabulary import from
the Book.

A third probe (Probe 4) is calibration: it confirms empirically that
in a single-binary `rustc` build with no other crates, `fn hi()` and
`pub fn hi()` at the crate root behave identically. This is the
justification for *requiring* a `mod` block today rather than treating
`pub` as standalone-atomic.

## Notes on deferred items

The lesson defers (and this appendix does not probe further):

- *File-based modules* — `mod foo;` (no body, semicolon-terminated)
  resolves to `foo.rs` or `foo/mod.rs`. Reference `items/modules.md`
  lines 70-89 cover the resolution rule. Same `mod` keyword, same
  privacy rule, body in a separate file. Lesson 097's territory.
- *Restricted visibility* — `pub(super)`, `pub(crate)`, `pub(in path)`,
  `pub(self)`. Reference `visibility-and-privacy.md` lines 125-204
  cover the four shapes. The rmp target uses `pub(super)` heavily on
  struct fields (`biguint/basic.rs:2-5`).
- *`pub mod foo { ... }`* — making the module declaration itself
  public. Today's `mod foo { ... }` is private at the crate root;
  the rmp target's `lib.rs` makes its top-level modules public with
  `pub mod bigint;` and `pub mod biguint;`.
- *`pub` on struct fields* — Reference `items/structs.md` line 22
  names `Visibility?` as an optional grammar position before the
  field name. Today only puts `pub` on the function-item position.
- *`pub` on enum variants* — enums are not yet installed.
- *`pub use` re-exports* — Reference `visibility-and-privacy.md`
  lines 209-233 cover this. Composes today's `pub` with lesson 044's
  `use`. The rmp target's `biguint/mod.rs` line 3 uses
  `pub use basic::BigUInt;`.
- *`pub trait` and `pub impl` blocks* — trait machinery is blocked.
- *`crate::` as a path root* — the lesson uses only the simplest path
  form, `foo::hi`. The `super::` and `self::` prefixes used inside
  modules are also deferred.
- *Nested submodules* — `mod foo { mod bar { ... } }`. Reference
  `items/modules.md` line 25 says "Modules can nest arbitrarily."
- *The `Visibility?` grammar slot on every item kind* — today only
  exercises it on a function. Reference items grammar lists this slot
  on `fn`, `struct`, `enum`, `union`, `const`, `static`, `mod`,
  `use`, `trait`, `impl`-associated-items, etc.

None of these are load-bearing for the centered claim "declare an
inline submodule with `mod foo { ... }`, expose one item with `pub`,
reach the item from outside via `foo::item`."
