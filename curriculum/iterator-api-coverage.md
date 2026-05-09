# `std::iter::Iterator` API Coverage Audit — v5

## 0. Scope and dating

- **v5 (2026-05-08)** supersedes v4 (commit `c0fe89215`, same
  date). v4 anchored the first closure-driven Iterator arc
  completion (lessons 149-152) covering the four most
  structurally distinct shapes. v5 records the **by-value
  predicate-consumer arc** (arc two) completion as accepted
  lessons 153-155 — covering `any`, `all`, `position` (the
  short-circuiting `&mut self` predicate-consumer family with
  by-value `Self::Item` closure parameters).
- v1 → v2 transition: 11 closure-free Iterator methods (lessons
  131-141) closed.
- v2 → v3 transition: 7 closure-prerequisite lessons (lessons
  142-148) closed.
- v3 → v4 transition: 4 closure-driven Iterator methods (lessons
  149-152) closed.
- v4 → v5 transition: 3 by-value predicate-consumer methods
  (lessons 153-155) closed. Each rotates exactly one structural
  slot from the prior sibling: `any` introduces `&mut self` +
  short-circuit + empty-`false` (relative to lesson 152's
  consuming `self` and `Option`-return), `all` inverts
  short-circuit polarity and empty-case identity (existential
  → universal), `position` rotates the bool return slot to
  `Option<usize>` (composes lesson 119's `Option<T>` with
  lesson 138's `usize` index counter).
- Audit anchored against
  `/Users/eli/InfoScraper/output/docs/rust/std/iter/trait.Iterator.md`
  (5114 lines, source URL
  https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html).
- Toolchain on host: `rustc 1.95.0` / `cargo 1.95.0`.
- Branch: `eduratchet/rust-moves`. Last accepted node:
  `155-iterator-position` (commit `142264647`). Total
  accepted nodes: **155** (was 152 at v4, 148 at v3, 141 at v2,
  130 at v1).
- Goal of this document: drive curriculum planning toward
  *quiz-standard understanding of the stable, non-esoteric public API
  of `std::iter::Iterator`*. Quiz claims under audit include: what a
  method does; whether it consumes `self`, borrows `&mut self`, or
  borrows `&self`; what it returns; what trait bounds it requires;
  whether it is lazy or consuming; when it short-circuits; what it
  yields on empty iterators; what its examples print; what
  panic/overflow notes apply.
- Out-of-scope: nightly-only experimental methods (15 listed in §3),
  deprecated methods (none on this trait at audit time), implementor
  list (everything below the trait declaration in `trait.Iterator.md`),
  the structurally-referenced adapter structs (e.g. `Map`, `Filter`,
  `Zip`, `Rev`) — those are referred to opaquely as the trait's
  "return type" without internal-fields lessons.

## 1. Page surface (counts)

The trait declaration declares **76 methods total**. Stability split:

- 1 required associated type: `Item` (1.0.0).
- 1 required method: `next(&mut self) -> Option<Self::Item>` (1.0.0).
- 60 stable provided methods (oldest 1.0.0, newest 1.82.0).
- 15 nightly-only experimental methods.

Total stable surface for this audit: **62 stable items** (1 assoc type
+ 1 required method + 60 provided methods).

The 14 stability-version cohorts (1.0.0, 1.5.0, 1.6.0, 1.11.0, 1.15.0,
1.21.0, 1.27.0, 1.28.0, 1.29.0, 1.30.0, 1.36.0, 1.51.0, 1.57.0, 1.82.0)
all stabilize *before* the local toolchain's 1.95.0, so every stable
item below is callable on this host.

## 2. Stable items in scope (62)

Listed in page order. `kind` column abbreviations: `req` = required,
`prov` = provided, `assoc` = associated type. `recv` = receiver shape.

| # | id | kind | recv | since | one-line semantic |
|---|---|---|---|---|---|
| 0 | `Item` | assoc | n/a | 1.0.0 | element type of the iterator |
| 1 | `next` | req | `&mut self` | 1.0.0 | advance, return next or `None` |
| 2 | `size_hint` | prov | `&self` | 1.0.0 | `(usize, Option<usize>)` lower/upper bound |
| 3 | `count` | prov | `self` | 1.0.0 | consume, return `usize` count |
| 4 | `last` | prov | `self` | 1.0.0 | consume, return last `Option<Item>` |
| 5 | `nth` | prov | `&mut self` | 1.0.0 | `nth(n)` returns Option, drops preceding |
| 6 | `step_by` | prov | `self` | 1.28.0 | adapter, every `step`th element; `step != 0` |
| 7 | `chain` | prov | `self` | 1.0.0 | adapter, append second iter; second is `IntoIterator` |
| 8 | `zip` | prov | `self` | 1.0.0 | adapter, pair with second; ends at shorter |
| 9 | `map` | prov | `self` | 1.0.0 | adapter, `FnMut(Item) -> B` |
| 10 | `for_each` | prov | `self` | 1.21.0 | consumer, `FnMut(Item)` |
| 11 | `filter` | prov | `self` | 1.0.0 | adapter, `FnMut(&Item) -> bool` |
| 12 | `filter_map` | prov | `self` | 1.0.0 | adapter, `FnMut(Item) -> Option<B>` |
| 13 | `enumerate` | prov | `self` | 1.0.0 | adapter, yields `(usize, Item)` |
| 14 | `peekable` | prov | `self` | 1.0.0 | adapter, adds `.peek()` |
| 15 | `skip_while` | prov | `self` | 1.0.0 | adapter, `FnMut(&Item) -> bool` |
| 16 | `take_while` | prov | `self` | 1.0.0 | adapter, `FnMut(&Item) -> bool` |
| 17 | `map_while` | prov | `self` | 1.57.0 | adapter, `FnMut(Item) -> Option<B>` |
| 18 | `skip` | prov | `self` | 1.0.0 | adapter, drop first `n` |
| 19 | `take` | prov | `self` | 1.0.0 | adapter, keep first `n` |
| 20 | `scan` | prov | `self` | 1.0.0 | adapter, stateful map-with-Option |
| 21 | `flat_map` | prov | `self` | 1.0.0 | adapter, `FnMut(Item) -> IntoIterator` |
| 22 | `flatten` | prov | `self` | 1.29.0 | adapter, `Item: IntoIterator` |
| 23 | `fuse` | prov | `self` | 1.0.0 | adapter, sticky `None` |
| 24 | `inspect` | prov | `self` | 1.0.0 | adapter, `FnMut(&Item)` peek without consuming |
| 25 | `by_ref` | prov | `&mut self` | 1.0.0 | borrow as `&mut Self` for partial consumption |
| 26 | `collect` | prov | `self` | 1.0.0 | consumer, `B: FromIterator<Item>` |
| 27 | `partition` | prov | `self` | 1.0.0 | consumer, `(B, B)` split, `Default + Extend` |
| 28 | `try_fold` | prov | `&mut self` | 1.27.0 | consumer with `?`-style early exit |
| 29 | `try_for_each` | prov | `&mut self` | 1.27.0 | consumer with `?`-style early exit |
| 30 | `fold` | prov | `self` | 1.0.0 | consumer, accumulator + `FnMut(B, Item) -> B` |
| 31 | `reduce` | prov | `self` | 1.51.0 | consumer, `FnMut(Item, Item) -> Item`, Option |
| 32 | `all` | prov | `&mut self` | 1.0.0 | consumer, predicate; short-circuits |
| 33 | `any` | prov | `&mut self` | 1.0.0 | consumer, predicate; short-circuits |
| 34 | `find` | prov | `&mut self` | 1.0.0 | consumer, returns first matching `Some(Item)` |
| 35 | `find_map` | prov | `&mut self` | 1.30.0 | consumer, first non-`None` map result |
| 36 | `position` | prov | `&mut self` | 1.0.0 | consumer, first matching index `Option<usize>` |
| 37 | `rposition` | prov | `&mut self` | 1.0.0 | consumer, last index; needs `DoubleEndedIterator + ExactSizeIterator` |
| 38 | `max` | prov | `self` | 1.0.0 | consumer, `Item: Ord` |
| 39 | `min` | prov | `self` | 1.0.0 | consumer, `Item: Ord` |
| 40 | `max_by_key` | prov | `self` | 1.6.0 | consumer, key projection |
| 41 | `max_by` | prov | `self` | 1.15.0 | consumer, custom `Ordering` comparator |
| 42 | `min_by_key` | prov | `self` | 1.6.0 | consumer, key projection |
| 43 | `min_by` | prov | `self` | 1.15.0 | consumer, custom `Ordering` comparator |
| 44 | `rev` | prov | `self` | 1.0.0 | adapter, needs `DoubleEndedIterator` |
| 45 | `unzip` | prov | `self` | 1.0.0 | consumer, `(FromA, FromB)`; needs `Item = (A, B)` |
| 46 | `copied` | prov | `self` | 1.36.0 | adapter, `Item = &T, T: Copy` |
| 47 | `cloned` | prov | `self` | 1.0.0 | adapter, `Item = &T, T: Clone` |
| 48 | `cycle` | prov | `self` | 1.0.0 | adapter, infinite repeat; needs `Self: Clone` |
| 49 | `sum` | prov | `self` | 1.11.0 | consumer, `S: Sum<Item>` |
| 50 | `product` | prov | `self` | 1.11.0 | consumer, `P: Product<Item>` |
| 51 | `cmp` | prov | `self` | 1.5.0 | lex compare, `Item: Ord`, other is `IntoIterator` |
| 52 | `partial_cmp` | prov | `self` | 1.5.0 | lex compare, `Item: PartialOrd<other::Item>` |
| 53 | `eq` | prov | `self` | 1.5.0 | lex equal, `Item: PartialEq<other::Item>` |
| 54 | `ne` | prov | `self` | 1.5.0 | lex not-equal |
| 55 | `lt` | prov | `self` | 1.5.0 | lex less-than |
| 56 | `le` | prov | `self` | 1.5.0 | lex less-or-equal |
| 57 | `gt` | prov | `self` | 1.5.0 | lex greater-than |
| 58 | `ge` | prov | `self` | 1.5.0 | lex greater-or-equal |
| 59 | `is_sorted` | prov | `self` | 1.82.0 | consumer, `Item: PartialOrd` |
| 60 | `is_sorted_by` | prov | `self` | 1.82.0 | consumer, custom comparator |
| 61 | `is_sorted_by_key` | prov | `self` | 1.82.0 | consumer, key projection |

## 3. Out of scope (15 nightly-only)

Excluded per scope rule. Listed for completeness; revisit if/when the
local docs flag them as stabilized.

| name | feature gate | tracking issue |
|---|---|---|
| `next_chunk` | `iter_next_chunk` | rust-lang/rust#98326 |
| `advance_by` | `iter_advance_by` | rust-lang/rust#77404 |
| `intersperse` | `iter_intersperse` | rust-lang/rust#79524 |
| `intersperse_with` | `iter_intersperse` | rust-lang/rust#79524 |
| `map_windows` | `iter_map_windows` | rust-lang/rust#87155 |
| `try_collect` | `iterator_try_collect` | rust-lang/rust#94047 |
| `collect_into` | `iter_collect_into` | rust-lang/rust#94780 |
| `partition_in_place` | `iter_partition_in_place` | rust-lang/rust#62543 |
| `is_partitioned` | `iter_is_partitioned` | rust-lang/rust#62544 |
| `try_reduce` | `iterator_try_reduce` | rust-lang/rust#87053 |
| `try_find` | `try_find` | rust-lang/rust#63178 |
| `array_chunks` | `iter_array_chunks` | rust-lang/rust#100450 |
| `cmp_by` | `iter_order_by` | rust-lang/rust#64295 |
| `partial_cmp_by` | `iter_order_by` | rust-lang/rust#64295 |
| `eq_by` | `iter_order_by` | rust-lang/rust#64295 |

## 4. Coverage by accepted graph (148 nodes)

### 4.1 Items already exercised at quiz depth — 18

Each item below has been installed as a centered Iterator move with
quiz-shaped semantic surface (consumes-`self` vs `&mut self` vs
`&self`, lazy-vs-consuming, short-circuit, empty-iterator behavior,
panic notes) anchored to the `Iterator` trait surface.

| Iterator item | Lesson(s) | Centered fact installed |
|---|---|---|
| `Item` (assoc type) | 132 | `type Item;` line read structurally; `Counter` impl supplies `type Item = u32;` |
| `next` | 131, 132 | `.next()` call on slice iter returns `Option<&T>`; trait declaration `fn next(&mut self) -> Option<Self::Item>` read structurally; `&mut self` receiver and E0596 fire when `mut` is dropped |
| `size_hint` | 141 | `(&self) -> (usize, Option<usize>)`; FIRST `&self` provided method; lower bound + upper-as-`Option`; default is `(0, None)`; hint not guarantee |
| `count` | 133 | `(self) -> usize`; first stable provided method; consuming receiver E0382 captured |
| `last` | 134 | `(self) -> Option<Self::Item>`; consuming receiver; `None` on empty |
| `nth` | 135 | `(&mut self, n: usize) -> Option<Self::Item>`; drops preceding elements; `&mut self` lacks `where Self: Sized` |
| `take` | 136 | `(self, n: usize) -> Take<Self>`; first lazy adapter; wrapper-struct return that itself implements `Iterator` |
| `skip` | 137 | `(self, n: usize) -> Skip<Self>`; lazy adapter inverse to `take` |
| `enumerate` | 138 | `(self) -> Enumerate<Self>`; first tuple-rewriting adapter; yields `(usize, Self::Item)`; no second parameter |
| `fuse` | 139 | `(self) -> Fuse<Self>`; sticky-`None` enforcement; three-way distinction `FusedIterator` marker / `Fuse<I>` struct / `Iterator::fuse` method |
| `step_by` | 140 | `(self, step: usize) -> StepBy<Self>`; first adapter with eager panic precondition (`step != 0`); panic at construction not first `.next()` |
| `for_each` | 149 | `(self, f: F) where F: FnMut(Self::Item)`; first closure-driven Iterator method; consumer; returns `()`; `break`/`continue` rejected from closure body (E0267 new) |
| `map` | 150 | `<B, F>(self, f: F) -> Map<Self, F> where F: FnMut(Self::Item) -> B`; first lazy closure-driven adapter; first wrapper struct with two type parameters; `type Item = B`; lazy — closure not called until iterated |
| `fold` | 151 | `<B, F>(self, init: B, f: F) -> B where F: FnMut(B, Self::Item) -> B`; first multi-parameter parens-bound; threading semantics (each closure return becomes next call's first arg); `init`'s type fixes `B`; E0593 (wrong arity) new |
| `reduce` | 152 | `<F>(self, f: F) -> Option<Self::Item> where F: FnMut(Self::Item, Self::Item) -> Self::Item`; first Option-returning closure-driven consumer; first-element-as-init pattern; homogeneous bound (all three positions `Self::Item`); `None` for empty; E0061 (wrong arg count) load-bearing contrast |
| `any` | 153 | `<F>(&mut self, f: F) -> bool where F: FnMut(Self::Item) -> bool`; first `&mut self` receiver on a closure-driven method; first short-circuit (on first `true`); iterator still usable after the call; empty → `false` (existential identity); E0267 (`break` inside closure) was new in 149 — today shape-only |
| `all` | 154 | same signature shape as `any` with inverted polarity: short-circuit on first `false`; empty → `true` (universal identity / vacuous truth); `unused_comparisons` lint surface for the `x < 0` impossible-predicate empty-case witness |
| `position` | 155 | `<P>(&mut self, predicate: P) -> Option<usize>` with `P: FnMut(Self::Item) -> bool`; same signature shape as `any`/`all` with return slot rotated from `bool` to `Option<usize>`; index-vs-value distinction (returned `usize` is the zero-based position in the yielded sequence, not the element value); composes lesson 119's `Option<T>` with lesson 138's `usize` iteration counter; type-parameter slot renamed `<F>` → `<P>` for "predicate" (cosmetic) |

This 18-row group covers (a) the **§5 first-arc plan complete**
(closure-free, lessons 131-141) — every receiver shape and the
lazy-adapter family; plus (b) the **§7 first closure-driven
Iterator arc complete** (lessons 149-152) — the four most
structurally distinct closure-driven shapes; plus (c) the **§8
by-value predicate-consumer arc complete** (lessons 153-155) —
the three short-circuiting `&mut self` predicate-consumer
methods with by-value `Self::Item` closure parameters
(`any`/`all`/`position`).

### 4.2 Partially exercised — 3 items

| Iterator item | Existing coverage | Gap to quiz |
|---|---|---|
| `rev` | 091 installs `(0..N).rev()` on `Range<i32>`. 124 installs `slice::Iter<'_, T>::rev`. | Not yet stated as a method on the `Iterator` *trait*; the `Self: Sized + DoubleEndedIterator` bound and the `Rev<Self>` adapter struct are uninstalled. |
| `zip` | 125 installs `v.iter().zip(w.iter())` for slice iterators. | The `IntoIterator`-shaped argument and the shortest-source rule are present in the lesson body but not anchored to the `Iterator::zip` signature. |
| Implicit `for x in expr` desugaring | 022/039/079/126 use `for` over `Range`, `&[T; N]`, `Vec::iter`. | The desugaring rule from the Reference (`for x in expr` ≡ `for x in IntoIterator::into_iter(expr)`) is named-deferred since 022. |

### 4.3 Ready now (no new prereqs) — 20

**v5 update:** the 3 by-value predicate-consumer methods
accepted as lessons 153-155 (`any`, `all`, `position`) have
moved to §4.1 (exercised at quiz depth). 20 closure-driven
Iterator methods remain ready-now.

| Iterator method | Likely bound | Receiver | Notes |
|---|---|---|---|
| `filter` | `FnMut(&Self::Item) -> bool` | `self` | lazy adapter; closure takes shared ref → introduces `&Self::Item` parameter shape (deref-read prereq) |
| `filter_map` | `FnMut(Self::Item) -> Option<B>` | `self` | lazy adapter combining filter + map (closure takes `Self::Item` by value, no deref) |
| `take_while` | `FnMut(&Self::Item) -> bool` | `self` | lazy short-circuiting adapter (deref-read prereq) |
| `skip_while` | `FnMut(&Self::Item) -> bool` | `self` | lazy adapter dual to take_while (deref-read prereq) |
| `map_while` | `FnMut(Self::Item) -> Option<B>` | `self` | lazy adapter; short-circuits on None (no deref) |
| `scan` | `FnMut(&mut St, Self::Item) -> Option<B>` | `self` | lazy stateful adapter; `&mut St` parameter introduces mutable-reference closure-parameter mechanic |
| `inspect` | `FnMut(&Self::Item)` | `self` | lazy adapter for side-effects (deref-read prereq) |
| `find` | `FnMut(&Self::Item) -> bool` | `&mut self` | consumer; first match (deref-read prereq) |
| `find_map` | `FnMut(Self::Item) -> Option<B>` | `&mut self` | consumer; first non-None map (no deref) |
| `rposition` | `FnMut(Self::Item) -> bool` | `&mut self` | consumer; needs `DoubleEndedIterator + ExactSizeIterator` |
| `partition` | `FnMut(&Self::Item) -> bool` | `self` | consumer; needs Default + Extend |
| `flat_map` | `FnMut(Self::Item) -> IntoIterator` | `self` | gated also on IntoIterator (4.4.2) |
| `try_fold` | `FnMut(B, Self::Item) -> Try` | `&mut self` | gated on Try sub-arc (4.4.4) |
| `try_for_each` | `FnMut(Self::Item) -> Try` | `&mut self` | gated on Try sub-arc (4.4.4) |
| `max_by` | `FnMut(&T, &T) -> Ordering` | `self` | needs Ordering / Ord |
| `max_by_key` | `FnMut(&Self::Item) -> K: Ord` | `self` | needs Ord |
| `min_by` | `FnMut(&T, &T) -> Ordering` | `self` | needs Ordering / Ord |
| `min_by_key` | `FnMut(&Self::Item) -> K: Ord` | `self` | needs Ord |
| `is_sorted_by` | `FnMut(&T, &T) -> bool` | `self` | needs PartialOrd at trait depth |
| `is_sorted_by_key` | `FnMut(Self::Item) -> K: PartialOrd` | `self` | needs PartialOrd at trait depth |

**Caveats.** Of the remaining 23, three (`flat_map`, `try_fold`,
`try_for_each`) carry an additional cross-arc dependency
(IntoIterator or Try), and six (`max_by`, `max_by_key`, `min_by`,
`min_by_key`, `is_sorted_by`, `is_sorted_by_key`) need an Ord /
PartialOrd / Ordering bound at trait depth. **Crucially:** v4
splits the remaining methods into two by-value vs by-reference
groups:

- **By-value-element closure parameter (no deref-read prereq):**
  `filter_map`, `map_while`, `find_map`, `position`, `rposition`,
  `all`, `any`. **7 methods.** These compose cleanly with the
  established `Range<u32>` discipline; closure parameter is
  `Self::Item` (or `&mut St` for `scan`).
- **By-reference-element closure parameter (deref-read prereq
  needed first):** `filter`, `take_while`, `skip_while`,
  `inspect`, `find`, `partition`. **6 methods.** Each takes
  `&Self::Item` in its closure parameter. To handle this cleanly
  on a `Range<u32>` source (which would yield `&u32` to the
  closure), the run needs a small move installing `*x` deref-read
  on shared references — lessons 047/048 explicitly deferred that
  mechanic. This sub-arc opens by installing deref-read first.

**The cleanest second arc** uses the by-value group (no deref
prereq) — specifically the **by-value predicate-consumer family**
`any` / `all` / `position` (3 methods, two new structural facts:
`&mut self` receiver on a closure-driven method, plus
short-circuit semantics).

### 4.4 Requires prereqs

#### 4.4.1 Closure sub-arc — **COMPLETE (lessons 142-148)**

The closure sub-arc is now closed. The seven lessons that
installed it:

| Step | Lesson | Centered fact | Commit |
|---|---|---|---|
| §6.1 | 142-closure-literal-bound-and-called | closure literal `\|x: u32\| x + 1` bound to `let` and called with parens; closure-as-value framing | `915347f66` |
| §6.2 | 143-unannotated-closure-first-use | drop the parameter annotation; first call fixes the type via type inference; second call with different type fires E0308 with cross-reference `note:` | `50049ef9b` |
| §6.3 | 144-closure-captures-outer-let | closure body may reference a name from the enclosing scope; nested `fn` items cannot, fires E0434 (closure/`fn` asymmetry) | `e59fd8401` |
| §6.4a | 145-generic-function-type-parameter | `fn name<T>(t: T) -> T` declares a type parameter that substitutes per call site (the "distinct mechanic, same `<T>` slot" lesson 114 deferred) | `b3b1b0434` |
| §6.4b | 146-trait-bound-on-type-parameter | `fn name<T: TRAIT>(t: T)` constrains substitution AND grants the body the right to use TRAIT's methods on `t` | `a3e256ddf` |
| §6.4c | 147-fn-trait-parenthesized-bound | parenthesized Fn-family bound `<F: Fn(T) -> R>` plus closure-as-argument; the syntax all closure-driven Iterator methods use | `79b010939` |
| §6.5 | 148-fn-fnmut-fnonce-distinction | three Fn-family traits with supertrait layering Fn ⊆ FnMut ⊆ FnOnce; capture mode (read/mutate/move) determines auto-impl; `mut f: F` requirement for FnMut bounds (closer) | `c837edef7` |

The 27 closure-driven Iterator methods previously listed under
§4.4.1 have moved to §4.3 (ready-now).

#### 4.4.2 `IntoIterator` sub-arc — currently gates 9 methods

`chain`, `zip` (full statement at trait depth), `cmp`, `partial_cmp`,
`eq`, `ne`, `lt`, `le`, `gt`, `ge`. (8 stable items + the desugar rule
for `for`.)

Requires installing the `IntoIterator` trait declaration:

```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

per `output/docs/rust/std/iter/trait.IntoIterator.md`. The `for x in
expr` desugaring rule from the Reference can land alongside.

#### 4.4.3 Bounded-by-other-trait sub-arc

| Iterator item | Bound prereq |
|---|---|
| `max`, `min` | `Item: Ord` (117/121/130 install Ord on concrete types; trait-bound form is its own move) |
| `sum` | `S: Sum<Item>` — Sum trait |
| `product` | `P: Product<Item>` — Product trait |
| `cmp`, `partial_cmp`, `eq`, `ne`, `lt`, `le`, `gt`, `ge`, `is_sorted` | Ord/PartialOrd/PartialEq trait-bound form |
| `copied` | `Copy` on `T` |
| `cloned` | `Clone` on `T` |
| `cycle` | `Self: Clone` |
| `collect` | `B: FromIterator<Item>` + turbofish |
| `partition` | `B: Default + Extend<Item>` |
| `unzip` | `FromA: Default + Extend<A>`, `FromB: Default + Extend<B>`, `Self: Iterator<Item=(A,B)>` |

Several of these (`Copy`, `Clone`, `Default`, `Extend`, `FromIterator`,
`Sum`, `Product`) are independent trait-introduction lessons. The
*generic-bound-on-method* shape itself is partially installed at 114
(generic trait parameter).

#### 4.4.4 Try sub-arc — currently gates 2 stable methods

`try_fold`, `try_for_each`. Both stabilize in 1.27.0 and use the
`Try` trait family from `std::ops`. The `?` operator and the
`Result`/`Option` early-exit forms are independent prereqs. Named-
deferred until the Try arc.

#### 4.4.5 Other small prereqs

| Iterator item | Prereq |
|---|---|
| `peekable` | adapter struct with a `.peek()` method on it; small lesson on its own |
| `by_ref` | `&mut Self` reference-to-iterator pattern — exists for "consume part of an iterator without losing the rest" |
| `rposition` | `Self: Sized + DoubleEndedIterator + ExactSizeIterator` (additional traits beyond `Iterator`) |
| `rev` (trait-depth statement) | `Self: Sized + DoubleEndedIterator` |

`DoubleEndedIterator` and `ExactSizeIterator` are sibling traits whose
declarations live in the same module (`output/docs/rust/std/iter/trait.DoubleEndedIterator.md`,
`trait.ExactSizeIterator.md`). They become small lessons of their
own once the closure arc is past.

## 5. First-arc plan — COMPLETE

Bottom-up sequence taking the trait from "named-deferred since 119"
to "structurally readable end-to-end":

1. **`iter.next()` call** — call `.next()` on a slice iterator,
   observe `Option<&T>`, four calls reach `None`. Composes
   123 + 119 + 006. Single new fact: `next` is callable, takes
   `&mut self`. *Highest-priority ready-now move.* — **Accepted as
   lesson 131 (commit `c6c8f43c6`).**
2. **`Iterator` trait declaration** — read `pub trait Iterator { type
   Item; fn next(&mut self) -> Option<Self::Item>; }` structurally.
   Composes 111-116 + 115 + 119 + step 1. Anchors every later
   "this method is on `Iterator`" claim. — **Accepted as lesson 132
   (commit `cfc2b485f`).**
3. **`count`** — smallest stable consumer, names the
   call-`next`-until-`None`-and-count semantic, names self-by-value
   consuming. Carries `usize::MAX` overflow note. — **Accepted as
   lesson 133 (commit `f9d06c37e`).**
4. **`last`** — small consumer, names the infinite-iterator panic
   trigger. — **Accepted as lesson 134 (commit `ca1dcf25b`).**
5. **`nth`** — `&mut self` consumer, drops preceding elements. —
   **Accepted as lesson 135 (commit `3d1719543`).**
6. **`take`** — small lazy adapter, returns `Take<Self>`. —
   **Accepted as lesson 136 (commit `45139e4b8`).**
7. **`skip`** — small lazy adapter, returns `Skip<Self>`. —
   **Accepted as lesson 137 (commit `a704bd9ba`).**
8. **`enumerate`** — yields `(usize, Item)`; `usize::MAX` overflow
   note. — **Accepted as lesson 138 (commit `4dad1a4f0`).**
9. **`fuse`** — sticky-`None` rule; the contrast probe writes a
   custom iterator that resumes after `None`. — **Accepted as
   lesson 139 (commit `370d3dbac`).**
10. **`step_by`** — `step != 0` panic. — **Accepted as lesson 140
    (commit `548d3647f`).**
11. **`size_hint`** — observation method, tuple result. — **Accepted
    as lesson 141 (commit `49381d6df`).**

After step 11 the closure-free non-consumer Iterator surface is
**covered ✓** (lesson 141, commit `49381d6df`, 2026-05-08). Every
receiver shape on the `Iterator` trait (`self` consuming / `&mut self`
mutating / `&self` read-only) now has at least one stdlib-anchored
witness. The lazy-adapter family (take, skip, enumerate, fuse,
step_by) is fully installed.

Likely intermediate capstone after the closure sub-arc: a small
realistic iterator pipeline (`v.iter().filter(...).map(...).collect()`)
read end-to-end.

Likely final capstone for this target: a single `Iterator` quiz with
N questions sampled across `kind`, `recv`, lazy/consuming, short-
circuit, and bounds.

## 6. Closure sub-arc — COMPLETE

Bottom-up sequence taking the run from "no closure machinery
installed" to "27 closure-driven Iterator methods are teachable":

1. **Closure literal `|x| x + 1` bound and called** — bind a
   one-parameter closure to a `let`, call it twice. The Book canonical
   example at `output/docs/rust/book/ch13-01-closures.md:210-212`
   gives three forms — fully annotated `|x: u32| -> u32 { x + 1 }`
   (Book v2), partially annotated `|x| { x + 1 }` (Book v3),
   no-braces expression body `|x| x + 1` (Book v4). — **Accepted as
   lesson 142 (commit `915347f66`).**
2. **Closure type inference and the "first call fixes the type"
   rule** — drop the parameter annotation; rustc infers ONE concrete
   type per parameter at first use; second call with a different
   type fires E0308 with cross-reference `note:` to the first call.
   — **Accepted as lesson 143 (commit `50049ef9b`).**
3. **Closure capturing an outer binding** — closure body may
   reference a name from the enclosing scope (immutable read);
   nested `fn` items cannot, fires E0434. The closure/`fn`
   asymmetry is what makes closures genuinely different from
   `fn` items, not just lighter syntax. — **Accepted as lesson
   144 (commit `e59fd8401`).**
4. **`FnMut`-bound parameter on a function** — sketched in v2 as
   one move; in execution split into three lessons.
   - **(4a) Generic function syntax** `fn id<T>(t: T) -> T`. —
     **Accepted as lesson 145 (commit `b3b1b0434`).**
   - **(4b) Trait bound on a generic function parameter**
     `fn say<T: std::fmt::Display>(t: T)`. — **Accepted as lesson
     146 (commit `a3e256ddf`).**
   - **(4c) Parenthesized Fn-family bound + closure-as-argument**
     `fn apply<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 { f(x) }`.
     — **Accepted as lesson 147 (commit `79b010939`).**
5. **`Fn` / `FnMut` / `FnOnce` distinction** — three Fn-family
   traits with supertrait layering Fn ⊆ FnMut ⊆ FnOnce; capture
   mode (read/mutate/move) determines auto-impl; `mut f: F`
   requirement for FnMut bounds. **Closer of the closure sub-arc.**
   — **Accepted as lesson 148 (commit `c837edef7`).**

After step 5, the closure sub-arc is **complete ✓** (lesson 148,
commit `c837edef7`, 2026-05-08). All seven prereq lessons are
accepted. The 27 closure-driven Iterator methods (audit §4.4.1)
become teachable next; 16 of them have no further cross-arc
gates and are listed in §4.3 as "fully ready-now."

## 7. First closure-driven Iterator arc — COMPLETE

Bottom-up sequence taking the run from "27 closure-driven
Iterator methods are teachable" (v3) to "the four most
structurally distinct closure-driven shapes are installed at
quiz depth":

1. **`Iterator::for_each`** — simplest closure-driven Iterator
   method. Consumer, `FnMut(Self::Item)`, returns `()`.
   `break`/`continue` rejected from closure body (E0267 new).
   Source: `Range<u32>` per discipline.
   — **Accepted as lesson 149 (commit `662deddf7`).**
2. **`Iterator::map`** — first lazy closure-driven adapter.
   `FnMut(Self::Item) -> B`, returns `Map<Self, F>` wrapper.
   First wrapper struct with two type parameters; `type Item =
   B`. Lazy semantics empirically witnessed via lazy/consume
   contrast pair plus `must_use` lint.
   — **Accepted as lesson 150 (commit `d23f02964`).**
3. **`Iterator::fold`** — first multi-parameter parens-bound.
   `<B, F>(self, init: B, f: F) -> B where F: FnMut(B,
   Self::Item) -> B`. Threading semantics; `init`'s type fixes
   `B`. E0593 (wrong arity) new today.
   — **Accepted as lesson 151 (commit `de0277005`).**
4. **`Iterator::reduce`** — first Option-returning closure-driven
   consumer. Single type parameter `<F>`; homogeneous bound
   `FnMut(Self::Item, Self::Item) -> Self::Item` (all three
   positions same type, no `B` slot). First-element-as-init;
   `None` for empty. E0061 contrast (1-arg method called with 2
   args) is the load-bearing structural witness.
   — **Accepted as lesson 152 (commit `7e624e74f`).**

After step 4 the first closure-driven Iterator arc is
**covered ✓** (lesson 152, commit `7e624e74f`, 2026-05-08).
The four most structurally distinct closure-driven shapes are
installed at quiz depth:

- **Consumer with no return** (`for_each` — `()`, FnMut(Item))
- **Lazy adapter with element-type rewrite** (`map` —
  `Map<Self, F>` wrapper, FnMut(Item) -> B)
- **Multi-parameter consumer with explicit init** (`fold` —
  `B`, FnMut(B, Item) -> B)
- **Option-returning consumer with first-element-as-init**
  (`reduce` — `Option<Item>`, FnMut(Item, Item) -> Item)

Likely intermediate capstone (deferred to a later cap):
end-to-end iterator pipeline `(1..N).filter(...).map(...).fold(0, ...)`.
Requires the deref-read prereq for `filter`'s `&Self::Item`
parameter; not yet ready.

## 8. By-value predicate-consumer arc — COMPLETE

The arc per §4.3's by-value predicate-consumer group landed
the three methods **`any` / `all` / `position`**.
Three methods with two centered structural facts:

1. **`&mut self` receiver on a closure-driven method** — first
   place a closure-driven Iterator method does not consume the
   iterator. Composes lesson 131's `&mut self` receiver on
   `next()` (which lessons 149-152 contrasted against by
   consuming `self`) with today's closure-bounded methods.
   Witnesses: after `iter.any(|x| ...)` returns, the iterator
   is still usable, and `iter.next()` continues from where
   `any` left off (or from the matching element if it
   short-circuited). This is the most distinct receiver-shape
   delta the sub-arc carries.
2. **Short-circuit semantics on a closure-driven Iterator
   method** — `any` returns `true` on the first matching
   element (skipping the rest); `all` returns `false` on the
   first non-matching element; `position` returns `Some(idx)`
   on the first matching element. First place the audience
   meets the structural rule "the closure may not be called
   on every element."

Corpus signatures (verified):

```
fn any<F>(&mut self, f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool,
fn all<F>(&mut self, f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool,
fn position<P>(&mut self, predicate: P) -> Option<usize> where Self: Sized, P: FnMut(Self::Item) -> bool,
```

All three take `Self::Item` **by value** (not `&Self::Item`),
so `Range<u32>` source keeps the established discipline (no
deref-read smuggle). All three return `bool` or `Option<usize>`
— two new return types for the closure-driven family but no
new return-type *mechanics* (`bool` is from lesson 013,
`Option<usize>` is from lesson 119 with `usize` as the
parameter — `usize` was likely installed during lesson 138's
enumerate work).

Bottom-up sequence (executed):

1. **`any`** — first `&mut self` closure-driven method, first
   short-circuit. Returns `bool`. Empty-input: `false`
   (existential identity). Witnesses `&mut self` reusability
   via `let mut it; it.any(...); it.next()`. — **Accepted as
   lesson 153 (commit `b856dc789`).**
2. **`all`** — sibling of `any` with inverted polarity.
   Short-circuit on first `false`. Empty-input: `true`
   (universal / vacuous truth). The `unused_comparisons` lint
   surfaces in the `(1..1_u32).all(|x| x < 0)` empty-case
   probe. — **Accepted as lesson 154 (commit `d1d280f85`).**
3. **`position`** — first match's zero-based index. Returns
   `Option<usize>`. Composes lesson 119 + lesson 138's `usize`
   counter. Index-vs-value distinction is the load-bearing
   semantic (matched value `15` vs returned index `5` are
   different numbers). — **Accepted as lesson 155 (commit
   `142264647`).**

After step 3 the by-value predicate-consumer arc is **covered ✓**
(lesson 155, commit `142264647`, 2026-05-08).

## 9. Next-arc plan — three candidate paths

After the by-value predicate-consumer arc closes (lesson 155),
three natural paths forward, ranked by smallness:

- **Path A — by-value `Option<B>`-returning closure consumer
  family** (`filter_map`, `find_map`, `map_while`). Each takes
  `Self::Item` by value (no deref smuggle), but the closure
  bound rotates from `FnMut(_) -> bool` (lessons 153-155) to
  `FnMut(_) -> Option<B>` — the closure decides via
  `Some(value)`/`None` instead of `true`/`false`. Three methods.
  Smallest extension because the `Option<B>` payload is a
  structural rotation of lesson 119 + lesson 152. Two of the
  three are consumers (`find_map`); one is a lazy adapter
  (`filter_map` returns `FilterMap<I, F>`). Likely arc:
  `find_map` first (consumer, simpler), `filter_map` second
  (lazy adapter), `map_while` third (lazy with short-circuit
  on `None`).
- **Path B — deref-read installation arc** — small move
  installing `*x` read-through on shared references (lessons
  047/048 deferred this). After deref-read, **six** methods
  unlock: `filter`, `inspect`, `find`, `take_while`,
  `skip_while`, `partition`. After deref-read, the audit's
  intermediate iterator-pipeline capstone becomes ready
  (`(1..N).filter(...).map(...).fold(0, ...)`). This path
  unlocks more methods but requires installing one
  not-yet-installed mechanic first.
- **Path C — IntoIterator sub-arc** (audit §4.4.2) — opens
  nine more methods including `chain`, `zip` at trait depth,
  and the six lex-comparison methods. Standalone trait
  introduction; structurally orthogonal to closure mechanics.

**Recommended next:** Path A. The smallest extension by
structural-novelty count (one new payload-rotation of an
already-installed mechanic) and stays inside the established
discipline (no new prereqs).

## 10. Stop condition for this target

A future revision of this audit reports that no stable, non-experimental
Iterator method or load-bearing concept (`Item`, `IntoIterator`, FnMut
closure semantics, lazy-vs-consuming framing, short-circuit notes,
empty-iterator behavior) remains uninstalled, OR a major intermediate
capstone closes a large sub-arc cleanly.
