---
id: 095-struct-with-named-fields
status: accepted
evidence: ../evidence/095-struct-with-named-fields.md
---

# Define a struct with named fields, build one, read a field

## The Move

Lesson 042 used `String`, an instance of a type the standard library
already declared. Lesson 093 named `Vec<i32>` the same way. Today is the
first time you *declare a type yourself*. The move composes three
inseparable pieces:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 7 };
    println!("p.x = {}, p.y = {}", p.x, p.y);
}
```

1. **Declare**: `struct Point { x: i32, y: i32 }` introduces a new
   *struct* type called `Point` with two named *fields*, `x` and `y`,
   each holding an `i32` (lesson 019). The keyword is `struct`. The
   field list lives in curly braces.
2. **Construct**: `Point { x: 3, y: 7 }` is a *struct expression*. It
   produces one *instance* of `Point` by giving each field a value.
   Bound to a name with `let p = ...;` (lesson 005), it sits on the
   right of `=` like any other value.
3. **Read**: `p.x` is a *field access*. The receiver `p`, a dot, and
   the field name `x`. The whole thing is an `i32` value. `p.y`
   reaches the other field the same way.

`./demo` prints `p.x = 3, p.y = 7`.

(This unlocks reading struct declarations in real Rust source — for
example the rmp target repo's `struct Bones { upper: u64, lower: u64 }`
or `struct ParseBigUIntError {}`. The `pub` keyword and `Vec<u64>`
field types you'll see there are deferred.)

## Mental Model Delta

- *Before*: "Type names like `String` and `Vec<i32>` refer to things
  the standard library built. I can construct instances and call
  methods on them, but the *type* itself was always someone else's
  work."
- *After*: "I can declare a type. `struct Name { field: Type, ... }`
  creates a new type whose values carry one piece of data per named
  field. To build one, I write `Name { field: value, ... }`. To read
  a piece of data, I write `instance.field`. The dot is the same dot
  from method calls (lesson 040), but with no parentheses — `p.x`
  reads a field; `p.x()` would be a method call."

## Prerequisites

- Installed concepts:
  - Lesson 002 (`fn main`): the body of `fn main` runs when the
    executable launches.
  - Lesson 005 (`let name = value;`): bind the constructed instance
    to a name.
  - Lesson 011 (`println!` positional `{}`): print the read fields.
  - Lesson 019 (`i32` type annotation): the type used on each field.
    The `field: Type` slot in a struct declaration is the same `: TYPE`
    machinery, repurposed.
  - Lesson 040 (method-call syntax `value.method(args)`): the
    *load-bearing contrast*. Today's `p.x` shares the dot but has no
    parentheses. Field access has no parens; method calls have parens.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Save the program above as `demo.rs` in a fresh directory. Compile and
run:

```console
$ rustc demo.rs
$ ./demo
p.x = 3, p.y = 7
```

The `struct Point { ... }` item lives *outside* `fn main` — struct
declarations are *items*, like `fn` itself. Inside `main`, one `let`
constructs the instance, and `println!` reads `p.x` and `p.y` into its
positional `{}` slots (lesson 011 says any expression can sit in the
argument list, and `p.x` is an expression of type `i32`).

*Now the contrast.* Drop a required field. Save `missing.rs` with the
same `struct Point { x: i32, y: i32 }` and a main of `let p = Point {
x: 3 }; println!("p.x = {}", p.x);`. Compile:

```
error[E0063]: missing field `y` in initializer of `Point`
 --> missing.rs:7:13
  |
7 |     let p = Point { x: 3 };
  |             ^^^^^ missing `y`

error: aborting due to 1 previous error
```

Read it with the lesson 003 map: headline carries E-code `E0063`;
location `missing.rs:7:13`; caret under `Point`. The rule the diagnostic
states is the rule today installs — every field declared in the struct
must be given a value at construction. Restoring `y: 7` compiles.

(Order of `key: value` pairs at construction does not have to match
declaration order — `Point { y: 7, x: 3 }` is the same value. Full
transcripts in the appendix.)

## What Changed

- `struct Name { field: Type, ... }` declares a new type with named
  fields. The `struct` keyword, the type name, and a brace-enclosed
  list of `name: type` pairs separated by commas.
- `Name { field: value, ... }` constructs an *instance* of that type.
  Every declared field must appear exactly once; missing one fires
  E0063 at compile time. Order of pairs is free.
- `instance.field` is a *field access* expression. It reads the value
  stored in that field. The whole expression has the field's declared
  type, so it slots into any expression position — `let`, `println!`'s
  argument list, the right of `=`, anywhere.
- The dot in `p.x` is *not* a method call. Method calls (lesson 040)
  always have parentheses after the name: `s.trim()`, `n.abs()`. Field
  access never does: `p.x`, not `p.x()`.

## Check Yourself

You write `tiny.rs`:

```rust
struct Pair {
    a: i32,
    b: i32,
}

fn main() {
    let q = Pair { a: 10, b: 20 };
    println!("a + b = {}", q.a + q.b);
}
```

(a) Does `rustc tiny.rs` accept the program?

(b) What single line does `./tiny` print?

(c) If you change line 7 to `let q = Pair { a: 10 };` and recompile,
what E-code appears in the headline, and which field name does the
diagnostic say is missing?

(d) `q.a` and `q.a()` both contain a dot. What is the difference?

(*Answers: (a) Yes. (b) `a + b = 30`. (c) E-code `E0063`; the
diagnostic names missing field `b`. (d) `q.a` is field access — it
reads `Pair`'s `a` field. `q.a()` is a method call and would only
compile if `Pair` had a method named `a`. Field access has no
parentheses; method calls always do.*)

## What To Ignore For Now

This lesson installs only the three composed pieces: declare, construct,
read. Every nearby struct feature below is real and deferred:

- *The `pub` keyword* on items or struct fields, and visibility paths
  `pub(super)` / `pub(crate)`. Today's struct and fields are private
  by default.
- *Tuple structs* `struct Foo(T1, T2);` — fields by position, accessed
  with `.0`, `.1`. Different shape.
- *Unit-like structs* `struct Foo;` — no fields at all.
- *Struct update syntax* `Foo { x: 1, ..other }`.
- *The `#[derive(...)]` attribute* (`Debug`, `Clone`, `Copy`,
  `PartialEq`). Today's struct does *not* derive anything.
- *Methods* on a struct via `impl Type { fn ... }`. You know how to
  *call* methods (lesson 040); authoring `impl` is a separate move.
- *Pattern destructuring* of struct values: `let Point { x, y } = p;`,
  `match p { Point { x, y } => ... }`.
- *Mutating a field* via `let mut p = ...; p.x = 5;`. Separate move.
- *Field-init shorthand* `Foo { x, y }` when local names match field
  names (Book Listing 5-5).
- *Generic structs* `struct Foo<T> { ... }`.
- *Lifetime annotations on reference fields* `struct Foo<'a> { name:
  &'a str }`.
- *`String`, `&str`, and `Vec<T>` as field types.* Today's fields are
  primitives already in the graph; owned and borrowed types raise
  ownership and lifetime questions that are deferred.
- *Enums* — the parallel data-type vocabulary (`enum Sign { Positive,
  Negative }`). The natural next move, but not today.
- *Move/drop semantics* when a struct value is passed to or returned
  from a function. Today's probe never moves the instance.
- All previously deferred items.

## Evidence

See `../evidence/095-struct-with-named-fields.md` for the corpus-quote
map, the rustc / system toolchain string, the working probe transcript,
the E0063 missing-field contrast, the E0609 bad-field-access secondary
contrast, the order-doesn't-matter auxiliary probe, and the
prerequisite-claim summary.
