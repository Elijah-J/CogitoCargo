# Evidence — 058-match-result-payload-variants

Audit appendix for `lessons/058-match-result-payload-variants.md`.
Holds the corpus-quote map, the toolchain string, the working- and
broken-contrast probe transcripts, a calibration probe for the
no-annotation case, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` -> `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/058-match-result-payload-variants.rs`); the broken-
  contrast and calibration `.rs` files are not committed -- their
  transcripts below are the artifacts.

## Sources

### `output/docs/rust/reference/patterns.md`

The Reference page for patterns. Already cited in lessons 030, 031,
051. Today's primary new corpus span is the *Tuple struct patterns*
subsection, which is the canonical license for the `Variant(subpattern)`
shape used in arm patterns.

Lines 968-980 (the *Tuple struct patterns* heading and grammar):

> ## Tuple struct patterns
>
> **Syntax**
>
> TupleStructPattern -> PathInExpression ( TupleStructItems? )
>
> TupleStructItems -> Pattern ( , Pattern )* ,?
>
> Tuple struct patterns match tuple struct and enum values that match
> all criteria defined by its subpatterns. They are also used to
> destructure a tuple struct or enum value.

This is the canonical Reference license for today's central new
mechanic: a pattern of the shape `Variant(subpattern)` where the
parenthesized contents are *patterns themselves*. Three substantive
claims rest on this:

1. The parenthesized contents are patterns (per the grammar production
   `TupleStructItems -> Pattern (, Pattern)* ,?`). The lesson's
   audience-level prose uses "subpattern"; the Reference's term is
   `Pattern`. Both refer to the same construct.
2. The path on the left is a `PathInExpression` -- the same path form
   used for variant *values* in cycle 052 (`Ok`, `Err`, or
   `Result::Ok`, `Result::Err`). The lesson uses the prelude-shortened
   `Ok` / `Err` form to match cycle 052's surface.
3. Tuple struct patterns "match tuple struct and enum values that
   match all criteria defined by its subpatterns" -- so for a payload
   variant the arm matches when the variant is the right one *and*
   each subpattern matches its corresponding payload component. For
   the single-payload variants `Ok(T)` and `Err(E)`, the pattern is
   `Ok(<one subpattern>)` / `Err(<one subpattern>)`.

Calibration: the Reference's term *tuple struct pattern* covers both
tuple structs (e.g. `struct Point(i32, i32)`) and tuple-like enum
variants (e.g. `Ok(T)`). The Reference's per-clause cross-reference at
line 988 makes this explicit: "A tuple struct pattern matches against
the tuple struct or *tuple-like enum variant* whose constructor is
resolved..." The lesson uses only the enum-variant case; tuple structs
themselves are deferred.

Lines 462-468 plus 514-517 (*Wildcard pattern* subsection, already
cited in lesson 031):

> The *wildcard pattern* (an underscore symbol) matches any value. It
> is used to ignore values when they don't matter.
>
> ...
>
> The wildcard pattern is always irrefutable.

Direct corpus license for `_` as a subpattern inside `Err(_)`. The
wildcard's "matches any value" applies to whatever value the variant
carries. The lesson reuses cycle 031's gloss without re-installing
the term *irrefutable*.

Lines 196-228 (*Identifier patterns* subsection):

> ## Identifier patterns
>
> **Syntax**
>
> IdentifierPattern -> ref? mut? IDENTIFIER ( @ PatternNoTopAlt )?
>
> Identifier patterns bind the value they match to a variable in the
> value namespace.
>
> ...
>
> Patterns that consist of only an identifier, possibly with a `mut`,
> match any value and bind it to that identifier. This is the most
> commonly used pattern in variable declarations and parameters for
> functions and closures.

Direct corpus license for the bare-identifier form `num` as a
subpattern inside `Ok(num)`. The Reference's *Identifier patterns*
subsection is the canonical name for this form; the lesson uses the
audience-level term *binding pattern* / *binding name* rather than
the formal term. Three substantive claims rest on this:

1. A bare identifier *is* a pattern (per the grammar production
   `IdentifierPattern -> ... IDENTIFIER ...`). It is *not* a special
   case of variant-pattern syntax -- it is a pattern in its own right
   that can appear anywhere a pattern can appear, including as a
   subpattern of a `TupleStructPattern`.
2. "Patterns that consist of only an identifier ... match any value
   and bind it to that identifier" -- direct corpus statement that
   `num` matches whatever value the surrounding `Ok(...)` covers and
   binds that value to the local name `num` for the arm's body. The
   lesson's "captures the payload into a local name available in the
   arm's body" rephrases this.
3. Calibration with cycle 031: lesson 031 explicitly deferred "Named
   bindings as catch-all" (the bare-name form as a pattern). Today's
   cycle picks up that deferred concept but in its more common
   *subpattern* role -- inside a variant constructor -- rather than
   the cycle-031 catch-all role at the top level.

Calibration: the Reference at lines 304-308 notes a subtlety -- "Path
patterns take precedence over identifier patterns. ... When a pattern
is a single-segment identifier, the grammar is ambiguous whether it
means an IdentifierPattern or a PathPattern. This ambiguity can only
be resolved after name resolution." For `num`, no constant or unit
variant of that name is in scope, so it resolves as an
IdentifierPattern. The lesson does not surface this name-resolution
subtlety; the empirical observation is just "`num` here is a binding."

### `output/docs/rust/std/result/enum.Result.md`

The std-library page for `Result`. Already cited in lessons 052 and
053. Reused today for the canonical declaration of the variants and
their payloads.

Lines 6-11 (the type's canonical declaration, re-cited for cycle 058):

> ```
> pub enum Result<T, E> {
>     Ok(T),
>     Err(E),
> }
> ```

Direct corpus statement that (a) `Result` is an enum with exactly two
variants `Ok` and `Err`, and (b) both variants carry a payload (`Ok(T)`,
`Err(E)`). For cycle 058 the load-bearing fact is (b): both variants
have a single-payload tuple-like form, so the payload-variant pattern
shape `Variant(subpattern)` is the *only* well-formed match arm shape
for this type (a bare `Ok =>` or `Err =>` would be a path-pattern
attempt that does not match payload-bearing variants). The captured
broken-contrast probe corroborates this empirically: `Err(_)` is what
rustc names as missing, with parentheses-and-subpattern syntax, not
bare `Err`.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book guessing-game chapter. Already cited in lessons 042, 044,
050, 051, 052, 053, 054, 055, 056. Reused today for the audience-level
walkthrough of payload-variant pattern matching on a `Result` from
`.parse()` -- the exact pattern this lesson installs.

Lines 1171-1174 (the canonical match shape, target audience-level
form):

> ```rust
> let guess: u32 = match guess.trim().parse() {
>     Ok(num) => num,
>     Err(_) => continue,
> };
> ```

Direct corpus precedent for the working-probe arm shape:
`Ok(num) => num,` and `Err(_) => <something>,`. The Book's
`<something>` is `continue` (deferred today -- composing with `loop`),
but the structural form -- `Ok(num)` binding the payload to a local
name `num` and emitting it as the arm value, paired with `Err(_)`
discarding the payload -- is identical to the lesson's working probe.

Lines 1200-1214 (the audience-level walkthrough, *load-bearing*):

> If `parse` is able to successfully turn the string into a number, it
> will return an `Ok` value that contains the resultant number. That
> `Ok` value will match the first arm's pattern, and the `match`
> expression will just return the `num` value that `parse` produced
> and put inside the `Ok` value. That number will end up right where
> we want it in the new `guess` variable we're creating.
>
> If `parse` is *not* able to turn the string into a number, it will
> return an `Err` value that contains more information about the
> error. The `Err` value does not match the `Ok(num)` pattern in the
> first `match` arm, but it does match the `Err(_)` pattern in the
> second arm. The underscore, `_`, is a catch-all value; in this
> example, we're saying we want to match all `Err` values, no matter
> what information they have inside them.

Direct audience-level corpus statement of *exactly* the lesson's two
new mechanics:

1. *"the `match` expression will just return the `num` value that
   `parse` produced and put inside the `Ok` value"* -- the corpus
   statement of "the binding `num` captures the `Ok` payload into a
   local available in the arm's body, and the arm body `num`
   evaluates to that captured value." The lesson's *Try It* walk and
   *Mental Model Delta* both rephrase this.
2. *"the `Err(_)` pattern ... The underscore, `_`, is a catch-all
   value; ... we want to match all `Err` values, no matter what
   information they have inside them"* -- the corpus statement of
   "`_` inside `Err(...)` matches and discards the payload." The
   lesson's "if it's `Err`, ignore the payload and use `-1`" rephrases
   this.

Calibration: the Book's example is inside a `loop` and uses
`Err(_) => continue` to retry. The lesson today uses
`Err(_) => -1` (a value, not control flow) to keep the move narrow.
The pattern shapes are identical; only the arm bodies differ. The
deferral of `continue` is explicit in *What To Ignore For Now*.

### `output/docs/rust/book/ch06-02-match.md`

The Book chapter on `match`. Cited as primary corpus source for the
audience-level introduction of payload-variant patterns.

Lines 121-180 (the `Coin::Quarter(state)` example):

> ```rust
> enum Coin {
>     Penny,
>     Nickel,
>     Dime,
>     Quarter(UsState),
> }
>
> ...
>
> fn value_in_cents(coin: Coin) -> u8 {
>     match coin {
>         Coin::Penny => 1,
>         Coin::Nickel => 5,
>         Coin::Dime => 10,
>         Coin::Quarter(state) => {
>             println!("State quarter from {state:?}!");
>             25
>         }
>     }
> }
> ```
>
> ...
>
> In the match expression for this code, we add a variable called
> `state` to the pattern that matches values of the variant
> `Coin::Quarter`. When a `Coin::Quarter` matches, the `state`
> variable will bind to the value of that quarter's state. Then, we
> can use `state` in the code for that arm ...

Direct audience-level corpus precedent for "a variable inside a
variant pattern binds the payload value, available in the arm body."
The Book's `Coin::Quarter(state)` is the same `Variant(subpattern)`
shape today's `Ok(num)` exercises -- variant on the left, bare
identifier as the subpattern that binds the payload. Calibration: the
Book uses a *user-defined* enum `Coin` whose `Quarter` variant carries
a `UsState`; the lesson uses the *standard-library* `Result` whose
`Ok` variant carries the parsed integer. The mechanic is identical.

Lines 200-258 (the `Option<i32>::Some(i)` example):

> ```rust
> fn plus_one(x: Option<i32>) -> Option<i32> {
>     match x {
>         None => None,
>         Some(i) => Some(i + 1),
>     }
> }
> ```
>
> ...
>
> Does `Some(5)` match `Some(i)`? It does! We have the same variant.
> The `i` binds to the value contained in `Some`, so `i` takes the
> value `5`.

Audience-level corpus precedent for the *same* pattern shape applied
to a different generic enum. The Book sentence "`i` binds to the
value contained in `Some`" is structurally identical to the lesson's
"the payload binds to the local name `num`." The Book's `Option<T>`
case is explicitly deferred under *What To Ignore For Now* (sibling
generic enum, future cycle).

### `output/docs/rust/error_codes/E0004.md`

The error-code explainer for E0004 *non-exhaustive patterns*. Already
cited in lessons 030, 031, 051. Reused today for the broken-contrast
probe.

Lines 4-7 (re-cited from cycle 051):

> This error indicates that the compiler cannot guarantee a matching
> pattern for one or more possible inputs to a match expression.
> Guaranteed matches are required in order to assign values to match
> expressions, or alternatively, determine the flow of execution.

The lesson does not re-explain E0004; it cites cycles 030/031/051 by
number. The new structural fact for cycle 058 is that for a
payload-bearing enum scrutinee, the missing-pattern label rustc
prints is also a `Variant(subpattern)`-shape pattern (e.g. `Err(_)`)
-- not a bare variant name. This corroborates the central new
mechanic: the pattern shape for a payload-bearing variant is *not*
`Err`, it is `Err(<subpattern>)`. The captured probe transcript shows
this explicitly.

### `output/docs/rust/reference/items/enumerations.md`

Already cited in lessons 051 and 052. Re-cited for the *constructor*
gloss applied to payload-variant patterns.

Lines 113-115 (already cited in cycle 052):

> A tuple-like variant can be instantiated with a call expression or
> a struct expression.

The cycle-052 use was "the call form `Ok(value)` builds a value." The
cycle-058 use is the *symmetric* one: where the value-construction
side has `Ok(value)` (a call expression), the pattern-destruction
side has `Ok(subpattern)` (a tuple struct pattern). Both share the
same `PathInExpression ( ... )` shape; they differ only in what
appears between the parens (an *expression* on the value side, a
*pattern* on the destruction side). The lesson body draws this
parallel via the term *constructor-shaped pattern*.

### Lesson 056's evidence appendix (existing)

The fact that `: i32` on the binding flows back through `.parse()` to
pin its target type is fully grounded in
`evidence/056-str-parse-to-i32.md` (citing `std/primitive.str.md` line
2176-2186 and `std/primitive.i32.md` lines 3779-3818). Today's lesson
reuses that fact directly: the working probe's `let good: i32 = match
"42".parse() { ... }` uses the same inference path -- the `: i32`
constrains the match, both arms must produce `i32`, and that
constraint propagates back through the match into `.parse()` to pin
its target type as `i32`. Not re-cited inline.

### Lesson 052's evidence appendix (existing)

The declaration `pub enum Result<T, E> { Ok(T), Err(E) }`, the
prelude-membership claim ("we don't need to specify `Result::` before
the `Ok` and `Err` variants"), and the audience-level
generic-type-parameters gloss are all fully grounded in
`evidence/052-result-enum-and-is-ok.md`. Today's lesson reuses these
facts directly. Not re-cited inline.

### Lesson 051's evidence appendix (existing)

The mechanic "match against enum variants" -- arms as path patterns,
exhaustiveness via E0004 naming the missing variant by its qualified
path -- is fully grounded in `evidence/051-ordering-enum-and-variant-match.md`
(citing `reference/patterns.md` lines 1146-1175 *Path patterns*).
Today extends "bare variant name as path pattern" to "variant name
plus parenthesized subpattern as tuple struct pattern." The Reference
treats them as distinct grammar productions (PathPattern vs
TupleStructPattern); the audience-level walk treats them as a smooth
extension. Not re-cited inline.

### Lesson 031's evidence appendix (existing)

The `_` wildcard's "matches any value" is fully grounded in
`evidence/031-match-integer-and-wildcard.md`. Cycle 058's new
contribution is using `_` *inside* a variant constructor; the
wildcard's matching behavior is unchanged. Not re-cited inline.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/058-match-result-payload-variants.rs`.
Identical source to the *Try It* block.

Transcript, captured 2026-05-07 in a fresh `mktemp -d`:

```text
--- ls before ---
demo.rs
--- cat demo.rs ---
fn main() {
    let good: i32 = match "42".parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    let bad: i32 = match "abc".parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    println!("good = {good}, bad = {bad}");
}
--- rustc demo.rs ---
rustc-exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
good = 42, bad = -1
demo-exit=0
--- temp dir removed ---
```

Notes (load-bearing observations):

- `rustc demo.rs` exits 0 silently. No warnings. The match arms with
  payload-variant patterns type-check cleanly.
- `./demo` prints exactly one line: `good = 42, bad = -1`. This is the
  load-bearing observation, with three sub-observations:
  - `good = 42`: `"42".parse()` produced `Ok(42_i32)`, the `Ok(num)`
    arm matched, the payload `42` bound to the local name `num`, the
    arm's body (the bare expression `num`) evaluated to `42`, and that
    became the match's value. `let good: i32 = ...;` bound `good` to
    `42`.
  - `bad = -1`: `"abc".parse()` produced `Err(<some parse-error
    value>)`, the `Ok(num)` arm did *not* match, the `Err(_)` arm
    matched (the variant matched and `_` matched the payload without
    binding), the arm's body `-1` became the match's value, and
    `bad` got `-1`.
  - The match was exhaustive -- both `Ok(...)` and `Err(...)` were
    covered, so rustc accepted the program with no E0004.
- The `: i32` annotation on each binding is what selects `.parse()`'s
  target as `i32` (cycle 056's inference). Without that annotation,
  the integer literal `-1` in the `Err(_)` arm is the *only* concrete
  type signal available, and rustc would default to `i32` for `-1` --
  see calibration probe below for the actual behavior.
- No panic. The success/failure decision is entirely inside the
  match -- where cycle 056 turned `Err` into a runtime panic via
  `.expect`, today the `Err` arm produces a value (`-1`) and execution
  continues. Exit 0.
- The `Err(_)` arm did NOT need to know the type of the `Err` payload
  -- the wildcard `_` matches any payload. This is the load-bearing
  reason the lesson does not have to install `ParseIntError` (the
  actual `Err` payload type) as a typed name today.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe (Shape A -- missing Err arm, E0004)

Source (not committed -- the transcript below is the artifact):

```rust
fn main() {
    let n: i32 = match "42".parse() {
        Ok(num) => num,
    };
    println!("n = {n}");
}
```

The change from the working probe: the `Err(_) => -1,` arm is removed
(and the second match block plus the binding rename to `n`). Captured
2026-05-07 in a fresh `mktemp -d` (filename `broken.rs`):

```text
--- cat broken.rs ---
fn main() {
    let n: i32 = match "42".parse() {
        Ok(num) => num,
    };
    println!("n = {n}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0004]: non-exhaustive patterns: `Err(_)` not covered
 --> broken.rs:2:24
  |
2 |     let n: i32 = match "42".parse() {
  |                        ^^^^^^^^^^^^ pattern `Err(_)` not covered
  |
note: `Result<i32, ParseIntError>` defined here
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/result.rs:557:0
 ::: /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/result.rs:566:4
  |
  = note: not covered
  = note: the matched value is of type `Result<i32, ParseIntError>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
3 ~         Ok(num) => num,
4 ~         Err(_) => todo!(),
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
rustc-exit=1
```

Notes (probe evidence -- not corpus quotation):

- The headline reads
  `error[E0004]: non-exhaustive patterns: `Err(_)` not covered`. Same
  E-code lessons 030, 031, 051 introduced. The headline's
  missing-pattern label is `Err(_)` -- a `Variant(subpattern)`-shape
  pattern, with the wildcard `_` as the subpattern. This is the
  load-bearing piece of probe evidence: rustc itself uses the
  payload-variant pattern shape to describe the missing case, which is
  honest empirical evidence that the right pattern shape for a
  payload-bearing variant is `Err(<subpattern>)`, not bare `Err`.
- The `--> broken.rs:2:24` location points at column 24 of line 2,
  which is the scrutinee `"42".parse()`. Same pattern as cycles
  030/031/051: rustc enforces exhaustiveness against the *scrutinee's
  type*, so the location is the scrutinee.
- The note `the matched value is of type `Result<i32, ParseIntError>``
  names the scrutinee's full type -- including the `Err` payload type
  `ParseIntError`, which was inferred from the `FromStr for i32` impl
  via the `: i32` annotation. The lesson does not surface
  `ParseIntError` as a typed name (deferred since cycle 056); the
  appearance in the diagnostic is honest probe evidence that the
  type exists, not material the lesson installs.
- The second `note:` block (`Result<i32, ParseIntError>` defined here)
  with `-->` pointing into `core/src/result.rs` is the
  standard-library declaration cross-reference. Same shape as cycle
  051's E0004 transcript (which also had a second `note:` pointing at
  `core/src/cmp.rs`). The lesson does not surface this; cycle 051's
  appendix already glossed it as "safe to ignore."
- The `help:` block source-diff suggests `Err(_) => todo!(),` as the
  new arm. `todo!()` is rustc's placeholder macro (carried-forward
  gloss from cycles 030, 031, 051). The real fix is the working probe's
  `Err(_) => -1,`.
- Exit code 1; no executable produced.
- *Calibration with cycle 051's E0004 transcript*: cycle 051's
  diagnostic named `std::cmp::Ordering::Equal` (a unit variant, by
  fully-qualified path) as the missing pattern. Today's diagnostic
  names `Err(_)` (a payload variant, with wildcard subpattern) as the
  missing pattern. Same E-code, same diagnostic structure, two
  different shapes of missing-pattern label -- corroborating that
  exhaustiveness is checked against whatever the scrutinee's type
  considers "complete coverage": all unit variants for cycle 051,
  all payload-variant pattern shapes for cycle 058. Together with
  cycles 030 (literal patterns) and 031 (range patterns) the run has
  now captured E0004 in four shapes.

This probe is *load-bearing* for the lesson's broken-contrast claim
"omitting the `Err(_)` arm fires E0004 with `Err(_)` named as the
missing pattern."

### Calibration probe (Shape B -- no `: i32` annotation)

Source (not committed -- the transcript below is the artifact):

```rust
fn main() {
    let good = match "42".parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    println!("good = {good}");
}
```

The change from the working probe: the `: i32` annotation is removed,
and the second match block is dropped (so the only constraint comes
from the integer literal `-1`). Captured 2026-05-07:

```text
--- cat misled.rs ---
fn main() {
    let good = match "42".parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    println!("good = {good}");
}
--- rustc misled.rs ---
rustc-exit=0
```

Notes (probe evidence -- not corpus quotation):

- `rustc misled.rs` exits 0 with no warnings. The program type-checks
  even *without* a `: i32` annotation on `good`, because:
  - the `Err(_) => -1,` arm produces an integer literal whose default
    type is `i32` (cycle 019 / 030's behavior);
  - both arms of the match must agree in type (cycle 030's rule), so
    the `Ok(num) => num` arm must also produce an `i32`;
  - that constrains `num` to be an `i32`, which constrains
    `"42".parse()` to produce a `Result<i32, _>` (cycle 056's
    inference);
  - the integer-default rule is enough to close the loop.
- This calibration probe is included to head off a potential
  red-team objection: "is the `: i32` on each binding actually
  load-bearing?" The honest answer is no -- in this exact probe the
  integer-literal-default would do the work. The lesson's *Try It*
  walk attributes the `i32` choice to the `: i32` annotation
  (consistent with cycle 056's framing) because (a) the annotation
  is the audience-level mechanism cycle 056 installed as the
  primary inference driver; (b) the working probe writes both
  bindings with `: i32` for clarity; (c) the alternative -- relying
  on integer-literal default -- is a separate inference channel
  not yet surfaced in any lesson and would be a misleading thing to
  install in passing.
- This probe is *not* load-bearing for any lesson claim. It is
  captured for audit transparency: the working-probe behavior is
  reachable by more than one inference path, and the lesson chose
  the one cycle 056 already installed.

### Calibration probe (Shape C -- references `num` in wrong arm, E0425)

Source (not committed -- transcript is the artifact). Used to verify
the *Check Yourself (c)* answer:

```rust
fn main() {
    let v: i32 = match "10".parse() {
        Ok(num) => 0,
        Err(_) => num,
    };
    let next: i32 = v + 1;
    println!("v = {v}, next = {next}");
}
```

Captured 2026-05-07:

```text
--- rustc tiny.rs ---
error[E0425]: cannot find value `num` in this scope
 --> tiny.rs:4:19
  |
4 |         Err(_) => num,
  |                   ^^^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
rustc-exit=1
```

Notes:

- The headline reads `error[E0425]: cannot find value `num` in this
  scope`. E0425 was first installed in lesson 005 as the "cannot find
  value" diagnostic for an undeclared name; lesson 008 reused it for
  unknown function names. Today it surfaces in a new context: a
  `match` arm can only see the bindings introduced by *its own*
  pattern. The `num` binding from `Ok(num)` is not in scope inside
  the `Err(_)` arm.
- Note rustc's caret points at the `num` reference, not at the arm
  pattern -- the diagnostic frames it as "this name doesn't exist
  here," which is exactly the audience-level mental model the
  *Check Yourself (c)* answer offers ("each arm sees only its own
  pattern's bindings").
- E0425 is already installed; the lesson's *Check Yourself* uses
  the audience-level phrasing "cannot find value `num`" matching
  the diagnostic text. No new E-code installed.
- Behavior also confirmed for the *Check Yourself* (a) and (b)
  predictions:
  - (a) `Ok(num) => num, Err(_) => 0,` with `"10"`: prints
    `v = 10, next = 11`, exit 0.
  - (b) Same arms with `"oops"`: prints `v = 0, next = 1`, exit 0.
    *No panic.*

This probe is *load-bearing* for the *Check Yourself (c)* answer.
Without it the answer would rest only on the Reference's
*Identifier patterns* statement that "the variable will shadow any
variables of the same name in scope. The scope of the new binding
depends on the context of where the pattern is used (such as a `let`
binding or a `match` arm)" (lines 212-214) -- which gestures at
arm-local scope but does not produce the empirical diagnostic the
*Check Yourself* answer references.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 058. Older supporting lessons are mentioned
above by number only.

- **Lesson 052 (load-bearing)** -- installed `Result<T, E>` as the
  prelude two-variant enum with `Ok(T)` / `Err(E)`. Cycle 052's only
  inspection method was `.is_ok()` (returning a `bool`). Today's
  cycle is the *first* one that opens an `Ok` or `Err` to look at
  the payload. The Book ch02 lines 1200-1214 walkthrough quoted
  above is the direct audience-level bridge: cycle 052 introduced
  the variants; cycle 058 introduces how to take their payloads
  apart. The lesson's "I couldn't get the payload out without
  panicking" framing makes this transition explicit. The new fact
  for 058 is the pattern shape `Variant(subpattern)`, with the
  payload accessible as a binding.
- **Lesson 051 (load-bearing)** -- installed `match` against enum
  variants as path patterns (`Ordering::Less`, etc.) plus
  exhaustiveness via E0004 naming the missing variant. Today
  *extends* the pattern shape: from `Variant` (a path pattern) to
  `Variant(subpattern)` (a tuple struct pattern). The Reference
  treats them as distinct grammar productions; the audience-level
  delta is small ("you write parens, with another pattern inside,
  to handle a payload variant"). Cycle 051's `Ordering` had no
  payloads -- it was the unit-variant case; cycle 058 picks up the
  payload-variant case explicitly deferred in cycle 051's *What To
  Ignore For Now*.
- **Lesson 031 (load-bearing)** -- installed the `_` wildcard pattern.
  Cycle 031's use was at the *top level* of a match arm pattern (`_
  => 99,` as the catch-all over all integer values). Cycle 058 uses
  `_` *inside* a variant constructor (`Err(_)`), where it matches
  the variant's payload while binding nothing. The wildcard's
  semantics are unchanged -- "matches any value." The new fact is
  that `_` is allowed in *subpattern* positions, not just at the top
  level of a pattern. The lesson body draws this parallel as "same
  wildcard, one level deeper."
- **Lesson 030** -- installed the whole `match` machine (scrutinee,
  arms `pattern => arm_expression,`, all arms share a type, matching
  arm wins, exhaustiveness). All carried over unchanged.
- **Lesson 056 (load-bearing)** -- installed `.parse()` returning
  `Result<TARGET, _>` and the inference path "the `: i32` annotation
  on the binding flows back through the use site to pin down the
  target type." Today's working probe's `let good: i32 = match
  "42".parse() { ... }` reuses this exact inference path: the `: i32`
  on the binding constrains the match's value to `i32`, both arms
  must produce `i32`, which forces `num: i32` (in the `Ok(num) => num`
  arm), which forces `.parse()` to produce a `Result<i32, _>`.
- **Lesson 055** -- installed `&str` as the type of string literals.
  Today's `"42"` and `"abc"` are `&str` values; `.parse()` is a
  method on `&str`.
- **Lesson 040 + 049** -- dot-form method call and
  receiver-is-an-expression rule. `"42".parse()` is the dot-form on
  a `&str` receiver; the `.parse()` call has no arguments.
- **Lesson 019** -- `let name: TYPE = value;` annotation slot. Used
  twice with `i32`. Today is also lesson 019's first use *inside* a
  match-result binding -- a small extension of where the slot
  appears.
- **Lessons 001, 002, 005, 011** -- compile and run, `fn main`,
  `let`, and the `{name}` placeholder in `println!`. All used
  unchanged.

## Older supporting lessons

- Lesson 008 (free-function call form) -- not used today; today's
  surface is method calls and matches, not free function calls.
- Lesson 005 (`let` binding) and 019 (annotation) -- both used
  unchanged.
- Lesson 042 (`Type::name(args)` no-receiver call) -- not used today;
  the `Result::Ok(...)` form would also work but the prelude lets
  the lesson use the bare `Ok(...)` form directly.
- Lesson 044 (`use` declaration) -- *not* used today; `Result`,
  `Ok`, `Err` are in the prelude (Book ch09-02 line 87, cited in
  cycle 052's evidence appendix), so no `use` line is needed.
- Lesson 053 (`.expect("msg")`, panic) -- *deliberately* not a
  load-bearing prerequisite. Today's lesson is the contrast to
  cycle 053's panic path: where `.expect(...)` panics on `Err`,
  today's `match { ..., Err(_) => default }` produces a value and
  execution continues. The lesson body draws this contrast in the
  *Try It* walk ("Where cycle 056's `.expect(...)` *panicked* on
  `Err`, today's `match` *avoids* the panic..."). Cycle 053 is
  *referenced* in the lesson body once but not in the dependency
  list.
- Cycles 024-034, 045-048, 052, 054, 055, 056 (E0308 family) --
  *not* exercised today. The broken contrast is E0004
  (non-exhaustive patterns), not E0308. The calibration probe Shape
  C is E0425 (cannot find value), already installed.
- Cycle 029 (underscore-prefix gloss for binding names) -- *not*
  used today. The `_` here is a wildcard *pattern*, not the
  underscore-prefix binding-name convention. Same character,
  different role. Cycle 031's evidence appendix already glossed this
  distinction; it is not re-installed today.

## Calibration: minor surface choices not surfaced in the lesson body

- The probe binding names are `good` and `bad`. The Book uses `guess`
  in this same shape (per ch02 line 1171); the lesson uses
  domain-neutral names so as not to lean on the guessing-game
  context.
- The probe message inside `Err(_)` is the integer literal `-1`. The
  Book uses `continue` (a control-flow statement, deferred). The `-1`
  signals "no value" via a sentinel rather than a control-flow
  effect; this keeps the move squarely on patterns and arm values
  rather than on `loop` interaction.
- The two `match` blocks are written separately rather than wrapped
  in a function. A function-bound version would also work but pulls
  in lesson 020/021 surface the lesson does not need today.
- Both bindings carry `: i32` annotations. Per the Shape B
  calibration probe above, the annotations are not strictly required
  here -- the integer literal `-1` in the `Err(_)` arm would default
  to `i32` and pin everything down. The annotations make cycle 056's
  inference path the audience-level explanation.
- Arm bodies are bare `num` (in `Ok(num)`) and bare integer literals
  (in `Err(_)`). Adding arithmetic in the `Ok` arm body (`num + 1`)
  would re-open the type-inference question (cycle 056 still infers
  `num`'s type from the `: i32` constraint flowing back, but doing
  so through `num + 1` is more inference fan-out than today's lesson
  surfaces). The *Check Yourself* (a) example uses the simpler
  `Ok(num) => num,` arm body and adds a separate `let next: i32 = v
  + 1;` line to demonstrate that the binding type is `i32`.
- The lesson's broken contrast is *missing* `Err(_)` (E0004) rather
  than (a) writing `Ok =>` without parens (would fire E0308 or
  similar -- variant pattern shape mismatch, untested today) or
  (b) adding a third arm (would compile cleanly but trip an
  unreachable-pattern warning, deferred). The orchestrator's
  suggested broken contrast -- E0004 missing-arm -- is the
  load-bearing one.
