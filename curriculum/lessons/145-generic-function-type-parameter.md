---
id: 145-generic-function-type-parameter
status: accepted
evidence: ../evidence/145-generic-function-type-parameter.md
---

# A `fn` declaration may carry a type parameter `<T>` after the name

## The Move

Every type written inside a `fn` signature so far has been a specific
named concrete type — `i32` (lessons 020/021), `u32`, `f64`. Today
the function declaration grows a *type parameter*: a placeholder for
a type, declared in angle brackets *after the function name and
before the parameter list*. The same source declaration handles more
than one concrete type — each call site picks one.

```rust
fn id<T>(t: T) -> T { t }

fn main() {
    let a: u32 = id(5_u32);
    let b: i32 = id(7_i32);
    println!("{} {}", a, b);
}
```

`rustc demo.rs` is silent; `./demo` prints `5 7`.

Three new tokens.

1. **`<T>` after the function name** — `fn id<T>(t: T) -> T`. Where
   lessons 020/021 wrote `fn id(t: i32) -> i32`, the declaration now
   has `<T>` between the name and the opening `(`. `T` is an
   ordinary uppercase identifier; the Book recommends `T` by
   convention.

2. **`T` used as a *type* in the parameter list and the return type**
   — `(t: T) -> T`. Where lessons 020/021 wrote `i32`, the
   declaration writes the placeholder. The Reference: "Inside the
   function signature and body, the name of the type parameter can
   be used as a type name."

3. **Per-call substitution** — at `id(5_u32)` rustc picks `T = u32`;
   at `id(7_i32)` rustc picks `T = i32`. The same `fn id<T>`
   declaration covers both. The Book's reading: "The function
   `largest` is generic over some type `T`."

This is the same `<T>` token lesson 114 installed on a *trait*
header; today it sits on a *function* header. Lesson 114
named-deferred this exact extension — "generic functions
`fn f<T>(t: T)` — distinct mechanic, same `<T>` slot."

## The contrast: drop `<T>` and `T` becomes a name out of scope

Same source, with the `<T>` deleted:

```rust
fn id(t: T) -> T { t }
```

`rustc nodecl.rs` rejects it:

```text
error[E0425]: cannot find type `T` in this scope
 --> nodecl.rs:1:10
  |
1 | fn id(t: T) -> T { t }
  |          ^ not found in this scope
  |
help: you might be missing a type parameter
  |
1 | fn id<T>(t: T) -> T { t }
  |      +++
```

Read with the lesson 003 map. Headline `error[E0425]: cannot find
type `T` in this scope` — the same E-code as lesson 005's `cannot
find value` and lesson 008's `cannot find function`, with the noun
`type`. Caret on the `T` after `t:` — the token rustc refuses to
resolve. The `help: you might be missing a type parameter` block
proposes the exact diff: insert `<T>` after the function name, with
three `+` markers under `<T>`. (rustc fires the same error a second
time for the `T` in the return position.) The `<T>` declarator is
*what makes `T` a name in scope* inside the signature.

## Mental Model Delta

- *Before*: "Every type in a `fn` signature is a specific named
  concrete type. The angle-bracket `<T>` shape is a *trait* mechanic
  (lesson 114)."
- *After*: "A `fn` declaration may also carry a type parameter
  between angle brackets *after the function name*. Inside the
  signature and body, the parameter name (`T`, `U`, ...) stands in
  any type position. Each call site picks a concrete type
  independently; the same source declaration covers all of them.
  Without `<T>`, using `T` in the signature fires E0425 — the
  type-position version of the `cannot find ...` family."

## Prerequisites

- Installed concepts:
  - **Lesson 008** (load-bearing): `fn name() { ... }` defines a
    function. Today extends the *header* with `<T>` after the name.
  - **Lesson 020** (load-bearing): `(p: TYPE)` parameter-list
    grammar; types mandatory. Today fills `TYPE` with `T`.
  - **Lesson 021** (load-bearing): `-> RTYPE` return type and
    `name(args)` as a value-carrying expression. Today fills `RTYPE`
    with `T`.
  - **Lesson 114** (load-bearing): `<T>` on a *trait* header
    declares a type parameter usable in any type position inside the
    body. Today is the *function* version of the same `<T>` slot —
    the "distinct mechanic, same `<T>` slot" lesson 114
    named-deferred verbatim.
  - **Lesson 003** (load-bearing): rustc diagnostic map. E0425 is
    already familiar from 005/008; today the inline label is `cannot
    find type`.
  - Cited: lesson 081 (`5_u32`/`7_i32`); lesson 080 (`u32`/`i32`);
    lesson 005 (`let name: TYPE = value;`); lesson 025 (the body
    `{ t }` is an *implicit return*); lesson 011 (`println!`); 002
    (`fn main`); 001 (`rustc && ./demo`).
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

In a fresh empty directory, save `demo.rs` from the working probe.
Run `rustc demo.rs && ./demo`. Output: `5 7`.

Now save `nodecl.rs` — same source, but drop the `<T>`:
`fn id(t: T) -> T { t }`. Run `rustc nodecl.rs`. The E0425
transcript above is what you should see.

## What Changed

- `fn name<T>(...)` declares a function with one type parameter,
  scoped to that function's signature and body.
- The type parameter sits in any type position — parameter type slot
  (020) and return type slot (021) both work.
- Each call site picks a concrete type for `T` independently. rustc
  generates a specialized version per substitution at compile time.
- Without the `<T>` declarator, rustc fires E0425 with `cannot find
  type ...`; the `help:` line proposes the missing `<T>` directly.

## Check Yourself

You write `tiny.rs`:

```rust
fn pass<U>(u: U) -> U { u }

fn main() {
    let a: u8 = pass(9_u8);
    let b: i64 = pass(123_i64);
    println!("{} {}", a, b);
}
```

(a) Does `rustc tiny.rs` compile, and what does `./tiny` print?

(b) Change *only* the declaration to `fn pass(u: U) -> U { u }` —
drop the `<U>`. What E-code, what inline label, and what `help:`
line?

(Answers: (a) compiles silently; prints `9 123` — `pass(9_u8)` runs
with `U = u8`, `pass(123_i64)` with `U = i64`. (b) E0425; inline
label `cannot find type `U` in this scope`; `help: you might be
missing a type parameter` proposes `fn pass<U>(u: U) -> U`.)

## What To Ignore For Now

This lesson installs *only* the type-parameter declaration on a
function header plus per-call substitution. The closure sub-arc
audit (`iterator-api-coverage.md` §6 step 4) sketches "FnMut-bound
parameter on a function" as one step; in execution it is split
into three: today (generic function syntax), lesson 146 (trait
bound on a generic function parameter), lesson 147 (the
`FnMut(T) -> R` parenthesized sugar plus closure as argument).
Other deferrals:

- **Trait bounds** — `fn f<T: SomeTrait>(...)` and the `where T:
  SomeTrait` form. Without a bound, the body can do almost nothing
  with `T` except move it around (today's `id` body is `t`).
  Lesson 146.
- **`Fn(...)` / `FnMut(...) -> R` parenthesized trait sugar** —
  the closure-as-argument shape. Lesson 147.
- **Multiple type parameters** — `fn f<T, U>(t: T, u: U)`.
- **Lifetime parameters** and **const generics** — `fn f<'a>(...)`,
  `fn f<const N: usize>(...)`. Deferred wholesale.
- **Generic struct types** — `struct Point<T> { x: T, y: T }`. Same
  `<T>` slot on a struct header; separate mechanic.
- **Turbofish** — `id::<u32>(5)`. Today's substitution is inferred
  from the argument.
- **Monomorphization at depth** — code-bloat trade-off, instantiation
  rules. Named lightly above; the depth is deferred.
- **Generic methods inside an `impl`** — `impl<T> Point<T> { ... }`.
  Composite of today with the trait-and-impl arc.
- **Trait objects** — `&dyn SomeTrait`. Different machinery
  (dynamic dispatch) for "function takes a value of various types";
  today's mechanic is static dispatch.

## Evidence

See `../evidence/145-generic-function-type-parameter.md`.
