# Evidence — 046-reference-function-parameter

Audit appendix for `lessons/046-reference-function-parameter.md`. Holds
the corpus-quote map, the toolchain string, the full working and
broken-contrast probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/046-reference-function-parameter.rs`); the
  broken-contrast `.rs` is not committed — its transcript below is the
  artifact.

## Sources

### `output/docs/rust/book/ch04-02-references-and-borrowing.md`

The Book chapter on references and borrowing. Primary corpus source
for the pedagogical move "a function whose parameter is a reference
reads through the caller's value without taking it." Already cited in
lesson 045 for the audience-level definition of *reference* and the
"`&s1` syntax lets us create a reference" sentence; today's citation
is the section that uses references *as function parameters*.

Lines 13-30 (the load-bearing example):

> Here is how you would define and use a `calculate_length` function
> that has a reference to an object as a parameter instead of taking
> ownership of the value:
>
> Filename: src/main.rs
>
> ```rust
> fn main() {
>     let s1 = String::from("hello");
>
>     let len = calculate_length(&s1);
>
>     println!("The length of '{s1}' is {len}.");
> }
>
> fn calculate_length(s: &String) -> usize {
>     s.len()
> }
> ```

This is the canonical Book example of a function with a reference
parameter. Three load-bearing structural facts mirror today's lesson
exactly:

1. The signature spells the parameter type with `&`: `s: &String` in
   the Book; `r: &i32` in the lesson. Same shape, different referent
   type. The Book uses `String` so its surrounding chapter can
   motivate ownership; today's lesson uses `i32` to keep ownership
   off-stage (deferred under *What To Ignore For Now*).
2. The call site builds the argument with prefix-`&`:
   `calculate_length(&s1)` in the Book; `show(&n)` in the lesson.
3. The caller's binding remains usable in the next statement:
   `println!("The length of '{s1}' is {len}.");` reads `s1` *after*
   the call. Today's lesson does the same with
   `println!("n is still: {n}");`. The Book does not surface the
   ownership reason in this snippet; today's lesson does not surface
   it either, and the empirical evidence is the working probe (the
   second `println!` succeeds and prints `42`).

Lines 32-36 (the load-bearing prose statement):

> First, notice that all the tuple code in the variable declaration
> and the function return value is gone. Second, note that we pass
> `&s1` into `calculate_length` and, in its definition, we take
> `&String` rather than `String`. These ampersands represent
> references, and they allow you to refer to some value without taking
> ownership of it.

Two sentences are load-bearing for the lesson body:

- "we pass `&s1` into `calculate_length` and, in its definition, we
  take `&String` rather than `String`" — the corpus statement of
  *both* sides of the lesson-020 type-match rule applied to a
  reference-typed parameter: the call site passes `&value`, and the
  signature accepts `&T`. The lesson uses this exact pattern with `n`
  and `&i32` in place of `s1` and `&String`.
- "These ampersands represent references, and they allow you to refer
  to some value without taking ownership of it" — the audience-level
  framing of the lesson's "reads through its argument rather than
  taking it over" sentence. The Book's "without taking ownership of
  it" hints at the deferred ownership story; the lesson softens to
  "reads through its argument rather than taking it over" so the word
  *ownership* does not surface in the main body.

Lines 88-93 (the corpus statement that the caller's value survives the
call):

> The scope in which the variable `s` is valid is the same as any
> function parameter's scope, but the value pointed to by the
> reference is not dropped when `s` stops being used, because `s`
> doesn't have ownership. When functions have references as
> parameters instead of the actual values, we won't need to return
> the values in order to give back ownership, because we never had
> ownership.

This is the corpus statement that licenses the lesson's empirical
claim. The lesson does *not* reproduce the structural reasoning ("`s`
doesn't have ownership"); it cites only the operational consequence
("the caller's binding is still usable"). The structural reason is
explicitly deferred under *What To Ignore For Now* — *Why `n` is still
usable after `show(&n)` returns* — and named as ownership only inside
that deferral.

Calibration: the Book chapter's `calculate_length` returns `usize`
(via `-> usize`) so it can demonstrate why the tuple-return version
of the previous chapter is unnecessary. Today's `show` returns nothing
(no `->` arrow, lesson 020's no-return shape) because the
return-types-with-references question is its own future move. The
Book's `s.len()` body inside `calculate_length` is a method call on
`&String` that involves autoref; today's `println!("...{r}")` body
side-steps that by formatting the reference directly via the
`Display` blanket impl for `&T` (lesson 045's "`{}` looks through
`&T`" claim). The Book's later sub-section *Mutable References*
introduces `&mut T` parameters; deferred.

### `output/docs/rust/reference/items/functions.md`

The Reference's spec for function items. Already cited in lesson 036
for the parameter-list grammar. Today's citation is the same spec, for
the rule that the parameter-type slot accepts any [Type].

Lines 22-34 (the function-parameter grammar):

> [FunctionParameters] →
>       [SelfParam] ,?
>     | ( [SelfParam] , )? [FunctionParam] ( , [FunctionParam] )* ,?
>
> [SelfParam] → [OuterAttribute]* ( [ShorthandSelf] | [TypedSelf] )
>
> [ShorthandSelf] → ( & | & [Lifetime] )? mut? self
>
> [TypedSelf] → mut? self : [Type]
>
> [FunctionParam] → [OuterAttribute]* ( [FunctionParamPattern] | ... | [Type]​ )
>
> [FunctionParamPattern] → [PatternNoTopAlt] : ( [Type] | ... )

Load-bearing pieces:

- `FunctionParamPattern → PatternNoTopAlt : ( Type | ... )` is the
  formal grammar for the lesson-020 `name: TYPE` shape. `Type` here
  is the same nonterminal that appears elsewhere in the Reference,
  including in the `ReferenceType` rule cited by lesson 045
  (`& Lifetime? mut? TypeNoBounds`). Putting those two grammar rules
  together: `&i32` is a valid `Type`, so `r: &i32` is a valid
  `FunctionParam`. This is the formal license for today's lesson.
- The `SelfParam` / `ShorthandSelf` / `TypedSelf` rules are explicitly
  deferred (method receivers — `&self` / `&mut self`). Today's
  function is a free function with no receiver.

Lines 46-48 (the *signature* sentence):

> Functions may declare a set of *input* [*variables*] as parameters,
> through which the caller passes arguments into the function, and
> the *output* [*type*] of the value the function will return to its
> caller on completion.

Corpus framing of the parameter-vs-argument distinction lesson 020
already installed; reused here without re-grounding. The lesson's
"the call site supplies a value of that type … as the argument"
draws on this.

Calibration: the Reference page covers function generics, where
clauses, function qualifiers (`const async safe unsafe extern`),
variadic functions, and ABIs. All deferred.

### `output/docs/rust/reference/types/pointer.md`

The Reference page for pointer/reference types. Already cited in
lesson 045 for the *Shared references* type grammar. Today's citation
is the same page, used for one additional load-bearing fact: the
shared reference grammar `& Lifetime? mut? TypeNoBounds` is the same
grammar that fills the parameter-`Type` slot (per the cross-link in
the `functions.md` evidence above).

Line 18 (the reference-type grammar):

> [ReferenceType] → & [Lifetime]? mut? [TypeNoBounds]

This is the grammar rule for `&i32` itself. Combined with the
`functions.md` rule that the parameter slot accepts any [Type], it is
the formal license for `r: &i32` as a parameter declaration.

The Reference does not repeat the *Shared references* prose that
lesson 045 cited (lines 22-30); the lesson 045 evidence appendix
already maps it. Today's lesson's body cites lesson 045 by name for
that prose rather than re-quoting.

Calibration: today's lesson omits the `Lifetime?` and `mut?` optional
parts of the grammar (deferred — *Lifetimes* and *`&mut T`*).

### `output/docs/rust/reference/expressions/operator-expr.md`

The Reference page for operator expressions, *Borrow operators*
section. Already cited in lesson 045 for the prefix-`&` operator.
Today's lesson cites lesson 045 by name for that grounding rather
than re-quoting; the call-site `&n` is the same operator from lesson
045 in a new context (an argument expression instead of the right of
a `let`).

Line 84 (the load-bearing claim, repeated from lesson 045 for ease of
audit):

> When applied to a [place expression], this expressions produces a
> reference (pointer) to the location that the value refers to.

The lesson's `&n` at the call site is exactly this case (`n` is a
let-bound name, which is a place expression per
`expressions.md` line 177 — same connection lesson 045's appendix
made). The result is a value of type `&i32` that is then supplied as
the argument to `show`.

Calibration: the Reference page also covers the `&mut`, `&&`, and
`&raw` borrow operators. All three are deferred. The lesson body uses
only the bare prefix-`&` shape.

### `output/docs/rust/error_codes/E0308.md`

The error-code explainer for E0308, "expected type did not match the
received type." Already cited in lessons 024, 025, 026, 028, 033, and
045. Reused here for the broken-contrast probe. One load-bearing
span.

Lines 4 and 29-32:

> Expected type did not match the received type.
>
> ...
>
> This error occurs when an expression was used in a place where the
> compiler expected an expression of a different type. It can occur in
> several cases, the most common being when calling a function and
> passing an argument which has a different type than the matching
> type in the function declaration.

Two load-bearing spans:

- Line 4 is the canonical one-line summary of E0308 (already cited in
  earlier lessons).
- Lines 29-32 are the corpus statement of the *exact* sub-case today's
  broken probe instantiates: "the most common [case] being when
  calling a function and passing an argument which has a different
  type than the matching type in the function declaration." Today's
  broken probe is `show(n)` where `show` declares `r: &i32` and `n` is
  `i32` — verbatim the case the explainer names.

The lesson cites the E-code by family ("same E-code as lesson 045's
broken contrast") rather than re-explaining E0308. The page's first
example (lines 11-15) — `plus_one("Not a number");` against `fn
plus_one(x: i32) -> i32` — is structurally identical to today's broken
probe (function call with the wrong argument type), with `&i32` /
`i32` swapped for `i32` / `&str`. The lesson does not reproduce that
example.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/046-reference-function-parameter.rs`.
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
fn show(r: &i32) {
    println!("via reference param: {r}");
}

fn main() {
    let n: i32 = 42;
    show(&n);
    println!("n is still: {n}");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
via reference param: 42
n is still: 42
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent on success (lesson 001).
- `./demo` prints exactly two lines, `via reference param: 42` then
  `n is still: 42`. The first line is the `println!` inside `show`,
  formatted via `{r}` over the parameter of type `&i32`; lesson 045's
  "`{}` looks through `&T`" gives `42`. The second line is the
  `println!` in `main`, *after* `show` returned, reading the original
  binding `n` of type `i32`; it also prints `42`.
- The two-lines-and-the-second-still-reads-`n` shape is the
  load-bearing empirical observation. It corroborates the lesson's
  "the binding `n` is still usable after the call" claim. It does
  *not* explain *why* — the structural reason involves *ownership*
  and the fact that `i32` is `Copy`, both deferred under *What To
  Ignore For Now*. The probe is the empirical witness; the corpus
  citation behind the operational consequence is
  `book/ch04-02-references-and-borrowing.md` lines 88-93 ("the value
  pointed to by the reference is not dropped when `s` stops being
  used, because `s` doesn't have ownership").
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe

Source: working-probe shape with line 7 changed from `show(&n);` to
`show(n);` (the prefix-`&` removed at the call site) and the trailing
`println!("n is still: {n}");` removed for clarity (it would also
diagnose, but the lesson body explicitly removes it to keep the
broken file small). Not committed; the transcript below is the
artifact. Captured 2026-05-07 in a fresh `mktemp -d` (filename
`broken.rs`):

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
broken.rs
--- cat broken.rs ---
fn show(r: &i32) {
    println!("via reference param: {r}");
}

fn main() {
    let n: i32 = 42;
    show(n);
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:7:10
  |
7 |     show(n);
  |     ---- ^ expected `&i32`, found `i32`
  |     |
  |     arguments to this function are incorrect
  |
note: function defined here
 --> broken.rs:1:4
  |
1 | fn show(r: &i32) {
  |    ^^^^ -------
help: consider borrowing here
  |
7 |     show(&n);
  |          +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Notes (probe evidence — not corpus quotation):

- The headline reads `error[E0308]: mismatched types` — the *generic*
  E0308 headline (same as lessons 024 and 045 captured), not one of
  the more specialized variants ("`if` and `else` have incompatible
  types" from lesson 026, etc.). Same E-code as lessons 024, 025,
  026, 028, 033, and 045 captured.
- The diagnostic has the lesson-003 parts (headline + `-->` location +
  source excerpt with caret + `help:` block + `--explain` trailer).
  The location `broken.rs:7:10` points at column 10 of line 7, which
  is the *first character of the argument* `n` inside `show(n)`.
- The caret block is two-part: an underline `----` under the function
  name `show` labelled `arguments to this function are incorrect`,
  and a single-character caret `^` under the argument `n` labelled
  `expected `&i32`, found `i32``. This is rustc's pinpoint of *both*
  ends of the mismatch — the function being called and the argument
  that delivered the wrong type. Probe evidence for the lesson's "the
  signature still declares `&i32`; the call passes `i32`" framing.
- Crucially, the diagnostic includes a *second* `-->` location:
  `note: function defined here` followed by `--> broken.rs:1:4` and
  the source excerpt for line 1 (`fn show(r: &i32) {`). This is the
  *dual-`-->`* pattern lesson 036 first installed (for E0061 arity
  errors). Today's E0308 carries it for the same reason: the type
  mismatch is between the call site and the definition, so rustc
  shows both.
- The `help:` block reads literally `help: consider borrowing here`,
  followed by a source-diff suggestion that re-prints line 7 with `&`
  inserted in front of `n`. This is identical wording to lesson 045's
  broken-contrast `help:` block (the same fix, suggested in two
  contexts: a `let` annotation in lesson 045, an argument in lesson
  046). rustc's runtime statement of the fix matches the lesson's
  intended teaching — the cleanest possible alignment.
- Exit code: 1. No executable was produced. The `ls after` shows only
  `broken.rs`.

The broken-contrast probe is necessary because the lesson makes a
contrastive claim ("with `&` it works, without it the `T`-vs-`&T`
mismatch fires E0308 at the call site"). The captured `expected
`&i32`, found `i32`` caret label, plus the `note: function defined
here` second `-->`, are the load-bearing pieces of probe evidence:
rustc itself distinguishes `&i32` from `i32` as different types when
matching arguments to parameters, and pinpoints both ends of the
mismatch. The corpus-level grounding is the combination of lesson
020's parameter-type-must-match-argument-type rule, lesson 045's
`&T`-vs-`T` distinction, and the E0308 explainer line 29-32 that
names this exact sub-case.

The orchestrator suggested an option (b) broken probe (define by
value, call by reference — `fn show(r: i32)` with `show(&n)`). It is
not captured here because option (a) suffices: the contrast claim is
about a missing `&` at the call site, and option (a) is the simpler
shape (one edit, not two) that produces the cleanest `help:` block
("consider borrowing here"). Option (b)'s `help:` block would
typically suggest removing the `&` ("consider removing the borrow"),
which is the opposite teaching from this lesson. Capturing only
option (a) keeps the appendix focused.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 046. Older supporting lessons are mentioned
above by number only.

- **Lesson 020 (load-bearing for the parameter-type-slot rule)** —
  installed `fn name(p: TYPE) { ... }` with `i32` as the example
  `TYPE`, and the rule that at the call site the argument's type must
  match the parameter's `TYPE`. Lesson 046 reuses the same rule with
  `&i32` in the `TYPE` slot. No new mechanism — only a new type form
  flowing through the same slot. Lesson 020's own framing already
  generalizes the slot ("name and the type of value it will hold");
  the lesson body explicitly notes the extension to a new `TYPE`.
- **Lesson 045 (load-bearing for `&T` and the prefix-`&` operator)**
  — installed (a) the *shared reference type* `&T`, (b) the prefix-`&`
  operator that builds a `&T` value from a `T` value, (c) the
  distinction `T` vs. `&T` and its E0308 *mismatched types* sub-case,
  (d) the `Display` "`{}` looks through `&T`" property. All four
  carry over unchanged to the function-parameter context. The only
  new context bit is *where* the `&value` expression sits: in lesson
  045 it sat on the right of `let`; today it sits as the argument
  inside a function call.
- **Lesson 003 (load-bearing for diagnostic shape)** — diagnostics
  have headline + `-->` location + source excerpt with caret +
  optional `help:` lines + optional `--explain` trailer. Today's
  broken-contrast walk uses that map without re-teaching it. The
  captured *second* `-->` (`note: function defined here`) is the
  dual-`-->` lesson 036 first observed; today's lesson cites lesson
  036 by name for that pattern.
- **Lesson 008 (load-bearing for two-function file shape)** — `fn
  name() { ... }` defines a second function and `name();` calls it.
  Today's program uses the same two-function shape with the
  parameters filled in (lesson 020) using a reference type (lesson
  045).
- **Lesson 005 (load-bearing for the named-placeholder `{r}`)** —
  installed `let name = value;` plus the named-placeholder `{name}`
  form for `println!`. Lesson 046 uses both unchanged inside `show`'s
  body and inside `main`.
- **Lessons 001, 002, 019** — `rustc file.rs` then `./name`; `fn
  main` is the entry point; `let name: i32 = value;`. Used unchanged.

## Older supporting lessons

Lessons 024, 025, 026, 028, 033 (E0308 family connection — the
broken-contrast probe fires E0308, the same E-code these lessons
installed for different type-annotation mismatch sub-cases (024 first;
025/028/033 reused; 026 specialized headline; 045 the `&T`-vs-`T`
sub-case). Not re-stated here beyond the family connection through
lesson 045.).

Lesson 036 (dual-`-->` precedent — lesson 036 first observed the
two-`-->` pattern in E0061 arity diagnostics; today's E0308 carries it
in the same `note: function defined here` form. Mentioned in the
lesson body's broken-contrast walk for the audit trail.).

Lesson 042 (extension-to-new-`TYPE` precedent — lesson 042 first put a
non-`i32` type in lesson 019's annotation slot (`String`); lesson 045
extended to `&i32`; today extends the *parameter*-type slot in the
same way. Mentioned for the audit trail; not re-stated.).
