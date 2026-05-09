---
id: 146-trait-bound-on-type-parameter
status: accepted
evidence: ../evidence/146-trait-bound-on-type-parameter.md
---

# Add `: TRAIT` after a type parameter to constrain it

## The Move

Lesson 145 declared `fn id<T>(t: T) -> T { t }`. The body could only
*move `t` around* — it returned `t` unchanged because that is the only
thing a `T` value supports when the function knows nothing about `T`.
Today the type parameter grows a *trait bound*: a colon after the
parameter name and a trait path, inside the same angle brackets.

```rust
fn say<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}

fn main() {
    say(5_u32);
    say(7_i32);
}
```

`rustc demo.rs` is silent; `./demo` prints two lines:

```text
5
7
```

The `<T: std::fmt::Display>` shape extends lesson 145's `<T>` with one
new segment: a colon and a trait path after the parameter name. That
segment is the *trait bound*. It does two things at once:

1. **It restricts the call site** — only types that implement
   `Display` can be substituted for `T`. `u32` and `i32` both do
   (the std doc page for `Display` lists `impl Display for i32`
   and `impl Display for u32` among its implementors), so both
   call sites compile.

2. **It enables the body** — inside the function, `T` values can be
   used wherever `Display` is required. The Reference's
   trait-bounds intro states: "in the body of a generic function,
   methods from `Trait` can be called on `Ty` values." Today's
   body uses `println!("{}", t)` — the `{}` placeholder asks for
   `Display`, and the bound is what makes the placeholder legal
   for a value of type `T`.

## The contrast: drop the bound and the body breaks

Same source, with the `: std::fmt::Display` deleted:

```rust
fn say<T>(t: T) {
    println!("{}", t);
}
```

`rustc nobound.rs` rejects it:

```text
error[E0277]: `T` doesn't implement `std::fmt::Display`
 --> nobound.rs:2:20
  |
2 |     println!("{}", t);
  |               --   ^ `T` cannot be formatted with the default formatter
  |               |
  |               required by this formatting parameter
  |
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
help: consider restricting type parameter `T` with trait `Display`
  |
1 | fn say<T: std::fmt::Display>(t: T) {
  |         +++++++++++++++++++
```

Read it with the lesson 003 map. Headline: a *new* error code,
`E0277`, with the inline label `\`T\` doesn't implement
\`std::fmt::Display\``. Caret on `t` inside the `println!` body —
the value rustc cannot format with `{}`. The `help:` line
proposes the exact diff that today's lesson installs: insert
`: std::fmt::Display` after `T` in the angle brackets.

The asymmetry is the point. With the bound, rustc knows every
concrete `T` that reaches the body implements `Display`, so
`println!("{}", t)` is legal. Without the bound, `T` could be
*any* type — including types that do not implement `Display` — so
rustc rejects the body at definition time. Lesson 145's body
`{ t }` got away with no bound because moving a value around does
not require any trait at all.

## Mental Model Delta

- *Before:* "A generic function `fn name<T>(t: T)` accepts any
  concrete type for `T`; the body can do almost nothing with `t`
  except move it around (lesson 145's `id` body)."
- *After:* "Adding `: TRAIT` after the type parameter restricts
  which types can be substituted at the call site to types that
  implement `TRAIT`, and grants the body the right to use
  `TRAIT`'s methods on `t`. The two restrictions come together:
  `<T: TRAIT>` widens what the body can do *and* narrows what the
  call site can pass."

## Prerequisites

- Installed concepts:
  - **Lesson 145** (load-bearing): `fn name<T>(t: T)` declares a
    type parameter with no bound. Today appends `: TRAIT` after the
    parameter name inside the same angle brackets.
  - **Lesson 011** (load-bearing): `println!("{}", t)` is the
    positional placeholder. Today's body is exactly this form;
    the bound is what makes the placeholder valid for `t: T`.
  - **Lesson 003** (load-bearing): rustc diagnostic map. E0277 is a
    new error code today; the four-part shape carries unchanged.
    The `help:` line proposes the missing bound directly.
  - Cited: lesson 081 (`5_u32`/`7_i32` literal-suffix forms);
    lesson 043 (the `std::fmt::Display` path reuses 043's
    `::`-separated multi-segment grammar; today's trailing segment
    is a trait name rather than a function name);
    lesson 080 (`u32`/`i32` integer family); lesson 020 (parameter
    slot `t: T`); lesson 008 (call shape `say(...)`); lesson 002
    (`fn main`); lesson 001 (`rustc && ./demo`).
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

In an empty directory, save `demo.rs` from the working probe.
Run `rustc demo.rs && ./demo`. Output: two lines, `5` and `7`.
Then save `nobound.rs` — drop the `: std::fmt::Display`, leaving
`fn say<T>(t: T) { ... }`. Run `rustc nobound.rs`. The E0277
transcript above is what you should see.

## What Changed

- A type parameter may carry a *trait bound* — `<T: TRAIT>` — and
  the bound restricts which concrete types can be substituted for
  `T` at the call site.
- The bound also grants the body the right to use `TRAIT`'s
  capabilities on a value of type `T`. Today's body uses
  `println!("{}", t)`, which requires `Display`.
- The trait path can be fully qualified (`std::fmt::Display`); no
  `use` declaration is needed for that to compile.
- Without the bound, rustc rejects the body at definition time
  with E0277 and proposes the exact missing bound in its `help:`
  line.

## Check Yourself

You write `tiny.rs`:

```rust
fn show<U: std::fmt::Display>(u: U) {
    println!("value = {}", u);
}

fn main() {
    show(42_u8);
    show(true);
}
```

(a) Does `rustc tiny.rs` compile, and what does `./tiny` print?

(b) Change only the declaration to `fn show<U>(u: U) { ... }` —
drop the bound. Which error code fires, and what does the `help:`
line propose?

(Answers: (a) compiles silently; prints `value = 42` then
`value = true` — both `u8` and `bool` implement `Display`. (b)
E0277 with inline label `\`U\` doesn't implement
\`std::fmt::Display\``; `help:` proposes
`fn show<U: std::fmt::Display>(u: U)`.)

## What To Ignore For Now

This lesson installs *only* the inline `<T: TRAIT>` form on a
single type parameter, with `Display` as the trait. Decomposition
note: this is the second move of the closure sub-arc step 4
decomposition (145 = generic function syntax; today = trait
bound; 147 = the `Fn(...)` / `FnMut(...) -> R` parenthesized sugar
plus closure-as-argument). Other deferrals:

- **The `where` clause form** — `fn name<T>(t: T) where T: Trait { ... }`.
  Same mechanic; the Reference says it directly: `fn f<A: Copy>() {}`
  is the same as `fn f<A>() where A: Copy {}`.
- **Multiple bounds with `+`** — `<T: A + B>` requires `T` to
  implement both.
- **Multiple type parameters with separate bounds** — `<T: A, U: B>`.
- **The parenthesized `Fn(...)` / `FnMut(...) -> R` sugar** —
  closure-as-argument shape, lesson 147.
- **`use` declarations** — today writes `std::fmt::Display`
  inline. Bringing it into scope with `use std::fmt::Display;`
  is a separate mechanic.
- **`Display`'s full surface** — the trait has one required
  method `fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`.
  Today uses `Display` only via `println!("{}", t)`.
- **Other formatting traits** — `Debug` (`{:?}`), `LowerHex`, ...
- **Implementing `Display` for your own type** — `impl Display for MyType`.
- **Trait objects `&dyn Trait`** — different machinery (dynamic
  dispatch); today's mechanic is static dispatch.
- **Generic struct types and generic methods inside `impl<T>`** —
  same `<T: Trait>` slot on different host items.
- **Turbofish** — `say::<u32>(5_u32)` for explicit substitution.

## Evidence

See `../evidence/146-trait-bound-on-type-parameter.md`.
