# Evidence — 048-mutable-reference-parameter

Audit appendix for `lessons/048-mutable-reference-parameter.md`. Holds
the corpus-quote map, the toolchain string, the full working and
broken-contrast probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/048-mutable-reference-parameter.rs`); the
  broken-contrast `.rs` is not committed — its transcript below is
  the artifact.

## Sources

### `output/docs/rust/book/ch04-02-references-and-borrowing.md`

The Book chapter on references and borrowing. Already cited in
lessons 045, 046, 047. Today's citation is the *Mutable References*
sub-section's canonical example — the Book's `change(&mut s)` /
`fn change(some_string: &mut String)` pair is structurally identical
to the lesson's `set_to_99(&mut n)` / `fn set_to_99(r: &mut i32)`
pair. One load-bearing span.

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

Three load-bearing structural facts mirror today's lesson exactly:

1. The Book's "we change `s` to be `mut`" matches the lesson's "the
   source is `let mut n` (lesson 006): `&mut` only applies to a
   `mut`-bound place." Same precondition, named via lesson 006.
2. The Book's "we create a mutable reference with `&mut s` where we
   call the `change` function" matches the lesson's "the call
   `set_to_99(&mut n)` builds a `&mut i32` from `n`." Same operator
   action at the call site. The Book uses `&mut s` on a `String`;
   today uses `&mut n` on an `i32` to keep ownership / `Copy`
   off-stage (already deferred from cycles 046 and 047).
3. The Book's "update the function signature to accept a mutable
   reference with `some_string: &mut String`" matches the lesson's
   "`set_to_99` declares one parameter, `r`, of type `&mut i32`."
   Same parameter-slot rule from lesson 020 with a `&mut T` type;
   the Book's `&mut String` differs from the lesson's `&mut i32` only
   in the referent type. The Book's body is `some_string.push_str(",
   world")` — a method call demonstrating writing-through; the
   lesson's body is the simpler primitive `*r = 99;` (lesson 047's
   deref-assign form), keeping methods / autoref / `&mut self` off-
   stage. Same teaching, primitive-instead-of-method-call to stay
   on the installed graph.

The Book's surrounding sentence — "This makes it very clear that the
`change` function will mutate the value it borrows" — is the audience-
level corpus statement of the lesson's claim that the function
mutates the caller's value. The lesson body's "the function mutated
the caller's binding" rephrases this.

Calibration: the Book chapter goes on (lines 165+) to describe the
borrow checker's restrictions (E0499 — at most one `&mut`; E0502 — no
mixing `&` and `&mut`). Both deferred, same as in 045/046/047. The
dangling-reference / lifetimes sub-section is also deferred. The
Book's `change` function returns nothing (no `->`), like the lesson's
`set_to_99`; reference return types stay deferred.

### `output/docs/rust/reference/items/functions.md`

The Reference's spec for function items. Already cited in lessons 036
and 046. Today's citation is the same `FunctionParam` /
`FunctionParamPattern` grammar — the parameter-`Type` slot accepts
any [Type], including the `ReferenceType` form `& mut TypeNoBounds`
(which is `&mut i32` once `TypeNoBounds = i32`).

Lines 22-34 (the function-parameter grammar, repeated from lesson 046
for ease of audit):

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
  formal grammar for the lesson-020 `name: TYPE` shape. `Type` is the
  same nonterminal that elsewhere admits `& Lifetime? mut?
  TypeNoBounds` (via the `ReferenceType` rule cited in lesson 047's
  appendix from `pointer.md` line 18). Plugging in `mut` and
  `TypeNoBounds = i32`: `&mut i32` is a valid `Type`, so `r: &mut
  i32` is a valid `FunctionParam`. This is the formal license for
  today's lesson.
- The `SelfParam` / `ShorthandSelf` / `TypedSelf` rules are explicitly
  deferred (method receivers — `&self` / `&mut self`). Today's
  function is a free function with no receiver.

Lines 46-48 (the *signature* sentence, repeated from lesson 046 for
ease of audit):

> Functions may declare a set of *input* [*variables*] as parameters,
> through which the caller passes arguments into the function, and
> the *output* [*type*] of the value the function will return to its
> caller on completion.

Corpus framing of the parameter-vs-argument distinction lesson 020
already installed; reused here without re-grounding. The lesson's
"the call site supplies a `&mut i32` argument by writing `&mut
binding`" draws on this.

Calibration: the Reference page covers function generics, where
clauses, function qualifiers (`const async safe unsafe extern`),
variadic functions, and ABIs. All deferred, same as in 046.

### `output/docs/rust/reference/types/pointer.md`

The Reference page for pointer/reference types. Already cited in
lessons 045, 046, 047. Today's citation pulls the same *Mutable
references* sub-section that lesson 047 grounded — the lesson body
cites lesson 047 by name for the `&mut T` type machinery rather than
re-quoting. The grammar and the type-page section are listed here for
ease of audit.

Line 18 (the reference-type grammar, repeated from lessons 045/046):

> [ReferenceType] → & [Lifetime]? mut? [TypeNoBounds]

Today's `&mut i32` instantiates this rule with `mut` present and
`TypeNoBounds = i32`, omitting the optional `Lifetime`. Combined with
`functions.md`'s rule that the `FunctionParam` slot accepts any [Type],
this is the formal license for `r: &mut i32` as a parameter type.

Lines 38-46 (the *Mutable references* section, already grounded in
lesson 047's appendix):

> ### [Mutable references (`&mut`)](#mutable-references-mut)
>
> Mutable references point to memory which is owned by some other
> value. A mutable reference type is written `&mut type` or `&'a mut
> type`.

Lesson 047's appendix maps these sentences to the type vocabulary
("a mutable reference type is written `&mut type`"). Today's lesson
cites lesson 047 by name for the type machinery — the *new* combined
fact today is putting `&mut T` in a parameter slot, which is the
function-grammar rule above, not the type-page rule.

Calibration: the page also covers raw pointers and smart pointers —
all deferred as in 047.

### `output/docs/rust/reference/expressions/operator-expr.md`

The Reference page for operator expressions. Already cited in lessons
034, 037, 045, 046, 047 for various operator spans. Today's lesson
cites lesson 047 by name for the *Borrow operators* (`&mut`) and the
*Dereference operator* (`*` and the deref-assign clause); the call-
site `&mut n` is the same operator from lesson 047 in a new context
(an argument expression instead of the right of a `let`). One
load-bearing repeat span.

Line 80 (the borrow operators' name, repeated from lessons 045/047):

> The `&` (shared borrow) and `&mut` (mutable borrow) operators are
> unary prefix operators.

Today's load-bearing word is `&mut`. The lesson body's "the same
prefix `&mut` operator from lesson 047, now used as an argument
expression instead of the right of `let`" cites lesson 047 by name
rather than re-quoting the operator's effect.

Line 84 (the operator's result, repeated from lessons 045/046/047):

> When applied to a [place expression], this expressions produces a
> reference (pointer) to the location that the value refers to.

The lesson's `&mut n` at the call site is exactly this case (`n` is
a `let mut`-bound name, which is a place expression — and a
*mutable* place expression, satisfying the line-92 *expr.operator.
borrow.mut* clause that lesson 047's appendix already mapped). The
result is a value of type `&mut i32` that is then supplied as the
argument to `set_to_99`.

The *Dereference operator* span (lines 192-212), including the
*expr.deref.mut* clause that licenses `*r = newval;`, is fully
mapped in lesson 047's appendix. Today's body's `*r = 99;` is the
same form in the same primitive type, so the lesson cites lesson 047
by name rather than re-quoting.

Calibration: the page also covers `&&` double-borrow, `&raw const`,
`&raw mut`, and `*const T`. All deferred.

### `output/docs/rust/error_codes/E0308.md`

The error-code explainer for E0308, "expected type did not match the
received type." Already cited in lessons 024, 025, 026, 028, 033,
045, 046, 047. Reused here for the broken-contrast probe. Two
load-bearing spans.

Line 4 (the canonical one-liner, repeated for ease of audit):

> Expected type did not match the received type.

Today's E0308 is yet another sub-case of the same general type-
mismatch rule. The *new* specific sub-case is "argument is `&i32`,
parameter is `&mut i32`" — a function-call type mismatch where the
two reference types differ in the `mut` modifier. The lesson body
cites the E-code by family ("same E-code family as lessons 045,
046, 047") rather than re-explaining E0308.

Lines 29-32 (the function-argument sub-case, repeated from lesson
046):

> This error occurs when an expression was used in a place where the
> compiler expected an expression of a different type. It can occur in
> several cases, the most common being when calling a function and
> passing an argument which has a different type than the matching
> type in the function declaration.

Today's broken probe is exactly this sub-case: `set_to_99(&n)` where
the parameter is declared `r: &mut i32` and the argument has type
`&i32`. The corpus statement names the case verbatim. The lesson
does not reproduce this prose; it simply cites the E-code family.

Calibration: rustc's specific caret label `types differ in
mutability` and `note: expected mutable reference &mut _ / found
reference &_` are not in the explainer page — they are *probe*
evidence, captured in the broken-contrast transcript below. The
lesson body labels them as such (e.g., "the caret label ... matches
lesson 047's broken probe").

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/048-mutable-reference-parameter.rs`.
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
fn set_to_99(r: &mut i32) {
    *r = 99;
}

fn main() {
    let mut n: i32 = 1;
    set_to_99(&mut n);
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
  `n` was `let mut`-bound to `1` on line 6; line 7 called
  `set_to_99(&mut n)`, whose body wrote `99` *through* the `&mut i32`
  parameter `r`; line 8 read `n` and got `99`. The empirical
  observation is the post-call read: the function mutated the
  caller's binding.
- Three pieces composed: lesson 020's `name: TYPE` parameter shape
  (here `r: &mut i32`), lesson 047's prefix-`&mut` operator (here at
  the call site as `&mut n` instead of on the right of `let`), and
  lesson 047's deref-assign form `*r = newval;` (here `*r = 99;` in
  a function body instead of in `main`). Pure composition — no new
  mechanism.
- The lesson's "two preconditions line up" framing is corroborated
  empirically: line 6 must be `let mut n` (lesson 006) for `&mut n`
  on line 7 to apply; the call-site argument must be `&mut n`, not
  `&n`, for the call to type-check.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe

Source: working-probe shape with line 7 changed from `set_to_99(&mut
n);` to `set_to_99(&n);` (the `mut` removed from the call-site
borrow, leaving a *shared* borrow into a *mutable*-reference parameter
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
fn set_to_99(r: &mut i32) {
    *r = 99;
}

fn main() {
    let mut n: i32 = 1;
    set_to_99(&n);
    println!("n = {n}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:7:15
  |
7 |     set_to_99(&n);
  |     --------- ^^ types differ in mutability
  |     |
  |     arguments to this function are incorrect
  |
  = note: expected mutable reference `&mut _`
                     found reference `&_`
note: function defined here
 --> broken.rs:1:4
  |
1 | fn set_to_99(r: &mut i32) {
  |    ^^^^^^^^^ -----------

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Notes (probe evidence — not corpus quotation):

- The headline reads `error[E0308]: mismatched types`. Same E-code
  lessons 024, 025, 026, 028, 033, 045, 046, 047 captured. Today is
  yet another sub-case of the same general type-mismatch rule. The
  lesson body cites the E-code by family ("same E-code family as
  lessons 045, 046, 047") without re-teaching it.
- The diagnostic has the lesson-003 parts: headline + `-->` location
  (`broken.rs:7:15`) + source excerpt with caret + `= note:` block +
  *second* `-->` (`note: function defined here`) + source excerpt
  with caret + `--explain E0308` trailer. No `help:` block — same as
  cycle 047's broken probe. rustc does not auto-suggest `&mut n`
  here (it cannot tell whether the user intended a shared or a
  mutable borrow once both annotations disagree).
- This diagnostic is a *clean composition* of cycles 046 and 047's
  broken probes — load-bearing for the lesson's "pure composition"
  framing:
  - The caret label `types differ in mutability` is identical text to
    cycle 047's broken probe (which fired E0308 in a `let`
    annotation, not a call). Probe evidence that rustc names
    *mutability* as the dimension of difference between `&i32` and
    `&mut i32` in *both* contexts (let-annotation and call-argument).
  - The second `-->` reading `note: function defined here` plus its
    own source-excerpt-with-caret block is identical structure to
    cycle 046's broken probe (which fired E0308 in a call-site `&i32`
    vs `i32` mismatch). Probe evidence for the dual-`-->` pattern
    (lesson 036's first observation) — rustc shows both ends of any
    call-site type mismatch.
  - The `= note:` block reads `expected mutable reference &mut _`
    over `found reference &_` — same text as cycle 047's broken
    probe. The wildcards `_` collapse the unspecified referent type
    (here `i32` on both sides); the mismatch is in the *kind* of
    reference, not the referent.
  - The caret block under line 7 is two-part: an underline `---------`
    under the function name `set_to_99` labelled `arguments to this
    function are incorrect` (cycle 046's exact label), and a
    two-character caret `^^` under the argument `&n` labelled `types
    differ in mutability` (cycle 047's exact label). Two label texts
    rustc uses, in two contexts, joined here in one diagnostic. Probe
    evidence for the lesson's "pure composition of 046 and 047"
    framing.
- Exit code: 1. No executable was produced. The `ls after` shows
  only `broken.rs`.
- The broken probe stops with exactly one error. There is no E0594
  ("cannot assign through `&` reference") secondary, even though line
  2 is `*r = 99;` and (hypothetically) `r` would have been `&i32` had
  line 7 succeeded. rustc bails at the first call-site type error,
  consistent with the lesson's deferral of E0594.
- The broken probe also does not fire E0596 ("cannot borrow as
  mutable"), because line 6 is `let mut n` and line 7's `&n` is a
  *shared* borrow (no `let mut` requirement). E0596 is deferred.

The broken-contrast probe is necessary because the lesson makes a
contrastive claim ("`&mut i32` parameter type matches `&mut n`
argument; passing `&n` instead fires E0308"). The captured `types
differ in mutability` caret label, the second `-->` (`note: function
defined here`), and the `= note: expected mutable reference &mut _ /
found reference &_` block are the load-bearing pieces of probe
evidence: rustc itself distinguishes `&i32` from `&mut i32` at the
call site, and pinpoints both ends of the mismatch (call site +
function definition). The corpus-level grounding for the contrastive
claim is the combination of lesson 020's parameter-type-must-match-
argument-type rule (carried over via lesson 046), lesson 047's `&T`-
vs-`&mut T` distinction, and the E0308 explainer line 29-32
function-argument sub-case.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 048. Older supporting lessons are mentioned
above by number only.

- **Lesson 020 (load-bearing for the parameter-type-slot rule)** —
  installed `fn name(p: TYPE) { ... }` with `i32` as the example
  `TYPE`, and the rule that at the call site the argument's type must
  match the parameter's `TYPE`. Lesson 048 reuses the same rule with
  `&mut i32` in the `TYPE` slot. No new mechanism — only a new type
  form flowing through the same slot. (Lesson 046 already extended to
  `&i32`; today extends further to `&mut i32`.)
- **Lesson 046 (load-bearing for the reference-parameter shape)** —
  installed (a) the parameter-type slot accepting a reference type,
  (b) the call-site `&binding` argument form, (c) the dual-`-->`
  diagnostic for call-site type mismatches involving reference
  parameters. All three carry over to today's `&mut`-typed
  parameter. The only change is swapping the prefix-`&` operator for
  the prefix-`&mut` operator throughout (signature, call site, and
  the `mut` modifier in the diagnostic text).
- **Lesson 047 (load-bearing for `&mut T` and the deref-assign form)**
  — installed (a) the *mutable reference type* `&mut T`, (b) the
  prefix-`&mut` operator that builds a `&mut T` value from a `let
  mut`-bound place, (c) the deref-assign form `*r = newval;`, (d) the
  `&T`-vs-`&mut T` E0308 sub-case with caret label `types differ in
  mutability` and `= note: expected mutable reference &mut _ / found
  reference &_`. All four carry over unchanged to today's function-
  parameter context. The only new context bit is *where* the `&mut
  value` expression sits: in lesson 047 it sat on the right of `let`;
  today it sits as an argument inside a function call. Symmetrically,
  the `*r = newval;` form sits inside a function body today instead
  of inside `main`.
- **Lesson 006 (load-bearing for the `let mut` precondition)** —
  installed `let mut name = value;` and the rule that bindings are
  immutable by default; `mut` makes them reassignable. Lesson 048
  needs `let mut n` so that `&mut n` at the call site applies. The
  lesson body cites lesson 006 by name and asserts the consequence
  ("`&mut` only applies to a `mut`-bound place"). The contrast variant
  that would fire E0596 is explicitly deferred.
- **Lesson 003 (load-bearing for diagnostic shape)** — diagnostics
  have headline + `-->` location + source excerpt with caret +
  optional `note:` / `= note:` lines + optional second `-->` +
  `--explain` trailer. Today's broken-contrast walk uses that map
  without re-teaching it.
- **Lesson 008 (load-bearing for two-function file shape)** — `fn
  name() { ... }` defines a second function and `name();` calls it.
  Today's program uses the same two-function shape with the parameter
  filled in by lesson 020 with a type from lesson 047.
- **Lesson 005 (load-bearing for the named-placeholder `{n}`)** —
  installed `let name = value;` plus the named-placeholder `{name}`
  form for `println!`. Lesson 048 uses both unchanged inside `main`.
- **Lessons 001, 002, 019** — `rustc file.rs` then `./name`; `fn
  main` is the entry point; `let name: i32 = value;`. Used unchanged.

## Older supporting lessons

Lessons 024, 025, 026, 028, 033 (E0308 family connection — the
broken-contrast probe fires E0308, the same E-code these lessons
installed for different type-annotation mismatch sub-cases. Not
re-stated here beyond the family connection through lesson 045).

Lesson 036 (dual-`-->` precedent — lesson 036 first observed the
two-`-->` pattern in E0061 arity diagnostics; lesson 046 carried it
to E0308 with `note: function defined here`; today's broken probe
inherits the same pattern through lesson 046. Mentioned for the
audit trail.).

Lesson 045 (parallel structure precursor — lesson 045 introduced the
shared reference type `&T` and prefix-`&` operator; lesson 046
extended to `&i32` parameters; lesson 047 introduced `&mut T` and
the deref-assign form; today's lesson combines 046's
parameter-extension and 047's `&mut T` into one move. Mentioned for
the audit trail.).
