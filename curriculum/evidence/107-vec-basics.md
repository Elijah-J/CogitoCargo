# Evidence — 107-vec-basics

This appendix grounds lesson 107's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` -> `Darwin x86_64`
- Probes run from a `mktemp -d` directory on this host. Same
  toolchain as recent accepted lessons (e.g. 106).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/107-vec-basics.rs`
is the working probe verbatim.

## Sources

### `output/docs/rust/std/macro.vec.md`

The std doc page for the `vec!` macro.

#### Lines 7-11 — the macro's three arms

> ```
> macro_rules! vec {
>     () => { ... };
>     ($elem:expr; $n:expr) => { ... };
>     ($($x:expr),+ $(,)?) => { ... };
> }
> ```

Three forms admitted: `vec![]` (the `()` arm — today's empty case),
`vec![elem; n]` (the repeating arm — explicitly deferred), and
`vec![v1, v2, ...]` (the comma-separated arm — today's prefilled
case). Direct corpus warrant for the two forms today's probe uses.

#### Lines 16-19 — what the macro produces

> Creates a [`Vec`](vec/struct.Vec.md "struct std::vec::Vec")
> containing the arguments.
>
> `vec!` allows `Vec`s to be defined with the same syntax as array
> expressions.

Corpus warrant for "the macro produces a `Vec<T>`."

#### Lines 21-28 — the prefilled form

> - Create a `Vec` containing a given list of elements:
>
> ```
> let v = vec![1, 2, 3];
> assert_eq!(v[0], 1);
> assert_eq!(v[1], 2);
> assert_eq!(v[2], 3);
> ```

Corpus warrant for `vec![1, 2, 3]` shape and for the indexing form
`v[0]` / `v[1]` / `v[2]` paired in the same example. The lesson's
`vec![10, 20, 30]` and `three[0]` / `three[2]` are exactly this
shape.

### `output/docs/rust/std/vec/struct.Vec.md`

The std `Vec` struct page.

#### Lines 84-92 — indexing

> ## Indexing
>
> The `Vec` type allows access to values by index, because it
> implements the [`Index`](../ops/trait.Index.md "trait
> std::ops::Index") trait. An example will be more explicit:
>
> ```
> let v = vec![0, 2, 4, 6];
> println!("{}", v[1]); // it will display '2'
> ```

Corpus warrant for `v[i]` shape on a `Vec<T>` and for the lesson's
"std `vec/struct.Vec` page leads with..." sentence. The example is
literally the construction-then-index shape today's working probe
uses.

#### Lines 94-105 — out-of-bounds is a panic

> However be careful: if you try to access an index which isn't in
> the `Vec`, your software will panic! You cannot do this:
>
> ```
> let v = vec![0, 2, 4, 6];
> println!("{}", v[6]); // it will panic!
> ```
>
> Use [`get`](...) and [`get_mut`](...) if you want to check whether
> the index is in the `Vec`.

Corpus warrant for "out-of-bounds `v[i]` panics" — today's contrast
probe. Also corpus warrant for the `v.get(i)` deferral named in *What
To Ignore For Now*.

#### Lines 2114-2126 — the `len` method

> 1.0.0 (const: 1.87.0) ·
>
> #### pub const fn [len](#method.len)(&self) -> [usize](...)
>
> Returns the number of elements in the vector, also referred to as
> its 'length'.
>
> ##### Examples
>
> ```
> let a = vec![1, 2, 3];
> assert_eq!(a.len(), 3);
> ```

Corpus warrant for `pub const fn len(&self) -> usize`, the
"Returns the number of elements in the vector" gloss, and the
example's `a.len()` shape — exactly today's `three.len()` form with
`three.len()` returning `3`.

### `output/docs/rust/book/ch08-01-vectors.md`

Friendly version of the same operations.

#### Lines 32-44 — the `vec!` macro for prefilled construction

> Rust conveniently provides the `vec!` macro, which will create a
> new vector that holds the values you give it. Listing 8-2 creates
> a new `Vec<i32>` that holds the values `1`, `2`, and `3`. The
> integer type is `i32` because that's the default integer type, as
> we discussed in the "Data Types" section of Chapter 3.
>
> ```rust
> fn main() {
>     let v = vec![1, 2, 3];
> }
> ```

Corpus warrant for "default `i32` element type" claim in the lesson
("would default to `Vec<i32>` (lesson 080)") and for the `vec!`
macro intro.

#### Lines 84-99 — indexing is one of two access modes

> Listing 8-4 shows both methods of accessing a value in a vector,
> with indexing syntax and the `get` method.
>
> ```rust
> fn main() {
>     let v = vec![1, 2, 3, 4, 5];
>
>     let third: &i32 = &v[2];
>     ...
> }
> ```

The Book uses `&v[2]` (the *reference* form, lesson 045 territory);
today's lesson uses bare `v[2]` because the `Vec<u64>` element type
is `Copy` and the lesson does not yet have references on the
indexing-receiver. The Book's later text covers both:

#### Lines 101-103 — zero-based

> Note a few details here. We use the index value of `2` to get the
> third element because vectors are indexed by number, starting at
> zero. Using `&` and `[]` gives us a reference to the element at
> the index value.

Corpus warrant for "indices count from `0`" — same rule as lesson
077's array indexing.

#### Lines 124-127 — out-of-bounds panics

> When we run this code, the first `[]` method will cause the
> program to panic because it references a nonexistent element.

Second corpus source for the panic claim (the std page is the
first).

## Direct prerequisite summaries

Each direct prerequisite's specific claim today reuses, in 1-3
bullets per Audit Trail Depth.

### Lesson 093 (load-bearing — `Vec<T>` is a prelude name)

- Lesson 093 named `String`, `Vec`, `Result`/`Ok`/`Err`,
  `Option`/`Some`/`None` as prelude members of
  `std::prelude::rust_2024`, written bare with no `use` line.
- Today's probe writes `let empty: Vec<u64> = vec![];` and `let
  three: Vec<u64> = vec![10, 20, 30];` — both use the bare `Vec`
  name, relying on the prelude membership 093 installed.
- The contrast (writing `std::vec::Vec` instead) is not centered;
  092's E0433 contrast probe used `HashMap` to witness "non-prelude
  needs `use` or full path." No new claim about prelude membership
  is made today.

### Lesson 077 (load-bearing — `a[i]` with `i: usize`)

- Lesson 077 installed the indexing form `a[i]` for arrays, the
  `usize` requirement on the index, and the E0277 diagnostic ("the
  type `[{integer}]` cannot be indexed by `i32`") that fires when
  the named index has a non-`usize` type.
- Today reuses *exactly* that shape on a `Vec<T>` operand: same
  brackets, same `usize` requirement, same E0277 (with `[u64]`
  rather than `[{integer}]` in the headline). Probe transcript
  below witnesses the carry-through.
- Today does *not* re-install the `usize` rule; it only reuses it.
  The lesson body cites lesson 077 in *What Changed* and prerequisites.

### Lesson 071 (load-bearing — macro invocation `name!(...)`)

- Lesson 071 installed the syntactic distinction between `name(...)`
  (function call) and `name!(...)` (macro invocation) and named the
  `!` as the syntactic mark. Lesson 071's *What To Ignore* listed
  "the bracketing alternatives `name![...]` and `name!{...}`" as
  deferred future moves.
- Today uses the bracket-flavored form `vec![...]`. The std `macro.vec`
  page above shows the bracket form is the natural shape for `vec!`
  ("`vec!` allows `Vec`s to be defined with the same syntax as array
  expressions" — array expressions use `[...]`).
- The bracket vs paren equivalence is documented in the Reference
  `macros.md` lines 26-29 verbatim: `[DelimTokenTree] -> ( ... ) | [
  ... ] | { ... }` — the macro grammar admits all three delimiter
  shapes. Today's working probe uses the `[ ... ]` arm; lesson 071's
  contrast probes used the `( ... )` arm. Operational claim: both
  shapes invoke the same macro.

### Lesson 040 (load-bearing — `value.method()`)

- Lesson 040 installed the dot-after-receiver method-call form. Today
  applies it to `.len()` on a `Vec<T>` receiver. Same shape, no new
  syntactic rule.
- Lesson 076 already installed `.len()` on arrays, so the *method
  name* is also installed. Today's only new piece is "the same name
  works on `Vec<T>` and returns `usize`" — witnessed by the std
  signature `pub const fn len(&self) -> usize` quoted above.

### Lesson 019 (load-bearing — `: TYPE` annotation slot)

- Lesson 019 installed `let name: TYPE = value;` with `i32` as the
  worked TYPE. Today plugs `Vec<u64>` into the same slot. The angle-
  bracket type-parameter syntax is not centered today (lesson 093
  already used `Vec<i32>` and `Result<i32, String>` annotations
  unchallenged); today's lesson lets the type annotation pin
  inference for the empty-vec case but does not install the generic
  syntax as a centered move.

### Lesson 078 (cited — out-of-bounds panic message shape)

- Lesson 078 installed `index out of bounds: the len is N but the
  index is M` as the panic-message shape for arrays at runtime. Today
  reuses that shape on `Vec<T>`. Probe 2 transcript below witnesses
  identical structure (with different `N` and `M`).
- Lesson 078's claim about *bounds-check timing* (compile-time for
  constant-evaluable indexes on arrays via the `unconditional_panic`
  lint, vs. runtime for parse-built indexes) is the centered contrast
  point today: arrays admit the compile-time rejection because length
  lives in the type; `Vec<T>` does not, so even constant indexes
  reach the runtime check. Probe 2 (constant index `5` past the end
  of a `vec![10, 20, 30]`) witnesses this — compile is silent, panic
  fires only at runtime.

## Probes

### Probe 1 — working: `vec![]`, `.len()`, `v[i]`

Source (committed at `observations/107-vec-basics.rs`):

```rust
fn main() {
    let empty: Vec<u64> = vec![];
    let three: Vec<u64> = vec![10, 20, 30];
    println!("empty.len() = {}", empty.len());
    println!("three.len() = {}", three.len());
    println!("three[0] = {}", three[0]);
    println!("three[2] = {}", three[2]);
}
```

Transcript:

```text
$ rustc demo.rs
$ ./demo
empty.len() = 0
three.len() = 3
three[0] = 10
three[2] = 30
$ echo $?
0
```

`rustc demo.rs` exits `0` and is silent — no warnings, no errors.
`./demo` produces four stdout lines and exits `0`. Three pieces all
compose: `vec![]` and `vec![10, 20, 30]` both produce `Vec<u64>`
values; `.len()` reads `0` and `3` (witnessing the empty form has
zero length and the prefilled form has length matching the literal
list); `three[0]` reads `10`, `three[2]` reads `30` (witnessing
zero-based indexing — index `0` is the first element).

### Probe 2 — contrast: out-of-bounds at runtime

Source (`broken.rs`):

```rust
fn main() {
    let three: Vec<u64> = vec![10, 20, 30];
    let bad = three[5];
    println!("bad = {}", bad);
}
```

Compile transcript:

```text
$ rustc broken.rs
$ echo $?
0
```

`rustc broken.rs` exits `0` and is silent. **This is the centered
pedagogical surprise**: lesson 077's auxiliary `nums[10]` on a
`[i32; 5]` *failed at compile time* with `error: this operation
will panic at runtime` and the deny-by-default `unconditional_panic`
lint, because the array's length `5` is part of its type. `Vec<u64>`
does not carry length in its type, so rustc cannot constant-evaluate
the bounds check, and the same shape compiles silently.

Run transcript:

```text
$ ./broken
$ echo $?
101
```

Stdout is empty (the `println!` never runs). Stderr (re-run from a
fresh `mktemp` directory with the filename `broken.rs` matching the
lesson's in-text filename):

```text
thread 'main' (138498819) panicked at broken.rs:3:20:
index out of bounds: the len is 3 but the index is 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The thread-id parens vary per run; lesson 053 deferred that detail.
The lesson body uses `(...)` to avoid implying a literal value. The
column `:20` is the start of `[5]` on the line `    let bad =
three[5];` — the bracketed indexing expression is what rustc reports
as the failure point.

The message line `index out of bounds: the len is 3 but the index
is 5` is exactly lesson 078's installed shape — `N = 3 = three.len()`,
`M = 5 = the failed index`. Wrapper lines `thread 'main' ... panicked
at file:line:col:` and `note: ... RUST_BACKTRACE=1 ...` and exit
status `101` are lesson 053's panic trailer.

### Probe 3 — auxiliary: `i32` index witnesses the same E0277 as arrays

Source (`i32_idx.rs`):

```rust
fn main() {
    let three: Vec<u64> = vec![10, 20, 30];
    let i: i32 = 1;
    let v = three[i];
    println!("v = {}", v);
}
```

Compile transcript (head):

```text
$ rustc i32_idx.rs
error[E0277]: the type `[u64]` cannot be indexed by `i32`
 --> i32_idx.rs:4:19
  |
4 |     let v = three[i];
  |                   ^ slice indices are of type `usize` or ranges of `usize`
  |
  = help: the trait `SliceIndex<[u64]>` is not implemented for `i32`
  ...
  = note: required for `Vec<u64>` to implement `Index<i32>`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
$ echo $?
1
```

Witnesses lesson 077's E0277 carry-through to `Vec<T>` operands.
Lesson 077's headline read `the type \`[{integer}]\` cannot be
indexed by \`i32\``; today's reads `the type \`[u64]\` cannot be
indexed by \`i32\`` — the placeholder `[{integer}]` is replaced by
the concrete element-typed slice `[u64]`, but the E-code, the
inline-gloss "slice indices are of type `usize` or ranges of
`usize`", and the trait-talk trailer (`SliceIndex<[u64]>`,
`Index<i32>`) are all the same diagnostic family. Today does not
re-install E0277; this probe is a regression check that the rule
extends to Vec.

### Probe 4 — Check Yourself (c) witness: empty-vec runtime panic

Source (`empty_idx.rs`):

```rust
fn main() {
    let empty: Vec<u64> = vec![];
    let x = empty[0];
    println!("x = {}", x);
}
```

Transcript:

```text
$ rustc empty_idx.rs
$ ./empty_idx
$ echo $?
101
```

Stdout empty; stderr:

```text
thread 'main' (138499931) panicked at empty_idx.rs:3:18:
index out of bounds: the len is 0 but the index is 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Witnesses Check Yourself (c)'s answer: an empty vector panics on
*any* index — `0 >= 0` fails the bounds check. The probe also
witnesses that compile succeeds even with a literal index `0` on a
literal `vec![]`: rustc does not constant-evaluate the bounds for a
`Vec<T>`, even when both operands are literal. This is the same
surprise as Probe 2 from a different angle.

## Claim-to-evidence map

| Lesson claim | Source |
|---|---|
| `vec![]` is admitted by the macro and produces an empty `Vec<T>` | std `macro.vec` lines 7-11 (the `()` arm); Probe 1 |
| `vec![v1, v2, ...]` is admitted by the macro and produces a prefilled `Vec<T>` | std `macro.vec` lines 7-11 (the `($($x:expr),+ ...)` arm), lines 21-28 example; Book ch08-01 lines 32-44; Probe 1 |
| The macro produces a `Vec<T>` | std `macro.vec` line 16 |
| The element type for `vec![]` cannot be inferred without an annotation | Book ch08-01 lines 23-30 verbatim "we added a type annotation here. Because we aren't inserting any values into this vector, Rust doesn't know what kind of elements we intend to store" — same logic applies to `vec![]`; confirmed by the working probe annotating explicitly |
| `vec![10, 20, 30]` would default to `Vec<i32>` without an annotation | Book ch08-01 lines 36-37 ("integer type is `i32` because that's the default integer type"); lesson 080 (default `i32` for integer literals) |
| The `!` is the macro mark | Lesson 071 (load-bearing) |
| `pub const fn len(&self) -> usize` returning the element count | std `vec/struct.Vec` lines 2116-2119 |
| `.len()` returns `usize` | std signature above; Probe 1 (`empty.len()` and `three.len()` print to `{}` slot, which formats `usize` like any integer) |
| `v[i]` indexing on `Vec<T>` | std `vec/struct.Vec` lines 86-92; Book ch08-01 lines 84-99; Probe 1 |
| Indexing requires `i: usize` | Lesson 077 (load-bearing); Probe 3 (the E0277 carry-through) |
| Out-of-bounds `v[i]` panics at runtime with `index out of bounds: the len is N but the index is M` | std `vec/struct.Vec` lines 94-102; Book ch08-01 lines 124-127; lesson 078 (load-bearing for the message shape); Probes 2 and 4 |
| `Vec<T>` does not carry length in its type, so the bounds check is runtime-only even for literal indexes | Probe 2 (literal index `5` compiles silently); contrast with lesson 077's auxiliary on a typed array `[i32; 5]` (compile-time `unconditional_panic`) |
| The panic trailer is lesson 053's shape (`thread 'main' ... panicked at ...`, exit 101) | Lesson 053 (load-bearing for panic shape); Probes 2 and 4 |

## Notes

- The probe directory `/tmp/lesson107-broken/` and other `mktemp`
  ephemeral dirs are not committed; only the
  `observations/107-vec-basics.rs` file is.
- Why `Vec<u64>` and not `Vec<i32>`? The element type was chosen to
  match rmp's actual `BigUInt::limbs: Vec<u64>` field type
  (`src/biguint/basic.rs:2-4` per the rmp target audit at
  `experimental/eduratchet2/runs/rust-moves/rmp-target-audit.md` line
  60: `pub(super) limbs: Vec<u64>`). The lesson does not load-bear on
  the specific element type; any unsigned integer suffices, and `u64`
  also matches lesson 080's integer-family vocabulary. Any `Copy`
  integer type would yield the same probe behaviour.
- The rmp source itself is referenced in the lesson body's closing
  parenthetical for context only; today's probes do not depend on
  the rmp tree being present in the local corpus.
