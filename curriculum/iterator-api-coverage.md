# `std::iter::Iterator` API Coverage Audit — v1

## 0. Scope and dating

- Audit captured 2026-05-08 against
  `/Users/eli/InfoScraper/output/docs/rust/std/iter/trait.Iterator.md`
  (5114 lines, source URL
  https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html).
- Toolchain on host: `rustc 1.95.0` / `cargo 1.95.0` (same toolchain
  the run has been probing on since lesson 001).
- Branch: `eduratchet/rust-moves`. Last accepted node: `130-capstone-rmp-cmp-ordering`
  (commit `b86783c21`). Total accepted nodes: 130.
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

## 4. Coverage by accepted graph (130 nodes)

### 4.1 Items already exercised at quiz depth — 0

No item in §2 has been formally installed as a centered Iterator move
with quiz-shaped semantic surface (consumes-`self` vs `&mut self`,
lazy-vs-consuming, short-circuit, empty-iterator behavior, panic
notes) anchored to the `Iterator` trait surface. Every prior iterator
lesson teaches a *concrete spelling* (e.g. `v.iter().rev()` on a
slice) without naming the underlying `Iterator::rev` method or its
trait bounds.

### 4.2 Partially exercised — 5 items

| Iterator item | Existing coverage | Gap to quiz |
|---|---|---|
| `Item` | 115-trait-associated-type installs the `type X;` syntax inside trait declarations | The specific `type Item;` line and its consumption in `next`'s signature is not anchored. |
| `next` | 119 installs `Option<T>` + `Some/None`. 022/039/079/123/126 use `for x in expr` over various concrete iterables. | The `.next()` *call*, repeated, and the `Option<&T>` return for slice iterators are uninstalled. The `&mut self` receiver of `next` is the new fact. |
| `rev` | 091 installs `(0..N).rev()` on `Range<i32>`. 124 installs `slice::Iter<'_, T>::rev`. | Not yet stated as a method on the `Iterator` *trait*; the `Self: Sized + DoubleEndedIterator` bound and the `Rev<Self>` adapter struct are uninstalled. |
| `zip` | 125 installs `v.iter().zip(w.iter())` for slice iterators. | The `IntoIterator`-shaped argument and the shortest-source rule are present in the lesson body but not anchored to the `Iterator::zip` signature. |
| Implicit `for x in expr` desugaring | 022/039/079/126 use `for` over `Range`, `&[T; N]`, `Vec::iter`. | The desugaring rule from the Reference (`for x in expr` ≡ `for x in IntoIterator::into_iter(expr)`) is named-deferred since 022. |

### 4.3 Ready now (no new prereqs) — 11 items

These can land as small lessons directly on top of the current graph,
without a closure arc, an `IntoIterator` arc, a Try arc, or an Ord/
Sum/Clone-bound arc.

| Iterator item | Why ready | Composes |
|---|---|---|
| `next` (call) | `let mut iter = v.iter(); iter.next()` returns `Option<&T>` | 119 + 123 + 006 |
| `Iterator` trait declaration | `pub trait Iterator { type Item; fn next(&mut self) -> Option<Self::Item>; }` read structurally | 111-116 + 115 + 119 + above |
| `count` | `(self) -> usize` | self-by-value 102 + 119 |
| `last` | `(self) -> Option<Self::Item>` | self-by-value 102 + 119 |
| `nth` | `(&mut self, n: usize) -> Option<Self::Item>` | `&mut self` 101 + 119 + 080 |
| `take` | `(self, n) -> Take<Self>`, lazy | self-by-value 102 + 080 |
| `skip` | `(self, n) -> Skip<Self>`, lazy | self-by-value 102 + 080 |
| `enumerate` | `(self) -> Enumerate<Self>` yielding `(usize, Item)` | self-by-value 102 + 072/073 (tuple) + 080 |
| `fuse` | `(self) -> Fuse<Self>` sticky-`None` rule | self-by-value 102 + 119 |
| `step_by` | `(self, step) -> StepBy<Self>`, panics on `step == 0` | self-by-value 102 + 080 |
| `size_hint` | `(&self) -> (usize, Option<usize>)` | 100 (`&self`) + 072/073 (tuple) + 119 |

Each of these is small enough to land as a single lesson without
splitting. Several share probes (e.g. all of `count`/`last`/`take`/
`skip` can drive a `Vec<u64>` literal through `.iter()` — already
installed at 123).

### 4.4 Requires prereqs

#### 4.4.1 Closure sub-arc — gates 27 methods

`map`, `for_each`, `filter`, `filter_map`, `skip_while`, `take_while`,
`map_while`, `scan`, `flat_map`, `inspect`, `partition`, `try_fold`,
`try_for_each`, `fold`, `reduce`, `all`, `any`, `find`, `find_map`,
`position`, `rposition`, `max_by_key`, `max_by`, `min_by_key`,
`min_by`, `is_sorted_by`, `is_sorted_by_key`.

The closure arc is its own multi-lesson sub-arc. Anticipated steps
(not prescriptive): closure literal `|x| x + 1`; explicit-annotation
form `|x: i32| -> i32 { x + 1 }`; closure capturing an outer binding;
closure as `FnMut`-bound parameter on a function; the
`FnOnce`/`FnMut`/`Fn` distinction. Without this arc, none of the 27
closure-taking methods can be taught honestly.

#### 4.4.2 `IntoIterator` sub-arc — gates 9 methods

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

#### 4.4.4 Try sub-arc — gates 2 stable methods

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

## 5. First-arc plan

Bottom-up sequence to take the trait from "named-deferred since 119"
to "structurally readable end-to-end":

1. **`iter.next()` call** — call `.next()` on a slice iterator,
   observe `Option<&T>`, four calls reach `None`. Composes
   123 + 119 + 006. Single new fact: `next` is callable, takes
   `&mut self`. *Highest-priority ready-now move.*
2. **`Iterator` trait declaration** — read `pub trait Iterator { type
   Item; fn next(&mut self) -> Option<Self::Item>; }` structurally.
   Composes 111-116 + 115 + 119 + step 1. Anchors every later
   "this method is on `Iterator`" claim.
3. **`count`** — smallest stable consumer, names the
   call-`next`-until-`None`-and-count semantic, names self-by-value
   consuming. Carries `usize::MAX` overflow note.
4. **`last`** — small consumer, names the infinite-iterator panic
   trigger.
5. **`nth`** — `&mut self` consumer, drops preceding elements.
6. **`take`** — small lazy adapter, returns `Take<Self>`.
7. **`skip`** — small lazy adapter, returns `Skip<Self>`.
8. **`enumerate`** — yields `(usize, Item)`; `usize::MAX` overflow note.
9. **`fuse`** — sticky-`None` rule; the contrast probe writes a
   custom iterator that resumes after `None`.
10. **`step_by`** — `step != 0` panic.
11. **`size_hint`** — observation method, tuple result.

After step 11 the closure-free non-consumer Iterator surface is
covered. Then start the **closure sub-arc** (4.4.1). Then the
**IntoIterator sub-arc** (4.4.2). Then the **bounded-by-other-trait
sub-arc** (4.4.3). Then the **Try sub-arc** (4.4.4). Sub-arcs may
proceed in parallel where their prereqs allow.

Likely intermediate capstone after the closure sub-arc: a small
realistic iterator pipeline (`v.iter().filter(...).map(...).collect()`)
read end-to-end.

Likely final capstone for this target: a single `Iterator` quiz with
N questions sampled across `kind`, `recv`, lazy/consuming, short-
circuit, and bounds.

## 6. Stop condition for this target

A future revision of this audit reports that no stable, non-experimental
Iterator method or load-bearing concept (`Item`, `IntoIterator`, FnMut
closure semantics, lazy-vs-consuming framing, short-circuit notes,
empty-iterator behavior) remains uninstalled, OR a major intermediate
capstone closes a large sub-arc cleanly.
