---
id: 058-match-result-payload-variants
status: accepted
evidence: ../evidence/058-match-result-payload-variants.md
---

# `match` a `Result` against payload-variant patterns `Ok(num)` and `Err(_)`

## The Move

Cycle 051 matched bare variant names (`Ordering::Less`). Cycle 052
introduced variants that *carry a payload* (`Ok(T)`, `Err(E)`) but
inspected them only with `.is_ok()`. Today joins the two: a `match`
arm's pattern can be `Variant(subpattern)` — the variant's name, then
a parenthesized *subpattern* lined up with the payload:

```rust
fn main() {
    let good: i32 = match "42".parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    let bad: i32 = match "abc".parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    println!("good = {good}, bad = {bad}");
}
```

`rustc demo.rs` exits 0; `./demo` prints `good = 42, bad = -1`. Two
new arm shapes:

- `Ok(num)` — variant `Ok`, subpattern `num`. The bare name `num` is a
  *binding pattern*: when the arm matches, the payload binds to the
  local name `num`, usable in the arm's body. `Ok(num) => num` means
  "if it's `Ok`, hand the payload out as the arm's value."
- `Err(_)` — variant `Err`, subpattern `_`. Cycle 031's wildcard,
  reused inside the constructor: matches any payload, binds nothing.
  `Err(_) => -1` means "if it's `Err`, ignore the payload and use
  `-1`."

Everything else is unchanged from cycles 030/031/051: matching arm's
expression is the whole match's value, all arms share a type,
exhaustiveness is enforced by **E0004**. With both `Ok(...)` and
`Err(...)` covered the match is exhaustive — `Result` has only two
variants.

## Mental Model Delta

- *Before:* "I can `match` an enum against bare variant names
  (cycle 051), and `Result<T, E>` carries payloads (cycle 052) — but
  the only inspection I had was `.is_ok()` returning a `bool`. I
  couldn't get the payload out without panicking."
- *After:* "A payload-bearing variant `Variant(T)` becomes the pattern
  `Variant(subpattern)` in a `match` arm. The subpattern is itself a
  pattern: a binding name like `num` captures the payload into a local
  for the arm's body; a wildcard `_` matches and discards it. With
  both `Ok(...)` and `Err(...)` covered the match is exhaustive. This
  is how Rust opens a `Result` without panicking."

## Prerequisites

- *Installed concepts:*
  - Lesson 052 (load-bearing): `Result<T, E>` with `Ok(T)` / `Err(E)`.
    Today is the first cycle that *opens* a payload variant.
  - Lesson 051 (load-bearing): `match` against enum variants and E0004
    naming the missing variant. Today extends "bare variant name" to
    `Variant(subpattern)`.
  - Lesson 031 (load-bearing): the `_` wildcard pattern. Reused today
    *inside* a variant constructor.
  - Lesson 030: the `match` machine — arms `pattern => arm_expression,`,
    all arms share a type, matching arm's value is the match's value.
  - Lesson 056 (load-bearing): `.parse()` returns `Result<TARGET, _>`;
    the `: i32` annotation flows back to pin the target type.
  - Lessons 055 (`&str` is the type of string literals like `"42"`),
    019 (`let name: TYPE = value;`), 040 + 049 (dot-form method call,
    receiver is any expression).
  - Lessons 001, 002, 005, 011: compile and run, `fn main`, `let`,
    `{name}` placeholder.
- *Ordinary computer-use assumptions:* same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`).

## Try It

Save the snippet from *The Move* as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
good = 42, bad = -1
```

First `match`: `"42".parse()` produces `Ok(42_i32)` (cycle 056's
inference: the `: i32` binding pins the target). `Ok(num)` matches;
the payload `42` binds to `num`; the arm's body is the bare expression
`num`, evaluating to `42`. `good` becomes `42`.

Second `match`: `"abc".parse()` produces an `Err(...)` (the payload
is some parse-error value — its shape is not part of this lesson).
`Ok(num)` doesn't match. `Err(_)` does: the variant matches and `_`
matches the payload without binding. The arm's value is `-1`, so
`bad` becomes `-1`. Where cycle 056's `.expect(...)` *panicked* on
`Err`, today's `match` *avoids* the panic by handling `Err`
explicitly.

*Predict (broken contrast).* Save `broken.rs` with the `Err(_)` arm
removed:

```rust
fn main() {
    let n: i32 = match "42".parse() {
        Ok(num) => num,
    };
    println!("n = {n}");
}
```

`rustc broken.rs` refuses with
`error[E0004]: non-exhaustive patterns: \`Err(_)\` not covered`. Same
E-code as cycles 030/031/051 — but the missing pattern is named in
the *new* shape: `Err(_)` is a variant constructor with a wildcard
subpattern, i.e. "`Err` with any payload." Adding `Err(_) => -1,`
fixes it.

## What Changed

- A `match` arm's pattern can be `Variant(subpattern)` — variant's
  name, then a parenthesized subpattern lined up with the payload.
  The subpattern is itself a pattern: a binding name like `num`
  *captures* the payload into a local; a wildcard `_` *discards* it.
- `Ok(num) => num` is the canonical "extract the payload" arm.
  Pairing it with `Err(_) => default` produces either the unwrapped
  `Ok` payload or the default — without panicking.
- Exhaustiveness still applies (E0004). For `Result`, covering both
  `Ok(...)` and `Err(...)` is enough.
- The cycle-031 wildcard `_` now also works *inside* a variant
  constructor — same wildcard, one level deeper.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let v: i32 = match "10".parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let next: i32 = v + 1;
    println!("v = {v}, next = {next}");
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) Change `"10"` to `"oops"` and recompile. What does it print, and
what is the exit code? Does the program panic?

(c) Without recompiling, predict: if you swapped the arm bodies
(`Ok(num) => 0, Err(_) => num,`), would rustc still accept the
program?

*(Answers: (a) Yes; prints `v = 10, next = 11`. `Ok(num)` binds
`num = 10`; the arm value is `num`, so `v` is `10`. (b) Prints
`v = 0, next = 1`, exit 0. `"oops".parse()` is `Err(...)`, `Err(_)`
fires, arm's `0` wins. *No panic* — `match` handles `Err` instead
of unwrapping. (c) No. The swapped `Err(_) => num,` arm references
`num`, but `num` is bound only by the `Ok(num)` pattern — each arm
sees only its own bindings. rustc refuses with "cannot find value
`num`".)*

## What To Ignore For Now

- *`continue` in arm body* — the Book's guessing-game uses
  `Err(_) => continue` to retry the loop. Today uses a value
  (`-1`), not control flow. Composes once `loop` and `match` are
  joined.
- *Match guards* (`Ok(n) if n > 0 => ...`), *or-patterns*
  (`Ok(0) | Ok(1) => ...`), *range patterns inside variants*
  (`Ok(0..=10) => ...`), *nested payload patterns* (`Ok(Some(x))`,
  `Ok((a, b))`), *`@`-bindings*, *`mut` / `ref` bindings*. All real
  Rust pattern shapes; all deferred.
- *`if let` and `while let`* — abbreviated single-arm matches on a
  variant. Direct successor.
- *The `Option<T>` enum* (`Some(T)` / `None`). Same payload-variant
  shape; deferred.
- *Defining your own payload-bearing enum.* Today uses
  standard-library `Result`.
- *Arm ordering.* Patterns are checked in source order (cycle 031);
  for non-overlapping `Ok(...)` / `Err(...)` reordering does not
  change which arm matches. Full overlap / unreachable story
  deferred.
- *Pattern-binding move/copy semantics.* For `i32` (Copy) the
  question is invisible; for owned types ownership rules apply.
  Deferred.
- *The exact `Err` payload type.* `"abc".parse()` returns `Err` of
  some parse-error value whose name is not installed; the wildcard
  `_` is what lets the lesson handle `Err` without naming it.
- All previously deferred items.

## Evidence

See `../evidence/058-match-result-payload-variants.md`.
