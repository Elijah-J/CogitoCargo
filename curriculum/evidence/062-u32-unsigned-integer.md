# Evidence — 062-u32-unsigned-integer

Audit appendix for `lessons/062-u32-unsigned-integer.md`. Holds the
corpus-quote map, the toolchain string, the working and broken-contrast
probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end. Only
  the working `.rs` is committed, under
  `observations/062-u32-unsigned-integer.rs`. The broken-contrast `.rs`
  is not committed; its transcript below is the artifact.

## Sources

### `output/docs/rust/std/primitive.u32.md`

The std-library page for the `u32` primitive type. New citation in this
run; cycle 019's evidence cited the sibling `primitive.i32.md` only
transitively. One load-bearing line.

Line 8:

> The 32-bit unsigned integer type.

This single sentence is the corpus statement for "what `u32` is": (a)
"32-bit" is the size, identical to cycle 019's `i32`; (b) "unsigned"
is the contrast vs. `i32`'s signedness; (c) "integer type" places it
in the same family as `i32`.

Calibration: lines 18-20 (`pub const MIN: u32 = 0`) and lines 30-39
(`pub const MAX: u32 ... 4294967295`) supply the value range
`0..=4_294_967_295` that the lesson mentions in passing as a numeric
fact. The lesson does *not* install `u32::MIN` or `u32::MAX` as
named associated constants — those are deferred. The numeric range
itself, however, is named once because the contrastive claim "negative
literals do not fit" depends on the audience knowing that `u32`'s
smallest value is `0`. Lines 43-50 (`BITS = 32`) corroborate the
"32-bit" framing without being separately cited.

### `output/docs/rust/book/ch03-02-data-types.md`

Already cited in cycle 019. Today's re-citation is for the table-row
contrast that cycle 019's evidence pointed at without reproducing.
Three load-bearing spans.

Lines 56-61 (the Integer Types preamble — corpus origin of "unsigned
starts with `u`"):

> An *integer* is a number without a fractional component. We used one
> integer type in Chapter 2, the `u32` type. This type declaration
> indicates that the value it’s associated with should be an unsigned
> integer (signed integer types start with `i` instead of `u`) that
> takes up 32 bits of space.

The parenthetical "signed integer types start with `i` instead of `u`"
is the corpus statement that ties the `i` in `i32` to *signed* and the
`u` in `u32` to *unsigned*. The sentence "should be an unsigned
integer ... that takes up 32 bits of space" is the audience-level
gloss of `u32` that the lesson uses.

Lines 63-72 (Table 3-1, "Integer Types in Rust"):

> | Length | Signed | Unsigned |
> | --- | --- | --- |
> | 8-bit | `i8` | `u8` |
> | 16-bit | `i16` | `u16` |
> | 32-bit | `i32` | `u32` |
> | 64-bit | `i64` | `u64` |
> | 128-bit | `i128` | `u128` |
> | Architecture-dependent | `isize` | `usize` |

The 32-bit row (line 69) puts `i32` and `u32` in the same Length cell
with `i32` in the Signed column and `u32` in the Unsigned column.
This is the corpus license for the lesson's framing "`u32` is `i32`'s
unsigned sibling — same width, no negatives." The other ten cell
entries (`i8` / `u8` / `i16` / `u16` / `i64` / `u64` / `i128` /
`u128` / `isize` / `usize`) are explicitly deferred in *What To
Ignore For Now*.

Lines 74-87 (the signed-vs-unsigned semantic explanation, plus the
range arithmetic):

> Each variant can be either signed or unsigned and has an explicit
> size. *Signed* and *unsigned* refer to whether it’s possible for the
> number to be negative—in other words, whether the number needs to
> have a sign with it (signed) or whether it will only ever be
> positive and can therefore be represented without a sign (unsigned).
> ...
> Each signed variant can store numbers from −(2n − 1) to 2n − 1 − 1
> inclusive, where *n* is the number of bits that variant uses. So,
> an `i8` can store numbers from −(27) to 27 − 1, which equals −128
> to 127. Unsigned variants can store numbers from 0 to 2n − 1, so a
> `u8` can store numbers from 0 to 28 − 1, which equals 0 to 255.

Two load-bearing pieces:

1. *"unsigned ... whether it will only ever be positive and can
   therefore be represented without a sign"* — the lesson's "values
   range from 0 upward, no negatives" framing.
2. The range formula *"unsigned variants can store numbers from 0 to
   2^n − 1"* — applied with `n = 32`, this gives `0` to `2^32 − 1`,
   i.e. `0..=4_294_967_295`. The lesson mentions this range once in
   *Mental Model Delta* without unpacking the formula. The numeric
   value `4_294_967_295` is also corroborated by `primitive.u32.md`
   line 38 (`assert_eq!(u32::MAX, 4294967295);`).

The line-83 phrase "two’s complement" is *not* cited — `u32` is
unsigned and uses ordinary unsigned-binary representation, not two's
complement. The Book's two's-complement remark is signed-only. The
lesson defers both representation models.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

Already cited in cycles 042, 050, 051, 060, 061. Reused here for the
`: u32` type annotation in the canonical input-parsing line. Two
load-bearing spans.

Line 897 (the canonical Book listing using `: u32`):

> ```rust
> let guess: u32 = guess.trim().parse().expect("Please type a number!");
> ```

This is the *exact* annotation slot that today's cycle teaches: the
type name `u32` plugged into the cycle-019 `let name: TYPE = value;`
shape. The Book uses the same slot for `parse`-driven type
disambiguation; the lesson uses it for an integer-literal binding.
Identical surface, different motivation.

Lines 937-941 (audience-level prose for the `u32` annotation):

> We need to tell Rust the exact number type we want by using
> `let guess: u32`. The colon (`:`) after `guess` tells Rust we’ll
> annotate the variable’s type. Rust has a few built-in number types;
> the `u32` seen here is an unsigned, 32-bit integer. It’s a good
> default choice for a small positive number. You’ll learn about
> other number types in [Chapter 3](ch03-02-data-types.md#integer-types).

Three load-bearing fragments:

1. "We need to tell Rust the exact number type we want by using
   `let guess: u32`. The colon (`:`) after `guess` tells Rust we’ll
   annotate the variable’s type." — direct corpus parallel to cycle
   019's installed annotation form, this time with `u32` filling the
   `TYPE` slot.
2. "the `u32` seen here is an unsigned, 32-bit integer" — the most
   audience-friendly corpus statement of what `u32` means. The lesson
   reuses this phrasing nearly verbatim.
3. "It’s a good default choice for a small positive number" — corpus
   license for the lesson's framing "you reach for `u32` when the
   value is logically non-negative."

The Book's surrounding shadowing ("doesn’t the program already have a
variable named `guess`?") is *not* cited — cycle 057 already installed
type-changing shadowing, but the lesson's working probe deliberately
avoids shadowing to keep the cycle narrow.

Lines 862-865 (sibling-list passage, used for negative-evidence in
*What To Ignore For Now*):

> A few of Rust’s number types can have a value between 1 and 100:
> `i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a
> 64-bit number; as well as others. Unless otherwise specified, Rust
> defaults to an `i32` ...

This is the corpus statement that integer types other than `i32` and
`u32` exist ("as well as others") and that `i32` is the inference
default. The lesson cites cycle 019 for the i32-default fact and
defers all other integer types per *What To Ignore For Now*.

### `output/docs/rust/error_codes/E0600.md`

The error-code explainer for E0600 *cannot apply unary operator to a
type that doesn't implement it*. New E-code in this run; the lesson
deliberately does *not* install E0600 as a load-bearing noun (per the
orchestrator directive). The page is consulted only to confirm the
broken-contrast probe's E-code is named correctly.

Page header (lines 3-4):

> An unary operator was used on a type which doesn’t implement it.

The page's only example is `!Question::Yes` on a custom enum — a
unary-`!` case structurally similar to today's unary-`-` case. The
page does not specifically describe the integer case; the lesson
treats E0600 the way cycle 005 treated E0425 — by *number only*,
with the diagnostic's audience-level prose ("unsigned values cannot
be negated") doing the explanatory work, not the E-code page itself.

Calibration: today's broken-contrast diagnostic carries E0600. Were
the lesson scoped wider to install the `Neg` trait and the
"types-that-implement-an-operator" pattern, E0600 would deserve
load-bearing treatment. It is deferred per *What To Ignore For Now*
("the precise mechanism — `u32` doesn't implement `Neg` — is
uninstalled"). Cycle 034's E0277 *trait bound not satisfied* is also
mentioned in *What To Ignore For Now* as the alternate diagnostic
the orchestrator-supplied probe sketch flagged as a possibility; the
captured probe shows E0600 fires here, not E0277. The diagnostic
text "unsigned values cannot be negated" is *probe* evidence (rustc
output), not corpus evidence.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/062-u32-unsigned-integer.rs`.
Identical source to the *Try It* code block.

Transcript, captured 2026-05-07 in a fresh `mktemp -d`:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
demo.rs
--- cat demo.rs ---
fn main() {
    let n: u32 = 42;
    let m: u32 = n + 1;
    println!("n = {n}, m = {m}");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
n = 42, m = 43
exit=0
```

Notes:

- `rustc demo.rs` exits 0 silently (cycle 001 shape).
- `./demo` prints exactly one line, `n = 42, m = 43`. Three operational
  facts are corroborated empirically:
  1. `let n: u32 = 42;` compiles and the binding holds the value `42`
     — same `let name: TYPE = value;` shape cycle 019 installed for
     `i32`, with the `TYPE` slot now filled by `u32`. The integer
     literal `42` fits in a `u32` slot because `42 >= 0`.
  2. `n + 1` compiles where `n: u32` and `1` is an integer literal —
     cycle 009's `+` operator works on `u32` the same way it works on
     `i32`. The result `43` is bound to `m` annotated `: u32`.
  3. `println!("n = {n}, m = {m}");` prints both `u32` values in
     decimal — cycle 011's `{name}` placeholder applies to `u32`
     unchanged.
- Only the working source is committed under `observations/`.

### Broken-contrast probe

Source: a near-copy with the binding value changed from `42` to `-1`.
Not committed; the transcript below is the artifact. Captured
2026-05-07 in a fresh `mktemp -d`:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
broken.rs
--- cat broken.rs ---
fn main() {
    let n: u32 = -1;
    println!("n = {n}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0600]: cannot apply unary operator `-` to type `u32`
 --> broken.rs:2:18
  |
2 |     let n: u32 = -1;
  |                  ^^ cannot apply unary operator `-`
  |
  = note: unsigned values cannot be negated

help: you may have meant the maximum value of `u32`
  |
2 -     let n: u32 = -1;
2 +     let n: u32 = u32::MAX;
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0600`.
exit=1
--- ls after ---
broken.rs
```

Notes (probe evidence — not corpus quotation):

- The headline reads `error[E0600]: cannot apply unary operator `-` to
  type `u32``. New E-code in this run; mentioned by *number only* in
  the lesson body and not installed as a load-bearing noun (per the
  orchestrator directive). The audience-level prose comes from the
  `note:` line, not from the E-code page.
- The `--> broken.rs:2:18` location points at column 18 of line 2,
  which is the `-1` literal — rustc treats `-1` as the unary operator
  `-` applied to the integer literal `1`, then tries to apply it to
  the inferred-`u32` operand and fails. The caret `^^` underlines
  both characters (`-` and `1`) of the literal.
- The `note:` reads literally `unsigned values cannot be negated` —
  the most audience-readable phrasing of the constraint. The lesson
  body's framing "negative literals don't fit in a `u32`" rephrases
  this in audience-level terms.
- The `help:` block reads `you may have meant the maximum value of
  `u32`` followed by a source-diff suggestion replacing the literal
  with `u32::MAX`. This is rustc guessing the user wanted the wrap-
  around equivalent of `-1` (which in two's-complement signed
  arithmetic is the all-bits-set bit pattern, identical to `u32::MAX`
  in unsigned representation). The lesson does *not* surface this
  suggestion in the body — `u32::MAX` is deferred per *What To
  Ignore For Now*. The suggestion is captured here for completeness.
- Exit code: 1. No executable was produced; `ls after` shows only
  `broken.rs`.

The broken-contrast probe is necessary because the lesson makes a
contrastive claim (*"with `42` it works, with `-1` it does not"*).
Without the probe, the asymmetry between signed and unsigned integer
types at the literal level would be a bare assertion. The captured
diagnostic — headline E-code plus the `note: unsigned values cannot
be negated` audience-level gloss — is the load-bearing piece of probe
evidence: rustc itself rejects the negative literal in a `u32` slot
with a self-explanatory message.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 062. Older supporting lessons are mentioned
above by number only.

- **Cycle 019 (load-bearing for the `: TYPE` annotation slot)** —
  installed `let name: TYPE = value;` with `i32` as the example
  `TYPE`. Cycle 062 uses two `let n: u32 = ...;` and
  `let m: u32 = ...;` annotations in the working probe; the only
  change from cycle 019's surface is the type name in the slot.
  Cycle 019 also installed the i32-default fact ("integer literals
  default to `i32`"), which today's cycle inverts: when the desired
  type is `u32`, the annotation is *required* to override the
  default — without the `: u32`, the literal `42` would infer to
  `i32`. The required-vs-explicit distinction is corpus-grounded by
  ch02 line 937 ("we need to tell Rust the exact number type we want
  by using `let guess: u32`").
- **Cycle 009 (load-bearing for `n + 1`)** — installed `+` between
  two integer values producing a new integer value that fits on the
  right of `let`. Cycle 062 reuses cycle 009 for `n + 1` with `n: u32`
  and the integer literal `1`. The corpus-grounding for "`+` works
  on `u32` the same way" comes from `primitive.u32.md` line 8 ("The
  32-bit unsigned integer type") plus the broader Book ch03-02
  framing that integer types share arithmetic operators (Table 3-1
  rows are interchangeable in operator support — the table makes
  no per-row distinction in arithmetic). Today's working probe shows
  the `+` empirically (`42 + 1 = 43`).
- **Cycles 001, 002, 005, 011** — `rustc file.rs` then `./name`; `fn
  main` is the entry point; `let name = value;` plus the `{name}`
  placeholder for `println!`. Used unchanged.

## Older supporting lessons

- Cycle 003 (rustc diagnostic shape — headline + `-->` + source
  excerpt with caret + optional `note:` / `help:` lines). The
  broken-contrast walk uses cycle 003's map without re-teaching it.
- Cycle 004 (statements run in source order). The working probe has
  three statements; the order matters because `let m: u32 = n + 1;`
  references `n` bound on the previous line.
- Cycle 034 (the `as` cast — installed E0277 *trait bound not
  satisfied* for `i32 as f64`'s adjacent surface). Mentioned in
  *What To Ignore For Now* as the alternate diagnostic the
  orchestrator-supplied probe sketch flagged as a possibility for
  the negative-literal case. The captured probe shows E0600 fires,
  not E0277; the precise reason (`u32` doesn't implement `Neg`)
  involves trait machinery deferred since cycle 040.
