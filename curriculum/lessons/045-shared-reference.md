---
id: 045-shared-reference
status: accepted
evidence: ../evidence/045-shared-reference.md
---

# Take a shared reference to a value with `&value`

## The Move

So far every binding has held a value directly. There is a second
shape: a binding can hold a *reference* to a value somewhere else. The
prefix operator `&` produces one:

```rust
let n: i32 = 42;
let r: &i32 = &n;
```

Read the second line right-to-left. `&n` is the new expression: the
prefix `&` applied to the binding `n` produces a *shared reference*
that *refers to* `n`'s value without copying it and without taking it
over. The annotation `: &i32` names the type of that reference — a
shared reference to an `i32`. So `r` is a value of type `&i32`,
distinct from the `i32` in `n`.

A whole program with both bindings:

```rust
fn main() {
    let n: i32 = 42;
    let r: &i32 = &n;
    println!("n = {n}, r = {r}");
}
```

prints `n = 42, r = 42`. The two formatted slots look identical, and
that is the empirical content: the `{}` placeholder *looks through* a
shared reference and formats the underlying value.

## Mental Model Delta

- Before: "A binding holds a value directly. The annotation `: i32`
  says the value is an `i32`."
- After: "A binding can also hold a *shared reference* to a value
  somewhere else. The prefix `&` builds one: given a value of type `T`,
  the expression `&value` produces a value of type `&T` referring to
  it. `T` and `&T` are **different types** — putting a `T` value in a
  `&T` slot is a type error (E0308). The reference is itself a value,
  bindable by `let`, carrying its own type annotation. The `{}`
  placeholder looks through `&T` and formats the underlying value."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 005: `rustc file.rs` then `./name`, silent on
    success; `fn main` is the entry point; named-placeholder `{name}`.
  - Lesson 003: rustc diagnostics have headline + `-->` + source
    excerpt + caret + optional `help:`.
  - Lesson 019 (load-bearing): `let name: TYPE = value;` with `i32` as
    the example `TYPE`. Today extends the slot to a new type, `&i32`.
  - Lesson 026 (load-bearing for E0308 family): the broken probe fires
    E0308 *mismatched types*, same E-code lessons 024/025/026/028/033
    installed.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, Linux/macOS shell (same as lesson 001).

## Try It

In a fresh empty directory, create `demo.rs`:

```rust
fn main() {
    let n: i32 = 42;
    let r: &i32 = &n;
    println!("n = {n}, r = {r}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
n = 42, r = 42
```

Walk the new line. `&n` on the right is the prefix `&` applied to the
existing binding `n`. The annotation `: &i32` declares the type of `r`
to be a shared reference to `i32`. After this, two bindings exist: `n`
of type `i32` holding `42`, and `r` of type `&i32` holding a reference
to the `n` binding's `i32`. The output shows the same `42` twice.

*Predict*: what if you drop the `&` and write `let r: &i32 = n;`?
Edit line 3 and recompile. rustc emits:

```
error[E0308]: mismatched types
 --> broken.rs:3:19
  |
3 |     let r: &i32 = n;
  |            ----   ^ expected `&i32`, found `i32`
  |            |
  |            expected due to this
  |
help: consider borrowing here
  |
3 |     let r: &i32 = &n;
  |                   +
```

Same E-code as lessons 024, 025, 026, 028, and 033 — *mismatched types*.
The annotation `&i32` set the expected type; the right-hand side `n`
delivered an `i32`. Without the `&`, those are not the same type. The
`help:` block names the fix directly: a source-diff inserting `&` to
make `&n`.

(Full transcripts are in `../evidence/045-shared-reference.md`.)

## What Changed

- You can take a shared reference to a value with the prefix operator
  `&`. `&n` produces a value of type `&i32` referring to `n`'s `i32`
  without copying or owning it.
- You know one new type form, `&T` (the *shared reference type*),
  built from any type `T`. Today's example is `&i32`. The
  type-annotation slot from lesson 019 accepts it: `let r: &i32 = ...;`.
- `T` and `&T` are *distinct types*. Putting a `T` value in a `&T`
  slot fires E0308 *mismatched types* — same E-code as lessons 024,
  025, 026, 028, 033 — with `help: consider borrowing here` suggesting
  `&value`.
- `{}` in `println!` looks through a shared reference, so a `&i32`
  and the `i32` it refers to format identically.

## Check Yourself

You write `pred.rs`:

```rust
fn main() {
    let count: i32 = 7;
    let r: &i32 = &count;
    println!("count = {count}, r = {r}");
}
```

(a) Does rustc accept the program? (b) What single line does `./pred`
print? (c) If you replaced line 3 with `let r: &i32 = count;`,
which E-code would the headline carry, and what would `help:` suggest?

(Answers: (a) Yes — same shape, different binding name. (b)
`count = 7, r = 7`. (c) E0308 *mismatched types* ("expected `&i32`,
found `i32`"); `help:` reads "consider borrowing here" with a
source-diff inserting `&` to make `&count`.)

## What To Ignore For Now

This lesson installs only one idea: the prefix `&` produces a shared
reference of type `&T` from a value of type `T`. Deferred:

- *The `*` dereference operator* — `*r` reads through a reference.
  Today's lesson side-steps it because `{}` looks through `&T`.
- *`&mut T` mutable references* — the parallel form built with `&mut`.
  Natural next move.
- *Ownership and move semantics* — what happens to `n` after `&n`.
  Today's `i32` is small and cheaply duplicable, so the question does
  not visibly bite. Future move.
- *The borrow checker* — additional rules about how references can
  coexist (E0382, E0502, E0505, E0596). Future moves.
- *Lifetimes* (`'a`, `'static`, elision) — every reference also has a
  *lifetime*, elided here. Heavy deferral.
- *References as function parameters* — `fn f(r: &i32) { ... }` and
  call-site `f(&value)`. Today's lesson uses references only in `let`.
- *Multiple `&T` references to the same value* — permitted, future move.
- *Reference comparison and equality* — its own surface.
- *Smart pointers* (`Box<T>`, `Rc<T>`) and the `Deref` trait — out of
  scope.
- *The string-slice type `&str`* — string literals have a
  reference-typed type, but unpacking the connection is a separate
  move.
- All previously deferred items.

## Evidence

See `../evidence/045-shared-reference.md` for the corpus-quote map,
the rustc / system toolchain string, the working probe transcript, the
broken-contrast E0308 transcript, and the prerequisite-claim summary.
