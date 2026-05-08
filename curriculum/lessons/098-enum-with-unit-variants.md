---
id: 098-enum-with-unit-variants
status: accepted
evidence: ../evidence/098-enum-with-unit-variants.md
---

# Define an enum with unit variants, construct one, match on it

## The Move

Lesson 095 declared a struct with named fields — *one* type whose value
holds *one of each* field's data. An enum is the parallel data-type
shape: one type whose value is *one of* a fixed list of named
alternatives. The standard library has handed you enums before
(`Ordering` in lesson 051, `Result` in lessons 052 and 058), but today
you declare one yourself. Three composed pieces:

```rust
enum Sign {
    Positive,
    Negative,
}

fn main() {
    let up = Sign::Positive;
    let down = Sign::Negative;
    let label_up = match up {
        Sign::Positive => "+",
        Sign::Negative => "-",
    };
    let label_down = match down {
        Sign::Positive => "+",
        Sign::Negative => "-",
    };
    println!("up = {label_up}, down = {label_down}");
}
```

1. **Declare**: `enum Sign { Positive, Negative }` at module scope
   introduces a new type called `Sign` whose values are exactly one
   of two named *variants*, `Positive` or `Negative`. The keyword is
   `enum`. Variant names live in curly braces, separated by commas.
2. **Construct**: `Sign::Positive` is a value of type `Sign` — the
   one that picks the `Positive` alternative. The same `::` you saw
   in lesson 043's `module::name` paths and in lesson 051's
   `Ordering::Less`. The enum name on the left, the variant name on
   the right.
3. **Match**: `match up { Sign::Positive => "+", Sign::Negative => "-"
   }` is lesson 030's `match` machine, unchanged, with the variants of
   your own enum as the patterns. Both arms must appear — exhaustiveness
   (lesson 030's E0004) is enforced just like before.

`./demo` prints `up = +, down = -`.

(This unlocks reading the rmp target's `enum Sign { Positive, Negative
}` in `bigint.rs` and `pub enum ConvertOutNumError { WouldOverflow }`
in `biguint/convert.rs`. The same target's `enum BigInt { Zero,
Nonzero(Nonzero) }` is *tuple-variant* shape — variants that carry a
payload — and is deferred below.)

## Mental Model Delta

- *Before*: "Enums are types the standard library hands me —
  `Ordering`, `Result`, `Option`. I can construct their variants and
  match on them, but the *type* itself was always someone else's
  declaration."
- *After*: "I declare an enum the way I declare a struct, but for the
  *one-of-many* shape instead of the *all-of-these* shape. A struct
  groups several pieces of data (one of each field) into one value;
  an enum lists several alternatives, and each value is exactly one
  of them. `enum Name { V1, V2 }` declares; `Name::Vk` reaches one
  variant value; `match` lists those same variants as patterns."

## Prerequisites

- Installed concepts:
  - Lesson 030 (*load-bearing*): the `match` form, with arms
    `pattern => arm_expression,`, arms sharing a type, and
    exhaustiveness enforced via E0004.
  - Lesson 058 (*load-bearing*): `match` on enum variants. The
    matching mechanism is unchanged today; the only new thing is
    that you wrote the enum yourself.
  - Lesson 051 (`Ordering`'s three unit variants), lesson 043
    (`module::name` `::` shape — today's `Sign::Positive` reuses
    that shape with an enum on the left and a variant on the right),
    lesson 095 (parallel data-type lesson, declaring `struct` with
    named fields).
  - Lesson 002 (`fn main`), lesson 005 (`let`), lesson 011
    (`println!` `{}`), lesson 003 (rustc diagnostic four-part map).
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs` in a fresh empty directory. Compile
and run:

```console
$ rustc demo.rs
$ ./demo
up = +, down = -
```

The `enum Sign { ... }` item lives *outside* `fn main`, like
lesson 095's `struct Point` — enums are *items*. Inside `main`, two
`let` bindings name two values of type `Sign`. Each `match` walks its
scrutinee against the two patterns in order, and the matching arm's
`&str` (the type of string literals — lesson 055 named it) becomes
the value of the whole `match`, which `let label_up = ...;` and
`let label_down = ...;` bind.

*Now the contrast.* Save `bad_variant.rs` with the same
`enum Sign { Positive, Negative }` and a main that tries to construct
a variant the enum does *not* declare:

```rust
fn main() {
    let mystery = Sign::Maybe;
    // ... matched same as above
}
```

Compile:

```
error[E0599]: no variant or associated item named `Maybe` found for enum `Sign` in the current scope
 --> bad_variant.rs:7:25
  |
1 | enum Sign {
  | --------- variant or associated item `Maybe` not found for this enum
...
7 |     let mystery = Sign::Maybe;
  |                         ^^^^^ variant or associated item not found in `Sign`

error: aborting due to 1 previous error
```

Read it with the lesson 003 map: headline carries E-code `E0599`;
location at `bad_variant.rs:7:25`; caret under `Maybe`. The rule the
diagnostic states is the rule today installs — variant names *belong
to the enum*. The path `Sign::Maybe` does not produce a value because
the enum has no such variant. (Compare with lesson 095's E0609 "no
field on type": fields belong to the struct the same way variants
belong to the enum.)

(A second contrast — removing one arm from the `match` — fires the
familiar `error[E0004]: non-exhaustive patterns: \`Sign::Negative\`
not covered`. Same exhaustiveness rule from lesson 030, now naming a
variant of *your* enum. Full transcript in the appendix.)

## What Changed

- `enum Name { V1, V2, ... }` declares a new type whose values are
  exactly one of the listed variants. The `enum` keyword, a type
  name, and a brace-enclosed comma-separated list of variant
  identifiers.
- `Name::Variant` is one specific value of type `Name`. Same `::`
  separator as lesson 043's `module::name`; here the namespace is
  "variants of an enum" instead of "items of a module."
- `match` on a value of type `Name` lists those variants as
  patterns. Lesson 030's exhaustiveness rule still applies — every
  variant must appear (or a wildcard, deferred today).
- A bad variant path fires `error[E0599]: no variant or associated
  item named ... found for enum`. Variant names are part of the
  type, the same way field names are part of a struct (lesson 095).

## Check Yourself

You write `tiny.rs`:

```rust
enum Switch {
    On,
    Off,
}

fn main() {
    let toggle = Switch::On;
    let state = match toggle {
        Switch::On => 1,
        Switch::Off => 0,
    };
    println!("state = {state}");
}
```

(a) Does `rustc tiny.rs` accept the program?

(b) What single line does `./tiny` print?

(c) If you change line 7 to `let toggle = Switch::Maybe;` and
recompile, what E-code appears in the headline?

(d) If instead you delete the `Switch::Off => 0,` arm and recompile,
what E-code appears, and which variant does the headline name as
not covered?

(*Answers: (a) Yes. (b) `state = 1`. (c) E0599 (no variant `Maybe`
found for enum `Switch`). (d) E0004, naming `\`Switch::Off\` not
covered`.*)

## What To Ignore For Now

Today installs only unit variants — variants with no data. Real and
deferred:

- *Tuple variants* `Variant(T1, T2)` — the natural follow-on, the
  shape of `Ok(T)` / `Err(E)` from lesson 058 and the rmp target's
  `enum BigInt { Zero, Nonzero(Nonzero) }`.
- *Struct-like variants* `Variant { field: T }`.
- *Discriminants* `Variant = 1` and casting `Sign::Positive as i32`.
- *`#[derive(...)]`* on enums (`Debug`, `Clone`, `Copy`, `PartialEq`).
- *`pub` on enums and variants.* Today's `Sign` is private; variants
  of a `pub` enum are public by default per the Reference, but that
  is not exercised here.
- *`if let Pattern = expr { ... }`* — alternative single-arm form.
- *Generic enums* like the shape of `Option<T>`.
- *The match wildcard `_` on user-defined enums.* Today's match is
  exhaustive by name; lesson 031's `_` is not used.
- *Pattern guards* `Pattern if cond => ...`, *recursive enums*,
  *methods on enums via `impl`*, *`use Sign::*;`* glob imports.
- All previously deferred items.

## Evidence

See `../evidence/098-enum-with-unit-variants.md` for the
corpus-quote map, the rustc / system toolchain string, the working
probe transcript, the E0599 bad-variant contrast probe, the E0004
non-exhaustive-match secondary contrast probe, and the
prerequisite-claim summary.
