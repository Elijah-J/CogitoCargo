# Evidence — 045-shared-reference

Audit appendix for `lessons/045-shared-reference.md`. Holds the
corpus-quote map, the toolchain string, the full working and broken-
contrast probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/045-shared-reference.rs`); the broken-contrast `.rs`
  is not committed — its transcript below is the artifact.

## Sources

### `output/docs/rust/reference/types/pointer.md`

The Reference page for pointer types. Primary corpus source for the
*shared reference type* `&T`. Three load-bearing spans.

Lines 12-18 (the *References* section header and syntax):

> ## [References (`&` and `&mut`)](#references--and-mut)
>
> **Syntax**
>
> [ReferenceType] → & [Lifetime]? mut? [TypeNoBounds]

This is the formal grammar statement for the reference type. Dropping
the optional `Lifetime` and the optional `mut` (today's lesson
deliberately omits both — see *What To Ignore For Now*) leaves the
base form `& TypeNoBounds`, which is the lesson's `&i32` annotation.

Lines 22-30 (the *load-bearing* corpus statement of the shared-
reference type):

> ### [Shared references (`&`)](#shared-references-)
>
> Shared references point to memory which is owned by some other value.
>
> When a shared reference to a value is created, it prevents direct
> mutation of the value. [Interior mutability] provides an exception
> for this in certain circumstances. As the name suggests, any number
> of shared references to a value may exist. A shared reference type
> is written `&type`, or `&'a type` when you need to specify an
> explicit lifetime.

Two sentences are load-bearing for the lesson body:

1. "Shared references point to memory which is owned by some other
   value." — the lesson's "refers to `n`'s value without copying it
   and without taking it over" rephrases this. The word *shared* and
   the framing "the value is owned by some other value" come from
   here.
2. "A shared reference type is written `&type`, or `&'a type` when you
   need to specify an explicit lifetime." — corpus license for the
   lesson's `&i32` annotation. The `'a` lifetime form is explicitly
   deferred under *What To Ignore For Now*.

The "any number of shared references to a value may exist" claim is
the corpus statement that would license a future "multiple `&T`"
move; today's program has only one shared reference, and the lesson
defers that rule explicitly.

The "prevents direct mutation" sentence is the load-bearing claim
behind the future-move *borrow checker* deferral, but does not bite on
today's program because the program never attempts to mutate `n`. The
lesson does not surface this restriction.

Calibration: the page also covers `&mut T`, raw pointers (`*const`,
`*mut`), and smart pointers. All three are explicitly deferred.

### `output/docs/rust/reference/expressions/operator-expr.md`

The Reference page for operator expressions, *Borrow operators*
section. Primary corpus source for the prefix-`&` operator (the action
that produces the shared reference). Already cited in lessons 034 and
037 for unrelated arithmetic operators; today's citation is the
*Borrow operators* sub-section. Three load-bearing spans.

Lines 66-76 (the *Borrow operators* header and syntax):

> ## [Borrow operators](#borrow-operators)
>
> **Syntax**
>
> [BorrowExpression] →
>       ( & | && ) [Expression]
>     | ( & | && ) mut [Expression]
>     | ( & | && ) raw const [Expression]
>     | ( & | && ) raw mut [Expression]

Formal grammar for the prefix-`&` operator. Today's lesson uses only
the first alternative (`&` followed by an expression), with the `&&`
double-borrow form, the `mut` form, and the `raw` forms all deferred.
The lesson does not reproduce the grammar — only the spoken-English
shape "the prefix `&`, applied to a binding."

Line 80 (the operator's name):

> The `&` (shared borrow) and `&mut` (mutable borrow) operators are
> unary prefix operators.

This is the corpus statement that `&` is a *unary prefix operator*.
The lesson's "the prefix operator `&`" / "the prefix `&`" labels come
from here. The Reference's parenthetical "shared borrow" is
synonymous with "shared reference" in this run's vocabulary; the
lesson uses *shared reference* (matching the type-page section header)
rather than *shared borrow* to avoid splitting hairs between the
operator and the value it produces.

Line 84 (the *load-bearing* corpus statement of what `&value`
produces):

> When applied to a [place expression], this expressions produces a
> reference (pointer) to the location that the value refers to.

This is the canonical Reference statement that `&value` *produces a
reference*. The lesson's "the expression `&value` produces a value of
type `&T` referring to it" rephrases this. The Reference's
"reference (pointer)" parenthetical is the bridge from "reference" to
"pointer" — the Book chapter 4-02 also uses both words. The lesson
uses *reference* throughout to match the type-page's section header
and avoid surfacing the "pointer" word, which carries C/C++ baggage
this audience does not need today.

The phrase "place expression" is technical Reference vocabulary not
yet installed in this run. The lesson body does not surface it; the
spoken-English "applied to the binding `n`" approximates the case
that bites today (a `let`-bound name is a place expression per
expressions.md line 177). The lesson's broken-contrast probe uses
exactly that case (`&n` where `n` is a `let`-bound name); other
borrow contexts (temporary expressions, etc.) are deferred along with
the *borrow checker*.

Lines 100-114 (the corpus example):

> ```rust
> #![allow(unused)]
> fn main() {
> {
>     // a temporary with value 7 is created that lasts for this scope.
>     let shared_reference = &7;
> }
> let mut array = [-2, 3, 9];
> {
>     // Mutably borrows `array` for this scope.
>     // `array` may only be used through `mutable_reference`.
>     let mutable_reference = &mut array;
> }
> }
> ```

Direct corpus precedent for the `let shared_reference = &literal;`
shape. The lesson uses `let r: &i32 = &n;` where `n` is a previously
let-bound `i32` rather than a literal `7`; the operator action is the
same. The lesson explicitly does *not* use the literal form (`&7`)
because its operand would be a value-expression that creates a
*temporary* (per the Reference's `[temporary value]` and
`[expr.operator.borrow.temporary]` clause on line 96), and the
temporary-and-its-lifetime question is part of the future *lifetimes*
move.

### `output/docs/rust/book/ch04-02-references-and-borrowing.md`

The Book chapter on references and borrowing. Cited for the
audience-friendly framing of *what a reference is* and the canonical
*creating a reference* syntax. Two load-bearing spans.

Lines 7-11 (the audience-level definition):

> Instead, we can provide a reference to the `String` value. A
> reference is like a pointer in that it's an address we can follow
> to access the data stored at that address; that data is owned by
> some other variable. Unlike a pointer, a reference is guaranteed to
> point to a valid value of a particular type for the life of that
> reference.

The Book's "owned by some other variable" matches the Reference's
"owned by some other value." The lesson uses *refers to* rather than
*points to* to avoid the pointer baggage (consistent with the *What To
Ignore For Now* deferral of "smart pointers" and the `Deref` trait).
The Book's "guaranteed to point to a valid value" is the Rust-vs-C/C++
distinction; today's lesson does not surface it because no UAF/dangling
scenario is constructed. The audience-level prose "refers to `n`'s
value without copying it and without taking it over" combines both the
Reference's "owned by some other value" and the Book's "an address we
can follow to access the data stored at that address" without
unpacking either pointer or ownership.

Lines 66-68 (the *creating a reference* sentence):

> The `&s1` syntax lets us create a reference that *refers* to the
> value of `s1` but does not own it. Because the reference does not
> own it, the value it points to will not be dropped when the
> reference stops being used.

Plain-English statement of the lesson's main concept. The Book uses
`&s1` (with a `String` operand); the lesson uses `&n` (with an `i32`
operand) so ownership questions stay invisible. The Book's "*refers*
to the value of `s1`" is verbatim the verb the lesson uses ("refers
to `n`'s value"). The Book's "but does not own it" is the seed of the
ownership story the lesson explicitly defers.

Calibration: the Book chapter's running example wraps the reference
into a function call (`calculate_length(&s1)` with `fn
calculate_length(s: &String) -> usize`). Today's lesson uses the
reference only as a `let` binding, deferring the function-parameter
case to *What To Ignore For Now*. The Book chapter's later sub-section
*Mutable References* introduces `&mut`; deferred. The Book's note
about the dereference operator `*` (line 45-48) is paraphrased as
"`*r` reads through a reference" in the lesson's deferral list and
the body deliberately uses `{r}` (which looks through the reference)
rather than `*r`.

### `output/docs/rust/std/fmt/trait.Display.md`

The std-library page for the `Display` trait. New citation in this
run; line 542 is the load-bearing single span.

Line 542 (the *load-bearing* corpus statement that `{}` looks through
a shared reference):

> ### impl<T> [Display] for [&T] where T: [Display] + ?[Sized],

This is the canonical signature for the *blanket implementation* of
`Display` for shared references: for any type `T` that implements
`Display`, `&T` also implements `Display`. The lesson does *not*
install the `Display` trait, blanket implementations, or generics. It
installs the operational claim alone: "`{}` in `println!` looks
through a shared reference." The grounding for that operational
claim is this line — `&i32` formats via the same `Display`
implementation `i32` uses, because the blanket impl forwards.

The lesson body does not surface the trait name `Display`, the
generic `<T>`, or the `+ ?Sized` bound. Each is explicitly deferred:
the lesson uses the audience phrase "`{}` looks through `&T`,"
matching the visible behaviour in the working probe (the two
formatted slots produce the same `42`). The probe is the empirical
corroboration; this corpus line is the structural reason.

Calibration: the same page (line 546) has the parallel impl for
`&mut T`. Both blanket impls have been present since Rust 1.0.0 (see
the version markers above lines 540 and 544). The deferred *`&mut T`
mutable references* future move will inherit the same "`{}` looks
through" property without re-grounding.

### `output/docs/rust/error_codes/E0308.md`

The error-code explainer for E0308, "expected type did not match the
received type." Already cited in lessons 024, 025, 026, 028, and 033.
Reused here for the broken-contrast probe. One load-bearing span.

Line 4:

> Expected type did not match the received type.

The lesson does not re-explain E0308; it cites lessons 024, 025, 026,
028, and 033 for the E-code's prior installations. This is the family
connection — today's E0308 is a new sub-case (`expected &i32, found
i32`) of the same general type-mismatch rule, with the new specific
trigger being the missing prefix `&`.

The page's third example (lines 22-25) is structurally similar to
today's broken probe:

> ```rust
> let x: f32 = "Not a float";
> //     ---   ^^^^^^^^^^^^^ expected `f32`, found `&str`
> //     |
> //     expected due to this
> ```

The same caret-and-label shape rustc uses today (with `&i32` instead
of `f32`, and an integer instead of a string slice) is corpus-shown
here. The lesson does not reproduce this example.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/045-shared-reference.rs`.
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
    let n: i32 = 42;
    let r: &i32 = &n;
    println!("n = {n}, r = {r}");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
n = 42, r = 42
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent on success (lesson 001).
- `./demo` prints exactly one line: `n = 42, r = 42`. The two
  formatted slots produce the *same* text, which is the lesson's
  empirical content: the `{}` placeholder formats `&i32` and the
  `i32` it refers to identically. This is the working-side
  corroboration of the `Display` blanket impl for `&T` (cited above
  from `trait.Display.md` line 542) — the lesson's audience-level
  "`{}` looks through `&T`" claim.
- The program also corroborates the lesson's "the original `n` stays
  usable after `&n`" claim implicitly: `println!` reads `{n}` *after*
  `&n` was taken on line 3, and the program compiles and runs without
  diagnostic. The Reference's "any number of shared references to a
  value may exist" plus the (unsurfaced) `Copy` semantics of `i32`
  are the structural reasons; today's lesson does not surface either.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe

Source: working-probe shape with line 3 changed from
`let r: &i32 = &n;` to `let r: &i32 = n;` (the prefix `&` removed).
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
fn main() {
    let n: i32 = 42;
    let r: &i32 = n;
    println!("r = {r}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:3:19
  |
3 |     let r: &i32 = n;
  |            ----   ^ expected `&i32`, found `i32`
  |            |
  |            expected due to this
  |
help: consider borrowing here
  |
3 |     let r: &i32 = &n;
  |                   +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Notes (probe evidence — not corpus quotation):

- The headline reads `error[E0308]: mismatched types`. Same E-code
  lessons 024, 025, 026, 028, and 033 installed (each for a different
  type-annotation mismatch sub-case). The headline word is the
  generic *mismatched types* — rustc's general E0308 headline — not
  one of the more specialized variants ("`if` and `else` have
  incompatible types" from lesson 026, etc.). The specialized sub-
  case here is delivered via the caret label `expected ` then a
  backtick-quoted `&i32`, then `, found `, then a backtick-quoted
  `i32` — a literal pinpoint of the type-mismatch shape this lesson
  installs.
- The diagnostic has the four lesson-003 parts: headline + `-->`
  location (`broken.rs:3:19`) + source excerpt with caret + `help:`
  block, plus the `--explain E0308` trailer that lesson 003 named.
- The caret block is two-part: an underline `----` under the type
  annotation `&i32` labelled `expected due to this`, and a single-
  character caret `^` under the right-hand side `n` labelled
  `expected `&i32`, found `i32``. This is rustc's pinpoint of *both*
  ends of the mismatch — the annotation that set the expected type,
  and the value that delivered the wrong type. Probe evidence for
  the lesson's "the annotation `&i32` set the expected type; the
  right-hand side `n` delivered an `i32`" framing.
- The `help:` block reads literally `help: consider borrowing here`,
  followed by a source-diff suggestion that re-prints line 3 with `&`
  inserted in front of `n`, marked by a `+` underline below the
  inserted character. This is rustc's own statement of this lesson's
  move: to fix the mismatch, insert `&`. The lesson body quotes the
  diagnostic verbatim (in the *Try It* section's prediction block)
  and then describes it ("a source-diff inserting `&` to make `&n`").
- Exit code: 1. No executable was produced. The `ls after` shows only
  `broken.rs`, no `broken` binary.
- Most load-bearing observation: the `help:` block here suggests *the
  exact move this lesson installs* (insert `&`), unlike lesson 042's
  E0425 probe (no help) and lesson 043's E0425 probe (help suggested
  a future move, the `use` declaration). Today's probe is the
  cleanest possible: rustc's runtime statement of the fix matches the
  lesson's intended teaching.

The broken-contrast probe is necessary because the lesson makes a
contrastive claim ("with the `&` it works, without it the `T`-vs-`&T`
mismatch fires E0308"). The captured `expected `&i32`, found `i32``
caret label is the load-bearing piece of probe evidence: rustc itself
distinguishes `&i32` from `i32` as different types in its diagnostic.
The corpus-level grounding for the contrastive claim is the
combination of the Reference's *Borrow operators* clause (the prefix
`&` produces a reference) plus the *Shared references* type-page
section (a shared reference is its own type spelled `&type`); this
probe is the live transcript that ties those corpus statements to
the specific `&i32` vs. `i32` instance.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 045. Older supporting lessons are mentioned
above by number only.

- **Lesson 019 (load-bearing for shape)** — installed
  `let name: TYPE = value;` with `i32` as the example `TYPE`. Lesson
  045 reuses the same shape with `&i32` in the `TYPE` slot. Lesson
  019's body framing already generalizes the slot ("the `: TYPE` slot
  between `name` and `=`"); the lesson body explicitly notes the
  extension to a new `TYPE`. No new annotation mechanism — only a new
  type form `&T` flowing through the same slot.
- **Lesson 026 (load-bearing for E0308)** — installed E0308 for a
  type-annotation mismatch (between `if`/`else` arms). Lesson 024
  installed E0308 first (semicolon dropping a block to `()`); lessons
  025, 028, and 033 installed further E0308 sub-cases. Today's
  E0308 is a new sub-case (`expected &i32, found i32`) on the same
  general rule. The lesson body cites the E-code by family
  ("same E-code as lessons 024, 025, 026, 028, and 033") without
  re-teaching it. Lesson 026 is named as the *load-bearing* prior
  because it is the most recent E0308 lesson with the `expected X,
  found Y` caret-label shape that today's probe also produces.
- **Lesson 003 (load-bearing)** — diagnostics have headline + `-->`
  location + source excerpt with caret + optional `help:` lines.
  Lesson 045's broken-contrast walk uses that map without re-teaching
  it. The captured two-part caret (`----` under the annotation, `^`
  under the value) is consistent with lesson 003's general source-
  excerpt-with-caret framing — rustc occasionally underlines more
  than one source span in a single diagnostic to pinpoint multiple
  ends of a mismatch.
- **Lesson 005 (load-bearing for the named-placeholder `{r}`)** —
  installed `let name = value;` plus the named-placeholder
  `{name}` form for `println!`. Lesson 045 uses both unchanged. The
  empirical claim that `{r}` formats the underlying value is grounded
  in the working probe's identical output (`r = 42`) and structurally
  in the `Display` blanket impl for `&T` (cited above).
- **Lessons 001, 002** — `rustc file.rs` then `./name`; `fn main` is
  the entry point. Used unchanged.

## Older supporting lessons

Lessons 024, 025, 028, 033 (E0308 family connection — the broken-
contrast probe fires E0308, the same E-code these lessons installed
for different type-annotation mismatch sub-cases (024 first
installed it; 025/028/033 reused); not re-stated here beyond the
family connection through lesson 026).

Lesson 042 (extension-to-new-`TYPE` precedent — lesson 042 was the
last lesson to put a non-`i32` type in lesson 019's annotation slot
(`String`). Today's `&i32` is a parallel extension — same shape,
different type. Mentioned in the lesson body's prerequisite summary
for the audit trail; not re-stated beyond the parallel.).
