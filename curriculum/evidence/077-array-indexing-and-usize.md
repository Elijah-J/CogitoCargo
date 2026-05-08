# Evidence — 077-array-indexing-and-usize

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end.
  Only the working `.rs` is committed, at
  `experimental/eduratchet2/runs/rust-moves/observations/077-array-indexing-and-usize.rs`.
  The wrong-type-index, constant-out-of-bounds, and negative-literal
  contrast `.rs` files are *not* committed; the transcripts below
  are the artifacts.

Same host and toolchain as recent accepted lessons (072-076).

## Sources

### `output/docs/rust/book/ch03-02-data-types.md`

Three load-bearing spans.

Lines 63-72 (Table 3-1, *Integer Types in Rust*):

> | Length | Signed | Unsigned |
> | --- | --- | --- |
> | 8-bit | `i8` | `u8` |
> | 16-bit | `i16` | `u16` |
> | 32-bit | `i32` | `u32` |
> | 64-bit | `i64` | `u64` |
> | 128-bit | `i128` | `u128` |
> | Architecture-dependent | `isize` | `usize` |

Corpus warrant for placing `usize` in the same typed-integer family
as `i32` (lesson 019) and `u32` (lesson 062). The *Architecture-
dependent* row is what makes today's "third typed integer" framing
honest: the column-structure is the same as lesson 062's u32, but
the *Length* column changes from a fixed bit-width to
"architecture-dependent." The lesson's *The Move* and *What
Changed* both reference this row.

Lines 89-91 (architecture dependence):

> Additionally, the `isize` and `usize` types depend on the
> architecture of the computer your program is running on: 64 bits
> if you're on a 64-bit architecture and 32 bits if you're on a
> 32-bit architecture.

Direct corpus warrant for the lesson's architecture-dependent
claim. The lesson uses the Book's exact "64 bits on a 64-bit ...
32 bits on a 32-bit" framing without paraphrase, since the framing
is short, plain, and the Book is the authoritative source. Prior
lessons (062 for `u32`) deliberately avoided this row; today
crosses it because today's index type is `usize`.

Lines 109-112 (when to use `usize`):

> So how do you know which type of integer to use? If you're
> unsure, Rust's defaults are generally good places to start:
> Integer types default to `i32`. The primary situation in which
> you'd use `isize` or `usize` is when indexing some sort of
> collection.

Two load-bearing pieces:

1. *"Integer types default to `i32`"* — already cited by lessons
   019 and 062, named here only because today's literal-index
   inference (rustc inferring `usize` for `0` and `1`) is *not* the
   default — it is forced by the `Index` slot context. This is the
   subtle point the lesson handles by saying "rustc infers `usize`
   from the position": the index *position* in `a[i]` is what
   forces `usize`, not the literal-default rule.
2. *"The primary situation in which you'd use `isize` or `usize`
   is when indexing some sort of collection."* — direct corpus
   warrant for the lesson's "explicit role is *indexing
   collections*" framing in *Mental Model Delta* and "explicitly
   named by the Book as the indexing type" in *What Changed*. The
   lesson's quoted "the primary situation in which you'd use
   `isize` or `usize` is when indexing some sort of collection"
   in *The Move* is verbatim from this passage.

### `output/docs/rust/reference/expressions/array-expr.md`

Lines 100-122 (the *Array and slice indexing expressions* section).
Three load-bearing pieces.

Line 110 (the **load-bearing** sentence the lesson quotes):

> Array and slice-typed values can be indexed by writing a
> square-bracket-enclosed expression of type `usize` (the index)
> after them.

This is the corpus warrant the lesson's *The Move* quotes verbatim.
Two facts in one sentence:

1. The expression form is *square-bracket-enclosed* and follows
   the array value.
2. The index expression has type `usize`.

The lesson does not paraphrase this sentence — it quotes it
because both facts are simultaneously load-bearing for two
co-installed concepts.

Line 118 (zero-based indexing):

> Indices are zero-based for arrays and slices.

Direct corpus warrant for the lesson's *zero-based* claim
("counting starts at `0`, so the last slot of an N-element array
is `a[N - 1]`"). The lesson's *What Changed* third bullet ("Indices
are zero-based") is also from this line.

Lines 120-122 (compile-time vs runtime bounds checking — cited only
in *What To Ignore*):

> Array access is a constant expression, so bounds can be checked
> at compile-time with a constant index value. Otherwise a check
> will be performed at run-time that will put the thread in a
> *panicked state* if it fails.

Cited only in *What To Ignore For Now* for the queue-E deferral.
The lesson does not center out-of-bounds today; the Reference's
own framing already separates compile-time-constant from runtime
indexes, which exactly mirrors the queue-D-vs-queue-E split.

The lesson also cites lines 137-141 (the example block with the
`unconditional_panic` lint) implicitly via the constant-out-of-
bounds probe transcript captured below.

### `output/docs/rust/std/primitive.usize.md`

Lines 7-12:

> The pointer-sized unsigned integer type.
>
> The size of this primitive is how many bytes it takes to
> reference any location in memory. For example, on a 32 bit
> target, this is 4 bytes and on a 64 bit target, this is 8 bytes.

Cross-corroborates the Book's architecture-dependent claim with a
slightly different framing ("pointer-sized"). The lesson does not
quote this page directly — the Book's "64 bits on a 64-bit ... 32
bits on a 32-bit" framing is plainer for the audience. Page is
named here so a red-team reviewer can confirm the cross-corpus
agreement.

### Sources NOT cited as load-bearing

- `output/docs/rust/std/primitive.array.md` — already used by
  lesson 076 for the array-to-slice coercion. Today does not lean
  on coercion; the Reference passage at line 110 directly names
  array-and-slice indexing as a single rule, so the lesson can
  state the index-type fact without involving coercion.
- `output/docs/rust/error_codes/E0277.md` — the diagnostic E-code
  for the wrong-type-index probe. Probe transcript captured below
  is load-bearing; the explainer page is not separately quoted.
  Same pattern as lessons 062, 072, 073, 074, 076.
- `output/docs/rust/book/ch03-02-data-types.md` lines 389-408
  (*Array Element Access*). The Book does show `let first = a[0];`
  / `let second = a[1];` here, which corroborates today's literal-
  indexed reads. Not separately quoted because the Reference's
  line 110 is the more precise statement; the Book lines are the
  audience-level pre-figure of the same fact and are *implicitly*
  the warrant for the probe's `nums[0]` and `nums[1]` lines.
- `output/docs/rust/book/ch03-02-data-types.md` lines 410-473
  (*Invalid Array Element Access*). This is queue item E. Today's
  *What To Ignore* names the runtime-panic shape; the Book lines
  themselves are the queue-E source, not load-bearing today.

## Probes

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/077-array-indexing-and-usize.rs`)
is the *working* version. Three contrast probes (wrong-type index,
constant out-of-bounds, negative literal) are documented as
separate runs below, not committed as separate `.rs` files.

### Probe 1: working program

Captured in a fresh empty temp dir created with `mktemp -d` and
removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- cat demo.rs ---
fn main() {
    let nums = [10, 20, 30, 40, 50];
    let first = nums[0];
    let second = nums[1];
    let i: usize = 2;
    let middle = nums[i];
    println!("first = {}", first);
    println!("second = {}", second);
    println!("middle = {}", middle);
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
first = 10
second = 20
middle = 30
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (no warnings, no errors),
  consistent with lesson 001.
- `./demo` prints exactly three lines, each witnessing a distinct
  claim:
  1. `first = 10` — `let first = nums[0];` returned the
     zero-th element of the literal `[10, 20, 30, 40, 50]`. This
     is the operational witness for the *zero-based* rule: index
     `0` yields the *first* listed value, not the second.
  2. `second = 20` — `let second = nums[1];` returned `20`,
     confirming index `1` is the *second* slot.
  3. `middle = 30` — `let i: usize = 2;` followed by
     `let middle = nums[i];` returned `30`. This is the
     load-bearing witness for the *named-binding indexing* claim:
     a `usize`-typed binding can sit in the index slot. Without
     this line, the lesson's "as soon as you put a *named binding*
     in the brackets, that binding's type matters" claim would
     be untested.
- The committed `.rs` file's source matches the *Try It* code
  block exactly. Only the working source is committed under
  `observations/`.

### Probe 2: primary contrast — wrong-type index (`i32` instead of `usize`)

Same temp dir family, separate file `broken.rs`:

```text
--- cat broken.rs ---
fn main() {
    let nums = [10, 20, 30, 40, 50];
    let i: i32 = 2;
    let x = nums[i];
    println!("x = {}", x);
}
--- rustc broken.rs (capturing stderr) ---
error[E0277]: the type `[{integer}]` cannot be indexed by `i32`
 --> broken.rs:4:18
  |
4 |     let x = nums[i];
  |                  ^ slice indices are of type `usize` or ranges of `usize`
  |
  = help: the trait `SliceIndex<[{integer}]>` is not implemented for `i32`
help: the following other types implement trait `SliceIndex<T>`
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/slice/index.rs:214:0
  |
  = note: `usize` implements `SliceIndex<[T]>`
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/bstr/traits.rs:197:0
  |
  = note: `usize` implements `SliceIndex<ByteStr>`
  = note: required for `[{integer}]` to implement `Index<i32>`
  = note: 1 redundant requirement hidden
  = note: required for `[{integer}; 5]` to implement `Index<i32>`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
exit=1
--- ls after ---
broken.rs
```

Read with lesson 003's diagnostic map:

- **Headline**: `error[E0277]: the type `[{integer}]` cannot be
  indexed by `i32``. Coded `[E0277]`. New E-code for this run; the
  trait-bound family that surfaces when a method/operation is
  reported through a trait that isn't implemented.
- **Location**: `broken.rs:4:18` — line 4, column 18, the start
  of the index expression `i` inside the brackets.
- **Source excerpt with caret**: `^` underlines `i`, the offending
  identifier. The caret is on the *index expression*, not on the
  array binding — the diagnostic is about *what's in the
  brackets*, not about `nums`.
- **Inline annotation**: `slice indices are of type `usize` or
  ranges of `usize``. This is the load-bearing piece. rustc states
  the rule directly, mirroring the Reference's line-110 wording.
- **Trailer**: trait-bound machinery — `the trait
  `SliceIndex<[{integer}]>` is not implemented for `i32``,
  `required for `[{integer}; 5]` to implement `Index<i32>``. The
  lesson's *Try It* explicitly tells the learner to leave this
  for later ("the `E0277` code and the trait talk below it ...
  are the structural reason — leave that machinery for later").
  Trait machinery has been deferred since lesson 040; today
  reads E0277 only by the headline + inline gloss.
- **Coded trailer**: `For more information about this error, try
  `rustc --explain E0277`.` — present because the headline is
  coded (lesson 070).
- **Exit code**: 1; no executable produced (`ls` shows only
  `broken.rs`).

This is the load-bearing negative probe for the lesson's two
co-installed concepts. The diagnostic does double duty:

1. *Indexing rule witnessed*: rustc's own gloss ("slice indices
   are of type `usize` or ranges of `usize`") is the audience-
   level restatement of the Reference's line 110. The probe shows
   this rule is rustc-enforced, not just documentation.
2. *`usize`-load-bearing witnessed*: changing the index variable's
   type from `usize` to `i32` is what triggers the diagnostic.
   Without a *named binding* in the index slot, the lesson could
   not show this — literal indexes would silently infer `usize`
   regardless. The probe is what makes the "with `usize` works,
   without `usize` fails" framing operationally verifiable.

Why this probe and not constant-out-of-bounds: constant-out-of-
bounds (Probe 3 below) is queue item E's neighborhood — it
witnesses the *bounds* rule, not the *index type* rule. Probe 2
isolates the index-type rule cleanly: it says nothing about
bounds (the value `2` is well in range for a 5-element array), so
the only fact the diagnostic tests is the type mismatch.

### Probe 3: auxiliary — constant out-of-bounds index

Captured for evidence transparency only. **Not** referenced in
the lesson body except as the queue-item-E placeholder in *What
To Ignore*. Documented here so a red-team reviewer can see the
alternative was considered and the queue-E framing is empirically
calibrated.

```text
--- cat broken.rs ---
fn main() {
    let nums = [10, 20, 30, 40, 50];
    let x = nums[10];
    println!("x = {}", x);
}
--- rustc broken.rs (capturing stderr) ---
error: this operation will panic at runtime
 --> broken.rs:3:13
  |
3 |     let x = nums[10];
  |             ^^^^^^^^ index out of bounds: the length is 5 but the index is 10
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: aborting due to 1 previous error

rustc-exit=1
--- ls ---
broken.rs
```

Notes:

- **Slight deviation from the brief's expectation**: the brief
  guessed this would be a *warning*. On rustc 1.95.0 the
  `unconditional_panic` lint is `deny` by default, not `warn`,
  so the diagnostic is a hard `error`, not a warning. The
  message body is otherwise consistent with the Book's queue-E
  passage at lines 410-473: same "index out of bounds: the length
  is N but the index is M" wording, just delivered at compile
  time as an `error` because the index is a literal `10` (rustc
  can evaluate the bounds check without running the program).
- The lesson's *What To Ignore For Now* uses the version that
  matches this rustc release: "with a constant index past the
  end, rustc on this release fires `error: this operation will
  panic at runtime`; with a runtime index past the end, the
  program panics with `index out of bounds: the length is N but
  the index is M`."
- This probe is *not* the lesson's centered contrast because the
  brief explicitly defers out-of-bounds to queue item E. Probe 2
  is the centered contrast; Probe 3 is documented for honesty
  and to show the queue-E split is empirically grounded.

### Probe 4: auxiliary — negative literal index

Captured for evidence transparency only. Documented for
completeness; the lesson body mentions this case in *What To
Ignore* but does not center it.

```text
--- cat neg.rs ---
fn main() {
    let nums = [10, 20, 30, 40, 50];
    let x = nums[-1];
    println!("x = {}", x);
}
--- rustc neg.rs (capturing stderr) ---
error: negative integers cannot be used to index on a `[{integer}; 5]`
 --> neg.rs:3:18
  |
3 |     let x = nums[-1];
  |                  ^^ cannot use a negative integer for indexing on `[{integer}; 5]`
  |
help: to access an element starting from the end of the `[{integer}; 5]`, compute the index
  |
3 |     let x = nums[nums.len() -1];
  |                  ++++++++++

error: aborting due to 1 previous error

exit=1
```

Notes:

- Uncoded `error:` headline (no `E####`). Different diagnostic
  family from Probe 2's E0277.
- The `help:` block proposes `nums[nums.len() -1]` as the way to
  read the last element — rustc's own audience-level gloss for
  what "from-the-end indexing" requires. The lesson does not
  teach this idiom today (it composes today's `a[i]` with `.len()`
  arithmetic), but the diagnostic is consistent with today's
  rule: the index *expression* needs to evaluate to a `usize`,
  and `nums.len() - 1` does, while `-1` does not.
- Negative-literal indexing is named in *What To Ignore* with the
  specific "negative integers cannot be used to index" phrasing,
  which matches this transcript verbatim.

### Negative / contrast probes

Probe 2 is the load-bearing negative probe for the lesson's
contrastive claim. Probes 3 and 4 are auxiliary; their
transcripts ground the queue-E and negative-literal references
in *What To Ignore* but are not load-bearing for any centered
claim today.

### Reproducibility note

Probe 1 is deterministic on rustc 1.95.0 — the program has no
randomness or environment dependency.

Probe 2's headline (`error[E0277]: the type `[{integer}]` cannot
be indexed by `i32``) and inline gloss (`slice indices are of
type `usize` or ranges of `usize``) are deterministic on this
rustc release. The exact wording is rustc-version-specific; the
*shape* — coded E0277 with a "cannot be indexed by X" headline
plus a `usize`-rule gloss — is grounded in lesson 003's
diagnostic map and is stable.

Probes 3 and 4 are also rustc-version-specific in wording but
stable in shape on this release.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 077.

- **Lesson 003 (load-bearing for the diagnostic map)** —
  installs the four-part read of headline + `-->` + source
  excerpt with caret + optional `help:` / `note:` lines. Probe 2
  is read with that map only; no new diagnostic vocabulary is
  installed today.
- **Lesson 005 (load-bearing for `let name = value;`)** —
  installs the binding form. Today reuses it five times: the
  array binding (from 076), two literal-indexed reads, the
  `usize`-typed index, and the variable-indexed read. No
  extension of `let`.
- **Lesson 019 (load-bearing for the `: TYPE` slot)** — installs
  `let name: TYPE = value;` as a *type annotation*. Today plugs
  `usize` into the `TYPE` slot. The slot itself is unchanged.
  Lesson 019's *What To Ignore* explicitly named "`u32`, ...,
  `usize`" as deferred future types; lesson 062 closed the `u32`
  one, today closes `usize`.
- **Lesson 062 (load-bearing for the unsigned-integer family)** —
  installs `u32` as the unsigned counterpart to `i32`. Today
  reuses the same pattern for `usize`: same column structure in
  Table 3-1, same `u`-prefix convention, same "starts at 0"
  framing. The new piece is the *Architecture-dependent* row,
  which lesson 062 deferred. Lesson 062's *What To Ignore*
  explicitly named "`isize`, `usize`" as deferred; today closes
  the `usize` half.
- **Lesson 076 (load-bearing for the array literal and `[T; N]`
  type)** — installed the array literal form, the `[T; N]` type,
  and the `.len()` call. Today indexes the literal `[10, 20, 30,
  40, 50]` exactly the way 076 built it. The new piece is the
  `[ ]` index expression following the array value. Lesson 076's
  *What To Ignore* explicitly named "*Array element access*
  `a[i]`" as queue item D — the explicit next move. Today closes
  it.

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
- `019-type-annotation-i32` — installed `i32` as the typed-name
  install pattern. Cited above as load-bearing for the `: TYPE`
  slot.
- `040-method-call-syntax` — installed the `value.method()`
  shape. Cited in *Mental Model Delta* and *What Changed* as the
  shape today's `value[index]` rhymes with: brackets follow the
  value the way the dot does. Not load-bearing for any
  compile-or-run claim; the analogy is pedagogical.
- `068-let-binding-scope`, `069-rustc-warnings`, `070-rustc-explain`,
  `071-macro-invocation-syntax`, `072-tuple-type-and-index`,
  `073-let-tuple-destructure`, `074-char-type`, `075-const-declaration`,
  `076-array-literal-and-type` — recent lessons on the same host
  and toolchain. Mentioned only to confirm the host environment
  is unchanged.

No trait-related lesson is cited. Probe 2's diagnostic *exposes*
trait machinery (`SliceIndex<[T]>`, `Index<i32>`), but the
lesson body explicitly defers reading the trait talk; the
inline gloss "slice indices are of type `usize` or ranges of
`usize`" is sufficient and non-trait-shaped.

## Book Ch1-3 closure-pass effect

This lesson **closes item D** in
`experimental/eduratchet2/runs/rust-moves/book-ch1-3-coverage.md`.
Item D's listed prereqs were 076 (array) and 019 (typed-name
install pattern), with `usize` "introduced inline as the natural
index type, mirroring lesson 062's u32 install — small enough to
fold into one move." Today carries out exactly that plan:
indexing + `usize` co-install, with 019 + 062 + 076 as the
load-bearing prior lessons.

With `a[i]` and `usize` installed, queue item **E** (out-of-bounds
runtime panic) becomes directly approachable — Probe 3 here is
the empirical preview. Queue item **F** (`for element in array`
iteration) was already approachable from lesson 076 + 022; today
does not bear on it. Queue item **G** (full integer family)
becomes a remaining-row enumeration: lessons 019, 062, and 077
together cover three of the twelve variants in Table 3-1, and a
future lesson can name the rest as a family.
