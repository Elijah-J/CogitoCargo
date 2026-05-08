# Evidence — 111-trait-decl-and-impl-for-type

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version --verbose`:

  ```
  rustc 1.95.0 (59807616e 2026-04-14)
  binary: rustc
  commit-hash: 59807616e1fa2540724bfbac14d7976d7e4a3860
  commit-date: 2026-04-14
  host: x86_64-apple-darwin
  release: 1.95.0
  LLVM version: 22.1.2
  ```

- `uname -a`:

  ```
  Darwin MacBookPro.lan 24.5.0 Darwin Kernel Version 24.5.0: Tue Apr 22 19:53:26 PDT 2025; root:xnu-11417.121.6~2/RELEASE_X86_64 x86_64
  ```

- Probes run in `/tmp/eduratchet-111/` on this host. Same toolchain
  family as recently accepted lessons (104-110).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/111-trait-decl-and-impl-for-type.rs`
is the working three-piece probe verbatim, with header comments
naming the centered E0599 contrast and the auxiliary wrong-type
probe captured below.

## Sources

### `output/docs/rust/book/ch10-02-traits.md`

The Book's *Defining Shared Behavior with Traits* chapter — the
canonical lesson-anchor source for today's move. Two load-bearing
passages.

#### Lines 14-19 — what a trait is

> A type's behavior consists of the methods we can call on that
> type. Different types share the same behavior if we can call the
> same methods on all of those types. Trait definitions are a way
> to group method signatures together to define a set of behaviors
> necessary to accomplish some purpose.

Corpus warrant for the *trait declaration as a group of method
signatures* framing the lesson uses in piece (1) of *The Move*.

#### Lines 36-56 — Listing 10-12: trait-with-one-signature, semicolon-instead-of-body

> ```rust
> pub trait Summary {
>     fn summarize(&self) -> String;
> }
> ```
>
> *Listing 10-12: A `Summary` trait that consists of the behavior
> provided by a `summarize` method*
>
> Here, we declare a trait using the `trait` keyword and then the
> trait's name, which is `Summary` in this case. We also declare
> the trait as `pub` so that crates depending on this crate can
> make use of this trait too, as we'll see in a few examples.
> Inside the curly brackets, we declare the method signatures that
> describe the behaviors of the types that implement this trait,
> which in this case is `fn summarize(&self) -> String`.
>
> After the method signature, instead of providing an
> implementation within curly brackets, we use a semicolon. Each
> type implementing this trait must provide its own custom
> behavior for the body of the method. The compiler will enforce
> that any type that has the `Summary` trait will have the method
> `summarize` defined with this signature exactly.
>
> A trait can have multiple methods in its body: The method
> signatures are listed one per line, and each line ends in a
> semicolon.

Corpus warrant for piece (1) of *The Move* — the trait-declaration
shape `trait Name { fn method(&self) -> T; }`, the
semicolon-replaces-body rule, and the matching-signature rule. The
lesson body quotes the second sentence verbatim. Lesson 111's
trait `Doubled { fn doubled(&self) -> u32; }` is the same shape with
a different method, return type, and no `pub` (deferred today).

#### Lines 60-112 — Listing 10-13: implementing the trait on a type

> ### Implementing a Trait on a Type
>
> Now that we've defined the desired signatures of the `Summary`
> trait's methods, we can implement it on the types in our media
> aggregator. Listing 10-13 shows an implementation of the
> `Summary` trait on the `NewsArticle` struct that uses the
> headline, the author, and the location to create the return
> value of `summarize`. ...
>
> ```rust
> impl Summary for NewsArticle {
>     fn summarize(&self) -> String {
>         format!("{}, by {} ({})", self.headline, self.author, self.location)
>     }
> }
> ```
>
> *Listing 10-13: Implementing the `Summary` trait on the
> `NewsArticle` and `SocialPost` types*
>
> Implementing a trait on a type is similar to implementing
> regular methods. The difference is that after `impl`, we put
> the trait name we want to implement, then use the `for`
> keyword, and then specify the name of the type we want to
> implement the trait for. Within the `impl` block, we put the
> method signatures that the trait definition has defined.
> Instead of adding a semicolon after each signature, we use
> curly brackets and fill in the method body with the specific
> behavior that we want the methods of the trait to have for the
> particular type.

Corpus warrant for piece (2) of *The Move* — the
`impl Trait for Type { ... }` header shape, the `for` keyword
between trait name and type, and the rule that the impl provides
the bodies the trait left as `;`. The lesson quotes the line
"after `impl`, we put the trait name we want to implement, then
use the `for` keyword, and then specify the name of the type we
want to implement the trait for" verbatim.

### `output/docs/rust/reference/items/traits.md`

The Reference's *Traits* item. Three load-bearing passages.

#### Lines 10-15 — the `Trait` grammar

> Trait →
>     unsafe? trait IDENTIFIER GenericParams? ( : TypeParamBounds? )? WhereClause?
>     {
>         InnerAttribute*
>         AssociatedItem*
>     }

Corpus warrant for the formal shape of a trait declaration. Today's
trait `Doubled { fn doubled(&self) -> u32; }` instantiates this
grammar with no `unsafe`, no `GenericParams`, no `TypeParamBounds`,
no `WhereClause`, no inner attributes, and one `AssociatedItem`
(the `fn doubled` signature).

#### Lines 19-25 — what a trait is

> A *trait* describes an abstract interface that types can
> implement. This interface consists of associated items, which
> come in three varieties:
>
> - functions
> - types
> - constants

Corpus warrant for the *abstract interface* phrasing in *What
Changed* and the named-but-deferred associated-types and
associated-constants entries in *What To Ignore For Now*.

#### Line 43 — semicolon vs body in a trait function

> Trait functions may omit the function body by replacing it with
> a semicolon. This indicates that the implementation must define
> the function. If the trait function defines a body, this
> definition acts as a default for any implementation which does
> not override it. Similarly, associated constants may omit the
> equals sign and expression to indicate implementations must
> define the constant value. Associated types must never define
> the type, the type may only be specified in an implementation.

Corpus warrant for piece (1) of *The Move* — the
semicolon-replaces-body rule. The lesson quotes the first two
sentences verbatim. The third sentence is the deferred *default
method body* ignore-bullet.

### `output/docs/rust/reference/items/implementations.md`

The Reference's *Implementations* item. Two load-bearing passages.

#### Lines 10-24 — the `Implementation` grammar splitting `InherentImpl` from `TraitImpl`

> Implementation → InherentImpl | TraitImpl
>
> InherentImpl →
>     impl GenericParams? Type WhereClause? {
>         InnerAttribute*
>         AssociatedItem*
>     }
>
> TraitImpl →
>     unsafe? impl GenericParams? !? TypePath for Type
>     WhereClause?
>     {
>         InnerAttribute*
>         AssociatedItem*
>     }

Corpus warrant for *Mental Model Delta* and *What Changed* — that
`Implementation` is exactly two distinct grammar rules. Lesson
100 instantiated `InherentImpl`; today instantiates `TraitImpl`
with no `unsafe`, no `GenericParams`, no negative-impl `!`, no
`WhereClause`, the `TypePath` `Doubled`, the `Type` `Counter`,
no inner attributes, one `AssociatedItem` (the `fn doubled` body).

#### Lines 30-35 — two kinds of implementation

> There are two types of implementations:
>
> - inherent implementations
> - trait implementations

Corpus warrant for the *Mental Model Delta*'s "two kinds of `impl`
block" framing. Plus lines 107-115:

> A *trait implementation* is defined like an inherent
> implementation except that the optional generic type
> declarations are followed by a trait, followed by the keyword
> `for`, followed by a path to a nominal type. ...
>
> The trait is known as the *implemented trait*. The implementing
> type implements the implemented trait.

Corpus warrant for the lesson's exact reading of the header — the
trait name comes between `impl` and `for`, the type comes after
`for`.

### `output/docs/rust/error_codes/E0599.md`

Already cited in lessons 100, 108. Page text:

> This error occurs when a method is used on a type which doesn't
> implement it.

Today's contrast probe fires E0599 with the trait-specific
diagnostic shape (the `= help:` line on traits and the
`note: \`Trait\` defines an item` block) — the same E-code lesson
100 installed with the inherent-method-not-found shape, now
witnessed in the trait-not-implemented shape. The diagnostic
itself is captured below in *Probes*; the page only confirms the
E-code's general meaning.

### `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`

The unlock-target file. First ten lines verbatim:

```rust
use super::basic::BigUInt;
use std::cmp::{self, Ord, Ordering};

impl PartialEq<BigUInt> for BigUInt {
    fn eq(&self, other: &BigUInt) -> bool {
        self.limbs == other.limbs
    }
}

impl Eq for BigUInt {}
```

Two trait-impl shapes here:

- `impl Eq for BigUInt {}` (line 10) is *exactly* today's
  `impl Trait for Type { ... }` header shape with an empty body.
  `Eq` declares no methods that lack defaults (it inherits from
  `PartialEq` and adds only the marker semantics), so the body is
  empty. Today's lesson names this in *What To Ignore For Now* as
  the immediate readability unlock.
- `impl PartialEq<BigUInt> for BigUInt { fn eq(&self, other: &BigUInt) -> bool { ... } }`
  (lines 4-8) is partially readable from today's installed
  concepts — the `impl Trait for Type { fn method(&self, other: &T) -> bool { body } }`
  outer shape is today's, modulo the generic trait parameter
  `<BigUInt>` inside the `Trait`-position TypePath, which is its
  own deferred mechanic. The lesson names this in *Mental Model
  Delta* and the graph entry's `unlocks:` list.

The capstone target on the run's roadmap is
`impl Add<&BigUInt> for &BigUInt { type Output = BigUInt; fn add(self, rhs: &BigUInt) -> BigUInt { ... } }`
in `src/biguint/add.rs`, which adds generic trait parameters,
associated types, and a non-`&self` receiver on top of today's
shape.

## Probes

All probes run in `/tmp/eduratchet-111/`.

### Probe 1: working three-piece probe

Source — same content as the committed observation file at
`observations/111-trait-decl-and-impl-for-type.rs`:

```rust
struct Counter {
    count: u32,
}

trait Doubled {
    fn doubled(&self) -> u32;
}

impl Doubled for Counter {
    fn doubled(&self) -> u32 {
        self.count * 2
    }
}

fn main() {
    let c = Counter { count: 21 };
    println!("doubled = {}", c.doubled());
}
```

Commands and transcripts:

```console
$ /Users/eli/.cargo/bin/rustc demo.rs ; echo EXIT $?
EXIT 0
$ ./demo ; echo EXIT $?
doubled = 42
EXIT 0
```

Both compile and run silent. Witness for the working case in *The
Move*.

### Probe 2: centered E0599 contrast — drop the impl block

Source — same as Probe 1 with the entire `impl Doubled for Counter { ... }`
block deleted:

```rust
struct Counter {
    count: u32,
}

trait Doubled {
    fn doubled(&self) -> u32;
}

fn main() {
    let c = Counter { count: 21 };
    println!("doubled = {}", c.doubled());
}
```

Command and transcript:

```console
$ /Users/eli/.cargo/bin/rustc no_impl.rs ; echo EXIT $?
error[E0599]: no method named `doubled` found for struct `Counter` in the current scope
  --> no_impl.rs:11:32
   |
 1 | struct Counter {
   | -------------- method `doubled` not found for this struct
...
11 |     println!("doubled = {}", c.doubled());
   |                                ^^^^^^^ method not found in `Counter`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Doubled` defines an item `doubled`, perhaps you need to implement it
  --> no_impl.rs:5:1
   |
 5 | trait Doubled {
   | ^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
EXIT 1
```

Witness for *Try It* and *What Changed*. Read with the lesson 003
four-part diagnostic map: headline `error[E0599]:`, location
`-->` at the call site `no_impl.rs:11:32`, source excerpt with
caret under `doubled`, and the `= help:` plus `note:` lines
verbatim quoted in the lesson body. The diagnostic's `= help:`
phrase *items from traits can only be used if the trait is
implemented and in scope* names today's rule from rustc's mouth.
The follow-on `note: \`Doubled\` defines an item \`doubled\`,
perhaps you need to implement it` and its second `-->` line
pointing at the trait declaration are the same multi-`-->`
diagnostic shape lesson 096 installed for E0603.

### Probe 3: auxiliary E0599 — wrong type that doesn't implement the trait

Source — Probe 1's struct/trait/impl unchanged, plus a *second*
struct `Other` and `fn main` rewritten to call `o.doubled()` on an
instance of `Other` that does *not* implement `Doubled`:

```rust
struct Counter {
    count: u32,
}

struct Other {
    n: u32,
}

trait Doubled {
    fn doubled(&self) -> u32;
}

impl Doubled for Counter {
    fn doubled(&self) -> u32 {
        self.count * 2
    }
}

fn main() {
    let o = Other { n: 7 };
    println!("doubled = {}", o.doubled());
}
```

Command and transcript:

```console
$ /Users/eli/.cargo/bin/rustc wrong_type.rs ; echo EXIT $?
error[E0599]: no method named `doubled` found for struct `Other` in the current scope
  --> wrong_type.rs:21:32
   |
 5 | struct Other {
   | ------------ method `doubled` not found for this struct
...
21 |     println!("doubled = {}", o.doubled());
   |                                ^^^^^^^ method not found in `Other`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Doubled` defines an item `doubled`, perhaps you need to implement it
  --> wrong_type.rs:9:1
   |
 9 | trait Doubled {
   | ^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
EXIT 1
```

Empirical witness that an `impl Trait for Type` block attaches the
trait's methods *to that specific type only* — `Other` is in the
same crate as `Counter` and as the trait `Doubled`, but the impl
exists for `Counter` only, and the dot call on an `Other` value
fires the same E0599 with the same `= help:` phrase. This sharpens
the *Mental Model Delta* claim that trait impls are
type-by-type — the impl is not a global registration. (The
diagnostic's `note:` block points the reader at the trait
declaration with a *perhaps you need to implement it* hint, which
is exactly what Probe 1 did for `Counter`.) Not centered in the
lesson body to keep the scope tight; named here as the
mental-model-sharpening probe.

## Claim-to-Evidence Mapping

| Lesson claim | Evidence |
|---|---|
| trait declaration body uses `;` instead of `{ ... }` for unimplemented methods | Reference `items/traits.md:43` verbatim, Book ch10-02 lines 51-55 verbatim |
| trait declaration on its own attaches no methods to any type | Reference `items/traits.md:19-25` ("describes an abstract interface that types can implement"), Book ch10-02 lines 14-19 |
| `impl Trait for Type { ... }` is a different grammar rule from `impl Type { ... }` | Reference `items/implementations.md:10-24` (`Implementation → InherentImpl | TraitImpl`) |
| header reading: `Trait for Type` vs `Type` between `impl` and `{` | Reference `items/implementations.md:107-115`, Book ch10-02 lines 106-112 |
| signature in impl must match signature in trait | Book ch10-02 lines 51-55 ("the method `summarize` defined with this signature exactly"), 109-112 ("Within the `impl` block, we put the method signatures that the trait definition has defined") |
| dot call shape `value.method()` is unchanged from lesson 040 | Lesson 040 (cited prerequisite) + Probe 1 transcript |
| without the impl block, the dot call fires E0599 | Probe 2 transcript |
| E0599's `= help:` line names today's rule verbatim | Probe 2 transcript line `= help: items from traits ...` |
| E0599's `note:` block points at the trait declaration | Probe 2 transcript lines `note: \`Doubled\` defines an item \`doubled\`, perhaps you need to implement it / --> no_impl.rs:5:1` |
| trait impls are type-by-type, not global | Probe 3 transcript (`Other` doesn't impl `Doubled`, same E0599 fires) |
| `impl Eq for BigUInt {}` in rmp `cmp.rs` is today's shape with empty body | rmp `src/biguint/cmp.rs:10` cited above |
| `impl Add<&BigUInt> for &BigUInt { ... }` is the run's capstone target | rmp `src/biguint/add.rs` cited in *What To Ignore For Now* |

## Direct-Prerequisite Summary

Three load-bearing prerequisites today, each summarized in the one
specific claim today carries:

- **Lesson 095** — declaration shape `struct Name { field: Type, }`
  and field access `value.field`. Today: `struct Counter { count: u32 }`
  is the trait's target type and `self.count` is field access in
  the body of `doubled`.
- **Lesson 100** — `impl Type { ... }` block, `&self` receiver,
  `&self`-shorthand-for-`self: &Self`, the rule that `&self` is
  what makes the dot call resolve. Today reuses *every* piece
  except the impl-block header, which gains `Doubled for` between
  `impl` and the type.
- **Lesson 040** — dot-call shape `value.method()`. Today's
  `c.doubled()` is exactly that shape. No new call syntax.
- **Lesson 008** — `fn name(p: T) -> R { ... }`. Today's trait
  signature `fn doubled(&self) -> u32;` is the same `fn` shape with
  the body replaced by `;`; today's impl method body
  `fn doubled(&self) -> u32 { self.count * 2 }` is the standard
  full form.

Older supporting lessons mentioned by number/title only: 002
(`fn main`), 005 (`let`), 009 (the `*` (multiplication) operator
used in the working probe's `self.count * 2`), 011 (`println!` with
`{}`), 012 (`bool` as the return type of `as_bool` in Check
Yourself's `tiny.rs`), 013 (`>` comparison in Check Yourself's
`self.n > 0`), 019 (the type-annotation slot), 080 (`u32` as integer
family member), 003 (the four-part diagnostic map for E0599), 001
(`rustc demo.rs && ./demo`).

## Negative / Contrastive Probes

Today's lesson makes the centered contrastive claim "with the impl
block the dot call works; without it, the dot call fails with
E0599." Probe 2 is the negative case witnessing the *fails with
E0599* half. Probe 3 sharpens it to "the impl is type-by-type,"
witnessed by E0599 firing on `Other` even though `Doubled` and
`Counter` are in the same crate.

No further negative probes are needed; the contrast is fully
exhausted by Probe 2 (drop the impl) and Probe 3 (call on a
non-implementing type).
