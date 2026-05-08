# Evidence — 074-char-type

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end.
  Only the working `.rs` is committed, at
  `experimental/eduratchet2/runs/rust-moves/observations/074-char-type.rs`.
  The broken-contrast `.rs` and the auxiliary multi-codepoint `.rs`
  are *not* committed; the transcripts below are the artifacts.

Same host and toolchain as recent accepted lessons (068-073).

## Sources

### `output/docs/rust/book/ch03-02-data-types.md`

The Book's *Character Type* subsection. Already cited in cycles 019,
033, 062, 072, 073 for sibling scalar-type sections; today's load-
bearing span is lines 221-245. Five load-bearing pieces.

Line 222:

> Rust's `char` type is the language's most primitive alphabetic
> type.

Corpus warrant for the lesson's first claim, "one more primitive
scalar type is named: `char`." The phrase "most primitive
alphabetic type" is the Book's own framing of `char`'s role in the
scalar-type set. The lesson re-words it to "primitive scalar type"
to keep the parallel with `i32`, `u32`, `f64`, and `bool`, all of
which sit in the same Ch3-2 subsection family.

Lines 228-234 (the Book's working example):

> ```rust
> fn main() {
>     let c = 'z';
>     let z: char = 'ℤ'; // with explicit type annotation
>     let heart_eyed_cat = '😻';
> }
> ```

The corpus's canonical three-line listing. The lesson's working
probe is structurally the same — `let c = 'z';` is verbatim; the
second line replaces the Book's `z: char = 'ℤ'` with
`letter: char = 'A'` (an ASCII letter for the canonical annotation
shape) and adds `math: char = 'ℤ'` to honor the non-ASCII
demonstration. The emoji `'😻'` is omitted to keep the source file
text-only-ASCII-plus-one-non-ASCII; the Book's "lot more than just
ASCII" claim is still witnessed by `'ℤ'` alone.

Lines 236-237 (the load-bearing single-quotes-vs-double-quotes
sentence):

> Note that we specify `char` literals with single quotation marks,
> as opposed to string literals, which use double quotation marks.

This is the corpus statement for the lesson's central contrast.
*Single quotes for `char`*, *double quotes for string literals* is
the rule the lesson installs. The Book uses the term "string
literals" exactly as the lesson does; the typed name for the
"string literal" type (`&str`) was installed incidentally by lesson
055 and is reused as a gloss only. Today's contrastive claim, "with
single quotes the `: char` annotation works; with double quotes it
fails," is the operational form of this Book sentence.

Lines 237-241 (the Unicode framing and the "lot more than just
ASCII" license):

> Rust's `char` type is 4 bytes in size and represents a Unicode
> scalar value, which means it can represent a lot more than just
> ASCII. Accented letters; Chinese, Japanese, and Korean characters;
> emojis; and zero-width spaces are all valid `char` values in
> Rust.

Two load-bearing fragments:

1. *"4 bytes in size"* — the lesson observes this as a background
   fact (in *What Changed* and *What To Ignore*). It is *not* used
   as a load-bearing premise for any compile-or-run claim. The
   lesson explicitly defers it under *What To Ignore For Now*: "The
   '4 bytes' size as load-bearing".
2. *"represents a Unicode scalar value, which means it can
   represent a lot more than just ASCII"* — the corpus license for
   *Mental Model Delta*'s "broad enough to cover accented letters,
   CJK characters, and emoji" and for the working probe's choice of
   `'ℤ'` as a non-ASCII example. The accented-letters / CJK / emoji
   list comes directly from the next Book sentence.

Lines 241-245 (the Unicode range, the "human intuition" caveat,
and the chapter-8 deferral):

> Unicode scalar values range from `U+0000` to `U+D7FF` and
> `U+E000` to `U+10FFFF` inclusive. However, a "character" isn't
> really a concept in Unicode, so your human intuition for what a
> "character" is may not match up with what a `char` is in Rust.
> We'll discuss this topic in detail in ["Storing UTF-8 Encoded
> Text with Strings"](ch08-02-strings.md).

Cited only for *What To Ignore For Now*: today defers the typed
range, the human-intuition caveat, and the UTF-8 details. The
Book's own forward pointer to chapter 8 is the corpus warrant for
"the Book defers UTF-8 to chapter 8."

Calibration: the Book's emoji example (`'😻'`) demonstrates the
"emoji is a valid `char`" subclaim; the lesson's working probe
uses `'ℤ'` instead (still non-ASCII, still a Unicode scalar
value, but visually less surprising in a terminal that may not
render colored emoji). Both characters witness the same claim
("non-ASCII works"); the substitution is a probe-presentation
choice, not a substantive deviation.

### `output/docs/rust/reference/types/char.md`

The Reference's character-type section. New citation in this run.
Two load-bearing pieces.

Line 8 (the formal definition):

> The `char` type represents a single [Unicode scalar value]
> (i.e., a code point that is not a surrogate).

Corroborates the Book's "represents a Unicode scalar value"
sentence with a more formal statement. The parenthetical "i.e., a
code point that is not a surrogate" is the technical refinement
the Book paraphrases as the `U+0000`-`U+D7FF` plus `U+E000`-
`U+10FFFF` range. The lesson does not unpack the term *surrogate*;
it is deferred under *What To Ignore For Now* ("the exact Unicode-
scalar-value range").

Line 31 (the layout claim):

> `char` is guaranteed to have the same size and alignment as
> `u32` on all platforms.

Corroborates the Book's "4 bytes in size" by giving an even more
precise spec: the Reference promises 32-bit width *and* alignment.
The lesson cites this in *What Changed* as a background corroboration
("same size and alignment as `u32`") without using it as a load-
bearing premise for any compile-or-run claim.

The Reference's *type.char.value* span (line 27) — "represented as a
32-bit unsigned word in the 0x0000 to 0xD7FF or 0xE000 to 0x10FFFF
range. It is immediate undefined behavior to create a `char` that
falls outside this range" — is *not* cited in the lesson body.
Undefined behavior is far out of scope for an audience-level
introduction; mentioned here only for transparency.

### Sources NOT cited as load-bearing

- `output/docs/rust/std/primitive.char.md` — the std primitive page
  for `char`. Useful but redundant: the Book lines 222-245 plus the
  Reference lines 8 and 31 cover every load-bearing claim. The std
  page's lines 9-12 ("A character type. ... `char` is a 'Unicode
  scalar value'.") would be a third corroboration; not separately
  cited to avoid over-citation.
- `output/docs/rust/error_codes/E0308.md` — the diagnostic E-code
  in the broken-contrast probe. The probe transcript captured here
  is load-bearing; the explainer page is not separately quoted in
  the lesson body, so it is not a separate citation. (Same pattern
  as lessons 062 and 073, where the E-code page is consulted but
  not load-bearing.)
- `output/docs/rust/reference/tokens.md` (character-and-string-
  literal grammar) — would be the formal grammar source for "single
  quotes for `char`, double quotes for string literals." The Book's
  audience-level sentence at lines 236-237 is sufficient; the
  Reference grammar is overkill for the lesson's audience and would
  install lexer-grammar vocabulary the run has not introduced.

## Probes

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/074-char-type.rs`)
is the *working* version. The broken-contrast probe and the
auxiliary multi-codepoint probe are documented as separate runs
below, not committed as separate `.rs` files (matching the
pattern of lessons 062, 071, 072, 073).

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
    let c = 'z';
    let letter: char = 'A';
    let math: char = 'ℤ';
    println!("c = {}", c);
    println!("letter = {}", letter);
    println!("math = {}", math);
}
--- rustc demo.rs (capturing stderr) ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
c = z
letter = A
math = ℤ
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent, consistent with lesson
  001 and matched by every working probe in this run.
- `./demo` prints exactly three lines, each witnessing a distinct
  claim:
  1. `c = z`: the bare `let c = 'z';` form binds and prints. The
     single-quote literal `'z'` was sufficient for rustc to infer
     the type as `char` without any annotation. This is the
     load-bearing observation for the lesson's "single-quote
     literals do the typing work" claim.
  2. `letter = A`: `let letter: char = 'A';` works — the
     lesson-019 `: TYPE` annotation slot accepts `char`. Same
     shape as `let n: u32 = 42;` (lesson 062) and
     `let x: f64 = 3.0;` (lesson 033); only the type name in the
     slot changes.
  3. `math = ℤ`: a non-ASCII character literal `'ℤ'` is also a
     valid `char`. This is the operational witness of the Book's
     "lot more than just ASCII" claim. The character `'ℤ'` is
     U+2124 (Double-Struck Capital Z), a non-surrogate Unicode
     scalar value well inside the Book's stated `U+0000`-`U+D7FF`
     range.
- The committed `.rs` file's source matches the *Try It* code
  block exactly. Only the working source is committed under
  `observations/`.

### Probe 2: broken contrast — double-quoted literal in `: char` slot

Same temp dir family, separate file `broken.rs`:

```text
--- cat broken.rs ---
fn main() {
    let c: char = "z";
    println!("c = {}", c);
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: mismatched types
 --> broken.rs:2:19
  |
2 |     let c: char = "z";
  |            ----   ^^^ expected `char`, found `&str`
  |            |
  |            expected due to this
  |
help: if you meant to write a `char` literal, use single quotes
  |
2 -     let c: char = "z";
2 +     let c: char = 'z';
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
--- ls after ---
broken.rs
```

Read with lesson 003's diagnostic map:

- **Headline**: `error[E0308]: mismatched types`. Coded `[E0308]`.
  Same E-code as the lesson 073 broken-contrast probe; same
  general "expected vs found" structure.
- **Location**: `broken.rs:2:19` — line 2, column 19, the start of
  the double-quoted literal `"z"`.
- **Source excerpt with caret**: a four-character `----`
  underlines the type annotation `char` (with the inline
  annotation `expected due to this`); a three-character `^^^`
  underlines the literal `"z"` (with the inline annotation
  `expected \`char\`, found \`&str\``). Two carets, two roles: one
  reports the *expected* type from the annotation, one reports the
  *found* type from the value. Together they tell the learner:
  "the slot says `char`, the literal gives `&str`, those do not
  match."
- **Inline annotation under the value caret**: `expected \`char\`,
  found \`&str\``. This is the lesson's load-bearing piece —
  rustc names both types in plain prose. The typed name `&str` was
  introduced incidentally by lesson 055; today reuses that gloss
  without re-installing it.
- **`help:` block**: literally `if you meant to write a \`char\`
  literal, use single quotes`, followed by a source-diff
  suggestion replacing `"z"` with `'z'`. This is rustc itself
  surfacing the very rule today's lesson installs. The lesson
  body quotes the `help:` line because it is the most audience-
  readable form of the contrast.
- **Trailer**: `For more information about this error, try
  \`rustc --explain E0308\`.` Present because the headline is
  coded (lesson 070's runnable-instruction shape).
- **Exit code**: 1; no executable produced (`ls` shows only
  `broken.rs`).

This is the load-bearing negative probe for the lesson's
contrastive claim ("with single quotes the `: char` annotation
works; with double quotes it fails"). Without this probe, the
single-vs-double quote distinction would be a bare assertion
sourced only from a Book sentence; the captured diagnostic shows
rustc itself enforcing the rule with a `help:` line that mirrors
the lesson's framing.

### Probe 3: auxiliary — multi-codepoint inside single quotes

Captured for evidence transparency only. **Not** referenced in the
lesson body. The diagnostic is lexer-level, not the E0308 type-
mismatch family the lesson installs, so it teaches a different
fact and is documented here only for completeness.

```text
--- cat aux.rs ---
fn main() {
    let c = 'ab';
    println!("c = {}", c);
}
--- rustc aux.rs (capturing stderr) ---
error: character literal may only contain one codepoint
 --> aux.rs:2:13
  |
2 |     let c = 'ab';
  |             ^^^^
  |
help: if you meant to write a string literal, use double quotes
  |
2 -     let c = 'ab';
2 +     let c = "ab";
  |

error: aborting due to 1 previous error

exit=1
```

Notes:

- The headline `error: character literal may only contain one
  codepoint` has *no* E-code. This is a non-coded error; lesson
  069's category map applies (`error:` is the category, no E-code
  trailer is present, no `--explain` follow-up). Documenting this
  is a transparency-only task; it is not surfaced in the lesson.
- The complementary `help:` block ("if you meant to write a
  string literal, use double quotes") is the symmetric mirror of
  Probe 2's help line. Together, the two probes show rustc has
  both directions of the rule built in: "double quotes around one
  letter in a `char` slot → use single quotes" and "two letters
  inside single quotes → use double quotes." The lesson uses only
  Probe 2 for its body; Probe 3 is here so a red-team reviewer
  can see that the chosen contrast (Probe 2) is the cleanest
  match for the type-annotation framing the lesson centers.

### Negative / contrast probes

Probe 2 is the load-bearing negative probe for the lesson's
contrastive claim. Probe 3 is auxiliary and not load-bearing.

### Reproducibility note

Probe 1 is deterministic on rustc 1.95.0 — the program has no
randomness or environment dependency. The non-ASCII character `'ℤ'`
is U+2124 in the source file, written in UTF-8 (the file is plain
UTF-8). The output `math = ℤ` is rendered the same way as long as
the terminal is UTF-8-capable; standard terminals on macOS and
Linux are.

Probe 2's headline (`error[E0308]: mismatched types`), inline
annotation (`expected \`char\`, found \`&str\``), and `help:`
block ("if you meant to write a `char` literal, use single
quotes") are deterministic on this rustc release. The exact
wording is rustc-version-specific; the *shape* — coded E0308 with
an "expected X, found Y" pair plus the `help:` source-diff — is
grounded in lesson 003's general diagnostic map and is stable.

Probe 3's headline (`error: character literal may only contain
one codepoint`) is also rustc-version-specific in wording but
stable in shape (an uncoded `error:` with a `help:` source-diff).

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 074. Older supporting lessons are mentioned
above by number only.

- **Lesson 005 (load-bearing for `let name = value;`)** — installs
  the binding form. Today reuses it unchanged: `let c = 'z';` is
  exactly lesson 005's shape with a single-quote literal as the
  value.
- **Lesson 019 (load-bearing for the `: TYPE` annotation slot)** —
  installs `let name: TYPE = value;` as a *type annotation*.
  Today extends the `TYPE` slot to `char`. The slot itself is
  unchanged; only the type name changes — the same kind of
  extension lessons 033 and 062 made for `f64` and `u32`. Lesson
  019's *What To Ignore* explicitly named "`char`, strings,
  tuples, arrays, structs, enums, references" as deferred future
  types; today closes the `char` half of that line.
- **Lesson 003 (load-bearing for the diagnostic map)** — installs
  the four-part read of headline + `-->` + source excerpt with
  caret + optional `help:` / `note:` lines. Probe 2 is read with
  that map only; no new diagnostic vocabulary is installed today.

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
- `033-f64-floats` — installed `f64` as the floating-point typed
  name; today's `: char` slot is structurally parallel
  (different family, same annotation shape).
- `055-string-trim` — incidentally named `&str` as the type of
  string literals. Today reuses that gloss for Probe 2's
  `expected \`char\`, found \`&str\`` annotation. The lesson
  body's reference to "the type lesson 055 calls `&str`" is the
  only place `&str` appears as a typed noun in today's prose;
  it is not re-installed.
- `062-u32-unsigned-integer` — installed `u32` as a sibling typed
  name to `i32`. Today's lesson follows the same shape: extend
  the lesson-019 `: TYPE` slot with one new type name, run a
  working probe, run a contrast probe.
- `069-rustc-warnings`, `070-rustc-explain` — the diagnostic
  category and `--explain` infrastructure that Probe 2's trailer
  exercises (`For more information about this error, try
  \`rustc --explain E0308\`.`).
- `072-tuple-type-and-index`, `073-let-tuple-destructure` — most
  recent accepted lessons on the same host and toolchain.
  Mentioned only to confirm the host environment is unchanged.

No trait-related lesson is cited. The brief explicitly excluded
trait machinery.

## Book Ch1-3 closure-pass effect

This lesson **closes item A** in
`experimental/eduratchet2/runs/rust-moves/book-ch1-3-coverage.md`.
Item A's listed prereqs were 005 (`let`) and 019 (`: TYPE`); both
were already installed before this cycle. With `char` now
installed, the Ch3-2 scalar-type set (`i32`, `u32`, `f64`, `bool`,
`char`) is fully covered. The remaining items in the Book Ch1-3
closure queue (B through T) are unaffected.
