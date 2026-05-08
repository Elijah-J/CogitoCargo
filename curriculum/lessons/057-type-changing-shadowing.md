---
id: 057-type-changing-shadowing
status: accepted
evidence: ../evidence/057-type-changing-shadowing.md
---

# Shadow a binding *with a different type* using a second `let`

## The Move

Cycle 007 installed shadowing: a second `let name = ...;` does not
reassign the old binding — it creates a fresh binding under the same
name. Cycle 007's example used the same kind of value (an integer) on
both sides. Today's move drops that constraint:

```rust
fn main() {
    let n = "42";
    let n: i32 = n.parse().expect("not a number");
    let doubled: i32 = n * 2;
    println!("n = {n}, doubled = {doubled}");
}
```

`rustc demo.rs` exits 0 silently; `./demo` prints `n = 42, doubled = 84`.

The first `let n = "42";` binds `n` to a `&str` (cycle 055 — string
literals are `&str`). The second `let n: i32 = ...;` shadows it with a
fresh binding whose type is `i32`. The right-hand side computes the new
value *from the old binding*: `n.parse()` runs while `n` still refers
to the old `&str`, producing `Ok(42_i32)` (cycle 056 — `.parse()` plus
the `: i32` annotation pin down the target type), then
`.expect("not a number")` extracts the integer (cycle 053). The third
line uses cycle 009's integer multiplication `*` on the new `n`, which
is only valid because that `n` is now an `i32`.

## Mental Model Delta

- *Before:* "Cycle 007 said a second `let name = ...;` shadows the old
  binding. The example reused the same kind of value on both sides
  (both integers)."
- *After:* "The new binding's *type* can differ from the old one's.
  `let n = "42";` then `let n: i32 = n.parse().expect("...");` is a
  shadow whose first `n` is a `&str` and whose second `n` is an `i32`.
  The Book ch03-01 lines 211-213 say this directly: 'because we're
  effectively creating a new variable when we use the `let` keyword
  again, we can change the type of the value but reuse the same name.'
  The shadowing rule from cycle 007 is unchanged — what changed is what
  I now know I'm allowed to put on the right-hand side."

## Prerequisites

- *Installed concepts:*
  - Lesson 007 (load-bearing): a second `let name = ...;` shadows the
    old binding rather than reassigning it. Today extends only one
    degree of freedom: the new binding's type no longer has to match.
  - Lesson 056 (load-bearing): `&str` has a method `.parse()` that
    returns `Result<TARGET, ...>` whose target type is selected by
    inference from a `: TYPE` annotation on the receiving `let`;
    `.parse().expect("msg")` yields the parsed value or panics on
    `Err`.
  - Lesson 055 (load-bearing): string literals like `"42"` are `&str`.
  - Lesson 019 (load-bearing): `let name: TYPE = value;` is the
    type-annotation form, with `i32` as the example typed name.
  - Lesson 009 (load-bearing): `*` is integer multiplication on `i32`,
    so `n * 2` succeeding is empirical proof the new `n` is an `i32`.
  - Lessons 040, 049, 052, 053: dot-form, method chaining,
    `Result<T, E>`, and `.expect`.
  - Lessons 001, 002, 005, 011: compile and run, `fn main`, `let`,
    and the `{name}` placeholder in `println!`.
- *Ordinary computer-use assumptions:* same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`).

## Try It

Save the snippet from *The Move* as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
n = 42, doubled = 84
```

Two things worth noticing about line 3:

- The right-hand side `n.parse()` runs *first*, while `n` still refers
  to the old `&str` binding. The shadow only takes effect once the
  whole `let n: i32 = ...;` statement has run, so the call on the
  right is allowed to reach the previous `n`.
- The annotation `: i32` does double duty: it declares the new
  binding's type *and* pins down `.parse()`'s target type via
  cycle 056's inference. Once line 3 finishes, every later mention
  of `n` sees the new `i32` binding — which is what makes line 4's
  `n * 2` legal (cycle 009's `*` works on `i32`).

## What Changed

- A second `let name: NEWTYPE = ...;` may bind a value of a *different
  type* than the old `name`. Cycle 007's shadowing mechanic is
  unchanged; the new degree of freedom is on the value side.
- The right-hand side of the shadow can use the *old* binding to
  compute the new one — the shadow only takes effect once the whole
  `let` has run.
- Writing the new type out (`: i32`) is what makes the type change
  legible. It also drives `.parse()`'s target type via cycle 056.
- One sentence: *the type can change across a shadow, but the
  shadowing mechanic — second `let`, no `mut` — is the same as
  cycle 007.*

## Check Yourself

You write `convert.rs` containing:

```rust
fn main() {
    let s = "7";
    let s: i32 = s.parse().expect("not a number");
    let next: i32 = s + 1;
    println!("s = {s}, next = {next}");
}
```

(a) You run `rustc convert.rs` and then `./convert`. Does it compile?
What does it print, and what is the exit code?

(b) You then change line 4 to `let next = s.trim();` and rebuild.
Does it still compile? Briefly say *why or why not*, in terms of which
binding `s` refers to on line 4 and what type that binding has.

*(Answers: (a) it compiles; `./convert` prints `s = 7, next = 8`,
exit 0. The first `s` is a `&str`; the second `let s: i32 = ...`
shadows it with an `i32`; `s + 1` is integer addition (cycle 009).
(b) it does not compile. By the shadowing rule, line 4's `s` refers
to the *new* binding, which has type `i32`. `.trim()` is a method on
`&str` (cycle 055), not on `i32`, so rustc refuses with a "no method
named `trim` found for type `i32`" diagnostic. The fix is to do the
trimming *before* the shadow, while `s` is still `&str`.)*

## What To Ignore For Now

This lesson installs only one new fact: the new binding's type may
differ from the old's. Deferred:

- *Shadowing in nested scopes.* The Book ch03-01 example also shadows
  inside `{ ... }` braces; requires installing *scope*.
- *The lifetime of the shadowed binding.* For `&str` (a borrow) the
  question is moot; for owned types like `String` it can matter.
- *Borrow-checker interactions with shadowing* — shadows that move,
  borrow, or alias the old binding have rules. Heavy deferral.
- *Pattern shadowing* — `if let Some(x) = ...` and `match` arm
  bindings also create new bindings.
- *E0599* — "no method named X found for type Y", the diagnostic the
  *Check Yourself* (b) answer describes. New E-code; captured in the
  evidence appendix only.
- *`u32` and other typed names.* The Book's actual guessing-game
  shadow is `let guess: u32 = guess.trim().parse().expect(...);`;
  same mechanic, `u32` is just not yet an installed typed name.
- *Chaining `.trim().parse()` end-to-end* — combines today's shadow
  with cycle 055's `.trim()`. The natural next cycle.
- *Why type-changing shadowing exists* — design rationale beyond the
  Book's "spares us from having to come up with different names."
  Today observes the empirical fact rather than arguing for it.

## Evidence

See `../evidence/057-type-changing-shadowing.md`.
