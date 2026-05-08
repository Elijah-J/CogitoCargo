## Evidence — 049-method-chaining

Audit appendix for `lessons/049-method-chaining.md`. Holds the
corpus-quote map, the toolchain string, the working probe transcript,
the parens-grouping bonus probe, the contrastive-probe justification,
and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/049-method-chaining.rs`); the parens-grouping bonus
  `.rs` is not committed — its transcript below is the artifact.

## Sources

### `output/docs/rust/reference/expressions/method-call-expr.md`

The Reference page for method-call expressions. Already cited in
lessons 040 and 041 as the canonical grammar source for the dot-form.
Today's citation reuses the same grammar production for its
*recursive* shape — the leading `Expression` of the production can
itself be a method-call expression, which is the formal license for
chaining. Two load-bearing spans, one corpus example.

Lines 8-10 (the *load-bearing* grammar production, repeated from
lessons 040/041 for ease of audit):

> **Syntax**
>
> [MethodCallExpression] → [Expression] . [PathExprSegment] (
> [CallParams]? )

The *load-bearing* word in this grammar is the leading `[Expression]`.
Lessons 040 and 041 read this as "the receiver is an expression"
without exercising the recursion. Today's lesson exercises it
directly: a `MethodCallExpression` *is* an `Expression` (because
method calls are listed under the Expression grammar), so the leading
`Expression` slot of the production accepts another method-call
expression as a sub-tree. Plugging
`Expression = MethodCallExpression(String::new())` and
`PathExprSegment = is_empty`, `CallParams = empty`, the production
yields `String::new().is_empty()` as a single legal
`MethodCallExpression`. The lesson body's "the receiver is *any
expression*" is the audience-level paraphrase of this grammar reading.
By the same recursion the grammar admits arbitrarily long chains
(`a.b().c().d()` etc.), which lesson 049's *What To Ignore For Now*
explicitly defers.

Note: the leading `[Expression]` slot also admits a *call expression*
(the `expr.call` form `f(args)`) and any other Expression form. In
today's working probe the inner expression is `String::new()`, which
is a *call expression* under `expr.call.syntax`
(`CallExpression → Expression ( CallParams? )` in `call-expr.md`
lines 8-10) — its `Expression` operand is the path expression
`String::new` and its `CallParams?` is empty. So `String::new()` is a
legal `Expression`, and plugging it into the `MethodCallExpression`
production produces today's chain. Lesson 049 does not surface
"path expression" / "call expression" terminology; it just observes
that "the result of one call can be the receiver of the next call."

Line 14 (the *load-bearing* introductory sentence, repeated from
lesson 040 for ease of audit):

> A *method call* consists of an expression (the *receiver*)
> followed by a single dot, an expression path segment, and a
> parenthesized expression-list.

The parenthetical "(the *receiver*)" is the corpus-level definition
of *receiver* as the leading expression. Today's lesson body cites
this label by name when it says "the receiver is *any expression*."
The lesson 040 evidence appendix already grounded this sentence; the
new reading today is that the leading expression *can itself be a
call*, which is licensed directly by the recursive grammar at line
8-10 plus the standard rule that a `MethodCallExpression` is itself
an `Expression`.

Lines 20-27 (the *corpus chaining example*, the most direct corpus
precedent for today's lesson):

> ```rust
> #![allow(unused)]
> fn main() {
> let pi: Result<f32, _> = "3.14".parse();
> let log_pi = pi.unwrap_or(1.0).log(2.72);
> assert!(1.14 < log_pi && log_pi < 1.15)
> }
> ```

The line `let log_pi = pi.unwrap_or(1.0).log(2.72);` is the corpus's
own working chaining example. It chains *two* method calls written
end-to-end — `.unwrap_or(1.0)` and `.log(2.72)` — with the value of
the first becoming the receiver of the second. Identical structure
to the lesson's `String::new().is_empty()`, with these differences:
(a) the corpus example's chain has *two* method calls, like today's
lesson; (b) the corpus example's receiver of the first method call
is a binding `pi`, where today's lesson's first method call is
`String::new()` (a call expression with no receiver — lesson 042);
(c) the corpus example's `.unwrap_or(1.0)` and `.log(2.72)` carry
arguments, while today's chain has empty argument lists. The shape
"value-producing call, then `.method(...)` directly on its result"
is structurally identical. The lesson body does not reproduce the
corpus example; it is direct corpus precedent for the move.

Calibration: the page's `expr.method.autoref-deref` clause (lines
29-35) describes the receiver-side autoref / autoderef machinery —
how the Reference picks among `T`, `&T`, `&mut T` candidate
receivers when looking up a method. Carrying over from lessons 040
and 041, this is *deferred* under *What To Ignore For Now*. Today's
lesson is consistent with that deferral: the working chain
`String::new().is_empty()` calls `&self` method `is_empty` on a
`String` value — the autoref step inserts `&` invisibly to satisfy
the `&self` signature — but the lesson does not surface that
mechanism. The reader sees a `String` value flowing into a method
that "works" without explicit `&`; the rule for *why* is reserved
for a future move.

### `output/docs/rust/alloc/string/struct.String.md`

The std-library page for `String`. Already cited in lesson 042 for
`String::new`. Today's citation pulls one new method, `is_empty`,
from the inherent-method block (the section before line 1572's
"Methods from Deref<Target = str>" header). One load-bearing span.

Lines 1311-1325 (the *load-bearing* `is_empty` definition and
example, in the inherent-method block for `String`):

> 1.0.0 (const: 1.87.0) ·
>
> #### pub const fn [is_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/stable/core/primitive.bool.html)
>
> Returns `true` if this `String` has a length of zero, and `false`
> otherwise.
>
> ##### Examples
>
> ```
> let mut v = String::new();
> assert!(v.is_empty());
>
> v.push('a');
> assert!(!v.is_empty());
> ```

Three load-bearing pieces:

1. The signature `pub const fn is_empty(&self) -> bool` is the formal
   corpus statement of the method's name, receiver-shape (`&self`),
   and return type (`bool`). The lesson body cites the signature's
   return-type half ("`is_empty` returns `true` when a `String` has
   length zero") without surfacing `&self` or `const fn`. Both are
   *What To Ignore For Now* items: the `&self` receiver carries the
   autoref machinery deferred from lesson 040, and `const fn` is the
   const-evaluation context, also deferred.
2. The one-line behavior description "Returns `true` if this
   `String` has a length of zero, and `false` otherwise" is the
   corpus statement of the method's semantics. The lesson body's
   "returns `true` when a `String` has length zero" is a direct
   audience-level paraphrase. The empirical claim that
   `String::new().is_empty()` evaluates to `true` rests on this
   sentence applied to `String::new()`'s output — a fresh empty
   `String`, length zero, per lesson 042's grounding of `String::new`.
3. The corpus example `let mut v = String::new(); assert!(v.is_empty());`
   is direct corpus precedent for the lesson's *two-step* form. The
   working probe's lines 3-4 read `let s: String = String::new(); let
   stepped: bool = s.is_empty();` — same structure as the corpus's
   first two lines, with two cosmetic differences: (a) the lesson
   uses an `: String` annotation (lesson 042 territory; the corpus
   relies on type inference); (b) the lesson uses `let stepped: bool
   = s.is_empty();` to bind the result, where the corpus uses
   `assert!(...)` to consume it. The semantic content is identical:
   both probe `is_empty()` on a fresh empty `String` and observe the
   `true` return.

The corpus example's third and fourth lines (`v.push('a');
assert!(!v.is_empty());`) demonstrate the negative case — after
mutation, `is_empty` returns `false`. The lesson does not exercise
the negative case (would require `String` mutation, which is
*What To Ignore For Now*).

Calibration: the same page also lists a *second* `is_empty` at line
1803 (signature `pub fn is_empty(&self) -> bool`, with example
`let s = ""; assert!(s.is_empty());`). That second entry sits under
the line-1572 "Methods from Deref<Target = str>" header — meaning
it is the `str` (string slice) method reachable on a `String` via
`Deref` coercion, not an inherent `String` method. The lesson uses
only the inherent `String::is_empty` at line 1313; the lesson's
`What To Ignore For Now` lists "`&str::is_empty`" as a deferred
item. The two methods produce the same boolean answer for an empty
`String`, so the empirical observation `chained = true` does not
distinguish them; the corpus citation specifically names the
inherent line-1313 entry to anchor the lesson on the *String*
method that lesson 042 already grounds.

### `output/docs/rust/book/ch05-03-method-syntax.md`

The Book chapter on method syntax. Already cited in lessons 040 and
041 as the audience-level introduction to method-call syntax.
Today's citation reuses the chapter's general framing — methods are
called from "after an instance" with the dot, parens, and arguments —
without pulling a new span. The Book chapter does not contain a
dedicated chaining example (the Book's first chaining example
appears later, in chapter 12 / chapter 13 territory in iterator
contexts). The lesson body cites this chapter implicitly via lessons
040 and 041 rather than re-quoting; the *new* corpus precedent for
today's chaining shape is the `pi.unwrap_or(1.0).log(2.72)` example
on `method-call-expr.md` lines 23-24, mapped above.

Calibration: the Book chapter's *Where's the `->` Operator?* section
(lines 135-180) introduces autoref/autoderef under the "automatic
referencing and dereferencing" name. The lesson's *What To Ignore
For Now* defers this — the lesson treats `String::new().is_empty()`
as "produces `true`" empirically, without explaining why a `&self`
method works on a `String` value without an explicit `&`. Same
deferral as lessons 040, 041.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/049-method-chaining.rs`.
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
fn main() {
    let chained: bool = String::new().is_empty();
    let s: String = String::new();
    let stepped: bool = s.is_empty();
    println!("chained = {chained}, stepped = {stepped}");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
chained = true, stepped = true
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent on success (lesson 001).
- `./demo` prints exactly one line, `chained = true, stepped =
  true`. Both bindings are `true`. The empirical claim "the chain
  produces the same value as the two-step form" rests on this
  observation: a single program contains both forms side-by-side and
  rustc and the runtime agree they evaluate to the same boolean
  value.
- The chain on line 2 (`String::new().is_empty()`) composes lesson
  042's `String::new()` with lesson 040's `.method()` form, with no
  intermediate `let`. The two-step form on lines 3-4 binds a name to
  the inner call's result first, then calls the method. Same
  underlying call sequence, different source arrangement. The
  println line shows the two values are equal.
- The lesson's main concept — "the receiver in
  `receiver.method(args)` is any expression" — is corroborated
  empirically: `String::new()` (a `Type::name()` expression with no
  receiver of its own) sits in the receiver slot of the next
  `.method()` call without ceremony, and the program compiles and
  runs.
- The collateral fact "`String::is_empty` returns `true` for an
  empty `String`" is corroborated by both `chained` and `stepped`
  reading `true` — the chain proves it indirectly (via the chained
  call), the two-step form proves it directly (the binding `s` holds
  a fresh empty `String`, then `s.is_empty()` returns `true`).
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Parens-grouping bonus probe

Source: working-probe shape with line 2 changed from `let chained:
bool = String::new().is_empty();` to `let chained: bool =
(String::new()).is_empty();` (explicit parens around the inner
call). Not committed; the transcript below is the artifact.
Captured 2026-05-07 in a fresh `mktemp -d` (filename `parens.rs`):

```text
--- cat parens.rs ---
fn main() {
    let with_parens: bool = (String::new()).is_empty();
    let without: bool = String::new().is_empty();
    println!("with_parens = {with_parens}, without = {without}");
}
--- rustc parens.rs ---
exit=0
--- ./parens ---
with_parens = true, without = true
exit=0
```

Notes (probe evidence — not corpus quotation):

- Both forms compile and produce identical output (`true`).
- The parens-explicit form `(String::new()).is_empty()` and the
  parens-implicit form `String::new().is_empty()` are interchangeable.
  Probe-level evidence that the chain *parses as*
  `(String::new()).is_empty()` — the implicit grouping the lesson
  body asserts. If `String::new().is_empty()` parsed any other way
  (e.g. as `String::new(().is_empty())`, which is gibberish, or as
  any other association), the with-parens form would have to differ
  in either compile-success or output. It does not.
- The grouping question is the audience-level concern: when reading
  `String::new().is_empty()` left-to-right, does the dot bind
  tighter than the parens, or do the parens close on `String::new()`
  first? The probe answers empirically: the parens close on
  `String::new()` first (left-associative chaining), matching the
  Reference grammar's recursive `Expression . PathExprSegment` form.
- The explicit-parens probe is not a *broken* contrast — both
  programs compile and run. It is a *grouping* probe, demonstrating
  that two source forms are equivalent. The README's contrastive-
  probe rule names "with X works, without X fails/differs" cases as
  the ones requiring a contrast probe; today's lesson's central
  claim (chain = two-step) is *equivalence*, demonstrated directly
  by both forms producing `true` in the working probe — see the
  contrastive-probe justification below.

### Contrastive-probe justification

The lesson does *not* make a "with X works, without X fails/differs"
claim. The central claim is *equivalence*: `String::new().is_empty()`
(the chain) and `let s = String::new(); s.is_empty()` (the two-step
form) produce the same value. This equivalence is demonstrated
*directly* in the working probe — both forms appear in the same
program, both bind to `true`, and the println shows them equal —
rather than via a broken contrast.

Per the README's *Audit Trail Depth* section: "when the move says
'with X this works, without X it fails/differs,' include a
negative/contrast probe or state why one is not needed." Today's
move says no such thing. So no broken contrast is required, and the
working probe alone suffices.

The orchestrator's prompt explicitly rejects the
`String::new.is_empty()` (no-parens-after-`new`) variant as too
heavy a contrast — it would surface function-pointer / function-item
machinery (the bare path `String::new` evaluates to a *function
item*, not a call) that lessons through 048 have not installed. The
parens-grouping bonus probe above is the closest thing to a
contrast: it shows that an *explicit* grouping produces the same
result, which is positive evidence for the implicit grouping the
lesson asserts. It is not strictly required by the README rule, but
it adds pedagogical clarity at low cost (no new diagnostic
machinery, no new grammar surface).

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 049. Older supporting lessons are mentioned
above by number only.

- **Lesson 040 (load-bearing for the dot-form receiver-as-expression
  reading)** — installed (a) the method-call syntax
  `receiver.method(args)` and (b) the Reference grammar
  `MethodCallExpression → Expression . PathExprSegment ( CallParams?
  )` with the leading `Expression` named the *receiver*. Lesson 049
  reuses (a) unchanged and exercises (b)'s recursion: the leading
  `Expression` is filled today with another call expression
  (`String::new()`) instead of the binding name `n` lesson 040
  used. The grammar is the same; the new exercise is plugging a
  call into the receiver slot. The lesson body's "the Reference
  grammar names the receiver an *Expression*" cites lesson 040 by
  name and reads the corpus production for its recursive shape.
- **Lesson 042 (load-bearing for `String::new()` as the inner call)**
  — installed (a) the no-receiver associated-function form
  `Type::name(args)`, (b) `String::new()` as the smallest concrete
  instance, returning a fresh empty `String`, (c) the `: String`
  annotation form. Lesson 049 reuses (a) and (b) verbatim — the
  inner expression of today's chain *is* `String::new()` from
  lesson 042, with the same return value (a fresh empty `String`).
  The empirical claim "`String::new().is_empty()` produces `true`"
  combines lesson 042's claim ("`String::new()` produces an empty
  `String`") with today's small new fact ("`is_empty` returns
  `true` for an empty `String`"). (c) is reused in the two-step
  form's `let s: String = String::new();`.
- **Lesson 019 (load-bearing for the `let name: TYPE = value;`
  shape)** — installed the type-annotation form. Today fills `TYPE`
  twice: once with `bool` (for `chained` and `stepped`, today's
  small extension to the annotation surface — `bool` was named in
  lesson 012 as a type but the annotation slot was not
  *exercised* with `bool` until now — same low-friction
  extension as cycles 042/045/047 used) and once with `String`
  (already extended in lesson 042). Same slot, no new annotation
  mechanism.
- **Lesson 012 (formatting / boolean values)** — installed `true`
  and `false` as the boolean values, and the rule that they print
  as the literal words `true`/`false` via `{}` placeholders. Today's
  output line "chained = true, stepped = true" relies on this:
  `chained` and `stepped` are `bool` values, formatted via the named
  placeholders `{chained}` and `{stepped}` (lesson 005 named-
  placeholder form), printed as the literal words. No new formatting
  mechanism.
- **Lessons 001, 002, 005** — `rustc file.rs` then `./name`; `fn
  main` is the entry point; `let name = value;` plus the named
  placeholder `{name}` for `println!`. Used unchanged.

## Older supporting lessons

Lesson 041 (qualified method-call form — lesson 049's chain
contrasts with the qualified-method shape only loosely. Lesson 041's
working probe pair `n.abs()` / `i32::abs(n)` showed two equivalent
*syntactic* shapes for the same call; today's chain shows two
equivalent *arrangements* for the same call sequence, with no new
syntax. Mentioned for the audit trail.).

Lesson 042 (no-receiver associated function — already named as a
load-bearing direct prerequisite above).

Lesson 040 evidence appendix's
`output/docs/rust/std/primitive.i32.md` citation is *not* reused
today; the new method-list source is `String`'s std-library page
(`alloc/string/struct.String.md`), already cited in lesson 042 for
`String::new`. Today extends the same page's citation surface to
include `String::is_empty`.

The `output/docs/rust/error_codes/E0425.md` citation reused across
lessons 005, 008, 040, 042, 043, 044 is *not* exercised today —
today's lesson does not have a broken probe firing E0425. The
parens-grouping bonus probe is *positive* evidence (both compile),
not an E-code probe.
