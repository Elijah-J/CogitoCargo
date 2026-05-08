---
id: 048-mutable-reference-parameter
status: accepted
evidence: ../evidence/048-mutable-reference-parameter.md
---

# Declare a function parameter typed `&mut i32` and call it with `&mut n`

## The Move

Lesson 046 put `&i32` in a function parameter slot and called it with
`&n`. Lesson 047 introduced `&mut i32` plus the deref-assign form
`*r = 99;`. Today composes the two: the parameter slot accepts
`&mut i32`, the call passes `&mut n`, and the body uses `*r = 99;`.
After the call returns, the caller's binding shows the write.

```rust
fn set_to_99(r: &mut i32) {
    *r = 99;
}

fn main() {
    let mut n: i32 = 1;
    set_to_99(&mut n);
    println!("n = {n}");
}
```

`set_to_99` declares one parameter, `r`, of type `&mut i32` — the
lesson-020 shape with lesson 047's mutable reference type in the slot.
The call `set_to_99(&mut n)` builds a `&mut i32` from `n` (lesson
047's prefix `&mut`) and passes it. Inside the body, `*r = 99;`
writes through `r` into the place `r` refers to — namely `n`. The
final `println!` prints `n = 99`: the function mutated the caller's
binding.

Two preconditions line up. The source is `let mut n` (lesson 006):
`&mut` only applies to a `mut`-bound place. And the call passes
`&mut n`, not `&n`: the parameter type `&mut i32` is distinct from
`&i32`.

## Mental Model Delta

- Before: "I can declare `fn show(r: &i32)` and call it with `&n`
  (lesson 046). Separately, I can build a `&mut i32` and write
  through it with `*r = 99;` on the right of `let` (lesson 047)."
- After: "The parameter-type slot accepts `&mut T` exactly as it
  accepted `&T`. The call site supplies the argument with `&mut
  binding`, and the body's `*r = newval;` writes through into the
  caller's place. No new mechanism — pure composition of 046 and 047."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 003, 005, 008, 019: compile/run, `fn main`,
    diagnostic shape, `let`, `{n}` placeholder, defining a second
    function, `let name: i32 = value;`.
  - Lesson 006: `let mut name` makes `n` mutably-borrowable.
  - Lesson 020: `fn name(p: TYPE)`; argument type must match `TYPE`.
  - Lesson 046 (load-bearing): the parameter-type slot accepts a
    reference type; the call supplies it with a prefix-borrow. Today
    swaps `&` for `&mut`.
  - Lesson 047 (load-bearing): the type `&mut i32`, the prefix `&mut`
    operator, and the deref-assign form `*r = newval;` — all reused
    unchanged.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, Linux/macOS shell.

## Try It

In a fresh empty directory, create `demo.rs` with the source above,
then compile and run:

```console
$ rustc demo.rs
$ ./demo
n = 99
```

`n` started at `1`; after `set_to_99(&mut n)` returned, `println!`
read `n` and got `99`. The function mutated the caller's binding.

*Predict*: edit only the call site, replacing `&mut n` with `&n` (a
*shared* borrow into a *mutable*-reference parameter slot). Compile:

```
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
```

Same E-code family as lessons 045, 046, 047. The caret label `types
differ in mutability` matches lesson 047's broken probe; the second
`-->` (`note: function defined here`) matches lesson 046's. No `help:`
block — same as 047.

## What Changed

- The parameter-type slot accepts `&mut T` the same way it accepts
  `&T` (lesson 046) and bare `T` (lesson 020).
- The call site supplies a `&mut i32` argument with `&mut binding` —
  the same prefix `&mut` from lesson 047, now in an argument position
  instead of the right of `let`.
- Inside the body, `*r = newval;` writes through the parameter into
  the caller's place; the caller's binding reads the new value after
  the call returns.
- Passing `&n` to a `&mut i32` parameter fires E0308 with caret label
  `types differ in mutability` (lesson 047) plus a second `-->`
  reading `note: function defined here` (lesson 046).

## Check Yourself

You write `pred.rs`:

```rust
fn zero_out(r: &mut i32) {
    *r = 0;
}

fn main() {
    let mut count: i32 = 7;
    zero_out(&mut count);
    println!("count = {count}");
}
```

(a) Does rustc accept the program? (b) What single line does `./pred`
print? (c) If you replaced the call with `zero_out(&count);`, which
E-code fires and what caret label appears under `&count`?

(Answers: (a) Yes. (b) `count = 0`. (c) E0308 with caret label `types
differ in mutability`; the `= note:` block reads `expected mutable
reference &mut _` over `found reference &_`.)

## What To Ignore For Now

Today composes lesson 046 with lesson 047. Deferred:

- *`*r` as a read-through* — the body uses ONLY hardcoded write `*r =
  99;`. Using `*r` on the *right* of `=` (e.g. `*r = *r + 1;`) is
  still a future move, carrying over from 047.
- *The borrow checker proper* — multiple `&mut` (E0499), mixing `&`
  and `&mut` (E0502), etc. Today's program holds one reference at a
  time, so the checker does not visibly bite.
- *E0596* (`&mut` on a non-`mut` binding) — `let mut n` side-steps.
- *E0594* (`*r = v` through `&T`) — the broken probe stops at E0308.
- *Lifetimes* (`'a`, elision). Heavy deferral.
- *Methods that take `&mut self`* and *autoref*. Today uses a free
  function with an explicit parameter.
- *Reborrowing* (`&mut *r`). Future move.
- *Multiple parameters mixing `&T` and `&mut T`*. Today uses one.
- *Reference return types* (`fn f() -> &mut T`). Future move.
- *Why `n` is still usable after the call returns.* The empirical
  fact is in the print line; the structural reason (ownership, `Copy`)
  is deferred.
- All previously deferred items.

## Evidence

See `../evidence/048-mutable-reference-parameter.md` for the corpus-
quote map, the rustc / system toolchain string, the working probe
transcript, the broken-contrast E0308 transcript, and the
prerequisite-claim summary.
