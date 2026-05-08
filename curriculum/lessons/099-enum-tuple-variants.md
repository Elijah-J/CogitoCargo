---
id: 099-enum-tuple-variants
status: accepted
evidence: ../evidence/099-enum-tuple-variants.md
---

# Add a tuple variant to an enum, construct one with a payload, match it with a binding subpattern

## The Move

Lesson 098 declared an enum with *unit* variants — names alone. Lesson
058 matched on `Result<T, E>`, whose variants `Ok(T)` and `Err(E)`
*carry a payload*. Today joins the two: a variant of an enum *you
declare* can carry a payload, just like `Ok` and `Err` do:

```rust
enum Brightness {
    Off,
    On(u32),
}

fn main() {
    let dim = Brightness::On(30);
    let dark = Brightness::Off;
    let dim_level = match dim {
        Brightness::Off => 0,
        Brightness::On(n) => n,
    };
    let dark_level = match dark {
        Brightness::Off => 0,
        Brightness::On(n) => n,
    };
    println!("dim_level = {dim_level}, dark_level = {dark_level}");
}
```

`./demo` prints `dim_level = 30, dark_level = 0`. Three pieces — only
the first is new today:

1. **Declare**: `On(u32)` is a *tuple variant*. The variant's name
   is followed by a parenthesized payload type. `Brightness` now has
   two variants of *different shapes*: `Off` (unit) and `On(u32)`
   (tuple, one `u32` payload).
2. **Construct** (new shape): `Brightness::On(30)` is a *call
   expression*. The path `Brightness::On` is the variant constructor
   and behaves like a function — applied to an argument it returns
   a value of type `Brightness`. The Book (Ch6.1) names this
   directly: "the name of each enum variant ... also becomes a
   function that constructs an instance of the enum." The unit form
   `Brightness::Off` is unchanged from lesson 098 — no parens.
3. **Match** (reused from lesson 058): `Brightness::On(n) => n` is
   the *same* arm shape lesson 058 used for `Ok(num)`. Variant path,
   then a parenthesized subpattern; the bare name `n` is a binding
   pattern that captures the payload into the local `n` for the
   arm's body.

(This unlocks reading the rmp target's `enum BigInt { Zero,
Nonzero(Nonzero) }` in `bigint.rs` — exactly today's mixed unit + tuple
form. The payload type `Nonzero` is a struct rather than a primitive;
that composition is one step beyond and is deferred below.)

## Mental Model Delta

- *Before*: "An enum lists named alternatives and each value is exactly
  one variant. Variants have no data of their own. Lesson 058 matched
  `Result`'s `Ok(num)` and `Err(_)`, but I didn't think of those as
  payload-carrying variants of an enum I could declare myself."
- *After*: "A variant can carry a payload. `Variant(T)` in the
  declaration says 'this variant wraps one `T`'. To build one, write
  `E::Variant(value)` — the variant path applied to a value, like
  calling a function. To open it in `match`, write `E::Variant(x)` —
  the same shape lesson 058 used for `Ok(num)`. Unit and tuple
  variants can sit side by side in one enum: the unit form is matched
  bare, the tuple form needs a parenthesized subpattern."

## Prerequisites

- Installed concepts:
  - Lesson 098 (*load-bearing*): `enum Name { V1, V2 }` declaration,
    `Name::Variant` construction for unit variants, exhaustive `match`.
    Today extends by *one rule*: a variant identifier may be followed
    by a parenthesized payload type.
  - Lesson 058 (*load-bearing*): `match` patterns of shape
    `Variant(subpattern)` with a binding subpattern that captures the
    payload. Today reuses this *exactly* on a user-declared enum.
  - Lesson 020 (cited): call-expression shape `name(value)`. Today's
    `Brightness::On(30)` is a call expression with the variant
    constructor on the left.
  - Lesson 030 (cited): `match` machinery and exhaustiveness via E0004.
  - Lesson 062 (cited): `u32`, the payload type used today.
  - Lessons 002, 005, 011, 003: `fn main`, `let`, `println!` `{name}`,
    rustc diagnostic four-part map.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
dim_level = 30, dark_level = 0
```

First match: `dim` holds `Brightness::On(30)`. The `Off` arm doesn't
match. The `On(n)` arm does: `30` binds to `n`, the arm body is the
bare expression `n`, the match's value is `30`. Second match: `dark`
holds `Brightness::Off`; the unit-pattern arm fires, value is `0`.

*Now the contrast.* Save `no_subpattern.rs` with the same enum but a
match arm that treats the tuple variant as if it were unit-style:

```rust
enum Brightness {
    Off,
    On(u32),
}

fn main() {
    let dim = Brightness::On(30);
    match dim {
        Brightness::Off => println!("off"),
        Brightness::On => println!("on"),
    }
}
```

Compile:

```
error[E0532]: expected unit struct, unit variant or constant, found tuple variant `Brightness::On`
  --> no_subpattern.rs:10:9
   |
 3 |     On(u32),
   |     ------- `Brightness::On` defined here
...
10 |         Brightness::On => println!("on"),
   |         ^^^^^^^^^^^^^^ help: use the tuple variant pattern syntax instead: `Brightness::On(_)`
```

Read it with the lesson 003 map: headline E-code **E0532**; location
`no_subpattern.rs:10:9`; caret under `Brightness::On`. The diagnostic
*names the fix*: `Brightness::On(_)` — the parenthesized-subpattern
form (wildcard `_` from lesson 031 reused inside the constructor,
exactly as lesson 058 did with `Err(_)`). The rule: a tuple variant
must be matched with parenthesized subpatterns lined up with its
payload positions.

## What Changed

- A variant can carry a payload. Declaration shape: `Variant(T)`.
  Mixed shapes are allowed in one enum.
- Constructing a tuple variant uses *call-expression* shape:
  `E::Variant(value)`. The path is the constructor. Unit variants
  remain `E::Variant`, no parens (lesson 098).
- Matching a tuple variant uses `E::Variant(subpattern)`. The
  subpattern is a pattern: a binding name like `n` *captures* the
  payload (lesson 058's `Ok(num)`); the wildcard `_` *discards* it
  (lesson 058's `Err(_)`).
- Matching a tuple variant *without* parens fires E0532; the
  diagnostic suggests `E::Variant(_)`.
- Exhaustiveness still applies (E0004). The missing-pattern label
  appears in the new shape, e.g. `Brightness::On(_)`.

## Check Yourself

You write `tiny.rs`:

```rust
enum Coin {
    Heads,
    Tails(i32),
}

fn main() {
    let a = Coin::Tails(42);
    let b = Coin::Heads;
    let va = match a {
        Coin::Heads => -1,
        Coin::Tails(n) => n,
    };
    let vb = match b {
        Coin::Heads => -1,
        Coin::Tails(n) => n,
    };
    println!("va = {va}, vb = {vb}");
}
```

(a) Does `rustc tiny.rs` accept the program (no errors, no warnings)?

(b) What single line does `./tiny` print?

(c) If you change *both* `Coin::Tails(n) => n,` arms to
`Coin::Tails => 0,` (no parens) and recompile, what E-code appears?

(*Answers: (a) Yes — no errors, no warnings; both variants
constructed so `dead_code` does not fire. (b) `va = 42, vb = -1`.
(c) E0532, suggesting `Coin::Tails(_)`.*)

## What To Ignore For Now

Today extends lesson 098 by one rule: variants can carry parenthesized
payloads. Real and deferred:

- *Struct-like variants* `Variant { field: T }` — the third variant
  shape per the Reference grammar.
- *Multiple payload positions* `Variant(T1, T2)` — same rule applied
  twice. Construction and match patterns each list two values.
- *Tuple variants whose payload is a struct or another enum* — the
  rmp target's `enum BigInt { Zero, Nonzero(Nonzero) }` uses a struct
  payload. Today's `u32` payload keeps the move primitive-typed.
- *Discriminants on tuple-variant enums* — Reference rules differ
  from the unit-only case.
- *The wildcard `_` in the subpattern* — `E::Variant(_)`. Today
  centers binding (`n`); `_` appears only in the E0532 fix suggestion.
- *Literal subpatterns* `E::Variant(0)` — composes today's pattern
  with literal patterns.
- *Constructor-as-function-value* — `Brightness::On` (no parens) is
  a function value of type `fn(u32) -> Brightness`. The auxiliary
  appendix probe shows rustc revealing this type.
- *Pattern guards*, *or-patterns*, *nested payload patterns*,
  *`@`-bindings*, *`if let`*, *generic enums* like `Option<T>`,
  *`#[derive(...)]`*, *`pub` on enums and variants*,
  *`use Sign::*;`*, *methods via `impl`*, *recursive enums*. All
  previously deferred items.

## Evidence

See `../evidence/099-enum-tuple-variants.md` for the corpus-quote map,
toolchain, working probe, the centered E0532 contrast, the auxiliary
E0308 "constructor-without-parens" contrast, the corroborating E0004
non-exhaustive-match probe, and the prerequisite-claim summary.
