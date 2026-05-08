---
id: 130-capstone-rmp-cmp-ordering
status: accepted
evidence: ../evidence/130-capstone-rmp-cmp-ordering.md
---

# Capstone: read rmp's `cmp.rs:12-33` PartialOrd + Ord pair end-to-end

## The Capstone

This is *Capstone Mode* — the fourth in this run after lessons 063,
067, 110, and 118. **No new Rust mechanic is centered today.** The
trait-and-iterator arc — lessons 119 through 129 — was assembled to
make one specific real-world rmp slice readable end-to-end: the
`PartialOrd` and `Ord` impls on `BigUInt` in `src/biguint/cmp.rs`,
lines 12-33. Today reads them.

The literal source from `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`,
plus the file-level `use` on line 2:

```rust
use std::cmp::{self, Ord, Ordering};

impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigUInt {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.limbs.len().cmp(&other.limbs.len());
        if ord == cmp::Ordering::Equal {
            for (left, right) in self.limbs.iter().rev().zip(other.limbs.iter().rev()) {
                match left.cmp(right) {
                    Ordering::Equal => {}
                    ord => return ord,
                }
            }
            return Ordering::Equal;
        } else {
            ord
        }
    }
}
```

Twenty-two lines, two `impl` blocks, one new `use` form, four named
deferrals (the supertrait `PartialOrd: PartialEq`, the supertrait
`Ord: Eq + PartialOrd`, the default RHS `<Rhs = Self>`, and the
auto-deref through `&u64` on `left.cmp(right)`).

## Walk Through The Code

Line by line, mapping each token to the lesson that installed it.

**Line 2 — `use std::cmp::{self, Ord, Ordering};`**

Lesson 044 installed the simple `use Path::Item;` form. Today's
declaration extends that with the *grouped* form: a path prefix
followed by `{...}` containing multiple items, plus the keyword
`self` (Reference `items/use-declarations.md` lines 27-31). The
prefix `std::cmp::` is shared. Inside the braces:

- `self` brings the *module* `cmp` itself into scope. After this
  line, the bare name `cmp` is usable as a path prefix — that is what
  makes `cmp::Ordering::Equal` on line 21 resolve.
- `Ord` brings the trait name into scope, so `impl Ord for BigUInt`
  on line 18 finds it without a `std::cmp::` prefix.
- `Ordering` brings the enum name into scope, so `Ordering` on lines
  19, 24, 25, and 28 works bare. (Lesson 051 installed the same form
  with one item.)

Three names imported by one line.

**Line 12 — `impl PartialOrd for BigUInt {`**

Lesson 111 installed `impl Trait for Type { ... }`. The trait name is
now `PartialOrd`. Std's declaration (`std/cmp/trait.PartialOrd.md`
lines 7-10) reads `pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
where Rhs: ?Sized`. Three named-deferred pieces, exactly as lesson
118 deferred for `PartialEq`:

- `<Rhs = Self>` — the same default-type-parameter machinery; rmp
  writes neither `<BigUInt>` nor `<Self>`, so `Rhs` defaults to
  `Self`, which inside this impl is `BigUInt`.
- `: PartialEq<Rhs>` — a supertrait clause. Read structurally as
  "to implement `PartialOrd` for a type, that type must also have
  `PartialEq` implemented." rmp's `cmp.rs:4-7` line satisfies this
  (lesson 118 read it).
- `where Rhs: ?Sized` — the `?Sized` bound, named-deferred since 118.

**Line 13 — `fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {`**

Lesson 100 installed `&self` as a borrowing receiver and the
`fn name(...) -> Type` shape. Lesson 120 installed `other: &Self` in
a non-receiver slot (inside an impl, `Self` is the impl-target type,
so `&Self` means `&BigUInt` here). Lesson 119 installed `Option<T>`
as the prelude type with `Some` / `None` constructors. The return
type is `Option<std::cmp::Ordering>`: an `Option` parameterized by
the `Ordering` enum (lesson 051), reached via the qualified path
`std::cmp::Ordering` (lesson 043). The line composes 100 + 120 +
119 + 051 + 043 with no new mechanic.

**Line 14 — `Some(self.cmp(other))`**

Lesson 122: `self.method(args)` — a sibling method on the same impl.
The sibling is `cmp`, declared in the second impl block below;
return type `Ordering` (lesson 127). Lesson 119: `Some(value)` is
the call-expression constructor wrapping a `T` into an `Option<T>`.
So this expression calls `cmp` to get an `Ordering`, then wraps it
in `Some` to produce an `Option<Ordering>`. Lesson 025: no `;`, no
`return`, so the expression is the function's implicit return.

`partial_cmp` here always returns `Some(...)` — never `None` —
because `BigUInt`s form a *total* order. The `Option` wrapping is
required by the trait signature, not by missing information.

**Line 16 — `}`** closes the impl block.

**Line 18 — `impl Ord for BigUInt {`**

Lesson 111 again, with trait name `Ord`. Std's declaration
(`std/cmp/trait.Ord.md` lines 7-9) reads `pub trait Ord: Eq +
PartialOrd { fn cmp(&self, other: &Self) -> Ordering; ... }`. The
`: Eq + PartialOrd` clause is *two* supertraits joined by `+`, each
named-deferred. Read structurally: to implement `Ord` for a type,
both `Eq` and `PartialOrd` must already be implemented for that type.
rmp satisfies both — `Eq` via line 10 (lesson 118 read it) and
`PartialOrd` via lines 12-16 above.

**Line 19 — `fn cmp(&self, other: &Self) -> Ordering {`**

Same composition as line 13, with `Ordering` directly (no `Option`)
and the `Self` shorthand (lesson 120) for the parameter type.

**Line 20 — `let ord = self.limbs.len().cmp(&other.limbs.len());`**

Read left to right. `self.limbs` is field access through a `&self`
receiver (lesson 095 + 100), reading the `Vec<u64>`. `.len()` (lesson
107) returns the count as `usize`. `.cmp(...)` (lesson 127) on a
`usize` compares it with another `usize`, taking `&Self` —
i.e., `&usize`. The argument is `&other.limbs.len()`: lesson 045's
prefix-`&` operator applied to the `usize` value `other.limbs.len()`.
Lesson 049 chains the dot-calls: `.limbs.len().cmp(...)` is one
expression. Lesson 005 binds the result to `ord` — type `Ordering`.

**Line 21 — `if ord == cmp::Ordering::Equal {`**

Lesson 026 installed `if condition { ... }` as an expression. The
condition is `ord == cmp::Ordering::Equal`: lesson 121 installed
`==` on two `Ordering` values. The right-hand side
`cmp::Ordering::Equal` is the centered new use of line 2's grouped
`use`: the bare `cmp` is now a usable path prefix, because `self` in
`use std::cmp::{self, ...}` brought the module name into scope.
Lesson 044's mechanic was extended; lesson 051's `Ordering::Equal`
constructor still works. The condition is `true` exactly when the
two limb-vectors have the same length.

**Line 22 — `for (left, right) in self.limbs.iter().rev().zip(other.limbs.iter().rev()) {`**

The whole iterator chain composes 123 + 124 + 125: `.iter()` produces
an iterator over a `Vec<u64>`, `.rev()` reverses it (so the
*most-significant* limb comes first — `limbs` is little-endian per
lesson 110), `.zip(...)` pairs it element-by-element with a second
identically-built iterator on `other.limbs`. The yielded values are
2-tuples of type `(&u64, &u64)`. The for-binding `(left, right)` is
lesson 126's tuple-pattern destructuring at the loop slot — no whole
`pair` name, just the two parts.

After this line, each loop pass has `left: &u64` and `right: &u64`,
walking from most-significant limb downward.

**Line 23 — `match left.cmp(right) {`**

Lesson 127: `.cmp` on `u64` returns `Ordering`. `left` and `right`
are *references* `&u64`, but `.cmp` is declared on `u64`; auto-deref
(named-deferred from 100) finds `u64::cmp` through the `&u64`. The
argument `right` is already a `&u64` and `.cmp` expects `&Self`
(i.e., `&u64`), so it matches without an extra `&` (lesson 113).
Lesson 058: `match` on the returned `Ordering` value.

**Line 24 — `Ordering::Equal => {}`**

Lesson 128 installed exactly this shape: a unit-variant pattern
(lesson 098) followed by an empty block body. Type `()`, value `()`.
The arm matches when this pair of limbs is equal, and the match
falls through to the next loop iteration (the loop body has no
further statements after the match).

**Line 25 — `ord => return ord,`**

Lesson 129 installed exactly this shape: a bare-name binding pattern
catches every other `Ordering` value (`Less` or `Greater`) and binds
it to the local name `ord`; the body `return ord;` exits the
*entire* `cmp` function with that value. The post-match code, the
rest of the for-loop, and the outer `if`-block tail are all skipped
for that call. The all-arms-share-a-type rule of lesson 030 is
exempted because `return ord;` is diverging (lesson 129 named this
exemption).

This `ord` *shadows* the outer `ord` from line 20 (the limb-length
comparison's `Ordering`); the shadowing is a mechanic deferred from
lesson 005, but the inner `ord` only exists inside this arm body, so
no behavior depends on the shadowing detail.

**Line 28 — `return Ordering::Equal;`**

Lesson 021: `return value;`. After the for-loop visits every limb
pair without diverging, every pair was `Equal`, so the two
`BigUInt` values are equal. This `return` is *not* in tail position
— it sits inside the if-then block — so the implicit form would
need restructuring. rmp's explicit `return` is stylistic.

**Line 29 — `} else {`** is lesson 014's `if-else`. The else branch
fires when the limb lengths differ.

**Line 30 — `ord`**

This is the *outer* `ord` from line 20 (the inner `ord` of line 25
is dead with the match block). Lesson 025: tail expression of the
else-block, which is the value of the whole if-else (lesson 026),
which is the function's tail expression. When limb lengths differ,
the limb-length `Ordering` is the answer — a longer `Vec<u64>`
represents a larger big integer because `limbs` is little-endian
and trailing-zero-free (lesson 110).

**Lines 31, 32, 33** close the if-else, the function body, and the
impl block.

## Three New Structural Facts

Three structural pieces show up today that no single prior lesson
centered. Each gets a brief, corpus-grounded statement.

**A. The `PartialOrd` trait declaration.** From
`output/docs/rust/std/cmp/trait.PartialOrd.md` lines 7-10:

```text
pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
where
    Rhs: ?Sized,
```

The required method is `fn partial_cmp(&self, other: &Rhs) ->
Option<Ordering>` (line 14). Provided methods `lt` / `le` / `gt` /
`ge` supply the `<`, `<=`, `>`, `>=` operators (Reference
`expressions/operator-expr.md` lines 527-530). The `<Rhs = Self>`
default and the `: PartialEq<Rhs>` supertrait are named-deferred,
matching 118's deferral for `PartialEq`. The supertrait clause is
structurally *why* `cmp.rs:4-7`'s `impl PartialEq` is required for
`cmp.rs:12-16` to compile.

**B. The `Ord` trait declaration.** From
`output/docs/rust/std/cmp/trait.Ord.md` lines 7-9:

```text
pub trait Ord: Eq + PartialOrd {
    // Required method
    fn cmp(&self, other: &Self) -> Ordering;
```

Two supertraits joined by `+`. Read structurally: to implement `Ord`
for a type, both `Eq` and `PartialOrd` must already be implemented
for that type. rmp's `cmp.rs:10` (lesson 118) plus `cmp.rs:12-16`
(today's first impl) satisfy them. The provided methods (`max`,
`min`, `clamp`) are named-deferred.

**C. The grouped `use` with `self`.** Reference
`items/use-declarations.md` lines 27-31:

> Simultaneously binding a list of paths with a common prefix, using
> the brace syntax `use a::b::{c, d, e::f, g::h::i};`
>
> Simultaneously binding a list of paths with a common prefix and
> their common parent module, using the `self` keyword, such as
> `use a::b::{self, c, d::e};`

rmp's `use std::cmp::{self, Ord, Ordering};` is the second form. It
is one statement that binds three names: the module `cmp`, the trait
`Ord`, and the enum `Ordering`. After this line, all three are
usable bare; `cmp` works as a path prefix, witnessed on line 21
(`cmp::Ordering::Equal`).

## Empirical Witness

Three probes, transcripts in the appendix.

**Probe 1 — self-contained mirror.** `observations/130-capstone-rmp-cmp-ordering.rs`
mirrors cmp.rs:1-33 verbatim with a small `BigUInt`-shaped struct
(field plain `pub` because the file has no parent module). `rustc
... && ./probe` prints:

```text
a.partial_cmp(&b) is Some(Less)? true
a.cmp(&b) = Less
a.cmp(&c) = Equal
zero.cmp(&from0) = Less
p.cmp(&q) (MSL 1 vs 2)  = Less
```

Each output line witnesses one piece. Line 1: `partial_cmp` returns
`Some(Less)`. Line 2: `cmp` returns `Less` directly (equal lengths,
MSL `100 < 200`). Line 3: equal-valued limbs → `Equal`. Line 4:
lengths differ (`vec![]` vs `vec![0]`), the else-branch fires, and
the limb-length `Ordering` is returned — `0 < 1` → `Less`. Line 5:
equal-length, big-endian comparison sees MSLs `1 < 2` and exits the
for-loop early via the `ord => return ord` arm.

**Probe 2 — cross-crate driver.** Path dependency
`bignum = { path = "/Users/eli/InfoScraper/output/repos/rmp" }`,
imports `BigUInt`, exercises `partial_cmp` / `cmp` and the
canonical-zero comparison. `cargo run`:

```text
a.partial_cmp(&b) == Some(Less): true
a.cmp(&b) = Less
a.cmp(&c) = Equal
zero().cmp(&from(0u64)) = Less
```

The first three lines compose with rmp's actual source. The fourth
is the honest defect (next section).

**Probe 3 — rmp's own tests.** `cd
/Users/eli/InfoScraper/output/repos/rmp && cargo test --lib` reports
`17 passed; 0 failed`. The capstone reads code that already passes
its own author's test suite.

**Contrast probes (transcripts in appendix).** Three negative probes
witness the trait-dispatch wiring:

- *Drop `impl PartialOrd`*: `impl Ord for BigUInt` then fails with
  `error[E0277]: can't compare \`BigUInt\` with \`BigUInt\``, with
  the help line `the trait \`PartialOrd\` is not implemented for
  \`BigUInt\`` and the `note: required by a bound in \`Ord\``.
  Witness for *fact B* — `Ord`'s `: ... + PartialOrd` supertrait is
  enforced at compile time. The same probe also fires E0599 on
  `.partial_cmp()` and E0369 on `<` — the operator and method paths
  both go through `PartialOrd`.
- *Drop `impl Ord`*: `.cmp()` fires `error[E0599]: \`BigUInt\` is not
  an iterator ... method \`cmp\` not found`, with the candidate list
  `Iterator` / `Ord`. Witness that `.cmp` on a `BigUInt` reaches the
  `Ord` impl; without it, the method does not exist.
- *Drop `impl PartialEq`*: `impl PartialOrd for Score` fails with
  `error[E0277]: can't compare \`Score\` with \`Score\` ... the
  trait \`PartialEq\` is not implemented for \`Score\` ... required
  by a bound in \`PartialOrd\``. Witness for *fact A* — `PartialOrd`'s
  `: PartialEq<Rhs>` supertrait is also enforced at compile time.

## Honest Defect

The cross-crate driver's fourth line surfaces a defect lessons 110
and 118 already identified on different axes. `BigUInt::zero()`
builds `BigUInt { limbs: vec![] }` (canonical, length 0).
`BigUInt::from(0u64)` builds `BigUInt { limbs: vec![0] }`
(non-canonical, length 1). Both represent the mathematical value 0.

Today's `cmp` body starts with `self.limbs.len().cmp(&other.limbs.len())`.
The lengths are 0 and 1; `usize::cmp(&0, &1)` returns
`Ordering::Less`. The if-condition is false, the else-branch fires,
the function returns `Ordering::Less`. So `BigUInt::zero() <
BigUInt::from(0u64)` — strict inequality between two values that
should be mathematically equal.

A standalone probe `canonical_zero_defect.rs` (transcript in
appendix) confirms three axes in one run: `zero.cmp(&from(0u64)) =
Less`, `zero < from(0u64) = true`, `zero == from(0u64) = false`.
Lesson 110 (length-only `is_zero`), lesson 118 (`==`), and today's
`<` / `cmp` all expose the same `From<u64>` discipline failure.
rmp's own tests (Probe 3) do not exercise this edge.

## What Changed

- Lessons 119-129 compose into reading rmp's `cmp.rs:12-33`
  end-to-end. Every token, every line, every operator dispatch is
  traceable to an installed lesson plus a small set of explicitly
  named deferrals (the supertraits, `?Sized`, `<Rhs = Self>`,
  auto-deref).
- Two new trait names enter the audience's vocabulary: `PartialOrd`
  (the trait `<` / `<=` / `>` / `>=` and `partial_cmp` dispatch
  through) and `Ord` (the trait `.cmp` dispatches through). Both
  are named, neither is *centered* — the run still has not
  installed the formal supertrait or default-type-parameter moves.
- One new compositional `use` form: `use std::cmp::{self, Ord,
  Ordering};` — grouped braces with the `self` keyword bringing the
  module itself into scope alongside two of its items. Lesson 044
  extended.
- The honest-defect picture broadens: rmp's `From<u64>` discipline
  failure now shows up on three axes (lesson 110 `is_zero`, lesson
  118 `==`, today `<` / `cmp`). Same root cause; three different
  witnesses.
- The full file `cmp.rs` is now read end-to-end across two
  capstones: the line-1 `use super::basic::BigUInt;` plus the
  `PartialEq` + `Eq` halves in lesson 118, and today's line-2
  grouped `use` plus the `PartialOrd` + `Ord` halves. No tokens
  in `cmp.rs` are unread.

## Check Yourself

Save the mirror probe as `mirror.rs` (with the cmp.rs:12-33 logic
verbatim and a small `BigUInt`-shaped struct).

(a) Predict `BigUInt { limbs: vec![5, 1] }.cmp(&BigUInt { limbs:
vec![999] })`. Walk the cmp body in your head, then confirm by
adding the line to the probe.

(b) On line 21 (`if ord == cmp::Ordering::Equal`), the `cmp` on the
right is the *module* and `Ordering::Equal` is its variant
constructor (lesson 051). Which line of the file makes the bare
`cmp` work, and which Reference grammar describes it?

(c) The line `ord => return ord,` has *two* different `ord` names —
the outer `let ord = ...;` on line 20 and the inner pattern binding
on line 25. If you swap the inner name to `other_ord`, does the
program still compile? Why or why not?

*(Answers: (a) The first `BigUInt` has length 2; the second has
length 1. The line-20 `usize::cmp` returns `Greater` (`2 > 1`). The
if-condition is false; the else-branch fires; `cmp` returns
`Greater`. The two-limb value is larger because limbs are
little-endian and trailing-zero-trimmed, so length corresponds to
magnitude. (b) Line 2's `use std::cmp::{self, Ord, Ordering};` —
the `self` keyword in the grouped `use` brings the module name
itself into scope. Reference: `items/use-declarations.md` lines
27-31. (c) Yes, it still compiles. The arm-pattern binding name is
local to the arm body, and the body is just `return other_ord;`.
The outer `ord` is unaffected. The actual rmp source's name reuse
is a stylistic choice; the inner binding shadows the outer
*inside this arm only*, but no code in the arm body reads the
outer name.)*

## What To Ignore For Now

Today reads only `cmp.rs:12-33`. Named deferrals:

- *The supertrait colon-form* (`PartialOrd: PartialEq<Rhs>` and
  `Ord: Eq + PartialOrd`). Read structurally only; the formal
  supertrait machinery is its own move.
- *The default-type-parameter `<Rhs = Self>` and the `?Sized`
  bound.* Same machinery 118 named-deferred.
- *Provided trait methods.* `PartialOrd`'s `lt` / `le` / `gt` / `ge`
  (lesson 116's mechanic in std use); `Ord`'s `max` / `min` /
  `clamp`. Each its own move.
- *The `<` / `<=` / `>` / `>=` operators on `BigUInt`.* Wired via
  `PartialOrd::lt` / `le` / `gt` / `ge` (Reference table lines
  527-530). Witnessed today only via the contrast probes' `E0369`
  on `<`; centered run-through is its own move.
- *Auto-deref on `left.cmp(right)` for `&u64` operands.* Named in
  100; surfaces today on line 23.
- *Shadowing in nested scopes.* Line 25's `ord` shadows line 20's
  `ord` inside the match arm. Lesson 005 named shadowing deferred;
  today rides the mechanic without exercising it.
- *The `Reverse<T>` adapter, `cmp::reverse`, `cmp::min`, `cmp::max`*
  — std `cmp` items unused by rmp's slice.
- *`#[derive(PartialOrd)]` and `#[derive(Ord)]`*. Named only in
  contrast probes' `help:` text.
- *E0277, E0369, E0599 catalogue.* The diagnostic E-codes surfaced
  by the contrast probes; the catalogue itself is its own arc.
- *`PartialOrd` for types that do **not** form a total order*
  (e.g., `f64` with `NaN`). `partial_cmp` can then return `None`.
  rmp's `BigUInt` is total-ordered, so `partial_cmp` always returns
  `Some(...)` — `Some(self.cmp(other))` is the canonical shape for
  total-order types. The `None` case is its own move.

The whole rest of rmp — `biguint/add.rs`, `mul.rs`, `div.rs`,
`shift.rs`, `format.rs`, the rest of `convert.rs`, `bigint.rs` —
composes the trait arc installed through 129 with arithmetic,
iteration, and formatting machinery yet to be installed.

## Evidence

See `../evidence/130-capstone-rmp-cmp-ordering.md`.
