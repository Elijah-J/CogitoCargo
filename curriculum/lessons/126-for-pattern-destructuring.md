---
id: 126-for-pattern-destructuring
status: accepted
evidence: ../evidence/126-for-pattern-destructuring.md
---

# Destructure tuples in the for-loop binding slot with `for (a, b) in iter`

## The Move

Lesson 125 walked two vecs in lockstep with `for pair in
v.iter().zip(w.iter())` and read each yielded tuple's parts as
`pair.0` and `pair.1`. Today replaces the single name `pair` with
a *tuple pattern* `(a, b)` directly in the for-binding slot:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    for (a, b) in v.iter().zip(w.iter()) {
        println!("{} / {}", a, b);
    }
}
```

`rustc demo.rs` is silent (exit 0). `./demo` prints three lines:

```text
10 / 100
20 / 200
30 / 300
```

Output is byte-identical to lesson 125's probe (the appendix runs
both forms and `diff`s them). Only the binding shape differs. Each
pass the zipped iterator yields a 2-tuple `(&u64, &u64)`. Lesson
125 bound the whole tuple to one name and read parts with `pair.0`
and `pair.1`; today's pattern splits the tuple at the binding step
— `a` to the first part, `b` to the second. Inside the body, `a`
and `b` are usable directly. No `.0` / `.1`, no `pair`.

The Reference's for-expression grammar is `for PATTERN in EXPRESSION
{ ... }` (loop-expr.md:202), and the runtime rule is "that value is
matched against the irrefutable pattern, the body of the loop is
executed" (line 212). Lesson 073 installed the tuple pattern `(a,
b)` on the left of `let`; today reuses the same shape at the
for-binding slot. The Book's chapter 19 says it directly under "for
Loops": "in `for x in y`, the `x` is the pattern."

## Mental Model Delta

- *Before:* "Lesson 073 lets me write `let (a, b) = pair;` to split
  a tuple at the let-binding step. Lesson 079's `for X in
  COLLECTION` puts a single name in `X`. To read a 2-tuple from
  `.zip()` I bind the whole tuple to one name and use `pair.0` /
  `pair.1`."
- *After:* "The for-loop's binding slot is a *pattern*, just like
  the left of `let`. When the iterator yields tuples, I can put a
  tuple pattern there: `for (a, b) in iter` binds `a` and `b` to
  the parts of each yielded tuple. The whole-tuple name is gone;
  only the parts are bound. The pattern's shape must match the
  yielded value — wrong shape fires `error[E0308]: mismatched
  types`, same as lesson 073's contrast."

## Prerequisites

- Installed concepts:
  - **Lesson 073** (load-bearing): the tuple pattern `(a, b)` on
    the left of `let`. Today moves the same pattern shape to the
    for-loop binding slot.
  - **Lesson 079** (load-bearing): `for X in COLLECTION { ... }`,
    one body pass per element. Today extends `X` from a single
    identifier to a tuple pattern.
  - **Lesson 125** (load-bearing): `v.iter().zip(w.iter())` yields
    2-tuples `(&u64, &u64)`. Today's probe consumes its output.
  - **Lesson 072** (cited): tuple type and `.0` / `.1` indexing —
    the appendix equivalence probe shows old and new forms produce
    identical output.
  - **Lesson 123** (cited): `v.iter()` yields `&T`. Probe 3's
    contrast applies a tuple pattern to `&u64` and rustc rejects.
  - **Lessons 011, 001, 002, 003, 005** (cited): `println!`;
    `rustc demo.rs && ./demo` silent on success; `fn main`; the
    diagnostic four-part map; `let`.
  - **Lessons 080, 019, 107** (cited): `u64`; `: TYPE` annotation;
    `Vec<T>` with `vec![]`.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Make a fresh empty directory, `cd` into it, save `demo.rs` with
the source above, and compile and run:

```console
$ rustc demo.rs
$ ./demo
10 / 100
20 / 200
30 / 300
```

Now the contrast. After destructuring, the original tuple has *no
name* in the body. In `broken.rs` replace the loop with `for (a, b)
in v.iter().zip(w.iter()) { println!("{:?}", pair); }` and compile:

```text
error[E0425]: cannot find value `pair` in this scope
 --> broken.rs:5:26
  |
5 |         println!("{:?}", pair);
  |                          ^^^^ not found in this scope
```

Read with the lesson 003 map. The destructuring pattern `(a, b)`
introduces only `a` and `b`; it does not also bind the whole tuple
to a name. Lesson 125's `for pair in iter` bound `pair`; today's
`for (a, b) in iter` does not.

## What Changed

- The for-loop's binding slot is a *pattern*, not just a single
  identifier (Reference: `for PATTERN in EXPRESSION { ... }`).
- A tuple pattern `(a, b)` in that slot destructures each yielded
  tuple into named parts — lesson 073's shape, reused.
- The whole tuple has no name after destructuring; using `pair`
  fires `error[E0425]: cannot find value \`pair\``.
- The pattern shape must match the yielded value. A 2-tuple
  pattern against `&u64` fires `error[E0308]: mismatched types`
  ("expected `u64`, found `(_, _)`"). A 3-tuple pattern against a
  2-tuple iterator fires E0308 with the lesson-073 message
  "expected a tuple with 2 elements, found one with 3 elements".
- This makes rmp's `src/biguint/cmp.rs:22` readable end-to-end:
  `for (left, right) in self.limbs.iter().rev().zip(other.limbs.iter().rev())`
  is lesson 125's iterator chain in EXPRESSION and today's tuple
  pattern in PATTERN.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let xs: Vec<u64> = vec![7, 8, 9];
    let ys: Vec<u64> = vec![1, 2, 3];
    for (x, y) in xs.iter().zip(ys.iter()) {
        println!("{} + {} = {}", x, y, x + y);
    }
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile silently? What does it print?

(b) You replace line 4 with `for (x, y, z) in
xs.iter().zip(ys.iter()) {`. What error code fires, and what does
the inline annotation under the caret say?

*(Answers: (a) Yes; three lines: `7 + 1 = 8`, `8 + 2 = 10`, `9 + 3
= 12`. `x` and `y` are each `&u64`, added via std's `Add for &u64`
impl (named-deferred). (b) E0308; "expected a tuple with 2
elements, found one with 3 elements" — same shape as lesson 073's
count-mismatch contrast, at the for-binding slot.)*

## What To Ignore For Now

Today installs only the tuple pattern at the for-binding slot.
Real and deferred:

- *Refutable patterns* — `for Some(x) in iter` is rejected because
  `Some(x)` does not match `None`. The Reference says the for-loop
  matches against an *irrefutable* pattern (loop-expr.md:212).
  Word "irrefutable" name-deferred; operationally the for-binding
  pattern must be one rustc can match unconditionally.
- *Other pattern shapes in for-binding* — struct, enum, slice. Each
  is its own move.
- *Wildcard `_`* — `for (_, b) in iter`. Future move.
- *Nested tuple patterns* — `for ((a, b), c) in iter`. Future move.
- *`mut` inside the pattern* — `for (mut a, b) in iter`. Future.
- *Type annotation on the pattern* — `for (a, b): (&u64, &u64) in
  iter`. Allowed; not installed.
- *match / if-let / while-let* — other places patterns appear.
  Today is just the for-binding slot.
- *Auto-deref / `Add for &u64`* — Check Yourself uses `x + y` on
  `&u64`. Std machinery; named-deferred from 125.

## Evidence

See `../evidence/126-for-pattern-destructuring.md`.
