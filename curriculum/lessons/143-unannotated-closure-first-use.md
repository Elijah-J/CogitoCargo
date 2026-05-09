---
id: 143-unannotated-closure-first-use
status: accepted
evidence: ../evidence/143-unannotated-closure-first-use.md
---

# Drop the closure parameter annotation; the first call fixes its type

## The Move

Lesson 142 wrote `|x: u32| x + 1` — a closure literal with the
parameter slot fully annotated. Today drops the annotation. The
parameter list becomes just `|x|` (Book v3/v4 form). Rustc still
accepts the literal, but it now has to learn `x`'s type from
*somewhere else*. Today installs the rule for where: the *first*
call site.

```rust
fn main() {
    let id = |x| x;
    let a = id(5_u32);
    let b = id(10_u32);
    println!("{}", a);
    println!("{}", b);
}
```

`rustc demo.rs` is silent; `./demo` prints two lines:

```text
5
10
```

The closure literal is `|x| x` — `|x|` is the parameter list with no
type annotation, `x` is the body (a single expression that returns its
own argument). Both call sites pass `u32`-suffixed integer literals
(lesson 081's `5_u32` form). Rustc reads `x: u32` off the *first* call
`id(5_u32)`, then checks the second call against that. Both calls
agree, so the program builds and runs.

## The new fact: first call fixes the type

Now write the contrast. Save `twocalls.rs` with one digit changed:

```rust
fn main() {
    let id = |x| x;
    let a = id(5_u32);
    let b = id(5_i32);
    println!("{} {}", a, b);
}
```

`rustc twocalls.rs` emits E0308:

```text
error[E0308]: mismatched types
 --> twocalls.rs:4:16
  |
4 |     let b = id(5_i32);
  |             -- ^^^^^ expected `u32`, found `i32`
  |             |
  |             arguments to this function are incorrect
  |
note: expected because the closure was earlier called with an argument of type `u32`
 --> twocalls.rs:3:16
  |
3 |     let a = id(5_u32);
  |             -- ^^^^^ expected because this argument is of type `u32`
  |             |
  |             in this closure call
note: closure parameter defined here
 --> twocalls.rs:2:15
  |
2 |     let id = |x| x;
  |               ^
help: change the type of the numeric literal from `i32` to `u32`
```

Three things stand out in the diagnostic, read with the lesson 003 map:

- The headline reports E0308 at the *second* call (line 4), not at the
  closure literal on line 2. Rustc accepted `|x| x` cleanly.
- The first `note:` reads "expected because the closure was earlier
  called with an argument of type `u32`" and points back at line 3,
  the *first* call. Rustc explicitly says: the first call fixed `x`'s
  type to `u32`; this later call disagrees.
- The second `note:` ("closure parameter defined here") underlines the
  `x` inside `|x|`. That is the parameter whose type just got fixed.

The `help:` line suggests changing `5_i32` to `5_u32`. Notice what is
*not* offered: rustc does not suggest making `id` accept both types.
The closure binds *one* concrete parameter type per parameter, fixed
at first use.

To witness that it really is the *first* call doing the fixing, swap
the order of the two call sites and recompile. The error flips: now
rustc says "expected because the closure was earlier called with an
argument of type `i32`" and reports `expected i32, found u32` at the
new second call. The transcript captures both directions.

The Book installs the same rule at
`output/docs/rust/book/ch13-01-closures.md:225-284`: "the compiler
will infer one concrete type for each of [the closure's] parameters
and for their return value... Those types are then locked into the
closure ... and we get a type error when we next try to use a
different type with the same closure."

## Mental Model Delta

- *Before:* "A closure literal `|x: T| body` has the parameter type
  written in the pipes (lesson 142). I assume the type is part of
  writing the closure."
- *After:* "The annotation is optional. `|x| body` is legal; rustc
  infers `x`'s type from the *first* call. The closure is then
  locked to that one type — calling it later with a different type
  fires E0308, with a `note:` block tying the rejection back to the
  first call. Closures are not generic: `|x| x` is one closure with
  one parameter type, picked at first use."

## Prerequisites

- Installed concepts:
  - **Lesson 142** (load-bearing): closure literal bound to a `let`
    and called with parens. Today drops the parameter annotation from
    `|x: u32| x` to `|x| x`; everything else is unchanged.
  - **Lesson 003** (load-bearing): rustc diagnostic map. Today's new
    feature is a second `-->` location inside a `note:` block,
    pointing at a different line from the headline.
  - **Lesson 081** (load-bearing): integer type suffix `5_u32`. Without
    it, bare `5` would default to `i32` and the negative contrast
    would not fire.
  - **Lesson 080** (load-bearing): `u32` and `i32` are distinct
    integer types.
  - **Lesson 005** (load-bearing): `let name = value;`.
  - Cited: lesson 008 (call-with-parens), lesson 011 (`println!` with
    `{}`), lesson 002 (`fn main`), lesson 001 (`rustc file.rs`,
    `./name`).
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

In a fresh empty directory, save the working probe as `demo.rs` from
the snippet above. Run `rustc demo.rs && ./demo`. Output: two lines,
`5` then `10`. Now save `twocalls.rs` with `5_u32` and `5_i32` as the
two arguments. Run `rustc twocalls.rs`; the diagnostic above is what
you should see, with the two `note:` blocks pointing at lines 3 and
2 respectively.

For the directionality witness, copy `twocalls.rs` to `orderswap.rs`,
swap the two arguments (`5_i32` first, then `5_u32`), and recompile.
Now the headline says `expected i32, found u32` and the `note:` says
"earlier called with an argument of type `i32`". The first call wins.

## What Changed

- A closure parameter does not need a type annotation. `|x| body` is
  legal alongside lesson 142's `|x: T| body` form.
- For an unannotated closure, rustc infers the parameter type from
  the *first* call site, then locks the closure to that one type.
- A later call with a different type fires `error[E0308]: mismatched
  types` at the later call, with a `note:` block at a second `-->`
  location pointing back to the first call ("expected because the
  closure was earlier called with an argument of type X").
- The `help:` block suggests changing the *argument's* type, not the
  closure's — a closure literal is not generic.
- Order matters: swapping the call sites flips which type wins.

## Check Yourself

You write `q.rs`:

```rust
fn main() {
    let f = |x| x;
    let _a = f("hi");
    let _b = f(7_i32);
}
```

Predict:

(a) Will `rustc q.rs` accept this?

(b) If not, which line does the headline `-->` point at, and what
type does rustc say it expected?

(c) Where does the `note:` `-->` point — and which call fixed the
expected type?

(Answers: (a) reject. (b) the `-->` points at line 4 with `expected
&str, found i32`. The first call `f("hi")` passed a string-literal
value, fixing `x` at a string type. (c) the `note: expected because
the closure was earlier called with an argument of type ...` points
at line 3, the first call. You do not need to know what `&str` is to
read the diagnostic; the lesson 003 map plus today's "first call
fixes the type" rule are enough. The `&str` type itself is deferred.)

## What To Ignore For Now

This lesson installs *only* the unannotated closure parameter and
the first-call-fixes-the-type rule. Deferred:

- **Closure capturing outer bindings** — closure sub-arc step 3.
- **The `Fn` / `FnMut` / `FnOnce` traits** — steps 4-5.
- **Passing closures to functions** as parameters; `impl Fn` /
  `Box<dyn Fn>` return forms.
- **Generic functions over closures** — step 4.
- **The `&str` type** — the Check-Yourself diagnostic mentions the
  name; the type itself is deferred.
- **Closure return-type annotation** — Book v2's `-> RTYPE` form.
- **Default-integer inference inside closure bodies** — what bare
  unsuffixed literals do. The suffix probes bypass this.
- **The `{closure@<loc>}` opaque-type spelling** as a category —
  lesson 142 saw rustc say it; deepening is deferred.

## Evidence

See `../evidence/143-unannotated-closure-first-use.md`.
