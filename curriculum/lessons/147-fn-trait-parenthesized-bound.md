---
id: 147-fn-trait-parenthesized-bound
status: accepted
evidence: ../evidence/147-fn-trait-parenthesized-bound.md
---

# A `Fn`-family trait bound uses *parens*: `<F: Fn(T) -> R>`

## The Move

Lesson 146 added a *trait bound* to a generic function — `<T: Display>`
restricts `T` to types that implement `Display`. Today extends that
shape to a special-cased family of traits used for *callable values*:
`Fn`, `FnMut`, `FnOnce`. For these traits the bound carries the
parameter and return types of the callable *inside parentheses*, not
inside angle brackets.

```rust
fn apply<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
    f(x)
}

fn main() {
    let add_one = |n: u32| n + 1;
    let r = apply(add_one, 5);
    println!("{}", r);
}
```

`rustc demo.rs` is silent; `./demo` prints one line:

```text
6
```

The bound is `F: Fn(u32) -> u32`. Same colon and same angle
brackets lesson 146 used for `T: std::fmt::Display` — but the
trait *carries arguments* in parens, followed by `-> R`. The
standard-library `Fn` trait page calls this "the special syntax
for `Fn` traits"; the Reference lists it as a path-segment form
where parenthesized inputs and an optional `-> Type` follow the
trait name. `F` then sits in the parameter slot `(f: F, ...)` as
a normal lesson-020 parameter.

The call site passes a *closure literal* — lesson 142's
`|n: u32| n + 1` shape, bound to `let add_one = ...`. Its
parameter `n: u32` and return value `n + 1` (a `u32`) match the
shape inside the parens. rustc substitutes the closure's
anonymous type for `F`. The body `f(x)` then reuses lesson 008's
parens-call shape on the parameter name; that is what the bound
*enables*.

## The contrast: shape mismatch fails at the *call site*

Same source, but pass a closure whose return type is `i32` instead
of `u32`:

```rust
fn main() {
    let returns_i32 = |n: u32| n as i32;
    let _ = apply(returns_i32, 5);
}
```

`rustc wrong_ret.rs` rejects it:

```text
error[E0271]: expected `{closure@wrong_ret.rs:6:23}` to return `u32`, but it returns `i32`
 --> wrong_ret.rs:6:32
  |
6 |     let returns_i32 = |n: u32| n as i32;
  |                       -------- ^^^^^^^^ expected `u32`, found `i32`
  |                       |
  |                       this closure
7 |     let _ = apply(returns_i32, 5);
  |             ----- ----------- closure used here
  |             |
  |             required by a bound introduced by this call
  |
note: required by a bound in `apply`
 --> wrong_ret.rs:1:24
  |
1 | fn apply<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
  |                        ^^^ required by this bound in `apply`
```

Read with the lesson 003 map. Headline: a *new* error code, `E0271`,
inline label "expected ... to return `u32`, but it returns `i32`."
Caret on the closure body `n as i32`. The follow-up `note: required
by a bound in \`apply\`` points at the `u32` *return slot of the
parenthesized bound* inside the function declaration, with caret on
exactly those three characters. rustc matches the closure's shape
against the bound's parens-and-arrow shape segment by segment. A
parameter-type mismatch instead fires `E0631`; a non-closure
argument fires `E0277` — lesson 146's E-code — with the same
"required by a bound in `apply`" pointer.

## Mental Model Delta

- *Before:* "A trait bound is `<T: TraitName>` — a colon, then the
  trait path, inside angle brackets (lesson 146)."
- *After:* "Most trait bounds use that inline form, but the `Fn`,
  `FnMut`, and `FnOnce` traits have a *parenthesized* sugar:
  `<F: Fn(T1, T2) -> R>` — the parameter types live inside
  parens after the trait name, and `-> R` after the parens names the
  return type. This is the trait shape used to describe *callable*
  values. With such a bound, the call site can pass any closure
  literal whose parameter and return shape matches; the body can
  call the parameter with parens."

## Prerequisites

- Installed concepts:
  - **Lesson 146** (load-bearing): `<T: TRAIT>` — the inline trait
    bound. Today is a special-case grammar for *one family* of
    traits, written with parens instead of angle-bracketed args.
  - **Lesson 145** (load-bearing): `fn name<T>(...)` — type
    parameter on a function header. Today's `<F: ...>` sits in the
    same slot; only the bound shape is new.
  - **Lesson 142** (load-bearing): closure literal `|n: u32| body`.
    Today's argument is exactly this; the closure's shape is what
    rustc matches against the bound.
  - **Lesson 003** (load-bearing): rustc diagnostic map. E0271 is
    new today; the four-part shape carries unchanged, with `note:
    required by a bound in <fn>` pointing at the bound's parenthesized
    return slot.
  - Cited: lessons 008 (call shape `f(x)`), 020 (parameter slot
    `f: F`), 021 (return type `-> u32`), 080 (`u32` named
    integer type used in the parens and on the function
    parameters); 011, 005, 002, 001.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is `6`.
Now save `wrong_ret.rs` — change only the closure to
`|n: u32| n as i32` — and recompile. The E0271 transcript above is
what you should see.

## What Changed

- For `Fn`, `FnMut`, and `FnOnce`, a trait bound is written with
  *parens* around the parameter types and `-> R` for the return
  type: `<F: Fn(u32) -> u32>`.
- A function with such a bound accepts any *closure literal* whose
  parameter and return shape matches. Lesson 142's `|n: u32| n + 1`
  is one such literal.
- The bound enables the body to call the parameter with parens —
  `f(x)` — same call shape as lesson 008.
- A shape mismatch fires either E0271 (return-type mismatch) or
  E0631 (parameter-type mismatch); the diagnostic's `note:` points
  back at the bound's offending slot inside the function
  declaration. A non-closure argument fires the lesson-146-familiar
  E0277.

## Check Yourself

You write `tiny.rs`:

```rust
fn run<G: Fn(i64) -> i64>(g: G, k: i64) -> i64 { g(k) }

fn main() {
    let triple = |n: i64| n * 3;
    println!("{}", run(triple, 7));
}
```

(a) Does it compile, and what does it print? (b) Change the
closure to `|n: i64| (n * 3) as i32` — return value is now `i32`.
Which E-code fires, and what does the `note: required by a bound
in <fn>` block point at?

(Answers: (a) silent; prints `21`. (b) E0271; the `note:` points at
the `i64` *return slot* inside the parenthesized bound on `run`.)

## What To Ignore For Now

This lesson installs *only* the parenthesized `Fn(T) -> R` bound
on a generic function plus closure-as-argument. With it, closure
sub-arc step 4 closes (lessons 145+146+147). Deferred:

- **The `Fn` / `FnMut` / `FnOnce` distinction** — three traits in
  the family with different rules on call-once-vs-repeated and
  capture mutation. Lesson 148, the closure sub-arc closer. Today
  uses only `Fn`. Why `FnMut` would force `mut f: F` (rustc fires
  E0596 without `mut`) is also Lesson 148.
- **Closure capture rules** — what the body reads, mutates, or
  moves determines which `Fn`-family trait the closure satisfies.
  Lesson 148 territory.
- **`Iterator` methods that take closures** — `for_each`, `map`,
  `filter`, ...: all use the parenthesized `Fn`-family bound
  shape. Teachable after lesson 148.
- **Function pointers `fn(u32) -> u32`** — lowercase `fn`, a
  *type* (not a trait); separate mechanic.
- **`impl Fn(...)` and `Box<dyn Fn(...)>` / `&dyn Fn(...)`** —
  parameter-position sugar and dynamic dispatch alternatives.
- **Multiple parameters `Fn(T, U) -> R`, no-return `Fn(T)`,
  higher-ranked `for<'a> Fn(&'a T) -> R`** — same mechanic
  extended; deferred.
- **The desugaring** — `Fn(T) -> R` is sugar for
  `Fn<(T,), Output = R>`. Implementor-side; deferred.

## Evidence

See `../evidence/147-fn-trait-parenthesized-bound.md`.
