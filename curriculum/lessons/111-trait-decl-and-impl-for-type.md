---
id: 111-trait-decl-and-impl-for-type
status: accepted
evidence: ../evidence/111-trait-decl-and-impl-for-type.md
---

# Declare a trait with one method signature; implement it for a struct

## The Move

Lesson 100 installed *inherent* implementations: `impl Counter { ... }`
attaches functions directly to `Counter`. Today installs the second
flavor of `impl` — a *trait* implementation. Three new items in one
program:

```rust
struct Counter {
    count: u32,
}

trait Doubled {
    fn doubled(&self) -> u32;
}

impl Doubled for Counter {
    fn doubled(&self) -> u32 {
        self.count * 2
    }
}

fn main() {
    let c = Counter { count: 21 };
    println!("doubled = {}", c.doubled());
}
```

`./demo` prints `doubled = 42`.

1. **`trait Doubled { fn doubled(&self) -> u32; }`** — a *trait
   declaration*. Inside sits one *method signature* with the body
   replaced by `;`. The Reference: "Trait functions may omit the
   function body by replacing it with a semicolon. This indicates
   that the implementation must define the function." On its own,
   the declaration attaches no methods to any type; it describes a
   *shape* — name, receiver, return type — that some type can later
   be required to provide.

2. **`impl Doubled for Counter { fn doubled(&self) -> u32 { ... } }`**
   — a *trait implementation*, a different `impl` block from lesson
   100. The Reference's grammar splits them as `InherentImpl` and
   `TraitImpl`. The Book: "after `impl`, we put the trait name we
   want to implement, then use the `for` keyword, and then specify
   the name of the type we want to implement the trait for." The
   body fills in what the trait declared; the signature in the impl
   must match the signature in the trait.

3. **`c.doubled()`** — the call. The dot-call shape from lesson 040
   is unchanged; the same call site now resolves to a *trait* method.

The point of traits is not yet visible from one program — the same
`doubled` could have been an inherent method. The pay-off arrives
later when one trait is implemented for many types, when generic
functions take "any type that implements `Doubled`," and when
operator-overloading traits like `Add` come into reach. Today
installs only the syntax and the single-type case.

## Mental Model Delta

- *Before*: "An `impl` block is `impl Type { ... }`. It attaches
  functions directly to `Type`. The dot call reaches those functions
  through `&self`."
- *After*: "There are *two* kinds of `impl` block. `impl Type { ... }`
  is *inherent* — its methods belong to `Type` only.
  `impl Trait for Type { ... }` is a *trait* impl — it fills in
  bodies for signatures that live in a separate `trait` block. Same
  dot call. The impl signature must match the trait signature.
  Reading the header: `Trait for Type` between `impl` and the brace
  means trait impl; just `Type` means inherent."

## Prerequisites

- Installed concepts:
  - **Lesson 095** (load-bearing): `struct Name { field: Type, }` and
    field access `value.field`. `Counter` is the trait's target;
    `self.count` is field access in the body.
  - **Lesson 100** (load-bearing): the `impl` block with `&self`
    methods. Today reuses the entire skeleton; the only header diff
    is `Doubled for` in the middle.
  - **Lesson 040** (load-bearing): the dot-call shape `value.method()`,
    unchanged.
  - **Lesson 008** (load-bearing): `fn name(p: T) -> R { ... }`. The
    method signature inside the trait is the same `fn` shape with the
    body replaced by `;`.
  - **Lessons 002, 005, 009, 011, 012, 013, 019** (cited): `fn main`,
    `let`, `*` (multiplication), `println!` `{}`, `bool`, `>`
    (comparison), the type-annotation slot.
  - **Lesson 080** (cited): `u32` as a member of the integer family.
  - **Lesson 003** (cited): the four-part diagnostic map for E0599.
  - **Lesson 001** (cited): `rustc demo.rs` then `./demo`.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, ability to run `./demo`.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
doubled = 42
```

Three top-level items: `struct`, `trait`, `impl ... for ...`. The
trait body has one line ending in `;` (a signature). The impl body
has the *same* signature followed by `{ ... }` filling it in.

*Now the contrast.* Save `no_impl.rs` — same source minus the entire
`impl Doubled for Counter { ... }` block; struct, trait, and `fn main`
unchanged:

```rust
struct Counter {
    count: u32,
}

trait Doubled {
    fn doubled(&self) -> u32;
}

fn main() {
    let c = Counter { count: 21 };
    println!("doubled = {}", c.doubled());
}
```

Compile:

```
error[E0599]: no method named `doubled` found for struct `Counter` in the current scope
  --> no_impl.rs:11:32
   |
 1 | struct Counter {
   | -------------- method `doubled` not found for this struct
...
11 |     println!("doubled = {}", c.doubled());
   |                                ^^^^^^^ method not found in `Counter`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Doubled` defines an item `doubled`, perhaps you need to implement it
  --> no_impl.rs:5:1
   |
 5 | trait Doubled {
   | ^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
```

Read with the lesson 003 map. Headline: E0599 — the same code
lessons 100 and 108 installed for "method not found." Caret under
`doubled` at the call site. The `= help:` line names today's rule
verbatim: *items from traits can only be used if the trait is
implemented and in scope*. The `note:` block points at the trait
declaration and adds *`Doubled` defines an item `doubled`, perhaps
you need to implement it*. Without the impl block, the signature is
a promise nobody kept; the dot call has nothing to dispatch to.

## What Changed

- `trait Name { ... }` declares an *abstract interface* — method
  signatures ending in `;`, no bodies. By itself it attaches nothing
  to any type.
- `impl Trait for Type { ... }` — distinct from `impl Type { ... }` —
  provides the bodies the trait declared. The signature must match.
- Reading the header: `Trait for Type` ⇒ trait impl; just `Type` ⇒
  inherent impl.
- The dot call `value.method()` is unchanged. The same call site now
  resolves to either an inherent method (100) or a trait method.
- Without the impl block, the dot call fails with E0599 whose
  `= help:` line names today's rule.

## Check Yourself

You write `tiny.rs`:

```rust
struct Tally { n: u32 }

trait AsBool { fn as_bool(&self) -> bool; }

impl AsBool for Tally {
    fn as_bool(&self) -> bool { self.n > 0 }
}

fn main() {
    let t = Tally { n: 7 };
    println!("as_bool = {}", t.as_bool());
}
```

(a) Does `rustc tiny.rs` accept the program (no errors, no warnings)?

(b) What single line does `./tiny` print?

(c) If you delete the entire `impl AsBool for Tally { ... }` block
but leave everything else unchanged, what E-code appears, and what
phrase in the `= help:` line names today's rule?

*(Answers: (a) Yes. (b) `as_bool = true`. (c) E0599; "items from
traits can only be used if the trait is implemented and in scope.")*

## What To Ignore For Now

Today installs only the smallest atomic trait machinery: declare one
trait with one signature, implement it for one struct, call it once.
Deferred:

- **Default method bodies** — `fn method(&self) { ... }` (curlies,
  not `;`) inside the trait acts as a default the impl may override.
- **`pub trait`** for cross-module visibility — lesson 096's `pub`
  composes into the trait header; today's single-file probe doesn't
  need it.
- **Multiple methods in one trait body**, and **multiple types
  implementing the same trait** — the latter is *the* point of trait
  machinery (one signature, many types); named as the future unlock,
  not probed.
- **Other receivers** (`&mut self`, `self`) and **signatures with
  extra parameters** — lessons 101 / 102's receiver shapes compose
  into trait methods identically.
- **Bringing the trait into scope** — `use module::TheTrait;` when
  caller and trait live in different modules.
- **Generic traits** `trait Seq<T>`, **trait bounds**, `impl Trait`
  returns, `dyn Trait`, trait objects — all blocked on generics.
- **Associated types** `type Output = ...;` and **associated
  constants** inside a trait.
- **Operator traits** — `std::ops::Add`, `Sub`, `Mul`, `AddAssign`,
  `Display`, `Debug`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`.
- **`#[derive(Clone)]`, `#[derive(Debug)]`, etc.** — derive macros
  expand at compile time to `impl Trait for Type` blocks.
- **`Self` inside the trait body** — same alias rule as lesson 100.
- **The orphan rule** — coherence constraint on which trait-type
  combinations are allowed.

## Evidence

See `../evidence/111-trait-decl-and-impl-for-type.md`.
