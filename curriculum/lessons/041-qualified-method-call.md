---
id: 041-qualified-method-call
status: accepted
evidence: ../evidence/041-qualified-method-call.md
---

# Call a method with the qualified form `Type::method(receiver, args)`

## The Move

Lesson 040 introduced the dot-form for calling a method on a value:

```rust
let n: i32 = -7;
let m: i32 = n.abs();   // dot-form
```

Rust has a *second* call shape for the same method, the *qualified
form*:

```rust
let n: i32 = -7;
let m: i32 = i32::abs(n);   // qualified form
```

The pieces, left to right: `i32` is the type the method is associated
with; `::` is the path separator; `abs` is the method name; `(n)` is
the argument list, with the *receiver* `n` passed as the first
argument. Both forms produce the same value `7`, and either fits on
the right of `let m: i32 = ...;`.

## Mental Model Delta

- Before: "Methods are called with `value.method(args)`. That is the
  one method-call shape."
- After: "There are *two* method-call shapes that reach the same
  method. The qualified form `Type::method(receiver, args)` names the
  type explicitly and lists the receiver as the first argument; the
  dot-form is shorter sugar. `n.abs()` and `i32::abs(n)` produce the
  same value."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 005, 019, 020, 021: `rustc`/`./name`, `fn main`,
    `let name: i32 = value;`, typed parameters, return types.
  - Lesson 003 (load-bearing): rustc diagnostics have headline + `-->`
    + source excerpt + caret + optional `note:`/`help:` lines.
  - Lesson 036 (load-bearing): arity mismatch fires E0061 "this
    function takes N arguments but M arguments were supplied," with
    a `note: ... defined here` line. The broken-contrast probe here
    reuses that exact diagnostic shape.
  - Lesson 040 (load-bearing): the dot-form `value.method(args)` and
    the concrete method `i32::abs`. This lesson's whole point is "the
    dot-form has a sibling that reaches the same method."
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

In a fresh empty directory, create `demo.rs`:

```rust
fn main() {
    let n: i32 = -7;
    let dot: i32 = n.abs();
    let qual: i32 = i32::abs(n);
    println!("dot = {dot}, qualified = {qual}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
dot = 7, qualified = 7
```

Line 3 is lesson 040 (dot-form, receiver-first). Line 4 is the new
shape: type first (`i32`), then `::`, then the method name (`abs`),
then a parenthesized argument list with the receiver `n` *inside*
the parens as the first argument. The output proves both calls reach
the same underlying function — `dot` and `qual` are both `7`.

*Predict*: what happens if you write the qualified form with empty
parens, `i32::abs()`? Edit line 4 to that and recompile. rustc emits:

```
error[E0061]: this function takes 1 argument but 0 arguments were supplied
 --> broken.rs:3:21
  |
3 |     let qual: i32 = i32::abs();
  |                     ^^^^^^^^-- argument #1 of type `i32` is missing
  |
note: method defined here
 ...
help: provide the argument
  |
3 |     let qual: i32 = i32::abs(/* i32 */);
  |                              +++++++++
```

Same E-code as lesson 036. The headline says 1 argument expected, 0
supplied; the caret highlights the missing slot inside the parens;
`note: method defined here` points back to the std-library definition.
The receiver is not free — in the qualified form it is the first
required argument.

(Full transcripts are in `../evidence/041-qualified-method-call.md`.)

## What Changed

- You can call a method using the qualified form
  `Type::method(receiver, args)` — type, `::`, method name, parens
  with the receiver as the first argument.
- For the receiver-bearing methods you have met, the qualified and
  dot-forms are equivalent: `n.abs()` and `i32::abs(n)` produce the
  same value.
- You know the failure mode: `i32::abs()` with no arguments fires
  E0061 (lesson 036's E-code). The receiver is mandatory; the
  dot-form just hides that fact by writing it before the dot.

## Check Yourself

You write `pred.rs` containing:

```rust
fn main() {
    let a: i32 = -42;
    let b: i32 = i32::abs(a);
    println!("b = {b}");
}
```

(a) Does rustc accept the program?

(b) What single line does `./pred` print?

(c) If you replaced line 3 with `let b: i32 = i32::abs();` and
recompiled, which E-code would the headline carry, and what would the
headline text say about the number of arguments?

(Answers: (a) Yes — the qualified form with one argument matches the
single-receiver signature `pub const fn abs(self) -> i32`. (b)
`b = 42`. (c) E0061; "this function takes 1 argument but 0 arguments
were supplied.")

## What To Ignore For Now

This lesson installs only one idea: the qualified form
`Type::method(receiver, args)` is a second call shape for the
receiver-bearing methods of lesson 040. Deferred:

- *Associated functions without a receiver* — `String::new()`,
  `Vec::with_capacity(10)`. The qualified form reaches those too, but
  the no-receiver case is its own move on a different type.
- *Nested paths* — `std::cmp::min(a, b)`, where the path has more
  than one `::` segment. Future move.
- *`use` declarations* — `use std::cmp::min;` shortens a long path.
  Future move.
- *Trait-disambiguation form* `<T as Trait>::method(...)`. Future move
  alongside traits.
- *`Self::method` form inside `impl` blocks*. `impl` itself is still
  deferred from lesson 040.
- *Method-resolution autoref/autoderef*. The dot-form silently inserts
  `&` or `&mut` for receiver-by-reference methods; the qualified form
  does not. Not visible on `i32::abs(self)` because `abs` takes `self`
  by value. Future move alongside references.
- All previously deferred items.

## Evidence

See `../evidence/041-qualified-method-call.md` for the corpus-quote
map, the rustc / system toolchain string, the working probe
transcript, the broken-contrast E0061 transcript, and the
prerequisite-claim summary.
