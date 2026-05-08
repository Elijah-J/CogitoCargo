---
id: 029-unit-type
move: "name the recurring \"nothing useful\" type from prior lessons — it is *the unit type*, written `()`; its one value is also written `()`; bind it explicitly with `let _name: () = ();` (or via an empty block, or a no-`->` function call)"
main_concept: "`()` is a real Rust type with exactly one value, also written `()`; it is the type Rust uses when there is no other meaningful value; three places it shows up in code already accepted in this run: (1) the literal `()` itself; (2) any block `{ ... }` whose final inner line ends in `;`, including the empty block `{}` (lesson 024); (3) a call to a function declared without `-> RTYPE` (lesson 021); the corpus name for `()` is the *unit type*, with the *unit value* spelled the same way; this lesson resolves the calibration debt accrued in lessons 021, 024, 025, 026, 027, and 028, all of which referred to `()` as \"nothing useful / no value\""
depends_on:
  - 001-rustc-compile-and-run
  - 002-fn-main-entry-point
  - 003-read-rustc-diagnostic
  - 005-let-binding
  - 008-define-and-call-function
  - 019-type-annotation-i32
  - 021-function-return-value
  - 024-statement-vs-expression
  - 025-implicit-return
  - 026-if-as-expression
  - 027-loop-and-break
  - 028-break-value
assumptions:
  - same ordinary computer-use assumptions as lesson 001 (terminal, plain-text editor, rustc on PATH, Linux/macOS shell)
unlocks:
  - future "tuple types with non-zero arity `(i32, i32)` etc." moves
  - future "the `_` binding-name convention and `_` wildcard pattern" moves
  - future "rustc warnings vs errors as a category distinction" moves
  - future "`if`-without-`else` evaluating to `()`" moves
  - future "`Result<(), E>` as a return type for `fn main`" moves
  - future "pattern-matching `()` and tuple destructuring `let (a, b) = (1, 2);`" moves
sources:
  - output/docs/rust/book/ch03-02-data-types.md
  - output/docs/rust/reference/types/tuple.md
probes:
  - experimental/eduratchet2/runs/rust-moves/observations/029-unit-type.rs
status: accepted
---

# The unit type `()` — naming the recurring "nothing useful" type

## The Move

Six prior lessons (021, 024, 025, 026, 027, 028) referred to `()` as
"nothing useful" or "no value" and explicitly deferred a real
introduction. This lesson installs it.

`()` is a real Rust type, called the *unit type*. It has exactly one
value, also written `()`, called the *unit value*. You can bind it
directly:

```rust
let _name: () = ();
```

You can also obtain a `()` from places already in this run's graph: an
empty block `{}` and a call to a function whose signature has no
`-> RTYPE`. All three forms compile under the same annotation `: ()`.

## Mental Model Delta

- Before: "rustc keeps printing `()` in diagnostics — `expected (),
  found integer`, `implicitly returns ()`, `expected i32, found ()`.
  I have been reading that as 'nothing useful' since lesson 021. It
  is not a value I have ever bound or written myself."
- After: "`()` is a real Rust type. It has exactly one value, written
  `()` too. Three places in code I have already accepted produce a
  `()` value: the literal `()`; the empty block `{}` (and any block
  whose tail line ends in `;` per lesson 024); and a call to a
  function with no `-> RTYPE` (lesson 021). The diagnostics from
  lessons 024, 025, 028 were rustc naming the type by name. There is
  no longer a calibration debt — `()` is just `()`."

## Prerequisites

- Installed concepts:
  - Lesson 001: `rustc file.rs` then `./name`, silent on success.
  - Lesson 002: body of `fn main` runs when the executable launches.
  - Lesson 003 (load-bearing): rustc diagnostics have a headline +
    `-->` location + source excerpt with caret + optional sub-lines.
    The broken-contrast walk decodes the E0308 in this lesson with
    exactly that skill.
  - Lesson 005 (load-bearing): `let name: TYPE = value;` binds a name;
    the slot the unit value lands in.
  - Lesson 008: `fn name() { ... }` plus `name();`. The probe defines
    `say_hello()` and calls it as a value-producing expression of
    type `()`.
  - Lesson 019 (load-bearing): `name: TYPE` attaches a type. The
    probe writes `: ()` as the type annotation; the broken-contrast
    diagnostic phrases the error in terms of this annotation.
  - Lessons 021, 024, 025, 026, 027, 028 (calibration
    carry-forward): each lesson named `()` in passing — as a
    function's "nothing useful" return (021); as the value of a
    block with a `;`-terminated tail (024); as rustc's
    `implicitly returns ()` body annotation (025); as the value of
    `if`-without-`else` (026); as the value of `loop { ...; break; }`
    (027); as `found ()` in the `expected i32, found ()` diagnostic
    (028) — and deferred a real introduction. This lesson resolves
    that debt. The broken-contrast probe's `expected (), found
    integer` is the reverse direction of lesson 028's diagnostic.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn say_hello() {
    println!("inside say_hello");
}

fn main() {
    let _unit_literal: () = ();
    let _empty_block: () = {};
    let _function_call: () = say_hello();
    println!("three () bindings compiled");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
inside say_hello
three () bindings compiled
```

The new piece is the *type annotation* `: ()` on the left of each
`let`, paired with three different right-hand sides that all produce
a `()` value:

1. `let _unit_literal: () = ();` — the literal `()` on the right.
   Tautological but the smallest possible existence proof: the unit
   value, written exactly the same way as its type, bound to a name
   under that type annotation.

2. `let _empty_block: () = {};` — an empty block. By lesson 024's
   tail-expression rule, `{}` has no tail expression, so its value
   is `()`. This is the limit case of "block whose tail line has
   `;`" — the block has no tail line at all.

3. `let _function_call: () = say_hello();` — a call to a function
   whose signature has no `-> RTYPE`. Lesson 021 said such a
   function returns "nothing useful". The full corpus name for that
   "nothing useful" is `()`. The call expression `say_hello()` has
   type `()`.

The fact that all three right-hand sides type-check against the
annotation `: ()` is the existence proof. `rustc demo.rs` exits 0
silently and the executable runs, printing the line from
`say_hello`'s body and then the line from `main`.

A small Rust convention is in play: each binding name starts with
`_`. rustc warns when you bind a name and never use it; the leading
`_` says "I know — I do not intend to use this name." Without the
underscores, this probe still compiles, but rustc prints three
`unused_variables` warnings. Warnings differ from the errors you
have seen so far (lesson 003's E0601, E0308, E0425 etc.) — they do
not stop compilation, and the executable is still produced. The
`_`-prefix convention is glossed here, not installed; warnings as a
category are deferred too.

Now do the contrast. In the same directory, save a second file
`broken.rs`:

```rust
fn main() {
    let n: () = 5;
    println!("never reached");
}
```

Compile it. The full transcript and a part-by-part walk live in
`## Evidence`; reading it with lesson 003's order:

- *Headline*: `error[E0308]: mismatched types`. Same E-code as
  lessons 024, 025, 026, 028.
- *`-->` location*: `broken.rs:2:17` — column 17 of line 2, the
  literal `5`.
- *Source excerpt*: dashes `--` underline the `()` annotation with
  the sub-line `expected due to this`; the caret `^` underlines the
  `5` with `expected (), found integer`.

That diagnostic is the *reverse direction* of lesson 028's. Lesson
028 had `expected i32, found ()` — the binding wanted an `i32` and
the loop produced `()`. This one has `expected (), found integer` —
the binding wants `()` and the literal `5` is an integer. Both
diagnostics confirm the same rule: `()` is its own type, distinct
from any integer type. rustc will not silently coerce one to the
other.

## What Changed

- You have a name for the type rustc has been printing as `()` since
  lesson 021. It is the *unit type*. Its single value is the *unit
  value*. Both are written `()`.
- You can write `let name: () = ();` directly. You can also obtain
  the unit value from `{}` (empty block, lesson 024) or from a call
  to a function with no `-> RTYPE` (lesson 021).
- The diagnostics in lessons 024, 025, 028 (`found ()`,
  `implicitly returns ()`, `expected i32, found ()`) were rustc
  *naming* the unit type. The calibration debt accrued in lessons
  021, 024, 025, 026, 027, and 028 is now closed.
- `()` is its own type, not an integer or any other primitive.
  rustc rejects `let n: () = 5;` with E0308 `expected (), found
  integer` — the reverse direction of lesson 028's diagnostic.
- A small unrelated Rust convention is now familiar: prefixing a
  binding name with `_` tells rustc you intentionally do not use the
  name, suppressing the `unused_variables` warning. Warnings differ
  from errors in that they do not stop compilation; both the
  `_`-prefix convention and warnings-vs-errors as categories are
  deferred to future moves.

## Check Yourself

You write `tiny.rs` containing:

```rust
fn nothing() {
}

fn main() {
    let _a: () = nothing();
    let _b: () = { let x: i32 = 1; };
    println!("done");
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) The right-hand side of `_b`'s binding is a block. By lesson
024's rule, what is its value, and why?

(c) If you change `let _a: () = nothing();` to
`let _a: i32 = nothing();`, what E-code does rustc emit, and what
does it say after `expected` and after `found`?

(Answers: (a) Yes; prints `done`. (b) The block is
`{ let x: i32 = 1; }` — its only inner line is a `let` statement
ending in `;`, so the block has no tail expression and its value is
`()`. The annotation `: ()` matches. (c) `error[E0308]: mismatched
types` with the source excerpt saying `expected i32, found ()`. The
reverse of this lesson's broken-contrast and the same direction as
lesson 028's.)

## What To Ignore For Now

- *Tuple types with non-zero arity* — `(i32, i32)`, `(f64, String)`
  etc. The Book (line 314) and the Reference (line 34) both frame
  `()` as the 0-arity *tuple type*. Tuples with two-or-more fields
  are real Rust types, but they are deferred. This lesson installs
  only `()`; it does NOT install tuple types in general.
- *The `_` prefix as a binding-pattern convention*, and `_` as a
  wildcard pattern in `match`/`let`. Glossed in passing in Try It;
  not installed. Future move (probably with `match`).
- *Warnings vs errors as a category distinction in rustc
  diagnostics.* Lesson 003 covered errors only; warnings have a
  different headline color and do not stop compilation. Mentioned
  in passing; future move.
- *`Termination` as a return type for `fn main`* — `main` can also
  return `Result<(), E>`. Carried-forward from lesson 002. Future
  move; will reuse `()` as installed here.
- *`mem::size_of::<()>()`* — the unit type has size 0. Reference
  fact; not installed.
- *Pattern-matching `()` and tuple destructuring* like
  `let (a, b) = (1, 2);`. Future move with tuples.
- *`if`-without-`else` evaluating to `()`* — mentioned at lessons
  026 and 027. Now that `()` is named, this can become its own
  small move whenever the orchestrator picks it up.
- All previously deferred items.

## Evidence

### Sources

Two corpus sources, both 1-2 sentences each. Both treat `()` as the
0-arity tuple type; this lesson installs only the 0-arity case.

- `output/docs/rust/book/ch03-02-data-types.md`, line 314:

  > The tuple without any values has a special name, *unit*. This
  > value and its corresponding type are both written `()` and
  > represent an empty value or an empty return type. Expressions
  > implicitly return the unit value if they don't return any other
  > value.

  This licenses the lesson's two main claims: `()` is both the type
  name and the value name; expressions produce `()` when they have
  no other value to produce. The Book introduces `()` inside its
  Tuples subsection; this lesson honors the original framing by
  noting that tuple types with non-zero arity exist but are
  deferred.

- `output/docs/rust/reference/types/tuple.md`, line 34:

  > For convenience and historical reasons, the tuple type with no
  > fields (`()`) is often called *unit* or *the unit type*. Its
  > one value is also called *unit* or *the unit value*.

  This is the canonical Reference statement of the same fact: `()`
  is the tuple type with zero fields, conventionally named "unit"
  or "the unit type". Line 47 of the same page adds: "various
  expressions will produce the unit value if there is no other
  meaningful value for it to evaluate to" — which matches the
  Book's framing and underwrites the three sources of `()` shown
  in the probe.

Calibration:
- Both sources frame `()` as the 0-arity tuple type. Tuples with
  non-zero arities (`(i32, i32)`, `(f64, String)`, etc.) are real
  Rust types but explicitly deferred. This lesson installs only
  `()`. The framing as "0-arity tuple" is mentioned in prose but
  not made load-bearing; the load-bearing fact is "`()` is a type
  with one value, also written `()`."
- The Book builds with `cargo run`; this lesson uses `rustc
  demo.rs` per lesson 001. Behavior identical.
- The probe binds `_`-prefixed names to suppress the
  `unused_variables` warning, a rustc convention. Without the
  prefix, the probe still compiles cleanly (warnings do not stop
  compilation), but rustc prints three warnings. This is glossed in
  Try It and What Changed; the `_`-prefix convention and
  warnings-vs-errors are deferred as separate future moves.

### Probes

Two probes were captured on rustc 1.95.0 (59807616e 2026-04-14) on
Darwin x86_64. The working probe is committed at
`experimental/eduratchet2/runs/rust-moves/observations/029-unit-type.rs`.
The broken-contrast probe is *not* committed under `observations/`;
its transcript is reproduced verbatim below.

Both probes were run in temp directories created with `mktemp -d`
and removed at the end.

#### Working probe

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn say_hello() {
    println!("inside say_hello");
}

fn main() {
    let _unit_literal: () = ();
    let _empty_block: () = {};
    let _function_call: () = say_hello();
    println!("three () bindings compiled");
}
--- rustc demo.rs (capturing stderr) ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
inside say_hello
three () bindings compiled
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 *and is silent* — no warnings printed.
  The `_`-prefix convention on each binding name is what keeps the
  output clean; without it, rustc would print three
  `unused_variables` warnings. Warnings do not stop compilation
  (separate future move).
- The two output lines are jointly the load-bearing observation:
  - `inside say_hello`: confirms `say_hello()` was actually called
    and produced its `println!` side effect. The third `let`'s
    right-hand side is therefore a real call, not a no-op.
  - `three () bindings compiled`: confirms the program reached the
    `println!` after the three bindings, which it could only do if
    all three `let` lines type-checked. That all three right-hand
    sides accept the annotation `: ()` is the existence proof for
    the move.
- The annotation `: ()` is required on each `let` for this lesson's
  point: it pins the expected type to the unit type explicitly,
  rather than relying on rustc's inference. (rustc's inference
  would also produce `()` here in the absence of the annotation,
  but writing the annotation makes the type-check verifiable.)

#### Broken-contrast probe

The probe `broken.rs` exists for the broken-contrast walk; the
transcript below is the artifact.

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
broken.rs
--- cat broken.rs ---
fn main() {
    let n: () = 5;
    println!("never reached");
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:2:17
  |
2 |     let n: () = 5;
  |            --   ^ expected `()`, found integer
  |            |
  |            expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Notes:

- The headline `error[E0308]: mismatched types` is the generic
  E0308 form (same as lessons 024, 025, 028). The `--explain E0308`
  trailer is also present, consistent with lesson 003.
- The `--> broken.rs:2:17` location points at column 17 of line 2 —
  the literal `5` itself, the actual mismatch site.
- The source excerpt traces the type flow within a single line:
  - The dashes `--` underline the type annotation `()`, with the
    sub-line `expected due to this` underneath. So rustc names
    where the expectation came from: the annotation.
  - The caret `^` underlines the literal `5`, with the trailing
    annotation `expected (), found integer`. So rustc names what
    the expected type was (`()`) and what was found instead
    (`integer`, rustc's umbrella term for an unconstrained integer
    literal).
- This is the *reverse direction* of lesson 028's diagnostic:
  - Lesson 028: `expected i32, found ()` — the binding wanted an
    `i32`, the loop produced `()`.
  - Lesson 029 (this one): `expected (), found integer` — the
    binding wants `()`, the literal `5` is an integer.
  Both confirm the same rule from two directions: `()` is its own
  type, distinct from any integer type, and rustc will not coerce
  between them.
- Exit code: 1. No executable was produced.

### Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs
  when the executable launches.
- `003-read-rustc-diagnostic` (accepted, load-bearing) —
  diagnostics have a headline + `-->` location + source excerpt
  with caret + optional sub-lines. The broken-contrast walk decodes
  E0308 with exactly this skill.
- `005-let-binding` (accepted, load-bearing) — `let name: TYPE =
  value;`. The three working-probe lines are exactly this shape
  with the type slot pinned to `()`.
- `008-define-and-call-function` (accepted) — `fn name() { ... }`
  and `name();`. The probe defines `say_hello()` and uses
  `say_hello()` as a value-producing expression of type `()`.
- `019-type-annotation-i32` (accepted, load-bearing) — `name: TYPE`
  attaches a type. The probe uses `: ()` as the type slot; the
  broken-contrast diagnostic's `expected due to this` sub-line
  points back at this annotation.
- `021-function-return-value` (accepted, calibration carry-forward)
  — a function with no `->` returns "nothing useful". This lesson
  names that "nothing useful" as `()`.
- `024-statement-vs-expression` (accepted, calibration
  carry-forward) — a block whose tail line ends in `;` evaluates
  to "nothing useful". This lesson names that as `()` and notes
  the empty block `{}` as the limit case.
- `025-implicit-return` (accepted, calibration carry-forward) —
  rustc's `implicitly returns ()` annotation was diagnostic
  shorthand for `()`.
- `026-if-as-expression` (accepted, calibration carry-forward) —
  `if`-without-`else` evaluates to `()`; deferred at the time.
- `027-loop-and-break` (accepted, calibration carry-forward) — a
  `loop { ...; break; }` produces `()` when `break;` carries no
  value.
- `028-break-value` (accepted, calibration carry-forward) —
  rustc's `expected i32, found ()` named `()` as what the loop
  produced. The broken-contrast in this lesson is the reverse
  direction of that same diagnostic.
