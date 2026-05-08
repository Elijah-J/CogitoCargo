# Evidence — 099-enum-tuple-variants

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -srvm` -> `Darwin 24.5.0 Darwin Kernel Version 24.5.0:
  Tue Apr 22 19:53:26 PDT 2025; root:xnu-11417.121.6~2/RELEASE_X86_64
  x86_64`
- Probes run in `/tmp/eduratchet099/` on this host. Same toolchain
  family as recent accepted lessons (082-098).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/099-enum-tuple-variants.rs`
is the working three-piece probe verbatim, with header comments naming
the centered E0532 contrast probe, the auxiliary E0308 probe, and the
corroborating E0004 probe captured below.

## Sources

### `output/docs/rust/book/ch06-01-defining-an-enum.md`

The Book's *Defining an Enum* chapter — same chapter lesson 098
relied on. Three new load-bearing passages for tuple-variant content
(lesson 098 already exercised lines 4-8, 13-26, 47-71).

#### Lines 152-174 — declaring tuple variants and the constructor-is-a-function rule

> However, representing the same concept using just an enum is more
> concise: Rather than an enum inside a struct, we can put data
> directly into each enum variant. This new definition of the
> `IpAddr` enum says that both `V4` and `V6` variants will have
> associated `String` values:
>
> ```rust
> fn main() {
>     enum IpAddr {
>         V4(String),
>         V6(String),
>     }
>
>     let home = IpAddr::V4(String::from("127.0.0.1"));
>
>     let loopback = IpAddr::V6(String::from("::1"));
> }
> ```
>
> We attach data to each variant of the enum directly, so there is no
> need for an extra struct. Here, it's also easier to see another
> detail of how enums work: The name of each enum variant that we
> define also becomes a function that constructs an instance of the
> enum. That is, `IpAddr::V4()` is a function call that takes a
> `String` argument and returns an instance of the `IpAddr` type. We
> automatically get this constructor function defined as a result of
> defining the enum.

Corpus warrant for the lesson's piece (1) framing — *Declare*: the
tuple-variant shape `Variant(T)` puts data directly into the variant.
Corpus warrant for piece (2) — *Construct*: the variant name is a
constructor function; `IpAddr::V4(String::from("127.0.0.1"))` is a
call expression that returns an instance of the enum. The lesson's
exact phrasing "the path `Brightness::On` is the variant constructor
and behaves like a function — you apply it to an argument and get
back a value of type `Brightness`" is a direct restatement of the
Book's "the name of each enum variant that we define also becomes a
function that constructs an instance of the enum." The auxiliary
contrast probe (Probe 3 below) witnesses this empirically by
revealing the constructor's exact function type
`fn(u32) -> Brightness {Brightness::On}` in an E0308 diagnostic.

#### Lines 236-247 — Listing 6-2 mixed unit + tuple + struct + multi-payload variants

> ```rust
> enum Message {
>     Quit,
>     Move { x: i32, y: i32 },
>     Write(String),
>     ChangeColor(i32, i32, i32),
> }
> ```
>
> *[Listing 6-2](#listing-6-2): A `Message` enum whose variants each
> store different amounts and types of values*
>
> This enum has four variants with different types:
>
> - `Quit`: Has no data associated with it at all
> - `Move`: Has named fields, like a struct does
> - `Write`: Includes a single `String`
> - `ChangeColor`: Includes three `i32` values

Corpus warrant for the *mixed-shape* enum claim today's probe
exercises — `Brightness { Off, On(u32) }` is the same shape as Listing
6-2's `Quit` (unit) + `Write(String)` (one-payload tuple) reduced to
two variants and with a primitive payload. The Book itself uses one
enum to display all three variant shapes side by side, naming the
audience-level rule that an enum can mix shapes. The lesson's *What
To Ignore For Now* names the deferred shapes (struct-like
`Move { x, y }`, multi-payload `ChangeColor(i32, i32, i32)`) using
this Listing's vocabulary.

### `output/docs/rust/reference/items/enumerations.md`

The Reference's *Enumerations* item — same source lesson 098 relied
on. Two new load-bearing passages for tuple-variant content.

#### Lines 19, 113-115 — tuple-variant grammar and call-expression instantiation

> EnumVariantTuple → ( TupleFields? )
> ...
> A tuple-like variant can be instantiated with a [call expression]
> (../expressions/call-expr.md) or a [struct expression]
> (../expressions/struct-expr.md).

Reference warrant: the declaration grammar admits a parenthesized
tuple-fields list after the variant identifier (`On(u32)` instantiates
this with one `u32` field). The construction rule is explicit: a
tuple-like variant is instantiated with a *call expression*
(`Brightness::On(30)` is exactly that). The Reference's call-expression
phrasing parallels lesson 020's function-call shape
`name(value)` — same call-expression machinery, the variant
constructor on the left instead of a function name.

#### Lines 121-137 — example with all three variant shapes

> ```rust
> enum Examples {
>     UnitLike,
>     TupleLike(i32),
>     StructLike { value: i32 },
> }
>
> use Examples::*; // Creates aliases to all variants.
> let x = UnitLike; // Path expression of the const item.
> let x = UnitLike {}; // Struct expression.
> let y = TupleLike(123); // Call expression.
> let y = TupleLike { 0: 123 }; // Struct expression using integer field names.
> let z = StructLike { value: 123 }; // Struct expression.
> }
> ```

Reference warrant for *exactly* the working probe's shape minus the
struct-like variant (deferred today). `TupleLike(i32)` is the
declaration shape, `TupleLike(123)` is the construction shape — both
identical in form to today's `On(u32)` and `Brightness::On(30)`. The
Reference's annotation "Call expression." for `TupleLike(123)` is the
load-bearing terminology behind the lesson's "call expression" framing.

### `output/docs/rust/reference/patterns.md`

The Reference's *Patterns* item — already used by lesson 058 for the
binding-pattern semantics. Two passages reused without modification:

#### Lines 968-980 — Tuple struct patterns grammar and usage

> ## [Tuple struct patterns](#tuple-struct-patterns)
>
> TupleStructPattern → PathInExpression ( TupleStructItems? )
>
> TupleStructItems → Pattern ( , Pattern )* ,?
>
> Tuple struct patterns match tuple struct and enum values that match
> all criteria defined by its subpatterns. They are also used to
> [destructure](#destructuring) a tuple struct or enum value.

Reference warrant for the lesson's piece (3) framing: a *tuple-struct
pattern* `E::Variant(subpattern)` matches a tuple-like enum variant
by name and applies the subpattern to the payload. The grammar
explicitly admits zero or more subpatterns separated by commas — for
single-payload variants like `On(u32)`, the pattern has exactly one
subpattern. Lesson 058 already installed this shape; today reuses it
on a user-declared enum.

#### Lines 196-218 — Identifier (binding) pattern semantics (cited from lesson 058)

> Identifier patterns bind the value they match to a variable in the
> value namespace ... Patterns that consist of only an identifier ...
> match any value and bind it to that identifier.

Reference warrant: the bare name `n` inside `Brightness::On(n)` is a
binding pattern that captures the payload. Lesson 058 already cited
this passage for `num` in `Ok(num)`; today reuses the same machinery
without modification, which is the load-bearing reason today is *not*
a brand-new pattern lesson but a one-rule extension of lesson 098.

### `output/docs/rust/error_codes/E0532.md`

The error code page for E0532 — the bad-pattern-shape error today's
centered contrast probe witnesses:

> Pattern arm did not match expected kind.
>
> Erroneous code example:
>
> ```rust
> enum State {
>     Succeeded,
>     Failed(String),
> }
>
> fn print_on_failure(state: &State) {
>     match *state {
>         // error: expected unit struct, unit variant or constant, found tuple
>         //        variant `State::Failed`
>         State::Failed => println!("Failed"),
>         _ => ()
>     }
> }
> ```
>
> To fix this error, ensure the match arm kind is the same as the
> expression matched.
>
> Fixed example:
>
> ```rust
> ...
> match *state {
>     State::Failed(ref msg) => println!("Failed with {}", msg),
>     _ => ()
> }
> ```

Direct corpus warrant: the lesson's centered E0532 contrast probe
(Probe 2 below) has the *exact same shape* as the corpus erroneous
example — a mixed unit+tuple enum (`Succeeded` + `Failed(String)` in
the corpus, `Off` + `On(u32)` in the probe), and a match arm that
treats the tuple variant as if it were unit-style (no parens). The
corpus's diagnostic message text is identical (`expected unit struct,
unit variant or constant, found tuple variant`) and the corpus's fix
prescription ("ensure the match arm kind is the same as the
expression matched") is exactly the rule today installs. E0532 is
new in this run's E-code collection — prior lessons have installed
E0001/E0004/E0063/E0277/E0308/E0384/E0425/E0583/E0599/E0600/E0601/
E0603/E0609/E0689 (and others); E0532 joins the family today with the
corpus warrant from `error_codes/E0532.md` and the empirical witness
in Probe 2.

### `output/docs/rust/error_codes/E0004.md`

Already cited by lesson 098 (and lessons 030, 031, 051, 058 before
it). Used here for the corroborating non-exhaustive-match probe
(Probe 4): the missing-pattern label is written as
`Brightness::On(_)`, the new tuple-variant pattern shape — same
E-code on a user-declared enum that has at least one tuple variant.

### `output/docs/rust/error_codes/E0308.md`

Already cited by many earlier lessons (most recently 094, 056, 062,
093). Used here for the auxiliary "constructor-without-parens"
probe (Probe 3): rustc surfaces the constructor's exact function
type `fn(u32) -> Brightness {Brightness::On}` in the diagnostic note,
empirically witnessing the Book's claim that the variant name "becomes
a function."

## Probes

### Probe 1 — working: declare with tuple variant, construct, match with subpattern

Source (`/tmp/eduratchet099/demo.rs`, also at
`observations/099-enum-tuple-variants.rs`):

```rust
enum Brightness {
    Off,
    On(u32),
}

fn main() {
    let dim = Brightness::On(30);
    let dark = Brightness::Off;
    let dim_level = match dim {
        Brightness::Off => 0,
        Brightness::On(n) => n,
    };
    let dark_level = match dark {
        Brightness::Off => 0,
        Brightness::On(n) => n,
    };
    println!("dim_level = {dim_level}, dark_level = {dark_level}");
}
```

Compile transcript:

```
$ rustc demo.rs
(no output; exit 0)
$ ls
demo  demo.rs
$ ./demo
dim_level = 30, dark_level = 0
(exit 0)
```

Witnesses:

- **Declaration of mixed shape** is accepted: `enum Brightness { Off,
  On(u32) }` puts a unit variant and a tuple variant side by side in
  one enum. The Reference grammar (lines 13-21) admits a
  comma-separated list of `EnumVariant`s where each variant
  independently chooses its shape. The Book's Listing 6-2 (lines
  236-247) is the corpus model for the mixed shape.
- **Construction with payload** is accepted: `Brightness::On(30)` is
  a call expression — the path `Brightness::On` applied to the
  argument `30`. The result is a value of type `Brightness`, bound to
  `dim` via lesson 005. `Brightness::Off` is unchanged from lesson
  098 — a path expression with no parens producing a unit-variant
  value, bound to `dark`.
- **Match with binding subpattern** is accepted: each `match`
  exhaustively lists `Brightness::Off` (unit pattern) and
  `Brightness::On(n)` (tuple-struct pattern with binding subpattern).
  When the `On(n)` arm matches, `n` binds to the payload (`30` for
  `dim`, never reached for `dark`). The arm body `n` evaluates to the
  bound value, the match's value is `30` for `dim` and `0` for `dark`.
- The output line `dim_level = 30, dark_level = 0` is the load-bearing
  observation that all three pieces composed: the tuple-variant
  declaration accepted the `(u32)` payload, the call-expression
  construction `On(30)` produced the right value, and the binding
  subpattern `On(n)` extracted `30` back out for the arm body.
- **Both variants are constructed** (`dim` for `On(30)`, `dark` for
  `Off`) so the `dead_code` warn-by-default lint does not fire on
  either. (An earlier draft that used a single `let _ = match
  ...; println!(...)` shape with only `On(30)` constructed produced
  `warning: variant \`Off\` is never constructed`. The committed probe
  avoids the warning to keep the lesson focused on the new rule —
  same tradeoff as lesson 098.)
- All three pieces compile under one `rustc` invocation. The
  declaration is an enum item (lesson 098); construction and matching
  happen inside `fn main` (lesson 002).

### Probe 2 — centered contrast: tuple variant matched without subpattern (E0532)

Source (`/tmp/eduratchet099/no_subpattern.rs`):

```rust
enum Brightness {
    Off,
    On(u32),
}

fn main() {
    let dim = Brightness::On(30);
    match dim {
        Brightness::Off => println!("off"),
        Brightness::On => println!("on"),
    }
}
```

Compile transcript:

```
$ rustc no_subpattern.rs
error[E0532]: expected unit struct, unit variant or constant, found tuple variant `Brightness::On`
  --> no_subpattern.rs:10:9
   |
 3 |     On(u32),
   |     ------- `Brightness::On` defined here
...
10 |         Brightness::On => println!("on"),
   |         ^^^^^^^^^^^^^^ help: use the tuple variant pattern syntax instead: `Brightness::On(_)`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0532`.
(exit 1)
```

Witnesses:

- *`error:` headline; build aborts.* `error[E0532]` is the headline
  E-code for "expected unit struct, unit variant or constant, found
  tuple variant." Lesson 003's diagnostic-map vocabulary reads this
  byte-for-byte: headline + `--> location` + source excerpt with
  caret + help-line. `error: aborting due to 1 previous error` is
  the trailer (lesson 069's contrast against `warning:`).
- *Caret under the whole pattern* `Brightness::On`. rustc points at
  the bare path that should have been a tuple-struct pattern.
- *The diagnostic's help-line names the fix verbatim*: `help: use
  the tuple variant pattern syntax instead: \`Brightness::On(_)\``.
  This is the load-bearing observation: the diagnostic *itself*
  states the rule today installs — tuple variants in match arms
  require the parenthesized-subpattern shape, with `_` available as
  the subpattern (lesson 031's wildcard, reused inside the
  constructor exactly as lesson 058 used `Err(_)`).
- *The supplementary note* points at the variant declaration
  `On(u32),` with `-------` and the label `\`Brightness::On\`
  defined here`, making the rule "the variant's *declared shape*
  determines the required pattern shape" visually explicit.
- *Build does not produce an executable.* `ls` after the failed
  compile shows only `no_subpattern.rs` (no new `no_subpattern`
  binary).
- *E0532 is a new E-code in this run's collection.* Prior runs have
  installed E0001/E0004/E0063/E0277/E0308/E0384/E0425/E0583/E0599/
  E0600/E0601/E0603/E0609/E0689; E0532 joins the family today.

### Probe 3 — auxiliary contrast: constructor without parens (E0308 reveals constructor function type)

Source (`/tmp/eduratchet099/no_payload.rs`):

```rust
enum Brightness {
    Off,
    On(u32),
}

fn describe(b: Brightness) {
    match b {
        Brightness::Off => println!("off"),
        Brightness::On(level) => println!("on at {level}"),
    }
}

fn main() {
    let dim = Brightness::On;
    describe(dim);
}
```

Compile transcript:

```
$ rustc no_payload.rs
error[E0308]: mismatched types
  --> no_payload.rs:15:14
   |
 3 |     On(u32),
   |     -- `On` defines an enum variant constructor here, which should be called
...
15 |     describe(dim);
   |     -------- ^^^ expected `Brightness`, found enum constructor
   |     |
   |     arguments to this function are incorrect
   |
   = note:          expected enum `Brightness`
           found enum constructor `fn(u32) -> Brightness {Brightness::On}`
note: function defined here
  --> no_payload.rs:6:4
   |
 6 | fn describe(b: Brightness) {
   |    ^^^^^^^^ -------------
help: use parentheses to construct this tuple variant
   |
15 |     describe(dim(/* u32 */));
   |                 +++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
(exit 1)
```

Witnesses:

- *The diagnostic's `note:` line literally spells the constructor's
  function type:* `found enum constructor \`fn(u32) -> Brightness
  {Brightness::On}\``. This is the empirical witness for the Book's
  claim (ch06-01 lines 169-174) that "the name of each enum variant
  that we define also becomes a function that constructs an instance
  of the enum" — rustc displays the function type signature directly.
  The lesson body cites the Book's framing without reciting this
  signature; the appendix captures it for grounding.
- *The supplementary note on the variant declaration* says `\`On\`
  defines an enum variant constructor here, which should be called`,
  making the *constructor must be called* rule visually explicit.
- *The help line suggests the fix*: `help: use parentheses to
  construct this tuple variant`, with the rustc-source-diff form
  `describe(dim(/* u32 */))` (which would not be the right fix in
  this exact program, but the *help text itself* is the rule
  statement).
- This contrast is *auxiliary* (named in *What To Ignore For Now*
  and the appendix only). The lesson centers Probe 2 (E0532) because
  E0308 here is reached only by passing the un-applied constructor
  through one extra step (a function call site); the centered E0532
  rule fires directly on the match-arm pattern, which is the more
  pedagogically accessible shape.

### Probe 4 — corroborating: non-exhaustive match on tuple variant (E0004 with new shape)

Source (`/tmp/eduratchet099/non_exhaustive.rs`):

```rust
enum Brightness {
    Off,
    On(u32),
}

fn main() {
    let dim = Brightness::On(30);
    match dim {
        Brightness::Off => println!("off"),
    }
}
```

Compile transcript:

```
$ rustc non_exhaustive.rs
error[E0004]: non-exhaustive patterns: `Brightness::On(_)` not covered
  --> non_exhaustive.rs:8:11
   |
 8 |     match dim {
   |           ^^^ pattern `Brightness::On(_)` not covered
   |
note: `Brightness` defined here
  --> non_exhaustive.rs:1:6
   |
 1 | enum Brightness {
   |      ^^^^^^^^^^
 2 |     Off,
 3 |     On(u32),
   |     -- not covered
   = note: the matched value is of type `Brightness`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
 9 ~         Brightness::Off => println!("off"),
10 ~         Brightness::On(_) => todo!(),
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
(exit 1)
```

Witnesses:

- *Same E-code as lesson 030.* Exhaustiveness on a user-declared enum
  with mixed unit + tuple variants applies unchanged. The headline
  names the missing pattern by its qualified path with the
  *new tuple-variant shape*: `\`Brightness::On(_)\`` — the
  parenthesized-wildcard form, exactly the fix shape Probe 2 named.
- *Two `-->` lines.* The first points at the scrutinee, the second
  inside the `note:` block at the variant declaration with `--`
  underlining `On` and the label `not covered`.
- *The `help:` block shows `Brightness::On(_) => todo!()`* as the
  rustc-source-diff fix — using the wildcard subpattern, the same
  shape Probe 2's help-line named. This is the cross-validation: the
  same `Variant(_)` shape appears as the missing-pattern label here
  and as the suggested fix at Probe 2.
- This contrast is *corroborating* (not centered in the lesson body).
  Lesson 098 already exercised E0004 on a user-declared unit-only
  enum; the new content here is that the missing-pattern label is
  written in the new tuple-variant shape rather than as a bare
  variant name. The lesson body mentions this in *What Changed* but
  defers the full transcript to the appendix.

## Prerequisite-claim summary

Direct prerequisites — each prerequisite's load-bearing claim used
by this lesson, summarized in 1-3 bullets per the run README:

- **Lesson 098 — `enum` with unit variants** (load-bearing). The
  declaration form `enum Name { V1, V2 }`, construction via path
  `Name::Variant`, and exhaustive `match` listing each variant.
  Today extends by *one rule*: a variant's identifier may be followed
  by a parenthesized payload type, making the variant a *tuple
  variant*. The unit-variant form `Off` in today's enum is unchanged
  from lesson 098.

- **Lesson 058 — `match` payload-variant patterns** (load-bearing).
  Match patterns of shape `Variant(subpattern)` where the subpattern
  is itself a pattern — most often a binding name like `num` (which
  captures the matched value into a local for the arm's body). Today
  reuses this *exactly* with `Brightness::On(n)`; the only difference
  is that the audience now declared the enum themselves rather than
  consuming the standard library's `Result`. The lesson body explicitly
  names the carry-through.

- **Lesson 020 — function with parameter** (cited). The call-expression
  shape `name(value)` for passing a single argument to a function.
  Today's construction `Brightness::On(30)` is a call expression with
  the variant constructor on the left. The Reference's
  items/enumerations.md line 115 explicitly invokes "call expression"
  vocabulary for tuple-variant instantiation. Lesson 098 used the
  same vocabulary for path expressions (unit-variant case); today
  extends to the call-expression case (tuple-variant case).

- **Lesson 030 — `match` form** (cited). The `match value { pattern
  => arm_expression, ... }` shape with arms sharing a type and
  exhaustiveness via E0004. Today's two `match`es are exactly this
  form. Probe 4 witnesses E0004 on the new tuple-variant shape, the
  same E-code lesson 030 installed.

- **Lesson 062 — `u32`** (cited). The payload type used in today's
  probe. `u32` was chosen because (a) it is already in the graph,
  (b) the literal `30` and `0` print directly with `{name}`
  placeholders, (c) it parallels the rmp target's primitive payloads
  in `enum BigInt { Zero, Nonzero(Nonzero) }` modulo the eventual
  struct-payload extension.

Older supporting lessons (cited only, no specific claim load-bearing):

- Lesson 001 (`rustc file.rs`; silent on success; produced
  executable). Used by all probe transcripts.
- Lesson 002 (`fn main` runs when the executable launches).
- Lesson 003 (rustc diagnostic four-part map). Used to read all three
  contrast probes' transcripts.
- Lesson 005 (`let name = value;`). Used four times in the working
  probe to bind variant values and match results.
- Lesson 011 (`println!` with `{name}`). Used once in the working
  probe with two `{name}` placeholders.
- Lesson 031 (`match` integer + `_` wildcard). Cited only — today's
  centered probe uses no wildcard, but the E0532 diagnostic's
  fix-suggestion `Brightness::On(_)` and the E0004 missing-pattern
  label both use the wildcard inside a tuple-struct pattern (lesson
  058's `Err(_)` shape).
- Lesson 051 (`Ordering` enum). Cited only — today's `match` machine
  is unchanged from 051's matching on standard-library variants.
- Lesson 069 (`warning:` vs `error:` category). Used implicitly in
  Probes 2-4 — every diagnostic trailer is `error: aborting due to 1
  previous error`, not a `warning:`.
- Lesson 094 (`unused_must_use` warn-by-default lint). Cited only —
  today's working probe avoids the related `dead_code` lint by
  constructing both variants. The lint family is named in passing in
  the probe-shape note above.

## Contrast-probe coverage

The lesson's centered contrastive claim is "a tuple variant must be
matched with a parenthesized subpattern; matching it as if it were a
unit variant fires a compile error that names the fix." This is
witnessed empirically by Probe 2 (the E0532 contrast) — the centered
teaching point.

Three secondary corroborating witnesses:

1. *Probe 3 (E0308 with constructor-function-type note)* witnesses
   the Book's claim that "the variant name becomes a function" by
   showing rustc's exact rendering `fn(u32) -> Brightness
   {Brightness::On}`. Auxiliary because reaching this diagnostic
   requires a function-call site, one step removed from the
   centered move; named in *What To Ignore For Now* and captured
   here for grounding.

2. *Probe 4 (E0004 non-exhaustive on user-declared tuple-variant
   enum)* witnesses that exhaustiveness applies unchanged from
   lesson 030, with the missing-pattern label written in the new
   tuple-variant shape `Brightness::On(_)`. Corroborating because
   E0004 itself is already installed; the new content is the shape
   of the missing-pattern label.

3. *The Check-Yourself probe* is independently runnable with all
   three answers verified against captured rustc output.

## Notes on deferred items

The lesson defers (and this appendix does not probe further):

- *Struct-like variants* `Variant { field: T }` — Reference
  items/enumerations.md lines 21, 60-66; Book Listing 6-2 line 239
  (`Move { x: i32, y: i32 }`).
- *Multiple payload positions* `Variant(T1, T2)` — Reference grammar
  `TupleFields → TupleField (, TupleField)* ,?` admits multiple
  fields; Book Listing 6-2 line 241 (`ChangeColor(i32, i32, i32)`)
  is the canonical example. Same rule applied to two payload
  positions; bigger move because both construction and match list
  two values.
- *Tuple variants with struct or enum payload* — the rmp target's
  `enum BigInt { Zero, Nonzero(Nonzero) }` has the `Nonzero` struct
  as the payload type. Composes today's move with lesson 095's
  struct as the payload type; one step beyond.
- *Discriminants on tuple-variant enums* — Reference lines 161-167
  restrict explicit discriminants on enums with non-unit variants
  to specific `#[repr(...)]` configurations. Lesson 098 already
  noted discriminants for the unit-only case; the tuple-variant
  case has stricter rules and is its own move.
- *Literal subpatterns inside tuple variants* `E::Variant(0)` —
  composes today's tuple-variant pattern with literal patterns
  (lesson 030 used literal patterns on `bool`; the integer-literal
  case inside a variant is a sibling extension).
- *Pattern guards* `E::Variant(n) if n > 0 => ...`, *or-patterns*
  `E::A | E::B => ...`, *nested payload patterns* like
  `E::V(Some(x))`, *`@`-bindings* like `n @ 0..=10`, *`mut` and
  `ref` bindings*. All real Rust pattern shapes; all deferred.
- *`if let Pattern = expr { ... }`* — alternative single-arm match
  form. Direct successor for one-variant inspection.
- *Generic enums* `enum Option<T> { None, Some(T) }` — composes
  today's tuple-variant declaration with type parameters.
- *`#[derive(...)]`* on enums (`Debug`, `Clone`, `Copy`, `PartialEq`).
  The rmp target's `#[derive(Clone, Copy, PartialEq, Eq)] enum Sign`
  line is one move beyond today's bare enum. Blocked on the trait
  machinery arc.
- *`pub` on enums and variants*, *`use Sign::*;`* glob imports of
  variants, *methods on enums via `impl`*, *recursive enums*. Same
  defers as lesson 098.

None of these are load-bearing for the centered claim "declare an
enum with at least one tuple variant, construct one with a payload,
match it with a binding subpattern."
