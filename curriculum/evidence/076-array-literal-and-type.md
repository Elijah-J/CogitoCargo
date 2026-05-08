# Evidence — 076-array-literal-and-type

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end.
  Only the working `.rs` is committed, at
  `experimental/eduratchet2/runs/rust-moves/observations/076-array-literal-and-type.rs`.
  The broken-contrast `.rs` and the auxiliary arity-mismatch `.rs`
  are *not* committed; the transcripts below are the artifacts.

Same host and toolchain as recent accepted lessons (072-075).

## Sources

### `output/docs/rust/book/ch03-02-data-types.md`

Ch3-2 *The Array Type* subsection. Already cited in cycles 019,
033, 062, 072, 073, 074 for sibling type sections; today's
load-bearing span is lines 247-385. Six load-bearing pieces.

Lines 247-251 (the *Compound Types* framing the lesson opens with):

> *Compound types* can group multiple values into one type. Rust
> has two primitive compound types: tuples and arrays.

Corpus warrant for the lesson's "second primitive compound type
... alongside the tuple" framing. The Book itself enumerates
exactly the two: tuple (lesson 072) and array (today). The
*Mental Model Delta* and *What Changed* both lean on this
explicit pairing.

Line 322 (the *Unlike a tuple* contrast):

> Unlike a tuple, every element of an array must have the same
> type. Unlike arrays in some other languages, arrays in Rust have
> a fixed length.

Two load-bearing fragments in one sentence:

1. *"every element of an array must have the same type"* —
   corpus statement of the *homogeneous* claim. The lesson uses
   the word *homogeneous* in *The Move* and *Mental Model Delta*;
   the Book says the same thing in plainer prose. The contrast
   probe (`[1, 2.5]`) is the operational witness.
2. *"arrays in Rust have a fixed length"* — the *fixed-length*
   claim. The lesson reuses the word "fixed-length" verbatim.

Lines 325-334 (the array-literal canonical example):

> We write the values in an array as a comma-separated list inside
> square brackets:
>
> ```rust
> fn main() {
>     let a = [1, 2, 3, 4, 5];
> }
> ```

Corpus warrant for the *array literal* form and for the lesson's
working probe line `let nums = [1, 2, 3, 4, 5];`. The probe's
literal is verbatim from the Book except for the binding name.

Lines 359-370 (the array-type canonical example):

> You write an array's type using square brackets with the type of
> each element, a semicolon, and then the number of elements in
> the array, like so:
>
> ```rust
> fn main() {
> let a: [i32; 5] = [1, 2, 3, 4, 5];
> }
> ```
>
> Here, `i32` is the type of each element. After the semicolon,
> the number `5` indicates the array contains five elements.

Corpus warrant for the lesson's *array type* description ("square
brackets, the element type, semicolon, length") in *The Move* and
the typed binding `let typed: [i32; 5] = [10, 20, 30, 40, 50];`
in the working probe. The Book's "type of each element" /
"number of elements" pair maps exactly onto the lesson's
"element type before the semicolon, length after."

Lines 372-385 (the repeat-init canonical example):

> You can also initialize an array to contain the same value for
> each element by specifying the initial value, followed by a
> semicolon, and then the length of the array in square brackets,
> as shown here:
>
> ```rust
> fn main() {
> let a = [3; 5];
> }
> ```
>
> The array named `a` will contain `5` elements that will all be
> set to the value `3` initially. This is the same as writing
> `let a = [3, 3, 3, 3, 3];` but in a more concise way.

Corpus warrant for the *repeat-init* form. The lesson uses
`[0; 4]` in the probe instead of `[3; 5]` to keep length-4
distinct from the length-5 in the other two probe lines (so the
three `.len()` outputs are visibly different: `5`, `5`, `4`); the
shape is verbatim from the Book. The lesson's quoted phrase "the
same as writing `let a = [3, 3, 3, 3, 3];` but in a more concise
way" comes from this passage; line 384 in particular is the
quoted line. The *Check Yourself* answer (c) `[7, 7, 7, 7]`
re-applies this rule.

Lines 336-338 (the deferred *stack* note):

> Arrays are useful when you want your data allocated on the
> stack, the same as the other types we have seen so far, rather
> than the heap (we will discuss the stack and the heap more in
> Chapter 4)

Cited only in *What To Ignore For Now* — today does not install
"stack vs heap" as a typed concept, and the Book itself defers to
chapter 4 in this passage.

Lines 340-344 (the deferred `Vec<T>` contrast):

> An array isn't as flexible as the vector type, though. A vector
> is a similar collection type provided by the standard library
> that *is* allowed to grow or shrink in size because its contents
> live on the heap.

Cited only in *What To Ignore For Now* for the `Vec<T>` deferral.
The Book itself contrasts here; today acknowledges it without
installing.

### `output/docs/rust/reference/types/array.md`

Lines 12-18:

> An array is a fixed-size sequence of `N` elements of type `T`.
> The array type is written as `[T; N]`.
>
> The size is a [constant expression] that evaluates to a `usize`.

Two load-bearing pieces:

1. *"fixed-size sequence of `N` elements of type `T` ... `[T; N]`"*
   — the formal definition that backs the lesson's "the array
   type is `[T; N]`" claim and the *Mental Model Delta* statement
   "`[i32; 5]` and `[i32; 6]` are different types" (different `N`
   values produce different types).
2. *"The size is a constant expression that evaluates to a
   `usize`"* — corpus warrant for *What To Ignore For Now*'s
   "constant-expression rule for `N`" deferral. The lesson does
   not unpack this rule because the probes use plain integer
   literals (`5`, `5`, `4`) which trivially satisfy it.

### `output/docs/rust/reference/expressions/array-expr.md`

Lines 18-32 (the two-form rule):

> *Array expressions* construct arrays. Array expressions come in
> two forms.
>
> The first form lists out every value in the array.
>
> The syntax for this form is a comma-separated list of expressions
> of uniform type enclosed in square brackets.
>
> This produces an array containing each of these values in the
> order they are written.

Load-bearing for the *array literal* description in *The Move*
and *Try It*. The Reference's phrase *"of uniform type"* is the
formal version of the Book's *"every element ... must have the
same type"* — both restate the homogeneous claim. The lesson
chooses the Book's plainer wording.

Lines 34-44 (the repeat-init grammar):

> The syntax for the second form is two expressions separated by a
> semicolon (`;`) enclosed in square brackets.
>
> The expression before the `;` is called the *repeat operand*.
>
> The expression after the `;` is called the *length operand*.

Load-bearing for the lesson's *repeat-init* shape: two
expressions separated by `;` inside `[ ]`. The lesson does not
introduce the formal terms *repeat operand* and *length operand*
(audience-level naming sticks to "element value" and "length");
the substantive shape is the same.

Line 92 (the multidimensional example):

> `[[1, 0, 0], [0, 1, 0], [0, 0, 1]]; // 2D array`

Cited only in *What To Ignore For Now* — multidimensional arrays
exist (`[[T; N]; M]`) but are deferred.

### `output/docs/rust/std/primitive.array.md`

Lines 7-14 (the type definition and the two literal forms):

> A fixed-size array, denoted `[T; N]`, for the element type, `T`,
> and the non-negative compile-time constant size, `N`.
>
> There are two syntactic forms for creating an array:
>
> - A list with each element, i.e., `[x, y, z]`.
> - A repeat expression `[expr; N]` where `N` is how many times to
>   repeat `expr` in the array.

Cross-corroborates the Book and Reference. Names *both* literal
forms in one place — useful as a sanity check that the lesson is
installing the canonical pair, not an idiosyncratic selection.

Lines 41-42 (the array-to-slice coercion that explains why
`.len()` is callable):

> Arrays coerce to slices ([T]), so a slice method may be called
> on an array. Indeed, this provides most of the API for working
> with arrays.

Load-bearing for *What To Ignore For Now*'s slice deferral and
for the lesson's choice to call `.len()` on arrays. `.len()` is
not impl'd directly on `[T; N]` in this corpus page; it lives on
the slice and is reachable via this coercion. The lesson body
does not lean on the word "coerce" (audience has not installed
that vocabulary); it just calls `.len()` and observes the answer.
*What To Ignore* names the coercion in one sentence as a
pointer for later.

### `output/docs/rust/std/primitive.slice.md`

Lines 794-805 (the actual `.len()` definition the lesson calls):

> `pub const fn len(&self) -> usize`
>
> Returns the number of elements in the slice.
>
> ##### Examples
>
> ```
> let a = [1, 2, 3];
> assert_eq!(a.len(), 3);
> ```

The canonical `.len()` definition. Two load-bearing pieces:

1. *Return type `usize`* — backs the lesson's passing mention
   that `.len()`'s return type is `usize`. The lesson is careful
   not to *install* `usize` as a centered typed name; it appears
   only as glossed-in-passing.
2. *The example `let a = [1, 2, 3]; assert_eq!(a.len(), 3);`* —
   the std doc itself uses an array literal as the receiver of
   `.len()`. This is the corpus warrant that the probe's
   `nums.len()`, `typed.len()`, `zeros.len()` are exactly the
   intended shape, not an unusual application of the method.

The *signature line* "1.0.0 (const: 1.39.0)" is not cited as
load-bearing (today does not install version stability or
const-fn machinery).

### Sources NOT cited as load-bearing

- `output/docs/rust/error_codes/E0308.md` — the diagnostic E-code
  for the contrast probe. Probe transcript captured here is
  load-bearing; the explainer page is not separately quoted in
  the lesson body. Same pattern as lessons 062, 072, 073, 074.
- `output/docs/rust/reference/types/slice.md` — slice as a
  separate type. Today defers slices to *What To Ignore* and only
  uses the *coercion* fact from `primitive.array.md`. The
  Reference's slice page is consistent with this but not separately
  needed for any prose claim.
- `output/docs/rust/book/ch03-02-data-types.md` lines 389-473 (the
  *Array Element Access* and *Invalid Array Element Access*
  subsections). These are queue items D and E. Today's *What To
  Ignore* names them; the Book lines themselves are not load-bearing
  for any claim today installs.

## Probes

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/076-array-literal-and-type.rs`)
is the *working* version. Two contrast probes (mixed-type literal
and arity mismatch) are documented as separate runs below, not
committed as separate `.rs` files.

### Probe 1: working program

Captured in a fresh empty temp dir created with `mktemp -d` and
removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let nums = [1, 2, 3, 4, 5];
    let typed: [i32; 5] = [10, 20, 30, 40, 50];
    let zeros = [0; 4];
    println!("nums.len() = {}", nums.len());
    println!("typed.len() = {}", typed.len());
    println!("zeros.len() = {}", zeros.len());
}
--- rustc demo.rs (capturing stderr) ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
nums.len() = 5
typed.len() = 5
zeros.len() = 4
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (no warnings, no errors),
  consistent with lesson 001.
- `./demo` prints exactly three lines, each witnessing a distinct
  claim:
  1. `nums.len() = 5` — the bare `let nums = [1, 2, 3, 4, 5];`
     form bound, *and* `.len()` returned `5`. The 5-element
     literal therefore produced a value with a length rustc and
     the runtime agree on. This is the load-bearing observation
     for the lesson's "the literal form works without an
     annotation" claim.
  2. `typed.len() = 5` — `let typed: [i32; 5] = [10, 20, 30, 40,
     50];` typechecked. The annotation `[i32; 5]` (lesson-019
     `: TYPE` slot, with the array-type expression `[i32; 5]`)
     accepted the 5-element integer-literal value. `.len()`
     returned `5` — the length declared in the type slot
     matches the runtime length.
  3. `zeros.len() = 4` — `let zeros = [0; 4];` (the repeat-init
     form) produced a 4-element array. `.len()` returned `4` —
     the Book's "the same as writing `[0, 0, 0, 0]` but in a
     more concise way" claim, observationally confirmed.
- Together the three lines witness all three syntactic shapes
  this lesson installs, plus the operational fact that an
  array's length is a real, accessible, type-tracked number.
- The committed `.rs` file's source matches the *Try It* code
  block exactly. Only the working source is committed under
  `observations/`.

### Probe 2: broken contrast — mixed-type array literal

Same temp dir family, separate file `broken.rs`:

```text
--- cat broken.rs ---
fn main() {
    let a = [1, 2.5];
    println!("{}", a.len());
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:2:17
  |
2 |     let a = [1, 2.5];
  |                 ^^^ expected integer, found floating-point number

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Read with lesson 003's diagnostic map:

- **Headline**: `error[E0308]: mismatched types`. Coded `[E0308]`.
  Same E-code as the broken-contrast probes for lessons 073 and
  074; same general "expected vs found" structure.
- **Location**: `broken.rs:2:17` — line 2, column 17, the start
  of the literal `2.5` inside the array literal.
- **Source excerpt with caret**: `^^^` underlines `2.5`. The
  caret is on the *second* element, not the first — a substantive
  observation the lesson uses verbatim ("rustc fixed the element
  type from the *first* element ... and then rejected the
  second").
- **Inline annotation**: `expected integer, found floating-point
  number`. This is the lesson's load-bearing piece. rustc names
  what it expected (an integer type — derived from the first
  element `1`, which is an integer literal defaulting to `i32`)
  and what it found (a floating-point literal). The phrasing maps
  the homogeneous-elements rule onto an audience-level diagnostic.
- **Trailer**: `For more information about this error, try
  \`rustc --explain E0308\`.` — present because the headline is
  coded (lesson 070's runnable-instruction shape).
- **Exit code**: 1; no executable produced (`ls` shows only
  `broken.rs`).

This is the load-bearing negative probe for the lesson's
contrastive claim ("with one element type the literal works;
with two element types it does not"). Without this probe, the
homogeneous-elements claim would rest only on the Book sentence
at line 322; the captured diagnostic shows rustc itself
enforcing the rule with the exact "expected ... found ..."
shape lessons 003 and 074 already taught the learner to read.

Why this probe and not the arity-mismatch alternative: the
mixed-type probe directly witnesses the *homogeneous* claim,
which is the more salient new fact today (tuples already
established fixed-arity at compile time in lesson 072). The
arity-mismatch probe would re-witness fixed-length, which is
already implicitly carried over from the tuple lesson. Probe 3
below records the arity-mismatch transcript for transparency.

### Probe 3: auxiliary — arity-mismatch (length 3 declared, 2 supplied)

Captured for evidence transparency only. **Not** referenced in
the lesson body. The diagnostic surfaces the *fixed-length*
claim, but the lesson chose the homogeneous probe (Probe 2) as
its load-bearing contrast. Documented here so a red-team
reviewer can see the alternative was considered.

```text
--- cat broken.rs ---
fn main() {
    let a: [i32; 3] = [1, 2];
    println!("{}", a.len());
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:2:23
  |
2 |     let a: [i32; 3] = [1, 2];
  |            --------   ^^^^^^ expected an array with a size of 3, found one with a size of 2
  |            |     |
  |            |     help: consider specifying the actual array length: `2`
  |            expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

Notes:

- Same E-code (`E0308`) as Probe 2, different inline annotation:
  `expected an array with a size of 3, found one with a size of
  2`. Witness for the *fixed-length* claim — rustc reports both
  sizes in plain prose.
- The `help:` block proposes "consider specifying the actual
  array length: `2`" — rustc literally suggests editing the
  declared length to match the supplied count. This is rustc
  exposing the same length-is-part-of-the-type fact from the
  other side: rather than relax the size to a range, the diagnostic
  asks for an exact rewrite.
- Caret position is on the *whole* literal `[1, 2]`, not on a
  single element — because the offending mismatch is the literal's
  arity against the type's `3`, not any single value.
- Documented here only for transparency; the lesson body uses
  Probe 2.

### Negative / contrast probes

Probe 2 is the load-bearing negative probe for the lesson's
contrastive claim. Probe 3 is auxiliary and not load-bearing.

### Reproducibility note

Probe 1 is deterministic on rustc 1.95.0 — the program has no
randomness or environment dependency.

Probe 2's headline (`error[E0308]: mismatched types`), inline
annotation (`expected integer, found floating-point number`),
and trailer (`For more information ... rustc --explain E0308`)
are deterministic on this rustc release. The exact wording is
rustc-version-specific; the *shape* — coded E0308 with an
"expected X, found Y" pair — is grounded in lesson 003's general
diagnostic map and is stable.

Probe 3's headline and `help:` block are also rustc-version-
specific in wording but stable in shape.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 076. Older supporting lessons are
mentioned below by number only.

- **Lesson 003 (load-bearing for the diagnostic map)** —
  installs the four-part read of headline + `-->` + source
  excerpt with caret + optional `help:` / `note:` lines. Probe 2
  is read with that map only; no new diagnostic vocabulary is
  installed today.
- **Lesson 005 (load-bearing for `let name = value;`)** —
  installs the binding form. Today reuses it three times: an
  array literal, an annotated array literal, and a repeat-init.
  No extension of `let`.
- **Lesson 019 (load-bearing for the `: TYPE` slot)** — installs
  `let name: TYPE = value;` as a *type annotation*. Today plugs
  the parameterized type expression `[i32; 5]` into the `TYPE`
  slot. The slot itself is unchanged; the new piece is that the
  *type expression* itself can be a parameterized form (square
  brackets, element type, semicolon, length) rather than a bare
  type name. Lesson 019's *What To Ignore* explicitly named
  "char, strings, tuples, *arrays*, structs, enums, references"
  as deferred future types; today closes the *arrays* item.
- **Lesson 040 (load-bearing for `value.method()`)** — installs
  the method-call form. Today calls `.len()` on each of the three
  array bindings. The dot-method-name-parens shape is unchanged.
- **Lesson 072 (load-bearing for the heterogeneous-vs-homogeneous
  contrast)** — installed *tuple types* with parens-and-commas
  and the heterogeneous-elements rule. Today is the homogeneous
  sibling: square-brackets-and-commas, all elements one type. The
  Book's line 322 contrast is the corpus warrant; lesson 072 is
  the prior-lesson side of that contrast.

## Older supporting lessons

Mentioned by id only, not load-bearing for any individual claim
today:

- `001-rustc-compile-and-run` — `rustc file.rs` then `./name`;
  rustc silent on success. Used as the compile-and-run shape for
  all probes.
- `002-fn-main-entry-point` — body of `fn main` runs when the
  executable launches.
- `004-statements-in-order` — the body of `fn main` is a sequence
  of `;`-terminated statements that run top to bottom.
- `011-println-positional-args` — `println!("{}", expr)`. Reused
  as-is; today does not extend `println!`. The probe's three
  `println!` lines all use positional `{}` substitution.
- `029-unit-type` — installed `()` as the *unit type*, the 0-arity
  tuple, and primed the audience for the compound-types family
  the Book frames at lines 247-251. Today is the array half of
  that family, alongside lesson 072's tuple half.
- `033-f64-floats` — installed `f64`. Cited because the contrast
  probe's `2.5` is a floating-point literal that defaults to `f64`
  (lesson 033's claim) and the diagnostic names "floating-point
  number".
- `068-let-binding-scope`, `069-rustc-warnings`, `070-rustc-explain`,
  `071-macro-invocation-syntax`, `073-let-tuple-destructure`,
  `074-char-type`, `075-const-declaration` — recent lessons on
  the same host and toolchain. Mentioned only to confirm the host
  environment is unchanged.

No trait-related lesson is cited. The brief explicitly excluded
trait machinery; the only trait-shaped fact in the picture is
"arrays coerce to slices, so the slice's `.len()` is callable on
arrays," and that fact is left at the operational level (call the
method, observe the answer) without naming the `Index` /
`Deref` / coercion machinery behind it.

## Book Ch1-3 closure-pass effect

This lesson **closes item C** in
`experimental/eduratchet2/runs/rust-moves/book-ch1-3-coverage.md`.
Item C's listed prereqs were 005, 019, 029, 072 — all installed
before this cycle. With the array literal, type, and repeat-init
now installed, the Ch3-2 *Compound Types* pair (tuples in 029 /
072 / 073, arrays in 076) is fully covered at the level Ch1-3
expects. Items D (array element access), E (out-of-bounds panic),
F (for-over-array iteration) become directly approachable — D is
the natural next move now that arrays exist.
