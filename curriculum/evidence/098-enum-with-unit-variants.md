# Evidence — 098-enum-with-unit-variants

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -srvm` -> `Darwin 24.5.0 Darwin Kernel Version 24.5.0:
  Tue Apr 22 19:53:26 PDT 2025; root:xnu-11417.121.6~2/RELEASE_X86_64
  x86_64`
- Probes run in `/tmp/eduratchet098/` on this host. Same toolchain
  family as recent accepted lessons (082-097).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/098-enum-with-unit-variants.rs`
is the working three-piece probe verbatim, with header comments naming
the expected output and the contrast probes captured below.

## Sources

### `output/docs/rust/book/ch06-01-defining-an-enum.md`

The Book's *Defining an Enum* chapter. Three load-bearing passages:

#### Lines 4-8 — what an enum is, contrasted with a struct

> Where structs give you a way of grouping together related fields and
> data, like a `Rectangle` with its `width` and `height`, enums give
> you a way of saying a value is one of a possible set of values.

Corpus warrant for the lesson's *Mental Model Delta* "After" framing
and for the load-bearing adjacent-but-distinct claim "a struct groups
several pieces of data into one value; an enum lists alternatives,
each value being exactly one of them." The Book itself frames the two
as parallel data-type vocabulary.

#### Lines 13-26 — variants as enumerated alternatives, the declaration shape

> Because these are the only possibilities for an IP address that our
> program will come across, we can *enumerate* all possible variants,
> which is where enumeration gets its name. ... we can express this
> concept in code by defining an `IpAddrKind` enumeration and listing
> the possible kinds an IP address can be, `V4` and `V6`. These are
> the variants of the enum:
>
> ```rust
> enum IpAddrKind {
>     V4,
>     V6,
> }
> ```

Corpus warrant for the lesson's piece (1) framing — *Declare*: the
`enum` keyword, the type name, curly braces, comma-separated variant
identifiers. The Book's `enum IpAddrKind { V4, V6 }` is the direct
shape model for today's `enum Sign { Positive, Negative }`.

#### Lines 47-71 — constructing variant values via `Type::Variant`

> We can create instances of each of the two variants of `IpAddrKind`
> like this:
>
> ```rust
> let four = IpAddrKind::V4;
> let six = IpAddrKind::V6;
> ```
>
> ... Note that the variants of the enum are namespaced under its
> identifier, and we use a double colon to separate the two. This is
> useful because now both values `IpAddrKind::V4` and `IpAddrKind::V6`
> are of the same type: `IpAddrKind`.

Corpus warrant for the lesson's piece (2) framing — *Construct*: the
path `Type::Variant` produces a value of the enum type. The Book
explicitly names the namespacing rule and the `::` separator. Today's
`Sign::Positive` and `Sign::Negative` are direct instances of the
same shape, with the lesson-043 path-syntax precedent already
installed for the audience.

### `output/docs/rust/reference/items/enumerations.md`

The Reference's *Enumerations* item. Lines 10-17 carry the formal
grammar; lines 27-50 carry the canonical example.

#### Lines 10-17 — the formal grammar

> Enumeration → enum IDENTIFIER GenericParams? WhereClause? { EnumVariants? }
>
> EnumVariants → EnumVariant ( , EnumVariant )* ,?
>
> EnumVariant →
>     OuterAttribute* Visibility?
>     IDENTIFIER ( EnumVariantTuple | EnumVariantStruct )? EnumVariantDiscriminant?

Reference warrant: the declaration grammar admits a brace-enclosed
list of variant identifiers separated by commas, with optional
trailing comma. Generic params, where clauses, outer attributes,
visibility modifiers, tuple-form variants, struct-form variants, and
explicit discriminants are all optional in the grammar — and all
deferred by today's lesson.

#### Lines 27-31 — the constructor framing

> An *enumeration*, also referred to as an *enum*, is a simultaneous
> definition of a nominal enumerated type as well as a set of
> *constructors*, that can be used to create or pattern-match values
> of the corresponding enumerated type.
>
> Enumerations are declared with the keyword `enum`.

Reference warrant for the centered claim that the `enum` declaration
introduces both a type and the variants used to construct values of
that type. The phrase "create or pattern-match" justifies the
lesson's bundling of construction and match into one move.

#### Lines 41-50 — the canonical unit-only example

> ```rust
> enum Animal {
>     Dog,
>     Cat,
> }
>
> let mut a: Animal = Animal::Dog;
> a = Animal::Cat;
> ```

This is the closest-fit corpus exemplar to today's probe shape — the
Reference's own two-variant example with the same `Type::Variant`
construction shape. Today's probe uses `Sign` / `Positive` / `Negative`
in place of `Animal` / `Dog` / `Cat` and adds a `match` to make the
third piece (matching) visible, replacing the Reference's `mut`
reassignment with two separate `let` bindings to keep `mut` out of
this lesson's scope.

#### Lines 86-88 — unit-only enum terminology

> If a field-less enum only contains unit variants, the enum is called
> an *unit-only enum*.

Reference warrant for the lesson's title and *What To Ignore*
list — today's `enum Sign { Positive, Negative }` is a unit-only enum
in the Reference's terminology. Tuple-like and struct-like variants
(grammar at lines 19, 21) are explicitly named as deferred.

### `output/docs/rust/error_codes/E0599.md`

The error code page for E0599 — the bad-variant error today's
centered contrast probe witnesses:

> This error occurs when a method is used on a type which doesn't
> implement it:
>
> ```rust
> struct Mouth;
> let x = Mouth;
> x.chocolate(); // error: no method named `chocolate` found for type
>                //        `Mouth` in the current scope
> ```

The corpus page focuses on the missing-method case, but rustc's
diagnostic message ("no variant or associated item named ...") is the
exact wording today's probe captured for the *missing-variant* case.
The lesson uses E0599 for the variant case; the error code itself
covers both surfaces — methods on types and variants/items on enums
— under the same E-code. The corpus page does not need to document
the variant-case wording explicitly because the same code is reused
across the family of "name not found in this scope" cases on a
typed receiver.

### `output/docs/rust/error_codes/E0004.md`

The error code page for E0004 — the non-exhaustive-patterns error
today's secondary contrast probe witnesses:

> This error indicates that the compiler cannot guarantee a matching
> pattern for one or more possible inputs to a match expression.
> ...
>
> ```rust
> enum Terminator {
>     HastaLaVistaBaby,
>     TalkToMyHand,
> }
>
> let x = Terminator::HastaLaVistaBaby;
>
> match x { // error: non-exhaustive patterns: `HastaLaVistaBaby` not covered
>     Terminator::TalkToMyHand => {}
> }
> ```

Direct corpus warrant: the lesson's E0004 secondary contrast probe
has the same shape as the corpus erroneous-code example. The corpus
example uses two unit variants in a match and removes one arm,
exactly mirroring today's probe shape with `Sign` / `Positive` /
`Negative` in place of `Terminator` / `HastaLaVistaBaby` /
`TalkToMyHand`. Both name the same E-code and the same kind of
missing variant. E0004 was first installed in lesson 030 and has
appeared in every subsequent match-related lesson (031, 051, 058);
today's secondary contrast adds it on a *user-declared* enum but
the rule is unchanged.

## Probes

### Probe 1 — working: declare, construct, match

Source (`/tmp/eduratchet098/demo.rs`, also at
`observations/098-enum-with-unit-variants.rs`):

```rust
enum Sign {
    Positive,
    Negative,
}

fn main() {
    let up = Sign::Positive;
    let down = Sign::Negative;
    let label_up = match up {
        Sign::Positive => "+",
        Sign::Negative => "-",
    };
    let label_down = match down {
        Sign::Positive => "+",
        Sign::Negative => "-",
    };
    println!("up = {label_up}, down = {label_down}");
}
```

Compile transcript:

```
$ rustc demo.rs
(no output; exit 0)
$ ls
demo  demo.rs
$ ./demo
up = +, down = -
(exit 0)
```

Witnesses:

- **Declaration** is accepted: `enum Sign { Positive, Negative }` at
  module scope is a valid enum item. `rustc` is silent on success,
  consistent with lesson 001.
- **Construction** is accepted: both `Sign::Positive` and
  `Sign::Negative` produce values of type `Sign`, and `let up = ...;`
  / `let down = ...;` bind them (lesson 005's binding form,
  unmodified).
- **Match** is accepted: each `match` walks its scrutinee against
  the two variant patterns in order, the matching arm's `&str`
  becomes the value of the whole `match`, and `let label_X = ...;`
  binds it. The output line `up = +, down = -` is the load-bearing
  observation that all three pieces composed: `Sign::Positive`
  produced the `&str` `"+"` through the match; `Sign::Negative`
  produced `"-"`. Both arms in each match are consulted (one for
  each scrutinee value), so the lesson's claim "the matching arm's
  expression is the match's value" is witnessed both ways.
- All three pieces compile under one `rustc` invocation. Enums are
  *items* like `fn` and `struct`, parsed and type-checked alongside
  the rest of the file.
- Both variants are constructed exactly so that `dead_code`
  (warn-by-default) does not fire on `Sign::Negative`. An earlier
  draft of the probe constructed only `Sign::Positive` and produced
  the warning `variant \`Negative\` is never constructed` with
  `note: \`#[warn(dead_code)]\` (part of \`#[warn(unused)]\`) on by
  default`. The committed probe avoids the warning to keep the
  lesson focused on the three-piece move; lesson 069 already named
  the warn-by-default category and lesson 094 named one specific
  warn lint, but a third lint is not centered today. The tradeoff
  is one `let` and one `match` more than strictly required for the
  three pieces, mirroring lesson 058's two-`match` shape on two
  scrutinees.

### Probe 2 — centered contrast: bad variant name (E0599)

Source (`/tmp/eduratchet098/bad_variant.rs`):

```rust
enum Sign {
    Positive,
    Negative,
}

fn main() {
    let mystery = Sign::Maybe;
    let label = match mystery {
        Sign::Positive => "+",
        Sign::Negative => "-",
    };
    println!("mystery = {label}");
}
```

Compile transcript:

```
$ rustc bad_variant.rs
error[E0599]: no variant or associated item named `Maybe` found for enum `Sign` in the current scope
 --> bad_variant.rs:7:25
  |
1 | enum Sign {
  | --------- variant or associated item `Maybe` not found for this enum
...
7 |     let mystery = Sign::Maybe;
  |                         ^^^^^ variant or associated item not found in `Sign`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
(exit 1)
```

Witnesses:

- *`error:` headline; build aborts.* `error[E0599]` is the headline
  E-code for "no variant or associated item." Lesson 003's
  diagnostic-map vocabulary reads this byte-for-byte. `error:
  aborting due to 1 previous error` is the trailer (lesson 069's
  contrast against `warning:`).
- *Caret under `Maybe`*, the variant-name token to the right of `::`.
  rustc points at exactly the bad name, not at the enum name on the
  left. The headline phrasing names the missing item (`\`Maybe\``)
  and the type it belongs to (`enum \`Sign\``), matching the
  lesson's *What Changed* bullet "variant names are part of the
  type."
- *The supplementary note* `variant or associated item \`Maybe\` not
  found for this enum` underlines the enum declaration `enum Sign`
  itself with `---------`, making the *belongs-to-the-enum* rule
  visually explicit. The diagnostic literally points at the
  declaration site to say "this is where the valid names live."
- *Build does not produce an executable.* `ls` after the failed
  compile shows only `bad_variant.rs` (no new `bad_variant`
  binary).
- *The lesson's centered rule is what the diagnostic states.*
  "Variant names belong to the enum" is the operational
  interpretation of `no variant or associated item named \`Maybe\`
  found for enum \`Sign\``.
- *E0599 is a new E-code in this run's collection.* Prior runs have
  installed E0308, E0425, E0384, E0601, E0004, E0063, E0609, E0277,
  E0689, E0583, E0603, and others. E0599 joins the family today
  with the corpus warrant from `error_codes/E0599.md` (the
  missing-method case) and the empirical witness here (the
  missing-variant case under the same E-code).

### Probe 3 — secondary contrast: non-exhaustive match (E0004)

Source (`/tmp/eduratchet098/broken.rs`):

```rust
enum Sign {
    Positive,
    Negative,
}

fn main() {
    let up = Sign::Positive;
    let label = match up {
        Sign::Positive => "+",
    };
    println!("up = {label}");
}
```

Compile transcript:

```
$ rustc broken.rs
error[E0004]: non-exhaustive patterns: `Sign::Negative` not covered
  --> broken.rs:8:23
   |
 8 |     let label = match up {
   |                       ^^ pattern `Sign::Negative` not covered
   |
note: `Sign` defined here
  --> broken.rs:1:6
   |
 1 | enum Sign {
   |      ^^^^
 2 |     Positive,
 3 |     Negative,
   |     -------- not covered
   = note: the matched value is of type `Sign`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
 9 ~         Sign::Positive => "+",
10 ~         Sign::Negative => todo!(),
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
(exit 1)
```

Witnesses:

- *Same E-code as lesson 030.* The non-exhaustive-patterns rule
  carries through unchanged from `match` on `bool` to `match` on
  `Ordering` (lesson 051) to `match` on `Result` (lesson 058) and
  now to `match` on a *user-declared* unit-only enum. The headline
  names the missing variant by its qualified path
  (`\`Sign::Negative\``).
- *Two `-->` lines.* The first `--> broken.rs:8:23` points at the
  scrutinee `up` (rustc enforces exhaustiveness against the
  scrutinee's type, so the location is the scrutinee). The second,
  inside the `note:` block, points at the enum declaration `--> broken.rs:1:6`
  with `^^^^` under `Sign` and `--------` under the missing variant
  `Negative,` — making the load-bearing rule "every variant the
  type declares must appear as a pattern" visually explicit, the
  same as lesson 051's E0004 transcript on `Ordering` modulo the
  enum name.
- *The `help:` block shows `todo!()` as the placeholder.* Same
  rustc-source-diff shape as lesson 030's E0004 transcript on
  `bool`. `todo!()` is a placeholder macro, not a real fix; the
  real fix is `Sign::Negative => "-"`.
- This contrast is *secondary* (not centered in the lesson body).
  The lesson centers E0599 because (a) E0004 is already installed
  by lesson 030 and reused at lessons 031, 051, 058 — its return
  here is corroborating, not new — and (b) the *new* rule today
  installs is "variant names belong to the enum," which E0599
  witnesses directly. The E0004 transcript is captured here for
  red-team verification that exhaustiveness applies to user-declared
  enums in the same shape it applied to standard-library ones.

## Prerequisite-claim summary

Direct prerequisites — each prerequisite's load-bearing claim used
by this lesson, summarized in 1-3 bullets per the run README:

- **Lesson 030 — `match` form** (load-bearing). `match value { pattern
  => arm_expression, ... }` is an expression; arms share a type;
  exhaustiveness is enforced and a missing case fires E0004 with the
  missing pattern named in the headline. Today's two `match`es are
  exactly this form, with `Sign::Positive` and `Sign::Negative` as
  variant patterns and `&str` arm expressions. Probe 3 witnesses
  E0004 on a user-declared enum, the same E-code as lesson 030.

- **Lesson 058 — `match` on enum variants** (load-bearing). Variant
  paths `Type::Variant` work as patterns in match arms; lesson 058
  used `Ok(num)` and `Err(_)` patterns on `Result`. Today reuses the
  variant-pattern shape but with unit-only variants (no payload), so
  the patterns are bare `Sign::Positive` and `Sign::Negative`
  without parenthesized subpatterns.

- **Lesson 051 — variants of `Ordering`** (cited). `Ordering` is a
  three-variant unit-only enum from the standard library; the lesson
  exercised matching on `Ordering::Less` / `Greater` / `Equal`.
  Today's `Sign` is a *user-declared* two-variant unit-only enum
  with the same shape. The mental-model bridge "I have matched on
  unit variants before; today I declare them" is the lesson 051
  precedent.

- **Lesson 043 — `module::name(args)`** (cited). `::` is the path
  separator. Today's `Sign::Positive` reuses the syntactic shape
  with an enum on the left and a variant on the right (a different
  *namespace* per Reference items/enumerations.md line 35 — the
  variant lives in the value namespace under the enum's name — but
  the syntactic shape is identical). Lesson 051's *What Changed*
  bullet already framed this for `Ordering::Less`; today extends
  the framing to a user-declared enum.

- **Lesson 095 — `struct` with named fields** (cited). The parallel
  data-type lesson. Today's `enum Name { Variant1, Variant2 }`
  reuses the `Name { ... }` brace-grouping shape from lesson 095's
  `struct Name { field: Type, ... }`, but lists *variants* (named
  alternatives) instead of *fields* (named pieces of data). The
  adjacent-but-distinct framing in *Mental Model Delta* and
  *What Changed* is grounded directly in Book ch06-01 lines 4-8.
  Probe 2's E0599 transcript parallels lesson 095's E0609 contrast:
  variant names belong to the enum the same way field names belong
  to the struct.

Older supporting lessons (cited only, no specific claim load-bearing):

- Lesson 001 (`rustc file.rs`; silent on success; produced
  executable). Used by all probe transcripts.
- Lesson 002 (`fn main` runs when the executable launches).
- Lesson 003 (rustc diagnostic four-part map). Used to read Probe 2's
  E0599 transcript and Probe 3's E0004 transcript.
- Lesson 005 (`let name = value;`). Used three times in the working
  probe to bind variant values and match results.
- Lesson 011 (`println!` with `{}`). Used once in the working probe
  with two `{name}` placeholders.
- Lesson 031 (`match` integer + `_` wildcard). Cited only — today's
  match uses no wildcard, named in *What To Ignore For Now* as the
  deferred composition.
- Lesson 044 (`use Path::final;`). Cited only — today's enum is
  declared in the same file, so no `use` is needed; lesson 051 used
  `use std::cmp::Ordering;` for the standard-library enum, and the
  glob form `use Sign::*;` is named in *What To Ignore* as a
  deferred shortcut.
- Lesson 055 (`&str` is the type of string literals). Cited only —
  the match arm bodies `"+"` and `"-"` are `&str` values; the
  inferred type of `label_up` and `label_down` is `&str`, named in
  *Try It* parenthetically.
- Lesson 069 (`warning:` vs `error:` category). Used implicitly in
  Probes 2 and 3 — the trailer is `error: aborting due to 1
  previous error`, not `warning:`.
- Lesson 094 (`unused_must_use` warn-by-default lint). Cited only —
  today's working probe avoids the related `dead_code` lint by
  constructing both variants. The lint family is named in passing in
  the probe-shape note above.

## Contrast-probe coverage

The lesson's contrastive claim is "variant names belong to the
enum's declaration; a path with a non-existent variant name fires a
compile error." This is witnessed empirically by Probe 2 (the E0599
contrast) — the centered teaching point.

A second, secondary contrastive claim — "exhaustiveness on a
user-declared enum requires every variant to appear as a pattern" —
is witnessed by Probe 3 (the E0004 contrast). The lesson does not
center this contrast in the body (E0004 is already installed at
lesson 030 and reused at 031, 051, 058 — its return on a
user-declared enum is corroborating but not new), but the probe is
captured here so the *exhaustiveness still applies* claim is
empirically verified on `Sign` rather than asserted by analogy alone.

## Notes on deferred items

The lesson defers (and this appendix does not probe further):

- *Tuple variants* `Variant(T1, T2)` — Reference items/enumerations.md
  lines 19, 54-69 cover the grammar and example. The rmp target's
  `enum BigInt { Zero, Nonzero(Nonzero) }` uses this shape; the
  natural follow-on after today.
- *Struct-like variants* `Variant { field: T }` — Reference lines 21,
  60-66.
- *Discriminants* `Variant = 1` — Reference lines 23, 139-202; the
  unit-only enum is the discriminant-friendliest shape, but today's
  lesson doesn't need them.
- *Casting unit variants to integers* `Sign::Positive as i32` —
  Reference lines 260-279 cover this for unit-only enums.
- *Variant visibility* `pub` on enums and variants — Reference
  lines 376-407 explicitly cover the rule that variants of a `pub`
  enum are public by default. Lesson 096 installed `pub` on
  functions; extending it to enums and variants is its own move.
- *`#[derive(...)]`* on enums — `Debug`, `Clone`, `Copy`, `PartialEq`.
  The rmp target's `#[derive(Clone, Copy, PartialEq, Eq)] enum Sign`
  line is one move beyond today's bare `enum Sign`. Blocked on the
  trait machinery arc.
- *`if let Pattern = expr { ... }`* — alternative single-arm match.
- *Generic enums* `enum Option<T> { ... }` — type parameters.
- *The wildcard `_` on user-declared enums.* Today's match is
  exhaustive by name; lesson 031's `_` is named only as deferred.
- *Pattern guards* `Pattern if cond => ...`, *recursive enums*,
  *methods on enums via `impl`*, *`use Sign::*;`* glob imports of
  variants.

None of these are load-bearing for the centered claim "declare an
enum with unit variants, construct a value of one variant, match on
it."
