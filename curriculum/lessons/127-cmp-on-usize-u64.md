---
id: 127-cmp-on-usize-u64
status: accepted
evidence: ../evidence/127-cmp-on-usize-u64.md
---

# Compare two `usize` values or two `u64` values with `a.cmp(&b)`

## The Move

Lesson 061 installed `a.cmp(&b)` on two `i32` values. Today extends
the same method-name to two more typed receivers from lesson 080's
twelve-name family — `usize` and `u64`. Same shape, same return
type, only the operand type changes.

```rust
use std::cmp::Ordering;

fn main() {
    let a: u64 = 100;
    let b: u64 = 200;
    match a.cmp(&b) {
        Ordering::Less => println!("u64: a < b"),
        Ordering::Greater => println!("u64: a > b"),
        Ordering::Equal => println!("u64: a == b"),
    }

    let c: usize = 5;
    let d: usize = 5;
    match c.cmp(&d) {
        Ordering::Less => println!("usize: c < d"),
        Ordering::Greater => println!("usize: c > d"),
        Ordering::Equal => println!("usize: c == d"),
    }
}
```

`rustc demo.rs` is silent (exit 0). `./demo` prints two lines:

```text
u64: a < b
usize: c == d
```

Two probes in one program. The first match scrutinee is
`a.cmp(&b)` with both operands `u64`; `100 < 200` reaches the
`Less` arm. The second is `c.cmp(&d)` with both operands `usize`;
`5 == 5` reaches the `Equal` arm.

The std `Ord` trait page declares one signature, `fn cmp(&self,
other: &Self) -> Ordering` (trait.Ord.md:9). Every implementing
type reuses that signature with `Self` replaced by itself. The same
page lists `impl Ord for u64` and `impl Ord for usize` alongside
`impl Ord for i32` (lines 467, 479). The per-primitive pages
restate the specialized signatures: `fn cmp(&self, other: &u64) ->
Ordering` (primitive.u64.md:4235) and `fn cmp(&self, other: &usize)
-> Ordering` (primitive.usize.md:4542). Same call shape, same
return type as lesson 061; only the operand type differs.

## Mental Model Delta

- Before: "`.cmp(&other)` works on `i32` and returns `Ordering` —
  lesson 061 showed me. I have not used it on any other type."
- After: "`.cmp(&other)` is the same method-name on `usize` and on
  `u64`. The signature is `fn cmp(&self, other: &Self) -> Ordering`
  for every type that has it, so the operand types must match —
  `u64.cmp(&u64)` works, `usize.cmp(&usize)` works, but
  `u64.cmp(&i32)` fires E0308 *mismatched types* with `expected
  `&u64`, found `&i32``. The operands must be the same type."

## Prerequisites

- Installed concepts:
  - Lesson 061 (load-bearing): `a.cmp(&b)` on `i32` returning
    `Ordering`, with `use std::cmp::Ordering;` and the three-arm
    match. Today extends the same method to `usize` and `u64`; the
    call shape and the match are unchanged.
  - Lesson 080 (load-bearing): names `usize` and `u64` as members
    of the integer-type family, plus the `: TYPE` annotation slot.
  - Lesson 051 (cited): the `Ordering` enum and the three-variant
    match.
  - Lessons 040, 044, 045 (cited): dot-call shape, `use std::cmp::Ordering;`
    bringing the variant names into scope, and the prefix-`&` operator
    that builds the `&b` / `&d` arguments.
  - Lessons 001, 002, 005, 011, 003, 019: `rustc demo.rs` then
    `./demo`, `fn main`, `let name: TYPE = value;`,
    `println!("{}", x)`, the diagnostic four-part map.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, Linux/macOS shell.

## Try It

Save the snippet above as `demo.rs` in a fresh directory.

```console
$ rustc demo.rs
$ ./demo
u64: a < b
usize: c == d
```

Now the contrast. Predict, do not run yet. Save `broken.rs`:

```rust
fn main() {
    let a: u64 = 100;
    let b: i32 = 5;
    let _ = a.cmp(&b);
}
```

The receiver `a` is `u64`. The argument `&b` is `&i32`. Lesson
061's signature says `cmp` takes `other: &Self`, where `Self` is
the receiver's type — here `u64`. So the argument slot expects
`&u64` and gets `&i32`.

```text
error[E0308]: mismatched types
 --> broken.rs:4:19
  |
4 |     let _ = a.cmp(&b);
  |               --- ^^ expected `&u64`, found `&i32`
  |               |
  |               arguments to this method are incorrect
  |
  = note: expected reference `&u64`
             found reference `&i32`
```

Same E-code (E0308) and same wording shape lesson 061 captured for
the missing-`&` contrast on `i32`, only the type pair changes.
rustc itself spells the rule: the argument's reference type must
match `&Self`. The `note: expected reference `&u64` / found
reference `&i32`` makes both sides explicit.

(Full transcripts are in `../evidence/127-cmp-on-usize-u64.md`.)

## What Changed

- You can compare two `u64` values three-way with `a.cmp(&b)`, and
  two `usize` values with `c.cmp(&d)`. Both calls return
  `Ordering`; both feed the lesson-051 three-arm match unchanged.
- One new fact about an old method: `.cmp` is reachable on `usize`
  and on `u64`, with the same `(&self, other: &Self) -> Ordering`
  signature lesson 061 installed for `i32`.
- `Self` in that signature is the receiver's type. The argument
  must have type `&Self`, so both operands must match —
  `u64.cmp(&u64)`, not `u64.cmp(&i32)`.
- Failure mode for cross-type `.cmp`: E0308 *mismatched types*
  with `expected `&u64`, found `&i32`` (or whatever pair you
  mixed). Same E-code as lesson 061's missing-`&` contrast;
  what changed is the type pair on the inline label.

## Check Yourself

You write `pred.rs`:

```rust
use std::cmp::Ordering;

fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![10, 20];
    match v.len().cmp(&w.len()) {
        Ordering::Less => println!("v shorter"),
        Ordering::Greater => println!("v longer"),
        Ordering::Equal => println!("same length"),
    }
}
```

(a) Lesson 107 says `Vec::len()` returns `usize`. Which lesson
covers the `.cmp(&...)` call here, and what type are both
operands?

(b) Does `rustc pred.rs` accept the program? What does `./pred`
print?

(c) If you replaced `&w.len()` with `w.len()` (drop the `&`),
which E-code would the headline carry?

*(Answers: (a) Today's lesson — both operands are `usize`,
because `v.len()` and `w.len()` each return `usize`. (b) Yes;
`v.len() == 3` and `w.len() == 2`, so `3.cmp(&2)` reaches the
`Greater` arm and prints `v longer`. (c) E0308 *mismatched
types* — `cmp` expects `&Self` (here `&usize`) and got bare
`usize`. Same E-code lesson 061 captured for the missing-`&`
case on `i32`.)*

## What To Ignore For Now

- *The `Ord` trait declaration itself.* Trait machinery
  (supertraits `Eq + PartialOrd`, provided methods `max` / `min` /
  `clamp`, custom `impl Ord for ...`) stays name-deferred as in
  lesson 061.
- *`PartialOrd` and `partial_cmp` returning `Option<Ordering>`.*
  For types like `f64` whose values include `NaN`, comparison can
  be undefined. `usize` and `u64` form a total order, so today's
  `cmp` always returns a real `Ordering`.
- *`.cmp` on the other ten integer types* (`i8`, `u8`, `i16`,
  `u16`, `i64`, `i128`, `u128`, `isize`) — all listed as `Ord`
  implementors; mechanic carries, but only `usize` and `u64` are
  exercised here.
- *`.cmp` on `f32` / `f64`* — NOT `Ord`; only `PartialOrd`. Deferred.
- *`.cmp` on `&str`, `String`, `Vec<T>`, slices, tuples,
  user-defined structs / enums* — each is its own move.
- *Trait-method dispatch* — the rule by which `.cmp` is reachable
  on every implementing type, name-deferred since lesson 040.
- All previously deferred items.

## Evidence

See `../evidence/127-cmp-on-usize-u64.md` for the corpus-quote
map, the toolchain string, the working probe transcript, the
cross-type contrast E0308 transcript, the corroborating
`Vec::len().cmp(&...)` transcript, and the prerequisite-claim
summary.
