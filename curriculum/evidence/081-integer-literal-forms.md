# Evidence — 081-integer-literal-forms

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end.
  Only the working `.rs` is committed, at
  `experimental/eduratchet2/runs/rust-moves/observations/081-integer-literal-forms.rs`.
  The contrast and auxiliary `.rs` files are *not* committed; the
  transcripts below are the artifacts.

Same host and toolchain as recent accepted lessons (074-080).

## Sources

### `output/docs/rust/book/ch03-02-data-types.md`

Two load-bearing spans, both inside the *Integer Types* subsection
the lesson 080 evidence appendix already used.

Lines 93-97 (the suffix and separator paragraphs):

> You can write integer literals in any of the forms shown in Table
> 3-2. Note that number literals that can be multiple numeric types
> allow a type suffix, such as `57u8`, to designate the type. Number
> literals can also use `_` as a visual separator to make the number
> easier to read, such as `1_000`, which will have the same value as
> if you had specified `1000`.

Direct corpus warrant for two of the lesson's load-bearing claims:

- *Type suffix*: "such as `57u8`, to designate the type." The lesson's
  *The Move* uses the Book's literal example `57u8` verbatim and
  gives the equivalent annotation form `let x: u8 = 57;`. The
  lesson's *What Changed* "a type suffix is a one-token alternative
  to a `: TYPE` annotation" is the operational rephrasing of the
  Book's "designate the type."
- *Separator*: "use `_` as a visual separator ... `1_000`, which
  will have the same value as if you had specified `1000`." The
  lesson's *The Move* and *What Changed* both quote `1_000 == 1000`
  in identical numeric form.

Lines 99-107 (Table 3-2, *Integer Literals in Rust*):

> | Number literals | Example |
> | --- | --- |
> | Decimal | `98_222` |
> | Hex | `0xff` |
> | Octal | `0o77` |
> | Binary | `0b1111_0000` |
> | Byte (`u8` only) | `b'A'` |

Direct corpus warrant for the lesson's enumeration. Five rows; the
lesson's *The Move* uses the Book's exact examples in four cases
(`0xff`, `0o77`, `0b1111_0000`, `b'A'`) and a small variant for
*Decimal* (`1_000` instead of `98_222`, with `_` to also exercise the
separator paragraph). The Book's *Byte (`u8` only)* parenthetical is
the load-bearing source for the lesson's "byte literal produces a
`u8`" claim and the *Check Yourself* (e) byte-vs-char distinction.

The "Decimal: `98_222`" row deliberately uses an underscore in the
example, which is the Book's own license for combining the separator
with the decimal form (and by table-symmetry, with hex, octal, and
binary).

### `output/docs/rust/reference/tokens.md`

Three load-bearing spans corroborating the Book's table at the formal
grammar level.

Line 295 (the *Byte literals* intro):

> A *byte literal* is a single ASCII character (in the `U+0000` to
> `U+007F` range) or a single *escape* preceded by the characters
> `U+0062` (`b`) and `U+0027` (single-quote), and followed by the
> character `U+0027`. ... It is equivalent to a `u8` unsigned 8-bit
> integer *number literal*.

Direct warrant for two of the lesson's byte-literal claims:

- *ASCII-only*: "single ASCII character (in the `U+0000` to `U+007F`
  range)." The lesson's "Only ASCII (codes `0`-`127`)" matches this
  range exactly (`0x7F == 127`). The contrast probe's headline
  `error: non-ASCII character in byte literal` is rustc itself
  enforcing the rule the Reference states.
- *Type is `u8`*: "It is equivalent to a `u8` unsigned 8-bit integer
  *number literal*." The lesson's "type: `u8`" is verbatim. The
  auxiliary mismatch probe (Probe 4 below) corroborates with rustc's
  own diagnostic naming `u8`.

Line 145 (*Suffixes*, the integer-suffix list):

> | Integer | Floating-point |
> | --- | --- |
> | `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`,
>   `i128`, `usize`, `isize` | `f32`, `f64` |

Direct corpus warrant for the lesson's "Any of the twelve integer
type names from lesson 080 may be the suffix." The Reference lists
the exact same twelve names lesson 080's Table 3-1 lists, in the
*Suffixes* section under *Numbers*. The lesson does not need to
re-quote this list because lesson 080's family-name install is
load-bearing on it.

Lines 540-556 (the four integer-literal forms, paraphrased):

> An *integer literal* has one of four forms:
>
> - A *decimal literal* starts with a *decimal digit* and continues
>   with any mixture of *decimal digits* and *underscores*.
> - A *hex literal* starts with the character sequence `U+0030`
>   `U+0078` (`0x`) and continues as any mixture (with at least one
>   digit) of hex digits and underscores.
> - An *octal literal* starts with the character sequence `U+0030`
>   `U+006F` (`0o`) and continues as any mixture (with at least one
>   digit) of octal digits and underscores.
> - A *binary literal* starts with the character sequence `U+0030`
>   `U+0062` (`0b`) and continues as any mixture (with at least one
>   digit) of binary digits and underscores.

Direct corpus warrant for "underscores between digits" being legal in
*all four* base forms (decimal, hex, octal, binary), which the
lesson's *The Move* paragraph "the underscore can appear between
digits in any of the four base forms" depends on.

The lesson's *What To Ignore For Now* names this Reference grammar as
deferred ("today follows the Book's audience-level shape, not the
formal railroad diagrams"). The grammar quoted here is used as
evidence-only corroboration; the lesson does not install
`BIN_LITERAL`, `SUFFIX_NO_E`, etc. as named concepts.

### Sources NOT cited as load-bearing

- `output/docs/rust/std/primitive.u8.md` — already cited as
  cross-corroboration for `u8::MIN/MAX` in lesson 080's evidence;
  reused implicitly through that lesson but not separately quoted.
  The byte literal `b'A' == 65u8` reuses 080's `u8 = 0..=255` range
  and does not need a separate range citation.
- `output/docs/rust/error_codes/index.md` — the
  `non-ASCII character in byte literal` error is uncoded (no `E####`
  trailer). Lesson 069's category map covers the uncoded `error:`
  shape used by Probe 2.
- `output/docs/rust/book/ch03-02-data-types.md` lines 56-91 —
  the *Integer Types* range and family content is lesson 080's
  citation; today reuses lesson 080 by name, not the Book directly,
  for the twelve-name set.

## Probes

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/081-integer-literal-forms.rs`)
is the *working* version. The contrast probe and three auxiliary
probes are documented as separate runs below, not committed as
separate `.rs` files (matching the pattern of lessons 062, 074, 077,
080).

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
    let dec = 1_000;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_0000;
    let suffix = 57u8;
    let byte = b'A';
    println!("dec = {}", dec);
    println!("hex = {}", hex);
    println!("oct = {}", oct);
    println!("bin = {}", bin);
    println!("suffix = {}", suffix);
    println!("byte = {}", byte);
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
dec = 1000
hex = 255
oct = 63
bin = 240
suffix = 57
byte = 65
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (no warnings, no errors),
  consistent with lesson 001.
- `./demo` prints six lines, each witnessing one form:
  1. `dec = 1000` — `1_000` evaluates to `1000`. Witnesses the
     separator's "no effect on value" claim. The decimal form was
     already in use since lesson 005; the literal `1_000` here only
     newly tests the `_` separator.
  2. `hex = 255` — `0xff` evaluates to `15 * 16 + 15 = 255`.
     Witnesses the hex-prefix form and the base-16 interpretation.
  3. `oct = 63` — `0o77` evaluates to `7 * 8 + 7 = 63`. Witnesses
     the octal-prefix form.
  4. `bin = 240` — `0b1111_0000` evaluates to `128 + 64 + 32 + 16 =
     240`. Witnesses the binary-prefix form *and* the separator
     inside a non-decimal base.
  5. `suffix = 57` — `57u8` binds with no `: TYPE` annotation and
     prints `57`. Witnesses the type-suffix form. The suffix is
     consumed at compile time — it does not appear in the output.
     The binding's type is `u8`; this is not directly visible from
     `{}` formatting, but Probe 5 below corroborates by triggering
     a type-mismatch when the suffixed value is forced into a
     non-`u8` slot.
  6. `byte = 65` — `b'A'` binds and prints `65`, which is the ASCII
     code of capital `A`. Witnesses the byte-literal form. As with
     Probe 1's line 5, the type is `u8` (visible via Probe 4's
     mismatch transcript below).
- The committed `.rs` file's source matches the *Try It* code block
  exactly. Only the working source is committed under
  `observations/`.

### Probe 2: contrast — non-ASCII byte literal

Same temp-dir family, separate file `broken.rs`:

```text
--- cat broken.rs ---
fn main() {
    let byte = b'ℤ';
    println!("byte = {}", byte);
}
--- rustc broken.rs ---
error: non-ASCII character in byte literal
 --> broken.rs:2:18
  |
2 |     let byte = b'ℤ';
  |                  ^
  |                  |
  |                  must be ASCII
  |                  this multibyte character does not fit into a single byte

error: aborting due to 1 previous error

exit=1
```

Read with lesson 003's diagnostic map:

- **Headline**: `error: non-ASCII character in byte literal`.
  *Uncoded* — no `E####`. Lesson 069's category map covers this
  uncoded shape.
- **Location**: `broken.rs:2:18` — line 2, column 18, the position
  of the non-ASCII character inside the byte literal.
- **Source excerpt with caret**: `^` underlines the single non-ASCII
  character `ℤ`.
- **Two label lines under the caret**: `must be ASCII` and `this
  multibyte character does not fit into a single byte`. These are
  rustc's own gloss for the byte-literal restriction the Reference
  states formally at line 295 (the `U+0000` to `U+007F` range).
- **Exit code**: 1; no executable produced.

This is the load-bearing negative probe for the lesson's
*ASCII-only* claim. Without it, "only ASCII characters are allowed
in a byte literal" would be a bare assertion sourced only from the
Reference; the captured diagnostic shows rustc itself enforcing the
rule with two label lines that name the rule directly.

Why `b'ℤ'` and not the brief's alternative `let n = _100;`: the
underscore-at-start probe fires `error[E0425]: cannot find value
\`_100\` in this scope` (Probe 3 below), which is the wrong shape
— the lexer already parsed `_100` as an *identifier* by the time
the resolver fails, so the diagnostic does not name the
"separator may not lead a literal" rule the lesson would want it
to demonstrate. The non-ASCII probe directly names the byte-
literal ASCII rule and is operationally cleaner.

### Probe 3: auxiliary — leading separator (rejected design choice)

Documented for evidence transparency; **not** referenced in the
lesson body. Confirms the brief's alternative contrast was
considered:

```text
--- cat alt.rs ---
fn main() {
    let n = _100;
    println!("n = {}", n);
}
--- rustc alt.rs ---
error[E0425]: cannot find value `_100` in this scope
 --> alt.rs:2:13
  |
2 |     let n = _100;
  |             ^^^^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
exit=1
```

The lexer parses `_100` as an *identifier* (identifiers may start
with `_`), so the rejection happens at name resolution, not at
the literal-form check. The headline `cannot find value \`_100\`
in this scope` does not mention the separator rule at all and
would teach the wrong thing. Probe 2 is the cleaner contrast.

### Probe 4: auxiliary — byte literal type confirmation

Documented to ground the lesson's "type: `u8`" claim about byte
literals operationally. **Not** referenced in the lesson body;
the lesson cites the Reference at line 295 directly:

```text
--- cat type.rs ---
fn main() {
    let c: char = b'A';
    println!("c = {}", c);
}
--- rustc type.rs ---
error[E0308]: mismatched types
 --> type.rs:2:19
  |
2 |     let c: char = b'A';
  |            ----   ^^^^ expected `char`, found `u8`
  |            |
  |            expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
exit=1
```

Notes:

- The headline `error[E0308]: mismatched types` is the same coded
  diagnostic family lesson 074 used for the `let c: char = "z";`
  contrast.
- The inline annotation `expected \`char\`, found \`u8\`` is rustc
  itself naming the byte literal's type as `u8`.
- This corroborates both the Reference's "It is equivalent to a
  `u8` ... number literal" (line 295) and the lesson's own *The
  Move* and *Mental Model Delta* claims that `b'A'` is a `u8`.
- Pedagogically also reinforces the lesson's *Check Yourself* (e)
  point about byte-vs-char being type-load-bearing.

### Probe 5: auxiliary — byte-literal value equals ASCII code

Documented to ground the lesson's specific claim "value: `65` (the
ASCII code of `A`)" via direct equality:

```text
--- cat ascii.rs ---
fn main() {
    let byte = b'A';
    let ascii_a: u8 = 65;
    let same = byte == ascii_a;
    println!("byte = {}, ascii_a = {}, same = {}", byte, ascii_a, same);
}
--- rustc ascii.rs && ./ascii ---
exit=0 (compile)
byte = 65, ascii_a = 65, same = true
exit=0 (run)
```

Notes:

- Two bindings, three observations: `byte` from `b'A'`, `ascii_a`
  from the literal `65` annotated `u8`, and a `bool` from `==`
  between them.
- Output `same = true` is the operational witness that `b'A' ==
  65u8`. Lesson 013's `==` and lesson 062's `: u8` annotation are
  both reused without modification.
- Combined with Probe 4's type witness, this shows: byte literal
  `b'A'` produces *u8 value 65*, identical in every observable way
  to the literal `65u8`. The only differences are notation
  (source-text shape) and the implicit ASCII-table lookup the
  byte-literal form does at compile time.

### Probe 6: auxiliary — type suffix combines with non-decimal forms

Documented to ground the lesson's "any of the twelve integer type
names ... [as suffix on] the literal" claim across base forms,
not just decimal:

```text
--- cat combo.rs ---
fn main() {
    let x = 0xFFu8;
    let y = 0b1111_0000u32;
    println!("x = {}, y = {}", x, y);
}
--- rustc combo.rs && ./combo ---
exit=0 (compile)
x = 255, y = 240
exit=0 (run)
```

Notes:

- `0xFFu8` is hex-form `0xFF` with suffix `u8`; value `255`,
  type `u8`.
- `0b1111_0000u32` is binary-form `0b1111_0000` with suffix `u32`;
  value `240`, type `u32`. Also exercises the separator inside a
  suffixed binary literal.
- Confirms the Reference's grammar (`INTEGER_LITERAL → ... [SUFFIX_NO_E]?`)
  applies to all four base forms equally; not just decimal.
- The lesson does not use this probe directly but relies on its
  generality to write "Any of the twelve integer type names is a
  valid suffix" without restriction to decimal.

### Probe 7: auxiliary — overflowing suffixed literal

Documented to corroborate the lesson's *What To Ignore* note about
the `overflowing_literals` lint applying to suffixed literals
exactly as it does to typed-bare literals (lesson 080):

```text
--- cat over.rs ---
fn main() {
    let x = 256u8;
    println!("x = {}", x);
}
--- rustc over.rs ---
error: literal out of range for `u8`
 --> over.rs:2:13
  |
2 |     let x = 256u8;
  |             ^^^^^
  |
  = note: the literal `256u8` does not fit into the type `u8` whose range is `0..=255`
  = note: `#[deny(overflowing_literals)]` on by default

error: aborting due to 1 previous error

exit=1
```

Notes:

- Same uncoded `error: literal out of range for \`u8\`` headline
  shape as lesson 080's contrast probe (Probe 2 of 080), with the
  one source-text difference: lesson 080 had `let too_big: u8 =
  256;` (annotation form), this one has `let x = 256u8;` (suffix
  form). Same lint, same range gloss, same exit code.
- Confirms that lesson 080's `overflowing_literals` install carries
  over unchanged to today's suffix form. Today's lesson explicitly
  defers this lint as a centered concept (already under *What To
  Ignore* on lesson 080).
- Not required to read the lesson body; documented for honesty
  about the cross-lesson interaction.

### Negative / contrast probes summary

Probe 2 is the centered, load-bearing negative probe. Probes 3-7
are auxiliary, documented for evidence transparency and to
corroborate specific load-bearing claims (type of byte literal in
Probe 4, value of byte literal in Probe 5, suffix combines with
non-decimal in Probe 6, overflow lint applies to suffixed literals
in Probe 7).

### Reproducibility note

Probe 1 is deterministic on rustc 1.95.0 — the program has no
randomness or environment dependency. Probes 2-7 are also
deterministic on this release. The exact wording of Probe 2's two
label lines (`must be ASCII` / `this multibyte character does not
fit into a single byte`) is rustc-version-specific; the *shape* —
uncoded `error:` with a "non-ASCII character in byte literal"
headline — is grounded in lesson 003's diagnostic map and is
stable.

The character `ℤ` in Probe 2 is `U+2124` (DOUBLE-STRUCK CAPITAL Z),
encoded as three UTF-8 bytes `0xE2 0x84 0xA4`. Any non-ASCII
character would produce the same headline; `ℤ` is reused from
lesson 074's working probe (`let math: char = 'ℤ';`) for
cross-lesson consistency.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 081.

- **Lesson 003 (load-bearing for the diagnostic map)** — installs
  the four-part read (headline + `-->` + source excerpt with
  caret + label/note lines). Probe 2 is read with that map only;
  no new diagnostic vocabulary is installed today. The uncoded
  `error:` shape was already covered by lesson 069.
- **Lesson 005 (load-bearing for `let name = value;`)** — installs
  the binding form. Today reuses it six times in the working probe
  with no extension.
- **Lesson 019 (load-bearing for the integer-literal default)** —
  installs `i32` as the default type for an unsuffixed integer
  literal. Today inherits this rule for the four unsuffixed
  numeric forms (decimal, hex, octal, binary). The Check Yourself
  answers (a) and (b) ride on this rule.
- **Lesson 062 (load-bearing for `u32` as a typed name)** —
  lesson 062's working probe is `let n: u32 = 42;`, the *annotation*
  form `let name: TYPE = value;` (lesson 019) with `u32` in the
  `TYPE` slot. The suffix form `0u32` does *not* appear in lesson
  062's lesson, evidence, or observation files. Today's `57u8` is
  the first time the run installs a literal-bound type *without* a
  separate `: TYPE` annotation — a one-token alternative to that
  annotation form, ranging over any of the twelve integer type
  names from lesson 080. Lesson 062's *What To Ignore For Now*
  explicitly named "type suffixes on literals (`42u32`,
  `1_000_u32`)" as *deferred*; today closes that line.
- **Lesson 080 (load-bearing for the twelve-name set)** — installs
  the family `i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize`.
  Today's "any of the twelve integer type names from lesson 080
  may be the suffix" claim depends on lesson 080's enumeration.
  Lesson 080's *What To Ignore For Now* explicitly named "integer
  literal forms — the `_` separator, hex `0x`, octal `0o`, binary
  `0b`, type suffix `57u8`, byte literal `b'A'`. ... Queue item H"
  as deferred; today closes that line.

## Older supporting lessons

Mentioned by id only, not load-bearing for any individual claim
today:

- `001-rustc-compile-and-run` — `rustc file.rs` then `./name`;
  rustc silent on success. Used as the compile-and-run shape for
  all probes.
- `002-fn-main-entry-point` — body of `fn main` runs when the
  executable launches.
- `004-statements-in-order` — sequence of `;`-terminated statements
  running top to bottom. The probe's twelve statements all reuse
  this rule.
- `011-println-positional-args` — `println!("{}", expr)`. Reused
  as-is.
- `013-comparison-operators` — `==`. Used in Probe 5 only; not
  load-bearing for any centered claim.
- `069-rustc-warnings`, `070-rustc-explain` — diagnostic-category
  infrastructure. Probe 2's uncoded `error:` shape is read with
  069's map; Probe 4's `error[E0308]` carries an `--explain` line
  read with 070's map.
- `074-char-type` (cited) — `'A'` is a `char`, contrast against the
  byte literal `b'A'` which is `u8`. Today's *What Changed* and
  *Check Yourself* (e) cite this contrast directly. Probe 4
  operationally witnesses the type difference.
- `076-array-literal-and-type` through `080-integer-type-family` —
  the integer-family arc (G, then today's H). Mentioned only to
  confirm the host environment is unchanged.

No trait-related lesson is cited.

## Book Ch1-3 closure-pass effect

This lesson **closes item H** in
`experimental/eduratchet2/runs/rust-moves/book-ch1-3-coverage.md`.
Item H's listed prereqs were 019 (i32 + literal default), 062
(`u32` as a typed name — note the planning doc's gloss
"typed-suffix shape via `0u32`" is inaccurate; lesson 062's actual
probe is `let n: u32 = 42;`, the annotation form), and "G if landed
first." Lesson 080 (item G) landed before this cycle, providing the
twelve-name set the suffix form ranges over. Today carries out
exactly the plan H describes: shallow operational install of the
notation table from Book Table 3-2, with one centered move covering
all five non-decimal forms plus the separator and the suffix slot.

With the literal forms installed, queue item **I** (integer overflow
at runtime) becomes directly approachable: I needs a small range
(`u8` from G), the typed-binding form (lesson 080), and a way to
write integer literals expressively (today). The remaining Ch1-3
closure queue items (J, K, L, M, N, O, P, Q, R, S, T, U, V) are
unaffected by today's install.
