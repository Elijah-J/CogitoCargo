# Evidence — Lesson 143: drop the closure parameter annotation; the first call fixes its type

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/143-unannotated-closure-first-use.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/143-unannotated-closure-first-use.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/143-unannotated-closure-first-use.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet143/` and compiled with
`rustc <file>`; resulting executables were run from the same directory.
Same host and toolchain as accepted lessons through 142.

## Run context — second move of the closure sub-arc

Per `iterator-api-coverage.md` §6 (last revised commit `cb9945066`),
the closure sub-arc is the next major arc after lesson 141 closed the
closure-free Iterator surface. The sub-arc has five steps:

1. *Closure literal bound and called* (lesson 142, accepted).
2. *Closure type inference and the "first call fixes the type" rule*
   (today).
3. *Closure capturing an outer binding.*
4. *`FnMut`-bound parameter on a function.*
5. *`Fn` / `FnMut` / `FnOnce` distinction.*

Lesson 142's `unlocks` field names today's move directly: "future
'closure type inference and the first call fixes the type rule' moves
(audit §6 step 2 — the next move in the closure sub-arc; reuses
today's closure literal syntax with the parameter annotation
*dropped*)". Today's `depends_on` therefore names 142 load-bearingly.

## Direct prerequisite — lesson 142 (closure literal bound and called)

Lesson 142 installed:

- The closure literal syntax `|param: T| body` with a fully annotated
  parameter slot (Book v2 form for the parameter) and a brace-free
  single-expression body (Book v4 form for the body), bound to a
  `let` and called with parens.
- The closure is a *value with its own type* — rustc spells the type
  `{closure@<file>:<line>:<col>: <line>:<col>}`.
- The lesson explicitly named "closure type inference and 'first
  call fixes the type'" as the deferred next move (closure sub-arc
  step 2).

Today drops the parameter annotation (Book v3/v4 form for the
parameter slot — keeping Book v4 for the body unchanged). Everything
else from 142 — the pipe-bracket-pipe brackets, the
single-expression body, the `let`-bound closure callable with parens
— carries unchanged. The new fact relative to 142 is what the type
*becomes* when no annotation is written: rustc reads it off the first
call site, locking the closure to one concrete parameter type for
the rest of its life.

## Direct prerequisite — lesson 003 (rustc diagnostic map)

Lesson 003 installed the four-part diagnostic map: headline + `-->`
location + source excerpt with caret + optional `help:` / `note:`
trailers. Today's centered diagnostic uses every part of the map plus
one new feature: a *second* `-->` location inside a `note:` block,
pointing at a different line from the headline.

Today's lesson body says:

> "The first `note:` reads 'expected because the closure was earlier
>  called with an argument of type `u32`' and points back at line 3,
>  the *first* call."

This is structurally what lesson 003 captured — `note:` blocks can
carry their own source location. The new mechanic today is that the
location pointed at by the `note:` is not the same as the headline's
location: rustc is using the `note:` to draw a *cross-reference*
between two source lines.

## Direct prerequisite — lesson 081 (integer type suffix)

Lesson 081 installed the integer literal type suffix: `57u8` (or
`57_u8`, the underscore being optional) is an integer literal whose
type is fixed at the literal itself, equivalent to
`let n: u8 = 57;`. Today's call sites use `5_u32` and `5_i32`, the
same syntactic form.

Without lesson 081's suffix form, the negative-contrast probe would
not work: bare unsuffixed integer literals default to `i32` (lesson
019/080), so `id(5)` and `id(7)` would both pass `i32` and the
closure would resolve happily. The integer-suffix form is the smallest
mechanism for forcing two different integer types into two call sites
without bringing in `let`-binding-with-annotation as a separate
indirection.

## Direct prerequisite — lesson 080 (integer type family)

Lesson 080 named the twelve integer types and installed `u32` and
`i32` as two siblings differing in the leading `i`/`u` letter (signed
vs unsigned). Today's negative contrast rests on rustc treating these
two types as incompatible — `i32 ≠ u32`. The diagnostic confirms it:
`expected u32, found i32`.

## Cited prereqs (load-bearing-but-restated-elsewhere)

- **Lesson 005**: `let name = value;` — used three times in each probe.
- **Lesson 008**: call-with-parens shape `name(args)` — used at each
  call site. Today reuses lesson 142's reuse of 008 (calling a
  `let`-bound closure value with parens), with no new claim.
- **Lesson 011**: `println!("{}", name)` for the working probe;
  `println!("{} {}", a, b)` for Probe 2/3 surface. Two-placeholder
  println already installed.
- **Lesson 002**: `fn main` is the entry point.
- **Lesson 001**: `rustc file.rs`, `./name`; rustc silent on success.

## Source — Book ch13-01-closures.md (canonical first-call-fixes walk)

The lesson's load-bearing source is
`output/docs/rust/book/ch13-01-closures.md`. The canonical
first-call-fixes-the-type passage runs lines 225-284. Three
load-bearing chunks:

### Lines 225-232 (the rule, before the example):

```text
For closure definitions, the compiler will infer one concrete type for each of
their parameters and for their return value. For instance, Listing 13-3 shows
the definition of a short closure that just returns the value it receives as a
parameter. This closure isn't very useful except for the purposes of this
example. Note that we haven't added any type annotations to the definition.
Because there are no type annotations, we can call the closure with any type,
which we've done here with `String` the first time. If we then try to call
`example_closure` with an integer, we'll get an error.
```

This is the source for the lesson's claim "rustc infers the
parameter type (and the return type) from the *first* call site". The
phrase "one concrete type for each of [the closure's] parameters"
grounds the lesson's "Closures are *not* generic" framing.

### Lines 236-243 (Listing 13-3 source):

```rust
fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
}
```

The Book's canonical probe. Today's working probe substitutes
`String::from("hello")` (named-deferred since lesson 042) and the
unsuffixed `5` (would default to `i32`) with the integer-suffix form
`5_u32` and `5_i32` (lesson 081, lesson 080) so the prereqs stay
within already-installed material. The structural shape — identity
closure `|x| x`, two calls with different argument types, second
call rejected — is preserved.

### Lines 252-279 (the diagnostic block):

```text
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |             --------------- ^ expected `String`, found integer
  |             |
  |             arguments to this function are incorrect
  |
note: expected because the closure was earlier called with an argument of type `String`
 --> src/main.rs:4:29
  |
4 |     let s = example_closure(String::from("hello"));
  |             --------------- ^^^^^^^^^^^^^^^^^^^^^ expected because this argument is of type `String`
  |             |
  |             in this closure call
note: closure parameter defined here
 --> src/main.rs:2:28
  |
2 |     let example_closure = |x| x;
  |                            ^
help: try using a conversion method
  |
5 |     let n = example_closure(5.to_string());
  |                              ++++++++++++
```

This is the canonical diagnostic the lesson body reproduces in
substituted form. Probe 2's transcript matches this structure
exactly:

- Headline `error[E0308]: mismatched types` at the *second* call
  site (line 4 in twocalls.rs, line 5 in the Book).
- First `note:` "expected because the closure was earlier called
  with an argument of type X" at the first call site (line 3 in
  twocalls.rs, line 4 in the Book).
- Second `note:` "closure parameter defined here" at the closure
  literal's `x` token (line 2 in both).
- A `help:` block proposing how to fix the *argument*, not the
  closure.

The substituted types (u32 / i32 instead of String / integer) change
only the contents of the `expected X, found Y` slots; the diagnostic
*structure* is identical. Probe 2 + Probe 3 (order swap) ground the
asymmetric "first-call wins" claim that the Book asserts in prose.

### Lines 281-284 (the rule, after the example):

```text
The first time we call `example_closure` with the `String` value, the compiler
infers the type of `x` and the return type of the closure to be `String`. Those
types are then locked into the closure in `example_closure`, and we get a type
error when we next try to use a different type with the same closure.
```

The verbatim canonical phrasing of today's centered fact, by the
authors. The lesson body quotes the load-bearing pieces of this
sentence ("Those types are then locked into the closure ... we get
a type error when we next try to use a different type").

### Note on the prompt's line range

The orchestrator prompt named `:238-274` for this canonical walk and
explicitly asked the worker to verify before citing. The actual span
including both the rule statement and the diagnostic is `:225-284`;
the prompt's `:238-274` covered Listing 13-3's source (236-243) and
most of the diagnostic block (247-279) but stopped before the
"locked into the closure" rule statement at 281-284. The lesson cites
the wider span. Listing 13-3's `|x| x` is the model the lesson's
identity-closure probes inherit.

## Probe 1 — working probe (unannotated closure, both calls u32)

Source: `observations/143-unannotated-closure-first-use.rs`.
Transcript: `observations/143-unannotated-closure-first-use.transcript.txt` Probe 1 block.

```rust
fn main() {
    let id = |x| x;
    let a = id(5_u32);
    let b = id(10_u32);
    println!("{}", a);
    println!("{}", b);
}
```

Output:

```text
5
10
```

Compile exit 0, run exit 0. Three load-bearing structural facts witnessed:

- The unannotated closure literal `|x| x` parses and binds — rustc does
  not require the parameter annotation that lesson 142 wrote.
- The first call `id(5_u32)` succeeds, fixing `x: u32`. The second
  call `id(10_u32)` is consistent with that inferred type and also
  succeeds. The two output lines `5` and `10` confirm both calls
  evaluated the body as identity (returning the argument unchanged).
- Rustc inferred the body's return type from the parameter type the
  same way it would for an annotated closure — printing `u32` values
  through the lesson-011 `{}` placeholder works without any extra
  annotation.

This grounds the "unannotated closure compiles" half of today's
centered fact. Probe 2 and Probe 3 ground the "first call fixes the
type" half.

## Probe 2 — negative contrast (first u32, second i32)

Source `twocalls.rs`. The lesson body reproduces this diagnostic
verbatim. Output exit 1; full transcript above. Three grounded facts
from rustc's mouth:

- The headline `error[E0308]: mismatched types` is reported at the
  *second* call site (line 4: `let b = id(5_i32);`), not at the
  closure literal on line 2. Rustc accepted `|x| x` cleanly. The
  inline label spells `expected u32, found i32`.
- The first `note:` block reads "expected because the closure was
  earlier called with an argument of type `u32`" with its own `-->`
  pointing at line 3 (`let a = id(5_u32);`). This is the centered
  diagnostic feature: rustc explicitly cross-references the *first*
  call as the source of the expectation.
- The second `note:` block reads "closure parameter defined here"
  with `-->` pointing at line 2's `x` inside the `|x|` literal — the
  parameter slot whose type just got fixed.

The `help:` block suggests changing `5_i32` to `5_u32` (i.e., changing
the *argument*, not the closure). No alternative help is offered. This
is the empirical witness for the lesson's "closures are not generic
over the parameter type" claim: rustc treats the closure as having
*one* fixed parameter type and asks the caller to conform.

## Probe 3 — directionality witness (first i32, second u32)

Source `orderswap.rs`. Same source as `twocalls.rs` minus the order
of the two call sites. Output exit 1. The diagnostic flips:

- Headline at line 4 with `expected i32, found u32` (opposite of
  Probe 2).
- First `note:` "expected because the closure was earlier called
  with an argument of type `i32`" at line 3 — pointing at the new
  first call.
- Second `note:` "closure parameter defined here" at line 2's `x`,
  unchanged.

The flip rules out the alternative reading that rustc somehow
prefers `u32` or `i32` independently. Only the *order* of the two
calls determines which type wins. This is the empirical asymmetry
that grounds the word "first" in "first call fixes the type".

Probe 2 + Probe 3 together are the negative/contrast probes for
today's lesson. The direct prerequisite-lesson 003 captured single-
location diagnostics; today's diagnostics span two source locations
and the direction matters, so two probes are necessary.

## Probe-not-needed — bare-unsuffixed integer body case

The orchestrator prompt suggested the body `|x| x + 1` as an
alternative to the identity body `|x| x`. I did test it
(`/tmp/eduratchet143/inferbody.rs` and `inferbroken.rs`):

- `|x| x + 1` with both calls suffixed `5_u32` and `10_u32`: compiles
  silently, outputs `6` and `11`.
- `|x| x + 1` with first call `5_u32` and second `5_i32`: emits the
  same E0308 diagnostic shape as Probe 2, with the same
  "earlier called" `note:` block.

The body shape is structurally not load-bearing for today's centered
fact — the diagnostic format is identical for `|x| x` and
`|x| x + 1`, the only difference being whether the body involves
arithmetic. The identity body `|x| x` is preferred for today because
(a) it is the Book's exact Listing 13-3 form, (b) it isolates the
inference question (the body returns the argument with no
arithmetic, so there is one type to infer, not "is `x + 1` a `u32`
or `i32`?"), (c) the lesson's `Try It` and `Check Yourself` blocks
need only one moving part. The arithmetic body is documented in this
appendix only to record that the alternative was tested and produces
the same diagnostic.

The Book at `:238` writes the closure as `|x| x` for exactly this
reason: "a short closure that just returns the value it receives as
a parameter ... isn't very useful except for the purposes of this
example."

## Probe-not-needed — letting the parameter type leak from the body

A different deferred shape (raised in the prompt's "Probe 1" suggested
form) is: write `|x| x + 1` with no suffixes, then let the body's
default-`i32` literal `1` constrain `x: i32`, and witness rustc rejecting a
later `let n: u32 = f(5)` with E0271. I ran a related shape during
lesson 142's evidence collection (see lesson 142's evidence appendix
"Probe-not-run — inferred-type case (E0271)") and it produced
`error[E0271]: type mismatch resolving <i32 as Add>::Output == u32`
because rustc infers `x: i32` from the body, not from the call site.

This is *also* a "first-use fixes the type" instance, but the "first
use" is the *body* not a call. Including this in today's lesson would
require explaining default-integer-literal behavior in the body
(separate from default-integer at the call site, which lesson 081's
suffix bypasses) and would not strengthen the centered claim. The
Book's canonical walk uses identity body `|x| x` precisely because
identity has no body literal to seed inference from — the call site
is unambiguously "first use". Today's lesson follows the Book.

## Probe — Check Yourself prediction grounded

Source `q.rs`:

```rust
fn main() {
    let f = |x| x;
    let _a = f("hi");
    let _b = f(7_i32);
}
```

I ran this during evidence collection. Output exit 1, with diagnostic:

```text
error[E0308]: mismatched types
 --> q.rs:4:16
  |
4 |     let _b = f(7_i32);
  |              - ^^^^^ expected `&str`, found `i32`
  |              |
  |              arguments to this function are incorrect
  |
note: expected because the closure was earlier called with an argument of type `&'static str`
 --> q.rs:3:16
  |
3 |     let _a = f("hi");
  |              - ^^^^ expected because this argument is of type `&'static str`
  |              |
  |              in this closure call
note: closure parameter defined here
 --> q.rs:2:14
  |
2 |     let f = |x| x;
  |              ^
```

The lesson's Check-Yourself answer matches this transcript: the
headline points at line 4 with `expected &str, found i32`, and the
`note:` at line 3 ties back to the first call. The minor wrinkle —
the headline says `&str` while the `note:` says `&'static str` — is
called out in the answer prose. Both `&str` and `&'static str` are
named-deferred today.

Side-probe `q.rs` is referenced in the lesson but not committed as
its own observation file because it would duplicate the Probe 2/3
shape with only the type substitution (string vs integer) changed —
and `&str` / `&'static str` are not yet installed types. The
diagnostic is documented here for audit.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| Lesson 142 wrote `\|x: u32\| x + 1` with parameter annotation | Lesson 142 (accepted) |
| Book ch13-01:208-213 lined up four forms | Book corpus, verified line range |
| `\|x\|` is the parameter list with no type annotation (Book v3/v4) | Book corpus :208-213 |
| `\|x\| x` parses and binds | Probe 1 transcript (compile-exit=0) |
| `id(5_u32)` and `id(10_u32)` produce `5` and `10` | Probe 1 output |
| Rustc reads `x: u32` off the *first* call | Probe 2 `note:` block ("expected because the closure was earlier called with an argument of type `u32`") |
| The headline reports E0308 at the second call, not the closure literal | Probe 2 transcript |
| Closures are *not* generic; the closure binds *one* concrete parameter type per parameter, fixed at first use | Book corpus :225-232 ("the compiler will infer one concrete type for each of their parameters") + :281-284 ("locked into the closure") + Probe 2 (rustc's `help:` does not offer making the closure accept both types) |
| Order matters; swapping calls flips the expected type | Probe 3 transcript |
| The Book quote at :225-232 and :281-284 | Book corpus, verified |
| `error[E0308]: mismatched types` headline structure | Lesson 003 (diagnostic map); Probe 2/3 transcripts |
| Two-placeholder `println!("{} {}", a, b)` | Lesson 011 (already installed; Probe 2/3 sources) |
| Check-Yourself answer (line 4 `--> q.rs:4:16`, headline `expected &str, found i32`, note pointing at line 3) | Side-probe `q.rs` transcript (this appendix) |

## Older supporting lessons (named only)

The following accepted lessons are cited in the lesson body but their
exact prereq claims are restated either in the direct-prereq sections
above or in the lesson's own Prerequisites bullets:

- 008-define-and-call-function — call-with-parens shape.
- 011-println-positional-args — `println!("{}", name)`.
- 005-let-binding — `let name = value;`.
- 002-fn-main-entry-point — `fn main` is the entry point.
- 001-rustc-compile-and-run — `rustc file.rs`, `./name`.
- 081-integer-literal-forms — type-suffix `5_u32`.
- 080-integer-type-family — `u32` and `i32` are sibling integer types.

## Deliberate scope discipline (per audit §6 step 2)

The orchestrator prompt names six things to NOT touch:

1. The `Fn`/`FnMut`/`FnOnce` trait family — deferred to steps 4-5.
2. Closure capturing outer bindings — deferred to step 3.
3. Generic functions over closures — deferred to step 4.
4. Passing closures to functions — deferred to step 4.
5. `impl Fn` / `Box<dyn Fn>` return forms — deferred.
6. The `{closure@<loc>}` opaque-type spelling as a *category* — lesson
   142 already named it from rustc's mouth; today does not deepen.

The lesson body's `What To Ignore For Now` section names all six
explicitly with their step number where applicable. Probe 2/3 transcripts
do not surface the `{closure@<loc>}` name (the diagnostic this time
reports `expected u32, found i32`, not a closure-typed mismatch — the
mismatch is at a call argument, and rustc names integer types). The
deferred list is honored.

## Run-context handoff to step 3

Lesson 142 plus today's lesson installs:

- The closure literal syntax with both annotated (142) and
  unannotated (today) parameter slots.
- The closure-as-value framing (142) plus the "one concrete parameter
  type fixed at first use" framing (today).

This is enough to read code that uses closures. The remaining steps
of the closure sub-arc are about *what closures can capture* (step 3:
outer bindings) and *what closures can be passed to* (steps 4-5:
generic functions with `Fn`/`FnMut`/`FnOnce` bounds). Today's
unlocks list names step 3 as the natural next move.
