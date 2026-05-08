---
id: 079-for-over-array
status: accepted
evidence: ../evidence/079-for-over-array.md
---

# Iterate an array's elements with `for element in a { ... }`

## The Move

Lesson 022 installed the shape `for var in 0..N { ... }` — repeat the
body once for each number in a range. Today extends that exact shape
to *arrays* (lesson 076). Drop an array value into the slot where
`0..N` used to sit, and the loop runs the body once for each *element*
of the array, with the loop variable bound to that pass's element:

```rust
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("the value is: {}", element);
}
```

Five passes. The first binds `element` to `10`, the next to `20`,
then `30`, `40`, `50`. Then the array is exhausted and the loop
exits. No new keyword, no `let mut`, no counter. The indexing of
lesson 077 and the bounds rule of lesson 078 are not in the picture
either: the loop never builds an index, so it cannot build a *bad*
one.

## Mental Model Delta

- *Before:* "To visit every slot of an array I have to compose lessons
  022 and 077 — `for i in 0..a.len() { ... a[i] ... }` — or write the
  older `while index < a.len()` shape from lesson 017."
- *After:* "Rust accepts an array directly in the `for ... in ...`
  collection slot. `for element in a { ... }` runs the body once per
  element, binding the loop variable to that element's *value* — not
  to an index. Same `for X in COLLECTION { ... }` shape as a range
  loop; just put an array where the range used to be. This is the
  preferred shape for visiting an array: no index, no `mut`, no chance
  of an out-of-bounds read."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002 (load-bearing): `rustc file.rs` then `./name`,
    silent on success.
  - Lesson 005 (load-bearing): `let name = value;` — used for
    `let a = [...];`.
  - Lesson 011 (cited): `println!("...{}", expr)` for the body's print.
  - Lesson 017 (cited): `while condition { ... }` — used in the
    side-by-side contrast (with `mut` from 006, `<` from 013, `+` from
    009, all already installed).
  - Lesson 022 (load-bearing): `for var in 0..N { ... }`. Today
    extends the COLLECTION slot from ranges to arrays.
  - Lesson 076 (load-bearing): the array literal and `[T; N]` type.
  - Lesson 077 (cited): `a[i]` and `usize` — referenced only in the
    contrast framing, not used by the working probe.
  - Lesson 078 (cited): the runtime out-of-bounds panic — the failure
    mode iteration avoids.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

Five lines, one per element, in array order. The `for` advances
through the array on its own; you never write `0..a.len()`, never
write `a[i]`, never write a counter.

Now the side-by-side contrast — the same task with the older
manual-indexing shape. Save `manual.rs`:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
}
```

`rustc manual.rs && ./manual` prints the same five lines. But
`manual.rs` needs `let mut index = 0;`, the condition `index <
a.len()`, the indexing `a[index]`, and the update `index = index +
1;`. `demo.rs` needs none of those. `manual.rs` also has bug surfaces
`demo.rs` cannot have: forget the update and the loop spins forever;
write the wrong bound and `a[index]` panics with lesson 078's runtime
message. The `for` shape removes both — there is no counter to forget
and no index to be wrong.

## What Changed

- New use of an old shape: `for X in COLLECTION { ... }` accepts an
  *array* in the COLLECTION slot, not just a range.
- The loop variable binds to each *element value* (not to an index).
  For `let a = [10, 20, 30, 40, 50];`, `element` takes the values
  `10`, `20`, `30`, `40`, `50`, in array order.
- No `mut`, no counter, no indexing. The `for` runs the body exactly
  `a.len()` times and exits. The `while index < a.len()` shape from
  lesson 017 is one alternative; the `for` shape is shorter and has
  no out-of-bounds failure mode.
- This is the form to reach for first when you want to visit every
  element of an array. The Book: "The safety and conciseness of `for`
  loops make them the most commonly used loop construct in Rust."

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let xs = [1, 2, 3];
    for x in xs {
        println!("x = {}", x);
    }
    println!("after");
}
```

You run `rustc tiny.rs && ./tiny`.

(a) How many lines does it print, in what order?

(b) On the second pass, what value is `x` bound to?

(c) Could this loop ever panic with `index out of bounds: the len is
3 but the index is M`? Why or why not?

*(Answers: (a) Four lines: `x = 1`, `x = 2`, `x = 3`, `after`. The
body runs once per element; `after` runs once after the loop exits.
(b) `2`. The loop binds `x` to each element in array order; the
second element of `[1, 2, 3]` is `2`. (c) No. The loop never builds
an index — it produces *element values* directly. There is no `xs[i]`
expression, so the bounds rule from lesson 078 has nothing to fire
on.)*

## What To Ignore For Now

Today installs only `for X in ARRAY { ... }` — same shape as lesson
022, just with an array in the collection slot. Real and deferred:

- *`.iter()`* — a method that exposes an array as an *iterator*.
  Today's bare `for element in a` works without it; separate move.
- *`for (i, v) in a.iter().enumerate()`* — the form that yields both
  the index and the element each pass.
- *The `Iterator` trait* — the standard-library machinery that makes
  `for ... in ...` work on more than ranges and arrays.
- *Iterating by reference vs by value* — `for x in &a` vs `for x in
  a`. Today's array of `i32` makes the distinction invisible because
  integers are cheap to copy; for other element types it would matter.
- *`for x in a.into_iter()`* — explicit by-value iteration.
- *Slice iteration* — `for x in &a[1..3]` or iterating over a `Vec`.
  Different types.
- *2024-edition array iteration semantic changes* — edition detail.
- *`while let Some(x) = iter.next()`* — the manual-pull form.
- *Iterator adapters* — `.map`, `.filter`, `.collect`, `.fold`.
- *`break` and `continue` inside `for element in a`* — already
  installed (lessons 027, 035) and compose with today; not new.

## Evidence

See `../evidence/079-for-over-array.md`.
