# Evidence — 091-range-reversal-rev

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` -> `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the
  end. Only the working `.rs` is committed, at
  `experimental/eduratchet2/runs/rust-moves/observations/091-range-reversal-rev.rs`.
  The contrast probe (no-parens E0689) and the auxiliary range
  probes (`(5..8).rev()`, `(0..3).rev()`) are *not* committed as
  separate `.rs` files; their transcripts below are the artifacts.

Same host and toolchain as recent accepted lessons (082-090).

## Sources

### `output/docs/rust/book/ch03-05-control-flow.md`

The Book's *Looping Through a Collection with `for`* subsection,
closing of the section. Lines 533-546 are the load-bearing span
introducing `.rev()`.

Lines 533-535 (the framing — the canonical introduction):

> Here's what the countdown would look like using a `for` loop and
> another method we've not yet talked about, `rev`, to reverse the
> range:

Lines 538-545 (the canonical example — the lesson's working probe
modulo positional vs `{number}` interpolation):

> ```rust
> fn main() {
>     for number in (1..4).rev() {
>         println!("{number}!");
>     }
>     println!("LIFTOFF!!!");
> }
> ```

Direct corpus warrant for the lesson's centered claims:

- *`.rev()` is a method that reverses a range*: the Book's framing
  "another method we've not yet talked about, `rev`, to reverse
  the range" is the canonical introduction. The lesson's *The
  Move* paragraph and *What Changed* bullet 1 restate this rule
  at name-and-use-only depth.
- *The countdown shape*: the Book's verbatim example is the
  lesson's working probe (positional `{}` substituted for
  `{number}` for consistency with lesson 011). The Book frames
  this as the preferred countdown alternative to lesson 017's
  `while`-with-`mut`-counter shape — see the same Book section
  lines 525-528 quoted in lesson 022's evidence.
- *Same `for ... in ...` shape*: the Book reuses the exact `for
  number in <range expression>` construct from lesson 022; only
  the range expression changes from `0..N` to `(1..4).rev()`.

The Book's framing "another method we've not yet talked about"
explicitly defers the iterator-trait machinery; today's lesson
honors that scope and treats `.rev()` as a name-and-use atom.

### `output/docs/rust/std/iter/trait.Iterator.md`

The `Iterator::rev` method documentation, lines 2990-3012. Two
load-bearing spans.

Lines 2990-2995 (the canonical description):

> #### fn rev(self) -> Rev<Self> where Self: Sized + DoubleEndedIterator,
>
> Reverses an iterator's direction.
>
> Usually, iterators iterate from left to right. After using
> `rev()`, an iterator will instead iterate from right to left.

Lines 2997-2998 (the requirement):

> This is only possible if the iterator has an end, so `rev()`
> only works on `DoubleEndedIterator`s.

Direct corpus warrant: the std library's authoritative statement
that `.rev()` reverses iteration direction. The lesson stays at
name-and-use-only depth — the trait names `Iterator` and
`DoubleEndedIterator` are mentioned in *What To Ignore For Now*
and explicitly deferred. The signature `fn rev(self) -> Rev<Self>`
is not surfaced learner-facing; the *Rev* iterator type is
deferred.

### `output/docs/rust/std/ops/struct.Range.md`

The `Range` struct's trait implementations, lines 139-141:

> ### impl<A> DoubleEndedIterator for Range<A> where A: Step,

Direct corpus warrant: `Range<A>` (the type produced by the
`start..end` syntax) implements `DoubleEndedIterator`, which is
the trait `.rev()` requires per the `Iterator::rev` signature
above. This is the load-bearing implementation that makes
`(1..4).rev()` legal — the lesson uses it without surfacing it,
at the same scope as the Book's "another method we've not yet
talked about."

## Probes

### Probe 1 (working, committed) — Book countdown verbatim

The committed file at
`experimental/eduratchet2/runs/rust-moves/observations/091-range-reversal-rev.rs`
is the working program from *Try It*, the Book's countdown example
modulo the format-string positional shape (`"{}!", number` instead
of `"{number}!"`, the shape from lesson 011).

Probe transcript, run in a clean temp directory created with
`mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
3!
2!
1!
LIFTOFF!!!
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent (consistent with lesson
  001).
- `./demo` prints exactly four lines: `3!`, `2!`, `1!`,
  `LIFTOFF!!!`. The first three come from the loop body (one
  print per pass, with `number` taking values `3`, `2`, `1` in
  order — *reversed* from the `1..4` range in lesson 022), and
  the fourth comes from the `println!("LIFTOFF!!!");` *after*
  the loop. This is the load-bearing observation for the
  reverse-iteration claim.
- The body prints values in the order `3, 2, 1` and never prints
  `4`. The exclusive-upper-bound rule from lesson 022 carries
  through: `4` is excluded both before and after reversal.
- Only the working source is committed under `observations/`.
  The temp dir was removed; no binaries are committed.

### Probe 2 (contrast, not committed) — no-parens E0689 diagnostic

Same source as Probe 1 modulo dropping the parentheses around
`1..4` on the `for` line: `for number in 1..4.rev() {`. This
witnesses the lesson's parens-required rule.

```text
--- cat broken.rs ---
fn main() {
    for number in 1..4.rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
--- rustc broken.rs ---
error[E0689]: can't call method `rev` on type `{integer}`
 --> broken.rs:2:24
  |
2 |     for number in 1..4.rev() {
  |                        ^^^ can't call method `rev` on type `{integer}`
  |
help: you must surround the range in parentheses to call its `rev` function
  |
2 |     for number in (1..4).rev() {
  |                   +    +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0689`.
exit=1
```

Notes:

- E-code is `E0689` ("can't call method `rev` on type
  `{integer}`"). rustc parsed `4.rev()` first as a method call on
  the integer literal `4`. The integer type is unresolved
  (`{integer}` — the inference placeholder), but the load-bearing
  observation is the parse: the dot binds tighter than `..`, so
  `1..4.rev()` parses as `1..(4.rev())`, not `(1..4).rev()`.
- The `help:` line is the load-bearing diagnostic content: "you
  must surround the range in parentheses to call its `rev`
  function", with a source-diff showing the `+ ` markers under
  the open paren before `1` and the close paren before `.rev()`.
  rustc's own statement of the parens-required rule.
- This is the negative-witness probe required for the lesson's
  contrastive claim "with parens it works, without parens
  rustc rejects." Read with the diagnostic map from lesson 003:
  headline + `-->` location + source excerpt + caret + `help:`
  source-diff + summary trailer + `try `rustc --explain E0689``.

### Probe 3 (auxiliary, not committed) — `(5..8).rev()` answer-key

Witnesses *Check Yourself* answer (b): `(5..8).rev()` produces
`7, 6, 5`.

```text
--- cat range58.rs ---
fn main() {
    for n in (5..8).rev() {
        println!("{}", n);
    }
}
--- rustc range58.rs ---
exit=0
--- ./range58 ---
7
6
5
exit=0
```

Notes:

- Three lines, in the order `7, 6, 5`. The exclusive upper bound
  `8` is not iterated. This corroborates the Book's
  "ending before another number" exclusive-upper-bound rule
  (lesson 022) under reversal: `5..8` produces `5, 6, 7` going
  up; `(5..8).rev()` produces those same three values in reverse.
- This probe also generalizes the working probe past the Book's
  specific `1..4` choice, witnessing that reversal of an
  arbitrary `start..end` range yields `end-1, end-2, ..., start`.

### Probe 4 (auxiliary, not committed) — `(0..3).rev()` shape sanity

Witnesses that the same shape holds when `start = 0` (the lesson
022 base case).

```text
--- cat zero3.rs ---
fn main() {
    for n in (0..3).rev() {
        println!("n = {}", n);
    }
    println!("done");
}
--- rustc zero3.rs ---
exit=0
--- ./zero3 ---
n = 2
n = 1
n = 0
done
exit=0
```

Notes:

- Three lines from the loop (`n = 2`, `n = 1`, `n = 0`), then
  `done`. Same four-line pattern as lesson 022's `for i in 0..3`
  probe, but the loop's three values are reversed. This is the
  cleanest possible witness that `.rev()` on a range produces
  exactly the lesson 022 set in reversed order — the visible
  differential between lesson 022 and today's lesson under one
  one-method edit.

## Prior lessons

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs` then
  `./name`, silent on success.
- `002-fn-main-entry-point` (accepted) — body of `fn main` runs
  when the executable launches.
- `003-read-rustc-diagnostic` (accepted) — diagnostic map applied
  to Probe 2's E0689 transcript.
- `011-println-positional-args` (accepted) — `println!("{}", x)`
  positional substitution; today uses `println!("{}!", number)`
  matching the run's accepted shape.
- `022-for-range` (accepted, load-bearing) — `for var in 0..N { ... }`
  iterates the exclusive range in ascending order; today swaps
  the bare range for `(start..end).rev()` and observes reversed
  order with the same `for` shape. Lesson 022's *What To Ignore
  For Now* explicitly named "Range methods like `.rev()`" as a
  deferred future move; today is that move.
- `040-method-call-syntax` (accepted, load-bearing) — the
  method-call form `value.method(args)` with receiver, dot,
  method name, and parenthesized argument list. Today applies
  the form to a range expression with empty arguments. Lesson
  040's *What To Ignore For Now* listed method chaining and
  associated functions as deferred, but today's `(1..4).rev()`
  is just the basic `value.method()` form from lesson 040
  itself, not a chain.
