---
id: 075-const-declaration
status: accepted
evidence: ../evidence/075-const-declaration.md
---

# Declare a constant with `const NAME: TYPE = value;`

## The Move

Write a statement of the shape `const NAME: TYPE = value;`. The
keyword is `const`, not `let`. The `: TYPE` annotation is
*required* — there is no bare `const NAME = value;` form. The
right side must be a *constant expression* — an expression rustc
can evaluate at compile time using only literals, other constants,
and basic arithmetic. The convention is `SCREAMING_SNAKE_CASE` for
the name. A `const` may sit *inside* a `fn` body or *outside*
every `fn`, at the *global* scope of the source file.

## Mental Model Delta

- *Before:* "I bind names with `let name = value;` (lesson 005).
  The `: TYPE` slot is optional (lesson 019). To make a binding
  reassignable I add `mut` (lesson 006). Bindings live inside a
  `fn` body."
- *After:* "There is a second name-introduction form for values
  that never change: `const NAME: TYPE = value;`. Five
  differences from `let`, all from the same Book section: (1)
  the keyword is `const`, not `let`; (2) `mut` is rejected —
  constants are *always* immutable; (3) the type annotation is
  *required*, not optional; (4) the value must be a constant
  expression rustc can evaluate at compile time, not the result
  of a function call that runs at runtime; (5) a `const` may sit
  outside every `fn` at *global* scope, where a `let` cannot. By
  convention, constant names use `SCREAMING_SNAKE_CASE`."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs` then `./name`; the body of
    `fn main` runs when the executable launches.
  - Lesson 003 (load-bearing): the four-part diagnostic map. The
    contrast probe is read with that map.
  - Lesson 005 (load-bearing): `let name = value;` binds a name
    to a value. `const` is a sibling form with five differences.
  - Lesson 006 (load-bearing): `let mut name = ...;` makes a
    binding reassignable; without `mut`, reassignment is
    rejected. The "no `mut` on `const`" claim is grounded by a
    contrast probe that fires `error: const globals cannot be
    mutable`.
  - Lesson 019 (load-bearing): the `: TYPE` annotation slot.
    Today reuses it but makes it *required*, not optional.
  - Lesson 062 (load-bearing): `: u32` plugs into the same
    annotation slot. The probe uses `u32` because the Book's
    canonical example does.
  - Lesson 011 (cited): positional `{}` printing.
  - Lesson 068 (cited): *scope* as the region where a name has
    meaning. Today extends "the enclosing `{ ... }` block" to
    include the *global scope* — outside every `fn` body.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

In a fresh empty directory, save `demo.rs`:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    const MAX_POINTS: u32 = 100;
    println!("THREE_HOURS_IN_SECONDS = {}", THREE_HOURS_IN_SECONDS);
    println!("MAX_POINTS = {}", MAX_POINTS);
}
```

Two `const` lines at different nesting levels.
`THREE_HOURS_IN_SECONDS` sits *outside* `fn main` — global scope.
`MAX_POINTS` sits *inside* `fn main`. Both work. The right side
of the global one is the expression `60 * 60 * 3`; rustc
evaluates it at compile time to `10800` and burns that value
into the program.

Compile and run:

```console
$ rustc demo.rs
$ ./demo
THREE_HOURS_IN_SECONDS = 10800
MAX_POINTS = 100
```

Now the contrast. Save `broken.rs`:

```rust
fn main() {
    const X = 5;
    println!("X = {}", X);
}
```

The only change from a working `const X: i32 = 5;` is the
missing `: i32` annotation. Compile it. Read the headline with
the lesson 003 map; full transcript in `## Evidence`:

```text
error: missing type for `const` item
 --> broken.rs:2:12
  |
2 |     const X = 5;
  |            ^ help: provide a type for the constant: `: i32`
```

The headline is uncoded (`error:` with no `E####`). The caret
sits between the name and the `=`, exactly where the missing
annotation would go, and the inline `help:` literally suggests
the fix `: i32`. (Two more contrast probes — `const mut` and a
runtime function call on the right — are in the evidence
appendix; both fail to compile, grounding the "no `mut`" and
"constant expression required" claims.)

## What Changed

- A second name-introduction form: `const NAME: TYPE = value;`.
- Five differences from `let`, all from Book Ch3-1 *Declaring
  Constants*: keyword `const` not `let`; `mut` is rejected
  ("Constants aren't just immutable by default — they're always
  immutable"); `: TYPE` is *required* ("the type of the value
  *must* be annotated"); the right side must be a *constant
  expression* rustc evaluates at compile time, "not the result
  of a value that could only be computed at runtime"; a `const`
  may sit *in any scope, including the global scope*.
- Convention: SCREAMING_SNAKE_CASE — "all uppercase with
  underscores between words."
- The constant-expression slot accepts arithmetic between
  literals. A non-`const fn` call does not fit there: rustc
  fires `error[E0015]: cannot call non-const function`. The
  full set of allowed operations is the Reference's
  `const_eval.md` topic.

## Check Yourself

You write `tiny.rs`:

```rust
const SECONDS_PER_MINUTE: u32 = 60;

fn main() {
    const MINUTES_PER_HOUR: u32 = 60;
    let total: u32 = SECONDS_PER_MINUTE * MINUTES_PER_HOUR;
    println!("seconds per hour = {}", total);
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) Which name is in *global* scope?

(c) If you delete the `: u32` from line 4, does `rustc tiny.rs`
still accept the program?

(d) If you replace line 4 with `const mut MINUTES_PER_HOUR: u32 =
60;`, does it compile?

(Answers: (a) Yes; prints `seconds per hour = 3600`. (b)
`SECONDS_PER_MINUTE`. (c) No; uncoded `error: missing type for
\`const\` item` with caret between the name and `=` and inline
`help: provide a type for the constant: \`: i32\``. (d) No;
rustc fires `error: const globals cannot be mutable`.)

## What To Ignore For Now

Today installs only the `const NAME: TYPE = value;` form, the
SCREAMING_SNAKE_CASE convention, and the five-fact difference
list against `let`. Each of the following is real and deferred:

- *`static` items* — `static NAME: T = ...;`. Sibling form rustc
  surfaces in the `const mut` contrast probe ("you might want to
  declare a static instead"); separate mechanics.
- *`const fn`* — declaring a function evaluable at compile time
  so it *can* sit on the right of a `const`. The runtime-call
  contrast probe fires E0015 *because* the called function is
  not `const fn`.
- *Const generics* — `fn f<const N: usize>() { ... }`. Same
  keyword, different role.
- *The full set of operations allowed in constant expressions*.
  The Book defers this to the Reference's `const_eval.md`. Today
  only observes that integer arithmetic between literals works.
- *Compile-time function evaluation (CTFE)* as a typed concept,
  beyond the operational fact that rustc evaluates the right
  side at compile time.
- *Constant propagation* as a named optimization ("essentially
  inlined wherever they are used", Reference).
- *Why the missing-type `help:` suggests `: i32`*. Rustc inferred
  the literal `5` would be `i32` (lesson 019); the suggestion is
  a hint about the inferred type, not a default for `const`.
  The programmer must still write the annotation.
- *Integer overflow inside a constant expression*. Today's
  `60 * 60 * 3 = 10800` fits cleanly in `u32`.

## Evidence

See `../evidence/075-const-declaration.md`.
