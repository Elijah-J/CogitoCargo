# Evidence — Lesson 126: tuple pattern at the for-loop binding slot

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/126-for-pattern-destructuring.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/126-for-pattern-destructuring.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/126-for-pattern-destructuring.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into a fresh scratch directory (`/tmp/eduratchet126/`)
and compiled with `rustc <file>`; resulting executables were run from
the same directory. Same host and toolchain as accepted lessons
123-125.

## Direct prerequisite — lesson 073 (tuple pattern on left of `let`)

Lesson 073 installed:

- The LEFT of `let` is a *pattern*, not just a name. A tuple
  pattern `(a, b)` parenthesized-and-comma-separated mirrors a
  tuple value's shape and produces one binding per field.
- The pattern's name count must equal the tuple's field count;
  mismatches fire `error[E0308]: mismatched types` with the inline
  annotation "expected a tuple with N elements, found one with M
  elements".

Today's lesson moves that exact pattern shape — `(a, b)` — from the
left of `let` to the for-loop's binding slot. The pattern's
mechanics (one binding per field, count must match the value) are
unchanged. Probe 6 in this appendix witnesses the count-mismatch
diagnostic at the for-binding slot and confirms it is structurally
the same as lesson 073's contrast.

## Direct prerequisite — lesson 079 (`for X in COLLECTION { ... }`)

Lesson 079 installed:

- `for X in COLLECTION { ... }`: the body runs once per element of
  COLLECTION, with the loop variable `X` bound to that element.
- The slot `X` was filled by a single bare identifier (`element`).

Today extends the `X` slot from a single identifier to a tuple
pattern. The per-pass / one-binding-per-element semantics are
unchanged; only the shape of the binding slot grows. The Reference
already specified that this slot is a pattern (loop-expr.md:202);
lesson 079 used the simplest pattern shape (a bare identifier),
and today uses the next-simplest (a tuple pattern).

## Direct prerequisite — lesson 125 (`v.iter().zip(w.iter())` yields tuples)

Lesson 125 installed:

- `v.iter().zip(w.iter())` is an iterator that yields 2-tuples
  paired position-wise from two `Vec<u64>` sources.
- Each yielded tuple has type `(&u64, &u64)` (Probe 5 of lesson 125
  witnessed this with `let _: &u64 = pair.0;` and `let _: &u64 =
  pair.1;`).
- The for-loop bound the whole tuple to a single name `pair`, and
  the parts were read with `pair.0` and `pair.1`.

Today's working probe consumes the same iterator chain. The
EXPRESSION slot of the for-loop is unchanged; only the PATTERN
slot changes from `pair` to `(a, b)`. Probe 4 in this appendix
runs both forms side by side and `diff`s the output to confirm
they are byte-identical — empirical witness that today's move is
just lesson 073's pattern shape applied at lesson 079's binding
slot, consuming lesson 125's iterator output.

## Older supporting lessons

- **Lesson 072** — tuple type `(A, B)` and indexing `.0` / `.1`.
  Used by Probe 4's old-form program; not used by the new-form
  centered probe.
- **Lesson 123** — `v.iter()` yields `&T` (here `&u64`). Probe 3's
  contrast applies a 2-tuple pattern to the `v.iter()` iterator
  directly — which yields `&u64`, not a tuple — and rustc fires
  E0308 because the pattern shape does not match the yielded type.
- **Lessons 040, 011, 001, 002, 003, 005** — same roles as in
  lesson 125: dot-call grammar (used by `.iter()` and `.zip()`);
  `println!`; `rustc`; `fn main`; the diagnostic four-part map;
  `let`.
- **Lessons 080, 019, 107** — `u64`; `: TYPE` annotation; `Vec<T>`
  with `vec![]`.
- **Lesson 049** — chained dot-calls `expr.method1().method2(arg)`
  used in `v.iter().zip(w.iter())`. Inherited from lesson 125.

## Probe 1 — working probe (`for (a, b) in v.iter().zip(w.iter())`)

Source committed at
`experimental/eduratchet2/runs/rust-moves/observations/126-for-pattern-destructuring.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    for (a, b) in v.iter().zip(w.iter()) {
        println!("{} / {}", a, b);
    }
}
```

Transcript:

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
10 / 100
20 / 200
30 / 300
$ echo "run-exit=$?"
run-exit=0
```

The centered claim — "the for-loop's binding slot accepts a tuple
pattern that destructures each yielded tuple into named parts" —
is carried by the silent compile and the three printed lines. The
pattern `(a, b)` matches against each yielded `(&u64, &u64)`; `a`
binds the first part, `b` the second. Inside the loop body, `a`
and `b` are used directly — no `.0` / `.1` indexing.

## Probe 2 — centered diagnostic contrast (`pair` not bound)

`broken_pair.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    for (a, b) in v.iter().zip(w.iter()) {
        println!("{:?}", pair);
    }
}
```

Transcript verbatim:

```text
$ rustc broken_pair.rs
error[E0425]: cannot find value `pair` in this scope
 --> broken_pair.rs:5:26
  |
5 |         println!("{:?}", pair);
  |                          ^^^^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
exit=1
```

This is the lesson's *centered* contrast. The destructuring pattern
`(a, b)` introduces only `a` and `b` as bindings; the original
tuple value has no name in the loop body. Lesson 125's `for pair
in iter` shape *did* bind `pair`; today's `for (a, b) in iter`
does not. E0425 is "An unresolved name was used"
(`error_codes/E0425.md:4`); rustc looks up `pair` in the loop
body's scope and finds nothing.

The lesson body centers this contrast because it isolates the
exact mental-model delta from lesson 125: "the whole tuple is no
longer named." A learner used to lesson 125's `pair.0` / `pair.1`
form might naively expect `pair` to still be in scope; the
diagnostic is the empirical correction.

## Probe 3 — alternative contrast (pattern-vs-element-type mismatch)

`broken_type.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    for (a, b) in v.iter() {
        println!("{} / {}", a, b);
    }
}
```

Transcript verbatim:

```text
$ rustc broken_type.rs
error[E0308]: mismatched types
 --> broken_type.rs:3:9
  |
3 |     for (a, b) in v.iter() {
  |         ^^^^^^    -------- this is an iterator with items of type `&u64`
  |         |
  |         expected `u64`, found `(_, _)`
  |
  = note: expected type `u64`
            found tuple `(_, _)`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

A 2-tuple pattern is matched against the iterator from `v.iter()`
directly, which yields `&u64` (lesson 123) — not a tuple. rustc
fires `mismatched types`: the iterator's items are `&u64`, the
pattern claims a 2-tuple shape, the two cannot match.

Note rustc reports the type as `u64` (after auto-dereferencing the
`&u64` for pattern-matching purposes) rather than `&u64`; this is
default-binding-mode behavior (Reference patterns.md:345 *Binding
modes*, name-deferred). The load-bearing fact for today is
unchanged: the pattern shape must match the yielded value type, or
rustc rejects.

This contrast is captured as a corroborator alongside Probe 2.
Probe 2 isolates the "whole tuple has no name" claim; Probe 3
isolates the "pattern shape must match the yielded type" claim.
Both are claims the lesson body makes; both have verbatim
diagnostics.

## Probe 4 — equivalence corroborator (old `pair.0/.1` vs new `(a, b)`)

`old_form.rs` (lesson 125's working probe verbatim):

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    for pair in v.iter().zip(w.iter()) {
        println!("{} / {}", pair.0, pair.1);
    }
}
```

`new_form.rs` (today's working probe verbatim):

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    for (a, b) in v.iter().zip(w.iter()) {
        println!("{} / {}", a, b);
    }
}
```

Transcript verbatim:

```text
$ rustc old_form.rs && rustc new_form.rs
$ ./old_form > old.out
$ ./new_form > new.out
$ diff old.out new.out
$ echo "diff-exit=$?"
diff-exit=0
$ cat old.out
10 / 100
20 / 200
30 / 300
```

`diff-exit=0` — both programs produce byte-identical output. The
only difference between the two programs is the for-binding shape
(`pair` vs `(a, b)`) and how the parts are read in the body
(`pair.0`, `pair.1` vs `a`, `b`). Empirical witness for the
lesson's positive claim: today's move is just lesson 073's tuple
destructuring applied at the for-loop binding slot. The mechanic
is unchanged; only the binding shape differs.

## Probe 5 — type-witness (`a` and `b` are each `&u64`)

`typecheck.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    for (a, b) in v.iter().zip(w.iter()) {
        let _: &u64 = a;
        let _: &u64 = b;
        println!("{} / {}", a, b);
    }
}
```

Transcript:

```text
$ rustc typecheck.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./typecheck
10 / 100
20 / 200
30 / 300
$ echo "run-exit=$?"
run-exit=0
```

The annotations `let _: &u64 = a;` and `let _: &u64 = b;` are
accepted by rustc. Empirical witness that the pattern parts are
bound at type `&u64` — the same per-element type lesson 125's
Probe 5 named via `pair.0: &u64` / `pair.1: &u64`. Today's
destructure puts the type on the bound names directly without an
indexing step.

## Probe 6 — count-mismatch contrast (3-tuple pattern, 2-tuple value)

`triple_pattern.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    for (a, b, c) in v.iter().zip(w.iter()) {
        println!("{} / {} / {}", a, b, c);
    }
}
```

Transcript verbatim:

```text
$ rustc triple_pattern.rs
error[E0308]: mismatched types
 --> triple_pattern.rs:4:9
  |
4 |     for (a, b, c) in v.iter().zip(w.iter()) {
  |         ^^^^^^^^^    ---------------------- this is an iterator with items of type `(&u64, &u64)`
  |         |
  |         expected a tuple with 2 elements, found one with 3 elements
  |
  = note: expected tuple `(&u64, &u64)`
             found tuple `(_, _, _)`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

The pattern's name count must match the tuple's field count.
Mirrors lesson 073's count-mismatch contrast `let (a, b, c) =
(3, 7);` — same E-code, same "expected a tuple with N elements,
found one with M elements" inline annotation, applied at the
for-binding slot rather than the let-binding slot.

This grounds the Check Yourself part (b) answer: a 3-tuple
pattern against a 2-tuple iterator fires E0308 with the inline
annotation "expected a tuple with 2 elements, found one with 3
elements" — verbatim what the question expects.

## Why this works — Reference grounding

### `output/docs/rust/reference/expressions/loop-expr.md` lines 195-212

Section title (line 195):

> ## [Iterator loops](#iterator-loops)

Syntax (lines 199-202):

> **Syntax**
> [IteratorLoopExpression] →
>     for [Pattern](../patterns.md#grammar-Pattern) in [Expression](../expressions.md#grammar-Expression)except [StructExpression](struct-expr.md#grammar-StructExpression) [BlockExpression](block-expr.md#grammar-BlockExpression)

Intro (line 208):

> A `for` expression is a syntactic construct for looping over
> elements provided by an implementation of `std::iter::IntoIterator`.

Condition (line 212):

> If the iterator yields a value, that value is matched against the
> irrefutable pattern, the body of the loop is executed, and then
> control returns to the head of the `for` loop. If the iterator is
> empty, the `for` expression completes.

Three load-bearing facts mapped to the lesson body:

- *Pattern slot* "for [Pattern] in [Expression]": the binding slot
  is grammatically a *Pattern*, not just an identifier. The lesson
  body cites this directly ("`for PATTERN in EXPRESSION { ... }`,
  loop-expr.md:202").
- *Match against the pattern*: each yielded value is matched
  against the pattern; bindings inside the pattern are introduced
  for use in the loop body. The lesson body's "pattern matches
  against the yielded tuple; `a` is bound to the first part, `b`
  to the second" reads from this rule.
- *Irrefutable*: the pattern must always match. Refutable patterns
  (`Some(x)`, literals, etc.) are rejected at the for-binding slot.
  The lesson body name-defers the term but states the operational
  rule in *What To Ignore For Now*.

### `output/docs/rust/reference/patterns.md` lines 71-97 and 1028-1066

Lines 73-97 (Patterns are used in):

> Patterns are used in:
>
> - [`let` declarations]
> - [Function] and [closure] parameters
> - [`match` expressions]
> - [`if let` expressions]
> - [`while let` expressions]
> - [`for` expressions]

Cited for the lesson's "the for-loop's binding slot is a *pattern*,
just like the left of `let`" generalization. The Reference lists
six contexts where patterns appear; lesson 073 used the first
(`let` declarations); today uses the sixth (`for` expressions).

Lines 1043-1064 (Tuple patterns):

> Tuple patterns match tuple values that match all criteria defined
> by its subpatterns. They are also used to [destructure] a tuple.
>
> ```rust
> let pair = (10, "ten");
> let (a, b) = pair;
>
> assert_eq!(a, 10);
> assert_eq!(b, "ten");
> ```

The grammar and example are the *same* shape as today's lesson —
just bound at the let-binding rather than for-binding slot. The
reference's tuple-pattern Reference example is what lesson 073
already cited; today extends its applicability domain.

Lines 137-146 (Refutability):

> A pattern is said to be *refutable* when it has the possibility
> of not being matched by the value it is being matched against.
> *Irrefutable* patterns, on the other hand, always match the
> value they are being matched against. Examples:
>
> ```rust
> let (x, y) = (1, 2);  // "(x, y)" is an irrefutable pattern
> ```

Cited for the lesson's *What To Ignore For Now* "irrefutable"
deferral. The tuple pattern `(a, b)` is irrefutable when its
subpatterns are (here both are bare identifiers, which are
irrefutable). The Reference name is name-deferred today; the
operational rule "the pattern must match unconditionally" is what
the lesson teaches.

### `output/docs/rust/book/ch19-01-all-the-places-for-patterns.md` lines 235-252

Section title and intro (lines 235-241):

> ### [`for` Loops](#for-loops)
>
> In a `for` loop, the value that directly follows the keyword
> `for` is a pattern. For example, in `for x in y`, the `x` is
> the pattern. Listing 19-5 demonstrates how to use a pattern in
> a `for` loop to destructure, or break apart, a tuple as part of
> the `for` loop.

Listing 19-5 (lines 242-249):

> ```rust
> fn main() {
>     let v = vec!['a', 'b', 'c'];
>
>     for (index, value) in v.iter().enumerate() {
>         println!("{value} is at index {index}");
>     }
> }
> ```

Load-bearing for the lesson's headline claim. The Book's prose "in
`for x in y`, the `x` is the pattern" is the audience-level
reading of the Reference grammar. Listing 19-5 is the same shape
today's working probe uses — `for (a, b) in iter` where the
iterator yields 2-tuples — applied to `.enumerate()` rather than
`.zip()`, but pattern-mechanically identical.

The lesson body cites this directly. Probe 1 reuses the Listing
19-5 shape with `.zip()` from lesson 125 in place of `.enumerate()`.

### `output/docs/rust/error_codes/E0425.md` line 4

> An unresolved name was used.

Probe 2's diagnostic is exactly this: the name `pair` is referenced
in the loop body but never bound — the destructuring pattern
introduced `a` and `b` instead. rustc looks up `pair` in scope and
reports E0425.

## rmp unlock — `cmp.rs:22` `for (left, right) in ...` end-to-end

Source `output/repos/rmp/src/biguint/cmp.rs` line 22 verbatim:

```rust
            for (left, right) in self.limbs.iter().rev().zip(other.limbs.iter().rev()) {
```

Decomposition by accepted lesson:

- `self.limbs` — field access on the `Vec<u64>` field `limbs` of
  the `&BigUInt` receiver (lesson 095, struct fields).
- `.iter()` — installs in lesson 123 as `Vec<T>::iter()` returning
  an iterator yielding `&T`.
- `.rev()` — installs in lesson 124 as the iterator-reversal
  adapter chained on `.iter()`.
- `.zip(...)` — installs in lesson 125 as the iterator-pairing
  adapter consuming a second iterator argument.
- `other.limbs.iter().rev()` — same chain as the receiver's, on a
  different `BigUInt`.
- `for (left, right) in ...` — *today's move*. The for-loop's
  PATTERN slot holds a tuple pattern that destructures each
  yielded `(&u64, &u64)` tuple into `left` and `right` for use in
  the loop body (which calls `left.cmp(right)` — lesson 117's
  `Ord::cmp` on the references).

Lessons 123 + 124 + 125 + 126 together make the entire `for ... in
... { ... }` head readable. The match arm body inside (using
`Ordering::Equal`, `return ord`, etc.) is composed of separately
installed concepts; only the for-loop head needed the iterator
chain plus today's destructuring.

## Claim-to-evidence map

- "The for-loop's binding slot accepts a *pattern*, not just a
  single identifier" — `loop-expr.md:202` syntax `for [Pattern] in
  [Expression]`; `loop-expr.md:212` "matched against the
  irrefutable pattern". Probe 1 silent compile is the empirical
  witness on a tuple pattern; Probe 2's E0425 indirectly witnesses
  by showing the destructuring binding slot replaces the bare-name
  binding slot.
- "A tuple pattern `(a, b)` in the for-binding slot destructures
  each yielded tuple into named parts" — `patterns.md:1043-1064`
  Tuple patterns; `book ch19-01:237-249` Book's "for Loops"
  section and Listing 19-5. Probe 1 silent compile and three
  printed lines (`10 / 100`, `20 / 200`, `30 / 300`) are the
  positive empirical witness; Probe 5's type witness `let _: &u64
  = a;` confirms the parts are bound at type `&u64`.
- "Old form (`pair.0`, `pair.1`) and new form (`(a, b)`) produce
  identical output" — Probe 4 transcript: `diff old.out new.out`
  exits 0; both programs compile silently and print the same three
  lines.
- "After destructuring, the whole tuple has no name" — Probe 2
  transcript verbatim (`error[E0425]: cannot find value \`pair\` in
  this scope`); `error_codes/E0425.md:4` "An unresolved name was
  used".
- "The pattern shape must match the yielded value's type" — Probe
  3 transcript verbatim (`error[E0308]: mismatched types`,
  `expected \`u64\`, found \`(_, _)\``); Probe 6 transcript verbatim
  (E0308 with "expected a tuple with 2 elements, found one with 3
  elements").
- "rmp `cmp.rs:22` `for (left, right) in ...` end-to-end readable"
  — `output/repos/rmp/src/biguint/cmp.rs:22` verbatim; the chain's
  PATTERN slot matches Probe 1's tuple-pattern shape with `(a, b)`
  swapped to `(left, right)`; the EXPRESSION slot is lesson 125's
  iterator chain composed with lesson 124's `.rev()`.

## Negative / contrast probe coverage

Two contrasts captured. Both are needed:

- **Probe 2 (E0425 on `pair`)** is the structural-boundary
  contrast. The destructuring pattern *replaces* the single-name
  binding; the original tuple value has no name in the loop body.
  Probe 2 has a verbatim diagnostic that names this directly
  ("cannot find value `pair` in this scope"). The lesson body
  centers this contrast because it isolates the exact mental-model
  delta from lesson 125.
- **Probe 3 (E0308 on type mismatch)** is the *type-shape* contrast.
  The pattern's shape must match the yielded value's type; a
  2-tuple pattern against `&u64` rejects. The lesson body's *What
  Changed* mentions this contrast; the appendix has the verbatim
  transcript.
- **Probe 6 (E0308 on count mismatch)** corroborates Probe 3 with
  the lesson-073 mirror — same E-code, same "expected a tuple with
  N elements, found one with M elements" inline annotation,
  applied at the for-binding slot. Used in the Check Yourself
  question; the answer cites this transcript shape.

Probes 4 and 5 are corroborative, not contrastive. Probe 4
witnesses the equivalence between lesson 125's pair.N form and
today's destructure form (`diff` exits 0). Probe 5 witnesses the
field types `(&u64, &u64)` directly.
