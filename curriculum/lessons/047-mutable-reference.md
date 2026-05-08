---
id: 047-mutable-reference
status: accepted
evidence: ../evidence/047-mutable-reference.md
---

# Take a mutable reference with `&mut`, then write through it with `*r = value;`

## The Move

Lesson 045's `&T` lets you *refer* to a value. The sibling type `&mut
T`, built with the prefix operator `&mut`, also lets you *write* to it,
by writing `*r = newval;` on the left of an assignment:

```rust
fn main() {
    let mut n: i32 = 1;
    let r: &mut i32 = &mut n;
    *r = 99;
    println!("n = {n}");
}
```

Read line 3 right-to-left. `&mut n` is the new expression: the prefix
`&mut` applied to the binding `n` produces a value of type `&mut i32`
referring to `n`. The annotation `: &mut i32` names that type.

Line 4 is the second new piece: `*r = 99;`. The prefix `*` on the
*left* of `=` is the *deref-assign* form. It writes the right-hand
value *through* `r` into the place `r` refers to — namely `n`. The
final `println!` reads `n` and prints `n = 99`.

Two prerequisites had to line up. The source is `let mut n` (lesson
006): only a `mut`-bound place can be mutably borrowed. And the
reference is typed `&mut i32`, distinct from lesson 045's `&i32`.

## Mental Model Delta

- Before: "Lesson 045's `&T` lets me refer to a value without copying.
  I have no way to modify the value through it."
- After: "There is a sibling reference type, `&mut T`, built by the
  prefix `&mut`. It carries *write-access*. The deref-assign form `*r
  = newval;` exercises that access — it stores `newval` into the
  place `r` refers to. `&T` and `&mut T` are distinct types (E0308 if
  you mix them); and `&mut` only applies to a `let mut`-bound source."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 003, 005: compile/run, `fn main`, diagnostic
    shape, `let` plus the `{n}` placeholder.
  - Lesson 006 (load-bearing): `let mut name` makes a binding
    reassignable. Today extends to "reassignable through a `&mut`
    borrow."
  - Lesson 019 (load-bearing): `let name: TYPE = value;`. Today fills
    `TYPE` with `&mut i32`.
  - Lesson 045 (load-bearing): `&T` is built by prefix `&` and is
    distinct from `T`. Today's `&mut T` is the parallel mutable form;
    the broken probe fires the same E0308 family.
- Ordinary computer-use assumptions: terminal, editor, `rustc` on
  `PATH`, Linux/macOS shell.

## Try It

In a fresh empty directory, create `demo.rs` with the source above.
Compile and run:

```console
$ rustc demo.rs
$ ./demo
n = 99
```

Empirical content: `n` started at `1`; after `*r = 99;` ran, the final
`println!` prints `99`. The write went through `r` to `n`.

*Predict*: edit only line 3, dropping the `mut` from the right-hand
side so it reads `let r: &mut i32 = &n;` (a *shared* borrow into a
*mutable*-reference slot). Compile:

```
error[E0308]: mismatched types
 --> broken.rs:3:23
  |
3 |     let r: &mut i32 = &n;
  |            --------   ^^ types differ in mutability
  |            |
  |            expected due to this
  |
  = note: expected mutable reference `&mut _`
                     found reference `&_`
```

Same E-code as lesson 045's broken contrast — *mismatched types* —
with a new caret label `types differ in mutability`. The annotation
`&mut i32` set the expected type; `&n` delivered a `&i32`. The `note:`
block spells the mismatch — `expected mutable reference &mut _` vs
`found reference &_`. Probe-level evidence that `&T` and `&mut T` are
distinct types.

## What Changed

- The prefix `&mut` produces a *mutable reference*. For a `let mut`-
  bound `n: T`, `&mut n` is a value of type `&mut T`.
- `&mut T` is a new type form parallel to `&T` (lesson 045). The
  lesson-019 annotation slot accepts it: `let r: &mut i32 = ...;`.
- `*r = newval;` writes through `r` into the place `r` refers to. The
  prefix `*` on the *left* of `=` is what exercises a mutable
  reference's write-access.
- `&T` and `&mut T` are distinct types. Putting a `&T` in a `&mut T`
  slot fires E0308 with caret label `types differ in mutability`.

## Check Yourself

You write `pred.rs`:

```rust
fn main() {
    let mut count: i32 = 7;
    let r: &mut i32 = &mut count;
    *r = 0;
    println!("count = {count}");
}
```

(a) Does rustc accept the program? (b) What single line does `./pred`
print? (c) If you replaced line 3 with `let r: &mut i32 = &count;`,
which E-code fires and what `note:` block appears?

(Answers: (a) Yes. (b) `count = 0`. (c) E0308 with caret label `types
differ in mutability`; `note:` reads `expected mutable reference &mut
_` / `found reference &_`.)

## What To Ignore For Now

Today installs only two pieces: the type `&mut T` and the deref-assign
form `*r = newval;`. Deferred:

- *Borrow-checker restrictions on `&mut T`* — at most one `&mut T`,
  no mixing `&T` and `&mut T` to the same value. Today's program has
  one reference at a time, so it does not bite. E0382/E0499/E0502/
  E0505 are future moves.
- *E0596* (`&mut x` on a non-`mut` binding) — `let mut n` side-steps.
- *E0594* (`*r = v` through `&T`) — today's broken probe stops earlier
  at E0308.
- *Lifetimes* (`'a`, elision). Heavy deferral.
- *`*r` as a read-through* on the *right* of `=` (e.g. `let v: i32 =
  *r;`). Today installs only the write-through use.
- *`&mut T` parameters* (`fn change(r: &mut i32)`). Natural next move.
- *Methods with `&mut self`*, *reborrowing* (`&mut *r`), the *`Copy`
  trait*, *`DerefMut`*, and *raw borrows `&raw mut`*. Out of scope.
- All previously deferred items.

## Evidence

See `../evidence/047-mutable-reference.md` for the corpus-quote map,
the rustc / system toolchain string, the working probe transcript, the
broken-contrast E0308 transcript, and the prerequisite-claim summary.
