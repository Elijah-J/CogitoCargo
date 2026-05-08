# Evidence — 051-ordering-enum-and-variant-match

Audit appendix for `lessons/051-ordering-enum-and-variant-match.md`.
Holds the corpus-quote map, the toolchain string, the full working and
broken-contrast probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/051-ordering-enum-and-variant-match.rs`); the broken-
  contrast `.rs` is not committed — its transcript below is the
  artifact.

## Sources

### `output/docs/rust/std/cmp/enum.Ordering.md`

The std-library page for `Ordering`. Primary source. Three load-bearing
spans.

Lines 6-14 (the type's canonical declaration):

> ```
> #[repr(i8)]
>
> pub enum Ordering {
>     Less = -1,
>     Equal = 0,
>     Greater = 1,
> }
> ```

This is the corpus statement that (a) `Ordering` is declared with the
`enum` keyword, (b) it has exactly three variants named `Less`,
`Equal`, and `Greater`, and (c) those variants are unit-like (no
parenthesized payload, no struct-like braces — just bare names with
`=` discriminant assignments). The lesson surfaces (a) by naming
`Ordering` "the standard library's three-variant enum" and listing
the variant names; (b) directly; (c) by saying "each variant has no
payload — each is just a name." The `#[repr(i8)]` attribute and the
`= -1`, `= 0`, `= 1` discriminants are explicitly deferred under *What
To Ignore For Now*.

Line 18 (the type's plain-English description):

> An `Ordering` is the result of a comparison between two values.

Audience-level statement of what `Ordering` *is for*. The lesson body's
"the standard library's three-variant enum for comparison results"
rephrases this. The lesson does not surface the *cmp* method that
*produces* `Ordering` values from comparisons — today's probe
hardcodes `Ordering::Less` directly.

Lines 22-30 (the canonical Examples block):

> ```
> use std::cmp::Ordering;
>
> assert_eq!(1.cmp(&2), Ordering::Less);
>
> assert_eq!(1.cmp(&1), Ordering::Equal);
>
> assert_eq!(2.cmp(&1), Ordering::Greater);
> ```

Direct corpus precedent for *both* (a) the `use std::cmp::Ordering;`
import line the lesson uses, and (b) the qualified-path variant-name
form `Ordering::Less` / `Ordering::Equal` / `Ordering::Greater` as
*values*. The corpus example calls `.cmp(&n)` to *produce* an
`Ordering` value and `assert_eq!` to compare; the lesson uses the
simpler "hardcode the variant" form `let direction: Ordering =
Ordering::Less;`. Both forms produce a value of type `Ordering`. The
`.cmp(&n)` method (and the `Ord` trait it lives on) is explicitly
deferred — *What To Ignore For Now* lists it.

Lines 32-50 (the Variants section headers):

> ## Variants
> ### Less = -1
> An ordering where a compared value is less than another.
> ### Equal = 0
> An ordering where a compared value is equal to another.
> ### Greater = 1
> An ordering where a compared value is greater than another.

Corpus statement of what each variant *means semantically*. The lesson
surfaces this as "less / greater / equal" labels in the working probe's
arm values, deliberately matching the corpus's per-variant gloss
without quoting them.

Calibration: the page goes on to document many methods on `Ordering`
(`is_eq`, `is_ne`, `is_lt`, `is_gt`, `is_le`, `is_ge`, `reverse`,
`then`, `then_with`) and a long list of trait implementations (`Clone`,
`Debug`, `Hash`, `Ord`, `PartialEq`, `PartialOrd`, `Copy`, `Eq`, ...).
*All* of these are explicitly deferred in the lesson's *What To Ignore
For Now*. The lesson treats `Ordering` as a three-variant enum whose
variants can be (i) bound by `let`, (ii) used as values on the right
of `=`, and (iii) used as patterns in `match`. Nothing else.

### `output/docs/rust/reference/items/enumerations.md`

The Reference page for enum items. Primary corpus source for the
*enum* and *variant* nouns. Three load-bearing spans.

Lines 25-27 (the canonical *enumeration* definition):

> An *enumeration*, also referred to as an *enum*, is a simultaneous
> definition of a nominal [enumerated type](../types/enum.md) as well
> as a set of *constructors*, that can be used to create or
> pattern-match values of the corresponding enumerated type.

This is the canonical Reference-level statement that *enum* is the
short name for an enumerated type and that an enum's "constructors"
(its variants, when unit-like) can both *create values* and be used in
*pattern matching*. The lesson uses both surfaces — `Ordering::Less`
as a value-creating expression on the right of `=`, and
`Ordering::Less` as a pattern in a `match` arm — but rephrases the
mechanic in audience-level prose: "a type whose values come from a
fixed list of named alternatives." The Reference's term *constructors*
is not installed; the lesson uses *variants* instead, which the
Reference also uses (next quote).

Lines 29-31 (the corpus license for the *enum* keyword):

> Enumerations are declared with the keyword `enum`.

The lesson does not declare a new enum (the move uses the
standard-library `Ordering` directly), but the *What To Ignore For
Now* section names "*Defining your own enum* with the `enum` keyword"
as a deferred future move; this corpus line is the precedent for that
deferral.

Lines 39-50 (the canonical example):

> ```rust
> #![allow(unused)]
> fn main() {
> enum Animal {
>     Dog,
>     Cat,
> }
>
> let mut a: Animal = Animal::Dog;
> a = Animal::Cat;
> }
> ```

Direct corpus precedent for (a) the `let a: Animal = Animal::Dog;`
shape — type annotation `: Animal` plus value-on-right `Animal::Dog`
— and (b) the `Type::Variant` qualified-path form for naming a
variant. The lesson's `let direction: Ordering = Ordering::Less;` is
the same shape with `Ordering` for `Animal` and `Less` for `Dog`.
Calibration: the Reference's example is on a *user-defined* enum;
`Ordering` is the same shape on a *standard-library* enum, with the
declaration site outside the user's source file.

Lines 71-83 (the *field-less enum* and *unit variant* terms):

> An enum where no constructors contain fields is called a *field-less
> enum*. ... If a field-less enum only contains unit variants, the
> enum is called an *unit-only enum*.

The Reference's terminology for `Ordering`'s shape: each variant has
no fields, so `Ordering` is a *field-less enum*; its variants are
*unit-like* (no payload). The lesson's "no payload — each is just a
name" rephrases the Reference's "unit variant" gloss without
installing the formal term.

Calibration: the Reference's main example uses a two-variant `Animal`
enum; this lesson uses the three-variant `Ordering` enum. The
mechanics are identical. The Reference's *Discriminants* subsection
(lines 139+) explicitly documents the `= -1`, `= 0`, `= 1`
discriminants that appear on `Ordering`'s page; this is one of the
*deferred* topics in *What To Ignore For Now*.

### `output/docs/rust/reference/patterns.md`

The Reference page for patterns. Already cited in lessons 030, 031 for
literal patterns and the wildcard. Reused here for the *path pattern*
clause that licenses variant-name patterns. Two load-bearing spans.

Lines 1146-1175 (the *Path patterns* subsection):

> ## Path patterns
>
> *Path patterns* are patterns that refer either to constant values
> or to structs or enum variants that have no fields.
>
> Unqualified path patterns can refer to:
>
> - enum variants
> - structs
> - constants
> - associated constants
>
> ...
>
> Path patterns are irrefutable when they refer to structs or an enum
> variant when the enum has only one variant or a constant whose type
> is irrefutable. They are refutable when they refer to refutable
> constants or enum variants for enums with multiple variants.

This is the canonical corpus license for using `Ordering::Less` /
`Ordering::Greater` / `Ordering::Equal` as *patterns* in a `match`
arm. Three substantive claims rest on this:

1. "Variant patterns work the same way `true`/`false` did" — the
   Reference's *Path patterns* subsection groups variant patterns with
   constants and unit structs as `PathPattern → PathExpression` (line
   1154), parallel to the *Literal Patterns* subsection lessons 030
   and 031 used.
2. "Each `Ordering` variant pattern is *refutable*, so multiple arms
   are needed for exhaustiveness" — the Reference's last quoted
   sentence says enum-variant patterns for enums with multiple
   variants are refutable. With three variants, no single
   variant-pattern is irrefutable on its own, so a `match` needs all
   three (or a wildcard).
3. "Variants with no fields are reachable directly as patterns" — the
   first quoted sentence licenses path patterns "to enum variants
   that have no fields." `Ordering`'s three variants have no fields
   (per `enumerations.md` lines 71-83 above), so they are reachable
   directly as path patterns.

Calibration: the Reference's *Path patterns* section also covers
*qualified* path patterns (line 1169-1171: "Qualified path patterns
can only refer to associated constants"), which is unrelated to the
lesson's surface — `Ordering::Less` is *unqualified* in the
Reference's grammar sense (no `<T>::method()` shape), even though it
is *path-prefixed* in the audience-level sense.

### `output/docs/rust/error_codes/E0004.md`

The error-code explainer for E0004 *non-exhaustive patterns*. Already
cited in lessons 030 and 031. Reused here for the broken-contrast
probe. Lines 4-7:

> This error indicates that the compiler cannot guarantee a matching
> pattern for one or more possible inputs to a match expression.
> Guaranteed matches are required in order to assign values to match
> expressions, or alternatively, determine the flow of execution.

The lesson does not re-explain E0004; it cites lessons 030 and 031 by
number. The new corpus-level fact for cycle 051 is structural rather
than textual: for an enum scrutinee, "every possible input" is
"every variant the enum declares." The broken-contrast probe's
headline names exactly that — the missing variant `Equal` — confirming
the structural reading.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book chapter that introduces the guessing game. Already cited in
lessons 042 (for `String::new`) and 050 (for `std::io::stdin`). Reused
here for the audience-level introduction of `Ordering`. One
load-bearing span.

Lines 788-800:

> First, we add another `use` statement, bringing a type called
> `std::cmp::Ordering` into scope from the standard library. The
> `Ordering` type is another enum and has the variants `Less`,
> `Greater`, and `Equal`. These are the three outcomes that are
> possible when you compare two values.
>
> Then, we add five new lines at the bottom that use the `Ordering`
> type. The `cmp` method compares two values and can be called on
> anything that can be compared. ... Then, it returns a variant of the
> `Ordering` enum we brought into scope with the `use` statement. We
> use a [`match`](ch06-02-match.md) expression to decide what to do
> next based on which variant of `Ordering` was returned from the
> call to `cmp` ...

Audience-level corpus statement of *exactly* the lesson's main
concept: (a) `Ordering` is an enum from the standard library,
(b) brought into scope by `use std::cmp::Ordering;`, (c) with three
variants `Less`, `Greater`, and `Equal`, (d) usable as the patterns of
a `match` to decide what to do based on which variant. The lesson's
audience-level prose mirrors this passage's framing without quoting.
The Book passage also names the `cmp` method as the source of
`Ordering` values; the lesson defers `cmp` and uses a hardcoded
`Ordering::Less` instead.

Lines 778-783 (the canonical match-on-Ordering form, listed for
audit-trail completeness):

> ```rust
> match guess.cmp(&secret_number) {
>     Ordering::Less => println!("Too small!"),
>     Ordering::Greater => println!("Too big!"),
>     Ordering::Equal => println!("You win!"),
> }
> ```

Direct corpus precedent for the three-arm `match` shape `Ordering::Less
=> ..., Ordering::Greater => ..., Ordering::Equal => ...`. The Book
form's arms call `println!` and produce `()`; the lesson's arms produce
`&str` values bound by `let label = ...;`. The arm shape is identical
to lesson 030's `pattern => arm_expression` rule.

Calibration: the Book introduces this in chapter 2 (the guessing
game), but its full mechanic — `cmp(&secret_number)` returning an
`Ordering` value — is not yet installable because `Ord` and references
are deferred. The lesson uses the simpler hardcoded form so that
*only* the enum-and-variant-match concept is being installed.

### `output/docs/rust/book/ch06-01-defining-an-enum.md`

The Book chapter that defines enums. Cited for the audience-level
introduction of *enum* and *variant*. One load-bearing span.

Lines 23-32 plus 68-70:

> We can express this concept in code by defining an `IpAddrKind`
> enumeration and listing the possible kinds an IP address can be,
> `V4` and `V6`. These are the variants of the enum:
>
> ```rust
> enum IpAddrKind {
>     V4,
>     V6,
> }
>
> fn main() {
>     let four = IpAddrKind::V4;
>     let six = IpAddrKind::V6;
>     ...
> }
> ```
>
> ...
>
> Note that the variants of the enum are namespaced under its
> identifier, and we use a double colon to separate the two. This is
> useful because now both values `IpAddrKind::V4` and `IpAddrKind::V6`
> are of the same type: `IpAddrKind`.

Audience-level corpus statements of:

1. "These are the variants of the enum" — direct gloss of *variants*,
   which the lesson uses unchanged.
2. "the variants of the enum are namespaced under its identifier, and
   we use a double colon to separate the two" — exactly the lesson's
   "the qualified path `Ordering::Less` — same `::` separator as
   `String::new()`, except the right-hand side names a variant rather
   than a function" claim. The Book uses *namespaced under its
   identifier* for the same idea the lesson presents as "the qualified
   path `Type::Variant`."
3. "both values `IpAddrKind::V4` and `IpAddrKind::V6` are of the same
   type: `IpAddrKind`" — the corpus statement that distinct variants
   share the enum type. The lesson surfaces this as "values of type
   `Ordering` ... that specifically is the `Less` alternative" — all
   three variants are `Ordering`-typed values.

Calibration: the Book introduces enums via *user-defined*
`IpAddrKind`; the lesson uses the *standard-library* `Ordering`. The
mechanic is identical; the lesson defers user-defined enums to a
future move.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/051-ordering-enum-and-variant-match.rs`.
Identical source to the *Try It* block.

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
    let direction: Ordering = Ordering::Less;
    let label = match direction {
        Ordering::Less => "less",
        Ordering::Greater => "greater",
        Ordering::Equal => "equal",
    };
    println!("direction = {label}");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
direction = less
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 silently. No warnings.
- `./demo` prints exactly one line: `direction = less`. This is the
  load-bearing observation: `direction` was bound to `Ordering::Less`,
  the `match` selected the `Ordering::Less` arm (not `Greater` or
  `Equal`), the arm value `"less"` became the value of the whole
  `match`, and `let label = ...;` bound `label` to `"less"`. The
  `println!` then printed `"less"` for the `{label}` slot.
- The annotation `: Ordering` on the binding is the lesson-019 shape
  with a new `TYPE`. rustc would also infer `Ordering` here from the
  right-hand side, but the annotation makes the type-check
  verifiable.
- `let label = ...;` (no annotation) lets rustc infer `label`'s type
  as `&'static str` (or, in the audience-level prose, just *the type
  of string literals*). Per the orchestrator's prompt, naming `&str`
  here would bundle a side-introduction that is unnecessary for the
  main concept; the lesson uses inference instead and lists `&str`
  under *What To Ignore For Now*.
- The trailing `,` after `Ordering::Equal => "equal",` is allowed
  (the grammar from lesson 030 says the final arm's `,` is optional).
  The probe writes it for consistency.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe

Source: same as the working probe with the `Ordering::Equal` arm
removed and the `let label = ...;` binding changed to
`let _label = ...;` (so the now-unused binding does not also fire an
unused-variable warning when the program would otherwise compile).
Not committed; the transcript below is the artifact. Captured
2026-05-07 in a fresh `mktemp -d` (filename `broken.rs`):

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
    let direction: Ordering = Ordering::Less;
    let _label = match direction {
        Ordering::Less => "less",
        Ordering::Greater => "greater",
    };
}
--- rustc broken.rs (capturing stderr) ---
error[E0004]: non-exhaustive patterns: `std::cmp::Ordering::Equal` not covered
 --> broken.rs:5:24
  |
5 |     let _label = match direction {
  |                        ^^^^^^^^^ pattern `std::cmp::Ordering::Equal` not covered
  |
note: `std::cmp::Ordering` defined here
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/cmp.rs:404:0
 ::: /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/cmp.rs:410:4
  |
  = note: not covered
  = note: the matched value is of type `std::cmp::Ordering`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
7 ~         Ordering::Greater => "greater",
8 ~         std::cmp::Ordering::Equal => todo!(),
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
exit=1
--- ls after ---
broken.rs
```

Notes (probe evidence — not corpus quotation):

- The headline reads
  `error[E0004]: non-exhaustive patterns: \`std::cmp::Ordering::Equal\` not covered`.
  Same E-code lessons 030 and 031 introduced. The headline names the
  missing pattern as the variant `Equal`, written using the *full
  path* `std::cmp::Ordering::Equal` even though the source had used
  the shorter `use`-shadowed form `Ordering::Equal`. This is honest
  probe evidence, captured here so the lesson body can describe the
  diagnostic accurately. rustc's behavior — preferring fully-qualified
  paths in diagnostics — does not change the load-bearing fact (the
  *variant* `Equal` is not covered).
- The `--> broken.rs:5:24` location points at column 24 of line 5,
  which is the scrutinee `direction`. Same pattern as lessons 030
  and 031: rustc enforces exhaustiveness against the *scrutinee's
  type*, so the location is the scrutinee.
- The source excerpt underlines `direction` with `^^^^^^^^^` and
  adds the sub-line `pattern \`std::cmp::Ordering::Equal\` not
  covered` — naming exactly which variant is missing.
- A *second* `note:` appears: `\`std::cmp::Ordering\` defined here`,
  with a `-->` pointing into the standard-library source
  (`/rustc/.../core/src/cmp.rs:404:0`). This is rustc telling the
  reader where the enum's *declaration* lives. The lesson body
  glosses this as "(A second `note:` points at the standard-library
  source where `Ordering` is defined; safe to ignore.)" The path
  `/rustc/<sha>/library/core/src/cmp.rs` is *not* installed —
  technically `Ordering` is re-exported from `core::cmp` to
  `std::cmp`, which is more than the lesson needs to surface.
- The `= note: the matched value is of type \`std::cmp::Ordering\``
  line names the scrutinee's type. rustc tells the learner *what
  type* must be covered, not just that something is missing. For
  `Ordering`, the three variants to cover are `Less`, `Greater`, and
  `Equal`; the working probe lists all three.
- The `help:` block shows a literal source-diff with `~` markers
  (modified-line markers, same shape lessons 030 and 031 captured).
  The diff suggests `std::cmp::Ordering::Equal => todo!(),` as the
  new arm. `todo!()` is rustc's placeholder macro (carried-forward
  gloss from lessons 030 and 031). The real fix is the working
  probe's `Ordering::Equal => "equal",`.
- Exit code: 1. No executable was produced.
- *Calibration with lessons 030 and 031*: lesson 030's E0004 named
  the missing pattern as `false` (a literal value); lesson 031's
  E0004 named the missing patterns as ranges
  (`i32::MIN..=0_i32 \| 4_i32..=i32::MAX`). Today's E0004 names the
  missing pattern as a variant name (`std::cmp::Ordering::Equal`).
  Same E-code, same diagnostic structure, three different shapes of
  missing-pattern label — confirming that exhaustiveness is checked
  against whatever the scrutinee's type considers a "complete" set
  of values.

The broken-contrast probe is necessary because the lesson makes a
contrastive claim ("with all three arms it works, omitting one
fails"). The captured E0004 — naming the *variant* that is missing —
is the load-bearing piece of probe evidence corroborating the central
new fact: exhaustiveness on an enum scrutinee means every variant
must be covered.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 051. Older supporting lessons are mentioned
above by number only.

- **Lesson 030 (load-bearing)** — installed the whole `match` form:
  scrutinee, arms `pattern => arm_expression`, separated by `,`, the
  matching arm's expression as the whole `match`'s value, the
  arms-must-share-type rule, exhaustiveness, and E0004. Lesson 051
  reuses every one of those without re-deriving them, and extends the
  scrutinee from `bool` (lesson 030) and `i32` (lesson 031) to
  `Ordering`. The new fact for 051 is what *patterns* look like for
  an enum scrutinee — `Ordering::Less` instead of `true`/`false` or
  `1`/`2`/`3`/`_`.
- **Lesson 031** — generalized lesson 030's `match` from `bool` to
  `i32` and installed the `_` wildcard for unbounded scrutinee types.
  Lesson 051 generalizes once more, to `Ordering`, but uses a
  *different* exhaustiveness strategy: with exactly three variants,
  the lesson lists all three explicitly rather than using `_`. The
  contrast is captured in the lesson's *Mental Model Delta* and in
  the *Prerequisites* bullet for lesson 031.
- **Lesson 042 (load-bearing)** — installed the qualified path
  `Type::name` with `::` as separator, where the right-hand side was
  a *function name* (`String::new`). Lesson 051 extends the same
  shape to a different right-hand side — a *variant name*
  (`Ordering::Less`). The Reference's path-pattern grammar
  (`PathPattern → PathExpression`) treats both kinds of right-hand
  side as path expressions; the audience-level distinction is "what
  the right-hand side names" — function vs. variant. Lesson 051's
  *What Changed* bullet 3 explicitly draws the contrast.
- **Lesson 043** — installed the nested-module-path shape
  `module::submodule::name(args)` with `std::cmp::min` as the
  example. Lesson 051's `std::cmp::Ordering` reuses the same
  three-segment path, but the final segment names a *type* rather
  than a *function*. The path-grammar mechanic is identical (per
  lesson 043's already-cited
  `output/docs/rust/reference/paths.md` grammar production
  `PathInExpression → PathExprSegment ( :: PathExprSegment )*`).
- **Lesson 044** — installed `use std::cmp::min;` at the top of a
  file, bringing the final-segment name into scope. Lesson 051 reuses
  the same form with a different payload: `use std::cmp::Ordering;`
  brings the *type name* `Ordering` into scope (not a function name).
  The Reference's *use declarations* grammar accepts any path,
  including paths that resolve to types. The lesson body's claim
  that `use std::cmp::Ordering;` is "lesson 044's shape" rests on
  this — the form is the same, only the kind of imported name
  differs.
- **Lesson 019 (load-bearing for shape)** — installed the
  type-annotation form `let name: TYPE = value;` with `i32` as the
  example `TYPE`. Lesson 042 already extended this to `String`.
  Lesson 051 extends it once more to `Ordering`. Three different
  `TYPE`s, same lesson-019 slot.
- **Lesson 003 (load-bearing)** — diagnostics have headline + `-->`
  location + source excerpt with caret + optional `note:` / `help:`
  sub-lines. Lesson 051's broken-contrast walk uses that map without
  re-teaching it. The specific observation that this E0004 has *two*
  `note:` lines (one naming the type, one pointing to the type's
  declaration in `core/src/cmp.rs`) is consistent with lesson 003's
  framing — the diagnostic's `note:` lines are optional and can
  appear multiple times.
- **Lessons 001, 002, 005** — `rustc file.rs` then `./name`; `fn
  main` is the entry point; `let name = value;` plus the named-
  placeholder `{name}` form for `println!`. Used unchanged today.

## Older supporting lessons

- Lesson 008 (free-function call form `name(args)` — the call shape
  underlying the *variant-name-as-value* construction; not load-
  bearing for cycle 051 because no function call is on the right of
  `=` in the working probe — the right-hand side is the variant value
  `Ordering::Less` directly).
- Lesson 040 (the dot-form method-call grammar — not used today).
- Lesson 041 (the qualified method-call form — superseded by lesson
  042's no-receiver case for cycle 051's purposes).
- Lessons 005, 008, 040, 042, 043, 044 (the E0425 "cannot find name"
  family — not exercised today; today's broken contrast is E0004,
  not E0425).
