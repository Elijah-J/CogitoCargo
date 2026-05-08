# Evidence — 128-empty-match-arm-body

Audit appendix for `lessons/128-empty-match-arm-body.md`. Holds the
corpus-quote map, the toolchain string, the working-probe
transcript, the arm-type-mismatch contrast E0308 transcript, the
corroborating `Ordering::Less` transcript, and the
prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -srm` → `Darwin 24.5.0 x86_64`
- Probes captured 2026-05-08 in `/tmp/lesson128/` on this host.
- The working `.rs` source plus a single `.transcript.txt` are
  committed under
  `observations/128-empty-match-arm-body.{rs,transcript.txt}`.
  The contrast and corroborating sources are not committed; their
  full transcripts live inside the `.transcript.txt` artifact.

## Sources

### `output/docs/rust/reference/expressions/block-expr.md`

The Reference's block-expression page. The load-bearing span is
the `value-no-trailing-expr` rule (line 70-82):

> [[expr.block.value-no-trailing-expr]]
>
> When a block does not contain a [final operand](block-expr.md#r-expr.block.inner-attributes)
> and the block does not diverge, the block has [unit type](../types/tuple.md#r-type.tuple.unit)
> and [unit value](../types/tuple.md#r-type.tuple.unit).
>
> ```rust
> let x: () = {}; // Has no final operand.
> assert_eq!(x, ());
> let x: () = { 0u8; }; // As above.
> assert_eq!(x, ());
> ```

This is *the* corpus statement that `{}` has unit type `()` and
unit value `()`. The lesson body's "by lesson 029 its type is
`()` and its value is `()`" carries forward exactly this
Reference rule (lesson 029 grounded the same fact via the Book
ch03-02 and Reference types/tuple.md). Today's centered move
puts that block in a match arm body slot.

The lesson's *What To Ignore For Now* names "block expressions
with statements inside" (the `{ 0u8; }` form on line 79 of the
Reference) as deferred — today's empty block has no statements
at all.

### `output/docs/rust/reference/expressions/match-expr.md`

The Reference's match-expression page. Two load-bearing spans.

Line 18-20 (the grammar specifying that an arm body is an
expression):

> [MatchArms](match-expr.md#railroad-MatchArms) →
>     ( [MatchArm](match-expr.md#grammar-MatchArm) => ( [ExpressionWithoutBlock](../expressions.md#grammar-ExpressionWithoutBlock) , | [ExpressionWithBlock](../expressions.md#grammar-ExpressionWithBlock) ,? ) )*
>     [MatchArm](match-expr.md#grammar-MatchArm) => [Expression](../expressions.md#grammar-Expression) ,?

Plain reading: each arm has the shape `MatchArm => Expression`,
and the expression slot can be either an *ExpressionWithoutBlock*
(a comma-separated bare expression) or an *ExpressionWithBlock*
(a block-expression). A block-expression is exactly the form
`{}` follows; an empty block trailing a `=>` has the optional
trailing `,`. This is the corpus license for "an arm body can be
a block expression" — including the empty one. The lesson body's
"a match arm body can be any expression" is the audience-level
restatement.

Line 138 (the match-type rule):

> [[expr.match.type]]
>
> The type of the overall `match` expression is the [least upper
> bound](../type-coercions.md#r-coerce.least-upper-bound) of the
> individual match arms.

This is the Reference's load-bearing rule for "all arms must
share a type" (lesson 030 already installed this rule informally
via the Book quote). When all arms are `()`, the LUB is `()`;
today's working probe lands here. When one arm is `()` and
another is `i32`, no LUB exists and rustc rejects with E0308 —
today's contrast probe lands here.

The neighboring `[expr.match.empty]` rule (line 142) — "if there
are no match arms, then the match expression is diverging and
the type is `!`" — is named-deferred under *What To Ignore For
Now*. Today's match has three arms.

### `output/docs/rust/error_codes/E0308.md`

The error-code explainer for E0308 *mismatched types*. Already
cited across this run. Today's contrast triggers the
match-arms-incompatible sub-form of E0308 with the explicit type
pair `()` vs `integer`. The lesson body cites the E-code by
family ("the familiar E0308 shape, with the new sub-form")
without re-explaining E0308 from scratch.

### `output/repos/rmp/src/biguint/cmp.rs` (target use site)

The unlock target. Lines 18-33 read:

```rust
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

Line 24 is the centered use site: `Ordering::Equal => {}`. Today's
move opens that arm shape end-to-end. The neighboring features —
the named-binding pattern `ord =>` on line 25, `return ord` from
inside a function on line 25, and the for-loop body's reading of
the inner `match` as a statement (which is exactly today's
"match-of-`()`-arms used as a statement" composition) — compose
moves already installed (lesson 060 binding pattern, lesson 089
function `return`, today's `()`-typed match-as-statement). The
empty arm body is the last piece needed for line 24 to read
audience-level.

## Probes

### Working probe

Source: `experimental/eduratchet2/runs/rust-moves/observations/128-empty-match-arm-body.rs`.
Identical to *The Move* code block in the lesson.

Transcript captured at
`experimental/eduratchet2/runs/rust-moves/observations/128-empty-match-arm-body.transcript.txt`.
Headline:

```text
--- rustc demo.rs ---
exit=0
(no output)

--- ./demo ---
done
exit=0
```

Notes:

- `rustc demo.rs` is silent, exits 0 — the empty `{}` arm body is
  perfectly legal Rust; no warning, no error.
- `./demo` prints exactly one line: `done`. The `Ordering::Equal`
  arm fires (the scrutinee is `Ordering::Equal`), and its body
  `{}` runs no code. The other two arms (`println!("less")` and
  `println!("greater")`) are *not* entered. Control then reaches
  the unconditional `println!("done")` after the match.
- The whole match has type `()`. All three arms have type `()`:
  `{}` is `()` per block-expr.md:72; both `println!(...)` calls
  return `()` (Reference / std). The match is used as a *statement*
  here — no `let x = match ...`, just the match followed by the
  next statement. This is what makes the empty arm body load-bearing
  for rmp `cmp.rs:24`: the surrounding match there is also used as
  a statement (its `()`-typed arms feed the for-loop body's flow,
  not a binding).

### Contrast probe — arm-type mismatch

Not committed; transcript embedded in
`128-empty-match-arm-body.transcript.txt`.

Source:

```rust
use std::cmp::Ordering;

fn main() {
    let o: Ordering = Ordering::Equal;
    let x = match o {
        Ordering::Equal => {},
        Ordering::Less => 5,
        Ordering::Greater => 10,
    };
    println!("{}", x);
}
```

Bind the match to `let x`. The first arm produces `()` (`{}`).
The other two produce integer literals. Lesson 030's
all-arms-share-a-type rule (Reference: "the type of the overall
`match` is the least upper bound of the arms", match-expr.md:138)
fails here — `()` and any integer type have no common type.

```text
error[E0308]: `match` arms have incompatible types
 --> broken.rs:7:27
  |
5 |       let x = match o {
  |  _____________-
6 | |         Ordering::Equal => {},
  | |                            -- this is found to be of type `()`
7 | |         Ordering::Less => 5,
  | |                           ^ expected `()`, found integer
8 | |         Ordering::Greater => 10,
9 | |     };
  | |_____- `match` arms have incompatible types

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

Notes (probe evidence — not corpus quotation):

- Headline: `error[E0308]: \`match\` arms have incompatible types`.
  E-code is E0308 (same family as the wider E0308 family used
  throughout this run); the sub-form `\`match\` arms have
  incompatible types` is specific to the LUB-of-arms failure mode.
- The dashes underline `{}` with the inline label `this is found
  to be of type \`()\``. This is the load-bearing piece of
  evidence for the lesson's *centered* claim: rustc itself names
  `{}`'s type as `()`, in source-spans, when the arm is read in
  this context. No reader has to take the lesson's word for it.
- The caret underlines the literal `5` with `expected \`()\`,
  found integer`. Once the first arm fixes the match's type at
  `()`, every subsequent arm is checked against `()`. An integer
  literal has type `{integer}` (rustc's umbrella for an
  unconstrained integer literal), which is not `()`.
- The wider source span (`5 | let x = match o {` ... `9 | };`)
  brackets the whole `match` with the trailing `\`match\` arms
  have incompatible types` label, naming the whole construct as
  the failed unit.
- Exit code: 1; no executable produced.

This contrast probe is the necessary witness for the lesson's
contrastive claim "the empty `{}` body fixes the arm's type at
`()`." rustc itself states the rule via the inline label `this
is found to be of type \`()\``.

### Corroborating probe — `Ordering::Less` scrutinee

Not committed; transcript embedded in
`128-empty-match-arm-body.transcript.txt`.

Source:

```rust
use std::cmp::Ordering;

fn main() {
    let o: Ordering = Ordering::Less;
    match o {
        Ordering::Equal => {}
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("greater"),
    }
    println!("done");
}
```

Same as the working probe with the scrutinee changed from
`Ordering::Equal` to `Ordering::Less`. Witnesses that the empty
`Equal` arm fires *only* when the scrutinee is `Ordering::Equal`;
when the scrutinee is `Ordering::Less`, the `Less` arm fires and
prints `less`. After the match, `println!("done")` runs
unconditionally either way.

```text
--- rustc corroborator.rs ---
exit=0
(no output)

--- ./corroborator ---
less
done
exit=0
```

Notes:

- Compiles silently, exits 0. Same source structure as the
  working probe; only the scrutinee literal differs.
- Output is two lines: `less` (the body of the `Less` arm) and
  `done` (the post-match `println!`). Neither the `Equal` nor the
  `Greater` arm fires.
- Witnesses three things at once: (a) the empty arm is selective
  (only `Equal` triggers it), (b) when a non-empty arm fires its
  body actually runs (`println!("less")` is reached), (c) control
  always falls through to the post-match `println!("done")` — the
  match is a statement-position expression with type `()`, so the
  next statement is reached every time.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 128.

- **Lesson 029 (load-bearing for `{} : ()`)** — installed the
  unit type `()` and named the empty block `{}` as one of the
  three sources of a `()` value (the others being the literal
  `()` and a no-`->` function call). Today's move puts that
  empty block in a match arm body slot. The Reference's
  block-expr.md:72 rule cited above is the same rule lesson 029
  installed via the Book ch03-02 / Reference types/tuple.md
  pair; today rests on it.
- **Lesson 030 (load-bearing for arm-body-as-expression and
  all-arms-share-a-type)** — installed `match` with arms
  `pattern => arm_expression` and the rule that all arm
  expressions must share a type. Today's working probe lands the
  rule with all three arms at `()`; the contrast probe trips the
  rule with one `()` arm and two integer-literal arms. The
  Reference's match-expr.md:138 (least-upper-bound) and the
  arm-grammar at line 18-20 ground both directions.
- **Lesson 051 (load-bearing for the `Ordering` scrutinee
  shape)** — installed match on `Ordering`'s three variants.
  Today's working probe is the same three-arm match shape with
  one arm body changed from a string literal to `{}`. The `use
  std::cmp::Ordering;` import line (lesson 044) carries through
  unchanged.

## Older supporting lessons

- Lesson 031 (cited only — exhaustiveness): three variants,
  three arms, no `_` needed; reused via 051.
- Lesson 011 (cited only): `println!(...)` in arm bodies; each
  call has type `()`. The corpus license is the std `println!`
  macro returning `()`; reused unchanged from earlier lessons.
- Lesson 044 (cited only): `use std::cmp::Ordering;`.
- Lesson 019 (cited only): `let o: Ordering = Ordering::Equal;`
  annotation slot.
- Lesson 003 (cited only): four-part diagnostic map applied to
  the contrast E0308 transcript walk in *Try It*.
- Lessons 001, 002, 005 (cited only): compile/run shape, `fn
  main`, `let name = value;`. Used unchanged.
- Wider E0308 family (lessons 024, 025, 026, 028, 029, 045-048,
  061, 127): different sub-cases of E0308. Today's contrast
  probe is the match-arms-incompatible sub-case.

## Why this contrast probe (and not a "missing arm body" probe)

The lesson's centered claim is "an empty arm body `{}` is `()`,
and a match whose arms all type `()` is itself `()`." The natural
contrastive claim is "if you change one arm's type away from
`()`, the all-arms-share-a-type rule rejects it." That is the
contrast probe captured above.

A "missing arm body" probe — `Pattern => ,` or `Pattern =>` with
nothing after — would be a *parse* error, not a *type* error, and
would not exercise today's centered claim about `{}`'s type. The
arm-grammar (match-expr.md:18-20) requires *some* expression
after `=>`; this is a syntactic prerequisite, not a fact about
the unit type. So no separate "missing body" probe is captured.
