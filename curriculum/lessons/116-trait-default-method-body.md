---
id: 116-trait-default-method-body
status: accepted
evidence: ../evidence/116-trait-default-method-body.md
---

# Give a trait method a default body in `{ ... }`; let the impl accept it with empty `{}`

## The Move

Lessons 111-115 wrote every trait method declaration with a
semicolon body slot: `fn doubled(&self) -> u32;`. The trailing `;`
meant *the impl must provide the body*. Today the body slot can
take a *second* shape — curly braces with a body inside the trait
itself, called a *default* body. An impl may then either leave the
method out entirely (and inherit the default) or write its own
override:

```rust
struct Counter {
    count: u32,
}

trait Greeting {
    fn greet(&self) -> u32 {
        100u32
    }
}

impl Greeting for Counter {}

fn main() {
    let c = Counter { count: 7 };
    println!("c.count = {}", c.count);
    println!("greet   = {}", c.greet());
}
```

`./demo` prints `c.count = 7` then `greet   = 100`. Two new
mechanics in one program:

1. **`fn greet(&self) -> u32 { 100u32 }` inside the trait** — the
   body slot is `{ 100u32 }` instead of `;`. The Reference:
   "If the trait function defines a body, this definition acts as
   a default for any implementation which does not override it."

2. **`impl Greeting for Counter {}`** — the impl block is empty.
   No `fn greet` line appears anywhere in the impl body. Because
   the trait gave `greet` a default, the empty impl is *complete*
   — the empty curlies say "use every default the trait provided."
   The Book frames it directly: "we specify an empty `impl` block
   with `impl Summary for NewsArticle {}`."

The second half of the mechanic is *override*. Add a second struct
and a second impl that supplies its own body for `greet`:

```rust
struct Tally { n: u32 }

impl Greeting for Tally {
    fn greet(&self) -> u32 { self.n * 2 }
}
```

With the working probe's trait declaration unchanged, `c.greet()`
still returns `100` (the default body), while a `Tally { n: 21 }`
returns `42` (`21 * 2`, the override body). The Book: "the syntax
for overriding a default implementation is the same as the syntax
for implementing a trait method that doesn't have a default
implementation." The override `fn` line is identical in shape to
every impl method in lessons 111-115.

This is also exactly why `impl Eq for BigUInt {}` from rmp's
`src/biguint/cmp.rs:10` compiles. `Eq` is std's "trait without
methods" (structurally, an empty impl body), so the empty `{}`
impl is complete. Reading that line *structurally* — empty impl
accepts the default — is the unlock today provides. The
supertrait constraint `pub trait Eq: PartialEq { }` is deferred.

## Mental Model Delta

- *Before*: "A trait method's body slot is always `;`. The impl
  must always provide the body. The impl block always has at least
  one `fn` line per trait method."
- *After*: "A trait method's body slot is either `;` or
  `{ ... }`. The `;` form means the impl must provide the body.
  The `{ ... }` form provides a *default* body the impl may either
  accept by leaving the method out entirely (`impl Trait for Type
  {}`), or override by writing its own `fn` line. An empty `{}`
  impl is legal exactly when every method the trait declared has
  a default body — which is why `impl Eq for BigUInt {}` works."

## Prerequisites

- Installed concepts:
  - **Lesson 111** (load-bearing): `trait Name { fn method(&self)
    -> T; }` and `impl Trait for Type { ... }`. Today extends 111
    by allowing the body slot to be `{ ... }` instead of `;` and
    the impl body to be empty `{}`.
  - **Lesson 008** (load-bearing): `fn name(&self) -> T { ... }`
    — the function-with-body shape. Today's default body uses the
    same shape, written inside the trait declaration.
  - **Lesson 095** (load-bearing): `struct Counter { count: u32 }`
    and field access `c.count`.
  - **Lesson 100** (load-bearing): `&self` as the receiver inside
    an impl block.
  - **Lesson 040** (load-bearing): the dot-call shape `c.greet()`.
  - **Lesson 115** (cited): E0046 with the multi-`-->` shape
    pointing at trait declaration as the contract; reused on a
    different missing item kind in today's sharpening probe.
  - **Lessons 002, 005, 009 (`*`), 011, 019, 080, 003, 001** (cited):
    unchanged from prior usage. The evidence appendix's *Older
    supporting lessons* section carries the topical detail.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
c.count = 7
greet   = 100
```

The trait body has one new shape vs. lesson 111: curly braces
in the body slot instead of `;`. The impl body has one new shape
vs. all of 111-115: empty `{}` with no `fn` lines.

*Now the contrast.* Save `no_default.rs` — same source, but change
the trait method's body slot back to `;`:

```rust
trait Greeting {
    fn greet(&self) -> u32;
}

impl Greeting for Counter {}
```

Compile:

```
error[E0046]: not all trait items implemented, missing: `greet`
 --> no_default.rs:9:1
  |
6 |     fn greet(&self) -> u32;
  |     ----------------------- `greet` from trait
...
9 | impl Greeting for Counter {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^ missing `greet` in implementation
```

Read with the lesson 003 map. Headline: E0046 — the same code
lesson 115 used for a missing associated type. The
multi-`-->` shape now names the missing item as the trait
*method* `greet`. Without the default body, `;` requires the impl
to provide the body; the empty impl provides nothing; E0046 fires.
With the default body, the same empty impl is legal. The default
body is what makes empty `{}` work.

## What Changed

- `fn method(&self) -> T { ... }` *inside* a trait declaration is
  legal. The curly-brace body is a *default* for any impl that
  does not override it.
- `impl Trait for Type {}` (empty body) accepts every default the
  trait provided and is the canonical "marker-trait-style" impl.
- The override syntax is the ordinary `fn` line; there is no
  separate keyword.
- Without the default body, the same empty impl fires E0046 with
  `missing: <method-name>` and points back at the trait
  declaration as the contract.
- `impl Eq for BigUInt {}` in rmp's `cmp.rs:10` is an instance of
  this shape — the empty impl accepts every default `Eq` provides.

## Check Yourself

You write `tiny.rs`:

```rust
struct Tally { n: u32 }

trait Loud {
    fn shout(&self) -> u32 { 999u32 }
}

impl Loud for Tally {}

fn main() {
    let t = Tally { n: 5 };
    println!("n      = {}", t.n);
    println!("shout  = {}", t.shout());
}
```

(a) Does `rustc tiny.rs` accept the program (no errors, no
warnings)?

(b) What two lines does `./tiny` print?

(c) If you change *only* the trait body so that `fn shout` ends in
`;` instead of `{ 999u32 }` — leaving the impl `{}` empty — what
E-code appears, and what identifier does the headline name as the
missing item?

*(Answers: (a) Yes. (b) `n      = 5` then `shout  = 999`. (c)
E0046; the headline reads `not all trait items implemented,
missing: `shout``.)*

## What To Ignore For Now

Today installs default *method* bodies, accepted by an empty `{}`
impl or overridden by a normal `fn` line. Deferred:

- **Default associated types** — `type Output = u32;` *inside the
  trait body*. Feature-gated; deferred from 115.
- **Multi-type dispatch** as a centered teaching — today's
  override demonstration uses a second struct (`Tally`) to contrast
  against the default-accepting `Counter`, which incidentally
  exhibits multi-type dispatch. The centered teaching is the
  default body machinery, not dispatch; the canonical
  multi-type-dispatch lesson remains a separate future move.
- **Supertraits** — `trait Eq: PartialEq` is the actual std
  declaration. The colon-after-trait-name shape is a separate
  mechanic; today reads `impl Eq for BigUInt {}` only structurally.
- **Marker traits as a category** — `Send`, `Sync`, `Copy`,
  `Sized`, `Eq`. Named in passing as "traits with no methods or
  only default-bodied methods"; the formal category is deferred.
- **Internal `Eq` details** — `assert_receiver_is_total_eq` and
  the deeper rationale for `Eq` requiring `PartialEq`.
- **`#[derive(Eq)]`, `#[derive(Debug)]`, etc.** — deferred.
- **Default bodies that call other trait methods** — the Book's
  `summarize`/`summarize_author` example; composes today's shape
  with itself.
- **All deferrals from 111-115** remain deferred (generics, trait
  bounds, lifetimes, the orphan rule, operator traits, etc.).

## Evidence

See `../evidence/116-trait-default-method-body.md`.
