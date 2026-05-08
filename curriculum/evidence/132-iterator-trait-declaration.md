# Evidence — Lesson 132: read the `Iterator` trait declaration as one unit

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/132-iterator-trait-declaration.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/132-iterator-trait-declaration.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/132-iterator-trait-declaration.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into a fresh scratch directory (`/tmp/eduratchet132/`)
and compiled with `rustc <file>`; resulting executables were run from
the same directory. Same host and toolchain as accepted lessons through
131.

## Direct prerequisite — lesson 131 (`iter.next()` on a slice iterator)

Lesson 131 installed:

- A slice iterator (`v.iter()`) has a method `.next()` that pulls one
  element. Calling it advances the cursor; once exhausted, calls
  return `None`.
- The return wrapper is `Option<&T>`, witnessed by the type-pin probe
  `let first: Option<&u64> = iter.next();`.
- Dropping `mut` from the iterator binding fires `E0596 cannot borrow
  ... as mutable` — the empirical witness for the `&mut self`
  receiver in `Iterator::next`'s declaration.

Today re-frames lesson 131's mechanic as the *required method* of
`std::iter::Iterator`. The `Option<&T>` return wrapper becomes
`Option<Self::Item>` with `Self::Item = &T` for that specific impl.
Lesson 131's *What To Ignore For Now* explicitly lists "the
`Iterator` trait declaration itself (next in the arc); `Self::Item`
and `Item = &'a T`" as deferred — today executes that named-deferred
move per the iterator-API audit §5 step 2.

## Direct prerequisite — lesson 111 (`pub trait Name { ... } impl ... for ... { ... }`)

Lesson 111 installed:

- `trait Name { fn method(&self) -> T; }` declares a trait carrying
  one method *signature* whose body is replaced by `;`.
- `impl Name for Type { fn method(&self) -> T { ... } }` is a *trait
  impl* — a different `impl` shape from the inherent `impl Type` of
  lesson 100.
- The signature in the impl must match the signature in the trait.

Today reads the stdlib version of that shape: the `pub trait
Iterator { ... }` block at `output/docs/rust/std/iter/trait.Iterator.md`
lines 7-13 is the same `pub trait Name { ... }` shape lesson 111
introduced. The user-defined `impl Iterator for Counter { ... }` in
the working probe is the same `impl Name for Type { ... }` shape.

## Direct prerequisite — lesson 115 (`type IDENTIFIER;` and `Self::IDENTIFIER`)

Lesson 115 installed:

- A trait body may declare a *required associated type* with
  `type IDENTIFIER;` (semicolon, no body).
- `Self::IDENTIFIER` is the path through the trait's `Self` to that
  associated type; it can sit anywhere a type is expected.
- An impl resolves the associated type with
  `type IDENTIFIER = ConcreteType;` *inside* the impl body.
- Without that resolution line, rustc fires
  `error[E0046]: not all trait items implemented, missing: \`IDENTIFIER\``.

Today's `type Item;` in the trait declaration is exactly lesson 115's
shape. `Self::Item` in `next`'s signature is the lesson-115 path. Per
impl, `type Item = T;` resolves it. Probe 4 confirms that dropping
the `type Item = u32;` line from a user-defined impl fires the same
E0046 with `missing: \`Item\``, just as lesson 115's contrast did
with `missing: \`Output\``.

## Direct prerequisite — lesson 116 (default method body and empty impl)

Lesson 116 installed:

- A trait method's body slot can be either `;` or `{ ... }`. The
  `{ ... }` form is a *default* body the impl may either accept by
  leaving the method out, or override.
- `impl Trait for Type {}` (empty body) is legal exactly when every
  method the trait declared either has a default body or is filled
  by the impl's own `fn` lines.

Today this is the licensor for the 75 provided methods in the
Iterator trait declaration: each provided method has a `{ ... }`
default body in the trait, so an impl that fills only `type Item`
and `fn next` inherits all of them automatically. Probe 2 witnesses
this empirically: the impl writes only the two required items, yet
`c.count()` is callable.

## Direct prerequisite — lesson 119 (`Option<T>` / `Some` / `None`)

Lesson 119 installed:

- `pub enum Option<T> { None, Some(T) }`. Both variants are in the
  prelude.
- `Some(value)` wraps a value into `Option<T>`; `None` is the
  payload-free variant.

Today's `Option<Self::Item>` is lesson 119 with `T = Self::Item`.
The working probe's `Some(v)` and `None` arms are the lesson-119
constructors with `T = u32`.

## Direct prerequisite — lesson 101 (`&mut self` receiver)

Lesson 101 installed:

- `&mut self` is the third receiver shape, after no-receiver and
  `&self`. It is the receiver-shorthand for `self: &mut Self`.
- A method declared with `&mut self` mutates the receiver and
  requires the call-site binding to be `mut`.

Lesson 131 saw `&mut self` empirically through the E0596 contrast.
Today sees it written into the trait declaration directly: line 11
of the synopsis at `trait.Iterator.md` and line 277 of the per-method
section both spell `fn next(&mut self) -> Option<Self::Item>`.

## Older supporting lessons

- **Lesson 040** (cited) — dot-call shape `v.method()`. Used in the
  working probe's `c.next()` and `c.count()`.
- **Lesson 095** (cited) — `struct Name { field: Type }` with named
  fields. The working probe's `Counter { value: u32, limit: u32 }`
  is that shape.
- **Lessons 011, 001, 002, 003, 005** (cited) — `println!` with
  positional placeholder; rustc compile + run; `fn main`; the
  diagnostic four-part map; `let`. Same roles as lessons 131, 130,
  129.
- **Lesson 123** (cited) — `v.iter()` returns a slice iterator
  yielding `&T`. Today refers back to it once when explaining where
  lesson 131's `&u64` came from (`type Item = &'a T` for the slice
  iterator's impl, per `struct.Iter.md:199`); lifetimes still
  deferred.

## Probe 1 — working probe (user-defined Iterator impl, required surface only)

Source committed at
`experimental/eduratchet2/runs/rust-moves/observations/132-iterator-trait-declaration.rs`:

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

Transcript:

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
Some(0)
Some(1)
Some(2)
None
$ echo "run-exit=$?"
run-exit=0
```

The centered claim — "filling in only `type Item` and `fn next` is
sufficient to satisfy the `Iterator` trait" — is carried by the silent
compile. This is *not* a slice iterator probe (lesson 131 covered
that); using a *user-defined* type witnesses that the trait surface,
not anything specific to slices, is what `next` requires. The four
output lines mirror the std doc example at
`output/docs/rust/std/iter/trait.Iterator.md:288-303` (three `Some`,
then `None`).

## Probe 2 — provided method `count()` reachable

Source: `provided.rs` (transcript file, full source there). Same
Counter, same impl supplying only `type Item` and `fn next`. Body of
`fn main` replaced by a single `println!("count = {}", c.count());`.

Transcript:

```text
$ rustc provided.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./provided
count = 3
```

Witnesses: the impl supplies *only* the required surface; the provided
method `count()` (one of the 75 provided methods listed in the
trait's synopsis box) is callable with no additional impl-body line.
This grounds the lesson's claim "the provided methods come for free
because the trait ships default bodies (lesson 116)."

## Probe 3 — type-pin (positive)

Source: `typetest.rs`. Same impl as Probe 1, with main body:

```rust
let mut c = Counter { value: 0, limit: 3 };
let first: Option<u32> = c.next();
println!("{:?}", first);
```

Transcript:

```text
$ rustc typetest.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./typetest
Some(0)
```

Witnesses: with `type Item = u32;` resolved, `c.next()` empirically
has type `Option<u32>` (silent compile of the type-annotated `let`).
This is lesson 115's "after `type Output = u32;` resolves,
`Self::Output` is `u32`" rule applied to the stdlib `Iterator` trait's
`Self::Item` slot. No new mechanism — composition only.

## Probe 4 — centered contrast (drop `type Item`)

Source: `no_item.rs`. Same probe with the line `type Item = u32;`
deleted from the impl body. Verbatim transcript:

```text
$ rustc no_item.rs
error[E0046]: not all trait items implemented, missing: `Item`
 --> no_item.rs:6:1
  |
6 | impl Iterator for Counter {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^ missing `Item` in implementation
  |
  = help: implement the missing item: `type Item = /* Type */;`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0046`.
exit=1
```

The centered contrast for "`type Item` is *required*, not optional".
The diagnostic format is the lesson-115 / lesson-116 multi-line E0046
shape; the help line literally writes the fix. This is one of the two
empirical witnesses for the trait's required surface being *exactly*
two items (Probe 5 is the other half).

## Probe 5 — corroborating contrast (drop `fn next`)

Source: `no_next.rs`. Same probe with the entire `fn next` block
deleted, leaving only `type Item = u32;` in the impl body. Verbatim
transcript:

```text
$ rustc no_next.rs
error[E0046]: not all trait items implemented, missing: `next`
 --> no_next.rs:6:1
  |
6 | impl Iterator for Counter {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next` in implementation
  |
  = help: implement the missing item: `fn next(&mut self) -> Option<<Self as Iterator>::Item> { todo!() }`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0046`.
exit=1
```

Witnesses: `fn next` is *required*, not optional. The help line spells
the required signature; note the qualified-path spelling `<Self as
Iterator>::Item` for `Self::Item` in the rustc help output — that is
how rustc disambiguates the path when no `type Item =` line yet
exists in the impl. Together with Probe 4, this sandwiches the
required-surface claim: drop *either* of the two required items, get
E0046 with the missing item named.

The 75 provided methods cannot trigger E0046 from absence — they are
the contrast against this rule. That is the lesson's operational
definition of "provided" vs "required".

## Why this works — std grounding

### `output/docs/rust/std/iter/trait.Iterator.md` lines 6-13 (synopsis box)

Verbatim:

```
pub trait Iterator {
    type Item;

Show 76 methods    // Required method
    fn next(&mut self) -> Option<Self::Item>;

    // Provided methods
    fn next_chunk<const N: usize>(
        &mut self,
    ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>
       where Self: Sized { ... }
    fn size_hint(&self) -> (usize, Option<usize>) { ... }
    fn count(self) -> usize
       where Self: Sized { ... }
    ...
```

(The synopsis box continues for many more lines; lines 14 onward list
the 75 provided methods. Today's lesson collapses them to `// ... 75
provided methods ...` because each is its own future move.)

This is the authoritative source for:

- the **trait header**: `pub trait Iterator {`. Same `pub trait Name {`
  shape as lesson 111; no supertrait colon, no generic parameter, no
  `where`-clause on the trait line itself.
- the **required associated type**: `type Item;` — semicolon, no
  body. Lesson-115 shape.
- the **required method**: `fn next(&mut self) -> Option<Self::Item>;`
  — semicolon, no body. Lesson-111 shape with lesson-115's
  `Self::Item` filling the return-type slot and lesson-101's
  `&mut self` receiver.
- the **provided/required split**: the comment `// Required method`
  precedes `fn next` and the comment `// Provided methods` precedes
  every other listed method.

The "Show 76 methods" string is the doc page's collapsed-by-default
indicator: 76 methods total = 1 required (`next`) + 75 provided. The
audit at `iterator-api-coverage.md` §1 splits the 75 provided into
60 stable + 15 nightly; stability is named-deferred today.

### `output/docs/rust/std/iter/trait.Iterator.md` lines 265-277 (per-item section headers)

Verbatim:

```
## Required Associated Types[§](#required-associated-types)

1.0.0 ·

#### type [Item](#associatedtype.Item)

The type of the elements being iterated over.

## Required Methods[§](#required-methods)

1.0.0 ·

#### fn [next](#tymethod.next)(&mut self) -> [Option](../option/enum.Option.md "enum std::option::Option")<Self::[Item](trait.Iterator.md#associatedtype.Item "type std::iter::Iterator::Item")>
```

This is the doc page's expansion of the synopsis box's required surface.
Two header sections — `## Required Associated Types` and
`## Required Methods` — each containing exactly one item. This
licenses the lesson's claim that the trait's required surface is
*exactly* two items (1 + 1 = 2). The 75 provided methods later live
under a `## Provided Methods` header (per page structure; not quoted
here because the count alone is load-bearing today, not the headers).

The `1.0.0 ·` markers on both required items establish that they have
been the trait's required surface since Rust 1.0; no stability story
to teach today.

### `output/docs/rust/error_codes/E0046.md`

Probes 4 and 5 produce `E0046` blocks. The error code documents the
"not all trait items implemented" rule. Lessons 115 and 116 already
installed the diagnostic shape; today reuses it on a stdlib trait
without re-teaching the format.

### Lesson 131's evidence — `output/docs/rust/std/slice/struct.Iter.md:195-207`

Already cited in lesson 131 to license `iter.next()` returning
`Option<&T>`. Today references it once: that page's

```
impl<'a, T> Iterator for Iter<'a, T>
    type Item = &'a T
    fn next(&mut self) -> Option<&'a T>
```

is the slice-iterator's *implementation* of today's trait declaration.
`type Item = &'a T` is the impl's resolution of the lesson-115 slot;
`fn next(&mut self) -> Option<&'a T>` is `Self::Item` resolved to
`&'a T`. The lifetime `'a` stays deferred (since lesson 123).

## Claim-to-evidence map

- "`std::iter::Iterator`'s required surface is exactly two items:
  `type Item;` and `fn next(&mut self) -> Option<Self::Item>;`" —
  `trait.Iterator.md:265-277` (two sibling headers `## Required
  Associated Types` and `## Required Methods` with one item each);
  `trait.Iterator.md:6-13` (synopsis box's `// Required method`
  comment precedes only `next`; `// Provided methods` precedes all
  others); Probes 4 and 5 (drop either item, E0046 fires; drop a
  provided method like `count`, no error possible because the impl
  never had to supply it).
- "All other methods are provided" — `trait.Iterator.md:13` comment
  `// Provided methods` precedes the 75 entries that follow `next`;
  Probe 2 (impl supplies only the required surface; `c.count()` is
  callable).
- "76 methods total = 1 required + 75 provided" —
  `trait.Iterator.md:10` "Show 76 methods" string; minus `next`
  (the only entry under `// Required method`) leaves 75; the
  iterator-API audit's §1 confirms.
- "`type Item;` is the lesson-115 associated-type slot" — lesson 115
  installed `type IDENTIFIER;` syntax in trait body and `Self::IDENTIFIER`
  path in method signatures; `trait.Iterator.md:8` and `:269` write
  exactly that shape with `IDENTIFIER = Item`.
- "`Self::Item` in `next`'s signature is resolved per impl" — lesson
  115 installed the rule "after `type Output = u32;` resolves,
  `Self::Output` is `u32`"; Probe 3 witnesses it on
  `Iterator::Self::Item` (silent compile of `let first: Option<u32>
  = c.next();`).
- "`fn next(&mut self) -> Option<Self::Item>` composes lesson 101's
  `&mut self`, lesson 119's `Option<T>`, and lesson 115's
  `Self::Item`" — each prerequisite installed its piece; today does
  the composition; `trait.Iterator.md:11` and `:277` are the
  authoritative spelling.
- "75 provided methods carry default bodies (lesson 116) so an impl
  that fills only the required surface inherits them" — lesson 116
  installed default-body mechanic; `trait.Iterator.md:13-...` shows
  every non-`next` method line ending in `{ ... }` (default-body
  shape); Probe 2 confirms empirically.
- "Slice iterator's `Option<&T>` from lesson 131 is `Option<Self::Item>`
  with `Self::Item = &T`" — `struct.Iter.md:199` `type Item = &'a T`;
  lesson 131 evidence appendix cites this. Today re-states it once
  inside The Move; lifetime still deferred.
- "Drop `type Item` from a user-defined impl, get E0046 with
  `missing: \`Item\`` and a help line writing `type Item = /* Type
  */;`" — Probe 4 verbatim transcript.
- "Drop `fn next`, get E0046 with `missing: \`next\`` and a help line
  writing the required signature" — Probe 5 verbatim transcript.

## Negative / contrast probe coverage

Two contrasts captured. Both needed because the lesson's centered
claim is *exactly two items required*:

- **Probe 4 (E0046 on missing `type Item`)** is the first half of the
  centered contrast. It witnesses that `type Item` is required, and
  the diagnostic literally writes the fix.
- **Probe 5 (E0046 on missing `fn next`)** is the second half. It
  witnesses that `fn next` is also required. Without both halves, the
  claim "required surface is *exactly* `type Item` and `fn next`"
  would only be half-grounded — Probe 4 alone could not rule out
  "maybe only `type Item` is required and you're free to drop `next`".

The 75 provided methods cannot trigger E0046 from absence (they are
inherited from default bodies). Probe 2's positive witness — calling
a provided method on an impl that never supplied it — is the
contrastive complement to Probes 4 and 5: required items must be
supplied; provided items must not be supplied (they are inherited).

## Iterator API audit alignment

This lesson is step 2 of the audit's first-arc plan
(`experimental/eduratchet2/runs/rust-moves/iterator-api-coverage.md`
§5):

> 2. **`Iterator` trait declaration** — read `pub trait Iterator { type
>    Item; fn next(&mut self) -> Option<Self::Item>; }` structurally.
>    Composes 111-116 + 115 + 119 + step 1. Anchors every later
>    "this method is on `Iterator`" claim.

The audit pre-committed to teaching this immediately after lesson 131
(step 1) so that every subsequent Iterator move (`count`, `last`,
`nth`, `take`, `skip`, `enumerate`, `fuse`, `step_by`, `size_hint`,
audit §5 steps 3-11) can refer to "the Iterator trait declaration"
without re-installing the structural read. Today's lesson is the
anchor those moves point back to.
