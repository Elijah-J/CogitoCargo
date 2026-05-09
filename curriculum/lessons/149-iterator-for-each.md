---
id: 149-iterator-for-each
status: accepted
evidence: ../evidence/149-iterator-for-each.md
---

# Apply a closure to every element with `iter.for_each(closure)`

## The Move

Lesson 148 closed the closure sub-arc: parenthesized-bound generic
functions accept closures, and the bound picks one of `Fn`, `FnMut`,
`FnOnce`. Today is the first stable Iterator method that exercises
that machinery — `for_each`. It takes one closure, calls it once per
element, and returns `()`.

```rust
fn main() {
    (1..4_u32).for_each(|x| println!("{}", x));

    let mut sum: u32 = 0;
    (1..4_u32).for_each(|x| sum += x);
    println!("sum = {}", sum);
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
1
2
3
sum = 6
```

The source `(1..4_u32)` is a `Range<u32>` value, the same shape lesson
091 used to call `.rev()` on a range. Wrapping it in parentheses lets
us call a method on the range value (lesson 091's parens rule).

The signature, verbatim from `output/docs/rust/std/iter/trait.Iterator.md:902`:

```text
fn for_each<F>(self, f: F) where Self: Sized, F: FnMut(Self::Item),
```

Read it segment by segment with the lesson 147 grammar:

- `<F>` — one type parameter for the closure (lesson 145).
- `(self, f: F)` — receiver `self`, consuming. The iterator binding is
  not usable afterwards (Probe 5 in the appendix witnesses E0382).
- *no return type slot* — `for_each` returns `()`. Probe 3 in the
  appendix witnesses this: binding the call to a `u32` fires E0308
  with rustc spelling the result type as `()`.
- `F: FnMut(Self::Item)` — the parenthesized bound (lesson 147),
  with `FnMut` (lesson 148) and the `Self::Item` associated-type slot
  (lesson 132). For a `Range<u32>` the iterator yields *owned* `u32`
  values, so `Self::Item` is `u32`. When you write `|x| ...`, rustc
  reads the closure's expected parameter type from the bound: `x` is
  `u32`. (Probe 8 in the appendix annotates `|x: u32|` explicitly and
  rustc accepts.)

The std-doc one-liner says (`trait.Iterator.md:904-906`): "Calls a
closure on each element of an iterator. This is equivalent to using a
`for` loop on the iterator, although `break` and `continue` are not
possible from a closure."

## Why the bound is `FnMut`, not `Fn`

The second `for_each` call captures `let mut sum: u32 = 0;` from the
enclosing scope (lesson 144) and *mutates* it via `sum += x` — plain
`+=` from lesson 023, on `u32 += u32`. By lesson 148's auto-impl rule,
a closure that mutates a captured binding implements `FnMut` (and
`FnOnce`) but *not* `Fn`. That is exactly why the trait page declares
the bound as `FnMut`: it is the most-permissive bound that still lets
`for_each` call the closure once per element. A `Fn` bound would
reject `|x| sum += x`; an `FnOnce` bound would reject calling the
closure more than once per call site.

You will see the `FnMut(...)` text again in rustc's diagnostic. Pass
a non-closure value — say, the integer `7` — and rustc fires E0277:

```text
error[E0277]: expected a `FnMut(u32)` closure, found `{integer}`
 --> non_closure_arg.rs:4:25
  |
4 |     (1..4_u32).for_each(7);
  |                -------- ^ expected an `FnMut(u32)` closure, found `{integer}`
```

Notice the rustc-spelled bound: `FnMut(u32)`. The closure parameter
type rustc *expects* is `u32` — `Self::Item` for `Range<u32>` — and
the trait is `FnMut`.

## `break` from inside the closure does not work

A `for` loop body is a regular block: you can `break` out. A closure
body is *not* a loop body — it is a separate function-like value. Try
`break` inside the closure:

```rust
(1..4_u32).for_each(|x| {
    if x == 2 { break; }
    println!("{}", x);
});
```

```text
error[E0267]: `break` inside of a closure
 --> break_in_closure.rs:7:13
  |
5 |     (1..4_u32).for_each(|x| {
  |                         --- enclosing closure
6 |         if x == 2 {
7 |             break;
  |             ^^^^^ cannot `break` inside of a closure
```

E0267 is a new error code today. Read with the lesson 003 map:
headline `E0267`, location at the `break` token, source excerpt with
the `--- enclosing closure` annotation pointing at `|x|`. `for_each`
is non-short-circuiting: every element gets visited.

The comparison `x == 2` is plain `==` between two `u32` values
(lesson 013) — `x` is owned, not a reference, so no extra step is
needed.

## Mental Model Delta

- *Before:* "I have closures, parenthesized-bound generic functions,
  and three iterator-method shapes (`.next()`, lazy adapters,
  consumers). I have not yet called an Iterator method that takes a
  closure."
- *After:* "`for_each` is the smallest closure-driven Iterator
  method: it consumes `self`, takes one `FnMut(Self::Item)` closure,
  calls it once per element, and returns `()`. The bound is `FnMut`
  so the closure can mutate captured state across calls. The closure
  body is not a loop body — `break` and `continue` do not reach into
  the surrounding code."

## Prerequisites

- Installed concepts (load-bearing):
  - **148**: `Fn` / `FnMut` / `FnOnce` and the auto-impl rule.
    `|x| sum += x` mutates a captured binding, so `FnMut`.
  - **147**: parenthesized `<F: FnMut(T)>` bound; today is the
    no-return-segment form.
  - **144**: closures capture outer `let` (`sum`).
  - **142**: closure literal `|p| body` — today as a *call argument*.
  - **132**: `Self::Item` associated-type slot.
  - **091**: `Range<A>: Iterator` for `A: Step`; the parens-rule for
    method calls on a range.
  - **081 + 080**: `4_u32` type-suffix form pins `Range<u32>`.
  - **023**: `n += value;` on a `mut` integer binding (both sides
    owned `u32` here).
  - **003**: rustc diagnostic map. E0267 is new; E0277, E0308, E0382,
    E0594 reappear unchanged in shape.
- Cited: 143 (`|x|` no-annotation), 145 (`<F>` slot), 013 (`==` on
  integers), 037 (`%` remainder), 011 (`println!`), 005/006
  (`let`/`let mut`), 002 (`fn main`), 001 (rustc + run).
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is four
lines, `1`, `2`, `3`, `sum = 6`. Then save `non_unit_bind.rs`
(line 4: `let r: u32 = (1..4_u32).for_each(|x| println!("{}", x));`)
and recompile — you should see an E0308 (full transcript in the
evidence appendix as Probe 3), with rustc spelling
`expected `u32`, found `()`` to confirm `for_each` returns `()`.

## What Changed

- `for_each` is the first closure-driven Iterator method. Signature
  `fn for_each<F>(self, f: F) where Self: Sized, F: FnMut(Self::Item)`
  — consume `self`, take one `FnMut` closure, return `()`.
- The closure's parameter type is `Self::Item`. For a `Range<u32>`,
  that is `u32` (owned) — rustc spells the bound `FnMut(u32)` in
  diagnostics.
- The bound is `FnMut`, not `Fn`. A closure that mutates a captured
  binding (`|x| sum += x`) implements `FnMut` (lesson 148); under a
  `Fn` bound it would be rejected.
- `for_each` returns `()`. Binding the call to a non-`()` type fires
  E0308 with `expected <T>, found ()`.
- `break` and `continue` are not allowed inside the closure body —
  it is not a loop body. Trying fires `error[E0267]: \`break\`
  inside of a closure`.

## Check Yourself

You write `q.rs`:

```rust
fn main() {
    let mut count: u32 = 0;
    (1..6_u32).for_each(|x| {
        if x % 2 == 0 { count += 1; }
    });
    println!("{}", count);
}
```

(a) Does `rustc q.rs` compile, and what does `./q` print?

(b) You drop the `mut`, leaving `let count: u32 = 0;`. Which E-code
fires, and where does its `help:` line propose the fix?

(Answers: (a) compiles silently; prints `2` — the even integers in
`1..6` are `2` and `4`. (b) E0594, "cannot assign to `count`, as it
is not declared as mutable." The caret falls on `count += 1` in the
closure body; `help:` proposes `let mut count: u32 = 0;` at the
binding site with `+++` markers. The `FnMut` bound lets the closure
*try* the mutation; lesson 006's rule on the captured binding still
applies.)

## What To Ignore For Now

- **Internal-iteration / `Chain` performance** (`trait.Iterator.md:909-911`):
  implementor-side detail.
- **`try_for_each`**: short-circuiting variant, gated on the `Try`
  sub-arc.
- **The `where Self: Sized` bound**: present but not centered, same
  as lessons 134-141.
- **`for x in (1..4_u32)` desugaring**: pulls in `IntoIterator`; its
  own move.
- **The other 26 closure-driven Iterator methods** — `map`,
  `filter`, `fold`, etc. (audit §4.4.1). Each is its own move.
- **`v.iter()` or `v.into_iter()` as the source**: both work, but
  `v.iter()` over `Vec<u32>` yields `&u32` — the bound becomes
  `FnMut(&u32)` and `sum += x` would need a different mechanic.
  Today's `Range<u32>` is the leaner source.

## Evidence

See `../evidence/149-iterator-for-each.md`.
