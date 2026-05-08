---
id: 046-reference-function-parameter
status: accepted
evidence: ../evidence/046-reference-function-parameter.md
---

# Declare a function parameter typed `&i32` and call it with `&n`

## The Move

Lesson 020 fixed the shape of a function parameter as `name: TYPE`,
with `i32` as the example `TYPE`. Lesson 045 introduced a new type
form, `&i32`. The `TYPE` slot in a parameter list accepts `&i32`, and
the call site supplies a value of that type by writing `&value`.

```rust
fn show(r: &i32) {
    println!("via reference param: {r}");
}

fn main() {
    let n: i32 = 42;
    show(&n);
    println!("n is still: {n}");
}
```

`show` declares one parameter, `r`, of type `&i32` — the lesson-020
shape with a new type in the slot. The call `show(&n)` builds a value
of type `&i32` from `n` (lesson 045's prefix-`&`) and passes it as
the argument. Inside the body, `{r}` formats the underlying value
(lesson 045's "`{}` looks through `&T`"). The program prints `via
reference param: 42` and then `n is still: 42`.

The second line is the empirical content: after `show(&n)` returns,
the binding `n` is still usable in `main`. The call did not consume
it.

## Mental Model Delta

- Before: "Function parameters take values by name and type, like
  `n: i32` (lesson 020). I can build a `&i32` value with `&n` (lesson
  045), but I have only used it on the right of `let`."
- After: "The parameter-type slot accepts any type — including a
  reference type. `fn show(r: &i32)` declares one parameter of type
  `&i32`; the call site supplies a value of that type, typically by
  writing `&binding`. The caller's binding is *still usable* after the
  call returns: a function with a `&T` parameter reads through its
  argument rather than taking it over."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 003, 005, 019: compile/run shape, `fn main`,
    diagnostic shape, `let name: i32 = value;`.
  - Lesson 008: defining a second function and calling it from `main`.
  - Lesson 020 (load-bearing): `fn name(p: TYPE) { ... }` and the rule
    that the parameter-type slot must match the argument's type at the
    call site. Today: `TYPE = &i32`.
  - Lesson 045 (load-bearing): `&T` is a type distinct from `T`; the
    prefix-`&` operator builds a `&T` value from a `T` value; `{}`
    formats `&T` and `T` identically.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, Linux/macOS shell (same as lesson 001).

## Try It

In a fresh empty directory, create `demo.rs`:

```rust
fn show(r: &i32) {
    println!("via reference param: {r}");
}

fn main() {
    let n: i32 = 42;
    show(&n);
    println!("n is still: {n}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
via reference param: 42
n is still: 42
```

*Predict*: edit only the call site, removing the `&`, so line 7
reads `show(n);` (and drop the last `println!` to keep the file
small). What does rustc say?

```rust
fn show(r: &i32) {
    println!("via reference param: {r}");
}

fn main() {
    let n: i32 = 42;
    show(n);
}
```

Compile:

```
error[E0308]: mismatched types
 --> broken.rs:7:10
  |
7 |     show(n);
  |     ---- ^ expected `&i32`, found `i32`
  |     |
  |     arguments to this function are incorrect
  |
note: function defined here
 --> broken.rs:1:4
  |
1 | fn show(r: &i32) {
  |    ^^^^ -------
help: consider borrowing here
  |
7 |     show(&n);
  |          +
```

Same E-code as lesson 045's broken contrast — *mismatched types* —
caught at the *call site* this time. The signature still declares
`&i32`; the call passes `i32`. rustc shows two `-->` locations (the
dual-`-->` pattern lesson 036 first observed): the call site on line 7
and the function definition on line 1. The `help:` block names the
exact fix: insert `&` to make `&n`.

(Full transcripts are in `../evidence/046-reference-function-parameter.md`.)

## What Changed

- A function parameter slot can be typed as a reference: `fn show(r:
  &i32) { ... }` is the lesson-020 shape with `&i32` in the type slot.
- At the call site, you supply a `&i32` value with `&binding`, just as
  lesson 045 built one for the right-hand side of `let`. Same operator,
  same type, new context.
- Forgetting the `&` at the call site fires E0308 *mismatched types*
  with `expected `&i32`, found `i32`` and `help: consider borrowing
  here` suggesting the `&`. The diagnostic names the function
  definition via a second `-->` location (lesson 036's dual-`-->`
  pattern).
- After `show(&n);`, the binding `n` is still usable in `main`. A
  function with a `&T` parameter *reads through* its argument rather
  than taking it over.

## Check Yourself

You write `pred.rs`:

```rust
fn report(r: &i32) {
    println!("r = {r}");
}

fn main() {
    let count: i32 = 7;
    report(&count);
    report(&count);
}
```

(a) Does rustc accept the program? (b) What two lines does `./pred`
print? (c) If you replaced line 7 with `report(count);`, which E-code
would the headline carry, and what would `help:` suggest?

(Answers: (a) Yes — same shape, different binding name, called twice.
(b) `r = 7` then `r = 7`. (c) E0308 *mismatched types* with `expected
`&i32`, found `i32``; `help:` reads "consider borrowing here" and
suggests inserting `&` to make `&count`.)

## What To Ignore For Now

This lesson installs only one idea: the parameter-type slot accepts a
reference type, and the call site supplies a `&value` argument.
Deferred:

- *Why `n` is still usable after `show(&n)` returns.* The empirical
  fact is what the working probe shows. The structural reason involves
  *ownership* and the fact that `i32` is *cheap-to-duplicate* — both
  deferred. If `n` were a non-`Copy` type like `String`, the rules
  around what stays usable after a call get more interesting. Future
  move.
- *The `*r` dereference operator.* `{r}` looks through `&T`
  automatically, so today's program never writes `*r`.
- *`&mut T` mutable references and `&mut`-typed parameters.* Natural
  next move.
- *The borrow checker* — multiple references, aliasing rules
  (E0382, E0502, E0505, E0596). Today's program has only one
  reference at a time, so the checker does not visibly bite.
- *Lifetimes* (`'a`, elision). Heavy deferral.
- *Method receiver references* (`&self`, `&mut self`) and the
  *autoref* rule that lets `s.len()` work without writing `&s`.
- *Reference return types* — `fn f() -> &T`.
- *Mixing reference and non-reference parameters in one function.*
  Covered by lesson 036's multi-parameter rule; today's lesson uses
  one parameter.
- All previously deferred items.

## Evidence

See `../evidence/046-reference-function-parameter.md` for the
corpus-quote map, the rustc / system toolchain string, the working
probe transcript, the broken-contrast E0308 transcript, and the
prerequisite-claim summary.
