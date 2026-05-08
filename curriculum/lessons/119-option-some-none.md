---
id: 119-option-some-none
status: accepted
evidence: ../evidence/119-option-some-none.md
---

# `Option<T>` with `Some(T)` / `None` constructors and a `match` on its variants

## The Move

`Option<T>` is `Result<T, E>`'s prelude sibling — same generic-enum
shape (lesson 052), but with **one** type parameter and **two**
variants. Three coupled facts:

1. **Type.** `pub enum Option<T> { None, Some(T) }` (std). `None` is a
   unit variant (no payload, lesson-098 shape); `Some(T)` is a tuple
   variant carrying one value of type `T` (lesson-052 shape). `Option`
   and both variants are in the prelude — no `use` needed.

2. **Constructors.** `Some(value)` is a call expression that wraps
   `value` into an `Option<T>`. `None` is a path expression (no
   parentheses, no payload) producing the same type's other variant.
   `Some(42)` lets rustc infer `T = i32` from the `42`. A bare `None`
   carries no value, so rustc *cannot* infer `T`; an annotation like
   `: Option<i32>` is required.

3. **`match`.** The lesson-058 payload-variant shape generalizes
   unchanged. `Some(n)` is a pattern that binds the payload to local
   `n`; `None` is the lesson-098 bare-variant pattern. With both arms
   covered the match is exhaustive — `Option` has only two variants —
   and lesson-030's "all arms share a type" rule still applies.

```rust
fn main() {
    let present: Option<i32> = Some(42);
    let absent: Option<i32> = None;

    let p_label = match present {
        Some(n) => n,
        None => -1,
    };
    let a_label = match absent {
        Some(n) => n,
        None => -1,
    };
    println!("present -> {}", p_label);
    println!("absent  -> {}", a_label);
}
```

`rustc demo.rs` exits 0 silently; `./demo` prints `present -> 42`
then `absent  -> -1`. `Some(n) => n` extracts the payload (058);
`None => -1` matches the bare variant (098).

## Mental Model Delta

- *Before:* "`Result<T, E>` (052) carries `Ok(T)` / `Err(E)` payloads
  and matches with `Variant(name) => ...` (058). Unit-variant enums
  match bare (098). `Option` is a name I have heard but never built."
- *After:* "`Option<T>` is `Result`'s prelude sibling — one type
  parameter, two variants. `Some(value)` builds the payload variant
  (052 shape); `None` builds the unit variant (098 shape). A `match`
  combines both: `Some(name) => arm` and `None => arm`. A bare `None`
  cannot pin `T` on its own; the binding needs an annotation when
  nothing else fixes the type."

## Prerequisites

- Installed concepts:
  - **Lesson 052** (load-bearing): generic enum with payload variant
    constructed by call expression. `Some(T)` is the sibling of
    `Ok(T)`; the call-expression construction is unchanged.
  - **Lesson 058** (load-bearing): `match` arm `Variant(name) => arm`
    with a payload binding. `Some(n) => n` is exactly this shape.
  - **Lesson 098** (load-bearing): unit-variant pattern `Variant => arm`,
    no parentheses. `None => -1` is exactly this shape.
  - **Lesson 019** (load-bearing): `let name: TYPE = value;`. Today
    fills the TYPE slot with `Option<i32>`.
  - **Lesson 030 / 031** (cited): `match` exhaustiveness, all arms
    share a type. Unchanged.
  - **Lesson 003** (cited): rustc diagnostic four-part map — used for
    the contrast probe's E0282 transcript.
  - **Lessons 001, 002, 005, 011** (cited): `rustc file.rs` then
    `./name`; `fn main`; `let`; `println!` with `{}`. Unchanged.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, Linux/macOS shell.

## Try It

Save the snippet from *The Move* as `demo.rs` in a fresh empty
directory:

```console
$ rustc demo.rs
$ ./demo
present -> 42
absent  -> -1
```

The first `match` finds `present` is `Some(_)`, binds `42` to `n`,
arm value `n` is `42`. The second finds `absent` is `None`, arm value
`-1`. Two `println!`s print the bound `i32`s.

*Now the contrast.* Save `broken.rs`:

```rust
fn main() {
    let absent = None;
}
```

Compile:

```text
error[E0282]: type annotations needed for `Option<_>`
 --> broken.rs:2:9
  |
2 |     let absent = None;
  |         ^^^^^^   ---- type must be known at this point
  |
help: consider giving `absent` an explicit type, where the type for type parameter `T` is specified
  |
2 |     let absent: Option<T> = None;
  |               +++++++++++
```

Read it with the lesson-003 map. Headline `E0282`; location
`broken.rs:2:9` on `absent`. The rule the diagnostic states is the
rule today installs: a bare `None` pins the variant but not `T`. Book
ch06-01:389-395 says the same — "the compiler can't infer the type
that the corresponding `Some` variant will hold by looking only at a
`None` value." Replacing `let absent = None;` with
`let absent: Option<i32> = None;` compiles. A corroborating probe in
the appendix shows `let present = Some(42);` *does* compile without
an annotation — `Some(42)`'s payload pins `T = i32`.

## What Changed

- `Option<T>` is the prelude's sibling of `Result<T, E>`: one type
  parameter, two variants (`Some(T)` with payload, `None` without).
  No `use` needed.
- `Some(value)` is a call-expression constructor (058 shape); `None`
  is a path-expression constructor (098 shape).
- `match` opens an `Option<T>` with `Some(name) => ...` and
  `None => ...`. Exhaustiveness still applies.
- A bare `None` cannot pin `T`; without other context, rustc fires
  `E0282` and the help line proposes `: Option<T>`. `Some(literal)`
  *can* pin `T`, because the payload's type flows back.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let maybe: Option<i32> = Some(7);
    let n = match maybe {
        Some(v) => v + 1,
        None => 0,
    };
    println!("n = {}", n);
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) Change `Some(7)` to `None` and recompile. What does it print?

(c) Without recompiling, predict: if you delete the `None => 0,` arm,
which E-code fires?

(d) Predict: does `let q = Some("hi");` need a `: Option<...>`
annotation?

*(Answers: (a) Yes; prints `n = 8`. `Some(v)` binds `v = 7`; arm value
`v + 1` is `8`. (b) `n = 0`. The `None` arm fires; arm value `0`. (c)
E0004 — non-exhaustive patterns: `\`None\` not covered`. (d) No. The
`"hi"` pins `T = &str`, so rustc infers `Option<&str>`. Only bare
`None` blocks inference.)*

## What To Ignore For Now

- *`.is_some()` / `.is_none()`* — sibling methods of Result's `.is_ok()`;
  named in `output/docs/rust/std/option/enum.Option.md` lines 39-41
  and 78-80; deferred.
- *`.unwrap()`, `.expect()`, `.unwrap_or()`, `.map()`, `.and_then()`,
  `.ok_or()`, `?`* — extraction and propagation; deferred.
- *`if let Some(x) = opt { ... }`* — single-arm shorthand; deferred.
- *`Option<T>` as a function parameter or return type* — composes
  today's mechanic with function signatures; deferred.
- *Non-primitive payloads* `Option<&T>`, `Option<Box<T>>`,
  `Option<Vec<T>>`; deferred.
- *Authoring your own generic enum* `enum My<T> { ... }` — today
  *consumes* std's generic Option; the authoring move is blocked on
  the generics installation arc.
- *`Option<std::cmp::Ordering>`* — the rmp `cmp.rs:13` use site; named
  as the future composition target, not centered today.
- *The relationship between `Option<T>` and pointers / null* — Book
  ch06-01:325-365 frames it; the mechanic is deferred.

## Evidence

See `../evidence/119-option-some-none.md`.
