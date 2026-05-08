# Evidence — Lesson 116: trait method default body

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/116-trait-default-method-body.md`
Observation: `experimental/eduratchet2/runs/rust-moves/observations/116-trait-default-method-body.rs`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

All four probes below were typed into a fresh scratch directory
(`/tmp/eduratchet2-116/`) and compiled with `rustc <file>`; the
resulting executables were run from the same directory.

## Direct prerequisite — lesson 111

Lesson 111 installed:

- `trait Name { fn method(&self) -> T; }` — the trait declaration
  with one method whose *body slot* is filled by `;`. The Reference
  quote in 111: "Trait functions may omit the function body by
  replacing it with a semicolon. This indicates that the
  implementation must define the function." Today extends 111 by
  filling the body slot with `{ ... }` instead — a *default body* —
  which the impl may either accept (empty `{}` impl body) or
  override (provide its own `fn` line).
- `impl Trait for Type { ... }` — the trait impl block whose body
  contains the bodies the trait declared. Today's working probe puts
  *nothing* inside this block (`impl Greeting for Counter {}`)
  because the trait's default body covers the only method.
- The dot-call shape `c.greet()` resolving to a trait method.
- E0599 "no method named" — what fires when no impl exists at all.
  Lesson 111's What To Ignore For Now line names **default method
  bodies** explicitly: "`fn method(&self) { ... }` (curlies, not
  `;`) inside the trait acts as a default the impl may override."
  Today is exactly that move.

## Direct prerequisite — lesson 115

Lesson 115 installed:

- E0046 "not all trait items implemented, missing: ..." with the
  multi-`-->` shape. The headline names the missing item (an
  associated type in 115; an associated function today). Today's
  optional sharpening probe (Probe 3) reuses the same E0046 code on
  a different missing item kind — empirical witness that the same
  diagnostic frames "the trait declared an item the impl did not
  provide" across both function and type item kinds.

## Older supporting lessons

- **Lesson 008** — `fn name(&self) -> T { ... }` shape with a
  body. Today's default body is the same shape, just written
  inside the trait body.
- **Lesson 095** — `struct Name { field: Type, }`, the struct
  expression `Name { field: value }`, field access `value.field`.
- **Lesson 100** — `&self` as the borrowing receiver inside an
  `impl` block.
- **Lesson 040** — the dot-call shape `c.method()`.
- **Lesson 080** — `u32` as a member of the integer family.
- **Lesson 011** — `println!("...{}", value)` with one positional
  `{}` slot.
- **Lesson 005** — `let c = Counter { count: 7 };`.
- **Lesson 002, 001** — `fn main` entry point; `rustc demo.rs`
  then `./demo`.
- **Lesson 003** — the four-part diagnostic map. Probe 3's
  E0046 transcript follows the same shape as 115's E0046 transcript.

## Probe 1 — working probe (default body accepted by empty `{}` impl)

```rust
struct Counter {
    count: u32,
}

trait Greeting {
    fn greet(&self) -> u32 {
        100u32
    }
}

impl Greeting for Counter {}

fn main() {
    let c = Counter { count: 7 };
    println!("c.count = {}", c.count);
    println!("greet   = {}", c.greet());
}
```

Compile and run on host:

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
c.count = 7
greet   = 100
$ echo "run-exit=$?"
run-exit=0
```

The trait declaration's `fn greet(&self) -> u32 { 100u32 }` has
curly braces *in the body slot* instead of `;`. The Reference at
`output/docs/rust/reference/items/traits.md` lines 43-44 states
verbatim: "If the trait function defines a body, this definition
acts as a default for any implementation which does not override
it." The impl block `impl Greeting for Counter {}` has *empty*
curlies; it provides no `fn` line for `greet` and therefore does
not override it; by the Reference rule, the trait's default body
applies, which is why `c.greet()` returns `100`.

The Book confirms the empty-impl-accepts-default reading at
`output/docs/rust/book/ch10-02-traits.md` lines 207-208 verbatim:
"To use a default implementation to summarize instances of
`NewsArticle`, we specify an empty `impl` block with `impl Summary
for NewsArticle {}`." That sentence is exactly the lesson today
centers, on a smaller probe.

The `c.count` field read in `main` keeps the `count` field "used"
so that `rustc demo.rs` is silent (no `dead_code` warning); the
field has no role in the trait machinery.

## Probe 2 — centered contrast (override the default)

The same trait, two structs, two impls — one accepts the default,
one overrides it.

```rust
struct Counter {
    count: u32,
}

struct Tally {
    n: u32,
}

trait Greeting {
    fn greet(&self) -> u32 {
        100u32
    }
}

impl Greeting for Counter {}

impl Greeting for Tally {
    fn greet(&self) -> u32 {
        self.n * 2
    }
}

fn main() {
    let c = Counter { count: 7 };
    let t = Tally { n: 21 };
    println!("c.count = {}", c.count);
    println!("counter = {}", c.greet());
    println!("tally   = {}", t.greet());
}
```

Compile and run on host:

```text
$ rustc override.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./override
c.count = 7
counter = 100
tally   = 42
$ echo "run-exit=$?"
run-exit=0
```

`c.greet()` returns `100` (the default body); `t.greet()` returns
`42` (`21 * 2`, the override body). One trait, two impls, two
behaviors — empirical witness for both halves of the mechanic in
one program. The Book at `ch10-02-traits.md` lines 235-239
verbatim: "Creating a default implementation doesn't require us to
change anything about the implementation of `Summary` on
`SocialPost` ... the syntax for overriding a default implementation
is the same as the syntax for implementing a trait method that
doesn't have a default implementation." That is the same shape as
the `Tally` impl above.

This probe also incidentally exercises *multi-type dispatch* (one
trait, two structs implementing it). That is its own pedagogical
deferral named since lesson 111; today's centered teaching is the
default body machinery, not the multi-type dispatch — but the
override demonstration cannot be done with a single struct, so the
second struct here is auxiliary witness rather than the centered
move.

## Probe 3 — optional sharpening contrast (drop the default body)

Same source as Probe 1 except the trait method's body slot uses `;`
instead of `{ 100u32 }`. The impl block is still empty `{}`:

```rust
struct Counter {
    count: u32,
}

trait Greeting {
    fn greet(&self) -> u32;
}

impl Greeting for Counter {}

fn main() {
    let c = Counter { count: 7 };
    println!("c.count = {}", c.count);
    println!("greet   = {}", c.greet());
}
```

Compile on host:

```text
$ rustc no_default.rs
error[E0046]: not all trait items implemented, missing: `greet`
 --> no_default.rs:9:1
  |
6 |     fn greet(&self) -> u32;
  |     ----------------------- `greet` from trait
...
9 | impl Greeting for Counter {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^ missing `greet` in implementation

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0046`.
$ echo "compile-exit=$?"
compile-exit=1
```

Read with the lesson 003 four-part map. **Headline**: E0046, the
same code lesson 115 installed, with the message
`not all trait items implemented, missing: `greet`` — the missing
item is the trait method, not the associated type. **Carets**: a
single span at the impl header `impl Greeting for Counter {}` with
the inline label `missing `greet` in implementation`. **Secondary
`-->`**: at the trait declaration's `fn greet(&self) -> u32;` line
with the label `` `greet` from trait``. **Trailer**: aborting due
to 1 previous error and `rustc --explain E0046`.

This is the same E0046 shape lesson 115 captured for a missing
associated type, only with a missing associated function instead.
Together Probes 1 and 3 witness the contrast: with the default
body in the trait, `impl Greeting for Counter {}` is legal; without
it, the same `{}` impl is the canonical E0046 trigger. The default
body is what makes empty `{}` impls legal.

E0046 source `output/docs/rust/error_codes/E0046.md` lines 22-25
verbatim: "you must, at minimum, provide implementations for all of
`Foo`'s required methods (meaning the methods that do not have
default implementations), as well as any required trait items like
associated types or constants." Today exercises exactly the
*not-required-because-it-has-a-default* clause of that rule.

## rmp unlock — `impl Eq for BigUInt {}`

Source `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`
line 10 verbatim:

```rust
impl Eq for BigUInt {}
```

This line is structurally exactly the working probe above:
`impl Trait for Type {}` with empty curlies. The std doc at
`output/docs/rust/std/cmp/trait.Eq.md` line 24 verbatim: "this
property cannot be checked by the compiler, and therefore `Eq` is
a trait without methods." The actual `Eq` trait does carry one
auto-generated method internally (`assert_receiver_is_total_eq`)
which has a default body, so `impl Eq for BigUInt {}` works by the
mechanism today installs — the empty impl accepts every default
body.

Two pieces of rmp's `Eq` usage are *deferred* and not made readable
today:

1. The std declaration `pub trait Eq: PartialEq { }` (line 7 of
   the same std doc) carries a *supertrait* constraint. Reading
   that constraint requires lesson 111's reference at
   `output/docs/rust/reference/items/traits.md` lines 235-258 (the
   *Supertraits* section), which today's lesson explicitly
   defers. The structural reading of `impl Eq for BigUInt {}` —
   "empty impl accepts every default body the trait provided" —
   is correct independent of the supertrait machinery.
2. The category of *marker traits* (traits with no methods or
   only default-bodied methods, like `Send`, `Sync`, `Copy`,
   `Sized`) is named only in passing in today's lesson; the
   formal category is deferred.

## Verbatim corpus quotes

### Reference `output/docs/rust/reference/items/traits.md`

Lines 41-44, the exact rule today centers:

> [items.traits.associated-item-decls]
>
> Trait functions may omit the function body by replacing it with a
> semicolon. This indicates that the implementation must define the
> function. **If the trait function defines a body, this definition
> acts as a default for any implementation which does not override
> it.** Similarly, associated constants may omit the equals sign
> and expression to indicate implementations must define the
> constant value. Associated types must never define the type, the
> type may only be specified in an implementation.

Lines 48-58, the canonical reference example showing both forms
side-by-side in one trait body:

> ```rust
> // Examples of associated trait items with and without definitions.
> trait Example {
>     const CONST_NO_DEFAULT: i32;
>     const CONST_WITH_DEFAULT: i32 = 99;
>     type TypeNoDefault;
>     fn method_without_default(&self);
>     fn method_with_default(&self) {}
> }
> ```

Today's `trait Greeting { fn greet(&self) -> u32 { 100u32 } }` is
exactly the `method_with_default` shape with a non-`()` return
type. The `method_without_default` shape (line 53) is what lesson
111's working probe used.

### Book `output/docs/rust/book/ch10-02-traits.md`

Lines 162-167:

> ### Using Default Implementations
>
> Sometimes it's useful to have default behavior for some or all of
> the methods in a trait instead of requiring implementations for
> all methods on every type. Then, as we implement the trait on a
> particular type, we can keep or override each method's default
> behavior.

Lines 207-208 — the exact framing of the empty-impl-accepts-default
shape that today's working probe uses on a smaller example:

> To use a default implementation to summarize instances of
> `NewsArticle`, we specify an empty `impl` block with
> `impl Summary for NewsArticle {}`.

Lines 235-239 — the override-syntax-is-the-same statement that
Probe 2 demonstrates:

> Creating a default implementation doesn't require us to change
> anything about the implementation of `Summary` on `SocialPost` in
> Listing 10-13. The reason is that the syntax for overriding a
> default implementation is the same as the syntax for implementing
> a trait method that doesn't have a default implementation.

### Error code `output/docs/rust/error_codes/E0046.md`

Lines 22-25 — the rule that *required* methods are exactly the
ones *without* default bodies:

> When trying to make some type implement a trait `Foo`, you must,
> at minimum, provide implementations for all of `Foo`'s required
> methods (meaning the methods that do not have default
> implementations), as well as any required trait items like
> associated types or constants.

### std `output/docs/rust/std/cmp/trait.Eq.md`

Line 7 — the formal declaration:

> `pub trait Eq: PartialEq { }`

Line 24 — the rationale that makes `impl Eq for T {}` a recurring
shape across std and the ecosystem:

> This property cannot be checked by the compiler, and therefore
> `Eq` is a trait without methods.

Line 64 — the canonical user-side empty-impl example, identical in
shape to rmp's `cmp.rs:10`:

> `impl Eq for Book {}`

### rmp `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`

Line 10 — the unlock target:

> `impl Eq for BigUInt {}`

## Claim-to-evidence map

- "A trait method's body slot can be either `;` (no default) or
  `{ ... }` (default body)" — Reference lines 43-44 verbatim quote;
  Reference lines 48-58 the canonical Example trait showing both
  forms; Probe 1 and Probe 3 witness both shapes.
- "An empty `{}` impl block accepts every default body the trait
  provided" — Reference lines 43-44 ("acts as a default for any
  implementation which does not override it"); Book lines 207-208
  verbatim ("we specify an empty `impl` block"); Probe 1 compiles
  silently and prints `100`.
- "An impl may override the default by providing its own `fn`
  line" — Book lines 235-239 verbatim ("the syntax for overriding
  a default implementation is the same as the syntax for
  implementing a trait method that doesn't have a default
  implementation"); Probe 2 demonstrates with `Tally` overriding.
- "Without the default body, the same empty `{}` impl fires E0046"
   — Reference lines 43-44 ("This indicates that the implementation
  must define the function"); E0046 source lines 22-25 verbatim;
  Probe 3 transcript.
- "`impl Eq for BigUInt {}` from rmp is exactly the empty-impl-
  accepts-default shape" — rmp `cmp.rs:10`; std `trait.Eq.md`
  lines 7, 24, 64.
- "The supertrait relationship `Eq: PartialEq` and the marker-trait
  category are separate mechanics deferred today" — std
  `trait.Eq.md` line 7 (`pub trait Eq: PartialEq { }`); Reference
  lines 235-258 (the *Supertraits* section, named-deferred).

## Negative / contrast probe coverage

The lesson makes one contrastive claim: "with the default body in
the trait, the empty `{}` impl works; without it, the same impl
fires E0046." Probe 3 is the negative probe. Probe 2 covers the
secondary contrast "the impl can either accept the default or
override it" by demonstrating both halves in one program.
