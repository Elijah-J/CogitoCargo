---
id: 073-let-tuple-destructure
status: accepted
evidence: ../evidence/073-let-tuple-destructure.md
---

# Pattern destructure a tuple on the LEFT of `let`

## The Move

The left side of `let` does not have to be a single bare name. It
can be a *tuple pattern* — a parenthesized comma-separated list of
names that mirrors the tuple's parens-and-commas shape, written
where the binding name would go:

```rust
let pair = (3, 7);
let (a, b) = pair;
```

After the second line, both `a` (= 3) and `b` (= 7) are bindings in
scope, just as if you had written `let a = pair.0; let b = pair.1;`.
The same shape works directly on a tuple expression: `let (a, b) =
(3, 7);` is one line that constructs the 2-tuple and immediately
splits it. The name count on the left must equal the field count on
the right; a mismatch fails at compile time with
`error[E0308]: mismatched types`.

## Mental Model Delta

- Before: "After lesson 072 I can put a tuple value on the right of
  `let` and read each field with `.0`, `.1`. The left of `let` is
  always a single name; for two names I write two `let` statements."
- After: "The left of `let` is a *pattern*, not just a name. A bare
  name is the simplest pattern (it binds the whole value). A *tuple
  pattern* `(a, b)` is another shape: it splits a tuple into one
  binding per field. The pattern's shape mirrors the tuple's. Other
  pattern shapes exist; today only installs this one."

## Prerequisites

The word *pattern* is new as a load-bearing term. The Reference uses
it for both the right of `=>` in `match` arms (lessons 030, 031, 058
used patterns there informally) and the left of `let`.

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs` then `./name`; `fn main` body
    runs when the executable launches; rustc silent on success.
  - Lesson 003 (load-bearing): the four-part diagnostic map
    (headline, `-->`, source excerpt with caret, optional
    `help:` / `= note:`). The contrast probe is read with that map.
  - Lesson 005 (load-bearing): `let name = value;`. Today extends
    the *left* from a single bare name to a tuple pattern. Lesson
    005's *What To Ignore* explicitly named this as a deferred
    future lesson; today closes that loop.
  - Lesson 072 (load-bearing): tuple types `(T1, T2, ...)`, tuple
    expressions, the first field is `0`. Today reuses these without
    extending them; lesson 072's *unlocks* names this move as the
    explicit next cycle (deferred-queue Q06).
  - Lessons 011, 033 (cited): positional `{}` printing and the `2.5`
    `f64` literal form. Used in the probe; no new behavior installed.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
fn main() {
    let pair = (3, 7);
    let (a, b) = pair;
    println!("a = {}", a);
    println!("b = {}", b);

    let (x, y, z) = (10, 20, 30);
    println!("x y z = {} {} {}", x, y, z);

    let (m, n) = (5, 2.5);
    println!("m = {}, n = {}", m, n);
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
a = 3
b = 7
x y z = 10 20 30
m = 5, n = 2.5
```

Three uses of the new shape sit in `main`. The first builds `pair`
the lesson-072 way, then a second `let` whose left side is the tuple
pattern `(a, b)` introduces two bindings — `a` from field `0`, `b`
from field `1`. The second, `let (x, y, z) = (10, 20, 30);`, is the
construct-and-destructure form: one `let` both creates a 3-tuple
and splits it. The third, `let (m, n) = (5, 2.5);`, reuses the
heterogeneous case from lesson 072 — the pattern does not care that
the fields have different types; it just hands each field to the
matching name.

Now the contrast. In the same directory, save `broken.rs`:

```rust
fn main() {
    let (a, b, c) = (3, 7);
    println!("{} {} {}", a, b, c);
}
```

Compile it. Read the headline with the lesson 003 map; the full
transcript is in `## Evidence`:

```text
error[E0308]: mismatched types
 --> broken.rs:2:9
  |
2 |     let (a, b, c) = (3, 7);
  |         ^^^^^^^^^   ------ this expression has type `({integer}, {integer})`
  |         |
  |         expected a tuple with 2 elements, found one with 3 elements
```

The caret underlines the *pattern* on the left; the inline annotation
says "expected a tuple with 2 elements, found one with 3 elements."
rustc reads the pattern as claiming a 3-field tuple and the value as
having 2 fields, and the two cannot match. The contrast is the
lesson's "with X works, without X fails" witness: when the pattern's
shape matches the tuple's, the program builds; when the counts
disagree, rustc rejects at compile time.

## What Changed

- The LEFT of `let` is a *pattern*, not just a name. A bare name is
  the simplest pattern; today's new shape is a *tuple pattern* —
  parenthesized, comma-separated names that mirror a tuple value
  and produce one binding per field.
- One `let` with a tuple pattern replaces a chain of single-name
  `let`s and `.N` accesses: `let (a, b) = pair;` instead of
  `let a = pair.0; let b = pair.1;`.
- The shape works on a previously-bound tuple and directly on a
  tuple expression — the right side is just an expression.
- The pattern's name count must equal the tuple's field count. A
  mismatch fails at compile time with `error[E0308]: mismatched
  types` and the inline annotation `expected a tuple with N
  elements, found one with M elements`.
- Other pattern shapes exist (the Reference's grammar lists
  identifier, literal, wildcard, and others); today installs only
  the tuple pattern and only on the left of `let`.

## Check Yourself

You write `tiny.rs` containing:

```rust
fn main() {
    let triple = (1, 2, 3);
    let (a, b, c) = triple;
    println!("{} {} {}", a, b, c);
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) Now change the second `let` to `let (a, b) = triple;`. Predict
the rustc headline and the inline annotation under the caret without
running anything.

(c) Why is part (b)'s problem caught by `rustc` rather than by the
running program?

(Answers: (a) Yes; prints `1 2 3`. (b) `error[E0308]: mismatched
types`, caret under the pattern `(a, b)`, inline annotation
`expected a tuple with 3 elements, found one with 2 elements` —
the lesson's contrast was 3-against-2, today's is 2-against-3; the
wording shape is the same in either direction. (c) The tuple type
carries a fixed length (lesson 072) and the pattern carries a fixed
name count, so rustc compares the two integers at compile time and
rejects the program before producing an executable.)

## What To Ignore For Now

Today installs only one pattern shape — *tuple pattern* — and only on
the left of `let`. Each of the following is real and deferred:

- *Tuple patterns in `match` arms* — `match pair { (0, _) => ... }`.
  A separate move from tuple patterns on the left of `let`.
- *The wildcard `_` inside a tuple pattern* — `let (a, _) = pair;`
  to ignore one field. Future move; today every name binds.
- *The rest pattern `..` inside a tuple pattern* — `let (a, ..) =
  triple;` to ignore the rest. Future move.
- *Refutable vs. irrefutable patterns* — the Reference calls today's
  shape *irrefutable*. Today is operational only; do not install
  the term.
- *Destructuring of structs or enums* — `let Point { x, y } =
  point;`, `let Some(n) = maybe_n;`. The Reference's *Destructuring*
  section names structs, enums, and tuples together, but each gets
  its own lesson.
- *Nested tuple patterns* — `let ((a, b), c) = nested;`. The
  parens-and-commas rule recurses; its own move.
- *Type annotation on the destructured pattern* — `let (a, b):
  (i32, i32) = pair;`. Allowed by the grammar; not installed today.
- *`mut` inside a tuple pattern* — `let (mut a, b) = pair;` to make
  one binding reassignable. Future move; today's bindings are
  immutable as in lesson 005.

## Evidence

See `../evidence/073-let-tuple-destructure.md`.
