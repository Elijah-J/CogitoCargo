---
id: 051-ordering-enum-and-variant-match
status: accepted
evidence: ../evidence/051-ordering-enum-and-variant-match.md
---

# `match` on `Ordering` against its three variants

## The Move

Three closely-coupled facts install together. (1) The standard library
has a type at path `std::cmp::Ordering` whose values are *one of three
named alternatives*: `Less`, `Greater`, and `Equal`. A type whose
values come from a fixed list of named alternatives is called an
*enum*; each alternative is a *variant*. (2) To name one specific
variant value, write the qualified path `Ordering::Less` — same `::`
separator as `String::new()` from lesson 042, except the right-hand
side names a variant rather than a function. (3) Lesson 030's `match`
machine extends to this new scrutinee type unchanged: each arm is
`Variant => arm_value`, and rustc still enforces exhaustiveness with
**E0004**. Three variants, three arms.

```rust
use std::cmp::Ordering;

fn main() {
    let direction: Ordering = Ordering::Less;
    let label = match direction {
        Ordering::Less => "less",
        Ordering::Greater => "greater",
        Ordering::Equal => "equal",
    };
    println!("direction = {label}");
}
```

The `use` line (lesson 044) brings `Ordering` into scope. The
annotation `: Ordering` is lesson 019's shape with a new `TYPE`. The
`match` examines `direction`, selects the first arm whose pattern
equals it, and produces the arm value `"less"`, which `let label = ...;`
binds.

## Mental Model Delta

- Before: "I know `match` on `bool` (lesson 030) and on an integer
  with `_` (lesson 031). Both use *literal* patterns."
- After: "Some types have a fixed list of *named alternative* values.
  Such a type is an *enum*; each alternative is a *variant*.
  `Ordering` is the standard library's three-variant enum for
  comparison results. Variant names are reached by the qualified path
  `Ordering::Less` (lesson 042's `::` shape, variant on the right).
  `match` on an enum scrutinee uses those qualified names as patterns;
  exhaustiveness still applies, so all three variants must appear (or
  fire E0004)."

## Prerequisites

- Installed concepts:
  - Lesson 030 (load-bearing): the `match` form — scrutinee, arms
    `pattern => arm_expression`, comma-separated; matching arm wins;
    arms share a type; exhaustiveness via E0004. Reused unchanged.
  - Lesson 031: `match` already generalized from `bool` to `i32` with
    a `_` catch-all. Today's contrast: `Ordering` has exactly three
    values, so listing all three is finite and no `_` is needed.
  - Lesson 042 (load-bearing): the qualified path `Type::name` with
    `::` as separator. Today extends the right-hand side from a
    function name to a *variant* name; same separator.
  - Lesson 043: nested module paths like `std::cmp::min`. The path
    `std::cmp::Ordering` reuses the same shape with a type final
    segment.
  - Lesson 044: `use std::cmp::Path;` brings the final-segment name
    into scope. Same form, new payload (a type instead of a function).
  - Lesson 019 (load-bearing for the *shape*): `let name: TYPE = value;`.
    Extended to `: Ordering` here, same way 042 extended it to
    `: String`.
  - Lesson 003 (load-bearing): rustc diagnostic shape; the broken-
    contrast walk decodes E0004 with that map.
  - Lessons 001, 002, 005: `rustc file.rs` then `./name`; `fn main`;
    `let name = value;` plus `println!("{name}")`.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

In a fresh empty directory, create `demo.rs` with the source above.
Compile and run:

```console
$ rustc demo.rs
$ ./demo
direction = less
```

Walk it. Line 1's `use` (lesson 044) brings `Ordering` into scope.
`Ordering::Less` on line 4 is a *variant value* — a value of type
`Ordering` that specifically is the `Less` alternative. The `match`
(lesson 030) compares `direction` to each arm's pattern in source
order; `Ordering::Less` matches the first arm; its value `"less"`
becomes the value of the whole `match` and binds to `label`. (rustc
infers the type of `label`; see *What To Ignore For Now*.)

Now the contrast. Save `broken.rs` — the same source with the
`Ordering::Equal` arm removed:

```rust
use std::cmp::Ordering;

fn main() {
    let direction: Ordering = Ordering::Less;
    let _label = match direction {
        Ordering::Less => "less",
        Ordering::Greater => "greater",
    };
}
```

Compile it. The headline reads
`error[E0004]: non-exhaustive patterns: \`std::cmp::Ordering::Equal\` not covered`
— same E-code as 030/031, with the missing variant named by its full
path (rustc spells variants unambiguously, even though the source used
the shorter `Ordering::Equal`). The `-->` points at the scrutinee, the
`note:` reads `the matched value is of type \`std::cmp::Ordering\``,
and the `help:` block source-diffs
`std::cmp::Ordering::Equal => todo!(),` as a new arm. (Full transcript
in `## Evidence`.) The new fact: the missing-pattern name is a
*variant*, not a literal or a range. Exhaustiveness on an enum means
*every variant the type declares* must appear as a pattern (or be
matched by a wildcard).

## What Changed

- You can `match` a value whose type is an enum, with each variant as
  a pattern. For `Ordering`: `Ordering::Less`, `Ordering::Greater`,
  `Ordering::Equal`.
- New noun: an *enum* is a type whose values come from a fixed list of
  named alternatives, called *variants*. `Ordering`'s variants have
  no payload — each is just a name.
- `Type::Variant` is the same `::` separator as `Type::name(args)`
  from lesson 042; what differs is the right-hand side. The path
  produces a value of type `Type`, bindable by `let` and usable as a
  pattern in `match`.
- `use std::cmp::Ordering;` (lesson 044's shape) lets the rest of the
  file write `Ordering` and `Ordering::Less` instead of the full
  `std::cmp::Ordering::Less`.
- Exhaustiveness on an enum scrutinee means *every* variant must be
  covered. Forgetting one fires E0004; the headline names the missing
  variant by its qualified path.

## Check Yourself

You write `tiny.rs`:

```rust
use std::cmp::Ordering;

fn main() {
    let direction: Ordering = Ordering::Equal;
    let label = match direction {
        Ordering::Less => "less",
        Ordering::Equal => "equal",
        Ordering::Greater => "greater",
    };
    println!("{label}");
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) The arms are reordered (`Equal` in the middle). Does the order
change which arm matches?

(c) If you delete the `Ordering::Less => "less",` arm and recompile,
which E-code does rustc emit, and which variant does the headline name
as not covered?

(Answers: (a) Yes; prints `equal`. (b) No: each `Ordering` value
matches exactly one of the three variants, so reordering
non-overlapping variant patterns does not change the result.
(c) `error[E0004]: non-exhaustive patterns: \`std::cmp::Ordering::Less\` not covered`
— same E-code as 030/031, with `Less` as the named-missing variant.)

## What To Ignore For Now

- *Defining your own enum* with the `enum` keyword
  (`enum Color { Red, Green, Blue }`). Today uses a standard-library
  enum.
- *Variants with payload* — `Some(T)`, `None`, `Ok(T)`, `Err(E)`,
  `Foo(i32)`. `Ordering`'s three variants are *unit-like* (no
  payload). Payloads change pattern syntax (`Some(n) => ...`).
- *Methods on enums* — `Ordering::is_eq()`, `.is_lt()`, `.reverse()`,
  `.then(...)` are documented on the `Ordering` page; not used today.
- *The `Ord` and `PartialOrd` traits*; *generic enums* like
  `Option<T>` and `Result<T, E>` (`Ordering` is *not* generic); *the
  `cmp(&other)` method on `i32`* (would replace the hardcoded
  `Ordering::Less`, but pulls in `Ord` and reference semantics).
- *Match guards*, *`if let` / `while let`* — future moves from 030's
  unlocks.
- *`use std::cmp::Ordering::*;`* (glob form importing all variants);
  *`#[derive(...)]`*; *discriminants* (the `= -1, 0, 1` numbers on the
  `Ordering` page); *memory layout*; *niche optimization*.
- *`&str`* — the inferred type of `label`. String literals have type
  `&str`; the lesson does not annotate or name it.
- All previously deferred items.

## Evidence

See `../evidence/051-ordering-enum-and-variant-match.md` for the
corpus-quote map, the rustc / system toolchain string, the working
probe transcript, the broken-contrast E0004 transcript, and the
prerequisite-claim summary.
