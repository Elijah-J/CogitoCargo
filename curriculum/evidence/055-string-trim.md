# Evidence — 055-string-trim

Audit appendix for `lessons/055-string-trim.md`. Holds the
corpus-quote map, the toolchain string, the working-probe and
broken-contrast probe transcripts, and the prerequisite-claim
summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/055-string-trim.rs`); the broken-contrast `.rs` and
  the calibration probe are not committed — their transcripts below
  are the artifacts.

## Sources

### `output/docs/rust/std/primitive.str.md`

The std page for the primitive `str` type. Primary corpus source for
this cycle — both for the *`.trim()` method* and for the `&str` typed
name. Three load-bearing spans.

Lines 12-14 (the load-bearing audience-level introduction of `&str`):

> The `str` type, also called a 'string slice', is the most primitive
> string type. It is usually seen in its borrowed form, `&str`. It is
> also the type of string literals, `&'static str`.

Three corpus claims rest on this:

1. *"string slice"* is the spoken-English name for `&str`. The lesson
   uses this label in *Mental Model Delta* and the deferral list.
2. `&str` is the *borrowed form* the type is "usually seen in" — the
   lesson installs `&str` as the typed name without distinguishing
   `str` from `&str` (per the explicit deferral).
3. *"string literals"* like `"hello"` have type `&'static str` — same
   `&str` shape with a `'static` lifetime. The lesson collapses this
   to "string literals are also `&str`" and explicitly defers the
   `'static` lifetime annotation under *What To Ignore For Now*. The
   lesson body's "string literals like `\"hello\"` (every `println!`
   format string this run has used) are also `&str`" rephrases this
   line directly.

Line 1725 (the canonical `.trim()` signature):

> #### pub fn [trim](#method.trim)(&self) -> &[str](primitive.str.md)

Direct corpus statement of:
- (a) `.trim` is a method (the `&self` receiver names a method-call
  shape, lesson 040). The lesson installs (a) verbatim.
- (b) The return type is `&str`. The lesson's annotation
  `let trimmed: &str = buf.trim();` is grounded by this signature.
- (c) The receiver type is `&self` — i.e., `.trim` is defined on `str`,
  not on `String`. The lesson does *not* surface this; it observes only
  that `.trim()` works on a `String` value via the dot-form. The
  underlying mechanism (deref coercion) is explicitly deferred.

Lines 1727-1738 (the description and example):

> Returns a string slice with leading and trailing whitespace removed.
>
> 'Whitespace' is defined according to the terms of the Unicode
> Derived Core Property `White_Space`, which includes newlines.
>
> ##### Examples
>
> ```
> let s = "\n Hello\tworld\t\n";
>
> assert_eq!("Hello\tworld", s.trim());
> ```

Three load-bearing claims:
- *"leading and trailing whitespace removed"* — verbatim source for
  the lesson's "leading and trailing whitespace removed" framing.
- *"includes newlines"* — corpus license for the central practical
  observation: `read_line`'s trailing `\n` is whitespace, so `.trim()`
  removes it.
- The Examples block shows the canonical input shape (`"\n Hello\tworld\t\n"`
  → `"Hello\tworld"`) — both surrounding whitespace *and* embedded
  whitespace within the result. The lesson's probe demonstrates the
  same behavior on a `read_line` buffer and on a leading/trailing
  spaces input (`"   spaced   \n"` → `"spaced"`).

The corpus example uses `s.trim()` on a `&str` directly; the lesson
calls `.trim()` on a `String`. The Book ch02 line 925 ("The `trim`
method on a `String` instance will eliminate any whitespace at the
beginning and end") gives explicit audience-level corpus license for
the receiver-is-`String` form (cited below).

Lines 1742-1743 and 1777-1778 (the deferred sibling methods):

> #### pub fn [trim_start](#method.trim_start)(&self) -> &[str](primitive.str.md)
> Returns a string slice with leading whitespace removed.

> #### pub fn [trim_end](#method.trim_end)(&self) -> &[str](primitive.str.md)
> Returns a string slice with trailing whitespace removed.

The lesson mentions `.trim_start()` / `.trim_end()` by name only in
*What To Ignore For Now*. These signatures are the corpus license for
the existence of those siblings; the lesson does not exercise them.

Line 69 (the deferred UTF-8 invariant):

> Rust libraries may assume that string slices are always valid UTF-8.

Cited only because the lesson calls `&str` "the borrowed view into a
sequence of UTF-8 bytes." The full UTF-8 story (encoding details,
byte-vs-character distinction) is explicitly deferred.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book guessing-game chapter. Already cited in lessons 042, 044,
050, 051, 052, 053, 054. Reused here for the audience-level statement
that `.trim()` works on a `String` and what it does to `read_line`'s
trailing `\n`. Two load-bearing spans.

Lines 923-933 (the audience-level walkthrough):

> We bind this new variable to the expression `guess.trim().parse()`.
> The `guess` in the expression refers to the original `guess`
> variable that contained the input as a string. The `trim` method on
> a `String` instance will eliminate any whitespace at the beginning
> and end, which we must do before we can convert the string to a
> `u32`, which can only contain numerical data. The user must press
> `enter` to satisfy `read_line` and input their guess, which adds a
> newline character to the string. For example, if the user types
> `5` and presses `enter`, `guess` looks like this: `5\n`. The `\n`
> represents "newline." (On Windows, pressing `enter` results in a
> carriage return and a newline, `\r\n`.) The `trim` method
> eliminates `\n` or `\r\n`, resulting in just `5`.

Two corpus claims:
- *"The `trim` method on a `String` instance"* — Book-level audience
  license that `.trim()` is callable on a `String` value. The std
  page's signature is on `str` (`fn trim(&self) -> &str`), but the
  Book frames it as a method on `String`. The lesson follows the
  Book's framing and explicitly defers the underlying mechanism
  (deref coercion) under *What To Ignore For Now*.
- *"The `trim` method eliminates `\n` or `\r\n`, resulting in just
  `5`"* — direct audience-level statement of the practical effect
  this lesson installs: `read_line`'s trailing newline goes away.

Calibration: the Book's running example chains `.trim().parse()` and
also uses shadowing (`let guess: u32 = guess.trim().parse().expect(...);`).
This lesson installs *only* `.trim()` and the `&str` typed name; the
chain extension to `.parse::<T>()` and the type-changing-shadowing
pattern are explicitly deferred. The probe annotates the result
`let trimmed: &str = buf.trim();` with no shadowing and a different
binding name to keep the type-introduction visible.

### `output/docs/rust/book/ch04-03-slices.md`

The Book chapter on slices (string slices specifically). New citation
for cycle 055. Used for the audience-level definition of `&str` as
"the type that signifies 'string slice'." Two load-bearing spans.

Lines 263-264 (the Book's audience-level type spelling):

> With all this information in mind, let's rewrite `first_word` to
> return a slice. The type that signifies "string slice" is written
> as `&str`:

Direct license for the lesson's "the typed name `&str`" framing. The
ch04-03 chapter uses the spelling `&str` as canonical; the lesson
mirrors that.

Lines 369-381 (the Book's audience-level statement that string
literals are `&str`):

> Recall that we talked about string literals being stored inside the
> binary. Now that we know about slices, we can properly understand
> string literals:
>
> ```rust
> let s = "Hello, world!";
> ```
>
> The type of `s` here is `&str`: It's a slice pointing to that
> specific point of the binary. This is also why string literals are
> immutable; `&str` is an immutable reference.

Direct audience-level corpus statement that *string literals are
`&str`*. The lesson uses this in *What Changed* ("String literals like
`\"hello\"` ... are also `&str`. That fact has been hiding inside the
formatting machinery; today gives the type a name."). The Book's
phrase "an immutable reference" is the structural observation that
`&str` is the *shared* reference form `&T` (lesson 045) applied to
the underlying `str` type — the lesson does not unpack `str` alone but
points back to lesson 045 as the source of the leading `&`.

### `output/docs/rust/book/ch08-02-strings.md`

The Book chapter on Strings. Cited for the audience-level
`String`-vs-`&str` distinction. One load-bearing span.

Lines 24-37 (the audience-level distinction):

> Rust has only one string type in the core language, which is the
> string slice `str` that is usually seen in its borrowed form,
> `&str`. ...
>
> The `String` type, which is provided by Rust's standard library
> rather than coded into the core language, is a growable, mutable,
> owned, UTF-8 encoded string type. When Rustaceans refer to "strings"
> in Rust, they might be referring to either the `String` or the
> string slice `&str` types, not just one of those types.

This is the audience-level corpus statement of the
`String`-vs-`&str` distinction the lesson installs. The lesson body's
"`String` and `&str` are different types" is the operational summary;
this Book chapter is the structural corpus grounding that they are
*meant* to be different types (one core-language, one std-library; one
borrowed view, one owned). The lesson explicitly defers the
"growable, mutable, owned" details to keep cycle 042's deferral list
intact.

### `output/docs/rust/error_codes/E0308.md`

The error-code reference page for E0308. Already cited in lessons
024-034, 045-048, 052, 054. Reused here for the broken-contrast probe
— putting a `String` value into a `&str`-typed slot fires E0308 with
caret label `expected `&str`, found `String``. The lesson body cites
the E-code by family ("same E-code as lessons 024, 025, 026, 028,
033, 045, 046, 047, 048, 052") without re-explaining it.

The page's third example (lines 22-25) is structurally identical to
today's broken probe modulo the type names:

> ```rust
> let x: f32 = "Not a float";
> //     ---   ^^^^^^^^^^^^^ expected `f32`, found `&str`
> //     |
> //     expected due to this
> ```

Same caret-and-label shape rustc produces today — `expected EXPECTED,
found FOUND` with two-part underline (annotation `expected due to
this`, value with the `^` carets). The lesson body does not reproduce
this example.

### Lesson 042's evidence appendix (existing)

The fact that `String::new()` returns a fresh empty `String` — load-
bearing for both probes today (the working probe builds `buf` from
`String::new()`; the broken probe puts the result of `String::new()`
in a `&str` slot to force the type mismatch). Not re-cited inline.

### Lesson 045's evidence appendix (existing)

The fact that `&T` is the *shared reference type* and that the leading
`&` on a type name is part of the type — load-bearing for the
typed-name spelling `&str` (the leading `&` is cycle 045's same
operator-turned-type-prefix, applied here to the underlying `str`).
Not re-cited inline.

### Lesson 054's evidence appendix (existing)

The fact that `read_line` *appends* and that the trailing `\n` from
Enter is included — load-bearing for the empirical observation that
the working probe's `buf` has 6 bytes for `"hello"` (5 visible
characters + the `\n`) and `.trim()` produces a `&str` with 5
characters. Not re-cited inline.

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/055-string-trim.rs`.
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
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    let trimmed: &str = buf.trim();
    println!("buf has {} bytes; trimmed = [{trimmed}]", buf.len());
}
--- rustc demo.rs ---
rustc-exit=0
--- ls after ---
demo
demo.rs
--- echo "hello" | ./demo ---
buf has 6 bytes; trimmed = [hello]
demo-exit=0
--- printf 'hi\n' | ./demo ---
buf has 3 bytes; trimmed = [hi]
demo-exit=0
--- printf '   spaced   \n' | ./demo ---
buf has 13 bytes; trimmed = [spaced]
demo-exit=0
```

Notes (load-bearing observations):

- `rustc demo.rs` exits 0 silently. The annotation `let trimmed: &str
  = buf.trim();` type-checks: `buf.trim()` produces a `&str` and the
  `&str` slot accepts it. This corroborates the std-page signature
  `pub fn trim(&self) -> &str` (line 1725).
- `echo "hello" | ./demo` prints `buf has 6 bytes; trimmed = [hello]`.
  The `6` confirms `read_line` appended `"hello\n"` to `buf` (lesson
  054); the `[hello]` confirms `.trim()` produced a `&str` containing
  exactly `hello` — no trailing newline, no leading newline, no spaces.
  Empirical corroboration of the std-page description (line 1727,
  "Returns a string slice with leading and trailing whitespace
  removed") plus the *includes newlines* clause (line 1730).
- `printf '   spaced   \n' | ./demo` prints `buf has 13 bytes; trimmed
  = [spaced]`. Thirteen bytes = 3 spaces + `spaced` (6) + 3 spaces +
  `\n` = 13 ✓. The `[spaced]` confirms `.trim()` strips leading
  whitespace too (3 spaces) plus trailing whitespace (3 spaces and a
  `\n`). This is the empirical demonstration of *both* sides — the
  lesson body shows the trailing-only case in the canonical `hello`
  example and the both-sides case in the `spaced` example.
- `printf 'hi\n' | ./demo` prints `buf has 3 bytes; trimmed = [hi]`.
  Three bytes = `h` + `i` + `\n` = 3 ✓. Corroborates that `.trim()`
  on a 3-byte buffer with one trailing `\n` produces a 2-byte `&str`
  (the bracketed `hi` shows zero embedded characters between `[` and
  `]` other than `h` and `i`).
- The two new methods (`.trim()` and `.len()`) appear in the same
  expression. `.trim()` is the lesson's main install. `.len()` is
  small collateral: a method on `String` returning `usize` (per the
  std page `alloc/string/struct.String.md`, not re-cited inline since
  the lesson does not unpack the return type — the `{}` placeholder
  formats it). Cycle 052 has the precedent for introducing a small
  collateral inspection method (`.is_ok()`); cycle 055 follows the
  same pattern with `.len()`.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe (Shape B — `String` value into `&str` slot)

Source (not committed — the transcript below is the artifact):

```rust
fn main() {
    let s: &str = String::new();
    println!("[{s}]");
}
```

Captured 2026-05-07 in a fresh `mktemp -d` (filename `broken.rs`):

```text
--- cat broken.rs ---
fn main() {
    let s: &str = String::new();
    println!("[{s}]");
}
--- rustc broken.rs ---
error[E0308]: mismatched types
 --> broken.rs:2:19
  |
2 |     let s: &str = String::new();
  |            ----   ^^^^^^^^^^^^^ expected `&str`, found `String`
  |            |
  |            expected due to this
  |
help: consider borrowing here
  |
2 |     let s: &str = &String::new();
  |                   +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
rustc-exit=1
--- ls after rustc ---
broken.rs
```

Notes (probe evidence — not corpus quotation):

- Headline `error[E0308]: mismatched types` — same E-code lessons
  024-034, 045-048, 052, 054 captured. The lesson body cites this
  E-code precedent.
- Caret label `expected `&str`, found `String`` — the load-bearing
  line. rustc itself names `&str` and `String` as distinct types in
  the diagnostic. Empirical confirmation of the lesson's
  "`String` and `&str` are different types" claim. Same caret-shape
  rustc produces for any other E0308 mismatch (lesson 026's `if/else`
  shape, lesson 045's `&i32`/`i32` shape, etc.); the new specific
  trigger is `String` in a `&str` slot.
- Two-part underline: `----` under the type annotation `&str` labelled
  `expected due to this`, and `^^^^^^^^^^^^^` under the value
  `String::new()` labelled with the type-mismatch label. Same
  diagnostic geometry lesson 003 installed and lesson 045's E0308
  probe captured. No new diagnostic shape today.
- `help: consider borrowing here` block proposes inserting `&` to make
  `&String::new()`. Calibration: this is the *exact* `help:` text
  lesson 045's broken probe captured — but the underlying mechanism
  is different. Lesson 045's probe was `let r: &i32 = n;` where the
  fix `&n` produces a `&i32` directly. *Today's* fix `&String::new()`
  produces a `&String`, not a `&str` — and that compiles only because
  of *deref coercion* (the rule that lets `&String` substitute for
  `&str` at coercion sites; explicitly deferred since cycle 040). The
  side probe below confirms `&String::new()` does compile. The lesson
  body does not unpack this and notes only that the help suggestion
  "does propose a fix that compiles" with deferral pointer.
- `error: aborting due to 1 previous error` — single error halt, no
  binary produced. `ls after rustc` shows only `broken.rs`. Canonical
  E0308 halting behavior.
- Exit code: 1.

This probe is *load-bearing* for the lesson's claim "`String` and
`&str` are distinct types — putting a `String` value in a `: &str`
slot fires E0308." Without it, the assertion would rest only on the
corpus signatures and lesson 042+045 inheritance; the captured
transcript is the empirical confirmation specific to the
`String`/`&str` pair.

### Calibration probe — does the rustc `help:` actually work?

Source (not committed — transcript only). Tests whether the
`help: consider borrowing here` suggestion (`&String::new()`) compiles
and runs:

```rust
fn main() {
    let s: &str = &String::new();
    println!("[{s}]");
}
```

Captured 2026-05-07 in a fresh `mktemp -d` (filename `misled.rs`):

```text
--- cat misled.rs ---
fn main() {
    let s: &str = &String::new();
    println!("[{s}]");
}
--- rustc misled.rs ---
rustc-exit=0
--- ./misled ---
[]
```

Notes (probe evidence — not corpus quotation):

- The rustc-suggested fix `&String::new()` *does* compile cleanly
  (rustc-exit=0). The program runs and prints `[]` (an empty `&str`
  surrounded by brackets — empty because `String::new()` produces an
  empty `String`).
- Mechanism: deref coercion (`&String` → `&str` via the `Deref` impl
  with `Target = str`). Explicitly deferred under *What To Ignore For
  Now* in lesson 055, and deferred since cycle 040.
- The lesson body notes only that "rustc `help: consider borrowing
  here` ... does propose a fix that compiles ... the lesson does not
  unpack why." This calibration probe is captured here as the
  empirical basis for that note. Without it, the lesson would have to
  either (a) silently lie that rustc's help is wrong, (b) skip
  describing the help block, or (c) install deref coercion. (c) is
  out of scope for this cycle; (a) is dishonest; (b) elides probe
  evidence the learner *will* see when they run the broken probe. The
  chosen path is to (d) describe the help block honestly and defer the
  mechanism to a future cycle.

Not committed: the lesson does not rely on the calibration probe for
any operational claim, only for the deferral note. The transcript
above is sufficient for audit.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 055.

- **Lesson 042 (load-bearing)** — installed `String` as the standard
  heap-allocated growable text type, with `String::new()` producing a
  fresh empty value. Today's working probe builds `buf` from
  `String::new()` (via lessons 042, 006, and 054); the broken-contrast
  probe puts the same `String::new()` call result into a `&str` slot
  to force E0308. The "two distinct string types" framing today is the
  natural extension of cycle 042's `String` install — `String` was the
  only string type until now; today adds `&str` as a second.
- **Lesson 040 (load-bearing)** — installed the dot-form
  `value.method(args)`. `.trim()` is one such method; the call is
  `buf.trim()` with no additional arguments, the simplest possible
  shape lesson 040 named. No new mechanism — just a new method name in
  the same syntactic surface.
- **Lesson 045 (load-bearing for the type-spelling)** — installed `&T`
  as the *shared reference type*: a leading `&` on a type name is part
  of the type. Today's `&str` reuses that same `&` in front of a
  different underlying type. The lesson body explicitly notes the
  parallel ("the leading `&` is cycle 045's same operator-turned-type-
  prefix"). Today does *not* re-install the operator-vs-type
  distinction; the typed name `&str` is treated as a unit.
- **Lesson 019 (load-bearing for shape)** — installed
  `let name: TYPE = value;` with `i32` as the example. Lesson 055
  reuses the same shape with `&str` in the `TYPE` slot. The lesson
  body lists the `TYPE`-slot extensions in order: `i32` (019),
  `String` (042), `&i32` (045), `Ordering` (051), `&str` (055). No
  new annotation mechanism; just a new type form flowing through the
  same slot.
- **Lesson 054 (load-bearing)** — installed `read_line`'s "append the
  input + trailing newline" semantic. Today's working probe relies on
  that `read_line` leaves a trailing `\n` in `buf`; `.trim()` is then
  motivated as the standard fix. Without lesson 054, the probe would
  need a different setup (e.g., a hard-coded `String::from("\n hello
  \n")`), which would obscure the `read_line`-`.trim()` pairing the
  Book's guessing-game chapter establishes.
- **Lesson 003 (load-bearing)** — diagnostics have headline + `-->`
  location + source excerpt with caret + optional `help:` lines.
  Lesson 055's broken-contrast walk uses that map without re-teaching
  it. The `help:` block is described per lesson 003's "optional"
  framing, with the calibration note explaining its deref-coercion
  caveat.

## Older supporting lessons

- Lessons 001, 002, 005 (compile/run, `fn main`, `let` + named
  placeholder) — used unchanged.
- Lesson 006 (`let mut`-binding) — load-bearing for the working
  probe's `let mut buf` (`read_line` requires `&mut buf`, lesson 048).
  Already load-bearing in cycle 054.
- Lesson 042's `String::new()` build path — already named under direct
  prereqs above.
- Lesson 044 (`use std::io;`) — used in the working probe to shorten
  `std::io::stdin()` to `io::stdin()`. Already load-bearing in cycle
  054.
- Lesson 048 (`&mut` argument form) — used in the working probe
  (`&mut buf`). Already load-bearing in cycle 054.
- Lesson 049 (method-chaining receiver-is-any-expression) — not
  exercised today (the lesson keeps the `.trim()` call on its own
  line via a `let` binding, rather than chaining `buf.trim().len()`
  or similar). Mentioned only via the working probe's
  `read_line(&mut buf).expect(...)` chain (lesson 054).
- Lesson 050 (`io::stdin()`), 052 (`Result<T, E>`), 053
  (`.expect("msg")`) — used unchanged via cycle 054's chain.
- Lessons 024, 025, 026, 028, 033 (E0308 family — earlier
  type-annotation mismatch sub-cases) — not re-stated. Cycle 054
  already named them in family.
- Lesson 029 (underscore-prefix gloss) — not used today. The probe's
  `trimmed` and `buf` are both consumed.

## Calibration: minor surface choices not surfaced in the lesson body

- The probe uses `buf.len()` inside the `println!` format string, in a
  positional `{}` slot rather than the named `{name}` form. Lesson 005
  installed both forms; the probe uses the positional form for
  `.len()`'s output because the receiver-of-`.len()` is a method call
  (`buf.len()`) rather than a bare binding name. The named-placeholder
  form `{trimmed}` is used for the `&str` value, since `trimmed` is a
  bare binding name. No new format-string mechanism.
- The buffer is named `buf` rather than the Book's `guess`. Same
  choice as cycle 054. The probe's behavior is identical regardless of
  name.
- The `let trimmed: &str = buf.trim();` line is on its own — not
  chained into `println!` and not chained into `.parse::<T>()`. This
  keeps the `&str` typed-name introduction maximally visible at the
  lesson's central install. The Book ch02's canonical chain
  `guess.trim().parse().expect(...)` is the natural follow-up but is
  out of scope for this cycle.
- The `.len()` collateral installs on `String`, not on `&str`. Both
  types have a `.len()` method (the std page lists `pub const fn
  len(&self) -> usize` on `str` at line 83, and the alloc page lists
  the same signature on `String`); the probe's `buf.len()` calls the
  `String` version. The lesson does not distinguish the two `len`s.
- No EOF probe is run today — `./demo < /dev/null` would produce
  `buf has 0 bytes; trimmed = []` (an empty buffer, an empty trimmed
  slice, no panic), per cycle 054's side probe behavior. The lesson
  does not exercise EOF.
- The broken-contrast probe assigns `String::new()` directly into the
  `&str` slot rather than building a `String` via `read_line` first,
  to keep the broken probe minimal (no stdin needed). The type
  mismatch fires identically.
- The calibration probe (`&String::new()` works via deref coercion)
  is captured but not committed. The lesson body acknowledges the
  rustc `help:` suggestion compiles without unpacking the mechanism.
