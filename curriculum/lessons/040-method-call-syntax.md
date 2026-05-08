---
id: 040-method-call-syntax
status: accepted
evidence: ../evidence/040-method-call-syntax.md
---

# Call a method on a value with `value.method(args)`

## The Move

So far calls have one shape: lesson 008's `name(args)`. Many functions
in Rust's standard library are *not* reachable that way — they live
attached to a specific type, and the natural way to call them is the
*method-call form*:

```rust
let n: i32 = -7;
let m: i32 = n.abs();
```

The expression `n.abs()` calls the `abs` *method* on the value held by
`n` and produces `7`, the absolute value of `-7`. The pieces, left to
right: `n` is the *receiver* (the value being acted on); `.` is the
method-call dot; `abs` is the method name; `()` is the (empty)
argument list. The whole thing is an expression of type `i32`, so it
fits on the right of `let` (lesson 021). There is no free-standing
function called `abs` in scope — if you write `abs(n)`, rustc rejects
with E0425 *and tells you to use the dot*.

## Mental Model Delta

- Before: "Calls have one shape, `name(args)`."
- After: "Calls have *two* shapes. The lesson-008 form `name(args)` is
  for free functions in scope. The new form `receiver.method(args)`
  is for functions *associated with a type* — the receiver provides
  both the input value and the namespace the method lives in. The dot
  is not a new control-flow rule; it is a different syntax for
  calling a function."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 005, 019: `rustc file.rs` then `./name`, silent
    on success; `fn main` is the entry point; `let name: TYPE = value;`
    binds a name; `i32` is the default integer type.
  - Lesson 003 (load-bearing): rustc diagnostics have headline + `-->`
    + source excerpt + caret + optional `help:` lines. The broken-
    contrast probe is read with that map.
  - Lesson 008 (load-bearing): `fn name() { ... }` plus `name();`
    introduced *free-function calls* — the call shape this lesson
    contrasts with. Lesson 008 also installed E0425 ("cannot find
    function in this scope") — the same E-code fires here when the
    free-function form is wrongly used for a method.
  - Lessons 020, 021: parameter lists and return types. Methods reuse
    the same machinery — `abs` takes no extra arguments and returns
    `i32`, so it slots into the right of `let`.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let n: i32 = -7;
    let m: i32 = n.abs();
    println!("n = {n}, m = {m}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
n = -7, m = 7
```

Walk it. The new line is `let m: i32 = n.abs();`. Read the right-hand
side as a *call*, just like lesson 008's `name()`, but in a different
shape. Instead of a bare name `abs` followed by `()`, you write the
*receiver* `n` first, then `.`, then the method name `abs`, then the
argument list `()`. The argument list is empty here — but the parens
are still required, the same way `name();` requires `()`. The whole
expression `n.abs()` produces an `i32`, lesson 021's rule carries it
to the right of `let`, and `println!` prints `n = -7, m = 7`.

Now the contrast. *Predict*: what happens if you write the free-
function form `abs(n)` instead? Edit `demo.rs` so the third line reads
`let m: i32 = abs(n);` and recompile. rustc emits:

```
error[E0425]: cannot find function `abs` in this scope
 --> broken.rs:3:18
  |
3 |     let m: i32 = abs(n);
  |                  ^^^ not found in this scope
  |
help: use the `.` operator to call the method `abs` on `i32`
  |
3 -     let m: i32 = abs(n);
3 +     let m: i32 = n.abs();
  |
```

Same E-code as lessons 005 and 008 (missing-name errors), but with a
`help:` block that names the fix directly: "use the `.` operator to
call the method `abs` on `i32`", with a source-diff replacing
`abs(n)` with `n.abs()`. That `help:` line is rustc's own statement
of this lesson's whole move, captured in the broken-contrast probe.

(Full transcripts are in `../evidence/040-method-call-syntax.md`.)

## What Changed

- You can call a method on a value with the shape
  `receiver.method(args)`: receiver, dot, method name, parenthesized
  argument list.
- You know there are *two* call shapes in Rust now: lesson 008's
  free-function form `name(args)` and this lesson's method-call form
  `value.method(args)`. They are different syntactic surfaces for
  calling a function; nothing about scopes, control flow, or returns
  is new.
- You know one concrete method, `i32::abs`, takes no extra arguments
  and returns the absolute value as an `i32`.
- You know the failure mode: writing `abs(n)` for a method-only name
  fires E0425 — the same E-code as missing values (lesson 005) and
  missing free functions (lesson 008) — and rustc points to the dot
  fix directly.

## Check Yourself

You write `pred.rs` containing:

```rust
fn main() {
    let a: i32 = -42;
    let b: i32 = a.abs();
    println!("a = {a}, b = {b}");
}
```

(a) Does rustc accept the program?

(b) What single line does `./pred` print?

(c) If you replaced line 3 with `let b: i32 = abs(a);` and recompiled,
which E-code would the headline carry, and what would the `help:` line
suggest?

(Answers: (a) Yes — `a.abs()` is the same shape as the lesson, with a
different receiver name. (b) `a = -42, b = 42`. The `abs` method
returns `42` for the input `-42`. (c) E0425 ("cannot find function
`abs` in this scope"); the `help:` block suggests "use the `.`
operator to call the method `abs` on `i32`", with a source-diff
replacing `abs(a)` with `a.abs()`.)

## What To Ignore For Now

This lesson installs only one idea: the syntax `value.method(args)`
calls a method on a value. Deferred:

- *Where methods are defined* — Rust uses `impl` blocks
  (`impl i32 { ... }` conceptually). For now, treat methods as facts
  of the language and look them up on the type's std-library page.
- *The `self` parameter*, including its reference forms `&self` and
  `&mut self`. Most std methods you will meet later (e.g. on `String`,
  `Vec`, `Option`) borrow `&self` or `&mut self` rather than consume.
  Future move alongside references.
- *Associated-function form `Type::name(args)`* — the *other* way to
  reach functions attached to a type, e.g. `String::new()` or
  `i32::from_str_radix("ff", 16)`. Future move; the `::` form vs.
  the `.` form is its own contrast.
- *Method chaining* — `a.b().c()`. Each `.method()` returns a value
  that can be the receiver for the next `.method()`. Future move.
- *Generic methods* — `"42".parse::<u32>()`. The `::<...>` after a
  method name supplies a type parameter. Future move.
- *Traits and `impl Trait for Type`*. Many methods come from traits
  rather than directly from a type. The Reference's method-call page
  mentions trait dispatch; ignore that for now.
- *Runtime panic on `i32::MIN.abs()`*. The std page warns the
  absolute value of `i32::MIN` cannot be represented as an `i32` and
  panics in debug mode. Future move under runtime panics / overflow.
- All previously deferred items.

## Evidence

See `../evidence/040-method-call-syntax.md` for the corpus-quote map,
the rustc / system toolchain string, the working probe transcript,
the broken-contrast E0425 transcript, and the prerequisite-claim
summary.
