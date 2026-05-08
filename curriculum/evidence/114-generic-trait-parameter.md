# Evidence — 114-generic-trait-parameter

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

- `which rustc`: `/Users/eli/.cargo/bin/rustc`.
- Probes run in `/tmp/eduratchet-114/`. Same toolchain family as
  recently accepted lessons (107-113).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/114-generic-trait-parameter.rs`
is the working probe verbatim with header comments naming the
centered E0107 contrast captured below.

## Sources

### `output/docs/rust/reference/items/traits.md`

The Reference's chapter on traits.

#### Lines 73-84 — generic trait declaration

> Type parameters can be specified for a trait to make it generic.
> These appear after the trait name, using the same syntax used in
> [generic functions](functions.md#generic-functions).
>
> ```rust
> trait Seq<T> {
>     fn len(&self) -> u32;
>     fn elt_at(&self, n: u32) -> T;
>     fn iter<F>(&self, f: F) where F: Fn(T);
> }
> ```

Direct corpus warrant for the lesson's centered move. The Reference
quote names every load-bearing mechanic of today's lesson:

1. *"Type parameters can be specified for a trait to make it
   generic"* — the trait may carry type parameters.
2. *"These appear after the trait name"* — the angle-bracket slot
   sits between the trait identifier and the body. Today's
   `trait AddRhs<RHS> { ... }` instantiates this position with one
   type parameter `RHS`.
3. The `Seq<T>` example shows `T` used as a type inside the trait
   body — `fn elt_at(&self, n: u32) -> T;` — exactly the pattern
   today's `fn add(&self, rhs: RHS) -> u32;` follows with `RHS` in
   the parameter slot rather than the return slot.

The lesson defers the `where F: Fn(T)` form (trait bounds and
higher-ranked trait shapes), the inner generic on the method
`fn iter<F>` (generic functions / methods), and the appearance of
the trait parameter in non-parameter slots beyond what today's
probe needs.

### `output/docs/rust/reference/items/generics.md`

The Reference's chapter on generic parameters.

#### Lines 8-12, 16, 24 — the generic-params grammar

> [GenericParams] → < ( [GenericParam] ( , [GenericParam] )* ,? )? >
>
> [GenericParam] → [OuterAttribute]* ( [LifetimeParam] |
> [TypeParam] | [ConstParam] )
>
> [TypeParam] → [IDENTIFIER] ( : [TypeParamBounds]? )? ( = [Type] )?
>
> [...]
>
> Functions, type aliases, structs, enumerations, unions, traits,
> and implementations may be parameterized by types, constants, and
> lifetimes. These parameters are listed in angle brackets (`<...>`),
> usually immediately after the name of the item and before its
> definition. For implementations, which don't have a name, they
> come directly after `impl`.

Direct corpus warrant for the angle-bracket position on the trait
declaration and on the impl. Today's `<RHS>` is the simplest
instance of `< ( GenericParam )? >` — one `TypeParam` whose
`IDENTIFIER` is `RHS` with no `TypeParamBounds` and no `= Type`
default. The deferred shapes (`: TypeParamBounds` and `= Type`)
are the trait-bounds and default-type-parameter mechanics today
defers.

The line "the same parameter name may not be declared more than
once" is unused today; one parameter only.

#### Lines 41-52 — concrete examples including `trait A<U>`

> Some examples of items with type, const, and lifetime parameters:
>
> ```rust
> fn foo<'a, T>() {}
> trait A<U> {}
> struct Ref<'a, T> where T: 'a { r: &'a T }
> struct InnerArray<T, const N: usize>([T; N]);
> struct EitherOrderWorks<const N: bool, U>(U);
> }
> ```

Direct corpus warrant for `trait A<U> {}` — a generic trait
declaration with one type parameter — as a shape the language
recognizes. Today's `trait AddRhs<RHS> { ... }` is the same shape
with a body.

### `output/docs/rust/reference/items/implementations.md`

The Reference's chapter on `impl` blocks.

#### Lines 18-24 — `TraitImpl` grammar

> [TraitImpl] →
>     unsafe? impl [GenericParams]? !? [TypePath] for [Type]
>     [WhereClause]?
>     {
>         [InnerAttribute]*
>         [AssociatedItem]*
>     }

Corpus warrant for the impl header shape including
`GenericParams?` (the angle-bracket parameters on the impl itself,
deferred today) and `TypePath` after `impl ...?` (which includes
the `<ConcreteType>` after the trait name). Today's
`impl AddRhs<u32> for Counter` instantiates `[TypePath]` with
`AddRhs<u32>` — the trait identifier followed by one concrete type
argument. The `[GenericParams]?` slot after `impl` itself is empty
today; that slot is what generic struct impls use
(`impl<T> Foo<T> { ... }`) and is deferred wholesale.

### `output/docs/rust/book/ch10-00-generics.md`

The Book's chapter 10 introduction.

#### Lines 4-15 — generics defined; pre-existing uses

> Every programming language has tools for effectively handling the
> duplication of concepts. In Rust, one such tool is *generics*:
> abstract stand-ins for concrete types or other properties. We can
> express the behavior of generics or how they relate to other
> generics without knowing what will be in their place when
> compiling and running the code.
>
> Functions can take parameters of some generic type, instead of a
> concrete type like `i32` or `String`, in the same way they take
> parameters with unknown values to run the same code on multiple
> concrete values. In fact, we already used generics in Chapter 6
> with `Option<T>`, in Chapter 8 with `Vec<T>` and `HashMap<K, V>`,
> and in Chapter 9 with `Result<T, E>`. In this chapter, you'll
> explore how to define your own types, functions, and methods with
> generics!

Corpus warrant for the framing the lesson adopts: generics are
"abstract stand-ins for concrete types," and the learner has
already *used* `Vec<T>` (lesson 107) and `Option<T>` (the prelude)
without seeing the declaration side. Today is "how to define your
own". The lesson body's connection paragraph between the
angle-bracket *use* shape and the angle-bracket *declaration* shape
is grounded here.

#### Lines 32-41 — placeholders defined

> Generics allow us to replace specific types with a placeholder
> that represents multiple types to remove code duplication.

Direct corpus warrant for the lesson's word "placeholder" used to
describe `RHS` inside the trait body.

### `output/docs/rust/std/ops/trait.Add.md`

The std-library page for `Add`.

#### Lines 6-13 — `Add` is generic

> ```
> pub trait Add<Rhs = Self> {
>     type Output;
>
>     // Required method
>     fn add(self, rhs: Rhs) -> Self::Output;
> }
> ```

Corpus warrant for the unlock claim. `std::ops::Add` is exactly the
shape today installs *plus* (a) a default type parameter `Rhs = Self`
(deferred), (b) an associated type `type Output` (deferred), and
(c) a `self`-by-value receiver (lesson 102). Today's
`trait AddRhs<RHS> { fn add(&self, rhs: RHS) -> u32; }` is the same
shape with the deferred mechanics removed and a fixed primitive
return.

#### Lines 17-21 — Rhs may be a non-Self type

> Note that `Rhs` is `Self` by default, but this is not mandatory.
> For example, `std::time::SystemTime` implements `Add<Duration>`,
> which permits operations of the form
> `SystemTime = SystemTime + Duration`.

Corpus warrant for the lesson's claim that the same trait can be
impl'd for the same target type with different concrete RHS
substitutions. The std example `Add<Duration>` for `SystemTime` is
the canonical instance. Today's lesson does not exercise multi-impl
in the probe; this passage names the future move.

### `output/docs/rust/error_codes/E0107.md`

The error-code page for the centered contrast probe.

> An incorrect number of generic arguments was provided.
> [...]
> When using/declaring an item with generic arguments, you must
> provide the exact same number.

The corpus page's headline ("incorrect number of generic arguments")
matches the diagnostic emitted by `rustc 1.95.0` for today's contrast
case verbatim — the headline shown is `error[E0107]: missing
generics for trait `AddRhs``, where `missing` is the specific case
"expected N, found 0". The example block in the corpus page covers
struct uses (`struct Bar { x: Foo }`) and function calls
(`foo::<bool>(x)`); today's probe instantiates the same E-code in
the trait-impl-header position.

### `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`

The rmp source target.

#### Line 4 — `impl PartialEq<BigUInt> for BigUInt`

```rust
impl PartialEq<BigUInt> for BigUInt {
    fn eq(&self, other: &BigUInt) -> bool {
        self.limbs == other.limbs
    }
}
```

Corpus warrant for the unlock claim. Today's lesson makes the outer
shape `impl PartialEq<BigUInt> for BigUInt` directly readable: the
trait `PartialEq` declared in std carries a type parameter (just like
today's `AddRhs<RHS>`); the impl substitutes that parameter with the
concrete type `BigUInt` (just like today's `<u32>`); the rest of the
impl is the lesson 113 shape — a method with `&self` and a
`&BigUInt` non-receiver parameter. After today, every token of
`impl PartialEq<BigUInt> for BigUInt { fn eq(&self, other: &BigUInt) -> bool { ... } }`
maps to an installed mechanic except the body's `self.limbs == other.limbs`
which uses `Vec` equality (lesson 107 installed `Vec` basics; `==`
on `Vec` composes onto today's machinery and is a future move).

### `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/add.rs`

The eventual capstone.

#### Lines 112-115 — the capstone outer shape

```rust
impl Add<&BigUInt> for &BigUInt {
    type Output = BigUInt;

    fn add(self: Self, rhs: &BigUInt) -> BigUInt {
```

Corpus warrant for the deferred unlock target. Today makes the
outer shape `impl Add<&BigUInt> for &BigUInt` *partially* readable:
the trait name `Add` and the angle-bracket `<&BigUInt>` are now
familiar (today's `<u32>` taught the slot); the `for &BigUInt` is
new (impl on a reference type, deferred); and `type Output = BigUInt`
is an associated type (deferred). The `self: Self` consuming
receiver shape was already installed by lesson 102. The progress
toward this capstone is real but not complete after today.

## Probes

All probes run in `/tmp/eduratchet-114/`. The committed observation
file is the working probe verbatim.

### Probe 1 — working (`demo.rs`)

```rust
struct Counter {
    count: u32,
}

trait AddRhs<RHS> {
    fn add(&self, rhs: RHS) -> u32;
}

impl AddRhs<u32> for Counter {
    fn add(&self, rhs: u32) -> u32 {
        self.count + rhs
    }
}

fn main() {
    let c = Counter { count: 7 };
    let total = c.add(35);
    println!("total = {}", total);
}
```

```
$ /Users/eli/.cargo/bin/rustc demo.rs
$ echo $?
0
$ ./demo
total = 42
$ echo $?
0
```

Compiles silently, exit 0. Prints `total = 42` (7 + 35), exit 0.
Witness: a generic trait with one type parameter, impl'd with one
concrete substitution, dispatched through the ordinary dot call,
runs to completion.

### Probe 2 — centered E0107 contrast (`no_arg.rs`)

Source: same as Probe 1 with one diff — drop the `<u32>` from the
impl header (`impl AddRhs for Counter` instead of
`impl AddRhs<u32> for Counter`). The trait declaration is unchanged,
the impl body's `fn add(&self, rhs: u32) -> u32 { self.count + rhs }`
is unchanged, and `fn main` is unchanged.

```rust
struct Counter {
    count: u32,
}

trait AddRhs<RHS> {
    fn add(&self, rhs: RHS) -> u32;
}

impl AddRhs for Counter {
    fn add(&self, rhs: u32) -> u32 {
        self.count + rhs
    }
}

fn main() {
    let c = Counter { count: 7 };
    let total = c.add(35);
    println!("total = {}", total);
}
```

```
$ /Users/eli/.cargo/bin/rustc no_arg.rs
error[E0107]: missing generics for trait `AddRhs`
 --> no_arg.rs:9:6
  |
9 | impl AddRhs for Counter {
  |      ^^^^^^ expected 1 generic argument
  |
note: trait defined here, with 1 generic parameter: `RHS`
 --> no_arg.rs:5:7
  |
5 | trait AddRhs<RHS> {
  |       ^^^^^^ ---
help: add missing generic argument
  |
9 | impl AddRhs<RHS> for Counter {
  |            +++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0107`.
$ echo $?
1
```

Diagnostic shape mapped to lesson 003's four-part frame:

- *Headline*: `error[E0107]: missing generics for trait `AddRhs``.
  E0107 is the corpus page's "incorrect number of generic arguments";
  the specific case here is "expected 1, found 0", which rustc
  surfaces as "missing generics for trait `AddRhs`".
- *Caret*: `no_arg.rs:9:6`, single caret span under `AddRhs` at the
  impl header. Inline label `expected 1 generic argument`.
- *`note: trait defined here, with 1 generic parameter: `RHS``: a
  second `-->` line at `no_arg.rs:5:7`, carets under `AddRhs` and
  dashes under `RHS`. Points back at the trait declaration as the
  contract — exactly the multi-`-->` shape lessons 100, 109, 111,
  112, 113 already installed for E0599 / E0603 / E0053 / E0308.
- *`help: add missing generic argument`*: proposes the exact diff
  `impl AddRhs for Counter` → `impl AddRhs<RHS> for Counter`. The
  rustc-generated help literally writes `RHS` (the placeholder
  name) — a learner reading this in context would substitute a
  concrete type like `u32` instead. The `+++++` markers show the
  five inserted characters.
- *Trailer*: standard `error: aborting due to 1 previous error`
  and `--explain E0107`.

The diagnostic empirically grounds the centered claim "the
`<ConcreteType>` after the trait name in the impl header is *required*
when the trait declares a type parameter." Without it, rustc refuses
to compile and points at the trait declaration as the contract
that demands the substitution.

### Probe 3 — auxiliary E0308 (`wrong_type.rs`)

Captured to demonstrate that the impl header's chosen concrete type
flows through to the call site: passing the wrong concrete type to
`c.add(...)` fires E0308 with rustc framing the expectation as
`u32` (the impl's substitution), not `RHS` (the trait's
placeholder). Source: same as Probe 1 with the call site changed
from `c.add(35)` to `c.add(35u64)`.

```rust
struct Counter {
    count: u32,
}

trait AddRhs<RHS> {
    fn add(&self, rhs: RHS) -> u32;
}

impl AddRhs<u32> for Counter {
    fn add(&self, rhs: u32) -> u32 {
        self.count + rhs
    }
}

fn main() {
    let c = Counter { count: 7 };
    let total = c.add(35u64);
    println!("total = {}", total);
}
```

```
$ /Users/eli/.cargo/bin/rustc wrong_type.rs
error[E0308]: mismatched types
  --> wrong_type.rs:17:23
   |
17 |     let total = c.add(35u64);
   |                   --- ^^^^^ expected `u32`, found `u64`
   |                   |
   |                   arguments to this method are incorrect
   |
note: method defined here
  --> wrong_type.rs:6:8
   |
 6 |     fn add(&self, rhs: RHS) -> u32;
   |        ^^^        ---
help: change the type of the numeric literal from `u64` to `u32`
   |
17 -     let total = c.add(35u64);
17 +     let total = c.add(35u32);
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
$ echo $?
1
```

Empirical witness for the post-substitution contract: at the call
site, the parameter type is `u32` (what the impl chose), not `RHS`
(the trait's placeholder). Note that the `note: method defined here`
block points at the trait declaration line `fn add(&self, rhs: RHS) -> u32;`
showing `RHS` literally — the placeholder name lives at the
declaration site — but the inline label on the caret says `expected
`u32``, the concrete type the impl substituted. This is the same
E0308 lesson 113 already installed; the auxiliary probe is
footnote-style only, grounding the deferral that the impl's
substitution propagates to call sites. It is not centered in the
lesson body because the centered teaching is about the impl-header
substitution itself (Probe 2), not its downstream effect.

### Probe 4 — Check Yourself (a)/(b) ground (`tiny.rs`)

```rust
struct Tally { n: u32 }

trait Combine<X> {
    fn combine(&self, x: X) -> u32;
}

impl Combine<u32> for Tally {
    fn combine(&self, x: u32) -> u32 {
        self.n + x
    }
}

fn main() {
    let t = Tally { n: 10 };
    println!("combined = {}", t.combine(5));
}
```

```
$ /Users/eli/.cargo/bin/rustc tiny.rs
$ echo $?
0
$ ./tiny
combined = 15
$ echo $?
0
```

Grounds Check Yourself (a) "yes, silent compile" and (b)
`combined = 15`.

### Probe 5 — Check Yourself (c) ground (`tiny_c.rs`)

Source: Probe 4 with the impl header changed from
`impl Combine<u32> for Tally` to `impl Combine for Tally` (drop the
`<u32>`). Trait declaration and impl body unchanged.

```rust
struct Tally { n: u32 }

trait Combine<X> {
    fn combine(&self, x: X) -> u32;
}

impl Combine for Tally {
    fn combine(&self, x: u32) -> u32 {
        self.n + x
    }
}

fn main() {
    let t = Tally { n: 10 };
    println!("combined = {}", t.combine(5));
}
```

```
$ /Users/eli/.cargo/bin/rustc tiny_c.rs
error[E0107]: missing generics for trait `Combine`
 --> tiny_c.rs:7:6
  |
7 | impl Combine for Tally {
  |      ^^^^^^^ expected 1 generic argument
  |
note: trait defined here, with 1 generic parameter: `X`
 --> tiny_c.rs:3:7
  |
3 | trait Combine<X> {
  |       ^^^^^^^ -
help: add missing generic argument
  |
7 | impl Combine<X> for Tally {
  |             +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0107`.
$ echo $?
1
```

Same E0107 shape as Probe 2 with `Combine<X>` instead of
`AddRhs<RHS>`. Inline label `expected 1 generic argument`. The
`note: trait defined here, with 1 generic parameter: `X`` block
points at the trait declaration. Grounds Check Yourself (c).

## Claim-to-evidence map

| Lesson claim | Source |
|---|---|
| `<RHS>` after the trait name in the declaration is the type-parameter slot | `reference/items/traits.md:73-84` (verbatim "Type parameters can be specified for a trait to make it generic. These appear after the trait name."); `reference/items/generics.md:8-12, 24` (grammar) |
| `RHS` may be used as a type inside the trait body | `reference/items/traits.md:78-80` (`Seq<T>` example uses `T` in `fn elt_at(&self, n: u32) -> T;`) |
| `<ConcreteType>` after the trait name in the impl header substitutes the placeholder | `reference/items/implementations.md:18-24` (TraitImpl grammar with TypePath after `impl`); `reference/items/generics.md:24` (parameters "come directly after `impl`" for implementations) |
| The impl method's signature uses the concrete type after substitution | Lesson 112 (load-bearing — contract-matching rule); Probe 1 (working transcript) |
| The same trait can be impl'd for the same target type with different RHS substitutions | `std/ops/trait.Add.md:17-21` (verbatim `SystemTime` `Add<Duration>` example); deferred from today's probe |
| `Vec<T>` and `Option<T>` are pre-existing uses of the same `<...>` shape | `book/ch10-00-generics.md:13-15` (verbatim "we already used generics ... with `Option<T>` ... `Vec<T>`"); lesson 107 (`Vec` basics) |
| Generics are "abstract stand-ins for concrete types" | `book/ch10-00-generics.md:4-8` (verbatim) |
| Working probe prints `total = 42` | Probe 1 (transcript verbatim) |
| Without `<ConcreteType>` in the impl header, rustc fires E0107 | Probe 2 (transcript verbatim); `error_codes/E0107.md` (corpus headline) |
| E0107 inline label `expected 1 generic argument` | Probe 2 (verbatim) |
| `note: trait defined here, with 1 generic parameter: `RHS`` points at the trait declaration | Probe 2 (verbatim) |
| `help: add missing generic argument` proposes the `<...>` insertion | Probe 2 (verbatim) |
| `std::ops::Add` is `pub trait Add<Rhs = Self> { type Output; fn add(self, rhs: Rhs) -> Self::Output; }` | `std/ops/trait.Add.md:6-13` (verbatim) |
| The rmp `cmp.rs:4` `impl PartialEq<BigUInt> for BigUInt` outer shape becomes readable | `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs:4` (corpus); today's `<ConcreteType>` shape unlocks this |
| The rmp `add.rs:112` `impl Add<&BigUInt> for &BigUInt` outer shape becomes partially readable | `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/add.rs:112-115` (corpus); the `<&BigUInt>` slot is the same as today's `<u32>` modulo the reference type and the impl-on-reference target type |
| Auxiliary: passing the wrong concrete type to the call site fires E0308 with `expected `u32`` | Probe 3 (verbatim) — the impl's substitution flows to the call site |
| Check Yourself (a)/(b): `tiny.rs` compiles silently and prints `combined = 15` | Probe 4 (verbatim) |
| Check Yourself (c): same E0107 shape with `Combine<X>` | Probe 5 (verbatim) |

## Direct prerequisite summary

- **Lesson 113** (load-bearing): installed the trait method shape
  `fn name(&self, p: &Type) -> R;` with a non-receiver parameter
  slot. Today extends 113's parameter slot to hold a *type variable*
  `RHS` instead of a concrete type; the parameter-list grammar
  itself is unchanged. The contrast probe (E0107) does not depend
  on 113's reference machinery — the working probe today uses a
  primitive type `u32` substituted for `RHS`, not `&Counter` — but
  the lesson body's framing "lessons 111-113 wrote concrete trait
  declarations: every type inside the trait body was specific" is a
  one-line summary of the trait arc up through 113.
- **Lesson 112** (load-bearing): installed the contract-matching
  rule "the impl signature must match the trait signature exactly"
  and the diagnostic E0053. Today refines that rule: the impl
  signature must match the trait signature *after* the impl
  header's chosen concrete type substitutes for the trait's type
  parameter. The impl method `fn add(&self, rhs: u32) -> u32`
  matches the trait method `fn add(&self, rhs: RHS) -> u32` after
  `RHS` is replaced by `u32` (what the impl header chose). Lesson
  112 is load-bearing for the reading of the impl method body.
- **Lesson 111** (load-bearing): installed the `trait Name { ... }`
  declaration and the `impl Trait for Type { ... }` block. Today
  extends both headers with angle-bracket type parameters. The
  trait-declaration position (between `trait` and `{`) and the
  impl-header position (between `impl` and `for`) are exactly the
  positions 111 installed; today's `<RHS>` and `<u32>` slot in
  before the brace and before `for` respectively without changing
  the rest of the structure.
- **Lesson 095** (load-bearing): `struct Counter { count: u32 }`
  and `self.count` field access. Reused unchanged in the working
  probe.
- **Lesson 100** (load-bearing): `&self` receiver. Reused unchanged
  in the trait method declaration and impl body.
- **Lesson 040** (load-bearing): the dot-call shape
  `value.method(arg)`. The call site `c.add(35)` is exactly that
  shape; the trait's genericity is invisible at the call site once
  the impl has chosen a concrete type.
- **Lesson 008** (load-bearing): the parameter-list grammar
  `(p1: T1, p2: T2)`. Today's `(&self, rhs: RHS)` slots `RHS` into
  the type position, where `RHS` is a name in scope from the trait
  header.

## Older supporting lessons

- 002 (`fn main`), 005 (`let`), 009 (`+`), 011 (`println!` `{}`),
  019 (type-annotation slot), 080 (`u32`), 003 (diagnostic
  four-part map), 001 (`rustc demo.rs && ./demo`), 107 (`Vec<T>`
  as the learner's first encounter with the angle-bracket shape on
  a *type used*). Each used in lesson 113 and unchanged today;
  cited only for the dependency record. Lesson 107 is the
  load-bearing connection paragraph in the lesson body that bridges
  "you have seen `<...>` on `Vec<T>`" to "today shows the
  declaration side."

## Deferrals

Each item below was named in the lesson's *What To Ignore For Now*
and is not load-bearing for today's claims:

- Multiple impls of the same trait for the same target type with
  different concrete substitutions — *the* canonical demonstration
  of why generic trait parameters exist; named, not probed today.
- Multiple type parameters on one trait (`trait T<A, B>`) — same
  grammar slot extended.
- Generic functions (`fn f<T>(t: T)`) — distinct mechanic, same
  `<T>` slot.
- Generic struct types (`struct S<T>`) — distinct mechanic, same
  `<T>` slot.
- Trait bounds and `where` clauses — constraints on which types may
  substitute.
- Default type parameters (`trait T<U = Self>` — what `std::ops::Add`
  uses).
- Associated types (`type Output = ...;`).
- Lifetime parameters (`trait T<'a>`).
- The orphan rule for generic impls.
- `Self` in the type-parameter slot.
- Trait method receivers other than `&self`.
- Operator traits from `std::ops` (compose today's shape with
  several deferred mechanics above).
- The full rmp `Add<&BigUInt> for &BigUInt` capstone — partially
  readable today, fully readable after associated types and
  impl-on-reference-target are installed.
