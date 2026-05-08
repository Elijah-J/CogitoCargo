# Evidence — 061-i32-cmp-returns-ordering

Audit appendix for `lessons/061-i32-cmp-returns-ordering.md`. Holds the
corpus-quote map, the toolchain string, the full working and broken-
contrast probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` (the Less probe) is committed,
  under `observations/061-i32-cmp-returns-ordering.rs`. The Equal,
  Greater, and broken-contrast `.rs` files are not committed; their
  transcripts below are the artifacts.

## Sources

### `output/docs/rust/std/cmp/trait.Ord.md`

The std-library page for the `Ord` trait. New citation in this run.
Three load-bearing spans.

Lines 6-19 (the trait declaration with `cmp`'s signature):

> ```
> pub trait Ord: Eq + PartialOrd {
>     // Required method
>     fn cmp(&self, other: &Self) -> Ordering;
>
>     // Provided methods
>     ...
> }
> ```

Two load-bearing facts in this declaration:

1. `cmp` is declared on the `Ord` trait — the structural reason `cmp`
   is reachable on `i32` is the `impl Ord for i32` block (next source
   below). The lesson does *not* surface the trait machinery; it
   names `Ord` in passing as "the std page for the `Ord` trait" and
   defers traits as a topic.
2. The signature `fn cmp(&self, other: &Self) -> Ordering` is the
   load-bearing claim used in the lesson body and *What Changed*. It
   says, in the order the lesson decodes it: a method (cycle 040),
   takes its first argument (the receiver) by shared reference
   (`&self`), takes a second argument also by shared reference
   (`other: &Self`), and returns an `Ordering` value.

The `pub trait Ord: Eq + PartialOrd { ... }` super-trait bound (the
`: Eq + PartialOrd` part) is explicitly deferred — the lesson's *What
To Ignore For Now* names both `Ord` and `PartialOrd` as deferred trait
machinery.

Lines 265-280 (the `cmp` Required-Method header, audience-level
description, the convention sentence, and Examples block):

> #### fn [cmp](#tymethod.cmp)(&self, other: &Self) -> [Ordering]
>
> This method returns an [`Ordering`] between `self` and `other`.
>
> By convention, `self.cmp(&other)` returns the ordering matching the
> expression `self <operator> other` if true.
>
> ##### Examples
>
> ```
> use std::cmp::Ordering;
>
> assert_eq!(5.cmp(&10), Ordering::Less);
> assert_eq!(10.cmp(&5), Ordering::Greater);
> assert_eq!(5.cmp(&5), Ordering::Equal);
> ```

Three load-bearing pieces:

- "This method returns an `Ordering` between `self` and `other`." —
  the corpus statement of the lesson's "the whole call expression has
  type `Ordering`" claim.
- The convention sentence — *"`self.cmp(&other)` returns the ordering
  matching the expression `self <operator> other` if true"* — is
  quoted verbatim in the lesson body's *Try It* section ("the std
  `Ord` page's convention sentence makes it verbal: ...") and is the
  corpus license for the *Check Yourself* (c) answer
  ("`x.cmp(&y) == Ordering::Equal` exactly when `x == y`").
- The Examples block is direct corpus precedent for the exact shape
  the lesson uses: receiver `.cmp(&other)`, where the argument is a
  `&` to the *literal/binding* on the right. The Examples block shows
  all three result variants (`Less`, `Greater`, `Equal`) with
  integer-literal receivers; the lesson's three runs (`a=3,b=5` /
  `a=3,b=3` / `a=7,b=5`) parallel them with `i32`-typed bindings.

The lesson does not cite the trait's *Corollaries* section, the
*Derivable* section, the *Lexicographical comparison* section, or the
list of provided methods (`max`, `min`, `clamp`). All four are
deferred; `min`/`max` are sibling free functions in `std::cmp`
(cycle 043 already installed `std::cmp::min`).

Calibration: the page also lists ~50 `impl Ord for ...` blocks (lines
381+), confirming `Ord` is implemented for many types beyond `i32`,
including the other integers, `bool`, `char`, `str`, `String`, `Vec`,
arrays, tuples, etc. The lesson defers all of these to *What To
Ignore For Now*.

### `output/docs/rust/std/primitive.i32.md`

The std-library page for the `i32` primitive type. Already cited in
cycle 040 (for `i32::abs`) and cycle 045 (transitively, for the
integer type the references referred to). Today's citation is the
specific `impl Ord for i32` block. One load-bearing span.

Lines 4066-4074 (the `Ord` implementation for `i32`):

> ### impl [Ord](cmp/trait.Ord.md "trait std::cmp::Ord") for [i32](primitive.i32.md)
>
> #### fn [cmp](cmp/trait.Ord.md#tymethod.cmp)(&self, other: &[i32](primitive.i32.md)) -> [Ordering](cmp/enum.Ordering.md "enum std::cmp::Ordering")
>
> This method returns an [`Ordering`] between `self` and `other`.

This is the corpus statement that (a) `i32` implements `Ord`, so
`cmp` is reachable on `i32`-typed values, and (b) the specialized
signature for `i32` reads `fn cmp(&self, other: &i32) -> Ordering`
— the trait's `&Self` is concretely `&i32`. The lesson body's
"there is a method named `cmp` on `i32`" is grounded here. The
`&i32` argument type is exactly what makes the broken-contrast probe
fire E0308 when the call site passes a bare `i32`.

Calibration: the same page also documents `clamp`, `max`, and `min`
as provided methods inherited from `Ord` (lines 4076-4092). The
lesson defers all three. The page also documents `impl PartialOrd
for i32` with `partial_cmp(&self, other: &i32) -> Option<Ordering>`
(lines 4111-4119); `partial_cmp` is the load-bearing reference for
the *What To Ignore For Now* deferral of the `f64::NAN` case.

### `output/docs/rust/std/cmp/enum.Ordering.md`

The std-library page for `Ordering`. Already cited in cycle 051 for
the type's declaration, the variants, the canonical Examples block,
and the per-variant gloss. Reused here for one load-bearing
re-citation. Lines 22-30:

> ## Examples
>
> ```
> use std::cmp::Ordering;
>
> assert_eq!(1.cmp(&2), Ordering::Less);
>
> assert_eq!(1.cmp(&1), Ordering::Equal);
>
> assert_eq!(2.cmp(&1), Ordering::Greater);
> ```

Cycle 051's evidence appendix already cited this block. Today's
re-citation is for what cycle 051 *deferred*: the Examples block
shows `cmp` *producing* `Ordering` values rather than hardcoding
them. With cycles 040, 045, and 051 installed, this corpus example
is now fully decodable: `1.cmp(&2)` is a method call (cycle 040)
with a shared reference argument (cycle 045) returning an
`Ordering` (cycle 051) — the exact composition this lesson teaches.
The lesson body's first sentence ("A pure composition cycle. Three
previously-installed cycles snap together") is grounded by the
fact that the corpus already uses this composition; the cycle
makes it readable.

The page's three Variants section — `Less = -1` / `Equal = 0` /
`Greater = 1` — is reused unchanged from cycle 051; the lesson
treats the three variants exactly as cycle 051 installed them.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book chapter for the guessing game. Already cited in cycles 042,
050, and 051. Reused here for the audience-level introduction of
`cmp` as the operation that *produces* `Ordering` values. Two
load-bearing spans.

Lines 778-783 (the canonical `match guess.cmp(&secret_number)`
listing):

> ```rust
> match guess.cmp(&secret_number) {
>     Ordering::Less => println!("Too small!"),
>     Ordering::Greater => println!("Too big!"),
>     Ordering::Equal => println!("You win!"),
> }
> ```

Direct corpus precedent for the `match receiver.cmp(&other) { ... }`
shape. The Book uses `guess` and `secret_number` (each a number);
today's lesson uses `a` and `b` (each `i32`). The arm shape (three
variant patterns each producing a `println!`-as-arm-body) is
identical to the lesson's working probe. Cycle 051's evidence
appendix already cited this listing for the three-arm match shape;
today's re-citation is for the `cmp(&other)` *call* in the scrutinee
position, which cycle 051 explicitly deferred.

Lines 793-800 (the audience-level prose statement of `cmp`):

> Then, we add five new lines at the bottom that use the `Ordering`
> type. The `cmp` method compares two values and can be called on
> anything that can be compared. It takes a reference to whatever
> you want to compare with: Here, it’s comparing `guess` to
> `secret_number`. Then, it returns a variant of the `Ordering` enum
> we brought into scope with the `use` statement. We use a [`match`]
> expression to decide what to do next based on which variant of
> `Ordering` was returned from the call to `cmp` with the values in
> `guess` and `secret_number`.

Three audience-level corpus statements load-bearing for the lesson:

1. "The `cmp` method compares two values" — direct gloss of `cmp`'s
   purpose. The lesson's "to ask which of two integers is smaller,
   call `a.cmp(&b)`" rephrases this for the integer case.
2. "It takes a reference to whatever you want to compare with" —
   audience-level corpus statement of cycle 045's prefix-`&` rule
   applied to `cmp`'s second argument. The lesson's "the second
   argument is taken by shared reference, so the call site writes
   `&b`" rephrases this.
3. "It returns a variant of the `Ordering` enum" — audience-level
   corpus statement that the call's result type is `Ordering`. The
   lesson's "the whole call expression has type `Ordering`"
   rephrases this.

Calibration: the Book's example uses `guess` (a `String`) and
`secret_number` (an integer), which actually *fails to type-check*
in the Book (the Book later resolves it via `let guess: u32 =
guess.trim().parse().expect(...)` shadowing). Today's lesson uses
two `i32`-typed bindings so that the `cmp` call type-checks
directly without involving cycle 057's type-changing shadowing. The
deferred *FULL guessing-game program* future move is the natural
re-composition of today's cycle with the input loop from cycle 060.

The Book passage also cross-references chapter 6 for `match`
mechanics and notes that the `cmp` method "can be called on anything
that can be compared" — an audience-level pointer at the `Ord`
trait, which the lesson defers.

### `output/docs/rust/error_codes/E0308.md`

The error-code explainer for E0308 *mismatched types*. Already cited
in cycles 024, 025, 026, 028, 033, 045, 046, 047, 048. Reused here
for the broken-contrast probe. The lesson cites the E-code by family
("same E-code as cycles 045 and 046's broken contrasts") rather than
re-explaining E0308.

The page's first example (lines 11-15) — `plus_one("Not a number");`
against `fn plus_one(x: i32) -> i32` — is structurally identical to
today's broken probe (a method call with the wrong argument type),
with the type pair flipped: the explainer's example expects `i32`
and finds `&str`, while today's probe expects `&i32` and finds `i32`.
Same general E0308 *call-site argument type mismatch* sub-case.
Cycle 046's evidence already mapped this passage; today's re-citation
is for the *method-call* rather than *function-call* form, which
rustc's E0308 diagnostic naturally distinguishes ("arguments to this
method are incorrect" vs. cycle 046's "arguments to this function
are incorrect").

## Probes

### Working probe (Less)

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/061-i32-cmp-returns-ordering.rs`.
Identical source to the *The Move* code block.

Transcript, captured 2026-05-07 in a fresh `mktemp -d`:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
demo.rs
--- cat demo.rs ---
use std::cmp::Ordering;

fn main() {
    let a: i32 = 3;
    let b: i32 = 5;
    match a.cmp(&b) {
        Ordering::Less => println!("a < b"),
        Ordering::Greater => println!("a > b"),
        Ordering::Equal => println!("a == b"),
    }
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
a < b
exit=0
```

Notes:

- `rustc demo.rs` exits 0 silently (cycle 001).
- `./demo` prints exactly one line, `a < b`. The line is the body of
  the `Ordering::Less` arm of the `match`, so reaching it confirms
  empirically that `3.cmp(&5)` produced the `Ordering::Less` variant.
  The std page's convention sentence — `self.cmp(&other)` matches
  `self <op> other` — is corroborated: `3 < 5` is true, so
  `Ordering::Less` is the result.
- Only the working source is committed under `observations/`.

### Working probes (Equal and Greater)

Two additional working probes captured to demonstrate that all three
arms are reachable empirically. Source for each is identical to the
Less probe with the `a` and/or `b` literals changed. Not committed;
transcripts below are the artifacts. Captured 2026-05-07 in fresh
`mktemp -d` directories.

**Equal probe** — `a = 3, b = 3`:

```text
--- cat equal.rs ---
use std::cmp::Ordering;

fn main() {
    let a: i32 = 3;
    let b: i32 = 3;
    match a.cmp(&b) {
        Ordering::Less => println!("a < b"),
        Ordering::Greater => println!("a > b"),
        Ordering::Equal => println!("a == b"),
    }
}
--- rustc equal.rs ---
exit=0
--- ./equal ---
a == b
exit=0
```

**Greater probe** — `a = 7, b = 5`:

```text
--- cat greater.rs ---
use std::cmp::Ordering;

fn main() {
    let a: i32 = 7;
    let b: i32 = 5;
    match a.cmp(&b) {
        Ordering::Less => println!("a < b"),
        Ordering::Greater => println!("a > b"),
        Ordering::Equal => println!("a == b"),
    }
}
--- rustc greater.rs ---
exit=0
--- ./greater ---
a > b
exit=0
```

The three runs together corroborate the empirical core of the
lesson: each of `Ordering::Less`, `Ordering::Equal`, and
`Ordering::Greater` is reached by exactly the input pattern the
std page's convention sentence predicts. The three-way truth-table
is:

| inputs            | `a.cmp(&b)`         | arm body printed |
|-------------------|---------------------|-------------------|
| `a = 3, b = 5`    | `Ordering::Less`    | `a < b`           |
| `a = 3, b = 3`    | `Ordering::Equal`   | `a == b`          |
| `a = 7, b = 5`    | `Ordering::Greater` | `a > b`           |

### Broken-contrast probe

Source: same as the working probe with line 6 changed from
`match a.cmp(&b) {` to `match a.cmp(b) {` (the prefix-`&` removed
at the call site). Not committed; the transcript below is the
artifact. Captured 2026-05-07 in a fresh `mktemp -d` (filename
`broken.rs`):

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
broken.rs
--- cat broken.rs ---
use std::cmp::Ordering;

fn main() {
    let a: i32 = 3;
    let b: i32 = 5;
    match a.cmp(b) {
        Ordering::Less => println!("a < b"),
        Ordering::Greater => println!("a > b"),
        Ordering::Equal => println!("a == b"),
    }
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:6:17
  |
6 |     match a.cmp(b) {
  |             --- ^ expected `&i32`, found `i32`
  |             |
  |             arguments to this method are incorrect
  |
note: method defined here
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/cmp.rs:999:7
help: consider borrowing here
  |
6 |     match a.cmp(&b) {
  |                 +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Notes (probe evidence — not corpus quotation):

- The headline reads `error[E0308]: mismatched types` — same E-code
  as cycles 024, 025, 026, 028, 033, 045, 046, 047, 048. The
  `expected `&i32`, found `i32`` caret label is identical wording to
  cycles 045 and 046; the only structural difference is that the
  underline-and-label pair on the *receiver/method* side reads
  `arguments to this method are incorrect` (today's case) instead of
  `arguments to this function are incorrect` (cycle 046's case).
  rustc distinguishes method calls from free-function calls in this
  noun, paralleling the cycle-041 contrast captured for E0061
  (where the noun also shifts between `function` and `method`
  depending on the call form).
- The `--> broken.rs:6:17` location points at column 17 of line 6,
  which is the `b` argument inside `a.cmp(b)`. Same pinpoint shape
  as cycle 046's broken probe.
- The dual-`-->` pattern from cycle 036 / cycle 046 is present:
  `note: method defined here` followed by a second `-->` pointing
  into the standard-library source
  (`/rustc/59807616e.../library/core/src/cmp.rs:999:7`). This is
  rustc telling the reader where `cmp` is defined. The lesson body
  glosses this as "the dual-`-->` shape cycle 036 first captured."
  The `cmp.rs:999:7` line number is a `core::cmp::Ord::cmp` method
  declaration; the technical detail that `core` re-exports to `std`
  is not surfaced (cycle 051's evidence appendix made the same
  observation for `Ordering`'s declaration site).
- The `help:` block reads literally `help: consider borrowing here`,
  followed by a source-diff suggestion that re-prints line 6 with
  `&` inserted in front of `b`. *Identical* wording to cycles 045
  and 046's broken-contrast `help:` blocks (the same fix, suggested
  in three contexts: `let` annotation in cycle 045, free-function
  argument in cycle 046, method argument in cycle 061). rustc's
  runtime statement of the fix matches the lesson's intended
  teaching — the cleanest possible alignment.
- Exit code: 1. No executable was produced. The `ls after` shows
  only `broken.rs`, no `broken` binary.

The broken-contrast probe is necessary because the lesson makes a
contrastive claim ("with `&` it works, without it E0308 fires at the
call site"). The captured `expected `&i32`, found `i32`` caret
label, plus the `note: method defined here` second `-->`, are the
load-bearing pieces of probe evidence: rustc itself distinguishes
`&i32` from `i32` as different types when matching method arguments
to declared parameter types. The corpus-level grounding is the
combination of cycle 045's prefix-`&` rule, cycle 046's
parameter-type-match rule generalized to method arguments, and the
`Ord::cmp` signature `fn cmp(&self, other: &Self) -> Ordering` from
the trait page (which specializes to `&i32` for `i32`'s `Ord` impl).

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 061. Older supporting lessons are mentioned
above by number only.

- **Cycle 040 (load-bearing for the dot-form)** — installed
  `receiver.method(args)` as a call shape distinct from cycle 008's
  free-function form. Cycle 061 reuses this for `a.cmp(&b)`: `a` is
  the receiver, `cmp` is the method name, `(&b)` is the
  parenthesized argument list. The whole call is an expression
  that has a value (cycle 040 also reused cycle 021's
  expression-on-the-right-of-`let` rule); today's call serves as
  the scrutinee of a `match` instead.
- **Cycle 044 (load-bearing for `use std::cmp::Ordering;`)** —
  installed `use path::final;` to bring a name into scope. Cycle 051
  already extended the same `use` form to bring the type `Ordering`
  into scope; cycle 061 reuses cycle 051's `use std::cmp::Ordering;`
  unchanged. Without the `use` line, the lesson would write
  `std::cmp::Ordering::Less` etc. — same mechanic, longer surface.
- **Cycle 045 (load-bearing for the `&b` argument)** — installed (a)
  the *shared reference type* `&T`, (b) the prefix-`&` operator
  building a `&T` value, (c) the distinction `T` vs. `&T` and its
  E0308 *mismatched types* sub-case, (d) the `Display` "`{}` looks
  through `&T`" property. Cycle 061 uses (a)-(c) directly: the
  argument `&b` builds a `&i32` value from the `i32` binding `b`,
  matching the `&i32` parameter type that `i32::cmp`'s signature
  declares; passing bare `b` instead is the E0308 case. (Property
  (d) is not exercised today since the `&i32` is consumed by the
  method, not formatted.) Cycle 046 generalized cycle 045 to
  function parameters and is the immediate precedent for the
  argument-side rule, but is *not* separately load-bearing because
  the rule it added (call-site type must match parameter type) is
  identical for free functions and methods — cycle 045 already
  installs the rule; cycle 046 just installed the function-parameter
  surface, which by the same rule applies to method parameters
  unchanged. The lesson body's broken-contrast walk cites cycles
  045 and 046 together for this reason; the graph dependency is
  cycle 045 alone (per orchestrator note: "046 is transitively
  involved but not separately load-bearing here").
- **Cycle 051 (load-bearing for `Ordering` and the three-variant
  match)** — installed (a) the `Ordering` enum and its three
  variants `Less` / `Greater` / `Equal`, (b) the qualified-path
  `Ordering::Variant` shape, (c) the `match scrutinee { Variant
  => ..., ... }` form on an enum scrutinee with exhaustiveness via
  E0004. Cycle 061 reuses (a)-(c) unchanged; the only structural
  difference is that cycle 051's scrutinee was a hardcoded
  `Ordering::Less` value, while today's scrutinee is the call
  expression `a.cmp(&b)` of type `Ordering`. The match machinery
  itself does not change.
- **Cycle 019 (load-bearing for the `: i32` annotation)** —
  installed `let name: TYPE = value;` with `i32` as the example
  `TYPE`. Cycle 061 uses two `let a: i32 = ...;` and `let b: i32 =
  ...;` annotations. The annotations are not strictly necessary
  (rustc would infer `i32` from the integer literals), but the
  lesson writes them out so the lesson body's "two `i32` values"
  framing is visible in the source.
- **Cycles 001, 002, 005** — `rustc file.rs` then `./name`; `fn
  main` is the entry point; `let name = value;` plus the named-
  placeholder `{name}` form for `println!`. Used unchanged.

## Older supporting lessons

- Cycle 003 (rustc diagnostic shape — headline + `-->` + source
  excerpt with caret + optional `note:` / `help:` lines). The
  broken-contrast walk uses cycle 003's map without re-teaching it.
- Cycle 008 (free-function call form). Not exercised today; the
  call shape is the dot form throughout.
- Cycle 013 (comparison operators `<`, `==`, `>` returning `bool`).
  Today's *Mental Model Delta* and *What Changed* contrast `cmp`
  against the comparison operators as two-way-vs-three-way; cycle
  013 is the reference for the two-way side, but is not
  load-bearing for any new fact today (no `<` or `==` appears in
  the working probe; the convention sentence from `Ord`'s page
  ties the two together verbally).
- Cycle 036 (dual-`-->` pattern). The broken-contrast probe's
  `note: method defined here` is the dual-`-->` form cycle 036
  first observed for E0061 arity errors and cycle 046 for E0308
  function-argument errors.
- Cycle 042 (qualified-path `Type::name` shape). Cycle 051
  already extended this to `Ordering::Variant`; today reuses cycle
  051 unchanged.
- Cycle 043 (nested module path `std::cmp::name`). Today's
  `std::cmp::Ordering` is the same three-segment path; mentioned
  only in *What To Ignore For Now* (where `std::cmp::min` and
  `std::cmp::max` are listed as siblings).
- Cycle 046 (reference function parameter — `fn show(r: &i32)`
  with call-site `show(&n)`). The function-side rule that lets a
  *parameter* slot accept `&i32` and a *call* site supply `&n` is
  what makes the broken-contrast probe diagnose the way it does.
  Not separately listed in `depends_on` per the orchestrator note,
  but mentioned in the lesson body's broken-contrast walk for the
  audit trail.
- Cycles 024, 025, 026, 028, 033, 047, 048 (the wider E0308 family
  — different sub-cases of "expected type X, found type Y").
  Today's broken-contrast probe fires E0308 with a method-argument
  sub-case; cited only by family.
