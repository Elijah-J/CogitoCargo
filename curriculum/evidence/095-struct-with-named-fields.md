# Evidence — 095-struct-with-named-fields

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -srvm` -> `Darwin 24.5.0 Darwin Kernel Version 24.5.0:
  Tue Apr 22 19:53:26 PDT 2025; root:xnu-11417.121.6~2/RELEASE_X86_64
  x86_64`
- Probes run in `/tmp/eduratchet095/` on this host. Same toolchain
  family as recent accepted lessons (082-094).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/095-struct-with-named-fields.rs`
is the working three-piece probe verbatim, with header comments naming
the expected output and the contrast probes captured below.

## Sources

### `output/docs/rust/book/ch05-01-defining-structs.md`

The Book's *Defining and Instantiating Structs to Structure Related
Data* chapter. Three load-bearing passages:

#### Lines 4-8 — what a struct is

> Structs are similar to tuples, discussed in "The Tuple Type"
> section, in that both hold multiple related values. Like tuples, the
> pieces of a struct can be different types. Unlike with tuples, in a
> struct you'll name each piece of data so it's clear what the values
> mean. Adding these names means that structs are more flexible than
> tuples: You don't have to rely on the order of the data to specify
> or access the values of an instance.

Corpus warrant for the lesson's *Mental Model Delta* "After" framing
(named fields; not order-dependent like tuples). Today's probe uses
the order-doesn't-matter property in Probe 3.

#### Lines 10-14 — the declaration syntax

> To define a struct, we enter the keyword `struct` and name the
> entire struct. A struct's name should describe the significance of
> the pieces of data being grouped together. Then, inside curly
> brackets, we define the names and types of the pieces of data, which
> we call *fields*.

Corpus warrant for the lesson's piece (1) framing — *Declare*: the
`struct` keyword, the struct name, curly braces with named-typed
fields. The Book's Listing 5-1 (`struct User { active: bool, ... }`)
is the canonical declaration shape; today's `struct Point { x: i32,
y: i32 }` is the same shape with different field names and the
audience's installed primitive type.

#### Lines 31-38 — the construction syntax

> To use a struct after we've defined it, we create an *instance* of
> that struct by specifying concrete values for each of the fields.
> We create an instance by stating the name of the struct and then
> add curly brackets containing *`key: value`* pairs, where the keys
> are the names of the fields and the values are the data we want to
> store in those fields. We don't have to specify the fields in the
> same order in which we declared them in the struct.

Corpus warrant for the lesson's piece (2) framing — *Construct*: the
struct expression `Name { key: value, ... }`. The clause "We don't
have to specify the fields in the same order" is the corpus warrant
for the Probe 3 order-free property the lesson mentions in *Try It*'s
parenthetical and witnesses below.

#### Lines 62-66 — the field-access syntax

> To get a specific value from a struct, we use dot notation. For
> example, to access this user's email address, we use `user1.email`.

Corpus warrant for the lesson's piece (3) framing — *Read*: the
field-access expression `instance.field`. The Book's `user1.email` is
the same shape as today's `p.x`.

### `output/docs/rust/reference/items/structs.md`

The Reference's *Structs* item. Lines 14-15, 22 are load-bearing for
the formal grammar; line 30 names the keyword:

> [StructStruct] →
>     struct IDENTIFIER GenericParams? WhereClause? ( { StructFields? } | ; )
> [StructFields] → StructField ( , StructField )* ,?
> [StructField] → OuterAttribute* Visibility? IDENTIFIER : Type

Reference warrant: the declaration grammar admits a brace-enclosed
list of `IDENTIFIER : Type` fields separated by commas, with optional
trailing comma. Generic params, where clauses, outer attributes, and
visibility modifiers are all optional in the grammar — and all
deferred by today's lesson.

Lines 38-44 — the Reference's compact example combining all three
pieces in one block:

> ```rust
> struct Point {x: i32, y: i32}
> let p = Point {x: 10, y: 11};
> let px: i32 = p.x;
> ```

This is the closest-fit corpus exemplar to today's probe shape — the
Reference itself names declaration, construction, and field access
together. Today's probe uses `x: 3, y: 7` (different values) and
inlines the field access in `println!` instead of binding it.

### `output/docs/rust/reference/expressions/struct-expr.md`

Lines 25-27 — the construction-must-cover-all-fields rule (read in
combination with the union-only single-field exception in lines
108-110, which does not apply to today's structs):

> A *struct expression* creates a struct, enum, or union value. It
> consists of a path to a struct, enum variant, or union item
> followed by the values for the fields of the item.

Lines 100-110 — the *Field struct expression* subsection:

> A struct expression with fields enclosed in curly braces allows you
> to specify the value for each individual field in any order. The
> field name is separated from its value with a colon.

Reference warrant for the Probe 2 missing-field rule today centers
("every field declared in the struct must be given a value at
construction"): a struct expression specifies "the value for each
individual field" — every field must be specified for the construction
to succeed. The Book is operationally explicit about the rule via
Listing 5-1's complete-field requirement; the Reference codifies it.

### `output/docs/rust/reference/expressions/field-expr.md`

Lines 9-10 — the field-access grammar:

> [FieldExpression] → Expression . IDENTIFIER

Lines 24-26 — the load-bearing field-vs-method-call distinction:

> Field expressions cannot be followed by a parenthetical
> comma-separated list of expressions, as that is instead parsed as a
> method call expression.

Reference warrant for the lesson's *no-parens-after-the-dot* rule:
`p.x` is parsed as field access; `p.x(...)` is parsed as a method call.
The two share the dot but are syntactically distinct, exactly as the
lesson's *What Changed* bullet states.

### `output/docs/rust/error_codes/E0063.md`

The error code page for E0063 — the missing-field error today's
contrast probe witnesses:

> A struct's or struct-like enum variant's field was not provided.
>
> Erroneous code example:
>
> ```rust
> struct Foo {
>     x: i32,
>     y: i32,
> }
>
> fn main() {
>     let x = Foo { x: 0 }; // error: missing field: `y`
> }
> ```

Direct corpus warrant: the lesson's E0063 contrast probe has the same
shape as the corpus erroneous-code example. The probe substitutes
`Point` for `Foo` (matching the Book's running shape) and uses
`x: 3` in place of `x: 0`. The diagnostic message wording on this
host (`missing field \`y\` in initializer of \`Point\``) is rustc
1.95.0's exact phrasing; the corpus example's source comment uses the
shorter `missing field: \`y\`` form. Both name the same field name
and the same E-code.

### `output/docs/rust/error_codes/E0609.md`

The error code page for E0609 — the bad-field-access secondary contrast
captured in Probe 4 below:

> Attempted to access a nonexistent field in a struct.

Direct corpus warrant for Probe 4's negative result. The lesson body
does not exercise this contrast (E0063 is the centered contrast
probe), but the appendix records the E0609 transcript so the
red-team can verify that the field-names-belong-to-the-type rule is
load-bearing for the lesson's mental model and not just an
unverifiable assertion.

## Probes

### Probe 1 — working: declare, construct, read

Source (`/tmp/eduratchet095/demo.rs`, also at
`observations/095-struct-with-named-fields.rs`):

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

Compile transcript:

```
$ rustc demo.rs
(no output; exit 0)
$ ls
demo  demo.rs
$ ./demo
p.x = 3, p.y = 7
(exit 0)
```

Witnesses:

- **Declaration** is accepted: `struct Point { x: i32, y: i32 }` at
  module scope is a valid struct item. `rustc` is silent on success,
  consistent with lesson 001.
- **Construction** is accepted: `Point { x: 3, y: 7 }` produces a
  value of type `Point`, and `let p = ...;` binds it (lesson 005's
  binding form, unmodified).
- **Field access** is accepted: `p.x` and `p.y` each evaluate to an
  `i32`, sit in `println!`'s positional `{}` slots (lesson 011), and
  print as `3` and `7`. The output line is `p.x = 3, p.y = 7`,
  exactly what the source string template encodes.
- All three pieces compile under one `rustc` invocation. There is no
  separate "declaration" build step; structs are *items* like `fn`,
  parsed and type-checked alongside the rest of the file.

### Probe 2 — contrast: missing field at construction (E0063)

Source (`/tmp/eduratchet095/missing.rs`):

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3 };
    println!("p.x = {}", p.x);
}
```

Compile transcript:

```
$ rustc missing.rs
error[E0063]: missing field `y` in initializer of `Point`
 --> missing.rs:7:13
  |
7 |     let p = Point { x: 3 };
  |             ^^^^^ missing `y`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0063`.
(exit 1)
```

Witnesses:

- *`error:` headline; build aborts.* `error[E0063]` is the headline
  E-code for "missing field." Lesson 003's diagnostic-map vocabulary
  reads this byte-for-byte. `error: aborting due to 1 previous error`
  is the trailer (lesson 069's contrast against `warning:`).
- *Caret under `Point`*, the struct-name token of the construction
  expression. The diagnostic names the field as `\`y\`` in plain
  English.
- *Build does not produce an executable.* `ls` after the failed
  compile shows only `missing.rs` (no new `missing` binary).
- *The lesson's centered rule is what the diagnostic states.* "Every
  field declared in the struct must be given a value at construction"
  is the operational interpretation of `missing field \`y\` in
  initializer of \`Point\``. The corpus E0063 page (lines 19-29)
  states the fix in the same shape: "Each field should be specified
  exactly once" with the corrected `Foo { x: 0, y: 0 }` example.

### Probe 3 — auxiliary: order of `key: value` pairs is free

Source (`/tmp/eduratchet095/reordered.rs`):

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { y: 7, x: 3 };
    println!("p.x = {}, p.y = {}", p.x, p.y);
}
```

Compile transcript:

```
$ rustc reordered.rs
(no output; exit 0)
$ ./reordered
p.x = 3, p.y = 7
(exit 0)
```

Witness: the construction `Point { y: 7, x: 3 }` (with `y` first)
produces the same value as Probe 1's `Point { x: 3, y: 7 }` (with
`x` first). Output is byte-identical. This is the corpus warrant
(Book ch05-01 line 34: "We don't have to specify the fields in the
same order in which we declared them in the struct") observed.

The lesson body mentions this as a parenthetical aside in *Try It*
without making it the centered claim.

### Probe 4 — auxiliary contrast: bad field access (E0609)

Source (`/tmp/eduratchet095/bad_field.rs`):

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 7 };
    println!("p.z = {}", p.z);
}
```

Compile transcript:

```
$ rustc bad_field.rs
error[E0609]: no field `z` on type `Point`
 --> bad_field.rs:8:28
  |
8 |     println!("p.z = {}", p.z);
  |                            ^ unknown field
  |
help: a field with a similar name exists
  |
8 -     println!("p.z = {}", p.z);
8 +     println!("p.z = {}", p.x);
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0609`.
(exit 1)
```

Witnesses:

- The *field-names-belong-to-the-type* rule the lesson's *Mental Model
  Delta* installs is load-bearing: rustc rejects `p.z` because `z` is
  not one of `Point`'s declared fields. The headline names the type
  (`on type \`Point\``) and the bad name (`\`z\``). The `help:` line
  even suggests `p.x` as a similarly-named existing field.
- The lesson body does *not* center this contrast (E0063 is the
  centered contrast for "every field must be initialized"). Probe 4
  is recorded here for red-team verification that the alternative
  contrast the audit named is also empirically witnessed on this
  host.

### Probe 5 — adjacent-but-distinct: field-vs-method-call shape

The lesson's load-bearing distinction "field access has no
parentheses; method calls have parentheses" is grounded by lesson 040
(method-call form) and the Reference's `expr.field.not-method-call`
clause (cited above). It does not require a separate compile probe —
both surfaces have already been observed in earlier accepted lessons
(`s.trim()` in lesson 055, `n.abs()` in lesson 040 itself, and `p.x`
in this lesson's Probe 1) — but the lesson's *What Changed* bullet
recapitulates the rule for navigability.

A confirming negative-syntax probe (writing `p.x()` on a struct with
no method named `x`) would fire E0599 ("no method named `x` on type
`Point`"). That probe is not captured because (a) the lesson does not
center the failure mode, only the syntactic distinction, and (b)
exhibiting the failure requires the audience to read an E0599
diagnostic that the graph has not yet installed. The
syntactically-stated rule from the Reference is sufficient grounding
for the assertion.

## Prerequisite-claim summary

Direct prerequisites — each prerequisite's load-bearing claim used by
this lesson, summarized in 1-3 bullets per the run README:

- **Lesson 002 — `fn main`** (cited). `fn main()` is the entry-point
  function whose body runs when the executable launches; the probe
  uses one `fn main` block, unchanged.

- **Lesson 005 — `let name = value;`** (load-bearing). `let p =
  expression;` binds `p` to the value of `expression`. Today's `let
  p = Point { x: 3, y: 7 };` is the same form with a struct-expression
  right-hand side; nothing about the binding form changes.

- **Lesson 011 — `println!` with positional `{}`** (load-bearing).
  Each `{}` in the format string is a positional placeholder consumed
  by the next extra argument. Lesson 011 explicitly says any
  expression can sit in the argument list (its probe used `a + b`
  directly). Today plugs `p.x` and `p.y` into the slots — both are
  expressions of type `i32`, exactly the kind of value lesson 011
  prints.

- **Lesson 019 — type annotation `i32`** (load-bearing). `i32` is
  the 32-bit signed integer type. Today's struct fields are typed
  `i32`. The `field: Type` slot in the struct declaration is the
  same `: TYPE` machinery from lesson 019, repurposed: lesson 019
  applied it to a `let` binding; today applies it to a struct field
  declaration.

- **Lesson 040 — method-call syntax `value.method(args)`**
  (load-bearing). The dot operator binds a *receiver* expression to
  a method name with a parenthesized argument list. Today's *no
  parens after the dot for field access* rule explicitly contrasts
  against lesson 040: `p.x` reads a field; `p.x()` would call a
  method named `x`. The two surfaces share the dot but differ on
  the trailing parenthesized argument list.

Older supporting lessons (cited only, no specific claim load-bearing):

- Lesson 001 (`rustc file.rs`; silent on success; produced
  executable). Used by the probe transcripts.
- Lesson 003 (rustc diagnostic four-part map). Used to read Probe 2's
  E0063 transcript and Probe 4's E0609 transcript.
- Lesson 042 (`String::new()` worked bare). Used in *The Move*'s
  motivating sentence: "Lesson 042 used `String`, an instance of a
  type the standard library already declared."
- Lesson 069 (`warning:` vs `error:` category). Used implicitly in
  Probe 2 — the trailer is `error: aborting due to 1 previous error`,
  not `warning: 1 warning emitted`.
- Lesson 093 (standard library prelude). Used in *The Move*'s
  motivating sentence as a parallel: "Lesson 093 named `Vec<i32>` the
  same way."

## Contrast-probe coverage

The lesson's contrastive claim is "every field declared in the struct
must be given a value at construction; missing one fires a compile
error." This is witnessed empirically by Probe 2 (the E0063
contrast).

A second, secondary contrastive claim — "field names are part of the
type, not free identifiers" — is witnessed by Probe 4 (the E0609
contrast). The lesson does not center this contrast in the body
(centering would dilute the move's three-piece focus), but the probe
is captured here so the rule the lesson's *Mental Model Delta*
installs ("instance.field reads a piece of data" — implicitly, a
*declared* piece of data) is empirically verified.

## Notes on deferred items

The lesson defers (and this appendix does not probe further):

- Visibility modifiers (`pub`, `pub(super)`, `pub(crate)`). Reference
  `items/structs.md` line 22 names `Visibility?` as an optional grammar
  position; today's struct uses the default-private form. The rmp
  target uses `pub` and `pub(super)` extensively — those are the next
  ready-now items in the audit's chain (items 2 and 3 of audit §11).
- Tuple structs `struct Foo(T1, T2);` and unit-like structs `struct
  Foo;` — Book ch05-01 lines 277-344 cover both. Distinct shapes;
  separate moves.
- Struct update syntax `Foo { x: 1, ..other }` — Book ch05-01 lines
  181-273 introduce it, with explicit move-vs-copy semantics that
  require ownership vocabulary not yet installed.
- The `#[derive(...)]` attribute — every struct in the rmp target
  carries one (`#[derive(Clone)]` etc.); deferred until the trait
  arc opens (Q07 in deferred-queue.md).
- Pattern destructuring of struct values — `let Point { x, y } = p;`
  and `match p { Point { x, y } => ... }`. Different machinery.
- Field-init shorthand — Book Listing 5-5. Cosmetic shortcut.
- Generic structs `struct Foo<T> { ... }` — type parameters.
- Lifetime-annotated fields — Book ch05-01 lines 346-418 explicitly
  cover the `struct User { username: &str, ... }` failure mode (E0106
  "missing lifetime specifier") and explicitly say "Chapter 10" for
  the fix; the audience is several arcs away from this.
- Struct fields holding `String`, `&str`, or `Vec<T>`. Today's
  fields are primitives (`i32`); owned and borrowed types as struct
  fields raise ownership, lifetime, and prelude-membership
  questions deferred until those arcs land.
- Mutating a field via `let mut p = ...; p.x = 5;` — Book Listing 5-3
  shows this; the `mut` keyword is installed (lesson 006) and `&mut`
  is installed (lesson 047), but field-mutation as a centered move is
  separate.
- Move/drop semantics for struct values — passing or returning a
  struct by value involves move semantics; today's probe never moves
  the instance.
- Enums — the parallel data-type vocabulary. Audit's item 4 in §11.

None of these are load-bearing for the centered claim "declare a
struct with named fields, construct an instance, read a field."
