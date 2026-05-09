# Evidence — Lesson 147: parenthesized `Fn(T) -> R` trait bound + closure-as-argument

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/147-fn-trait-parenthesized-bound.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/147-fn-trait-parenthesized-bound.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/147-fn-trait-parenthesized-bound.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet147/` and compiled with
`rustc <file>`; resulting executables were run from the same
directory. Same host and toolchain as accepted lessons 145, 146.

## Run context — closure sub-arc step 4 decomposition (move 3 of 3)

Per `iterator-api-coverage.md` §6, the closure sub-arc has five
steps. Steps 1-3 (lessons 142-144) installed closure literals,
unannotated-first-call, and capture. Step 4 was sketched as a
single move "FnMut-bound parameter on a function." In execution
that conflates three distinct new mechanics:

1. *Generic function syntax* — `fn name<T>(...)` with no bound.
   Lesson 145, accepted commit `b3b1b0434`.
2. *Trait bound on the type parameter* — `<T: Trait>`. Lesson
   146, accepted commit `a3e256ddf`.
3. *The parenthesized `Fn(...)` / `FnMut(...) -> R` sugar plus
   closure-as-argument wiring* — today.

After today, the closure sub-arc step 4 closes. Lesson 148 will
install the `Fn` / `FnMut` / `FnOnce` distinction (audit §6 step
5) and close the closure sub-arc entirely; the 27 closure-driven
Iterator methods (audit §4.4.1) become teachable.

The audit document itself is not updated here; that is an
orchestrator action after 148 lands.

## Choice of trait — `Fn` vs `FnMut`

The orchestrator's prompt named both `Fn` and `FnMut` as
candidate working-probe shapes. Empirical comparison (Side-probe
D in the transcript): with `Fn`, the body `f(x)` compiles
silently. With `FnMut`, the body `f(x)` fires E0596:

```text
error[E0596]: cannot borrow `f` as mutable, as it is not declared as mutable
 --> fnmut_no_mut.rs:2:5
  |
2 |     f(x)
  |     ^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
1 | fn apply<F: FnMut(u32) -> u32>(mut f: F, x: u32) -> u32 {
  |                                +++
```

Picking `FnMut` would force the lesson to introduce
`mut`-on-a-function-parameter as a side-fact. Lesson 006
installed `let mut`; lessons 020/021 did not extend `mut` to
parameter slots. Composing them empirically would work, but it
would compound a second new mechanic onto today's centered move.
The orchestrator's recommendation was Option A (`Fn`); the
empirical witness above confirms why.

Lesson 148 (the closure sub-arc closer) is the right place to
introduce the `Fn` / `FnMut` / `FnOnce` distinction *and* the
`mut f: F` consequence at the same time, motivated by capture
rules.

The std-library `FnMut.md` page itself uses `mut func: F` in its
canonical example (line 65 verbatim
`fn do_twice<F>(mut func: F) where F: FnMut()`), corroborating
the empirical finding.

## Direct prerequisite — lesson 146 (inline trait bound `<T: TRAIT>`)

Lesson 146 installed:

- The shape `<T: TRAIT>`: a colon and a trait path appended after
  the type parameter name, inside the angle brackets.
- The bound restricts the call site to types implementing `TRAIT`
  *and* enables the body to use `TRAIT`'s methods on `T` values.
- A new error code, E0277, fires when the body uses a method or
  capability that the trait would provide but the bound is
  missing. The diagnostic shape is the lesson 003 four-part map.
- The trait path can be fully qualified (`std::fmt::Display`) and
  no `use` declaration is needed.

Today extends this exact mechanic with one new fact: the trait
name may be followed by *parens* and an optional `-> R`. The
parens carry the parameter types of the callable; `-> R` carries
its return type. The colon-and-trait-path placement, the
inside-angle-brackets placement, and the per-call substitution
machinery are unchanged.

The lesson body's "same colon, same angle brackets" sentence
references lesson 146's `<T: std::fmt::Display>` shape directly.

## Direct prerequisite — lesson 145 (generic function `<T>`)

Lesson 145 installed:

- The `<T>` slot on a `fn` header — between the function name
  and the parameter list.
- Per-call substitution: each call site picks a concrete type for
  `T` independently.

Today fills the type parameter slot with `F` and adds the
parenthesized bound; the per-call substitution at the call site
`apply(add_one, 5)` picks the concrete closure type.

## Direct prerequisite — lesson 142 (closure literal)

Lesson 142 installed `let name = |param: T| body;` — a closure
literal bound to `let`. Today's argument `let add_one = |n: u32|
n + 1;` is exactly that shape. The closure's parameter type and
return type are what rustc matches against the bound's parens
and `-> R` slot.

## Direct prerequisite — lesson 003 (rustc diagnostic map)

Lesson 003 installed the four-part diagnostic map. E0271 is a
*new* error code today; the diagnostic shape is unchanged. The
new feature is the *kind* of mismatch the diagnostic surfaces:
"expected ... to return `u32`, but it returns `i32`" — a
return-type mismatch between the closure body and the bound's
`-> R` slot. Probe 3's E0631 diagnostic is similarly new with a
parameter-type mismatch surface; Probe 4's E0277 is the same
code lesson 146 installed, with a new "expected a `Fn(u32)`
closure, found `{integer}`" payload.

In each diagnostic the `note: required by a bound in <fn>` block
points back at the offending segment of the parenthesized bound
inside the function declaration:

- Probe 2 (E0271): caret at column 24 on the `u32` *return slot*.
- Probe 3 (E0631): caret at column 24 on the same return slot
  (this is unintuitive — the offending segment is the `(u32)`
  parameter slot at column 16, but rustc's caret follows
  `^^^` underlining placement convention; the diagnostic's
  `expected closure signature` line names the parameter mismatch
  explicitly).
- Probe 4 (E0277): caret at column 13 on the *full* `Fn(u32)
  closure` portion of the bound.

The parenthesized bound is treated as a structured object that
can be matched segment by segment, with the diagnostic able to
point at the failing segment.

## Cited prereqs (load-bearing-but-restated-elsewhere)

- **Lesson 008**: call-with-parens `name(value)`. Reused for
  `f(x)` in the body and `apply(add_one, 5)` at the call site.
- **Lesson 020**: typed parameter `(p: TYPE)`. Reused for
  `f: F` and `x: u32`.
- **Lesson 021**: return type `-> RTYPE`. Reused for
  `-> u32` on the function signature.
- **Lesson 080**: `u32` as a named integer type (used in the
  bound's parens, on the function parameter `x: u32`, and inside
  the closure annotation `|n: u32|`). The Check Yourself
  generalizes to `i64`. The bare `5` at the call site is inferred
  to `u32` from `x: u32`; no literal suffix (lesson 081) is
  needed today.
- **Lesson 011**: `println!("{}", arg)`.
- **Lesson 005**: `let r = ...`.
- **Lessons 002, 001**: `fn main`, `rustc file.rs`, `./name`.

## Source — `output/docs/rust/std/ops/trait.Fn.md` (the special syntax for Fn traits)

The corpus file `output/docs/rust/std/ops/trait.Fn.md` is the
trait page for `Fn`. Verified by reading.

### Lines 45-47 (the centered claim)

```text
Also of note is the special syntax for `Fn` traits (e.g.
`Fn(usize, bool) -> usize`). Those interested in the technical
details of this can refer to the relevant section in the
*Rustonomicon*.
```

This is the lesson body's centered grounding: the trait page
itself names `Fn(...)` as a "special syntax" for the trait —
exactly the parenthesized form the lesson installs. The
example `Fn(usize, bool) -> usize` is structurally identical
to today's `Fn(u32) -> u32`, with two parameters instead of
one. Verified at lines 45-47.

The reference into the Rustonomicon HRTB section is the formal
treatment of the desugaring `Fn(T) -> R` ≡ `Fn<(T,), Output = R>`;
the lesson's *What To Ignore For Now* section names this
desugaring as deferred.

### Lines 60-68 (canonical example: closure passed to a generic function via Fn bound)

```text
fn call_with_one<F>(func: F) -> usize
    where F: Fn(usize) -> usize {
    func(1)
}

let double = |x| x * 2;
assert_eq!(call_with_one(double), 2);
```

This is the corpus's canonical shape match for today's working
probe. Differences:

- The corpus uses the `where` clause form (`where F: Fn(...)`);
  today uses the inline form (`<F: Fn(...)>`). Per Reference
  `trait-bounds.md:40` (lesson 146's evidence), these are the
  same mechanic.
- The corpus uses `usize`; today uses `u32` (lesson 080's
  integer family).
- The corpus uses an unannotated closure `|x| x * 2`; today uses
  an annotated `|n: u32| n + 1` for prerequisite discipline
  (lesson 142 installed the annotated form).

The structural identity is exact: a generic function with one
type parameter constrained by `Fn(T) -> R`, a closure literal
bound to `let`, and the closure passed as an argument to the
function whose body calls it with parens.

Verified at lines 60-68.

## Source — `output/docs/rust/std/ops/trait.FnMut.md` (Side-probe D corroboration)

The corpus file `output/docs/rust/std/ops/trait.FnMut.md` is the
trait page for `FnMut`. Verified by reading.

### Lines 45-47 (mirrors Fn.md's special-syntax note)

```text
Also of note is the special syntax for `Fn` traits (e.g.
`Fn(usize, bool) -> usize`).
```

Same wording as `Fn.md`. The "special syntax" applies to the
whole `Fn`/`FnMut`/`FnOnce` family.

### Lines 65-66 (canonical FnMut example with `mut func`)

```text
fn do_twice<F>(mut func: F)
    where F: FnMut()
{
    func();
    func();
}
```

The std doc itself writes `mut func: F` for an `FnMut`-bounded
parameter. This is the source corroborating Side-probe D's
empirical witness: `FnMut` requires `mut` on the parameter for
the body to call it. The lesson defers `FnMut` to lesson 148 and
uses `Fn` today; this corpus quote is the textual confirmation.

Verified at lines 65-66.

## Source — `output/docs/rust/reference/paths.md` (the formal grammar)

The corpus file `output/docs/rust/reference/paths.md` covers the
formal grammar of paths. Verified by reading.

### Lines 193-197 (the type-path-fn grammar)

```text
TypePathSegment → PathIdentSegment ( ::? ( GenericArgs | TypePathFn ) )?

TypePathFn → ( TypePathFnInputs? ) ( -> TypeNoBounds )?

TypePathFnInputs → Type ( , Type )* ,?
```

This is the formal Reference grammar that admits today's
`Fn(u32) -> u32` form. A type-path segment may be followed
either by `GenericArgs` (the `<T, U>` form lesson 146 / 145
used) *or* by `TypePathFn` (today's parenthesized form). The
formal grammar treats them as alternatives; today's lesson
installs the second alternative.

Verified at lines 193-197. The example at line 222 of the same
file (`type G = std::boxed::Box<dyn std::ops::FnOnce(isize) -> isize>;`)
shows the parenthesized form on `FnOnce`, paired with the
fully-qualified path `std::ops::FnOnce` — confirming that the
parens-and-arrow form attaches to the *trait name* segment
regardless of how the rest of the path is written.

## Source — `output/docs/rust/book/ch13-01-closures.md` (Book treatment)

The corpus file `output/docs/rust/book/ch13-01-closures.md`
covers closures. Verified by reading.

### Lines 449-482 (the `unwrap_or_else` example)

```text
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

The Book's exposition (lines 471-477):

> The `F` type is the type of the parameter named `f`, which is
> the closure we provide when calling `unwrap_or_else`.
>
> The trait bound specified on the generic type `F` is
> `FnOnce() -> T`, which means `F` must be able to be called
> once, take no arguments, and return a `T`.

The Book treats the parenthesized-bound shape as the standard
way to specify "this generic parameter is a callable with this
signature." Lesson 146's `T: TRAIT` shape is the prerequisite
grammar; today is the special-cased extension. Verified at lines
449-482.

The Book's example uses `FnOnce`; today uses `Fn`. The trait
choice is orthogonal to the parenthesized-bound mechanic — all
three Fn-family traits use the same parens-and-arrow syntax. The
Fn/FnMut/FnOnce distinction is lesson 148.

### Lines 437-447 (Book's three-line summary of the Fn-family)

```text
- `FnOnce` applies to closures that can be called once. ...
- `FnMut` applies to closures that don't move captured values
  out of their body but might mutate the captured values. ...
- `Fn` applies to closures that don't move captured values out
  of their body and don't mutate captured values, as well as
  closures that capture nothing from their environment.
```

Today's working probe uses a closure that captures nothing —
`|n: u32| n + 1` — so it implements `Fn` (and hence also `FnMut`
and `FnOnce`, since the family is layered). The lesson body
does *not* center this layered relationship; lesson 148 will. For
today the witness is empirical: Probe 1 compiles with `Fn` as the
bound. Verified at lines 437-447.

## Probe 1 — working probe (Fn(u32) -> u32 bound; closure passed as arg)

Source: `observations/147-fn-trait-parenthesized-bound.rs`.
Transcript: PROBE 1 block.

```rust
fn apply<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
    f(x)
}

fn main() {
    let add_one = |n: u32| n + 1;
    let r = apply(add_one, 5);
    println!("{}", r);
}
```

Output: `6`. Compile exit 0, run exit 0. Three load-bearing
structural facts witnessed:

- The bound `<F: Fn(u32) -> u32>` parses cleanly. No syntax
  error; the parens-and-arrow form is accepted by rustc inside
  the angle brackets, after the colon, with the bare trait name
  `Fn` (no `std::ops::` prefix needed — Side-probe A confirms
  the qualified form also works).
- The body `f(x)` compiles. The bound is what makes the
  parens-call shape on `f` legal; without an Fn-family bound,
  `f` could not be called.
- The call site `apply(add_one, 5)` works. rustc substitutes
  `add_one`'s anonymous closure type for `F` and verifies the
  closure's signature matches `Fn(u32) -> u32`.

## Probe 2 — return-type mismatch (E0271)

Source `wrong_ret.rs` (in transcript). The single-line
modification is changing the closure body from `n + 1` to
`n as i32` so the return type becomes `i32`. The rest is
unchanged.

```rust
let returns_i32 = |n: u32| n as i32;
let _ = apply(returns_i32, 5);
```

Output:

```text
error[E0271]: expected `{closure@wrong_ret.rs:6:23}` to return `u32`, but it returns `i32`
 --> wrong_ret.rs:6:32
  |
6 |     let returns_i32 = |n: u32| n as i32;
  |                       -------- ^^^^^^^^ expected `u32`, found `i32`
  |                       |
  |                       this closure
7 |     let _ = apply(returns_i32, 5);
  |             ----- ----------- closure used here
  |             |
  |             required by a bound introduced by this call
  |
note: required by a bound in `apply`
 --> wrong_ret.rs:1:24
  |
1 | fn apply<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
  |                        ^^^ required by this bound in `apply`
```

Compile exit 1. Six grounded facts from rustc's mouth:

- Error code is `E0271`. The inline label "expected ... to
  return `u32`, but it returns `i32`" names the mismatch in
  exact terms.
- Caret on `n as i32` (the closure's body) at column 32 — the
  expression that returns the wrong type.
- A second annotated span on the call line marks the call as
  the source of the bound that was violated.
- The `note: required by a bound in \`apply\`` block carries its
  own `-->` line at `wrong_ret.rs:1:24`, the position of the
  *return slot* `u32` inside the parenthesized bound. Caret on
  the three characters of `u32`. This is the structural witness
  for the lesson body's claim: rustc matches the closure's shape
  against the bound's parens-and-arrow shape segment by segment.
- The diagnostic shape is the lesson 003 four-part map (headline
  + `-->` + source excerpt + `note:`/`help:`), unchanged.
- The single-segment modification (return type) flips
  acceptance. Probes 1 and 2 differ only in the closure's body
  expression; opposite outcomes. This is the contrastive witness.

## Probe 3 — parameter-type mismatch (E0631)

Source `wrong_param.rs` (in transcript). Modification: closure
parameter type changed from `u32` to `i32`, body returns `u32`.
Output:

```text
error[E0631]: type mismatch in closure arguments
 --> wrong_param.rs:7:19
  |
6 |     let takes_i32 = |n: i32| n as u32;
  |                     -------- found signature defined here
7 |     let _ = apply(takes_i32, 5);
  |             ----- ^^^^^^^^^ expected due to this
  |             |
  |             required by a bound introduced by this call
  |
  = note: expected closure signature `fn(u32) -> _`
             found closure signature `fn(i32) -> _`
note: required by a bound in `apply`
 --> wrong_param.rs:1:24
  |
1 | fn apply<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
  |                        ^^^ required by this bound in `apply`
help: consider wrapping the function in a closure
  |
7 |     let _ = apply(|arg0: u32| takes_i32(/* i32 */), 5);
  |                   +++++++++++          +++++++++++
```

Compile exit 1. Five grounded facts:

- New error code `E0631` for parameter-shape mismatch.
- The `= note:` block names both shapes inline:
  `expected closure signature \`fn(u32) -> _\`` /
  `found closure signature \`fn(i32) -> _\``.
- The `note: required by a bound in \`apply\`` block again
  points at the bound, with caret on column 24 of line 1
  (rustc's caret placement here is structural — it points at
  three characters of `u32` inside the bound, which happens to
  be the *return* position; the parameter mismatch is named in
  the `= note:` block above).
- The `help:` block proposes wrapping `takes_i32` in a closure
  literal that bridges the parameter type — a structural fix.
- A different error code from Probe 2's E0271, because rustc
  surfaces parameter shape mismatches and return shape
  mismatches with different diagnostics. Both surface the same
  `note: required by a bound` pointer back at the function's
  bound.

## Probe 4 — non-closure argument (E0277)

Source `non_closure_arg.rs` (in transcript). Modification: the
call site passes `7` (a primitive integer) instead of a closure.

Output (excerpt):

```text
error[E0277]: expected a `Fn(u32)` closure, found `{integer}`
 --> non_closure_arg.rs:6:19
  |
6 |     let _ = apply(7, 5);
  |             ----- ^ expected an `Fn(u32)` closure, found `{integer}`
  ...
  = help: the trait `Fn(u32)` is not implemented for `{integer}`
note: required by a bound in `apply`
```

Compile exit 1. The error code is `E0277` — *the same code lesson
146 installed*. The new payload is "expected a `Fn(u32)` closure,
found `{integer}`": rustc spells the bound out using the
parenthesized form. This is the symmetric contrast to lesson
146's Side-probe B (where rustc named `\`Widget\` doesn't
implement \`std::fmt::Display\``) — same diagnostic shape, with
the trait name written in the parenthesized form today.

## Side-probe A — fully-qualified `std::ops::Fn(...)` is equivalent

Source `qualified.rs` (in transcript). Same source as Probe 1 with
`Fn(u32) -> u32` replaced by `std::ops::Fn(u32) -> u32`. Compile
exit 0, prints `6`. Witnesses that the parenthesized-trait-arg
form attaches to the trait name *segment* of the path, regardless
of whether the path is bare (`Fn`) or fully qualified
(`std::ops::Fn`). The bare form works because `Fn` / `FnMut` /
`FnOnce` are language items recognized by rustc without an
explicit `use`. The lesson body uses the bare form for surface
minimality; this side-probe documents that the qualified form
behaves identically.

The Reference example at `paths.md:222`
(`type G = std::boxed::Box<dyn std::ops::FnOnce(isize) -> isize>;`)
is the corpus precedent for the qualified form.

## Side-probe B — Check Yourself (a)

Source `tiny.rs` (in transcript). `fn run<G: Fn(i64) -> i64>(g: G, k: i64) -> i64 { g(k) }`
with `let triple = |n: i64| n * 3;` and `run(triple, 7)`. Compile
exit 0, prints `21`. Witnesses that the lesson generalizes from
`u32` to `i64` and from `+ 1` to `* 3` — the parenthesized-bound
mechanic is type-agnostic.

## Side-probe C — Check Yourself (b)

Source `tiny_b.rs` (in transcript). Same as Side-probe B but the
closure body returns `i32` instead of `i64`. Fires E0271 with
`expected ... to return \`i64\`, but it returns \`i32\`` and
`note: required by a bound in \`run\`` pointing at the `i64`
return slot inside `Fn(i64) -> i64`. Verifies the Check Yourself
answer empirically.

## Side-probe D — `FnMut` requires `mut f: F`

Source `fnmut_no_mut.rs` (in transcript). Same as Probe 1 but the
bound is `FnMut(u32) -> u32` instead of `Fn(u32) -> u32`. The
body `f(x)` fails E0596:

```text
error[E0596]: cannot borrow `f` as mutable, as it is not declared as mutable
 --> fnmut_no_mut.rs:2:5
  |
2 |     f(x)
  |     ^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
1 | fn apply<F: FnMut(u32) -> u32>(mut f: F, x: u32) -> u32 {
  |                                +++
```

Witnesses why today's lesson uses `Fn` rather than `FnMut`. The
`mut`-on-a-function-parameter mechanic (the `+++` markers under
the inserted `mut ` in the help block) is a side-fact lessons
006/020/021 have not installed. Composing it with today's
parenthesized-bound mechanic would compound two new mechanics in
one lesson, against the run's "one move per lesson" rule.

This is *not* a contrast probe for today's centered claim — it is
a probe documenting why a different bound trait was rejected. The
empirical evidence (E0596 with `mut` proposed) plus the corpus
(`FnMut.md:65` writes `mut func: F` in its canonical example) is
the joint grounding.

Lesson 148 will install the Fn/FnMut/FnOnce distinction and the
`mut f: F` consequence together.

## Probe-not-needed — multiple parameters `Fn(T, U) -> R`

The grammar at `paths.md:197` (`TypePathFnInputs → Type ( , Type )* ,?`)
admits comma-separated parameter types in the parens. The Fn-trait
page example at `Fn.md:45-47` itself uses two parameters
(`Fn(usize, bool) -> usize`). Today's lesson uses one parameter
for surface minimality; the multi-parameter form is named-deferred.

## Probe-not-needed — no-return form `Fn(T)`

The grammar at `paths.md:195` (`TypePathFn → ( TypePathFnInputs? ) ( -> TypeNoBounds )?`)
admits an optional `-> Type` segment. When omitted, the return
type defaults to `()` — the unit type. Today's lesson uses a
return type for symmetry with lesson 142's `n + 1` body; the
no-return form is named-deferred.

The corpus at `FnMut.md:65-66` itself uses `F: FnMut()` (no
return type) — corpus precedent for the no-return form.

## Probe-not-needed — the `where` clause form

The Reference at `trait-bounds.md:38-42` (lesson 146's evidence)
verbatim names the equivalence
`fn f<A: Copy>() {}` ≡ `fn f<A>() where A: Copy {}`. Today's
lesson uses the inline form per the prompt's scope discipline.
The `Fn.md:60-66` corpus example uses the `where` form
(`where F: Fn(usize) -> usize`); the lesson body uses the inline
form because lesson 146 installed the inline form.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| `Fn`/`FnMut`/`FnOnce` is a "special-cased family of traits" used for callable values | `Fn.md:45-47` verbatim "the special syntax for `Fn` traits"; `FnMut.md:45-47` same |
| The bound `F: Fn(u32) -> u32` parses and runs | Probe 1 transcript: compile-exit=0, run-exit=0 |
| Output `6` | Probe 1 output |
| The trait name `Fn` carries arguments in parens followed by `-> R` | `Fn.md:45-47` example `Fn(usize, bool) -> usize`; Reference `paths.md:195` formal grammar `TypePathFn → ( TypePathFnInputs? ) ( -> TypeNoBounds )?` |
| The signature is the same colon-and-angle-brackets shape as lesson 146, with parens replacing angle-bracketed args | Cross-reference: lesson 146's `<T: std::fmt::Display>` and today's `<F: Fn(u32) -> u32>` |
| Closure literal `\|n: u32\| n + 1` is acceptable at the call site | Probe 1 transcript; Lesson 142 (closure literal mechanic) |
| Body `f(x)` reuses lesson 008's parens-call shape | Probe 1 source; Lesson 008 |
| Probe 2 fires `error[E0271]: expected ... to return \`u32\`, but it returns \`i32\`` | Probe 2 transcript |
| The diagnostic's `note: required by a bound in \`apply\`` points at the `u32` *return slot* inside the parenthesized bound | Probe 2 transcript: caret at column 24 line 1 on three characters of `u32` |
| Probe 3 fires `error[E0631]: type mismatch in closure arguments` | Probe 3 transcript |
| Probe 3's `= note:` names `expected closure signature \`fn(u32) -> _\`` / `found closure signature \`fn(i32) -> _\`` | Probe 3 transcript verbatim |
| Probe 4 fires `error[E0277]: expected a \`Fn(u32)\` closure, found \`{integer}\`` | Probe 4 transcript |
| Probe 4's diagnostic spells the trait name in the parenthesized form | Probe 4 transcript line 6 verbatim |
| `Fn` works without `std::ops::` prefix | Probe 1 (bare) and Side-probe A (qualified) both compile and produce identical output |
| Choosing `FnMut` would force `mut f: F` (rejected for today) | Side-probe D (E0596 with `mut` proposed); `FnMut.md:65` corpus example writes `mut func: F` |
| Closures that capture nothing implement `Fn` | Book `ch13-01-closures.md:444-447` verbatim; Probe 1 closure `\|n: u32\| n + 1` captures nothing |
| Closure sub-arc step 4 closes after today | `iterator-api-coverage.md` §6 (run context); accepted lessons 145, 146, 147 (today) form the three-move decomposition |

## Older supporting lessons (named only)

The following accepted lessons are cited in the lesson body or
prerequisites; their exact prereq claims are restated above:

- 146-trait-bound-on-type-parameter — `<T: TRAIT>` inline trait bound.
- 145-generic-function-type-parameter — `fn name<T>(t: T)`.
- 142-closure-literal-bound-and-called — `let name = |p: T| body;`.
- 003-read-rustc-diagnostic — four-part diagnostic map.
- 080-integer-type-family — `u32`/`i32`/`i64` distinct.
- 020-function-with-parameter — `t: T` parameter slot.
- 021-function-return-value — `-> T` return slot.
- 011-println-positional-args — `println!("{}", arg)`.
- 008-define-and-call-function — `name(arg);` call shape.
- 005-let-binding — `let name = value;`.
- 002-fn-main-entry-point, 001-rustc-compile-and-run.

## Deliberate scope discipline

The orchestrator's prompt named scope items to NOT touch. The
lesson body's *What To Ignore For Now* section names them:

1. The `Fn`/`FnMut`/`FnOnce` distinction (lesson 148).
2. Why `FnMut` would force `mut f: F` (lesson 148, Side-probe D
   here is the witness that the side-fact exists).
3. Closure capture rules (lesson 148).
4. Iterator methods that take closures (post-lesson-148).
5. Function pointers `fn(u32) -> u32` (different mechanic).
6. `impl Fn(...)` and `Box<dyn Fn(...)>` / `&dyn Fn(...)`.
7. Multiple parameters `Fn(T, U) -> R`, no-return form `Fn(T)`,
   higher-ranked `for<'a> Fn(&'a T) -> R`.
8. The desugaring `Fn(T) -> R` ≡ `Fn<(T,), Output = R>`.

The `where` clause form is named-deferred via lesson 146's
Probe-not-needed entry (Reference `trait-bounds.md:38-42`
verbatim equivalence applies to today's bound shape unchanged).

## Run-context handoff to lesson 148

Lessons 142-147 install:

- Closure literal syntax with annotated and unannotated
  parameters, and capture rules (142, 143, 144).
- Generic function syntax and inline trait bound (145, 146).
- Parenthesized `Fn(T) -> R` trait bound on a generic function
  parameter, plus closure-as-argument (today).

Lesson 148 (the closure sub-arc closer) will add the `Fn` /
`FnMut` / `FnOnce` distinction. After 148, the closure sub-arc
prereqs are complete; the 27 closure-driven Iterator methods
(audit §4.4.1) become teachable, starting with `for_each`.

Today's `unlocks` lists lesson 148 directly, plus the deferred
bullets named above.
