# Evidence — Lesson 145: a `fn` declaration may carry a type parameter `<T>`

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/145-generic-function-type-parameter.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/145-generic-function-type-parameter.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/145-generic-function-type-parameter.transcript.txt`

## Toolchain

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

Probes were typed into `/tmp/eduratchet145/` and compiled with
`rustc <file>`; resulting executables were run from the same
directory. Same host and toolchain as accepted lessons through 144.

## Run context — closure sub-arc step 4 decomposition

Per `iterator-api-coverage.md` §6, the closure sub-arc has five
steps. Steps 1-3 (lessons 142-144) installed: closure literal bound
and called (142, accepted, commit `915347f66`); unannotated closure
first-call-fixes-the-type (143, accepted, commit `50049ef9b`);
closure body capturing an outer binding (144, accepted, commit
`e59fd8401`).

The audit v2 §6 sketches step 4 as a single move "FnMut-bound
parameter on a function." In execution this conflates three
distinct new mechanics:

1. *Generic function syntax* — `fn name<T>(...)`. The `<T>` slot on
   a *function* header. Lesson 114 installed this slot on a *trait*
   header and named the function-header version as deferred —
   "generic functions `fn f<T>(t: T)` (distinct mechanic, same
   `<T>` slot)." This lesson (145) opens that distinct mechanic.
2. *Trait bound on the type parameter* — `<T: Trait>` (or the
   `where` form). Without a bound the body can do almost nothing
   with `T`; with a bound the body can call the trait's methods on
   `T`. Lesson 146.
3. *The parenthesized `FnMut(T) -> R` sugar plus closure-as-argument
   wiring* — the actual closure-passes-through-function shape.
   Lesson 147.

Today's lesson is mechanic (1) only. The lesson body explicitly
flags the decomposition so future audit readers see it. Per the
prompt's process note, the audit document itself is not updated
here; that is an orchestrator action after 145/146/147 land.

## Direct prerequisite — lesson 114 (generic trait parameter)

Lesson 114 installed:

- The angle-bracket grammar `<T>` between an item-header keyword and
  the body, declaring a type parameter scoped to the item.
- The rule that inside the item's body, the parameter name is in
  scope and may sit in any type position (in 114, inside a trait
  method's signature).
- The connection to `Vec<T>` / `Option<T>` *uses* of the same
  syntax on type names.

Today reuses every claim. The new fact relative to 114 is purely
*placement*: the `<T>` slot is on a `fn` header (between the
function name and the parameter list) instead of a `trait` header
(between the trait name and the body). Lesson 114's `move:` field
in the graph entry verbatim names this extension as deferred:
"generic functions `fn f<T>(t: T)` (distinct mechanic, same `<T>`
slot)." Today's lesson is exactly that.

The key conceptual difference from 114: in 114 the impl chooses the
concrete substitution at the impl header (one substitution per
impl block). In a generic function, the *call site* chooses the
concrete substitution (one substitution per call). Today's working
probe witnesses this — the same `fn id<T>` declaration is called
twice with two different concrete types in the same `main`.

## Direct prerequisite — lesson 008 (define and call function)

Lesson 008 installed `fn name() { ... }` defined and `name();`
called. The probe extends 008's header grammar by inserting `<T>`
between the function name and the opening `(`. The *call* shape is
unchanged: `id(5_u32)` and `id(7_i32)` in `fn main` reuse
008's `name(arg);` plus 020's argument-supplied form, with the
twist that the argument's type fixes `T` for that call.

## Direct prerequisite — lesson 020 (function with parameter)

Lesson 020 installed the parameter-list grammar `(p: TYPE)` and the
hard rule that parameter types are mandatory. Today fills the `TYPE`
slot with the type parameter `T`. The mandatory-types rule extends:
the `<T>` declarator must be present *before* `T` can sit in the
parameter type slot, otherwise rustc fires E0425 (Probe 2). Lesson
020's parse-error "if this is a parameter name, give it a type" is
*not* what fires today — `t: T` is a syntactically complete
parameter (lesson 020 form); the issue is that `T` itself, qua type
name, is undeclared.

## Direct prerequisite — lesson 021 (function return value)

Lesson 021 installed `-> RTYPE` as the return-type slot and
`name(args)` as a value-carrying expression usable on the right of
`let`. Today fills `RTYPE` with `T` and uses both calls on the right
of `let` (`let a: u32 = id(5_u32);`, `let b: i32 = id(7_i32);`).
The lesson 021 evidence noted that the implicit-return form
(`{ t }` with no `return`) was deferred there; today's probe uses
it (the implicit-return form was installed at lesson 025).

## Direct prerequisite — lesson 003 (rustc diagnostic map)

Lesson 003 installed the four-part diagnostic map. E0425 is *not* a
new error code today — lesson 005 installed it for `cannot find
value`, lesson 008 reused it for `cannot find function`. Today's
probe extends the same code's third reading: `cannot find type`.
The diagnostic shape is unchanged. The new feature is the `help:
you might be missing a type parameter` block, which is structurally
similar to lesson 020's `help: if this is a parameter name, give
it a type` block — both are rustc proposing the exact diff to
make a name resolvable.

## Cited prereqs (load-bearing-but-restated-elsewhere)

- **Lesson 081**: literal-suffix forms `5_u32`, `7_i32`. Today
  uses these to fix the argument types at the call sites.
- **Lesson 080**: `u32`, `i32` are distinct named members of the
  integer family. Today's lesson rests on the fact that `u32` and
  `i32` are *different* types (so the call sites pick different
  concrete `T`).
- **Lesson 005**: `let name: TYPE = value;`. Used for the two
  annotated bindings in `main` (the annotations are not
  load-bearing for the centered fact — substitution happens
  whether or not `a`/`b` are annotated — but they make the per-call
  outcome auditable).
- **Lesson 025**: implicit-return form `{ t }` (a single-expression
  body without `return` returns the expression's value). Today's
  body is exactly `{ t }`.
- **Lesson 011**: `println!("{} {}", a, b)` consumes two `{}`
  slots.
- **Lesson 002**: `fn main` is the entry point.
- **Lesson 001**: `rustc file.rs`, `./name`; rustc silent on
  success.

## Source — Reference items/functions.md (the canonical generic-function spec)

The corpus file `output/docs/rust/reference/items/functions.md`
contains the formal specification of generic functions. Verified by
reading the file at the cited spans. Three load-bearing passages:

### Lines 124-130 (the *Generic functions* heading and intro)

```text
[[items.fn.generics]]

## Generic functions

[[items.fn.generics.intro]]

A *generic function* allows one or more *parameterized types* to
appear in its signature. Each type parameter must be explicitly
declared in an angle-bracket-enclosed and comma-separated list,
following the function name.
```

This is the source for the lesson body's centered fact: type
parameters are declared in angle brackets *following the function
name*. Verified at lines 124-130.

### Lines 132-140 (the canonical example block)

```rust
fn main() {
// foo is generic over A and B

fn foo<A, B>(x: A, y: B) {
}
}
```

The Reference's canonical example. Today's probe substitutes the
single-parameter form `fn id<T>(t: T) -> T { t }` because the lesson
installs only one type parameter; multiple type parameters are
named-deferred. The Reference example shows two for completeness.

### Lines 142-144 (the body-can-name-T claim)

```text
[[items.fn.generics.param-names]]

Inside the function signature and body, the name of the type
parameter can be used as a type name.
```

Verbatim in the lesson body. This is the load-bearing claim that
`T` is in scope inside the function's signature and body, usable in
any type position. Verified at lines 142-144.

### Lines 159-176 (per-call instantiation, the substitution claim)

```text
[[items.fn.generics.mono]]

When a generic function is referenced, its type is instantiated
based on the context of the reference. For example, calling the
`foo` function here:

fn foo<T>(x: &[T]) where T: Debug {
    // details elided
}

foo(&[1, 2]);

will instantiate type parameter `T` with `i32`.
```

This is the source for the lesson's "per-call substitution" claim.
The Reference's example uses a slice argument `&[i32]`; today's
probe uses a value argument and two distinct call sites with
different concrete types, but the mechanic is identical — the
context of the reference (here, the literal's type) instantiates
`T`. Verified at lines 159-176.

## Source — Book ch10-01-syntax.md (the textbook generic-functions section)

The corpus file `output/docs/rust/book/ch10-01-syntax.md` contains
the textbook treatment of generics. The "In Function Definitions"
subsection runs from line 9 to line 158 (the next subsection
heading "In Struct Definitions" is at line 160). Verified by
reading the file at those bounds.

### Lines 76-90 (the centered "between the name and the parameter list" passage)

```text
When we use a parameter in the body of the function, we have to
declare the parameter name in the signature so that the compiler
knows what that name means. Similarly, when we use a type parameter
name in a function signature, we have to declare the type parameter
name before we use it. To define the generic `largest` function,
we place type name declarations inside angle brackets, `<>`, between
the name of the function and the parameter list, like this:

```rust
fn largest<T>(list: &[T]) -> &T {
```

We read this definition as "The function `largest` is generic over
some type `T`."
```

This is the source for two lesson body claims:

- "between the name of the function and the parameter list" — the
  exact placement claim, lesson body bullet 1.
- "The function ... is generic over some type `T`." — lesson body
  bullet 3, quoted with `largest` (the Book's name) preserved
  rather than substituted to `id`.

Verified at lines 76-90.

### Lines 71-74 (the `T`-by-convention recommendation)

```text
You can use any identifier as a type parameter name. But we'll use
`T` because, by convention, type parameter names in Rust are short,
often just one letter, and Rust's type-naming convention is
UpperCamelCase. Short for *type*, `T` is the default choice of most
Rust programmers.
```

The source for the lesson body's "the Book recommends `T` by
convention" claim. Verified at lines 71-74.

### Lines 436-495 (Performance / Monomorphization, named lightly)

The "Performance of Code Using Generics" section explains
monomorphization. The lesson body names it lightly and the
*What To Ignore For Now* section defers depth (code-bloat trade-off,
generic function instantiation rules). The lesson's claim "rustc
generates a specialized version per substitution at compile time"
is grounded by lines 442-448 of this section verbatim:

```text
Rust accomplishes this by performing monomorphization of the code
using generics at compile time. *Monomorphization* is the process of
turning generic code into specific code by filling in the concrete
types that are used when compiled. In this process, the compiler
does the opposite of the steps we used to create the generic
function in Listing 10-5: The compiler looks at all the places
where generic code is called and generates code for the concrete
types the generic code is called with.
```

Verified at lines 442-448. The lesson body uses the gentler
phrasing "rustc generates a specialized version per substitution at
compile time" rather than the technical term *monomorphization* —
the term is named in *What To Ignore For Now* but is not put on the
critical path.

## Probe 1 — working probe (generic id called with two concrete types)

Source: `observations/145-generic-function-type-parameter.rs`.
Transcript: `observations/145-generic-function-type-parameter.transcript.txt` PROBE 1 block.

```rust
fn id<T>(t: T) -> T { t }

fn main() {
    let a: u32 = id(5_u32);
    let b: i32 = id(7_i32);
    println!("{} {}", a, b);
}
```

Output:

```text
5 7
```

Compile exit 0, run exit 0. Three load-bearing structural facts witnessed:

- The `<T>` declarator parses cleanly between the function name and
  the opening `(`. No syntax error fires.
- `T` in the parameter type slot and the return type slot resolves —
  rustc accepts `t: T` and `-> T` because `T` is the name declared
  by `<T>`.
- The same source declaration `fn id<T>(t: T) -> T { t }` accepts
  both calls — `id(5_u32)` (which substitutes `T = u32`) and
  `id(7_i32)` (which substitutes `T = i32`). The output `5 7` is the
  pair of values returned by the two calls; both pass through the
  generic body unchanged. The type annotations on `a` and `b`
  (`u32` and `i32`) confirm the per-call substitution: `id(5_u32)`
  returns a `u32`, `id(7_i32)` returns an `i32`.

## Probe 2 — negative contrast (drop `<T>`, T becomes unresolved)

Source `nodecl.rs` (in transcript). Output:

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

error[E0425]: cannot find type `T` in this scope
 --> nodecl.rs:1:16
  |
1 | fn id(t: T) -> T { t }
  |                ^ not found in this scope
  |
help: you might be missing a type parameter
  |
1 | fn id<T>(t: T) -> T { t }
  |      +++

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.
```

Compile exit 1. Five grounded facts from rustc's mouth:

- The error code is `E0425` with the inline label `cannot find type
  `T` in this scope`. This is the same E-code lesson 005 installed
  for `cannot find value` and lesson 008 reused for `cannot find
  function`; the noun rotates to `type`. The diagnostic shape is the
  lesson 003 four-part map unchanged.
- The first caret lands on the `T` token at line 1 column 10 — the
  `T` after `t:` in the parameter list. The second caret lands at
  line 1 column 16 — the `T` after `->` in the return type
  position. Both are the lesson 020/021 type slots; both reject the
  unresolved `T`.
- The `help: you might be missing a type parameter` block proposes
  the exact diff: insert `<T>` after the function name. Three `+`
  markers under `<T>` indicate the three inserted characters
  `<`, `T`, `>`. Both errors carry the same `help:` block — rustc
  proposes the *same fix* for both unresolved positions, namely
  declaring the type parameter once.
- The `error: aborting due to 2 previous errors` line counts both
  E0425 errors. The `--explain E0425` trailer appears once at the
  end (lesson 003's rule that an `E####` code's trailer appears
  once per code).
- The single-line modification (`<T>` deletion) is what flips
  acceptance. Probe 1 vs Probe 2 differ in *one segment* — the
  presence of `<T>` after the function name — same body `{ t }`,
  same parameter list `(t: T)`, same return position `-> T`,
  opposite outcomes.

This is the contrastive witness for the lesson's claim "the `<T>`
declarator is what makes `T` a name in scope inside the
signature." E0412's explainer page (corpus file
`output/docs/rust/error_codes/E0412.md`) documents the historical
predecessor: E0412's "no longer emitted" note marks the merge of
"unresolved type name" into the broader E0425 family. The corpus
file's example at lines 21-22 — `fn foo(x: T) {} // type name `T`
is not in scope` — is the exact shape of Probe 2; the corpus's
fix at line 45 — `fn foo<T>(x: T) {}` — is the exact diff rustc
proposes today.

## Side-probe A — substitution mismatch witnesses per-call substitution

Source `mismatch.rs` (in transcript). The substitution machinery
is normally invisible at the call site — Probe 1 just runs. To
make it *visible*, write a call that constrains `T` two
inconsistent ways:

```rust
fn id<T>(t: T) -> T { t }

fn main() {
    let a: i32 = id(5_u32);  // argument forces T=u32; annotation forces T=i32
    println!("{}", a);
}
```

Output:

```text
error[E0308]: mismatched types
 --> mismatch.rs:4:21
  |
4 |     let a: i32 = id(5_u32);
  |                  -- ^^^^^ expected `i32`, found `u32`
  |                  |
  |                  arguments to this function are incorrect
  |
help: the return type of this call is `u32` due to the type of the argument passed
 --> mismatch.rs:4:18
  |
4 |     let a: i32 = id(5_u32);
  |                  ^^^-----^
  |                     |
  |                     this argument influences the return type of `id`
note: function defined here
 --> mismatch.rs:1:4
  |
1 | fn id<T>(t: T) -> T { t }
  |    ^^    ----
help: change the type of the numeric literal from `u32` to `i32`
  |
4 -     let a: i32 = id(5_u32);
4 +     let a: i32 = id(5_i32);
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Compile exit 1. The diagnostic narrates the substitution
machinery directly — the sub-help at column 18 says "the return
type of this call is `u32` due to the type of the argument
passed" and "this argument influences the return type of `id`."
That is rustc making the per-call substitution rule *visible* in
prose. The `note: function defined here` points at the original
declaration's `<T>` and parameter list, the contract source.

The lesson body does *not* center this probe — Probe 2 is the
centered contrast. Side-probe A is documented here because: (a) it
witnesses the per-call substitution mechanic with rustc's own
words; (b) the prompt suggested it as option (b) of the contrast
choices. Probe 2 was preferred for the centered contrast because
it grounds the load-bearing fact (the `<T>` declarator is what
makes `T` a name in scope) more directly.

## Probe-not-needed — pinning the unsubstituted generic

The prompt suggested option (c) — `let _: u32 = id;` — to surface
the unsubstituted generic type. That probe would expose
`fn(T) -> T` as a generic-function item type, which carries
machinery (function-item types, function-pointer types, the `for<T>`
quantifier in error messages) far beyond today's centered fact.
Probes 1 and 2 already establish the centered claim; option (c)
would surface implementation detail of how rustc represents
generic functions internally, which is not on today's path.

## Probe-not-needed — multi-call with non-integer types

A side probe earlier in development used `let s: &str = id("hello"); let n: u8 = id(42_u8);` —
that probe compiles silently and prints `hello 42`, witnessing
substitution across `&str` *and* a `u8`. The lesson body keeps
both calls inside the integer family (lesson 080) because `&str`
introduces string slice machinery the run has not installed.
The integer-only form is sufficient to ground per-call
substitution.

## Probe-not-needed — turbofish

Reference `items/functions.md:178-181` describes the
turbofish form `id::<u32>(5)` for explicit substitution. Today's
probe uses inferred substitution (the argument's type fixes `T`).
The turbofish form is named in *What To Ignore For Now* but
unprobed — adding it would extend the surface beyond the
centered fact.

## Claim-to-evidence mapping

| Lesson claim | Source |
|---|---|
| Every type in a `fn` signature so far has been a specific named concrete type | Lessons 020, 021, 080 |
| `fn id<T>(t: T) -> T { t }` parses and runs | Probe 1 transcript: compile-exit=0, run-exit=0 |
| `id(5_u32)` returns `5_u32`; `id(7_i32)` returns `7_i32`; output `5 7` | Probe 1 output |
| Type parameter declared *after the function name and before the parameter list* | Reference `items/functions.md:130` (verbatim "following the function name") and Book `ch10-01-syntax.md:80-81` (verbatim "between the name of the function and the parameter list") |
| Inside the signature/body, `T` is usable as a type name | Reference `items/functions.md:144` (verbatim) |
| Per-call substitution | Reference `items/functions.md:159-176` (verbatim "When a generic function is referenced, its type is instantiated based on the context of the reference") |
| Convention is `T`, short uppercase | Book `ch10-01-syntax.md:71-74` (verbatim) |
| "The function `largest` is generic over some type `T`" | Book `ch10-01-syntax.md:87-89` (verbatim) |
| The `<T>` here is the same slot lesson 114 installed on a trait header | Lesson 114 graph entry — `move:` field names "generic functions `fn f<T>(t: T)` (distinct mechanic, same `<T>` slot)" verbatim as deferred unlock |
| Probe 2 fires `error[E0425]: cannot find type `T` in this scope` | Probe 2 transcript: rustc emits exact code + headline |
| Caret on `T` at line 1 column 10 (parameter type) and column 16 (return type) | Probe 2 transcript: `--> nodecl.rs:1:10` and `--> nodecl.rs:1:16` |
| `help: you might be missing a type parameter` proposes `fn id<T>(t: T) -> T` | Probe 2 transcript: verbatim, with three `+` markers under `<T>` |
| E0425 is the same code lessons 005/008 installed | Lesson 005, lesson 008 evidence appendices; corpus `error_codes/E0425.md` |
| rustc generates a specialized version per substitution at compile time | Book `ch10-01-syntax.md:442-448` (verbatim) |
| Side-probe A: substitution mismatch fires E0308 with "this argument influences the return type" | Side-probe A transcript: rustc emits exact code + sub-help text |

## Older supporting lessons (named only)

The following accepted lessons are cited in the lesson body or
prerequisites but their exact prereq claims are restated above or
in the lesson's own Prerequisites bullets:

- 081-integer-literal-forms — `5_u32`, `7_i32` literal-suffix
  forms.
- 080-integer-type-family — `u32`, `i32` as distinct family
  members.
- 005-let-binding — `let name: TYPE = value;`.
- 025-implicit-return — single-expression body returns the
  expression's value (today's body `{ t }`).
- 011-println-positional-args — `println!("{} {}", a, b)`.
- 002-fn-main-entry-point — `fn main` is the entry point.
- 001-rustc-compile-and-run — `rustc file.rs`, `./name`.

## Deliberate scope discipline

The prompt named eight things to NOT touch. The lesson body's *What
To Ignore For Now* section names all eight explicitly:

1. Trait bounds (`<T: Trait>`) — deferred to lesson 146.
2. The `FnMut(T) -> R` parenthesized sugar — deferred to lesson 147.
3. Generic struct types (`struct S<T>`) — separate mechanic, named
   in deferred bullet.
4. Lifetime parameters (`<'a>`) — deferred wholesale.
5. The `where` clause form — same mechanic, deferred (named under
   trait bounds).
6. Multiple type parameters — same mechanic extended, deferred.
7. Closures as arguments to generic functions — lesson 147's
   territory, deferred under trait bounds.
8. Monomorphization at depth — named lightly, depth deferred.

The probe body `{ t }` is exactly the move-it-around-only form the
prompt described: without a trait bound, the body cannot call any
methods on the generic value; the only thing it can do is return
the parameter unchanged. This is structurally why "trait bounds"
is the natural next move (lesson 146).

The lesson uses lesson 081's literal-suffix forms (`5_u32`,
`7_i32`) per the prompt's recommendation; this fixes the argument
type unambiguously at the call site without relying on any
substitution-from-annotation chain.

## Run-context handoff to lessons 146 and 147

Lessons 142-145 install:

- Closure literal syntax with annotated and unannotated parameters
  (142, 143).
- First-call-fixes-the-type rule for unannotated parameters (143).
- The closure/`fn` asymmetry centered on capture (144).
- Generic function syntax `fn name<T>(...)` and per-call
  substitution (today).

Lesson 146 will add the trait-bound machinery `fn f<T: Trait>(...)`
on top of today's mechanic. Lesson 147 will add the parenthesized
`Fn(T) -> R` / `FnMut(T) -> R` sugar (which is itself a special
case of trait bounds with a particular grammar) plus the
closure-as-argument call site. After 147, the closure sub-arc
prereqs are complete and the first closure-driven Iterator method
(`for_each`, `map`, ...) becomes teachable.

Today's `unlocks` lists lessons 146 and 147 directly, plus the
deferred bullets above.
