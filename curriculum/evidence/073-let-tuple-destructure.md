# Evidence — 073-let-tuple-destructure

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Sources

### `output/docs/rust/book/ch03-02-data-types.md`

Lines 277-285, the canonical Book example of `let` tuple destructuring:

> ```rust
> fn main() {
>     let tup = (500, 6.4, 1);
>
>     let (x, y, z) = tup;
>
>     println!("The value of y is: {y}");
> }
> ```

Lines 287-291, the Book's destructuring sentence:

> This program first creates a tuple and binds it to the variable
> `tup`. It then uses a pattern with `let` to take `tup` and turn it
> into three separate variables, `x`, `y`, and `z`. This is called
> *destructuring* because it breaks the single tuple into three parts.

Load-bearing for two of the lesson's claims:

- "the left side of `let` does not have to be a single bare name"
  and "uses a pattern with `let`" — the Book's sentence is the
  corpus warrant for the generalization installed in The Move and
  Mental Model Delta. The lesson stops short of building an
  audience-level theory of patterns, but the Book explicitly names
  the construct as "a pattern with `let`."
- The two-step shape `let tup = (...); let (x, y, z) = tup;` is
  exactly the two-step shape the lesson's working probe uses for
  `pair`, with element types adjusted to `i32` so the example only
  uses types installed by prior lessons.

Calibration: lines 270-273 (the sentence "The variable `tup` binds
to the entire tuple because a tuple is considered a single compound
element. To get the individual values out of a tuple, we can use
pattern matching to destructure a tuple value, like this:") motivate
*why* one would want this move at all — it is the alternative to
`pair.0` / `pair.1` access from lesson 072. The lesson cites it
implicitly through "one `let` ... replaces a chain of single-name
`let`s and `.N` accesses" in What Changed.

### `output/docs/rust/reference/patterns.md`

Lines 32 (the patterns intro):

> Patterns are used to match values against structures and to,
> optionally, bind variables to values inside these structures.

Cited for the lesson's "the left of `let` is a *pattern*, not just a
name" generalization in Mental Model Delta and What Changed.

Lines 71-77 (Patterns are used in: `let` declarations):

> Patterns are used in:
>
> - [`let` declarations](statements.md#let-statements)
> - [Function](items/functions.md) and [closure](expressions/closure-expr.md) parameters
> - [`match` expressions](expressions/match-expr.md)
> - [`if let` expressions](expressions/if-expr.md)

Load-bearing for the prerequisite framing: patterns are not unique
to `match`. The Reference lists the *places where patterns appear*,
and `let` is the first listed. The lesson uses this to license
"the Reference uses [the word *pattern*] for both the right of `=>`
in `match` arms ... and the left of `let`."

Lines 99-105 (the Destructuring section):

> Patterns can be used to *destructure* [structs](items/structs.md),
> [enums](items/enumerations.md), and [tuples](types/tuple.md).
> Destructuring breaks up a value into its component pieces. The
> syntax used is almost the same as when creating such values.

Load-bearing for: "splits a tuple value into one binding per field"
in The Move (the "breaks up a value into its component pieces" half)
and "the pattern's parens-and-commas shape mirrors the tuple's" in
The Move and Mental Model Delta (the "the syntax used is almost the
same as when creating such values" half). The lesson's *What To
Ignore* names structs and enums separately and explicitly defers
them, which is the exact list the Reference puts together here.

Lines 137-154 (Refutability):

> A pattern is said to be *refutable* when it has the possibility of
> not being matched by the value it is being matched against.
> *Irrefutable* patterns, on the other hand, always match the value
> they are being matched against. Examples:
>
> ```rust
> let (x, y) = (1, 2);               // "(x, y)" is an irrefutable pattern
> ```

Cited only for the *What To Ignore For Now* item: today's tuple
pattern on the left of `let` is the Reference's explicit example of
an *irrefutable* pattern. The lesson does **not** install the
refutability vocabulary; it operationally observes that the pattern
shape works.

Lines 1030-1066 (Tuple patterns section):

> Tuple patterns match tuple values that match all criteria defined
> by its subpatterns. They are also used to [destructure](#destructuring)
> a tuple.
>
> [...]
>
> An example of using tuple patterns:
>
> ```rust
> let pair = (10, "ten");
> let (a, b) = pair;
>
> assert_eq!(a, 10);
> assert_eq!(b, "ten");
> ```

The Reference's canonical example is structurally identical to the
lesson's working probe (`let pair = (3, 7); let (a, b) = pair;`),
with element types swapped from `(i32, &str)` to `(i32, i32)` so the
example only uses types installed by prior lessons (`&str` is not
installed as a typed name in this run beyond lesson 055's incidental
use). Quoting the Reference's example verbatim here is the
load-bearing source-of-truth for the syntax of the construct.

Calibration: lines 1032-1041 give the Reference grammar
`TuplePattern → ( TuplePatternItems? )` with three alternatives:
`Pattern ,`, `RestPattern`, or `Pattern ( , Pattern )+ ,?`. The
lesson's working probe uses the third alternative
(`Pattern , Pattern (, Pattern)*`); the second alternative (rest
pattern `..`) is named under *What To Ignore*. The first
alternative (a 1-element tuple pattern with trailing comma) is the
1-ary case lesson 072 already deferred and is not raised again.

### Sources NOT cited as load-bearing

- `output/docs/rust/reference/expressions/let-expr.md` — covers
  `let`-as-expression (`if let`, `while let`); not relevant to the
  *statement* form `let pat = expr;` used today.
- `output/docs/rust/reference/statements.md` (let statements
  section) — would be a more direct source for the `let` statement
  grammar, but the Book quote (ch03-02 lines 277-291) and the
  patterns Reference (lines 99-105 and 1030-1066) together cover
  every load-bearing claim. Listed for transparency.
- `output/docs/rust/error_codes/E0308.md` — the diagnostic E-code
  in Probe 2's headline. The probe transcript captured here is
  load-bearing; the explainer page is not separately quoted in the
  lesson body, so it is not a separate citation.

## Probes

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/073-let-tuple-destructure.rs`)
is the *working* version. The broken-contrast probe
(`let (a, b, c) = (3, 7);`) is documented as a separate run below,
not committed as a separate `.rs` file (matching the pattern of
lessons 008, 029, 071, 072).

### Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -sm
Darwin x86_64
```

Same host and toolchain as accepted lessons 029, 068-072.

### Probe 1: working program

Captured in a fresh empty temp dir created with `mktemp -d` and
removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- cat demo.rs ---
fn main() {
    let pair = (3, 7);
    let (a, b) = pair;
    println!("a = {}", a);
    println!("b = {}", b);

    let (x, y, z) = (10, 20, 30);
    println!("x y z = {} {} {}", x, y, z);

    let (m, n) = (5, 2.5);
    println!("m = {}, n = {}", m, n);
}
--- rustc demo.rs (capturing stderr) ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
a = 3
b = 7
x y z = 10 20 30
m = 5, n = 2.5
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (consistent with lesson 001
  and matched by every working probe in this run).
- `./demo` prints four lines, each witnessing a distinct claim:
  - `a = 3` and `b = 7` together: the two-line form `let pair =
    (...); let (a, b) = pair;` produced two bindings, with `a`
    holding the value at field `0` and `b` holding the value at
    field `1`. This is the load-bearing observation for the lesson's
    statement that "`a` from field `0`, `b` from field `1`."
  - `x y z = 10 20 30`: the construct-and-destructure form
    `let (x, y, z) = (10, 20, 30);` works as a single statement at
    arity 3, witnessing that the shape generalizes from arity 2 and
    that the pattern can sit on the left of a *literal tuple
    expression*, not only of a previously-bound tuple.
  - `m = 5, n = 2.5`: heterogeneous fields. `5` is an integer
    literal (lesson 019: defaults to `i32` here, since rustc has no
    other constraint and the matching slot in the pattern is just a
    name) and `2.5` is an `f64` literal (lesson 033). The
    destructured pattern does not introduce any constraint on
    matching element types between the pattern's slots and the
    tuple's fields beyond "one name per field, in order."
- All four `println!` calls reuse the positional `{}` form from
  lesson 011 with `f64` printing established by lesson 033 — no new
  printing behavior is installed.
- The probe deliberately omits a type annotation on the pattern
  (`let (a, b): (i32, i32) = pair;` is grammatical but deferred). It
  also omits `mut`, the wildcard `_`, and the rest pattern `..` —
  all listed under *What To Ignore* — to keep the probe surface
  exactly the move under test.

### Probe 2: broken contrast — arity mismatch (3-pattern, 2-tuple)

Same temp dir, separate file `broken.rs` containing:

```text
--- cat broken.rs ---
fn main() {
    let (a, b, c) = (3, 7);
    println!("{} {} {}", a, b, c);
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:2:9
  |
2 |     let (a, b, c) = (3, 7);
  |         ^^^^^^^^^   ------ this expression has type `({integer}, {integer})`
  |         |
  |         expected a tuple with 2 elements, found one with 3 elements
  |
  = note: expected tuple `({integer}, {integer})`
             found tuple `(_, _, _)`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Read with lesson 003's diagnostic map:

- **Headline**: `error[E0308]: mismatched types`. Coded `[E0308]`.
- **Location**: `broken.rs:2:9` — line 2, column 9, the start of
  the tuple pattern `(a, b, c)`.
- **Source excerpt with caret**: nine-character `^^^^^^^^^` underlines
  the *pattern* `(a, b, c)` on the left side of `=`. A separate
  `------` underlines the right-hand side `(3, 7)` with the inline
  annotation `this expression has type \`({integer}, {integer})\``.
  Together: rustc reports the type *of the value* and points the
  caret at *the pattern* whose declared shape disagrees.
- **Inline annotation under the caret**: `expected a tuple with 2
  elements, found one with 3 elements`. This is the lesson's
  load-bearing piece: rustc reads the pattern as claiming a
  3-element tuple ("found one with 3 elements" — meaning the
  pattern asks for that) and the value as having 2 elements
  ("expected a tuple with 2 elements" — meaning the value's type
  forces that). The phrasing puts the value's type as "expected"
  and the pattern's claim as "found"; the orientation matters less
  to a learner than the headline number pair (2 vs. 3).
- **`= note:` line**: a two-line block — `expected tuple
  \`({integer}, {integer})\`` then `found tuple \`(_, _, _)\``. The
  underscores in `(_, _, _)` reflect that the pattern's slots are
  bare names rustc has not pinned to a type yet; the wider `note:`
  serves as a structured restatement of the inline annotation.
- **Trailer**: `For more information about this error, try \`rustc
  --explain E0308\`.` — present because the headline is coded
  (lesson 070's runnable-instruction shape).
- **Exit code**: 1; no executable produced (`ls` shows only
  `broken.rs`).

This is the load-bearing negative probe for the lesson's contrastive
claim ("with a name count equal to the field count, it works; with
unequal counts, rustc rejects at compile time"). The probe also
witnesses that rustc is willing to enumerate the two integers (2
and 3) in plain prose, which is the audience-level form of the
"name count must equal field count" rule.

### Probe 3: auxiliary — destructuring a non-tuple

Captured for evidence transparency only. **Not** referenced in the
lesson body. The lesson's *What To Ignore* deliberately does not
exercise this contrast because the diagnostic shape ("expected
integer, found `(_, _)`") teaches a different lesson (the type the
pattern projects onto must match the value's type *as a whole*, not
just in count) which is out of scope for today.

```text
--- cat nontup.rs ---
fn main() {
    let (a, b) = 5;
    println!("{} {}", a, b);
}
--- rustc nontup.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> nontup.rs:2:9
  |
2 |     let (a, b) = 5;
  |         ^^^^^^   - this expression has type `{integer}`
  |         |
  |         expected integer, found `(_, _)`
  |
  = note: expected type `{integer}`
            found tuple `(_, _)`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

Documented here only to confirm that destructuring a non-tuple
*also* fires E0308, with the inline annotation phrased "expected
integer, found `(_, _)`" rather than as a count comparison. A
red-team reviewer can see that the lesson's chosen contrast (Probe
2) is the *cleanest* one — the count comparison directly mirrors
the operational rule the lesson installs.

### Negative / contrast probes

Probe 2 is the load-bearing negative probe for the lesson's
contrastive claim. Probe 3 is auxiliary and not load-bearing.

### Reproducibility note

Probe 1 is deterministic on rustc 1.95.0 — the program has no
randomness or environment dependency.

Probe 2's headline (`error[E0308]: mismatched types`), inline
annotation (`expected a tuple with 2 elements, found one with 3
elements`), and `= note:` block (`expected tuple \`({integer},
{integer})\`` / `found tuple \`(_, _, _)\``) are deterministic on
this rustc release. The exact wording is rustc-version-specific;
the *shape* — coded `error[E0308]` with a count-pair phrase plus
`note:` lines naming both tuple types — is grounded in the corpus
and is stable. If a future rustc tweaks wording, the lesson's
substantive claims (the pattern shape must match the tuple's shape;
mismatch is caught at compile time) survive unchanged.

## Prior lessons

Direct prerequisites (load-bearing claims):

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`; rustc silent on success. Used as the compile-and-run
  shape for both probes.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs
  when the executable launches. Used as the container for both
  probes.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — the
  four-part diagnostic map (headline, `-->`, source excerpt with
  caret, optional `help:` / `= note:` lines). Probe 2 is read with
  that map only; no new diagnostic vocabulary is installed today.
- `005-let-binding` (accepted, load-bearing) — `let name = value;`.
  Today extends *the left side* of that form from a single bare
  name to a tuple pattern. The right side is unchanged. Lesson
  005's *What To Ignore* explicitly named today's move (`Pattern
  destructuring on the left side of let, e.g. let (a, b) = ...;`)
  as a deferred future lesson.
- `072-tuple-type-and-index` (accepted, load-bearing) — installs
  tuple types `(T1, T2, ...)`, tuple expressions `(v1, v2, ...)`,
  and the field-numbering convention (first field is `0`). Today
  reuses these without extending them. The forward-link in lesson
  072's *unlocks* names this move as the explicit next cycle
  (deferred-queue Q06): "future *pattern destructuring on the left
  of `let` — `let (a, b) = pair;` (deferred-queue Q06, the
  explicit next move)* moves." Lesson 072's evidence appendix
  (lines 76-79) also says: "The lesson's *What To Ignore For Now*
  explicitly defers that form to the next cycle (deferred-queue
  Q06) and cites those exact lines."

Older supporting lessons (mentioned by id only, not load-bearing
for any individual claim today):

- `011-println-positional-args` — `println!("{}", expr)`. Reused
  as-is; today does not extend `println!`.
- `019-type-annotation-i32` — integer literals default to `i32`.
  Cited under *What To Ignore* (the `let (a, b): (i32, i32) = ...`
  annotated-pattern form) but not load-bearing for any prose claim
  in today's lesson body.
- `029-unit-type` — installed `()` as the 0-arity tuple. Lesson
  072 already builds on it; today does not invoke it directly.
- `030-match-expression`, `031-match-arm-bare-literal`,
  `058-match-result-payload-variants` — used patterns informally on
  the right of `=>`. Cited in Prerequisites only to acknowledge
  prior informal use of patterns.
- `033-f64-floats` — `f64` and the `2.5` literal. Used in one
  probe line.
- `068-let-binding-scope`, `069-rustc-warnings`,
  `070-rustc-explain`, `071-macro-invocation-syntax`,
  `072-tuple-type-and-index` — recent lessons on the same host and
  toolchain. Mentioned only to confirm the host environment is
  unchanged.

No trait-related lesson is cited. The brief explicitly excluded
trait machinery as a prerequisite.

## Deferred-queue effect

This lesson **closes Q06** in
`experimental/eduratchet2/runs/rust-moves/deferred-queue.md`.
Q06's listed missing prerequisites were: tuple types and tuple
values (installed by lesson 072), and "pattern syntax beyond a bare
name" (installed by today's lesson, on the left of `let`). With
both prereqs now installed, Q06 moves out of *Requires
prerequisites* and into *Closed since the original pass* on the
next deferred-queue update.
