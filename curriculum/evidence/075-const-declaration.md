# Evidence — 075-const-declaration

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end.
  Only the working `.rs` is committed, at
  `experimental/eduratchet2/runs/rust-moves/observations/075-const-declaration.rs`.
  The three contrast `.rs` files (`broken.rs` / `brokenmut.rs` /
  `brokenruntime.rs`) are *not* committed; the transcripts below
  are the artifacts.

Same host and toolchain as recent accepted lessons (068-074).

## Sources

### `output/docs/rust/book/ch03-01-variables-and-mutability.md`

The Book's *Declaring Constants* subsection (Ch3-1, lines 109-157).
Already cited by lessons 005, 006, 007, 057, 068 for the sibling
*Variables and Mutability* / *Shadowing* sections; today's load-
bearing span is lines 111-157. Six load-bearing pieces.

Lines 111-113 (the high-level framing of constants):

> Like immutable variables, *constants* are values that are bound
> to a name and are not allowed to change, but there are a few
> differences between constants and variables.

Corpus warrant for the lesson's first claim, "a second name-
introduction form for values that never change." The lesson re-
words "values that are bound to a name and are not allowed to
change" to "values that never change" — same content; tighter for
the audience.

Lines 115-120 (the load-bearing keyword and annotation rules):

> First, you aren't allowed to use `mut` with constants. Constants
> aren't just immutable by default—they're always immutable. You
> declare constants using the `const` keyword instead of the `let`
> keyword, and the type of the value *must* be annotated.

Three load-bearing claims in this passage:

- *No `mut` allowed* — installed verbatim. The lesson's contrast
  probe `brokenmut.rs` grounds this with a captured rustc
  diagnostic.
- *Keyword is `const`, not `let`* — installed verbatim. The
  working probe shows the keyword.
- *Type annotation `must` be present* — installed verbatim. The
  lesson's primary contrast probe `broken.rs` grounds this with
  the captured `error: missing type for \`const\` item`
  diagnostic.

Lines 122-123 (the *any-scope-including-global-scope* claim):

> Constants can be declared in any scope, including the global
> scope, which makes them useful for values that many parts of
> code need to know about.

Corpus warrant for the lesson's fifth difference. The working
probe witnesses this directly: `THREE_HOURS_IN_SECONDS` at the
top of the source file, outside `fn main`, compiles and is
visible inside `fn main`.

Lines 125-126 (the *constant expression* rule):

> The last difference is that constants may be set only to a
> constant expression, not the result of a value that could only
> be computed at runtime.

Corpus warrant for the lesson's fourth difference. Captured
runtime-call contrast probe `brokenruntime.rs` grounds it with
`error[E0015]: cannot call non-const function`.

Lines 128-135 (the canonical example, used in the working probe):

> ```rust
> #![allow(unused)]
> fn main() {
> const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
> }
> ```

The lesson's working probe lifts `const THREE_HOURS_IN_SECONDS:
u32 = 60 * 60 * 3;` verbatim and places it at the global scope
(outside `fn main`) to additionally exercise the lines-122-123
claim. The Book wraps the example in a `fn main` so the example
can run inline; the lesson moves it to the global scope to
ground the scope-flexibility claim. The arithmetic right side
`60 * 60 * 3` is the Book's own choice.

Lines 137-141 (the *naming convention* claim):

> The constant's name is `THREE_HOURS_IN_SECONDS`, and its value
> is set to the result of multiplying 60 (the number of seconds
> in a minute) by 60 (the number of minutes in an hour) by 3 (the
> number of hours we want to count in this program). Rust's
> naming convention for constants is to use all uppercase with
> underscores between words.

Corpus warrant for the lesson's SCREAMING_SNAKE_CASE convention
claim. The lesson quotes "all uppercase with underscores between
words" verbatim. This is a convention, not a compiler-enforced
rule; the lesson does not test it with a probe (a lowercase
`const` name would compile with at most a `non_upper_case_globals`
lint warning, not an error — out of scope today).

Lines 142-146 (the *constant-expression compile-time evaluation*
clarification):

> The compiler is able to evaluate a limited set of operations at
> compile time, which lets us choose to write out this value in a
> way that's easier to understand and verify, rather than setting
> this constant to the value 10,800. See the [Rust Reference's
> section on constant evaluation](../reference/const_eval.md) for
> more information on what operations can be used when declaring
> constants.

Corpus warrant for two lesson claims: (a) "rustc evaluates
`60 * 60 * 3` at compile time to `10800`" — verified by running
the working probe and observing it prints `THREE_HOURS_IN_SECONDS
= 10800`; (b) the *What To Ignore For Now* deferral of "the full
set of operations allowed in constant expressions" to the
Reference's `const_eval.md`. The Book itself defers there.

### `output/docs/rust/reference/items/constant-items.md`

Cross-corroboration only — the Book covers everything load-
bearing for today. Two lines used:

- Line 19: "Constants are essentially inlined wherever they are
  used, meaning that they are copied directly into the relevant
  context when used." Used only as the *What To Ignore* gloss
  for *Constant propagation*; not load-bearing for any compile-
  or-run claim today.
- Line 27 (the *items.const.static* rule): "Constants must be
  explicitly typed." Independent corroboration of the Book's
  *must* rule on lines 117-118. Not separately cited in the
  lesson body since the Book is sufficient; named here so the
  Reference is on record as agreeing.

## Probes

### Probe 1 — working program (committed)

Captured at
`experimental/eduratchet2/runs/rust-moves/observations/075-const-declaration.rs`.
The committed file is the working program. Transcript:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- cat 075-const-declaration.rs ---
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    const MAX_POINTS: u32 = 100;
    println!("THREE_HOURS_IN_SECONDS = {}", THREE_HOURS_IN_SECONDS);
    println!("MAX_POINTS = {}", MAX_POINTS);
}
--- rustc 075-const-declaration.rs ---
exit=0
--- ls after compile ---
075-const-declaration
075-const-declaration.rs
--- ./075-const-declaration ---
THREE_HOURS_IN_SECONDS = 10800
MAX_POINTS = 100
exit=0
```

Three load-bearing observations:

- Both `const` declarations compile under one `rustc` invocation,
  with no diagnostic. The global one (`THREE_HOURS_IN_SECONDS`)
  is visible from inside `fn main`, grounding lines 122-123.
- The right side `60 * 60 * 3` is replaced in the printed output
  by `10800`, witnessing rustc's compile-time evaluation
  (lines 142-146).
- Two name styles work — the seven-token-with-underscores
  `THREE_HOURS_IN_SECONDS` and the two-token `MAX_POINTS` — both
  in SCREAMING_SNAKE_CASE per Book lines 140-141.

The probe uses `u32` and not `i32` to match the Book's canonical
example exactly (line 133: `const THREE_HOURS_IN_SECONDS: u32 =
60 * 60 * 3;`). Lesson 062 (`u32` annotation slot) is therefore
load-bearing on the prerequisites list. The arithmetic
`60 * 60 * 3 = 10800` and `100` both fit `u32` cleanly; integer
overflow is deferred (per *What To Ignore*).

### Probe 2 — primary contrast: missing `: TYPE` annotation

Not committed; transcript is the artifact.

```text
--- cat broken.rs ---
fn main() {
    const X = 5;
    println!("X = {}", X);
}
--- rustc broken.rs ---
error: missing type for `const` item
 --> broken.rs:2:12
  |
2 |     const X = 5;
  |            ^ help: provide a type for the constant: `: i32`

error: aborting due to 1 previous error

exit=1
```

This is the lesson's primary contrast. Grounds Book lines 117-120
("the type of the value *must* be annotated"): with the `: TYPE`
annotation present, the program compiles (probe 1 is the smoking
gun); without it, rustc rejects the program. The headline is
uncoded (`error:` with no `E####`); the caret sits between the
name and the `=`, exactly where the annotation would go; the
inline `help:` literally proposes `: i32` (the inferred-from-
literal type). The lesson reproduces this transcript verbatim
in *Try It* (lines 105-110 of the lesson body) and the *Check
Yourself* answer for (c).

The `: i32` suggestion is *not* a default for `const`; rustc
suggests it because the literal `5` would have been inferred to
`i32` by the lesson-019 default rule. The programmer is still
required to write the annotation. The lesson notes this in
*What To Ignore*.

### Probe 3 — auxiliary contrast: `const mut`

Not committed; transcript is the artifact.

```text
--- cat brokenmut.rs ---
fn main() {
    const mut X: u32 = 5;
    println!("X = {}", X);
}
--- rustc brokenmut.rs ---
error: const globals cannot be mutable
 --> brokenmut.rs:2:11
  |
2 |     const mut X: u32 = 5;
  |           ^^^ cannot be mutable
  |
help: you might want to declare a static instead
  |
2 -     const mut X: u32 = 5;
2 +     static mut X: u32 = 5;
  |

error: aborting due to 1 previous error

exit=1
```

Grounds Book lines 115-117 ("you aren't allowed to use `mut`
with constants. Constants aren't just immutable by default —
they're always immutable"): with `mut` present, the program does
not compile. The headline is uncoded (`error:` with no `E####`).
The caret sits under the literal `mut` keyword, with the inline
annotation `cannot be mutable`. The `help:` block proposes a
source-diff substitution to `static mut` — surfacing `static` as
a sibling form for the *What To Ignore* deferral.

Two additional pieces this probe gives the lesson:

- The phrase "const globals" in the rustc headline corroborates
  lines 122-123 — rustc itself uses the term "global" for `const`
  items.
- The `help:` block is the rustc citation for *What To Ignore*'s
  `static` deferral. The lesson reuses this hint directly.

This probe is referenced from the lesson body's *Check Yourself*
answer for (d) and from *What To Ignore* on `static`.

### Probe 4 — auxiliary contrast: non-`const fn` call on the right

Not committed; transcript is the artifact.

```text
--- cat brokenruntime.rs ---
fn add_one(n: u32) -> u32 {
    n + 1
}

fn main() {
    const X: u32 = add_one(5);
    println!("X = {}", X);
}
--- rustc brokenruntime.rs ---
error[E0015]: cannot call non-const function `add_one` in constants
 --> brokenruntime.rs:6:20
  |
6 |     const X: u32 = add_one(5);
  |                    ^^^^^^^^^^
  |
note: function `add_one` is not const
 --> brokenruntime.rs:1:1
  |
1 | fn add_one(n: u32) -> u32 {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: calls in constants are limited to constant functions, tuple structs and tuple variants

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0015`.
exit=1
```

Grounds Book lines 125-126 ("constants may be set only to a
constant expression, not the result of a value that could only
be computed at runtime"): with a non-`const fn` call on the
right side, the program does not compile. The headline is the
coded `error[E0015]: cannot call non-const function`. The caret
sits under the function-call expression. The `note:` block
points back at the function definition. The trailer is the
lesson-070 `--explain` form, but the lesson does not require
the learner to follow it today.

This probe surfaces the term "constant function" (`const fn`),
which the *What To Ignore* section names as the deferred next
move on this thread. The error code `E0015` is named in the
lesson's *What Changed* bullet about non-`const fn`.

## Prior lessons

Direct prerequisites — exact load-bearing claims (1-3 bullets each):

- `001-rustc-compile-and-run` (accepted) — the `rustc file.rs` /
  `./name` workflow used by all four probes.
- `002-fn-main-entry-point` (accepted) — the body of `fn main`
  runs when the executable launches; this is what makes
  `MAX_POINTS` in the working probe observable in the printed
  output.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — the
  four-part diagnostic map (headline / `-->` location / source
  excerpt with caret / help+note). Probes 2-4 are read with
  this map; the lesson body's contrast section instructs the
  reader to do exactly that.
- `005-let-binding` (accepted, load-bearing) — `let name = value;`
  is the established name-introduction form. `const` is the
  sibling form against which today's five differences are stated;
  every difference is "compared to lesson 005."
- `006-mut-binding` (accepted, load-bearing) — `let mut name = ...;`
  makes a binding reassignable; without `mut`, reassignment fails
  with E0384. Today's claim "constants do not accept `mut`" is
  the analogous-but-stronger rule (constants are always
  immutable, even with `mut`); the captured probe-3 error
  `const globals cannot be mutable` is the rustc form of this.
- `019-type-annotation-i32` (accepted, load-bearing) — the
  `let name: TYPE = value;` annotation form, with the rule that
  the annotation is *optional* (rustc usually infers). Today's
  difference (3) flips this: the `: TYPE` slot is *required* on
  `const`. The probe-2 diagnostic `error: missing type for
  \`const\` item` is the rustc form of "required."
- `062-u32-unsigned-integer` (accepted, load-bearing) — `: u32`
  plugs into the lesson-019 annotation slot. The Book's
  canonical example uses `u32`; the working probe uses `u32`
  to match. Without lesson 062, the lesson would have to either
  switch to `i32` (departing from the Book) or install `u32`
  inline (a second move).

Older supporting lessons — mention only:

- `004-statements-in-order` — `;`-terminated statements run top
  to bottom in `fn main`; restated in 005's prerequisites.
- `011-println-positional-args` — positional `{}` placeholder.
  Both probes' `println!` lines use it.
- `068-let-binding-scope` — *scope* as a region of code. Today
  extends "the enclosing `{ ... }` block" rule to include the
  *global scope* (outside every `fn`); the extension is itself
  the Book's claim on lines 122-123, not 068's.

Citations are surfaced in the lesson body via lesson numbers
only (per the audit-trail-depth standard); load-bearing claims
are restated above.

## Negative / contrast probes

Three contrast probes captured (probes 2-4 above). Each grounds
one of the five differences:

| Difference | Source line | Grounding probe |
|---|---|---|
| 1. keyword `const` not `let` | 117 | working probe (probe 1) shows the keyword in use |
| 2. no `mut` allowed | 115-117 | probe 3 fires `error: const globals cannot be mutable` |
| 3. `: TYPE` required | 117-120 | probe 2 fires `error: missing type for \`const\` item` |
| 4. constant expression only | 125-126 | probe 4 fires `error[E0015]: cannot call non-const function` |
| 5. any scope including global | 122-123 | working probe shows global-scope `const` visible inside `fn main` |

Difference 1 has no contrast probe of its own — using `let` at
global scope would fire a different error (`error: expected one of
\`!\` or \`::\`, found \`name\`` or `expected item`, depending on
parse path), which would muddy the lesson's narrative. The
working probe is sufficient: it uses the keyword `const` and
compiles; lesson 005 already established that `let` outside a
`fn` body is not the form the run has used. The five-difference
list is presented in the lesson body as a coherent block, with
probes for the four differences a learner could plausibly try
to violate (drop the annotation; add `mut`; call a runtime
function; declare inside a fn).

## Audience-vocabulary check

The lesson uses these words; each is either already-installed or
audience-glossable:

- *constant* / *constants* — installed today as the noun for
  `const` items.
- *constant expression* — new; glossed inline ("an expression
  rustc can evaluate at compile time using only literals, other
  constants, and basic arithmetic"). The full set of allowed
  operations is deferred.
- *compile time* / *runtime* — used informally since lesson 001
  (`rustc` runs at compile time, the executable runs at runtime);
  load-bearingly named in lessons 003, 019, 068. Not re-installed
  today.
- *scope* — installed by lesson 068 as "the region of code where
  a name has meaning."
- *global scope* — new today; glossed inline ("outside every
  `fn`, at the global scope of the source file"). The Book's
  own phrase, line 122.
- *immutable* — installed by lessons 005 / 006 (the immutable-
  by-default rule and `mut` for reassignability).
- *type annotation* — installed by lesson 019.
- *function call* — used informally since lesson 008 (`fn name()
  { ... }` definition + `name();` call).
- *`const fn`* — named only in *What To Ignore*; deferred.
- *static* — named only in *What To Ignore* (and surfaced by
  rustc in probe 3); deferred.

No other audience-level vocabulary is introduced.
