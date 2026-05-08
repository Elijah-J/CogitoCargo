# Evidence — 119-option-some-none

Audit appendix for `lessons/119-option-some-none.md`. Holds the
corpus-quote map, the toolchain string, the working- and contrast-
probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -srm` → `Darwin 24.5.0 x86_64`
- Probes captured 2026-05-08 in `/tmp/eduratchet119/`. Only the
  working `.rs` is committed (under
  `observations/119-option-some-none.rs`); the contrast probe's
  source and transcript are below.

## Sources

### `output/docs/rust/std/option/enum.Option.md`

The std-library page for `Option`. Primary source for the type's
declaration. Three load-bearing spans.

Lines 5-9 (the canonical declaration):

> ```
> pub enum Option<T> {
>     None,
>     Some(T),
> }
> ```

Direct corpus statement that (a) `Option` is declared with the `enum`
keyword, (b) it has exactly two variants `None` and `Some`, (c) `None`
is a unit variant (no payload) and `Some(T)` is a tuple variant carrying
a `T`, and (d) `Option` is generic over one type parameter `T`. The
lesson's centered "Type" claim rests directly on this declaration.

Calibration: the std page lists the variants in the order
`None, Some(T)`. The Book ch06-01:359-363 lists them in the same order
(see below). Some Rust prose elsewhere lists them as `Some(T), None`;
the order is irrelevant to the semantics.

Lines 17-29 (the per-variant docs):

> ## Variants
>
> ### None
> No value.
>
> ### Some(T)
> Some value of type `T`.

Per-variant gloss. The lesson's "`None` is a unit variant ... `Some(T)`
... carrying one value of type `T`" rephrases this. Calibration: the
audience-level "no value" / "some value of type T" wording is what
the Book's ch06-01:397-401 narrative prose unfolds further.

Lines 31-39 (the impl block opening, named only — not centered today):

> ## Implementations
>
> ### impl<T> Option<T>

Cited only to ground the future-move framing in *What To Ignore For
Now* (the impl-block surface holds `is_some`, `is_none`, `unwrap`,
etc., named at lines 39, 78, etc.). Today does not exercise any of
this surface.

Lines 39-41 and 78-80 (the `is_some` and `is_none` signatures):

> #### pub const fn is_some(&self) -> bool
> Returns `true` if the option is a `Some` value.
> ...
> #### pub const fn is_none(&self) -> bool
> Returns `true` if the option is a `None` value.

Cited only in *What To Ignore For Now* — these are the sibling methods
of Result's `.is_ok()` / `.is_err()` from lesson 052, named-deferred
here. The lesson does *not* exercise them; the working probe uses
`match` instead, mirroring lesson 058's match-based opening of a
payload variant.

### `output/docs/rust/book/ch06-01-defining-an-enum.md`

The Book's *Defining an Enum* chapter, *The `Option` Enum* section.
Audience-level introduction. Four load-bearing spans.

Line 314 (the introduction):

> This section explores a case study of `Option`, which is another
> enum defined by the standard library.

Direct corpus warrant for "`Option<T>` is a generic enum from the
standard library." The Book frames `Option` as a sibling of the
chapter's earlier `Message` enum and of the standard library's
`Result`.

Lines 353-364 (the declaration as the Book presents it):

> This enum is `Option<T>`, and it is defined by the standard library
> as follows:
>
> ```rust
> #![allow(unused)]
> fn main() {
> enum Option<T> {
>     None,
>     Some(T),
> }
> }
> ```

The Book's audience-level rendering of the declaration matches the
std page's at `enum.Option.md:5-9` modulo the `pub` visibility
modifier (the Book elides it; the std page has it). The lesson's
centered claim rests on both spans together.

Lines 366-370 (prelude membership — load-bearing for the "no `use`"
claim):

> The `Option<T>` enum is so useful that it's even included in the
> prelude; you don't need to bring it into scope explicitly. Its
> variants are also included in the prelude: You can use `Some` and
> `None` directly without the `Option::` prefix. The `Option<T>` enum
> is still just a regular enum, and `Some(T)` and `None` are still
> variants of type `Option<T>`.

Direct audience-level license for "no `use` line is needed" in the
lesson body. Mirrors the lesson-052 prelude claim for `Result`. The
working probe corroborates empirically — it compiles without any
`use std::option::*;` line.

Lines 372-378 (the `<T>` gloss — generic type parameter, deferred
treatment):

> The `<T>` syntax is a feature of Rust we haven't talked about yet.
> It's a generic type parameter, and we'll cover generics in more
> detail in Chapter 10. For now, all you need to know is that `<T>`
> means that the `Some` variant of the `Option` enum can hold one
> piece of data of any type ...

Audience-level license for the lesson's "generic enum from the
standard library" framing without surfacing the full generics
machinery. The lesson 052 evidence already used this exact rhetorical
move for `Result<T, E>`; today reuses it for `Option<T>`.

Lines 380-395 (the type-annotation requirement on bare `None` —
load-bearing for the contrast probe):

> ```rust
> fn main() {
>     let some_number = Some(5);
>     let some_char = Some('e');
>
>     let absent_number: Option<i32> = None;
> }
> ```
>
> The type of `some_number` is `Option<i32>`. The type of `some_char`
> is `Option<char>`, which is a different type. Rust can infer these
> types because we've specified a value inside the `Some` variant.
> For `absent_number`, Rust requires us to annotate the overall
> `Option` type: The compiler can't infer the type that the
> corresponding `Some` variant will hold by looking only at a `None`
> value. Here, we tell Rust that we mean for `absent_number` to be of
> type `Option<i32>`.

Direct corpus license for the lesson's centered contrastive claim.
The Book explicitly states (a) `Some(literal)` is inferable
(`Some(5)` → `Option<i32>`, `Some('e')` → `Option<char>`), and (b)
bare `None` is *not* — "Rust requires us to annotate the overall
`Option` type." The contrast probe witnesses (b) empirically as
E0282; the corroborating probe (`some_no_anno.rs` in
`observations/119-option-some-none.transcript.txt`) witnesses (a)
empirically.

Lines 399-405 (the "different types" framing — cited):

> When we have a `Some` value, we know that a value is present, and
> the value is held within the `Some`. When we have a `None` value,
> in some sense it means the same thing as null: We don't have a
> valid value. So, why is having `Option<T>` any better than having
> null?
>
> In short, because `Option<T>` and `T` (where `T` can be any type)
> are different types, the compiler won't let us use an `Option<T>`
> value as if it were definitely a valid value.

Cited only — names the deferred topic in *What To Ignore For Now*
("the relationship between `Option<T>` and pointers / null"). The
lesson does not center the null comparison; it stays inside the
two-coupled-claim discipline of lesson 052's evidence appendix.

### `output/docs/rust/error_codes/E0282.md`

Cited only — the formal explainer for E0282 "type annotations
needed." The contrast probe's E-code is E0282; the lesson reads it
with the lesson-003 four-part diagnostic map. The corpus page
documents the rule that the compiler needs enough information from
the program text to determine the concrete type for every type
variable. Today's `let absent = None;` is exactly such a case: the
right-hand side `None` carries no payload, so `T` in `Option<T>` has
no constraint, and rustc cannot proceed.

### `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`

Cited only — the rmp use site that motivates lesson 119. Lines 12-15:

> ```rust
> impl PartialOrd for BigUInt {
>     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
>         Some(self.cmp(other))
>     }
> }
> ```

The return type `Option<std::cmp::Ordering>` and the call expression
`Some(self.cmp(other))` are exactly the two surfaces today's lesson
installs — the type, and the `Some(value)` constructor. Today does
*not* read this slice end-to-end; the trait `PartialOrd`, the
specific use of `Option<Ordering>`, and the `partial_cmp` method body
all compose later. Today only installs `Option<T>` itself, mirroring
how lesson 052 installed `Result<T, E>` itself before lesson 058
opened a `Result` payload variant.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/119-option-some-none.rs`.
Identical source to the *Try It* block.

Transcript, captured 2026-05-08 in `/tmp/eduratchet119/`:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
$ cat demo.rs
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
$ rustc demo.rs
exit=0
$ ls
demo  demo.rs
$ ./demo
present -> 42
absent  -> -1
exit=0
```

Witnesses (load-bearing observations):

- **Compile silent on success.** No warnings. In particular, no `use
  std::option::Option;` or `use std::option::{Some, None};` line is
  needed. Corroborates Book ch06-01:366-370 ("the `Option<T>` enum is
  so useful that it's even included in the prelude").
- **`Some(42)` and `None` both type-check at `Option<i32>`.** The
  `: Option<i32>` annotations on both bindings (lesson 019's slot,
  lesson 052's pattern) tell rustc what `T` is. `Some(42)` would
  also have inferred (see corroborating probe below); the annotation
  is redundant on the `Some` line but required on the `None` line,
  so both lines carry it for parallelism (matching the Book at
  ch06-01:380-388 where `let some_number = Some(5);` omits the
  annotation but `let absent_number: Option<i32> = None;` requires
  it).
- **`Some(n) => n` arm extracts the payload.** Same lesson-058 shape:
  `Some(value-pattern) => arm` binds the payload to `n`, arm value
  is `n`. For `present = Some(42)`, the arm fires and `p_label = 42`.
- **`None => -1` arm matches the bare unit variant.** Same lesson-098
  shape: a unit-variant pattern with no parentheses. For
  `absent = None`, the arm fires and `a_label = -1`.
- **Exhaustiveness is satisfied.** `Option` has only two variants;
  both arms appear; rustc accepts the match. The exhaustiveness rule
  from lesson 030 is unchanged.
- **All arms share a type.** Both arm bodies are `i32` (`n` from a
  pinned `Option<i32>`, and the literal `-1`). The whole match is
  `i32`; `let p_label = match ...;` binds an `i32`. The lesson-030
  arm-type-uniformity rule is unchanged.
- **`println!` formats `i32` via `{}`.** Already exercised many times
  since lesson 011; no new format spec.

### Contrast probe — E0282 (centered)

Source (not committed — transcript is the artifact):

```rust
fn main() {
    let absent = None;
}
```

Transcript:

```text
$ cat broken.rs
fn main() {
    let absent = None;
}
$ rustc broken.rs
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

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0282`.
exit=1
```

Witnesses (probe evidence — not corpus quotation):

- **Headline E-code is E0282** — "type annotations needed for
  `Option<_>`". This is the exact E-code the lesson body cites; the
  diagnostic shape matches the lesson-003 four-part map (headline
  with E-code, `-->` location, source excerpt with caret, `help:`
  block).
- **The `Option<_>` placeholder** — rustc has resolved the *enum* to
  `Option` (so it knows the variant `None` came from `Option`, not
  from some user-defined enum), but the type parameter `T` is
  represented as `_`, the rustc placeholder for "couldn't infer."
  Direct probe evidence that the constructor `None` pins the variant
  but not `T`, exactly the lesson's centered claim.
- **The caret under `absent`** — points at the *binding name*, not
  the right-hand side. The under-line `---- type must be known at
  this point` annotates the literal `None`. Two pieces, one
  diagnostic block: the binding needs a type, and the right-hand
  side fails to provide one.
- **The help line** suggests `let absent: Option<T> = None;` with the
  `+++++++++++` underlines highlighting the inserted annotation slot
  — exactly the slot lesson 019 installed. The audience can apply
  the fix mechanically.
- **Build does not produce an executable** — `ls` after the failed
  compile shows only `broken.rs`, no `broken` binary. Same shape as
  every prior contrast probe in this run (e.g. lesson 098's E0599
  contrast).
- **The Book ch06-01:389-395 prediction holds.** "The compiler can't
  infer the type that the corresponding `Some` variant will hold by
  looking only at a `None` value" → empirical E0282 with `Option<_>`
  placeholder. The probe is the operational form of the Book's
  claim.

### Corroborating probe — `Some(literal)` infers without annotation

Source (not committed):

```rust
fn main() {
    let present = Some(42);
    let n = match present {
        Some(v) => v,
        None => -1,
    };
    println!("n = {}", n);
}
```

Transcript:

```text
$ rustc some_no_anno.rs
exit=0
$ ./some_no_anno
n = 42
exit=0
```

Witnesses:

- **`let present = Some(42);` compiles without an annotation.**
  Confirms Book ch06-01:387-389 — "Rust can infer these types because
  we've specified a value inside the `Some` variant." `42` is `i32`
  by default (lesson 019's integer-literal default), so `present` is
  inferred to be `Option<i32>`.
- **The asymmetry is what the lesson's centered contrast captures.**
  `Some(value)` pins `T` through the payload's type; `None` does not,
  because the variant carries nothing. This is the load-bearing
  observation behind both the working probe (which annotates both
  bindings for parallelism) and the contrast probe (which strips
  the annotation off `None` and gets E0282).
- The `Some(v) => v` arm uses a different binding name (`v`) than
  the working probe (`n`) just to keep the corroborating probe
  source distinct from the working probe; the mechanic is the same.

## Direct prerequisite claims

Summarizing only the *specific claim* each direct prerequisite
contributes to lesson 119. Older supporting lessons mentioned by
number only.

- **Lesson 052 (load-bearing)** — installed `Result<T, E>` as a
  generic enum with payload variant declared `Variant(T)` and
  constructed by call expression `Variant(value)`. Today extends the
  *generic-enum-with-payload-variant* shape from `Result<T, E>` to
  `Option<T>`. The shape is identical: `pub enum Option<T> { None,
  Some(T) }` is structurally `pub enum Result<T, E> { Ok(T), Err(E) }`
  with one type parameter (`T` instead of `T, E`) and one variant
  changed from a payload variant to a unit variant (`None` instead
  of `Err(E)`). The call-expression construction `Some(42)` is
  exactly lesson 052's `Ok(5)` shape. The prelude-membership claim
  rides 052's prelude framing for `Result`.

- **Lesson 058 (load-bearing)** — installed the `match` arm pattern
  `Variant(name) => arm` with payload binding. Today reuses the shape
  unchanged for `Some(n) => n`. Lesson 058 also explicitly named
  "the `Option<T>` enum (`Some(T)` / `None`). Same payload-variant
  shape; deferred." in its *What To Ignore For Now* — today picks up
  that exact deferred move.

- **Lesson 098 (load-bearing)** — installed the unit-variant pattern
  `Variant => arm` (no parentheses, nothing to bind). Today reuses
  the shape unchanged for `None => -1`. Lesson 098 also explicitly
  named "Generic enums like the shape of `Option<T>`" in its *What
  To Ignore* — today is one half of that deferral, on the
  *consumption* side (using std's existing `Option<T>`); the
  *authoring* side (declaring `enum My<T> { ... }`) remains deferred
  on the generics-installation arc.

- **Lesson 019 (load-bearing)** — installed `let name: TYPE = value;`
  where TYPE is a type name. Today fills the TYPE slot with
  `Option<i32>` — a typed name with one angle-bracketed type
  argument. Lesson 052 already exercised the multi-argument form
  `Result<i32, i32>`; today's one-argument form is a strict subset.
  The contrast probe's `help:` line `let absent: Option<T> = None;`
  is a direct echo of this slot.

- **Lessons 030 and 031 (cited)** — `match` exhaustiveness (E0004)
  and the rule that all arms share a type. Both unchanged today.
  Today does not exercise wildcards (031's `_`) or the diagnostic
  itself (E0004) — it ships a working two-arm match where both arms
  appear by name.

- **Lesson 003 (cited)** — rustc diagnostic four-part map. Used to
  read the contrast probe's E0282 transcript. Today's diagnostic
  matches the map (headline `error[E0282]: ...`, location
  `--> broken.rs:2:9`, source excerpt with caret under `absent` and
  underline `---- type must be known at this point` under `None`,
  `help:` block with the annotation suggestion).

- **Lessons 001, 002, 005, 011 (cited)** — `rustc file.rs` then
  `./name`; `fn main`; `let`; `println!` with `{}`. Unchanged.

## Older supporting lessons

- Lesson 051 (`Ordering` — the std-enum-with-match precedent). Cited
  only — `Option` is now a third std enum the audience matches
  against, after `Ordering` (051) and `Result` (058). The pattern is
  the same; today just adds one more concrete std enum to the
  catalogue.

- Lesson 008 (`fn`), lesson 040 (method-call dot syntax), lesson 044
  (`use` declaration). Mentioned only in *What To Ignore* — today
  defers `Option`'s method surface (`is_some`, `unwrap`, `map`,
  etc.) and does not introduce a `use` line because both `Some` and
  `None` are in the prelude.

- Lesson 099 (tuple-variant declaration on a user-defined enum).
  Cited only — today's `Some(T)` is a *consumed* tuple variant
  (declared by std), parallel to 099's authored tuple variant. The
  lesson does *not* exercise authoring of generic tuple variants;
  that composes today's mechanic with the deferred generics
  installation.

## Calibration: minor surface choices not surfaced in the lesson body

- The probe annotates *both* bindings as `: Option<i32>` rather than
  only `absent`. `let present: Option<i32> = Some(42);` is technically
  redundant — `Some(42)` would infer to `Option<i32>` on its own per
  Book ch06-01:387-389 — but writing both in parallel keeps the type
  visible at every binding site for the audience and matches the
  Book's narrative pattern (the Book at ch06-01:380-386 omits the
  annotation on `Some(5)`/`Some('e')` and adds it on `None`; the
  lesson chooses the more uniform shape because the audience has
  not yet seen 1300 lessons of inference shortcuts).

- The probe uses `i32` rather than `u32` or `i64`. `42` and `-1` fit
  any signed integer type, but `i32` keeps the type-namespace lessons
  019/020/021 already installed. The `-1` payload-free arm value is
  the simplest "obvious sentinel" choice that distinguishes from the
  payload values 42; the lesson does not center sentinel-value
  semantics (the `match` could equally return any `i32`).

- The probe builds two `match`es, one per scrutinee, rather than one
  `match` over a sequence. This mirrors lesson 058's two-`match`
  shape (one for `"42"` and one for `"abc"`) and lesson 098's
  two-`match` shape (one for `Sign::Positive` and one for
  `Sign::Negative`), so each variant is visibly opened in its own
  match.

- The corroborating probe uses `v` rather than `n` as the binding
  name to keep its source visually distinct from the working probe's;
  the binding name is irrelevant to the mechanic.

## Contrast-probe coverage

The lesson's centered contrastive claim is "a bare `None` cannot pin
`T` on its own; an annotation is required." This is witnessed
empirically by the contrast probe (E0282 with `Option<_>` placeholder
and a `help:` line proposing `let absent: Option<T> = None;`).

The corroborating probe witnesses the *positive* half of the
asymmetry: `let present = Some(42);` *does* compile, because
`Some(42)`'s payload pins `T`. Together the two probes demonstrate
what the Book at ch06-01:387-395 asserts in prose.

No additional contrast is needed for the *type* claim — `Option<T>`'s
declaration is corpus-quoted at `enum.Option.md:5-9`, and the
working probe's silent compile witnesses that the type is in scope
without a `use` line. No additional contrast is needed for the
*match* claim — the working probe's two match arms are exactly the
shapes lessons 058 and 098 already proved.

## Notes on deferred items

The lesson defers (and this appendix does not probe further):

- *`Option`'s method surface* — `is_some`, `is_none`, `unwrap`,
  `expect`, `unwrap_or`, `map`, `and_then`, `ok_or`, `?`. Each is its
  own future move. The std page at `enum.Option.md:39-...` documents
  them in detail; today reads the declaration only.
- *`if let Some(x) = opt { ... }`* — the single-arm shorthand;
  Reference `expressions/if-expr.md` covers this; deferred.
- *`Option<T>` as a function parameter or return type* — composes
  today's mechanic with the lesson-021 `-> RTYPE` slot used by
  lesson 052 for `Result<i32, i32>`. The rmp `cmp.rs:13` use site is
  exactly this composition (`fn partial_cmp(...) -> Option<...>`)
  and is the natural follow-on.
- *Non-primitive payloads* `Option<&T>`, `Option<Box<T>>`,
  `Option<Vec<T>>`, etc. — each composes today's mechanic with a
  separate type machinery (references, `Box`, `Vec`, etc.).
- *Authoring your own generic enum* `enum My<T> { ... }` — blocked
  on the generics-installation arc that lesson 052 first named and
  lesson 098 reaffirmed. Today is on the *consumption* side.
- *`Option<std::cmp::Ordering>`* — the rmp use site. Named as the
  unlock target in `graph.md`; not centered today.
- *The `Option<T>` vs null framing* (Book ch06-01:325-365). Audience-
  level motivation deferred; the mechanic is what today centers.
- *E0282 in general* — today's contrast is one specific witness of
  E0282 ("type annotations needed for `Option<_>`"). The full E0282
  catalogue covers many other unannotated-type-variable cases
  (closures, integer-type defaults blocked by trait calls, etc.) and
  is its own arc.

None of these are load-bearing for today's centered claim
"`Option<T>` is the prelude's two-variant generic enum with `Some(T)`
and `None` constructors and a payload/unit `match` shape."
