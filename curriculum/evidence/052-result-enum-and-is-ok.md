# Evidence — 052-result-enum-and-is-ok

Audit appendix for `lessons/052-result-enum-and-is-ok.md`. Holds the
corpus-quote map, the toolchain string, the working- and broken-
contrast probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/052-result-enum-and-is-ok.rs`); the broken-contrast
  `.rs` is not committed — its transcript below is the artifact.

## Sources

### `output/docs/rust/std/result/enum.Result.md`

The std-library page for `Result`. Primary source. Three load-bearing
spans.

Lines 6-11 (the type's canonical declaration):

> ```
> pub enum Result<T, E> {
>     Ok(T),
>     Err(E),
> }
> ```

Direct corpus statement that (a) `Result` is declared with the `enum`
keyword, (b) it has exactly two variants named `Ok` and `Err`, (c) the
variants have payload — `Ok(T)` carries a `T` and `Err(E)` carries an
`E`, and (d) `Result` is *generic* over two type parameters `T` and
`E`. The lesson's main concept rests directly on this declaration: the
"composite move" surfaces (a) by naming `Result` an enum, (b) directly
listing both variants, (c) by introducing the `Variant(T)` declaration
shape, and (d) by introducing `<T, E>` as type parameters and showing
the use site `Result<i32, i32>`.

Lines 15 (the type's plain-English description):

> `Result` is a type that represents either success (`Ok`) or failure
> (`Err`).

Audience-level statement of what `Result` is *for*. The lesson body's
"`Ok(T)` for success and `Err(E)` for failure" rephrases this. The
lesson's working-probe `parity` function is shaped to this exact split
— the even branch builds `Ok(n)` (success), the odd branch builds
`Err(n)` (failure).

Lines 23-31 (the variant docs):

> ### Ok(T)
> Contains the success value
>
> ### Err(E)
> Contains the error value

Per-variant gloss. The lesson does not surface the *exact* "success
value / error value" wording but does install the success-vs-failure
split.

Lines 41-53 (the `is_ok` method's signature, summary, and example):

> #### pub const fn is_ok(&self) -> bool
>
> Returns `true` if the result is `Ok`.
>
> ##### Examples
>
> ```
> let x: Result<i32, &str> = Ok(-3);
> assert_eq!(x.is_ok(), true);
>
> let x: Result<i32, &str> = Err("Some error message");
> assert_eq!(x.is_ok(), false);
> ```

Direct corpus precedent for (a) the method's signature `is_ok(&self)
-> bool` (a method on `Result<T, E>` returning a `bool`), (b) the
audience-level claim "returns `true` if the result is `Ok`," and (c)
the empirical truth-table — `Ok(_)` → `true`, `Err(_)` → `false`. The
lesson's working probe corroborates (c) directly: `parity(4)` returns
`Ok(4)` and `a.is_ok()` prints `true`; `parity(7)` returns `Err(7)`
and `b.is_ok()` prints `false`.

Calibration: the `Result` page documents many more methods
(`is_ok_and`, `is_err`, `is_err_and`, `ok`, `err`, `as_ref`, `map`,
`unwrap`, `expect`, `unwrap_or`, `and_then`, etc.) and many trait
implementations. *All* of them are deferred under *What To Ignore For
Now*. The lesson installs only `.is_ok()`. Lines 78-92 document
`is_err` (the trivial sibling) — the lesson mentions `.is_err()` by
name without exercising it.

### `output/docs/rust/book/ch09-02-recoverable-errors-with-result.md`

The Book chapter on recoverable errors with `Result`. Audience-level
introduction. Three load-bearing spans.

Lines 10-21 (the `Result` declaration, presented to learners):

> Recall from "Handling Potential Failure with `Result`" in Chapter 2
> that the `Result` enum is defined as having two variants, `Ok` and
> `Err`, as follows:
>
> ```rust
> enum Result<T, E> {
>     Ok(T),
>     Err(E),
> }
> ```

Audience-level corpus statement that `Result` is an enum, has two
variants `Ok` and `Err`, and the declaration has `<T, E>` plus the
payload-carrying forms `Ok(T)` and `Err(E)`. The lesson's "(2)" claim
rests on this. Calibration: the Book elides `pub` (and the page
indents inside an outer `fn main()`); the std-page declaration above
is the canonical authoritative form.

Lines 23-30 (the *generic type parameters* gloss):

> The `T` and `E` are generic type parameters: We'll discuss generics
> in more detail in Chapter 10. What you need to know right now is
> that `T` represents the type of the value that will be returned in
> a success case within the `Ok` variant, and `E` represents the type
> of the error that will be returned in a failure case within the
> `Err` variant. Because `Result` has these generic type parameters,
> we can use the `Result` type and the functions defined on it in
> many different situations where the success value and error value
> we want to return may differ.

Direct audience-level license for the lesson's "*type parameters* —
placeholders that get filled in at use sites" framing. The Book's
"generic type parameters" wording maps to the lesson's "type
parameters" with "generic" elided to keep the surface narrow. The
Book also explicitly defers full generics treatment to Chapter 10;
the lesson similarly defers it under *What To Ignore For Now*.

Line 87 (the prelude membership claim):

> Note that, like the `Option` enum, the `Result` enum and its
> variants have been brought into scope by the prelude, so we don't
> need to specify `Result::` before the `Ok` and `Err` variants in
> the `match` arms.

Direct corpus license for the lesson's "`Result` (along with `Ok` and
`Err`) is in the *prelude*, so no `use` line is needed" claim. The
working probe corroborates empirically: it compiles and runs without
any `use std::result::*;` line. (Lesson 044 installed `use` for items
*not* in the prelude — `min` from `std::cmp` — so the audience-level
contrast is implicit: prelude items are pre-imported, others need
`use`.)

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book guessing-game chapter. Already cited in lessons 042, 050,
051. Lines 338-348 (the *enumeration / variants* introduction tied to
`Result`):

> `read_line` ... also returns a `Result` value. `Result` is an
> *enumeration*, often called an *enum*, which is a type that can be
> in one of multiple possible states. We call each possible state a
> *variant*.
>
> ...
>
> `Result`'s variants are `Ok` and `Err`. The `Ok` variant indicates
> the operation was successful, and it contains the successfully
> generated value. The `Err` variant means the operation failed, and
> it contains information about how or why the operation failed.

Audience-level corpus statement of the success/failure split for
`Result`. The lesson's "two payload variants" + "`Ok(T)` for success
and `Err(E)` for failure" framing rephrases this. Calibration: the
Book introduces `Result` here in the context of `read_line()`, which
returns `io::Result<usize>` — a type alias deferred under *What To
Ignore For Now*. Today's lesson uses a hand-written `parity` function
returning `Result<i32, i32>` so that all involved types are already
installed.

### `output/docs/rust/reference/items/enumerations.md`

The Reference page for enum items. Already cited in lesson 051. Reused
here for the *payload-variant grammar* and the *call-expression
construction* claim. Three load-bearing spans.

Lines 15-19 (the variant grammar productions):

> EnumVariant →
>     OuterAttribute\* Visibility?
>     IDENTIFIER ( EnumVariantTuple | EnumVariantStruct )? EnumVariantDiscriminant?
>
> EnumVariantTuple → ( TupleFields? )

The grammar production licenses the `Variant(T)` form: a variant is
an identifier optionally followed by an `EnumVariantTuple`, which is
parenthesized tuple-fields. Lesson 051's `Ordering::Less` exercised
the no-suffix form; today the `Ok(T)` / `Err(E)` declaration in
`std/result/enum.Result.md` exercises the `EnumVariantTuple` form.
The audience-level prose in the lesson collapses the grammar to
"variant carries a payload, written `Variant(T)`."

Lines 52-67 (the *named or unnamed fields* example):

> Enum constructors can have either named or unnamed fields:
>
> ```rust
> enum Animal {
>     Dog(String, f64),
>     Cat { name: String, weight: f64 },
> }
>
> let mut a: Animal = Animal::Dog("Cocoa".to_string(), 37.2);
> a = Animal::Cat { name: "Spotty".to_string(), weight: 2.7 };
> }
> ```

Direct corpus precedent for the term *constructor* applied to enum
variants (lesson 051 already surfaced *constructor* in passing; today
makes it load-bearing) and for the call-form construction
`Animal::Dog("Cocoa".to_string(), 37.2)`. The lesson uses the simpler
single-payload form `Ok(n)` / `Err(n)` for `Result<i32, i32>`. The
lesson explicitly defers struct-like variants (the `Cat` form).

Lines 113-115 plus 117-137 (the call-expression instantiation rule
and the canonical example):

> [items.enum.tuple-expr]
> A tuple-like variant can be instantiated with a call expression or
> a struct expression.
>
> ...
>
> ```rust
> enum Examples {
>     UnitLike,
>     TupleLike(i32),
>     StructLike { value: i32 },
> }
>
> use Examples::*; // Creates aliases to all variants.
> let x = UnitLike;          // Path expression of the const item.
> let x = UnitLike {};       // Struct expression.
> let y = TupleLike(123);    // Call expression.
> let y = TupleLike { 0: 123 }; // Struct expression using integer field names.
> let z = StructLike { value: 123 }; // Struct expression.
> }
> ```

Direct corpus license for the lesson's central new mechanic:
`TupleLike(123)` is a *call expression*, exactly parallel to
`Ok(n)` and `Err(n)` in the working probe. The Reference's
`TupleLike(123) // Call expression` annotation is the canonical
statement that *"a tuple-like variant can be instantiated with a call
expression."* The lesson does not surface the term "tuple-like
variant" in the audience-level prose; it uses the more concrete
"variant with payload, written `Variant(T)`" framing.

### `output/docs/rust/reference/expressions/call-expr.md`

The Reference page for call expressions. Already cited in lessons 008,
041, 049. Reused here to back the claim that `Ok(n)` and `Err(n)` are
call expressions in the same grammar slot lesson 008 first installed.

Lines 9-10 (the call grammar):

> CallExpression → Expression ( CallParams? )
> CallParams → Expression ( , Expression )\* ,?

The lesson's `Ok(n)` and `Err(n)` are CallExpressions whose function
operand is the variant constructor (a path expression) and whose
single argument is `n` (lesson 020's parameter). Lesson 049 already
installed that the function-operand slot accepts any expression
producing a callable. Today extends the kind of callable: a
*tuple-like variant constructor*, per the enumerations.md grammar
above.

Calibration: the lesson does not show the connection to the call
grammar in the body — it just shows `Ok(n)` and lets the reader's
prior call-shape model from lessons 008/049 do the work. The
Reference license is captured here so red-team can verify the form
is grammatically sanctioned.

### `output/docs/rust/book/ch06-01-defining-an-enum.md`

The Book chapter that defines enums. Already cited in lesson 051.
Reused here for the audience-level statement that variants can carry
data of varying kinds. One load-bearing span.

Lines 238-254:

> ```
> enum Message {
>     Quit,
>     Move { x: i32, y: i32 },
>     Write(String),
>     ChangeColor(i32, i32, i32),
> }
> ```
>
> *Listing 6-2: A `Message` enum whose variants each store different
> amounts and types of values*
>
> This enum has four variants with different types:
>
> - `Quit`: Has no data associated with it at all
> - `Move`: Has named fields, like a struct does
> - `Write`: Includes a single `String`
> - `ChangeColor`: Includes three `i32` values

Direct audience-level corpus statement that variants can carry
*data*, with `Write(String)` exactly the `Variant(T)` shape today
generalizes to `Ok(T)` / `Err(E)`. The Book passage's "Has no data
associated with it at all" describes lesson 051's case (`Ordering`'s
unit variants); "Includes a single `String`" describes today's case.
The lesson body's "variants without payload (lesson 051) vs. variants
with payload (today)" mental-model contrast rests directly on this
Book-level distinction.

Calibration: the Book passage uses a *user-defined* `Message` enum;
the lesson uses the *standard-library* `Result`. The mechanic is
identical. Defining your own enum with payload variants is deferred.

Line 297 (constructor call form on a payload variant):

> ```
> let m = Message::Write(String::from("hello"));
> ```

Direct audience-level precedent for the call-form constructor
`Message::Write(arg)`. The lesson's `Ok(n)` / `Err(n)` is the
`Type::Variant(arg)` form with the `Type::` prefix elided thanks to
prelude membership.

### `output/docs/rust/error_codes/E0308.md`

The error-code explainer for E0308 *mismatched types*. Already cited
many times (lessons 024, 025, 026, 028, 033, 045, 046, 047, 048). The
lesson does not re-explain E0308; the broken-contrast probe simply
confirms the same E-code fires when an `Ok(7)` (a `Result<{integer},
_>`) is assigned to an `i32` slot. The new structural fact for cycle
052 is that `Ok(7)` is a `Result<T, E>` value, *not* an integer — the
constructor *wraps*, it does not return the underlying value.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/052-result-enum-and-is-ok.rs`.
Identical source to the *Try It* block.

Transcript, captured 2026-05-07 in a fresh `mktemp -d`:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
demo.rs
--- cat demo.rs ---
fn parity(n: i32) -> Result<i32, i32> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(n)
    }
}

fn main() {
    let a = parity(4);
    let b = parity(7);
    println!("a is ok: {}", a.is_ok());
    println!("b is ok: {}", b.is_ok());
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
a is ok: true
b is ok: false
exit=0
--- temp dir removed ---
```

Notes (load-bearing observations):

- `rustc demo.rs` exits 0 silently. No warnings. In particular: no
  `use std::result::Result;` or `use std::result::{Ok, Err};` is
  needed. This corroborates Book ch09-02 line 87 ("the `Result` enum
  and its variants have been brought into scope by the prelude").
- `./demo` prints exactly two lines: `a is ok: true` and
  `b is ok: false`. This is the load-bearing truth-table corroboration
  of `is_ok`'s spec from `std/result/enum.Result.md` line 43:
  "Returns `true` if the result is `Ok`."
  - `parity(4)`: `4 % 2 == 0` is `true`, so `Ok(4)` is returned, and
    `a.is_ok()` returns `true`.
  - `parity(7)`: `7 % 2 == 0` is `false`, so `Err(7)` is returned, and
    `b.is_ok()` returns `false`.
- The `if` expression body of `parity` (lesson 026) returns one of
  `Ok(n)` or `Err(n)`. Both are `Result<i32, i32>` values, so both
  arms have the same type — required by lesson 026's "arms must agree
  in type" rule. rustc accepts the function definition.
- The function signature `fn parity(n: i32) -> Result<i32, i32>` fills
  lesson 021's `-> RTYPE` slot with a new typed name. The type slot
  accepts `Result<i32, i32>` — there is no syntactic constraint that
  the return type be a *primitive* or a *single-identifier* type.
- `Ok(n)` and `Err(n)` are call expressions — function-operand
  position holds the constructor (a path-expression resolving to a
  tuple-like variant), and the parenthesized argument list holds `n`.
  Per `reference/items/enumerations.md` lines 113-115, "A tuple-like
  variant can be instantiated with a call expression."
- `a.is_ok()` and `b.is_ok()` are method-call expressions (lesson
  040). The method takes `&self` (per `std/result/enum.Result.md`
  line 41), so `a` is borrowed, not consumed. The lesson does *not*
  surface the `&self` detail; it simply notes the method returns
  `bool`. The take-no-ownership detail is irrelevant for the working
  probe (each binding is used once after `is_ok()`).
- The `{}` placeholder in `println!` formats `bool` (per
  `std/fmt/trait.Display.md`'s `Display for bool` impl, already
  exercised by lessons 012 and 013). No new format spec is introduced.

### Broken-contrast probe

Source (not committed):

```rust
fn main() {
    let n: i32 = Ok(7);
    println!("{}", n);
}
```

Captured 2026-05-07 in a fresh `mktemp -d` (filename `broken.rs`):

```text
--- cat broken.rs ---
fn main() {
    let n: i32 = Ok(7);
    println!("{}", n);
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:2:18
  |
2 |     let n: i32 = Ok(7);
  |            ---   ^^^^^ expected `i32`, found `Result<{integer}, _>`
  |            |
  |            expected due to this
  |
  = note: expected type `i32`
             found enum `Result<{integer}, _>`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Notes (probe evidence — not corpus quotation):

- The headline reads `error[E0308]: mismatched types`. Same E-code
  lessons 024, 025, 026, 028, 033, 045, 046, 047, 048 installed.
  Today's contribution is the *content* of the mismatched types,
  not the E-code itself.
- The source-excerpt label reads `expected \`i32\`, found
  \`Result<{integer}, _>\``. This is the load-bearing piece of probe
  evidence. It corroborates the central new fact: `Ok(7)` is *not*
  an `i32` — it is a `Result<T, E>` value. The constructor *wraps*
  the integer; it does not produce one.
- The annotation `i32` is underlined `---` with sub-line `expected due
  to this` (lesson 019's annotation slot is what created the
  expectation). The right-hand side `Ok(7)` is underlined `^^^^^` and
  labeled with the found type. Both labels appear in the same
  diagnostic block — same shape lessons 024-034 / 045-048 captured.
- The `note:` block reads `expected type \`i32\` / found enum
  \`Result<{integer}, _>\``. rustc explicitly names the kind of item
  on the *found* side as *enum*. This is direct probe evidence that
  rustc treats `Result<T, E>` as an enum (not as some other kind of
  type), reinforcing the lesson's "`Result<T, E>` is the prelude's
  two-variant enum" claim.
- The `{integer}` in the type label is rustc's unspecified integer
  type variable — `7` is an integer literal that has not yet been
  forced to a specific concrete type because the *only* constraint on
  the value is "it must be an `i32`," and that constraint is
  contradicted by the `Result` wrapper. The lesson body glosses this
  as "an integer" without naming integer-type inference. The `_` for
  `E` is rustc's "type parameter could not be inferred from this
  context alone" placeholder — there is no information anywhere in
  the snippet to constrain `E`. The lesson surfaces `_` only as a
  placeholder; the `T`/`E` inference machinery is deferred under
  *What To Ignore For Now*.
- Exit code: 1. No executable was produced.
- *Calibration with prior E0308 probes*: lessons 045/046 captured
  E0308 with `expected \`&i32\`, found \`i32\``; lesson 047 captured
  `expected \`&mut i32\`, found \`&i32\`` with `types differ in
  mutability`; today the labels read
  `expected \`i32\`, found \`Result<{integer}, _>\``. Same E-code,
  same diagnostic shape, three different kinds of contrast — all
  corroborating that E0308 fires whenever the type at a use site
  disagrees with what was supplied.

The broken-contrast probe is optional per the orchestrator's
prompt — the central claims are positive equivalences (constructor
build + truth-table). The probe is included because it cleanly
distinguishes "Ok(value) wraps the value" from "Ok(value) returns the
value," which is the most likely confusion for a learner just leaving
lesson 051's bare-name variants. The captured "found enum
`Result<{integer}, _>`" message is honest probe evidence backing the
"`Result<T, E>` is a distinct type, not the same as the payload type"
claim from the lesson body.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 052. Older supporting lessons are mentioned by
number only.

- **Lesson 051 (load-bearing)** — installed *enum* and *variant* as
  audience-level nouns and showed `Type::Variant` for unit variants
  (`Ordering::Less`). Lesson 052 extends the *variant* concept from
  "bare name" to "carries a payload, written `Variant(T)` in the
  declaration and called `Variant(value)` at use sites." The Book
  ch06-01 *Listing 6-2* span quoted above (lines 238-254) is the
  direct audience-level bridge between unit variants and payload
  variants. The lesson's *Mental Model Delta* spells out the contrast
  in one before/after pair. Calibration: lesson 051 also defers
  "variants with payload" by name in its *What To Ignore For Now*
  section; today picks that exact future move.
- **Lesson 040 (load-bearing)** — installed the dot-form method-call
  grammar `value.method(args)`. Lesson 052 reuses this slot for
  `a.is_ok()` and `b.is_ok()` with no arguments. The `Result::is_ok`
  signature `pub const fn is_ok(&self) -> bool` (per `std/result/
  enum.Result.md` line 41) takes a `&self` and no extra parameters,
  so the dot-form call site has empty parentheses. Lesson 052 does
  *not* surface the `&self` detail; it just exercises the method.
- **Lesson 021 (load-bearing)** — installed the function return-type
  slot `-> RTYPE`. Lesson 052 fills the slot with `Result<i32, i32>`
  — a typed name with angle-bracketed type parameters. Lesson 021's
  *unlocks* explicitly listed "Result/Option returns" as a future
  move; today picks that future move.
- **Lesson 026** — installed `if condition { a } else { b }` as an
  expression whose value is the chosen branch's value. The body of
  `parity` is exactly that shape; both branches produce a
  `Result<i32, i32>` value, satisfying lesson 026's "arms must agree
  in type" rule. The `if` expression itself comes from cycle 026.
- **Lesson 025** — installed *implicit final-expression return*: a
  function body's tail expression — no `;`, no `return` — is the
  function's return value when `-> RTYPE` is declared. Today's
  `parity` body uses this form: `if n % 2 == 0 { Ok(n) } else { Err(n) }`
  is the function body's tail expression, with no trailing semicolon
  and no `return` keyword. The `if` expression IS the function body's
  tail expression, and so its value (one of `Ok(n)` or `Err(n)`) is
  the function's return value.
- **Lessons 037 and 013** — `n % 2 == 0` uses the remainder operator
  (lesson 037) and the `==` comparison (lesson 013) producing a
  `bool`. Lesson 037's *unlocks* explicitly listed "even/odd checks
  via `n % 2 == 0`" as a future move; today's working probe is
  exactly that pattern.
- **Lesson 020** — `parity(n: i32)` uses the lesson-020 parameter
  slot. Lesson 019 — `let n: i32 = Ok(7);` in the broken-contrast
  probe uses the lesson-019 annotation slot. Both used unchanged.
- **Lessons 001, 002, 005, 008, 012** — `rustc file.rs` then
  `./name`; `fn main` is the entry point; `let name = value;`;
  free-function calls; `bool` and its `{}` formatting. All used
  unchanged.

## Older supporting lessons

- Lesson 042 (`Type::name(args)` no-receiver call form). Today's
  `Ok(n)` / `Err(n)` is the same call-shape but with the `Type::`
  prefix elided thanks to prelude membership. Calling
  `Result::Ok(n)` is also legal — see *Calibration* below — but the
  lesson uses the prelude form for surface economy.
- Lesson 044 (`use` declaration). *Not* used today: `Result`, `Ok`,
  `Err` are in the prelude (Book ch09-02 line 87), so no `use` line
  is needed. The lesson's prose explicitly notes this absence.
- Lesson 049 (method chaining). *Not* used today: the working probe
  uses `let a = parity(4); ... a.is_ok()` rather than the chained
  form `parity(4).is_ok()` — the chained form is also valid, but
  binding intermediate values keeps the variant identity visible at
  each step.
- Lesson 050 (`std::io::stdin()`). *Not* load-bearing today, but
  lesson 050's *unlocks* listed "the `Result<T, E>` enum" as a future
  move — today picks that future move. Future cycles will compose
  lesson 050's stdin handle with `Stdin::read_line(&mut buf)`
  returning `io::Result<usize>`.

## Calibration: minor surface choices not surfaced in the lesson body

- The probe writes `Ok(n)` rather than `Result::Ok(n)`. Both compile.
  The shorter form is what the prelude buys you and is the form the
  Book uses throughout chapters 2 and 9.
- `let a = parity(4);` does not annotate the type. rustc infers
  `Result<i32, i32>` from the function signature. Annotating the
  binding (`let a: Result<i32, i32> = parity(4);`) would also work
  but adds surface for no payoff — the type is determined by the
  function signature. Lesson 019's annotation slot has been exercised
  enough times that omitting it here is not a new burden.
- `parity` accepts `i32` rather than `u32`. The probe values 4 and 7
  fit either, but the function signature uses `i32` to stay in the
  type-namespace lessons 019/020/021 already installed.
