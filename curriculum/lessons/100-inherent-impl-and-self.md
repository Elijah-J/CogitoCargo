---
id: 100-inherent-impl-and-self
status: accepted
evidence: ../evidence/100-inherent-impl-and-self.md
---

# Author methods on a struct with `impl Type { ... }`, `&self`, and `Self`

## The Move

Lesson 040 installed the *call site* `value.method()` for methods the
standard library wrote. Lesson 095 let you declare your own type. Today
joins the two: you author your own associated functions and methods on
your own struct. The move composes four inseparable pieces:

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
    fn current(&self) -> u32 {
        self.count
    }
}

fn main() {
    let c = Counter::new();
    println!("count = {}", c.current());
}
```

`./demo` prints `count = 0`. Four pieces, all new today:

1. **`impl Type { ... }`** is an *inherent implementation*: a block
   whose body is a sequence of `fn` items attached to `Type`. Read
   `impl Counter { ... }` as "implement things on `Counter`."
2. **Associated function `fn new() -> Self`** — a function inside the
   impl whose parameter list does *not* start with `self`. Called from
   outside via the path form `Counter::new()` (lesson 043's `::` shape,
   but with a *type* on the left rather than a module).
3. **Method `fn current(&self) -> u32`** — a function inside the impl
   whose first parameter is `&self`. Called from outside via the dot
   form `c.current()` (lesson 040). The Reference: `&self` is shorthand
   for `self: &Self`. Today uses the shorthand spelling.
4. **`Self`** — inside the impl, `Self` is an alias for the type the
   impl is attached to. The Book: "the type `Self` is an alias for the
   type that the `impl` block is for." So `-> Self` here means
   `-> Counter`.

In `main`, two call shapes: `Counter::new()` is the path form,
calling the associated function; `c.current()` is the dot form,
calling the method on `c`.

## Mental Model Delta

- *Before*: "I can declare a struct (lesson 095) and call methods that
  the standard library already wrote on its types (lesson 040). I have
  not authored my own methods."
- *After*: "`impl Type { ... }` is the block where I author functions
  attached to `Type`. Functions whose first parameter is `&self` are
  *methods* — I call them via `value.method()`. Functions without a
  `self` parameter are *associated functions* — I call them via
  `Type::name()`. Inside the impl block, `Self` is shorthand for the
  type the block is attached to, so `-> Self` and `Counter` are the
  same type."

## Prerequisites

- Installed concepts:
  - Lesson 095 (*load-bearing*): `struct Name { field: Type }`,
    `Name { field: value }`, `instance.field`. Today reuses all three
    inside the impl: the declared `Counter` is the impl target,
    `Counter { count: 0 }` is the body of `new`, and `self.count` is
    a field access on the receiver.
  - Lesson 040 (*load-bearing*): the call site `value.method()`. Today
    fills in the authoring side — `&self` is what makes a function
    reachable via the dot.
  - Lessons 020 and 021 (*load-bearing*): `fn name(p: T) -> R { ... }`.
    Functions inside an impl are the same `fn` shape, with one new
    rule: the first parameter may be `&self`.
  - Lesson 043 (*load-bearing*): the path call `path::name(args)`.
    Today's `Counter::new()` is the same shape with a *type* on the
    left of `::` rather than a module.
  - Lesson 062 (cited): `u32`, the field type and method return type.
  - Lessons 002, 005, 011 (cited): `fn main`, `let`, `println!` `{}`.
  - Lesson 003 (cited): the diagnostic four-part map.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
count = 0
```

The struct is lesson 095. The new shape is the `impl Counter { ... }`
block. Inside it, `fn new() -> Self` has no `self` parameter — an
associated function. `fn current(&self) -> u32` has `&self` first — a
method. In `main`, `Counter::new()` is the path call;
`c.current()` is the dot call.

*Now the contrast.* Same struct and impl block, with two coupled edits:
`&self` is removed from `current`'s signature, and `self.count` in the
body is replaced with `0` (so `self` stays out of scope). The call site
is left unchanged at `c.current()`. Save as `no_self.rs`:

```rust
struct Counter { count: u32 }

impl Counter {
    fn new() -> Self { Counter { count: 0 } }
    fn current() -> u32 { 0 }
}

fn main() {
    let c = Counter::new();
    println!("count = {}", c.current());
}
```

Compile:

```
error[E0599]: no method named `current` found for struct `Counter` in the current scope
  --> no_self.rs:10:30
   |
 1 | struct Counter { count: u32 }
   | -------------- method `current` not found for this struct
...
10 |     println!("count = {}", c.current());
   |                              ^^^^^^^ this is an associated function, not a method
   |
   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: the candidate is defined in an impl for the type `Counter`
  --> no_self.rs:5:5
   |
 5 |     fn current() -> u32 { 0 }
   |     ^^^^^^^^^^^^^^^^^^^
help: use associated function syntax instead
   |
10 -     println!("count = {}", c.current());
10 +     println!("count = {}", Counter::current());
   |
```

Read it with the lesson 003 map. Headline: E0599 (the same E-code
lesson 098 used; the Reference catalogs it as "method or item not
found"). Caret under `current` at the call site. The diagnostic
states today's rule twice: the inline label `this is an associated
function, not a method`, and the `= note:` line `to be used as
methods, functions must have a \`self\` parameter`. The `help:`
proposes the path form `Counter::current()` as the alternative call
shape — the exact distinction today installs.

## What Changed

- `impl Type { ... }` is an *inherent implementation*. Its `fn` items
  are *associated items* of `Type`.
- A `fn` inside an impl with `&self` first is a *method* — call it
  with the dot form `value.method()` (lesson 040).
- A `fn` inside an impl with no `self` parameter is an *associated
  function* — call it with the path form `Type::name()`. The Book's
  canonical example is `String::from`; today's is `Counter::new`.
- `Self` inside the impl is an alias for the type the impl is
  attached to. `-> Self` here means `-> Counter`.
- Without `&self`, the dot form cannot reach the function — rustc
  says so verbatim in E0599's label and `note:` lines.

## Check Yourself

You write `tiny.rs`:

```rust
struct Tally {
    n: u32,
}

impl Tally {
    fn new() -> Self {
        Tally { n: 7 }
    }
    fn value(&self) -> u32 {
        self.n
    }
}

fn main() {
    let t = Tally::new();
    println!("value = {}", t.value());
}
```

(a) Does `rustc tiny.rs` accept the program (no errors, no warnings)?

(b) What single line does `./tiny` print?

(c) If you delete `&self` from `fn value(&self) -> u32` (signature
becomes `fn value() -> u32`) **and** replace `self.n` in the body with
`0` (so `self` stays out of scope), but leave `t.value()` at the call
site unchanged, what E-code appears, and what call form does the
`help:` block propose as the fix?

(*Answers: (a) Yes — silent compile. (b) `value = 7`. (c) E0599;
the `help:` block proposes the path form `Tally::value()`.*)

## What To Ignore For Now

Today installs only inherent impl with two receiver shapes (`&self`,
or no receiver) plus `Self` as a type alias. Real and deferred:

- *`&mut self`* — the mutation receiver. Natural follow-on (lesson 101).
- *`self` by value* — the consuming receiver. Future move after
  `&mut self`.
- *Multiple `impl` blocks for one type* — the Book Listing 5-16 notes
  "each struct is allowed to have multiple `impl` blocks." The rmp
  target uses this heavily.
- *Trait impls* `impl Trait for Type { ... }` — different syntax,
  blocked on trait machinery.
- *Associated constants* `const N: u32 = ...;` and *associated types*
  `type Output = ...;` inside an impl block.
- *Generic impls* `impl<T> Type<T> { ... }` and lifetime-parameterized
  impls `impl<'a> Type<'a> { ... }`.
- *Auto-deref on method calls* — the rule that lets `c.current()`
  match `&self` without writing `&c`. Deferred.
- *Calling `Self::name()` from inside the impl body* — the `Self`
  shorthand for `Type::name()` self-references.
- *`pub` on impl items* — visibility on associated functions and
  methods. Today's probe uses default-private visibility. Deferred.
- *Calling `&self` methods via the path form* — `Counter::current(&c)`
  works (the auxiliary appendix probe witnesses this); `c.current()`
  is the idiomatic shape.
- *Trait methods under the dot* — many real-world dot calls resolve
  to trait methods. Future move under trait machinery.

## Evidence

See `../evidence/100-inherent-impl-and-self.md` for the corpus-quote
map, the rustc / system toolchain string, the working probe
transcript, the centered E0599 contrast, the auxiliary E0061
"method-via-path" contrast, the `Self { ... }` construction-shape
witness, and the prerequisite-claim summary.
