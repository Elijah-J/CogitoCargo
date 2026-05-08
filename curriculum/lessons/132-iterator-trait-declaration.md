---
id: 132-iterator-trait-declaration
status: accepted
evidence: ../evidence/132-iterator-trait-declaration.md
---

# Read the `std::iter::Iterator` trait declaration as a single structural unit

## The Move

Lesson 131 called `.next()` on the slice iterator from lesson 123.
The std doc page that licenses that mechanic opens with a *trait
declaration* — same `pub trait Name { ... }` shape as lesson 111.
Read its synopsis box and the iterator world fits on three lines:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ... 75 provided methods ...
}
```

(From `output/docs/rust/std/iter/trait.Iterator.md` lines 7-13, with
the 76 listed methods after `fn next` collapsed.) Two items —
`type Item;` and `fn next(...)` — are the trait's *required*
surface, sectioned later under `## Required Associated Types` and
`## Required Methods`. The other 75 — `count`, `last`, `nth`,
`take`, `skip`, `enumerate`, `map`, `filter`, `fold`, `collect`,
and so on — sit in the `// ...` slot and are *provided* (lesson
116): the trait ships default bodies; an impl may leave them out.

`type Item;` is the lesson-115 associated-type slot, resolved per
impl. `Self::Item` in the `next` signature is the lesson-115 path
through that slot. `&mut self` is lesson 101. `Option<Self::Item>`
is lesson 119 with `T = Self::Item`. The slice iterator from lesson
131 is one impl of this trait — its `type Item = &T` is what made
lesson 131's `Option<&u64>` appear.

## Mental Model Delta

- *Before:* "Lesson 131's `.next()` works on a slice iterator. I do
  not know what trait it belongs to or how `count`, `nth`, and the
  other Iterator methods fit in."
- *After:* "One trait, `std::iter::Iterator`, declares the iterator
  contract: two required items (`type Item;` + `fn next`) plus 75
  provided methods with default bodies. Lesson 131's `Option<&u64>`
  was `Option<Self::Item>` with `Self::Item = &u64`."

## Prerequisites

- Installed concepts:
  - **Lesson 131** (load-bearing): `.next()` returns `Option<&T>` on
    a slice iterator. Today's `Option<Self::Item>` generalizes the
    `&T`.
  - **Lesson 111** (load-bearing): `pub trait Name { ... } impl Name
    for Type { ... }`. Today reads a stdlib instance.
  - **Lesson 115** (load-bearing): `type IDENTIFIER;` in trait body,
    `Self::IDENTIFIER` in method signatures, resolved by
    `type IDENTIFIER = T;` in the impl.
  - **Lesson 116** (load-bearing): trait methods with `{ ... }`
    bodies are *defaults* an impl may leave out. Licenses today's
    75 provided methods being free for the impl.
  - **Lesson 119** (load-bearing): `Option<T>` / `Some` / `None`.
    Today's `T` is the `Self::Item` slot.
  - **Lesson 101** (load-bearing): `&mut self` receiver.
  - **Lessons 040, 011, 001, 002, 003, 005, 095** (cited): dot-call;
    `println!`; rustc compile + run; `fn main`; diagnostic map;
    `let`; `struct` with named fields.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save as `demo.rs`, compile, run. The probe defines a user-defined
iterator from scratch — no slice — to witness that the required
surface is *exactly* `type Item` and `fn next`:

```rust
struct Counter {
    value: u32,
    limit: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value < self.limit {
            let v = self.value;
            self.value += 1;
            Some(v)
        } else {
            None
        }
    }
}

fn main() {
    let mut c = Counter { value: 0, limit: 3 };
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());
}
```

`rustc demo.rs` is silent. `./demo` prints:

```text
Some(0)
Some(1)
Some(2)
None
```

The impl supplies *only* `type Item = u32;` and the `fn next` body
— the trait's required surface. Yet `c.count()`, `c.last()`,
`c.nth(1)` and 70-odd more methods are also callable: replace the
four `println!` lines with `println!("count = {}", c.count());` and
recompile — output is `count = 3`. The provided methods are free
because the trait ships default bodies (lesson 116).

*Now the contrast.* Delete the line `type Item = u32;`:

```text
error[E0046]: not all trait items implemented, missing: `Item`
 --> no_item.rs:6:1
  |
6 | impl Iterator for Counter {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^ missing `Item` in implementation
  |
  = help: implement the missing item: `type Item = /* Type */;`
```

Read with the lesson 003 map. Headline `E0046` — same code as
lessons 115 and 116. The label `missing `Item` in implementation`
and the help line `type Item = /* Type */;` name the absent item.
Dropping `fn next` instead fires the same E0046 with `missing:
\`next\`` and a help line writing the required signature. Either
required item, dropped, triggers the same shape.

## What Changed

- `std::iter::Iterator`'s required surface is exactly two items:
  `type Item;` and `fn next(&mut self) -> Option<Self::Item>;`. The
  other 75 methods listed in the synopsis box are *provided*.
- Provided methods carry default bodies (lesson 116). An impl that
  fills only the required surface inherits the full method set.
- `Self::Item` (lesson 115) is the return-type slot of `next`.
  Lesson 131's `&u64` was the slice iterator's `type Item = &T`
  resolved.
- Dropping either required item fires `error[E0046]: missing:
  \`Item\`` (or `\`next\``). Provided methods cannot trigger E0046
  from absence — they are inherited.

## Check Yourself

Predict, then run.

```rust
struct Two;

impl Iterator for Two {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(7)
    }
}

fn main() {
    let mut t = Two;
    println!("{:?}", t.next());
    println!("{:?}", t.next());
}
```

(a) Does it compile silently? What does it print?

(b) You delete `type Item = i32;`. What E-code, and which identifier
does the headline name as missing?

(c) Restore `type Item = i32;`, then delete the entire `fn next`
block instead. Same E-code? What is missing now?

(d) The trait lists `count`, `last`, `nth`, and 72 more method
signatures. Is your impl missing those too?

*(Answers: (a) Yes; `Some(7)` then `Some(7)` — the iterator never
returns `None`. (b) E0046; `missing: \`Item\``. (c) Same E0046;
`missing: \`next\``. (d) No: the other 75 are *provided* — the trait
ships default bodies (116) and the impl inherits them.)*

## What To Ignore For Now

Deferred: each of the 75 provided methods (each is its own future
move per the iterator-API audit); stable-vs-nightly stability split;
`Sized` / `?Sized` and the `where Self: Sized` bound on most
provided-method signatures; lifetimes (the `'a` in `&'a T`);
supertrait colon-form (`trait FusedIterator: Iterator`); generic
associated types; `IntoIterator` and the `for x in v` desugar;
closure machinery (gates `map`, `filter`, `fold`, `for_each`, etc.);
the implementor list further down the page; the formal "lazy
adapter" / "consumer" definitions.

## Evidence

See `../evidence/132-iterator-trait-declaration.md`.
