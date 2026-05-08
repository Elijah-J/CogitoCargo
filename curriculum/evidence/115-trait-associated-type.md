# Evidence — 115-trait-associated-type

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version`: `rustc 1.95.0 (59807616e 2026-04-14)`.
- `which rustc`: `/Users/eli/.cargo/bin/rustc`.
- Probes run in `/tmp/eduratchet-115/`. Same toolchain family as
  recently accepted lessons (107-114).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/115-trait-associated-type.rs`
is the working probe verbatim.

## Sources

### `output/docs/rust/reference/items/traits.md`

The Reference's chapter on traits.

#### Lines 19-23 — three associated-item kinds

> A *trait* describes an abstract interface that types can implement.
> This interface consists of associated items, which come in three
> varieties:
>
> - functions
> - types
> - constants

Direct corpus warrant for the framing: the trait grammar already
admits associated *types* alongside functions and constants. Lessons
111-114 exercised the *function* variety. Today centers the *type*
variety.

#### Lines 43-46 — declaration without definition for associated items

> Trait functions may omit the function body by replacing it with a
> semicolon. This indicates that the implementation must define the
> function. If the trait function defines a body, this definition
> acts as a default for any implementation which does not override
> it. Similarly, associated constants may omit the equals sign and
> expression to indicate implementations must define the constant
> value. **Associated types must never define the type, the type may
> only be specified in an implementation.**

Direct corpus warrant for today's central rule. The trait body's
`type Output;` form has *no* default — the Reference is explicit
that for the trait declaration form, the impl is *required* to
specify the type. The lesson body quotes this verbatim.

The example block at lines 49-55 shows `type TypeNoDefault;`
alongside the function and constant signatures, the same shape
today's `type Output;` instantiates.

### `output/docs/rust/reference/items/associated-items.md`

The Reference's chapter on associated items.

#### Lines 8-14 — associated-item grammar

> [AssociatedItem] →
>     [OuterAttribute]\* (
>         [MacroInvocationSemi]
>       | ( [Visibility]? ( [TypeAlias] | [ConstantItem] | [Function] ) )
>     )

Corpus warrant for the formal grammar of associated items. Today's
`type Output;` instantiates the `TypeAlias` alternative (without a
visibility modifier and without outer attributes).

#### Lines 242-261 — associated types defined; trait-side declaration grammar

> *Associated types* are type aliases associated with another type.
>
> Associated types cannot be defined in inherent implementations nor
> can they be given a default implementation in traits.
>
> An *associated type declaration* declares a signature for
> associated type definitions. It is written in one of the following
> forms, where `Assoc` is the name of the associated type ... :
>
> ```rust
> type Assoc;
> type Assoc: Bounds;
> type Assoc<Params>;
> type Assoc<Params>: Bounds;
> type Assoc<Params> where WhereBounds;
> type Assoc<Params>: Bounds where WhereBounds;
> ```

Direct corpus warrant for the trait-side syntax. Today's
`type Output;` is the simplest form of the first row — no `Bounds`,
no `Params`, no `WhereBounds`. The other rows are deferred today
(bounds → trait-bounds arc; `Params` → generic associated types).

#### Lines 277-290 — definition shape inside the impl

> An *associated type definition* defines a type alias for the
> implementation of a trait on a type.
>
> They are written similarly to an *associated type declaration*,
> but cannot contain `Bounds`, but instead must contain a `Type`:
>
> ```rust
> type Assoc = Type;
> type Assoc<Params> = Type; // the type `Type` here may reference `Params`
> type Assoc<Params> = Type where WhereBounds;
> ```

Direct corpus warrant for the impl-side syntax. Today's
`type Output = u32;` is the simplest form of the first row — no
`Params`, no `WhereBounds`. The Reference's second row (with
`Params`) is GAT territory and deferred.

#### Line 294 — `<Item as Trait>::Assoc` is an alias

> If a type `Item` has an associated type `Assoc` from a trait
> `Trait`, then `<Item as Trait>::Assoc` is a type that is an alias
> of the type specified in the associated type definition.

Corpus warrant for the lesson's claim that "after resolution,
`Self::Output` *is* `u32` — it is just an alias for the concrete
type." Today's lesson uses the simpler `Self::Output` form rather
than the fully-qualified `<Item as Trait>::Assoc` (which is named in
*What To Ignore For Now*); but the alias semantics are the same.

#### Lines 363-373 — associated-types container example

> Consider the following example of a `Container` trait. Notice that
> the type is available for use in the method signatures:
>
> ```rust
> trait Container {
>     type E;
>     fn empty() -> Self;
>     fn insert(&mut self, elem: Self::E);
> }
> ```

Direct corpus warrant for the use of `Self::Assoc` *as a type*
inside a trait method signature. Today's `Self::Output` in
`fn doubled(&self) -> Self::Output;` is the same shape with
`Output` instead of `E`.

The container example also exercises an associated type in a
non-receiver *parameter* slot (`elem: Self::E`); today defers that
position to a future move and centers the *return-type* slot.

### `output/docs/rust/book/ch20-02-advanced-traits.md`

The Book's chapter 20.2 on advanced traits.

#### Lines 13-18 — associated types defined

> *Associated types* connect a type placeholder with a trait such
> that the trait method definitions can use these placeholder types
> in their signatures. The implementor of a trait will specify the
> concrete type to be used instead of the placeholder type for the
> particular implementation. That way, we can define a trait that
> uses some types without needing to know exactly what those types
> are until the trait is implemented.

Direct corpus warrant for the lesson's plain-English framing of
"placeholder filled by the impl." The lesson's word "resolve" maps
to the Book's "specify the concrete type to be used instead of the
placeholder."

#### Lines 91-105 — generics vs associated types

> The difference is that when using generics, as in Listing 20-14,
> we must annotate the types in each implementation; because we can
> also implement `Iterator<String> for Counter` or any other type,
> we could have multiple implementations of `Iterator` for `Counter`.
> In other words, when a trait has a generic parameter, it can be
> implemented for a type multiple times, changing the concrete types
> of the generic type parameters each time. [...]
>
> With associated types, we don't need to annotate types, because we
> can't implement a trait on a type multiple times. In Listing 20-13
> with the definition that uses associated types, we can choose what
> the type of `Item` will be only once because there can be only one
> `impl Iterator for Counter`.

Direct corpus warrant for the lesson's central mental-model contrast
with lesson 114: generic parameters allow many impls of the same
trait for the same target type with different substitutions;
associated types allow exactly one impl per (trait, target type)
pair, with the associated type chosen exactly once. The lesson's
"unique per (trait, target type) pair" phrasing is grounded here.

#### Lines 169-189 — `std::ops::Add` shape

> ```rust
> trait Add<Rhs=Self> {
>     type Output;
>
>     fn add(self, rhs: Rhs) -> Self::Output;
> }
> ```
>
> [...]
>
> The new part is `Rhs=Self`: This syntax is called *default type
> parameters*.

Corpus warrant for the unlock claim. `std::ops::Add` composes today's
associated type with lesson 114's generic trait parameter and a
default type parameter (`Rhs=Self`, deferred). After today, the
`type Output;` and `type Output = ...;` lines in any `Add` impl are
fully readable.

### `output/docs/rust/std/ops/trait.Add.md`

The std-library page for `Add`.

#### Lines 6-13 — `Add` declaration

> ```
> pub trait Add<Rhs = Self> {
>     type Output;
>
>     // Required method
>     fn add(self, rhs: Rhs) -> Self::Output;
> }
> ```

Direct corpus warrant for the std-Add-uses-Output claim. The
associated type's name is `Output` — the same name today's lesson
chose, intentionally matching std's convention. Today's
`fn doubled(&self) -> Self::Output;` is the same shape as std's
`fn add(self, rhs: Rhs) -> Self::Output;` with the `Rhs` parameter
removed.

### `output/docs/rust/error_codes/E0046.md`

The error-code page for the centered contrast probe.

> Items are missing in a trait implementation.
>
> [...]
>
> When trying to make some type implement a trait `Foo`, you must,
> at minimum, provide implementations for all of `Foo`'s required
> methods (meaning the methods that do not have default
> implementations), as well as any required trait items like
> associated types or constants.

Direct corpus warrant for the contrast probe's diagnostic. The page
explicitly names "associated types or constants" as required trait
items the impl must provide — exactly the failure today's
`no_type.rs` exhibits.

### `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/add.rs`

The eventual capstone target.

#### Lines 112-115 — the rmp Add impl

```rust
impl Add<&BigUInt> for &BigUInt {
    type Output = BigUInt;

    fn add(self: Self, rhs: &BigUInt) -> BigUInt {
```

Direct corpus warrant for the unlock claim. After today, the line
`type Output = BigUInt;` (the resolution of `Add`'s associated
type) is fully readable: it is exactly today's `type Output =
ConcreteType;` form with `BigUInt` filling the `ConcreteType` slot.
The `Add<&BigUInt>` outer header is lesson 114's mechanic with a
reference type as the concrete substitution. The `for &BigUInt`
impl-on-reference target type and the `self: Self` consuming
receiver are lesson-100/102 material; the body's algorithm is its
own future move. The progress toward this capstone is real but
not complete after today.

The lesson body also notes that the impl method writes the resolved
type concretely (`fn add(self, rhs: &BigUInt) -> BigUInt`, with
`BigUInt` rather than `Self::Output` in the return position) — and
that today's probe matches this style for direct rmp readability.

## Probes

All probes run in `/tmp/eduratchet-115/`. The committed observation
file is the working probe verbatim.

### Probe 1 — working (`demo.rs`)

```rust
struct Counter {
    count: u32,
}

trait Doubled {
    type Output;
    fn doubled(&self) -> Self::Output;
}

impl Doubled for Counter {
    type Output = u32;
    fn doubled(&self) -> u32 {
        self.count * 2
    }
}

fn main() {
    let c = Counter { count: 21 };
    println!("doubled = {}", c.doubled());
}
```

```
$ /Users/eli/.cargo/bin/rustc demo.rs
$ echo $?
0
$ ./demo
doubled = 42
$ echo $?
0
```

Compiles silently, exit 0. Prints `doubled = 42` (21 * 2), exit 0.
Witness: a trait with one associated type `Output`, an impl
resolving it to `u32`, an impl method whose signature uses the
concrete `u32` form, and a dot call dispatching through the
ordinary path — all run to completion.

### Probe 2 — centered E0046 contrast (`no_type.rs`)

Source: same as Probe 1 with one diff — drop the `type Output = u32;`
line from the impl block. The trait declaration (still requiring
`type Output;`), the impl method `fn doubled(&self) -> u32 { ... }`,
and `fn main` are unchanged.

```rust
struct Counter {
    count: u32,
}

trait Doubled {
    type Output;
    fn doubled(&self) -> Self::Output;
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

```
$ /Users/eli/.cargo/bin/rustc no_type.rs
error[E0046]: not all trait items implemented, missing: `Output`
  --> no_type.rs:10:1
   |
 6 |     type Output;
   |     ----------- `Output` from trait
...
10 | impl Doubled for Counter {
   | ^^^^^^^^^^^^^^^^^^^^^^^^ missing `Output` in implementation

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0046`.
$ echo $?
1
```

Diagnostic shape mapped to lesson 003's four-part frame:

- *Headline*: `error[E0046]: not all trait items implemented, missing: `Output``.
  E0046 is the corpus page's "items are missing in a trait
  implementation"; the specific case here is "missing one associated
  type". The headline names the missing item by its identifier
  `Output`.
- *Caret 1* at `no_type.rs:6:5`, dashes under `type Output;` in the
  trait body, label ``Output` from trait` — the contract item.
- *Caret 2* at `no_type.rs:10:1`, carets under `impl Doubled for
  Counter` at the impl header, inline label
  `missing `Output` in implementation` — where the resolution
  should have been.
- *Trailer*: standard `error: aborting due to 1 previous error` and
  `--explain E0046`.

The diagnostic empirically grounds the centered claim "the
`type IDENTIFIER = ConcreteType;` line in the impl block is *required*
when the trait declares `type IDENTIFIER;`." Without it, rustc
refuses to compile and points at the trait declaration as the
contract that demands the resolution.

### Probe 3 — auxiliary E0053 (`wrong_type.rs`)

Captured to demonstrate the post-resolution contract-matching rule:
the impl method's signature must match the trait method's signature
*after* `Self::Output` is replaced by the impl's chosen concrete
type. Source: same as Probe 1 except the impl resolves
`type Output = u64;` while the impl method's signature still says
`-> u32`.

```rust
struct Counter {
    count: u32,
}

trait Doubled {
    type Output;
    fn doubled(&self) -> Self::Output;
}

impl Doubled for Counter {
    type Output = u64;
    fn doubled(&self) -> u32 {
        self.count * 2
    }
}

fn main() {
    let c = Counter { count: 21 };
    println!("doubled = {}", c.doubled());
}
```

```
$ /Users/eli/.cargo/bin/rustc wrong_type.rs
error[E0053]: method `doubled` has an incompatible type for trait
  --> wrong_type.rs:12:26
   |
12 |     fn doubled(&self) -> u32 {
   |                          ^^^ expected `u64`, found `u32`
   |
note: type in trait
  --> wrong_type.rs:7:26
   |
 7 |     fn doubled(&self) -> Self::Output;
   |                          ^^^^^^^^^^^^
   = note: expected signature `fn(&Counter) -> u64`
              found signature `fn(&Counter) -> u32`
help: change the output type to match the trait
   |
12 -     fn doubled(&self) -> u32 {
12 +     fn doubled(&self) -> u64 {
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0053`.
$ echo $?
1
```

Witness for the post-resolution contract-matching rule. After
`type Output = u64;` resolves, `Self::Output` *is* `u64`, and the
trait method's signature reads (effectively) `fn doubled(&self) -> u64`.
The impl method writes `-> u32`, which mismatches; rustc fires E0053
(installed by lesson 112). The `note: type in trait` block points at
the trait's `Self::Output` token — i.e. the contract before
resolution — but the `= note: expected signature` summary writes the
*resolved* form `fn(&Counter) -> u64`, witnessing that rustc has
substituted `Output = u64` before checking. This probe is
appendix-only because E0053 is already installed by lesson 112; what
is new here is that the rule extends post-resolution.

### Probe 4 — alternate spelling (`self_output.rs`)

Captured to confirm that the impl method's return type may be
written as either `Self::Output` or the resolved concrete type.
Source: same as Probe 1 with the impl method's return type changed
from `-> u32` to `-> Self::Output`.

```rust
struct Counter {
    count: u32,
}

trait Doubled {
    type Output;
    fn doubled(&self) -> Self::Output;
}

impl Doubled for Counter {
    type Output = u32;
    fn doubled(&self) -> Self::Output {
        self.count * 2
    }
}

fn main() {
    let c = Counter { count: 21 };
    println!("doubled = {}", c.doubled());
}
```

```
$ /Users/eli/.cargo/bin/rustc self_output.rs
$ echo $?
0
$ ./self_output
doubled = 42
$ echo $?
0
```

Compiles silently, exit 0; prints `doubled = 42`. Empirical witness
that both the concrete-type form (`-> u32`, Probe 1) and the
`Self::Output` form (`-> Self::Output`, here) compile and run
identically. The lesson body chooses the concrete-type form because
rmp's `add.rs:115` does (`fn add(self, rhs: &BigUInt) -> BigUInt`).

### Probe 5 — Check Yourself (a)/(b) ground (`tiny.rs`)

```rust
struct Tally { n: u32 }

trait Halved {
    type Output;
    fn halved(&self) -> Self::Output;
}

impl Halved for Tally {
    type Output = u32;
    fn halved(&self) -> u32 {
        self.n / 2
    }
}

fn main() {
    let t = Tally { n: 50 };
    println!("halved = {}", t.halved());
}
```

```
$ /Users/eli/.cargo/bin/rustc tiny.rs
$ echo $?
0
$ ./tiny
halved = 25
$ echo $?
0
```

Grounds Check Yourself (a) "yes, silent compile" and (b)
`halved = 25` (50 / 2 with integer division from lesson 009).

### Probe 6 — Check Yourself (c) ground (`tiny_no_type.rs`)

Source: Probe 5 with the `type Output = u32;` line dropped from the
impl block. Trait declaration and impl method body unchanged.

```rust
struct Tally { n: u32 }

trait Halved {
    type Output;
    fn halved(&self) -> Self::Output;
}

impl Halved for Tally {
    fn halved(&self) -> u32 {
        self.n / 2
    }
}

fn main() {
    let t = Tally { n: 50 };
    println!("halved = {}", t.halved());
}
```

```
$ /Users/eli/.cargo/bin/rustc tiny_no_type.rs
error[E0046]: not all trait items implemented, missing: `Output`
 --> tiny_no_type.rs:8:1
  |
4 |     type Output;
  |     ----------- `Output` from trait
...
8 | impl Halved for Tally {
  | ^^^^^^^^^^^^^^^^^^^^^ missing `Output` in implementation

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0046`.
$ echo $?
1
```

Same E0046 shape as Probe 2 with `Halved`/`Tally` instead of
`Doubled`/`Counter`. Headline `not all trait items implemented,
missing: `Output``. Grounds Check Yourself (c).

## Claim-to-evidence map

| Lesson claim | Source |
|---|---|
| `type IDENTIFIER;` declares an associated type as a required trait item | `reference/items/traits.md:43` ("Associated types must never define the type, the type may only be specified in an implementation."); `reference/items/associated-items.md:250-261` (declaration grammar `type Assoc;`) |
| The semicolon-body form mirrors a method signature with no body | `reference/items/traits.md:43` ("Trait functions may omit the function body by replacing it with a semicolon."); `reference/items/associated-items.md:30` ("definitions that contain the actual implementation and declarations that declare signatures for definitions") |
| `Self::Output` is a path to the associated type usable in any type position | `reference/items/associated-items.md:294` (`<Item as Trait>::Assoc` is an alias to the resolved type); `reference/items/associated-items.md:368-372` (`Container` example uses `Self::E` in a method signature) |
| Inside the trait declaration, `Self::Output` is the only way to refer to the associated type | `reference/items/traits.md:43` (the trait may not give a default), implies the trait body has no other way to spell the eventually-resolved type |
| `type IDENTIFIER = ConcreteType;` inside the impl resolves the associated type | `reference/items/associated-items.md:281-290` (definition grammar `type Assoc = Type;`) |
| After resolution, `Self::Output` *is* `ConcreteType` (an alias) | `reference/items/associated-items.md:294` (verbatim "is a type that is an alias of the type specified in the associated type definition") |
| The impl method's return type may be written either as `Self::Output` or as the concrete type | Probe 1 (concrete `u32` form, compiles); Probe 4 (`Self::Output` form, compiles); both produce `doubled = 42` |
| Today's probe uses the concrete-type form because rmp does | `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/add.rs:115` (`fn add(self: Self, rhs: &BigUInt) -> BigUInt` — concrete `BigUInt`, not `Self::Output`) |
| Generic parameters are filled at the impl header; many impls per (trait, target type) | Lesson 114 (load-bearing); `book/ch20-02-advanced-traits.md:91-99` (verbatim "when a trait has a generic parameter, it can be implemented for a type multiple times") |
| Associated types are filled inside the impl body; one impl per (trait, target type) | `book/ch20-02-advanced-traits.md:100-105` (verbatim "we can choose what the type of `Item` will be only once because there can be only one `impl Iterator for Counter`") |
| Working probe prints `doubled = 42` | Probe 1 (transcript verbatim) |
| Without the `type Output = u32;` line, rustc fires E0046 with `missing: `Output`` | Probe 2 (transcript verbatim); `error_codes/E0046.md` (corpus headline) |
| The E0046 diagnostic points at the trait declaration as the contract | Probe 2 (verbatim — `type Output;` at line 6 underlined as ``Output` from trait`) |
| `std::ops::Add` declares `type Output;` as an associated type | `std/ops/trait.Add.md:6-13` (verbatim) |
| The rmp `add.rs:113` `type Output = BigUInt;` is fully readable after today | `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/add.rs:113` (corpus); today's `type Output = u32;` is the same shape with `BigUInt` filling the concrete-type slot |
| Auxiliary: post-resolution contract mismatch fires E0053 | Probe 3 (verbatim — `Self::Output` resolved to `u64`, impl method writes `-> u32`) |
| Check Yourself (a)/(b): `tiny.rs` compiles silently and prints `halved = 25` | Probe 5 (verbatim) |
| Check Yourself (c): same E0046 shape | Probe 6 (verbatim) |

## Direct prerequisite summary

- **Lesson 114** (load-bearing): installed the trait declaration with
  a generic type parameter `<RHS>` filled at the impl header. Today
  installs the *dual* mechanic — a type slot inside the trait body
  filled inside the impl body. The lesson's framing ("trait
  declarations carry fillable type slots; 114's was at the header,
  today's is at the body") rests on 114. The mental-model contrast
  in *Mental Model Delta* and *What Changed* is the central
  pedagogical bridge.
- **Lesson 112** (load-bearing): installed the contract-matching
  rule and the diagnostic E0053. Today extends the rule to the
  post-resolution form: after `type Output = u32;` resolves, the
  impl method's signature must match either the concrete form or
  `Self::Output`. Probe 3 witnesses E0053 firing when the impl
  method's signature mismatches the resolved type.
- **Lesson 111** (load-bearing): installed `trait Name { ... }` and
  `impl Trait for Type { ... }`. Today places one new associated
  item inside each block: `type Output;` in the trait body and
  `type Output = u32;` in the impl body. The block syntax is
  unchanged.
- **Lesson 100** (load-bearing): installed `Self` as a *type alias*
  inside an impl block. Today's `Self::Output` is a path *through*
  that alias to a name the trait declared. The same `Self` is reused
  as the namespace prefix for the associated-type path; this is the
  bridging concept that lets a learner read `Self::Output` as "the
  thing called `Output` belonging to `Self`."
- **Lesson 095** (load-bearing): `struct Counter { count: u32 }` and
  `self.count` field access. Reused unchanged in the working probe.
- **Lesson 040** (load-bearing): the dot-call shape `c.doubled()`,
  unchanged.
- **Lesson 008** (load-bearing): the function/method signature
  grammar `fn name(p: T) -> R { ... }`. Today's signatures slot
  `Self::Output` (in the trait declaration) or `u32` (in the impl
  method) into the return-type position.

## Older supporting lessons

- 002 (`fn main`), 005 (`let`), 009 (`*`, integer division),
  011 (`println!` `{}`), 019 (type-annotation slot), 080 (`u32`),
  003 (diagnostic four-part map), 001 (`rustc demo.rs && ./demo`).
  Each used in lesson 114 and unchanged today; cited only for the
  dependency record.

## Deferrals

Each item below is named in the lesson's *What To Ignore For Now*
and is not load-bearing for today's claims:

- **Default associated types** — `type Output = u64;` *inside the
  trait body* declares a default the impl may override. Today's
  trait gives no default, so the impl is required to resolve.
- **Multiple associated types in one trait** — `type A; type B;`.
  Each requires its own resolution line in the impl. Rule extends
  naturally.
- **Associated types with trait bounds** — `type Output: Display;`.
  Blocked on the trait-bounds arc.
- **Generic associated types (GATs)** — `type Item<'a>;`. Blocked
  on lifetime parameters.
- **Associated types as parameter types** —
  `fn foo(&self, x: Self::Output)`. Same path syntax in a different
  position; cleaner as a separate move.
- **Cross-impl access via qualified path** —
  `<Counter as Doubled>::Output` — the fully-qualified form
  `reference/items/associated-items.md:292-298` describes. Today
  uses the simpler `Self::Output` form inside the trait body; the
  qualified form is for *outside* an impl, deferred.
- **Default type parameters on the trait** — `trait T<U = Self>` —
  what `std::ops::Add` uses. Already deferred from 114.
- **Multiple impls dispatching different `Output` types** — one
  trait, many target types, each impl resolving `Output`
  differently. The dispatch payoff (e.g., std's `Iterator::Item`).
  Named, not probed today.
- **Operator traits from `std::ops`** — compose today's mechanic
  with 114's generic parameter and a default type parameter. The
  next major composite move.
- **The orphan rule, `where` clauses, lifetimes** — wholesale from
  111/114.
- **The full rmp `Add<&BigUInt> for &BigUInt` capstone** — partially
  readable today (the `type Output = BigUInt;` line is now fully
  readable; the `Add<&BigUInt>` outer header reuses 114's mechanic
  with a reference type as the concrete substitution; the
  `for &BigUInt` impl-on-reference-target type is a separate
  deferred mechanic).
