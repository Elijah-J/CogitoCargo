---
id: 131-iterator-next-call
status: accepted
evidence: ../evidence/131-iterator-next-call.md
---

# Pull elements from a slice iterator one at a time with `iter.next()`

## The Move

Lesson 123 fed `v.iter()` straight into a `for`-loop. The same iterator
has a method `.next()` that pulls *one* element. Bind the iterator to a
name, then call `.next()` once per element you want.

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let mut iter = v.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
```

`rustc demo.rs` is silent (exit 0). `./demo` prints four lines:

```text
Some(10)
Some(20)
Some(30)
None
```

`let mut iter = v.iter();` binds the iterator to a reassignable name
(006 + 123). The `mut` is required — the contrast probe shows why.
Each `iter.next()` is the dot-call shape (lesson 040). Each call hands
back the *next* element — wrapped — and advances the iterator's
internal cursor. Once every element has been yielded, calls return
`None`. Four calls on a three-element vec: three `Some(_)` payloads in
vec order, then `None`.

The wrapper is `Option<&u64>`. The std `Iterator::next` declaration is
`fn next(&mut self) -> Option<Self::Item>`; for the slice iterator,
`Item = &u64`. `{:?}` (lesson 093) prints the inner reference without
a `&` for primitive types — empirical output is `Some(10)`, not
`Some(&10)`. The appendix type-pin probe `let first: Option<&u64> =
iter.next();` compiles silently, witnessing the actual return type.

## Mental Model Delta

- *Before:* "`for x in v.iter()` walks the elements, but the loop is
  the only shape I know. I cannot ask the iterator for *one* element
  on demand. `Option<T>` (119) and iterators (123) are separate ideas."
- *After:* "An iterator has a method `.next()` that takes `&mut self`
  and returns `Option<&T>`. Each call yields the next element wrapped
  in `Some(&...)`; once exhausted, calls return `None`. The iterator
  carries a cursor — that is why the binding has to be `mut`, and it
  is why the `for`-loop walks each element exactly once. The loop is
  one way to drive `.next()`; explicit calls are another. Lessons 119
  and 123 compose into one shape: `Option<&T>`."

## Prerequisites

- Installed concepts:
  - **Lesson 123** (load-bearing): `v.iter()` returns a slice iterator
    yielding `&T`. Today binds it and pulls elements with `.next()`.
  - **Lesson 119** (load-bearing): `Option<T>` / `Some` / `None`.
    Today's `T` is `&u64`, so the wrapper is `Option<&u64>`.
  - **Lesson 006** (load-bearing): `let mut name = value;`. `.next()`'s
    receiver is `&mut self`; the binding must be `mut`. Contrast probe
    fires E0596 with a `help:` proposing `let mut iter`.
  - **Lesson 040** (load-bearing): `value.method(args)`. `iter.next()`
    is that shape with empty arg list.
  - **Lesson 093** (cited): `{:?}` Debug placeholder — `Option<&u64>`
    Debug-prints as `Some(10)` / `None`.
  - **Lessons 011, 001, 002, 003, 005, 080, 019, 107** (cited):
    `println!`; rustc compile + run; `fn main`; diagnostic four-part
    map; `let`; `u64`; `: TYPE`; `Vec<T>` / `vec![...]`.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the source above as `demo.rs`, compile and run; output is the
four lines shown in *The Move*.

Now the contrast. `.next()` borrows the iterator as mutable. Save
`broken.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let iter = v.iter();
    println!("{:?}", iter.next());
}
```

Compile:

```text
error[E0596]: cannot borrow `iter` as mutable, as it is not declared as mutable
 --> broken.rs:4:22
  |
4 |     println!("{:?}", iter.next());
  |                      ^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
3 |     let mut iter = v.iter();
  |         +++
```

Read with the lesson 003 map. **Headline** `E0596`. **Location**
`broken.rs:4:22`, on `iter`. **Source excerpt** carets `iter` and
labels it `cannot borrow as mutable`. **Help** literally writes
today's fix: `let mut iter = v.iter();`, `+++` marking where to insert
`mut`. The diagnostic states why `mut` is required: `.next()` needs
to *borrow `iter` as mutable*.

## What Changed

- A slice iterator has a method `.next()`; call it with the dot-call
  shape `iter.next()`.
- Return type is `Option<&T>`: `Some(&value)` per element, then `None`
  when exhausted.
- `{:?}` prints `Some(10)` not `Some(&10)` — the `&` is invisible in
  Debug for primitive types, but the type still carries the `&`
  (appendix Probe 4 + 5 pin this).
- `.next()` takes `&mut self`; binding must be `let mut iter`.
  Dropping `mut` fires `E0596 cannot borrow \`iter\` as mutable`.
- For slice iterators, once `None` is returned every subsequent call
  returns `None` — exhausted means `None` forever.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let xs: Vec<u64> = vec![5, 6];
    let mut it = xs.iter();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile silently? What three lines does it print?

(b) You drop the `mut` from line 3, leaving `let it = xs.iter();`. What
E-code does rustc emit, and what does its `help:` line propose?

(c) On the third call, what value does `.next()` return? Why?

*(Answers: (a) Yes; `Some(5)`, `Some(6)`, `None`. (b) E0596 — `cannot
borrow \`it\` as mutable`. `help:` proposes `let mut it = xs.iter();`.
(c) `None`. The vec has two elements; the first two calls return
`Some(&5)` and `Some(&6)`; after that the iterator is exhausted.)*

## What To Ignore For Now

Deferred: the `Iterator` trait declaration itself (next in the arc);
`Self::Item` and `Item = &'a T`; the `'a` lifetime; iterators that
resume after `None` (`Fuse<I>`); other consumer methods (`.count()`,
`.last()`, `.nth(n)`, `.collect()`, `.sum()`, `.fold(...)`); other
adapters (`.map()`, `.filter()`, `.enumerate()`, …) — most need
closures; `while let Some(x) = iter.next()`; `iter_mut()` /
`into_iter()`; the `impl<T: Debug + ?Sized> Debug for &T` formatter
rule that makes `{:?}` print `Some(10)` not `Some(&10)`.

## Evidence

See `../evidence/131-iterator-next-call.md`.
