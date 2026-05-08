# Evidence — 072-tuple-type-and-index

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Sources

### `output/docs/rust/book/ch03-02-data-types.md`

Lines 252-256, the Book's first sentence under `#### The Tuple Type`:

> A *tuple* is a general way of grouping together a number of values
> with a variety of types into one compound type. Tuples have a fixed
> length: Once declared, they cannot grow or shrink in size.

Load-bearing for two of the lesson's claims:

- "fixed-length bundle of values" — the lesson's *fixed-arity* claim
  in The Move and the "tuples have fixed length" sentence under
  Mental Model Delta both flow from the second sentence here.
- "values with a variety of types" — the lesson's "element types do
  not have to match" claim and the *heterogeneous* word in Try It
  both come from the first sentence here.

Lines 258-269, the construction-and-annotation example:

> We create a tuple by writing a comma-separated list of values inside
> parentheses. Each position in the tuple has a type, and the types of
> the different values in the tuple don't have to be the same. We've
> added optional type annotations in this example:
>
> ```rust
> fn main() {
>     let tup: (i32, f64, u8) = (500, 6.4, 1);
> }
> ```

Load-bearing for the lesson's *tuple expression* description ("a
parenthesized comma-separated list of values"), the explicit
*optional annotation* phrasing in Try It, and the heterogeneity
claim restated. The lesson's working probe is the same shape — a
binding of an annotated tuple — restricted to 2-tuples and 3-tuples
of `i32` plus one `(i32, f64)`, all element types already installed
by prior lessons.

Lines 293-312, the indexing example and the "first index is 0"
sentence:

> We can also access a tuple element directly by using a period (`.`)
> followed by the index of the value we want to access. For example:
>
> ```rust
> fn main() {
>     let x: (i32, f64, u8) = (500, 6.4, 1);
>
>     let five_hundred = x.0;
>
>     let six_point_four = x.1;
>
>     let one = x.2;
> }
> ```
>
> This program creates the tuple `x` and then accesses each element
> of the tuple using their respective indices. As with most
> programming languages, the first index in a tuple is 0.

Load-bearing for: the syntactic description of *tuple indexing
expression* ("a period followed by the index"); the "first index is
`0`" claim; and the lesson's `pair.0`/`pair.1`/`triple.2`/`mixed.0`/
`mixed.1` accesses, which are exactly the same shape on slightly
different tuple values.

Calibration: lines 277-285 of the same Book section show the
*destructuring* form `let (x, y, z) = tup;`. The lesson's
*What To Ignore For Now* explicitly defers that form to the next
cycle (deferred-queue Q06) and cites those exact lines. Today
installs only construction `(v1, v2, ...)` and access `expr.N`.

Calibration: line 314 contains the unit-tuple sentence Lesson 029
already grounded ("The tuple without any values has a special name,
*unit*..."). Today's lesson references lesson 029 to align the
0-arity case with the non-zero-arity case introduced today; line 314
is not requoted in the lesson body to avoid re-installing what 029
already installed.

### `output/docs/rust/reference/types/tuple.md`

Lines 14-18, the structural-type definition and the type-syntax rule:

> *Tuple types* are a family of structural types for heterogeneous
> lists of other types.
>
> The syntax for a tuple type is a parenthesized, comma-separated
> list of types.

Load-bearing for: the lesson's "type is `(T1, T2, ...)`" claim in
The Move, Mental Model Delta, and What Changed; and for the
"heterogeneous" framing reinforced from the Book.

Lines 24-30, the field-arity and field-name rules:

> A tuple type has a number of fields equal to the length of the
> list of types. This number of fields determines the *arity* of the
> tuple. A tuple with `n` fields is called an *n-ary tuple*. For
> example, a tuple with 2 fields is a 2-ary tuple.
>
> Fields of tuples are named using increasing numeric names matching
> their position in the list of types. The first field is `0`. The
> second field is `1`. And so on. The type of each field is the type
> of the same position in the tuple's list of types.

Load-bearing for: the lesson's "each field has a numeric *name*
matching its position" sentence; the "first index is `0`" claim
(the Reference says it directly, not just by example); and the
"rustc knows the field names from the type itself" claim under
What Changed (which is a paraphrase of the second sentence here —
the field names are part of the type, not the value).

The word *arity* is used in the lesson title ("two or more fields")
and in The Move ("a 2-tuple"), licensed by the second sentence
above.

Lines 32-34, the unit framing (cross-link to lesson 029):

> For convenience and historical reasons, the tuple type with no
> fields (`()`) is often called *unit* or *the unit type*. Its one
> value is also called *unit* or *the unit value*.

Already lesson 029's load-bearing quote. Cited again here only as
the cross-link justifying "Lesson 029's `()` is the 0-arity case
of this family" in What Changed and Mental Model Delta.

Lines 36-43, the tuple-type-examples table:

> Some examples of tuple types:
>
> - `()` (unit)
> - `(i32,)` (1-ary tuple)
> - `(f64, f64)`
> - `(String, i32)`
> - `(i32, String)` (different type from the previous example)
> - `(i32, f64, Vec<String>, Option<bool>)`

Cited for two of the lesson's *What To Ignore* items: (1) the
1-ary form `(i32,)` is real but deferred (the entry "1-ary tuples
and the trailing-comma rule"); (2) examples beyond the lesson's
arity range exist but are not installed.

Calibration: lines 20-22 give the *1-ary* disambiguation rule
("1-ary tuples require a comma after their element type to be
disambiguated with a parenthesized type"). The lesson's
*What To Ignore For Now* names the rule; today's working probe
uses 2-tuples and 3-tuples to sidestep it.

### `output/docs/rust/reference/expressions/tuple-expr.md`

Lines 16-22, the tuple-expression definition:

> A *tuple expression* constructs [tuple values].
>
> The syntax for tuple expressions is a parenthesized, comma
> separated list of expressions, called the *tuple initializer
> operands*.

Load-bearing for: the lesson's "tuple expression" naming and
"parenthesized comma-separated list of values" description in Try It
and Mental Model Delta.

Lines 32-42, fields and indexing inside a tuple expression:

> The number of tuple initializer operands is the arity of the
> constructed tuple.
>
> [...]
>
> For other tuple expressions, the first written tuple initializer
> operand initializes the field `0` and subsequent operands
> initializes the next highest field. For example, in the tuple
> expression `('a', 'b', 'c')`, `'a'` initializes the value of the
> field `0`, `'b'` field `1`, and `'c'` field `2`.

Load-bearing for: the "value is `(v1, v2, ...)`" half of the
parens-and-commas pun; the "first written value goes into field 0"
claim in Try It (where `pair.0` is the `3` from `(3, 7)`); the
mapping between the tuple-expression operand list and the field
numbers used to access them.

Lines 53-71, the tuple-indexing-expression syntax:

> A *tuple indexing expression* accesses fields of [tuples] and
> [tuple structs].
>
> The syntax for a tuple index expression is an expression, called
> the *tuple operand*, then a `.`, then finally a tuple index.
>
> [...]
>
> The syntax for the *tuple index* is a [decimal literal] with no
> leading zeros, underscores, or suffix. For example `0` and `2`
> are valid tuple indices but not `01`, `0_`, nor `0i32`.

Load-bearing for: the lesson's *tuple indexing expression* term in
Try It; the "plain decimal number with no leading zeros, no
underscores, no type suffix" rule in Try It and What Changed. The
Reference's three negative examples (`01`, `0_`, `0i32`) are the
exact corpus warrant for that rule. The lesson does not enumerate
the negative examples in body text; they live here in evidence.

Lines 77-83, the index-name rule:

> The tuple index must be a name of a field of the type of the tuple
> operand.
>
> [...]
>
> Evaluation of tuple index expressions has no side effects beyond
> evaluation of its tuple operand.

Load-bearing for the broken-contrast E0609 claim: the index must be
a *field name* of the tuple's type, and the field names are exactly
those declared by the type. `pair.2` on a 2-tuple violates this
rule because the type `(i32, i32)` has only fields `0` and `1`.
This is the corpus warrant for the lesson's "rustc knows the field
names from the type itself" claim.

Lines 87-100, the indexing examples used as a sanity check:

> ```rust
> let pair = ("a string", 2);
> assert_eq!(pair.1, 2);
>
> // Indexing a tuple struct
> struct Point(f32, f32);
> ```

The Reference's `pair` is structurally analogous to the lesson's
`pair`. Cited here as a verbatim demonstration that the access
shape on a 2-tuple matches today's probe; the `Point` struct in
the same example is the very tuple-struct construct the lesson
defers under *What To Ignore*.

### `output/docs/rust/error_codes/E0609.md` (not in corpus path list above; verified by `--explain`)

The diagnostic captured in Probe 2 carries the trailer
`For more information about this error, try \`rustc --explain
E0609\`.`. Lesson 070 installed that trailer as a runnable
instruction. The corpus E0609 page is implied by the captured
diagnostic; it is not a load-bearing source separate from the
captured probe transcript, because the lesson body never quotes it.
Listed here only for transparency about which E-code's page would
be the canonical explainer if the learner runs the trailer.

### Sources NOT cited

- `output/docs/rust/reference/patterns.md` — covers tuple patterns
  in `match` and `let`. The lesson explicitly defers both. Not
  load-bearing today.
- `output/docs/rust/reference/types/struct.md` (tuple structs) —
  named under *What To Ignore* but not installed; not a corpus
  warrant for any prose claim today.
- `output/docs/rust/book/ch03-02-data-types.md` lines 319-end (the
  *Array Type* section). Adjacent in the corpus but unrelated;
  arrays differ from tuples (homogeneous vs heterogeneous, slice
  indexing vs `.N`). Future move.

## Probes

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/072-tuple-type-and-index.rs`)
is the *working* version. The broken-contrast probe (`pair.2` on a
2-tuple) is documented as a separate run below, not committed as a
separate `.rs` file (matching the pattern of lessons 008, 029, 071).

### Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -sm
Darwin x86_64
```

Same host and toolchain as accepted lessons 029, 068-071.

### Probe 1: working program

Captured in a fresh empty temp dir created with `mktemp -d` and
removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let pair: (i32, i32) = (3, 7);
    let triple = (10, 20, 30);
    let mixed: (i32, f64) = (5, 2.5);

    let first = pair.0;
    let second = pair.1;

    println!("pair = ({}, {})", first, second);
    println!("triple.2 = {}", triple.2);
    println!("mixed = ({}, {})", mixed.0, mixed.1);
}
--- rustc demo.rs (capturing stderr) ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
pair = (3, 7)
triple.2 = 30
mixed = (5, 2.5)
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (consistent with lesson 001;
  no warnings, no errors).
- `./demo` prints three lines, each witnessing a distinct claim:
  - `pair = (3, 7)`: confirms `pair.0` evaluated to `3` and `pair.1`
    evaluated to `7` — i.e., the first written tuple-expression
    operand initialized field `0`, exactly as the Reference's
    `expr.tuple.fields` rule says. This is the load-bearing
    observation for the indexing claim.
  - `triple.2 = 30`: confirms (a) rustc accepted `let triple = (10,
    20, 30);` *without* an annotation (so type inference from three
    integer literals produced a valid tuple type); and (b) the
    indexing expression `triple.2` typechecks and evaluates to the
    third operand `30`. Together: a 3-tuple of `i32` is a real
    inferable type, and field `2` exists on it.
  - `mixed = (5, 2.5)`: confirms the heterogeneous case — the
    annotation `: (i32, f64)` accepted the operand list `(5, 2.5)`
    where `5` is an integer literal (lesson 019: defaults to `i32`,
    but here pinned to `i32` by the annotation slot at field 0) and
    `2.5` is a floating-point literal (lesson 033: defaults to
    `f64`, here matching the annotation slot at field 1). Element
    types do not have to match.
- All three `println!` calls reuse the `{}` form from lesson 011 and
  the `f64` printing established by lesson 033 — no new printing
  behavior is installed.
- The two intermediate bindings `let first = pair.0;` and `let
  second = pair.1;` are not strictly necessary (the lesson could
  have written `pair.0` and `pair.1` directly inside the
  `println!`), but binding them first keeps the probe surface small
  per the *What To Ignore* line "binds the fields to `first` and
  `second` first to keep one move per lesson" — the move under test
  is *reading* a field by `.N`, not composing it with arithmetic or
  format-string features.

### Probe 2: broken contrast — out-of-bounds index `pair.2`

Same temp dir, separate file `broken.rs` containing:

```text
--- cat broken.rs ---
fn main() {
    let pair: (i32, i32) = (3, 7);
    let bad = pair.2;
    println!("{}", bad);
}
--- rustc broken.rs (capturing stderr) ---
error[E0609]: no field `2` on type `(i32, i32)`
 --> broken.rs:3:20
  |
3 |     let bad = pair.2;
  |                    ^ unknown field
  |
  = note: available fields are: `0`, `1`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0609`.
exit=1
--- ls after ---
broken.rs
```

Read with lesson 003's diagnostic map:

- **Headline**: `error[E0609]: no field \`2\` on type \`(i32, i32)\``.
  Coded `[E0609]`. The headline names *both* the offending field
  number and the type that does not have it; the type appears as
  `(i32, i32)`, in the same parens-and-commas shape the lesson uses
  in its prose.
- **Location**: `broken.rs:3:20` — line 3, column 20, the literal
  `2` after the `.`. (Column 20 is the position of the `2` in
  `let bad = pair.2;` counting from 1 — the leading 4-space
  indentation plus `let bad = pair.` puts the `2` at column 20.)
- **Source excerpt with caret**: a single `^` underlines the `2`
  with the inline annotation `unknown field`. The caret is on the
  index, not on the operand `pair`, which matches the Reference's
  rule that the *tuple index* must be a field name and is what
  rustc points at when the rule fails.
- **`= note:` line**: `available fields are: \`0\`, \`1\``. This is
  the lesson's load-bearing audience-level enumeration: rustc lists
  the legal field names of the tuple's type. The list is exactly
  the field names declared by the type `(i32, i32)`, witnessing
  that the field names are part of the type and that there are
  exactly two of them on a 2-tuple.
- **Trailer**: `For more information about this error, try \`rustc
  --explain E0609\`.` — present because the headline is coded
  (lesson 070's runnable-instruction shape).
- **Exit code**: 1; no executable produced (`ls` shows only
  `broken.rs`).

This is the load-bearing negative probe for the lesson's
*contrastive* claim ("with a field number that exists on the type,
the program builds; with one that does not, rustc rejects at compile
time"). The same probe also witnesses the *fixed-length* claim:
rustc enumerates the fields, so the type carries the length.

### Probe 3 (auxiliary, not in lesson body): 1-ary disambiguation

Captured but **not** in the lesson body. Documented here only for
transparency about why the lesson's *What To Ignore For Now* names
the 1-ary trailing-comma rule as real and deferred:

```text
--- cat broken2.rs ---
fn main() {
    let p: (i32,) = (5);
    println!("{}", p.0);
}
--- rustc broken2.rs (capturing stderr) ---
warning: unnecessary parentheses around assigned value
 --> broken2.rs:2:21
  |
2 |     let p: (i32,) = (5);
  |                     ^ ^
  |
  = note: `#[warn(unused_parens)]` (part of `#[warn(unused)]`) on by default
help: remove these parentheses
  |
2 -     let p: (i32,) = (5);
2 +     let p: (i32,) = 5 ;
  |

error[E0308]: mismatched types
 --> broken2.rs:2:21
  |
2 |     let p: (i32,) = (5);
  |            ------   ^^^ expected `(i32,)`, found integer
  |            |
  |            expected due to this
  |
  = note: expected tuple `(i32,)`
              found type `{integer}`
help: use a trailing comma to create a tuple with one element
  |
2 |     let p: (i32,) = (5,);
  |                       +

error: aborting due to 1 previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0308`.
exit=1
```

This probe witnesses the Reference rule (`type.tuple.restriction`
and `expr.tuple.unary-tuple-restriction`) operationally: rustc's
own `help: use a trailing comma to create a tuple with one element`
line is the rule restated by the compiler. The lesson does **not**
install this; it cites the rule under *What To Ignore* and
sidesteps it by using only 2-tuples and 3-tuples in the working
probe. Listed here so a red-team reviewer can confirm the
deferral is grounded.

### Negative / contrast probes

Probe 2 is the load-bearing negative probe for the contrastive
claim. Probe 3 is auxiliary evidence for the 1-ary deferral but is
not load-bearing for any claim in the lesson body. No further
negative probe is needed.

### Reproducibility note

Probe 1 is deterministic on rustc 1.95.0 — the program has no
randomness or environment dependency.

Probe 2's *headline* (`error[E0609]: no field \`2\` on type
\`(i32, i32)\``), *caret annotation* (`unknown field`), and
*`= note:` line* (`available fields are: \`0\`, \`1\``) are
deterministic on this rustc release. The exact wording is
rustc-version-specific; the *shape* (a coded `error[E0609]` with
"no field N on type T" headline and a `= note:` enumerating the
legal fields) is grounded in the corpus pages cited above and is
stable. If a future rustc tweaks wording, the lesson's substantive
claims (tuples have fixed length; rustc knows the field names from
the type; the index must be a field name of the type) survive
unchanged; only the literal headline strings might need a refresh.

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
  The probe binds tuples and tuple fields with this exact shape;
  today does not extend the binding form.
- `019-type-annotation-i32` (accepted, load-bearing) — `name: TYPE`
  attaches a type annotation between the binding name and the `=`,
  and integer literals default to `i32` in the absence of other
  constraints. Today fills the `TYPE` slot with `(i32, i32)` and
  `(i32, f64)`. The "rustc infers `(i32, i32, i32)` from three
  integer literals" claim for `triple` is exactly lesson 019's
  inference rule applied to each operand.
- `029-unit-type` (accepted, load-bearing) — installs `()` as the
  *unit type*, the 0-arity tuple, and explicitly flags non-zero-
  arity tuples (`(i32, i32)`, `(f64, String)` etc.) as a deferred
  future move. Today is exactly that move; the prose framing
  "lesson 029's `()` is the 0-arity case of this family" comes
  straight from lesson 029's *What To Ignore For Now*.

Older supporting lessons (mentioned by id only, not load-bearing
for any individual claim today):

- `009-arithmetic-on-integers` — `+` between integers. Cited under
  *What To Ignore* to acknowledge that `pair.0 + pair.1` would work
  but is held off the probe.
- `011-println-positional-args` — `println!("{} ...", ...)`. Reused
  as-is for printing each field; today does not extend `println!`.
- `033-f64-floats` — `f64` as the default floating-point type and
  the `2.5` literal form. Used to license the `(i32, f64)` tuple's
  second field type.
- `070-rustc-explain` — the `--explain E####` trailer is a runnable
  instruction. Mentioned only to confirm the trailer in Probe 2 is
  decorated as expected on this run.
- `068-let-binding-scope`, `069-rustc-warnings`, `071-macro-
  invocation-syntax` — recent lessons on the same host and
  toolchain. Mentioned only to confirm the host environment is
  unchanged since lessons 029/019 were captured.

No trait-related lesson is cited. The brief explicitly excluded
trait machinery as a prerequisite for tuples, and the *What To
Ignore For Now* defers the upper-arity-limit story along with
traits.
