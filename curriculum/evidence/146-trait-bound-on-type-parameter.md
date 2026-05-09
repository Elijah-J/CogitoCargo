# Evidence — Lesson 146: a generic type parameter may carry a trait bound `<T: Trait>`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/146-trait-bound-on-type-parameter.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/146-trait-bound-on-type-parameter.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/146-trait-bound-on-type-parameter.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet146/` and compiled with
`rustc <file>`; resulting executables were run from the same
directory. Same host and toolchain as accepted lessons through 145.

## Run context — closure sub-arc step 4 decomposition (move 2 of 3)

Per `iterator-api-coverage.md` §6, the closure sub-arc has five
steps. Steps 1-3 (lessons 142-144) installed closure literals and
capture. The audit v2 §6 step 4 was sketched as a single move
"FnMut-bound parameter on a function." In execution it conflates
three distinct new mechanics:

1. *Generic function syntax* — `fn name<T>(...)` with no bound.
   Lesson 145, accepted commit `b3b1b0434`.
2. *Trait bound on the type parameter* — `<T: Trait>`. Today.
3. *The parenthesized `Fn(...)` / `FnMut(...) -> R` sugar plus
   closure-as-argument wiring* — lesson 147.

Today is mechanic (2) only. Lesson 145's body `{ t }` got away with
no bound because moving a value around does not require any trait;
*any* useful generic body needs a bound. Today installs the bound
syntax with `Display` (the simplest body-side use is
`println!("{}", t)`, which is lesson 011's exact form).

The audit document itself is not updated here; that is an
orchestrator action after 146 and 147 land.

## Direct prerequisite — lesson 145 (generic function type parameter)

Lesson 145 installed:

- The `<T>` slot on a `fn` header — between the function name and
  the parameter list.
- Per-call substitution: each call site picks a concrete type for
  `T` independently.
- The body of a generic function with no bound can do almost
  nothing with a `T` value except move it around (lesson 145's
  `id` body is `{ t }`).

Today reuses every claim. The new fact relative to 145 is the
*bound*: a colon and a trait path appended after the parameter
name, inside the same angle brackets. The new effect is that the
body can do more than move `t` around — it can use methods from
the trait.

## Direct prerequisite — lesson 011 (println! with positional placeholders)

Lesson 011 installed `println!("{}", arg)` — the positional
placeholder consumed by one extra argument. Today's body is
exactly this form. The `{}` placeholder is the call-site of the
`Display` trait — formatted output is what `Display` provides.
The trait bound `<T: std::fmt::Display>` is what makes the
placeholder usable on a value of generic type `T`; Probe 2's
E0277 diagnostic is rustc enforcing this requirement.

## Direct prerequisite — lesson 003 (rustc diagnostic map)

Lesson 003 installed the four-part diagnostic map. E0277 is a
*new* error code today. The diagnostic shape is unchanged: there
is a headline `error[E0277]: ...`, a `-->` location, a source
excerpt with caret, and a `help:` block proposing the fix. The
new feature is the *kind* of fix: a missing trait bound, written
inline with `+` markers under the inserted text. The
`= note:` line ("in format strings you may be able to use
`{:?}` (or `{:#?}` for pretty-print) instead") is a sibling
suggestion (use a different placeholder); the lesson body does
not center it because today's centered fact is "add the bound,"
not "switch to `Debug`."

## Cited prereqs (load-bearing-but-restated-elsewhere)

- **Lesson 081**: `5_u32`/`7_i32` literal-suffix forms. Used at
  the call sites to fix the argument types unambiguously.
- **Lesson 043**: nested module paths. The `std::fmt::Display`
  path reuses 043's `::`-separated multi-segment grammar; today's
  trailing segment is a trait name rather than a function name.
- **Lesson 080**: `u32`, `i32` are distinct integer types. Both
  implement `Display` (verified at
  `output/docs/rust/std/fmt/trait.Display.md:189-191` for `i32`
  and `:221-223` for `u32`).
- **Lesson 020**: parameter slot `t: T`. Today reuses it; the
  novelty is in the angle brackets, not the parameter list.
- **Lesson 008**: call shape `name(arg);`. Today reuses it for
  `say(5_u32)` and `say(7_i32)`.
- **Lesson 002**: `fn main`. **Lesson 001**: `rustc file.rs`,
  `./name`.

## Source — Reference trait-bounds.md (the equivalence and the body-can-call claim)

The corpus file `output/docs/rust/reference/trait-bounds.md`
contains the formal specification of trait bounds on generic
items. Verified by reading the file.

### Lines 38-42 (the inline-vs-where equivalence)

```text
Trait and lifetime bounds provide a way for generic items to
restrict which types and lifetimes are used as their parameters.
Bounds can be provided on any type in a where clause. There are
also shorter forms for certain common cases:

- Bounds written after declaring a generic parameter:
  `fn f<A: Copy>() {}` is the same as
  `fn f<A>() where A: Copy {}`.
```

This is the source for the lesson's What-To-Ignore-For-Now claim
that the inline form `<T: Trait>` and the `where T: Trait` form
are the same mechanic. The lesson body uses the inline form; the
`where` form is named-deferred. Verified at lines 38-42.

### Lines 44-50 (the body-can-call-Trait's-methods claim)

```text
Bounds on an item must be satisfied when using the item. When
type checking and borrow checking a generic item, the bounds can
be used to determine that a trait is implemented for a type. For
example, given `Ty: Trait`

- In the body of a generic function, methods from `Trait` can be
  called on `Ty` values. Likewise associated constants on the
  `Trait` can be used.
- Associated types from `Trait` can be used.
- Generic functions and types with a `T: Trait` bounds can be
  used with `Ty` being used for `T`.
```

The lesson body quotes the third bullet's phrasing: "in the body
of a generic function, methods from `Trait` can be called on
`Ty` values." This is the load-bearing claim that the bound
*enables the body* to use the trait's capabilities on a value of
the parameter type. Verified at lines 44-50.

The lesson's body-side use is `println!("{}", t)`. The `{}`
placeholder dispatches to the trait method `Display::fmt` (the
required method per `output/docs/rust/std/fmt/trait.Display.md:8-9`),
but the lesson does not name the method explicitly — the
audience-level claim is "the bound is what makes the placeholder
work on `t: T`," which is the user-visible consequence of the
Reference's "methods from `Trait` can be called" rule.

### Reference items/generics.md line 16 (the formal grammar)

```text
TypeParam → IDENTIFIER ( : TypeParamBounds? )? ( = Type )?
```

The formal grammar at `output/docs/rust/reference/items/generics.md:16`
shows the optional `: TypeParamBounds` slot after the identifier
inside angle brackets. Today's `<T: std::fmt::Display>` is one
instance of this grammar production. Verified at line 16.

## Source — Book ch10-02-traits.md (the textbook trait-bound section)

The corpus file `output/docs/rust/book/ch10-02-traits.md` covers
traits in the textbook. Verified by reading.

### Lines 384-397 (Trait Bound Syntax — the centered passage)

```text
#### Trait Bound Syntax

The `impl Trait` syntax works for straightforward cases but is
actually syntax sugar for a longer form known as a *trait bound*;
it looks like this:

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

This longer form is equivalent to the example in the previous
section but is more verbose. We place trait bounds with the
declaration of the generic type parameter after a colon and
inside angle brackets.
```

This is the source for the lesson body's centered grammar claim:
"with the declaration of the generic type parameter after a
colon and inside angle brackets." Verified at lines 384-397. The
Book's example uses a custom `Summary` trait declared earlier in
the chapter; today's lesson substitutes the standard-library
`Display` trait so the audience does not have to install a new
trait declaration first. The grammar slot is identical.

The Book at lines 386-388 names the inline `<T: Trait>` form as
the "longer form" relative to the `impl Trait` sugar at lines
369-372. Today's lesson uses the longer form (the inline
trait-bound form is what the prompt scoped); `impl Trait` in the
parameter position is a deferred related sugar.

### Lines 443-466 (the where-clause form)

```text
#### Clearer Trait Bounds with `where` Clauses

Using too many trait bounds has its downsides. Each generic has
its own trait bounds, so functions with multiple generic type
parameters can contain lots of trait bound information between
the function's name and its parameter list, making the function
signature hard to read. For this reason, Rust has alternate
syntax for specifying trait bounds inside a `where` clause after
the function signature.
```

The Book confirms the `where` clause form is alternate syntax
for the same mechanic, matching the Reference's verbatim
equivalence. Today's lesson defers the `where` form to a future
move and names it explicitly in *What To Ignore For Now*.
Verified at lines 443-466.

## Source — Display trait doc (the trait used for the bound)

The corpus file `output/docs/rust/std/fmt/trait.Display.md`
documents `std::fmt::Display`. Verified by reading.

### Lines 6-15 (trait declaration and one-line semantics)

```text
pub trait Display {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

Format trait for an empty format, `{}`.
```

Lines 6-10 give the trait's declaration; line 15 gives the
audience-level reading: `Display` is the trait that backs the
empty `{}` format placeholder. The lesson body uses this
relationship without naming the `fmt` method (deferred under
*What To Ignore For Now*).

### Lines 161-229 (Display impls for primitives)

The page's *Implementors* section lists `impl Display` for many
types. Each implementor block runs over two lines: a version
annotation line, then the heading line. Verified spans for the
implementors used by today's probes and side-probes:

- Line 163 heading: `impl Display for bool`
- Line 167 heading: `impl Display for char`
- Line 175 heading: `impl Display for f32`
- Line 179 heading: `impl Display for f64`
- Line 183 heading: `impl Display for i8`
- Line 191 heading: `impl Display for i32`
- Line 215 heading: `impl Display for u8`
- Line 223 heading: `impl Display for u32`

This is the source for the lesson body's claim that both `u32`
and `i32` implement `Display`. Both sides of the working probe
(`5_u32` and `7_i32`) are covered. Side-probe A's additional
types `bool` and `char` are also covered.

## Source — error_codes/E0277.md (the missing-trait-bound error)

The corpus file `output/docs/rust/error_codes/E0277.md` documents
E0277. Verified by reading.

### Lines 50-87 (the canonical example identical to today's contrast)

```text
Or in a generic context, an erroneous code example would look
like:

```
fn some_func<T>(foo: T) {
    println!("{:?}", foo); // error: the trait `core::fmt::Debug` is not
                           //        implemented for the type `T`
}
```

Note that the error here is in the definition of the generic
function. Although we only call it with a parameter that does
implement `Debug`, the compiler still rejects the function. It
must work with all possible input types. In order to make this
example compile, we need to restrict the generic type we're
accepting:

```
use std::fmt;

// Restrict the input type to types that implement Debug.
fn some_func<T: fmt::Debug>(foo: T) {
    println!("{:?}", foo);
}
```
```

This is the corpus's exact-shape match for today's lesson:
unbounded generic function whose body uses a formatting trait
fails E0277; adding the bound fixes it. The corpus uses `Debug`
(`{:?}`); today substitutes `Display` (`{}`) because lesson 011
already installed the `{}` placeholder and the audience does not
have a `Debug` placeholder yet. The fix mechanism is identical:
"restrict the generic type ... [with] `<T: TRAIT>`." Verified at
lines 50-87.

The corpus example uses a `use std::fmt;` declaration to bring
`fmt::Debug` into scope; today's lesson uses the fully-qualified
path `std::fmt::Display` directly because `use` is named-deferred
in this run.

## Probe 1 — working probe (Display bound; two integer types)

Source: `observations/146-trait-bound-on-type-parameter.rs`.
Transcript: `observations/146-trait-bound-on-type-parameter.transcript.txt` PROBE 1 block.

```rust
fn say<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}

fn main() {
    say(5_u32);
    say(7_i32);
}
```

Output:

```text
5
7
```

Compile exit 0, run exit 0. Three load-bearing structural facts
witnessed:

- The bound segment `<T: std::fmt::Display>` parses cleanly. No
  syntax error; the colon-then-trait-path form is accepted by
  rustc inside the angle brackets.
- The body `println!("{}", t)` compiles. The `{}` placeholder is
  legal on a value of type `T` *because* `T` carries the
  `Display` bound — the Reference's "methods from `Trait` can be
  called on `Ty` values" rule applied to `Display`'s `fmt` method.
- Both call sites work. `u32` and `i32` are distinct concrete
  types, both implementing `Display`; rustc accepts each
  substitution independently. The `5\n7\n` output is the pair of
  values, formatted.

## Probe 2 — negative contrast (drop the bound; body fails E0277)

Source `nobound.rs` (in transcript). The single-line modification
is dropping `: std::fmt::Display` from the angle brackets,
leaving `fn say<T>(t: T)` (the lesson 145 shape). Body and call
sites are unchanged.

Output:

```text
error[E0277]: `T` doesn't implement `std::fmt::Display`
 --> nobound.rs:2:20
  |
2 |     println!("{}", t);
  |               --   ^ `T` cannot be formatted with the default formatter
  |               |
  |               required by this formatting parameter
  |
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
help: consider restricting type parameter `T` with trait `Display`
  |
1 | fn say<T: std::fmt::Display>(t: T) {
  |         +++++++++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

Compile exit 1. Six grounded facts from rustc's mouth:

- The error code is `E0277` with the inline label
  `\`T\` doesn't implement \`std::fmt::Display\``. The diagnostic
  is the lesson 003 four-part shape unchanged (headline + `-->` +
  source excerpt with caret + `help:`). The `--explain E0277`
  trailer fires because the headline carries an `E####` code.
- Caret position: line 2 column 20 — the `t` token in
  `println!("{}", t)`. The `--` underline two columns earlier
  marks the `{}` placeholder as the *requirement source* (with
  inline label "required by this formatting parameter"), and the
  `^` underlines `t` as the value that fails to satisfy the
  requirement. Two annotated spans on one source line.
- Inline label on the caret: `\`T\` cannot be formatted with the
  default formatter`. "Default formatter" is rustc's phrasing
  for the `{}` placeholder (which dispatches to `Display`). The
  message names `T` (the unbounded generic parameter), not a
  concrete type — rustc is rejecting at *definition* time,
  matching the corpus claim at E0277.md:65-67 ("the error here
  is in the definition of the generic function").
- The `= note:` line proposes a sibling alternative: use `{:?}`
  (the `Debug` placeholder). The note exists because some types
  that lack `Display` *do* implement `Debug`. Today's lesson does
  not center this — switching the placeholder is a different
  fix axis from adding the bound. The note is captured here for
  full audit.
- The `help: consider restricting type parameter \`T\` with trait
  \`Display\`` block proposes the exact diff today's lesson
  installs: insert `: std::fmt::Display` after `T` in the angle
  brackets, with `+` markers under the 19 inserted characters
  (`:`, ` `, `s`, `t`, `d`, `:`, `:`, `f`, `m`, `t`, `:`, `:`,
  `D`, `i`, `s`, `p`, `l`, `a`, `y`). The fix rustc proposes *is*
  the lesson's centered move.
- The single-line modification — adding the bound — flips
  acceptance. Probe 1 and Probe 2 differ in one segment (the
  presence of `: std::fmt::Display` after `T`). Same body, same
  call sites, opposite outcomes. This is the contrastive witness
  for the lesson body's claim "the bound is what makes the
  placeholder legal for a value of type `T`."

## Side-probe A — Display bound accepts multiple primitive types

Source `extra.rs` (in transcript). Same `fn say<T: std::fmt::Display>`
declaration, but the call sites pass `u32`, `bool`, `char`. All
three implement `Display` per the std doc.

```rust
fn main() {
    say(5_u32);
    say(true);
    say('q');
}
```

Output:

```text
5
true
q
```

Compile exit 0, run exit 0. Witnesses that the bound's
"restricts call sites" rule accepts *any* type implementing
`Display` — not just integer types. The lesson body's centered
working probe stays inside the integer family for prerequisite
discipline (lesson 080 / 081), but this side-probe documents
that the mechanic is general. Not centered in the lesson body.

## Side-probe B — non-Display type fails E0277 at the call site

Source `notdisplay.rs` (in transcript). Same `fn say<T: std::fmt::Display>`
declaration; call site passes a custom `struct Widget;` that does
not implement `Display`.

```rust
fn say<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}

struct Widget;

fn main() {
    say(Widget);
}
```

Output:

```text
error[E0277]: `Widget` doesn't implement `std::fmt::Display`
 --> notdisplay.rs:8:9
  |
8 |     say(Widget);
  |     --- ^^^^^^ unsatisfied trait bound
  |     |
  |     required by a bound introduced by this call
  |
help: the trait `std::fmt::Display` is not implemented for `Widget`
 --> notdisplay.rs:5:1
  |
5 | struct Widget;
  | ^^^^^^^^^^^^^
note: required by a bound in `say`
 --> notdisplay.rs:1:11
  |
1 | fn say<T: std::fmt::Display>(t: T) {
  |           ^^^^^^^^^^^^^^^^^ required by this bound in `say`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

Compile exit 1. Witnesses the *call-site* enforcement of the
bound: rustc rejects the call `say(Widget)` because `Widget`
fails to satisfy `T: Display`. The diagnostic shape extends Probe
2's: now there are *three* annotated spans — the call line, the
type definition (`struct Widget;`), and the bound itself in the
function signature (`note: required by a bound in \`say\``). The
`note:` block ties the failing call back to the bound that was
violated, with its own `-->` line at line 1 column 11 — the
position of `std::fmt::Display` inside the angle brackets.

This is the symmetric contrast to Probe 2: Probe 2 shows the
"no bound" failure (body cannot use `{}`), Side-probe B shows
the "bound exists but unsatisfied" failure (call site cannot
pass a non-Display type). Together they witness both halves of
the lesson body's "two-things-at-once" claim — the bound
restricts call sites *and* enables the body. Side-probe B is not
centered in the lesson body because the lesson is scoped narrowly
to one new fact ("add `: TRAIT` after the type parameter") and
adding the second-failure-mode probe would expand the surface
beyond the centered move. It is captured here for audit.

## Probe-not-needed — turbofish

Today's substitution is inferred from the argument's type at
each call site (the lesson 145 mechanic, unchanged). The
explicit-substitution form `say::<u32>(5_u32)` is named in
*What To Ignore For Now* but unprobed; adding it would extend
the surface beyond the centered fact.

## Probe-not-needed — the where-clause form

The Reference at lines 38-42 verbatim names the equivalence
`fn f<A: Copy>() {}` ≡ `fn f<A>() where A: Copy {}`. Today's
lesson uses the inline form per the prompt's scope discipline.
A `where`-clause-shaped probe would compile and run identically;
the source equivalence is corpus-grounded and does not need
empirical verification.

## Probe-not-needed — `use std::fmt::Display;` shortened path

`use` declarations are deferred in this run. The fully-qualified
path `std::fmt::Display` is what the lesson body and probes use.
The corpus example at E0277.md:71-77 uses `use std::fmt;` then
writes `fmt::Debug`; today's lesson uses neither — the path
`std::fmt::Display` is fully qualified inline. Both forms compile
identically; the path-shortening is orthogonal to today's
centered fact.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| Lesson 145's `id` body could only move `t` around | Lesson 145 evidence (body `{ t }`); Reference `trait-bounds.md:46` (without a bound, no trait methods are callable on `Ty`) |
| `fn say<T: std::fmt::Display>(t: T)` parses and runs | Probe 1 transcript: compile-exit=0, run-exit=0 |
| Output `5\n7\n` | Probe 1 output |
| Bound = colon and trait path after parameter name inside angle brackets | Reference `items/generics.md:16` (formal grammar `IDENTIFIER ( : TypeParamBounds? )?`); Book `ch10-02-traits.md:396-397` (verbatim "after a colon and inside angle brackets") |
| Inline `<T: Trait>` is the same as `where T: Trait` | Reference `trait-bounds.md:40` (verbatim equivalence `fn f<A: Copy>() {}` ≡ `fn f<A>() where A: Copy {}`) |
| Body of generic function may use Trait's methods on `T` values | Reference `trait-bounds.md:48` (verbatim "in the body of a generic function, methods from `Trait` can be called on `Ty` values") |
| `{}` placeholder asks for `Display` | Display trait page `output/docs/rust/std/fmt/trait.Display.md:15` (verbatim "Format trait for an empty format, `{}`.") |
| Both `u32` and `i32` implement `Display` | Display page line 191 (`i32`) and line 223 (`u32`) |
| Probe 2 fires `error[E0277]: \`T\` doesn't implement \`std::fmt::Display\`` | Probe 2 transcript: rustc emits exact code + headline |
| Caret on `t` at line 2 column 20 | Probe 2 transcript: `--> nobound.rs:2:20` |
| `help: consider restricting type parameter \`T\` with trait \`Display\`` proposes `<T: std::fmt::Display>` | Probe 2 transcript verbatim |
| Without bound, rustc rejects the body at *definition* time | Probe 2 transcript (error fires on `nobound.rs:2:20` before any call); Corpus `error_codes/E0277.md:65-67` (verbatim "the error here is in the definition of the generic function ... It must work with all possible input types.") |
| Adding the bound is the lesson's centered fix | Probe 2's `help:` block proposes exactly that; Corpus E0277.md:69-77 shows the same fix structure with `Debug` |
| `Display` impls exist for `bool`, `char` (Side-probe A) | Display page line 163 (`bool`), line 167 (`char`); Side-probe A transcript |
| Non-Display type fails at call site (Side-probe B) | Side-probe B transcript: rustc emits E0277 with `note: required by a bound in \`say\`` |

## Older supporting lessons (named only)

The following accepted lessons are cited in the lesson body or
prerequisites; their exact prereq claims are restated above or
in the lesson's own Prerequisites bullets:

- 145-generic-function-type-parameter — `fn id<T>(t: T) -> T`.
- 011-println-positional-args — `println!("{}", t)`.
- 003-read-rustc-diagnostic — four-part diagnostic map.
- 081-integer-literal-forms — `5_u32`, `7_i32`.
- 043-nested-module-paths — `::`-separated multi-segment path grammar.
- 080-integer-type-family — `u32`, `i32` distinct.
- 020-function-with-parameter — `t: T` parameter slot.
- 008-define-and-call-function — `name(arg);` call shape.
- 002-fn-main-entry-point — `fn main`.
- 001-rustc-compile-and-run — `rustc file.rs`, `./name`.

## Deliberate scope discipline

The prompt named ten things to NOT touch. The lesson body's
*What To Ignore For Now* section names them:

1. The `where` clause form — same mechanic, deferred.
2. Multiple bounds with `+` — deferred.
3. Multiple type parameters with separate bounds — deferred.
4. The parenthesized `Fn(...)` / `FnMut(...) -> R` sugar — lesson 147.
5. `use` statements as a syntactic mechanic — fully-qualified
   path is what the lesson uses inline.
6. `Display`'s full trait surface — only that it makes `{}` work.
7. The implementor list of `Display` — named lightly.
8. Generic struct types or generic methods inside `impl<T>` — deferred.
9. Turbofish — deferred.
10. (Not in prompt list, but added defensively:) `&dyn Trait`
    trait objects — different machinery, deferred.

The body uses `println!("{}", t)` — lesson 011's exact form —
because today's audience already knows the placeholder. The
trait surface stops at "the bound makes the placeholder work."
The literal-suffix forms `5_u32`/`7_i32` follow the prompt's
recommendation and stay inside the integer family for
prerequisite discipline (lessons 080/081).

## Run-context handoff to lesson 147

Lessons 142-146 install:

- Closure literal syntax with annotated and unannotated
  parameters (142, 143).
- First-call-fixes-the-type rule for unannotated parameters
  (143).
- Closure-vs-`fn`-item asymmetry centered on capture (144).
- Generic function syntax `fn name<T>(...)` and per-call
  substitution (145).
- Trait bound on a generic function parameter `<T: Trait>` and
  how it restricts call sites and enables the body (today).

Lesson 147 will add the parenthesized `Fn(T) -> R` /
`FnMut(T) -> R` sugar (which is itself a special-case grammar
for trait bounds whose target is one of the `Fn`-family traits)
plus the closure-as-argument call site. After 147, the closure
sub-arc prereqs are complete and the first closure-driven
Iterator method becomes teachable (audit §4.4.1).

Today's `unlocks` lists lesson 147 directly, plus the deferred
bullets above.
