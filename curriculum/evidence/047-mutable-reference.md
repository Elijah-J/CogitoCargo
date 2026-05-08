# Evidence — 047-mutable-reference

Audit appendix for `lessons/047-mutable-reference.md`. Holds the
corpus-quote map, the toolchain string, the full working and
broken-contrast probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/047-mutable-reference.rs`); the broken-contrast `.rs`
  is not committed — its transcript below is the artifact.

## Sources

### `output/docs/rust/reference/types/pointer.md`

The Reference page for pointer/reference types. Already cited in
lessons 045 and 046 for the `&T` / *Shared references* section. Today's
citation is the same page for the *Mutable references* sub-section
(the type spec for `&mut T`). Three load-bearing spans.

Lines 12-18 (the *References* section header and grammar, repeated
from lesson 045 for ease of audit):

> ## [References (`&` and `&mut`)](#references--and-mut)
>
> **Syntax**
>
> [ReferenceType] → & [Lifetime]? mut? [TypeNoBounds]

Today's lesson uses the form `& mut TypeNoBounds` (omitting the
`Lifetime?` optional part — explicitly deferred under *What To Ignore
For Now*). The lesson 045 evidence appendix grounded the no-`mut` form
of this same grammar rule; today's lesson grounds the `mut`-bearing
form via the same grammar.

Lines 36-42 (the *load-bearing* corpus statement of the mutable
reference type):

> ### [Mutable references (`&mut`)](#mutable-references-mut)
>
> Mutable references point to memory which is owned by some other
> value. A mutable reference type is written `&mut type` or `&'a mut
> type`.

Two sentences are load-bearing for the lesson body:

1. "Mutable references point to memory which is owned by some other
   value." — same shape as lesson 045's shared-reference sentence
   (`Shared references point to memory which is owned by some other
   value`). The lesson body's "[`&mut n`] referring to `n`" rephrases
   this with the audience-friendly *refers to* verb (matching the
   Book's wording — see below).
2. "A mutable reference type is written `&mut type` or `&'a mut type`."
   — corpus license for the lesson's `&mut i32` annotation. The `'a`
   lifetime form is explicitly deferred.

Line 46 (the *load-bearing* corpus statement that distinguishes `&mut`
from `&` operationally):

> A mutable reference (that hasn't been borrowed) is the only way to
> access the value it points to, so is not `Copy`.

The phrase "the only way to access the value it points to" is the
audience-level corpus statement of write-access — through a `&mut T`
you can read or modify the value, in a way the value's original
binding cannot interleave with. The lesson body's "carries
write-access" condenses this. The Reference's "is not `Copy`" carries
the borrow-checker / single-mutable-borrow rule that today's lesson
explicitly defers (one `&mut T` at a time); the lesson does not
surface either word.

Calibration: the page also covers raw pointers (`*const`, `*mut`) and
smart pointers. Both deferred, same as lesson 045.

### `output/docs/rust/reference/expressions/operator-expr.md`

The Reference page for operator expressions. Already cited in lessons
034, 037, 045, 046 for various operator spans. Today's citation pulls
on three sub-sections that are all on this single page: *Borrow
operators* (the prefix `&mut` operator), *The dereference operator*
(the prefix `*` operator), and the *expr.deref.mut* clause that
licenses the deref-assign form.

Line 80 (the operators' name, repeated from lesson 045 for ease of
audit):

> The `&` (shared borrow) and `&mut` (mutable borrow) operators are
> unary prefix operators.

Today's load-bearing word is `&mut` — the parallel partner of lesson
045's `&`. The lesson body's "the prefix operator `&mut`" / "the
prefix `&mut`" labels come from here. The Reference's parenthetical
"mutable borrow" is the corpus name; the lesson uses *mutable
reference* (matching the type-page section header) for consistency
with the type vocabulary, the same way lesson 045 chose *shared
reference* over *shared borrow*.

Line 84 (the operator's result, repeated from lesson 045 for ease of
audit):

> When applied to a [place expression], this expressions produces a
> reference (pointer) to the location that the value refers to.

This is the corpus statement that `&mut value` *produces a reference*
— same sentence that licensed lesson 045's prefix `&`, now read for
the `&mut` half of the disjunction. The lesson body's "produces a
value of type `&mut i32`" rephrases this. The lesson 045 appendix
already noted that "place expression" is technical Reference
vocabulary; today's lesson likewise approximates with "the binding
`n`" — `n` is a `let`-bound name, which is a place expression.

Line 92 (the *load-bearing* corpus statement that `&mut`'s operand
must be a *mutable* place):

> `&mut` evaluates its operand in a mutable place expression context.

This is the corpus statement behind the lesson body's "only a `mut`-
bound place can be mutably borrowed." The Reference's "mutable place
expression context" is shorthand for "a place that is allowed to be
written to" — and the rule from lesson 006 says that a binding is
allowed to be written to iff it was declared with `let mut`. The
lesson does not surface the technical phrase; it cites lesson 006 by
name and asserts the consequence.

Lines 192-212 (the *Dereference operator* section — the load-bearing
corpus source for the `*` operator and the deref-assign form):

> ## [The dereference operator](#the-dereference-operator)
>
> **Syntax**
>
> [DereferenceExpression] → * [Expression]
>
> The `*` (dereference) operator is also a unary prefix operator.
>
> When applied to a [pointer] it denotes the pointed-to location.
>
> If the expression is of type `&mut T` or `*mut T`, and is either a
> local variable, a (nested) field of a local variable or is a mutable
> [place expression], then the resulting memory location can be
> assigned to.

Three load-bearing sentences:

1. "The `*` (dereference) operator is also a unary prefix operator."
   — corpus license for the lesson's "the prefix `*`" label. The word
   *dereference* is the corpus name; the lesson body uses
   *deref-assign* only for the specific `*r = newval;` *form on the
   left of `=`*, to flag the lesson's deliberate scope (write-through
   only, read-through deferred).
2. "When applied to a [pointer] it denotes the pointed-to location."
   — corpus statement that `*r` names the place `r` refers to. The
   lesson body's "writes the right-hand value *through* `r` into the
   place `r` refers to" is the audience-level rephrase. The
   Reference's *pointer* word is the umbrella for both `&T` /
   `&mut T` (references) and `*const T` / `*mut T` (raw pointers); the
   lesson uses *reference* throughout, consistent with lesson 045's
   choice.
3. "If the expression is of type `&mut T` ... the resulting memory
   location can be assigned to." — the *load-bearing* corpus statement
   of the deref-assign mechanic. This is what licenses the lesson's
   `*r = 99;` line. The Reference says it explicitly: when `r` is of
   type `&mut T`, the place `*r` denotes is *assignable*. Combined
   with the prior "the dereference operator" syntax rule, this is the
   corpus license for the whole `*r = newval;` form. The lesson body
   does not reproduce the conditional shape; it asserts the
   consequence operationally.

The Reference's *expr.deref.mut* sentence also names "or `*mut T`" and
"or is a mutable [place expression]" — the raw-pointer case and the
nested-field case. Both are deferred. The lesson uses only the simplest
case: a `let`-bound `r: &mut i32`.

Lines 220-231 (the corpus example):

> On non-pointer types `*x` is equivalent to `*std::ops::Deref::deref(&x)`
> in an [immutable place expression context] and `*std::ops::DerefMut::deref_mut(&mut x)`
> in a mutable place expression context.
>
> ```rust
> #![allow(unused)]
> fn main() {
> let x = &7;
> assert_eq!(*x, 7);
> let y = &mut 9;
> *y = 11;
> assert_eq!(*y, 11);
> }
> ```

Direct corpus precedent for the lesson's `*r = newval;` write-through
form — line 228 reads `*y = 11;` where `y` was bound to `&mut 9`.
Identical structure to the lesson's `*r = 99;` after `let r: &mut i32
= &mut n;`, with the difference that the corpus example uses an
inline literal `&mut 9` (creating a temporary) where the lesson borrows
a previously `let mut`-bound `n` (avoiding the temporary-and-its-
lifetime question that lesson 045 already deferred). The corpus's
`assert_eq!(*y, 11)` line uses `*y` as a *read-through* — explicitly
deferred under *What To Ignore For Now*.

The "non-pointer types" sentence that names `DerefMut::deref_mut` is
the trait-machinery sentence the lesson explicitly defers. The
working probe uses a primitive `&mut i32` (a pointer type, by the
Reference's terminology), so the `DerefMut` desugaring does not bite.

Calibration: the page also covers `&&` double-borrow, `&raw const`,
`&raw mut`, and `*const T`. All deferred.

### `output/docs/rust/book/ch04-02-references-and-borrowing.md`

The Book chapter on references and borrowing. Already cited in lessons
045 and 046. Today's citation is the *Mutable References* sub-section
(the canonical Book example of `&mut T`). One load-bearing span.

Lines 141-163 (the *Mutable References* introductory example):

> ### [Mutable References](#mutable-references)
>
> We can fix the code from Listing 4-6 to allow us to modify a borrowed
> value with just a few small tweaks that use, instead, a *mutable
> reference*:
>
> Filename: src/main.rs
>
> ```rust
> fn main() {
>     let mut s = String::from("hello");
>
>     change(&mut s);
> }
>
> fn change(some_string: &mut String) {
>     some_string.push_str(", world");
> }
> ```
>
> First, we change `s` to be `mut`. Then, we create a mutable reference
> with `&mut s` where we call the `change` function and update the
> function signature to accept a mutable reference with `some_string:
> &mut String`. This makes it very clear that the `change` function
> will mutate the value it borrows.

Three load-bearing structural facts mirror today's lesson:

1. The Book's "we change `s` to be `mut`" matches the lesson's "the
   source is `let mut n` (lesson 006): only a `mut`-bound place can
   be mutably borrowed." Same precondition.
2. The Book's "we create a mutable reference with `&mut s`" matches
   the lesson's "the prefix `&mut` applied to the binding `n` produces
   a value of type `&mut i32`." The Book uses `&mut s` on a `String`;
   the lesson uses `&mut n` on an `i32` so the `Copy`-vs-non-`Copy`
   question stays invisible. Same operator action.
3. The Book uses the mutable reference as a function argument
   (`change(&mut s)` calling `fn change(some_string: &mut String)`);
   today's lesson uses it as the right-hand side of a `let`. The
   *function-parameter* extension is explicitly named as the natural
   next move under *What To Ignore For Now*. The lesson body's "you
   can write *through* a mutable reference" is the operational
   summary; the Book's `some_string.push_str(", world")` is a method
   call demonstrating writing-through, where the lesson uses the
   primitive `*r = newval;` form to keep autoref / methods off-stage.

Calibration: the Book chapter goes on (lines 165-205) to describe the
borrow checker's restrictions (E0499 — at most one `&mut`; E0502 —
no mixing `&` and `&mut`). Both are deferred under *What To Ignore
For Now*. The lesson body uses a single-`&mut`-at-a-time program where
neither rule visibly bites. The dangling-reference / lifetimes
sub-section (lines 312+) is also deferred.

The Book's earlier *References and Borrowing* prose (lines 7-11 and
66-68) was already mapped in lesson 045's evidence appendix; today's
lesson cites lesson 045 by name for the audience-level definition of
*reference* rather than re-quoting.

### `output/docs/rust/error_codes/E0308.md`

The error-code explainer for E0308, "expected type did not match the
received type." Already cited in lessons 024, 025, 026, 028, 033, 045,
and 046. Reused here for the broken-contrast probe. One load-bearing
span.

Line 4 (the canonical one-liner, repeated from lesson 045 for ease of
audit):

> Expected type did not match the received type.

Today's E0308 is yet another sub-case of the same general type-
mismatch rule. The new specific sub-case is `&i32` (shared reference)
vs. `&mut i32` (mutable reference) — the kinds of references differ
in the `mut` modifier. The lesson body cites the E-code by family
("same E-code as lesson 045's broken contrast") rather than
re-explaining E0308.

The page's three example errors (lines 11-25) cover function-argument
type mismatch (`plus_one("Not a number")`), `if`-condition type
mismatch (`if "Not a bool"`), and `let` annotation type mismatch (`let
x: f32 = "Not a float"`). Today's broken probe is structurally the
*third* form — `let r: &mut i32 = &n;`, a `let`-annotation mismatch —
matching the same caret-and-label shape rustc shows on the explainer
page. The lesson does not reproduce this example.

Calibration: rustc's E0308 diagnostics for *reference-type* mismatches
carry a specialized caret label (`types differ in mutability`) and a
specialized `note:` block (`expected mutable reference &mut _ / found
reference &_`) that the explainer page does not list. These are
*probe* evidence, captured in the broken-contrast transcript below.
The lesson body labels them as such in *What Changed* and *Check
Yourself*.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/047-mutable-reference.rs`.
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
    let mut n: i32 = 1;
    let r: &mut i32 = &mut n;
    *r = 99;
    println!("n = {n}");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
n = 99
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent on success (lesson 001).
- `./demo` prints exactly one line, `n = 99`. The original binding
  `n` was `let mut`-bound to `1` on line 2; line 4 wrote `99` *through*
  the mutable reference `r`; line 5 read `n` and got `99`. The write
  on line 4 is the load-bearing empirical observation: the
  through-the-reference assignment changed the value that the original
  binding `n` reads.
- Three pieces composed: lesson 045's prefix `&` (here in its `&mut`
  form), lesson 045's `&T` annotation slot (here filled with `&mut i32`),
  and the new deref-assign form `*r = newval;` (the `*` operator on
  the *left* of `=`).
- The program also corroborates the lesson's "both prerequisites had
  to line up" framing: removing the `mut` from line 2 (`let n: i32 =
  1;`) would fire E0596 (deferred); replacing line 3's right-hand `&mut`
  with `&` would fire E0308 (the broken-contrast probe below).
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe

Source: working-probe shape with line 3 changed from `let r: &mut i32
= &mut n;` to `let r: &mut i32 = &n;` (the `mut` removed from the
right-hand side, leaving a *shared* borrow into a *mutable*-reference
slot). Not committed; the transcript below is the artifact. Captured
2026-05-07 in a fresh `mktemp -d` (filename `broken.rs`):

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
broken.rs
--- cat broken.rs ---
fn main() {
    let mut n: i32 = 1;
    let r: &mut i32 = &n;
    *r = 99;
    println!("n = {n}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:3:23
  |
3 |     let r: &mut i32 = &n;
  |            --------   ^^ types differ in mutability
  |            |
  |            expected due to this
  |
  = note: expected mutable reference `&mut _`
                     found reference `&_`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Notes (probe evidence — not corpus quotation):

- The headline reads `error[E0308]: mismatched types`. Same E-code
  lessons 024, 025, 026, 028, 033, 045, and 046 captured. Today is
  yet another sub-case of the same general type-mismatch rule. The
  lesson body cites the E-code by family ("same E-code as lesson 045's
  broken contrast") without re-teaching it.
- The diagnostic has the lesson-003 parts: headline + `-->` location
  (`broken.rs:3:23`) + source excerpt with caret + `note:` block +
  `--explain E0308` trailer. No `help:` block in this case (rustc
  does not auto-suggest `&mut n`; it cannot tell whether the user
  intended a shared or a mutable borrow once both annotations
  disagree). Different from lessons 045 and 046, which had `help:
  consider borrowing here`.
- The caret block is two-part, matching lesson 045's shape: an
  underline `--------` under the type annotation `&mut i32` labelled
  `expected due to this`, and a two-character caret `^^` under the
  right-hand side `&n` labelled `types differ in mutability`. The
  caret label is the load-bearing piece of probe evidence: rustc
  itself names *mutability* as the dimension of difference between
  the two reference types — exactly the distinction the lesson
  installs.
- The `= note:` block is even more precise: `expected mutable
  reference &mut _` over `found reference &_`. The wildcards `_`
  stand for the unspecified referent type (here `i32` on both sides);
  rustc collapses them because the mismatch is in the *kind* of
  reference, not the referent type. Probe evidence that `&T` and
  `&mut T` are two distinct families of types, parameterized over the
  same `T`. The lesson body quotes this `note:` block verbatim in
  *Try It* and the *Check Yourself* answer.
- Exit code: 1. No executable was produced. The `ls after` shows only
  `broken.rs`.
- The broken probe stops with exactly one error. There is no E0594
  ("cannot assign through `&` reference") secondary, even though line
  4 is `*r = 99;` and (in a hypothetical world where line 3
  succeeded) `r` would have been a `&i32`. rustc bails out at the
  first type error in the `let`, so the lesson's `## What To Ignore
  For Now` deferral of E0594 is consistent with the captured
  transcript: E0594 simply does not surface here.
- The broken probe also does not fire E0596 ("cannot borrow as
  mutable"), because the source binding *is* `let mut n` and the
  expression on the right side is `&n` (a shared borrow, which has
  no `let mut` requirement). E0596 would fire on a different broken
  variant — `let n: i32 = 1; let r: &mut i32 = &mut n;` — which is
  not today's contrast. Per orchestrator instruction, that variant is
  deferred to a future cycle.

The broken-contrast probe is necessary because the lesson makes a
contrastive claim ("`&T` and `&mut T` are distinct types"). The
captured `types differ in mutability` caret label and `expected
mutable reference &mut _ / found reference &_` `note:` block are the
load-bearing pieces of probe evidence: rustc itself distinguishes
`&i32` from `&mut i32` and pinpoints *mutability* as the dimension.
The corpus-level grounding for the contrastive claim is the
combination of the Reference's *Shared references* and *Mutable
references* sub-sections (two distinct sub-headers under one *References*
parent), each with its own grammar production sharing the
`& Lifetime? mut? TypeNoBounds` schema where the optional `mut`
flips the kind.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 047. Older supporting lessons are mentioned
above by number only.

- **Lesson 006 (load-bearing for the `let mut` precondition)** —
  installed `let mut name = value;` and the rule that bindings are
  immutable by default; `mut` makes them reassignable. Lesson 047
  composes 006 with today's `&mut`: the source binding *must* be
  `let mut`-bound, otherwise the prefix `&mut` cannot apply (the
  Reference's *expr.operator.borrow.mut* clause: "`&mut` evaluates
  its operand in a mutable place expression context"). The lesson
  body cites lesson 006 by name and asserts the consequence
  ("only a `mut`-bound place can be mutably borrowed"). The
  contrast variant that would fire E0596 ("cannot borrow as mutable,
  as ... is not declared as mutable") is explicitly deferred.
- **Lesson 019 (load-bearing for the type-annotation slot)** —
  installed `let name: TYPE = value;` with `i32` as the example
  `TYPE`. Lesson 045 extended `TYPE` to `&i32`. Today extends to
  `&mut i32`. Same slot, new type form. No new annotation mechanism.
- **Lesson 045 (load-bearing for the reference-type machinery)** —
  installed (a) the *shared reference type* `&T`, (b) the prefix
  `&` operator that builds a `&T` value from a `T` value, (c) the
  distinction `T` vs. `&T` and its E0308 *mismatched types* sub-case,
  (d) the operational vocabulary of "reference" / "refers to" /
  "without copying or owning." All four carry over by analogy to
  today's `&mut T`, with the parallel grammar (the same Reference
  *Borrow operators* span yields both forms via the `& | &&` and
  `mut?` alternations). The new bits today are the `mut` modifier on
  both the type and the operator, plus the deref-assign form.
- **Lesson 003 (load-bearing for diagnostic shape)** — diagnostics
  have headline + `-->` location + source excerpt with caret +
  optional `help:` / `note:` lines + `--explain` trailer. Today's
  broken-contrast walk uses that map without re-teaching it. The
  captured `= note: expected mutable reference &mut _ / found
  reference &_` is consistent with lesson 003's general source-
  excerpt shape — rustc occasionally appends a `note:` block to
  E0308 to clarify the precise nature of a type mismatch.
- **Lesson 005 (load-bearing for the named-placeholder `{n}`)** —
  installed `let name = value;` plus the named-placeholder `{name}`
  form for `println!`. Lesson 047 uses both unchanged inside `main`.
  The empirical claim that the final `println!` prints `99` (not the
  original `1`) is grounded directly in the working probe.
- **Lessons 001, 002** — `rustc file.rs` then `./name`; `fn main` is
  the entry point. Used unchanged.

## Older supporting lessons

Lessons 024, 025, 026, 028, 033, 046 (E0308 family connection — the
broken-contrast probe fires E0308, the same E-code these lessons
installed for different type-annotation mismatch sub-cases. Not
re-stated here beyond the family connection through lesson 045.).

Lesson 045 (parallel structure — today's lesson is the *mutable* twin
of 045's *shared* reference. The same Reference *Borrow operators*
section, the same Reference *References* type-page section header, and
the same Book chapter 4-02 are cited; today's grounding pulls from
the parallel `mut`-bearing sentences and grammar rules). Already named
as a load-bearing direct prerequisite above.

Lesson 046 (parallel-extension precedent — lesson 046 was the most
recent reference-type lesson, extending lesson 045 to a function-
parameter context. Today's lesson stays in `let` context but adds the
write-through capability instead. Both are post-045 expansions of the
reference-type surface; mentioned for the audit trail.).
