---
id: 151-iterator-fold
status: accepted
evidence: ../evidence/151-iterator-fold.md
---

# Reduce a range to one value with `iter.fold(init, |acc, x| ...)`

## The Move

Lesson 149 installed `for_each` (one-closure consumer, returns `()`).
Lesson 150 installed `map` (one-closure lazy adapter with `-> B`).
Today's `fold` is the third closure-driven Iterator method, and the
first whose closure takes **two** parameters: an *accumulator* and an
*element*. `fold` consumes the iterator, calls the closure once per
element threading the accumulator through, and returns the final
accumulator.

```rust
fn main() {
    let s = (1..4_u32).fold(0_u32, |acc, x| acc + x);
    println!("{}", s);
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
6
```

The signature, verbatim from `output/docs/rust/std/iter/trait.Iterator.md:2365`:

```text
fn fold<B, F>(self, init: B, f: F) -> B
   where Self: Sized,
         F: FnMut(B, Self::Item) -> B,
```

Read it with the lesson 147 grammar. `<B, F>` is two type parameters
(same as `map`). `(self, init: B, f: F)` is the receiver plus **two**
non-receiver arguments — the first Iterator method in the run with a
non-self argument list of length two. `-> B` says `fold` returns a `B`
(not `()`, not a wrapper struct). The new piece today is *inside the
parens of the bound*: `FnMut(B, Self::Item) -> B` has **two parameter
slots, comma-separated**, instead of one. That is the closure analogue
of lesson 036's comma-separated function parameter list — first slot
`B`, second slot `Self::Item`, return `B`.

For `(1..4_u32)`, `Self::Item = u32` (lesson 091 + 081). The literal
`0_u32` fixes `init: B`, so `B = u32`. The closure `|acc, x| acc + x`
therefore receives two `u32` parameters and must return a `u32`. Probe
5 in the appendix is the empirical witness: pass a non-closure (`7`)
as the second argument and rustc spells the expected type as
`FnMut(u32, u32)` — both slots visible.

## How `fold` threads the accumulator

For Probe 1 the iteration walks like this (the corpus has the same
table at `trait.Iterator.md:2421-2427`):

| step | acc (in)  | x  | acc (out, returned by closure) |
| ---- | --------- | -- | ------------------------------ |
| 1    | 0 (init)  | 1  | 0 + 1 = 1 |
| 2    | 1         | 2  | 1 + 2 = 3 |
| 3    | 3         | 3  | 3 + 3 = 6 |

After the last element there are no more steps, and `fold` returns the
final accumulator `6`. Probe 2 in the appendix swaps `init` to `100_u32`
and prints `106` — direct empirical witness that `init` is genuinely
the value `acc` starts at.

## Two contrasts

Pass a *one-parameter* closure to `fold` and you get a new error code
today, **E0593** (Probe 3):

```text
error[E0593]: closure is expected to take 2 arguments, but it takes 1 argument
 --> wrong_arity.rs:5:24
  |
5 |     let s = (1..4_u32).fold(0_u32, |x| x);
  |                        ^^^^        --- takes 1 argument
  |                        |
  |                        expected closure that takes 2 arguments
```

The bound's parens segment is structurally a count, not just a label.

Pass an `init` whose type disagrees with `Self::Item` (Probe 4,
`init = 0_i32` against the `u32`-yielding source) and `B` is fixed to
`i32` from `init`, so the closure body `acc + x` becomes `i32 + u32` —
rustc fires both E0308 and E0277. Change `init` and the closure's
expected shape changes with it.

## Mental Model Delta

- *Before:* "Closure-driven Iterator methods I have seen take a closure
  with *one* parameter (the element). The bound is some shape of
  `FnMut(Self::Item)`, optionally with `-> B`."
- *After:* "Some Iterator methods take an initial accumulator value
  *and* a closure with **two** parameters — accumulator and element,
  comma-separated inside the bars, same shape as a function's parameter
  list. `fold` is the canonical one. It consumes `self`, threads the
  accumulator through (each closure call's return becomes the next
  call's first argument), and returns the final accumulator."

## Prerequisites

- Installed concepts (load-bearing):
  - **150** (`map`): `<B, F>` two type parameters and the `-> B` arrow
    segment of the bound. Today reuses both.
  - **149** (`for_each`): consuming `self` shape. Today's `fold` also
    takes `self` by value (Probe 6 witnesses E0382 with
    `note: \`fold\` takes ownership of the receiver \`self\``).
  - **148** (`FnMut`): the auto-impl rule. Today's closures capture
    nothing.
  - **147**: parenthesized `<F: FnMut(T) -> R>` bound. Today extends
    the parens segment from one slot to two.
  - **142** + **036**: closure literal `|param| body` and the comma-
    separated parameter list `name: TYPE, name: TYPE`. Today writes
    `|acc, x|` — pipes around a comma-separated list of two parameter
    names, same shape as a function's parameter list.
  - **132**: `Self::Item` slot. Today's bound's second parameter is
    `Self::Item`; for `Range<u32>` this resolves to `u32`.
  - **091** + **081** + **080**: `1..4_u32` is a `Range<u32>`,
    `Self::Item = u32`; the suffix on `init` (`0_u32`) pins `B = u32`.
  - **025**: implicit-return rule. The closure body `acc + x` (no
    `;`) is a single expression whose value is the closure's return.
  - **009**: `+` and `*` on integers.
  - **003**: rustc diagnostic map. **E0593** is new today.
- Cited: 145 (`<F>` slot, here `<B, F>`), 143 (`|x|` no-annotation),
  144 (capture mechanic — today's closures capture nothing), 020
  (typed parameter shape `name: TYPE` reused inside `|acc: u32, x: u32|`
  in Probe 2), 011, 005, 002, 001.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`; output is `6`. Edit `0_u32` to
`100_u32` (Probe 2's shape); output is `106`. Same source, same
closure, same iteration count — the only thing that changed is what
`acc` starts at.

## What Changed

- `fold` is the third closure-driven Iterator method. Signature
  `fn fold<B, F>(self, init: B, f: F) -> B where Self: Sized,
  F: FnMut(B, Self::Item) -> B`. Two non-receiver arguments
  (`init: B` and `f: F`), return type `B`.
- The closure has **two** parameters, comma-separated inside the bars
  — `|acc, x|`, spelled `FnMut(u32, u32)` in rustc's diagnostics.
  Same comma-separated shape as a function's parameter list (lesson
  036), now inside the closure pipes.
- The accumulator threads through the iteration: rustc starts with
  `init`, then on each step passes `(current_acc, next_element)` to
  the closure and takes the closure's return as the *new* `acc`.
  After the last element, the final `acc` is `fold`'s return value.
- `init`'s type fixes `B`; the closure's accumulator parameter and
  return type must agree with `B`. Mismatch fires E0308 + E0277
  (Probe 4).
- A closure with the wrong number of parameters fires **E0593**
  `closure is expected to take 2 arguments, but it takes 1 argument`
  (Probe 3). E0593 is new today.
- `fold` consumes `self` (Probe 6 — E0382 with `note: \`fold\` takes
  ownership of the receiver \`self\``).

## Check Yourself

You write `q.rs`:

```rust
fn main() {
    let p = (1..4_u32).fold(1_u32, |acc, x| acc * x);
    println!("{}", p);
}
```

(a) Does `rustc q.rs` compile silently, and what does `./q` print?

(b) You change the initial value from `1_u32` to `0_u32` (everything
else the same). What does `./q` print, and why?

(Answers: (a) Compiles silently; prints `6`. Iteration: `1*1=1`,
`1*2=2`, `2*3=6`. (b) Prints `0`. The first closure call is
`0 * 1 = 0`, then every later step is `0 * x = 0`. `init` is what
`acc` *starts at*, so picking `0` for a multiplicative fold collapses
the whole thing.)

## What To Ignore For Now

- **`reduce`** — variant of `fold` that uses the *first* element as
  the initial accumulator; returns `Option<Self::Item>`. Separate
  move (`trait.Iterator.md:2469`).
- **`try_fold`** — short-circuiting variant; gates on the `Try`
  trait sub-arc.
- **`rfold`** — right-associative version, defined on
  `DoubleEndedIterator`. Gates on that sub-arc
  (`trait.Iterator.md:2392-2395`).
- **Accumulator type different from element type** — today keeps
  `B = Self::Item = u32` for surface minimality; the mechanic itself
  permits any `B`.
- **Internal-iteration note** (`trait.Iterator.md:2397-2404`) —
  implementor-side.
- **`v.iter()` / `v.into_iter()` as the source** — each composes a
  different `Self::Item` resolution.

## Evidence

See `../evidence/151-iterator-fold.md`.
