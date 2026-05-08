# Evidence — 094-unused-must-use-result

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `cargo --version` -> `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`
- `uname -srvm` -> `Darwin 24.5.0 Darwin Kernel Version 24.5.0:
  Tue Apr 22 19:53:26 PDT 2025; root:xnu-11417.121.6~2/RELEASE_X86_64
  x86_64`
- Probes run in `/tmp/eduratchet094/` on this host. Same toolchain
  family as recent accepted lessons (082-093).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/094-unused-must-use-result.rs`
is the warning-trigger probe verbatim, with header comments naming
the expected output and the contrast probe.

## Sources

### `output/docs/rust/rustc/lints/listing/warn-by-default.md`

Lines 5742-5779 are the canonical entry for the `unused_must_use`
lint. The page is the warn-by-default lint listing — every lint
named here fires by default with no learner opt-in, which is
lesson 069's category.

> ## [unused-must-use](#unused-must-use)
>
> The `unused_must_use` lint detects unused result of a type flagged as
> `#[must_use]`.
>
> ### [Example](#example-126)
>
> ```rust
> fn returns_result() -> Result<(), ()> {
>     Ok(())
> }
>
> fn main() {
>     returns_result();
> }
> ```
>
> This will produce:
>
> ```text
> warning: unused `Result` that must be used
>  --> lint_example.rs:6:5
>   |
> 6 |     returns_result();
>   |     ^^^^^^^^^^^^^^^^
>   |
>   = note: this `Result` may be an `Err` variant, which should be handled
>   = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
> help: use `let _ = ...` to ignore the resulting value
>   |
> 6 |     let _ = returns_result();
>   |     +++++++
> ```
>
> ### [Explanation](#explanation-126)
>
> The `#[must_use]` attribute is an indicator that it is a mistake to
> ignore the value. See [the reference](../../../reference/attributes/diagnostics.md#the-must_use-attribute) for more details.

Direct corpus warrant for the lesson's centered claims:

- *Discarding a value of a `#[must_use]` type fires the
  `unused_must_use` lint*: line 5744 verbatim.
- *The diagnostic shape is `warning:` headline + bordered source
  excerpt + two `= note:` lines (one explaining "may be an `Err`
  variant," one naming the lint as `#[warn(unused_must_use)]` (part
  of `#[warn(unused)]`) on by default) + a `help: use \`let _ = ...\`
  to ignore the resulting value` line*: lines 5762-5773. Probe 1
  below reproduces this byte-for-byte.
- *The escape hatch is `let _ = ...`*: line 5770 (`help:` text) and
  line 5772 (`let _ = returns_result();`) are the corpus articulation
  of what the lesson centers as the deliberate-discard form.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book's *Handling Potential Failure with `Result`* subsection.
Lines 360-388 are the load-bearing introduction of the warning the
audience will see in the rand-capstone neighborhood:

> If you don't call `expect`, the program will compile, but you'll get a warning:
>
> ```console
> $ cargo build
>    Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
> warning: unused `Result` that must be used
>   --> src/main.rs:10:5
>    |
> 10 |     io::stdin().read_line(&mut guess);
>    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
>    |
>    = note: this `Result` may be an `Err` variant, which should be handled
>    = note: `#[warn(unused_must_use)]` on by default
> help: use `let _ = ...` to ignore the resulting value
>    |
> 10 |     let _ = io::stdin().read_line(&mut guess);
>    |     +++++++
>
> warning: `guessing_game` (bin "guessing_game") generated 1 warning
>     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
> ```
>
> Rust warns that you haven't used the `Result` value returned from `read_line`,
> indicating that the program hasn't handled a possible error.
>
> The right way to suppress the warning is to actually write error-handling code,
> but in our case we just want to crash this program when a problem occurs, so we
> can use `expect`.

Direct corpus warrant for the lesson's centered claims:

- *The probe shape — `io::stdin().read_line(&mut buf);` without
  `.expect(...)` — is the canonical Book example for this lint.*
  The Book's variable is `guess`; the lesson's is `buf` (matching
  lesson 054). Probe 1 below reproduces the rest verbatim.
- *The program still compiles* ("the program will compile, but
  you'll get a warning"). Witnessed by Probe 1's `exit=0` plus the
  produced executable.
- *The `Result` exists because the call could have been an `Err`*
  ("Rust warns that you haven't used the `Result` value returned
  from `read_line`, indicating that the program hasn't handled a
  possible error"). The lesson's *Why does this lint exist* answer
  is a paraphrase of this sentence.
- *`.expect(...)` is one way to suppress the warning; writing
  error-handling code is the "right way."* The lesson defers the
  full error-handling apparatus (the Book points readers to
  `ch09-02-recoverable-errors-with-result.md`); today only names
  `let _ = ...` as the explicit-discard form.

Note on the diagnostic differences between the Book's transcript
and this host's transcript:

- *Cargo wrapping*: the Book's transcript shows the `cargo build`
  preamble (`Compiling guessing_game v0.1.0`) and trailer (`warning:
  \`guessing_game\` (bin "guessing_game") generated 1 warning` /
  `Finished \`dev\` profile`). The lesson's probe uses bare
  `rustc demo.rs` and so emits the underlying-rustc trailer
  `warning: 1 warning emitted` instead. Same lint, same diagnostic;
  Cargo's wrapper just swaps the trailer and adds the preamble.
- *The `(part of \`#[warn(unused)]\`)` parenthetical*: the Book's
  transcript reads ``#[warn(unused_must_use)]` on by default``,
  while this host's `rustc 1.95.0` reads ``#[warn(unused_must_use)]`
  (part of `#[warn(unused)]`) on by default``. The parenthetical
  names the lint group the Book chose to omit; the lint listing
  page (above) confirms the parenthetical form is the current
  canonical wording. Same form already noted in lesson 069's
  appendix for `unused_variables`.

These are surface differences; the centered claims (warning headline,
build does not abort, executable produced, `let _ = ...` silences,
discarded `Result` may be an `Err`) are unchanged.

### `output/docs/rust/reference/attributes/diagnostics.md`

The Reference's *The `must_use` attribute* section (lines 404-447 are
load-bearing for the lesson's framing).

> The *`must_use` attribute* is used to issue a diagnostic warning
> when a value is not "used".
>
> The `must_use` attribute can be applied to user-defined composite
> types ([`struct`s], [`enum`s], and [`union`s]), [functions], and
> [traits].
>
> ...
>
> When used on user-defined composite types, if the [expression] of an
> [expression statement] has that type, then the `unused_must_use`
> lint is violated.

Direct corpus warrant for the lesson's framing:

- *`#[must_use]` is an attribute on types or functions; `Result<T,
  E>` carries it.* The Reference says the attribute can apply to
  enums (`Result` is an enum). The lint's *type* clause (line 425) —
  "if the expression of an expression statement has that type, then
  the `unused_must_use` lint is violated" — is the rule the lesson
  states as "fires when a `Result<T, E>` value is the value of an
  expression statement that nothing else uses."
- *Authoring `#[must_use]` on your own types is the broader feature
  today defers* — the Reference's lines 431-446 show user-authored
  `#[must_use]` examples, and the lesson's *What To Ignore For Now*
  bullet on "Authoring `#[must_use]`" points here implicitly.

The Reference does not use the exact phrase "warn-by-default lint"
for `unused_must_use`; that phrase comes from the rustc lint listing
above. The two pages are complementary: the Reference defines the
attribute and which expressions it covers; the lint listing names
the lint that fires and its default level.

### Lesson 069 — `rustc` warnings vs errors

Direct prerequisite. Lesson 069 installed:

- The `warning:` headline word as the category marker;
- "Warnings do not abort the build; an executable is still produced
  and `rustc` exits 0";
- The `warning: N warning(s) emitted` trailer (vs `error: aborting
  due to N previous error(s)`).

Today's lesson uses lesson 069's vocabulary verbatim and applies it
to the `unused_must_use` diagnostic. Lesson 069's *What To Ignore
For Now* explicitly named "the full `rustc` lint *system* ... the
full lint taxonomy: `dead_code`, `unused_imports`, `unused_mut`, and
many others. `unused_variables` is the example here only because it
is the smallest reachable warning." Today adds *one* specific lint
from that deferred taxonomy without reopening the full system —
`unused_must_use` joins `unused_variables` (lesson 069) and
`non_snake_case` (lesson 089) as the third specific warn-by-default
lint named in this run.

### Lesson 052 — `Result<T, E>` with `Ok`/`Err` and `.is_ok()`

Direct prerequisite. Lesson 052 installed:

- `Result<T, E>` as the prelude's two-variant enum (`Ok(T)`, `Err(E)`);
- `.is_ok()` as one consumer; the lesson framed it as "the simplest
  inspection that does not require taking the payload back out."

Today's role: the lint's name "unused `Result`" attaches because
today's discarded value is a `Result`. Lesson 052's vocabulary —
*variant*, *constructor*, *type parameter*, *prelude* — carries
through unchanged. No new claim about `Result` is introduced today.

### Lesson 053 — `.expect("msg")` and panic

Direct prerequisite. Lesson 053 installed `.expect("msg")` as the
consumer that "extracts the `Ok` payload, or panics on `Err`." The
lesson's *Try It* shows the consumer chain working; today's program
is exactly that chain *minus* the `.expect(...)` call. The lesson's
text says: "if you forget `.expect`, rustc compiles silently" —
which is *false* in the strict sense (rustc emits a warning), but
the warning was deferred until today. Lesson 053 did not assert the
silence claim outright; the lesson framed `.expect` as the way to
*get the payload out*, and today fills in what `rustc` does when no
consumer is chained at all.

### Lesson 054 — `read_line` returns a `Result`

Direct prerequisite. Lesson 054's body line 33 says
"`.read_line(&mut buf)` ... Returns a `Result` (lesson 052) whose
`Ok` payload is the byte count read." Today's probe is lesson 054's
chain `io::stdin().read_line(&mut buf).expect("Failed to read line");`
with the `.expect(...)` call removed. Both lesson 054's *What To
Ignore For Now* (the `io::Result<T>` type alias) and the centered
claim that `read_line` returns a `Result` are inherited unchanged.

## Probes

### Probe 1 — working: discarded `Result` triggers the warning

Source (`/tmp/eduratchet094/demo.rs`):

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf);
    println!("got: {}", buf);
}
```

Compile transcript:

```
$ rustc demo.rs
warning: unused `Result` that must be used
 --> demo.rs:5:5
  |
5 |     io::stdin().read_line(&mut buf);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this `Result` may be an `Err` variant, which should be handled
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
  |
5 |     let _ = io::stdin().read_line(&mut buf);
  |     +++++++

warning: 1 warning emitted

(exit 0)
$ ls
demo  demo.rs
$ echo "hello" | ./demo
got: hello

(exit 0)
```

Witnesses:

- *`warning:` headline; build does not abort*: lesson 069's category.
  Exit 0; `demo` is in `ls` output; `./demo` runs successfully.
- *Diagnostic shape matches the corpus listing*: byte-for-byte
  identical to the lint listing snippet above (modulo file name and
  line number — the listing uses `lint_example.rs:6:5`; this probe
  uses `demo.rs:5:5`).
- *`(part of \`#[warn(unused)]\`)` parenthetical present*: confirms
  the current canonical wording on this `rustc 1.95.0`. Matches
  lesson 069's appendix observation that this parenthetical is
  modern.
- *`help: use \`let _ = ...\` to ignore the resulting value` line
  present*: with the source diff `5 |     let _ = io::stdin().read_line(&mut buf);` /
  `  |     +++++++` showing the `+` underlines for the inserted
  `let _ = ` prefix.
- *Trailer reads `warning: 1 warning emitted`*, not `error: aborting
  due to 1 previous error`. Lesson 069's diagnostic-shape claim.
- *Discarded `Result` may have been `Ok`*: piping `"hello"` makes
  the system call succeed, so the discarded `Result` was `Ok(6)`
  (six bytes for `"hello\n"`) and the rest of `main` proceeded
  normally — the lesson's "got away with it" framing.

### Probe 2 — contrast: `let _ = ...` silences the warning

Source (`/tmp/eduratchet094/demo_silenced.rs`):

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
    println!("got: {}", buf);
}
```

Compile transcript:

```
$ rustc demo_silenced.rs
(no output; exit 0)
$ ls
demo_silenced  demo_silenced.rs
$ echo "world" | ./demo_silenced
got: world

(exit 0)
```

Witnesses:

- *No warning fires when `let _ = expr;` consumes the value.* Same
  source modulo line 5; the only difference is the prepended
  `let _ = `; the warning disappears.
- *The program runs identically.* `let _ = ...` does not change
  runtime behavior — it just satisfies the lint by making the value
  bound (to the placeholder `_`).
- *The `help:` line's suggestion works as advertised.* The lint
  listing (line 5770) and this run's diagnostic both suggest
  `let _ = ...`; this probe shows the suggestion silences the
  warning when applied.

This is the contrastive claim: with `read_line(&mut buf);` (Probe 1)
the warning fires; with `let _ = read_line(&mut buf);` (Probe 2) it
does not. Same shape minus the `let _ = ` prefix; same compiler;
same host.

### Probe 3 — `Option<T>` is not `#[must_use]` (negative contrast)

The lesson's *What To Ignore For Now* claims "`Option<T>` is *not*
`#[must_use]` — discarding an `Option`-returning call does not fire
this lint." This is a load-bearing claim because it bounds the
lesson's installed rule (it is "for `Result`," not "for any enum
with payloads"). Captured separately:

Source (`/tmp/eduratchet094/option_check.rs`):

```rust
fn maybe() -> Option<i32> {
    Some(7)
}

fn main() {
    maybe();
}
```

Compile transcript:

```
$ rustc option_check.rs
(no output; exit 0)
$ ls
option_check  option_check.rs
```

Witness: rustc compiles silently — no `unused_must_use` warning.
Discarding an `Option<i32>` from a function call is not a violation
of the lint. (The Reference's lint description applies only to types
flagged with `#[must_use]`; `Option<T>` does not carry that
attribute. The lesson's bound on the installed rule is empirically
correct on this host.)

This probe also doubles as a negative contrast for "the lint fires
on `#[must_use]` types only" — running the same expression-statement
shape (a bare call to a function returning a non-`#[must_use]` enum)
produces no warning.

### Probe 4 — `rustc --explain` does not work for lint names

Source: none (a documentation-only probe).

Transcript:

```
$ rustc --explain unused_must_use
error: unused_must_use is not a valid error code

(exit 0)
```

Witness: `--explain` is for `E####` codes only (lesson 070); lints
have no `E####`. The lesson's *What To Ignore For Now* bullet
"`rustc --explain unused_must_use`" reflects this; the corpus
listing at `output/docs/rust/rustc/lints/listing/warn-by-default.md`
is the canonical reference instead.

(Note: the `error:` headline here is rustc's argument-validation
error, not a compile error; exit 0 because no compilation was
attempted. Treated as a side observation, not part of the lesson's
centered claim.)

## Prerequisite-claim summary

- **Lesson 069 — `rustc` warnings vs errors** (load-bearing). The
  `warning:` headline word is the category marker; warnings do not
  abort the build, exit is 0, executable produced. Today's
  diagnostic is exactly that category.

- **Lesson 052 — `Result<T, E>` with `Ok`/`Err`** (load-bearing).
  `Result<T, E>` is a two-variant enum; the lint's name "unused
  `Result`" attaches because today's discarded value is a `Result`.

- **Lesson 053 — `.expect("msg")` and panic** (load-bearing).
  `.expect("msg")` is the consumer that today's program is missing.
  Lesson 053's chain is the canonical counter-example — it consumes
  the `Result`, so no warning fires.

- **Lesson 054 — `read_line` returns a `Result`** (load-bearing).
  `io::stdin().read_line(&mut buf)` returns a `Result<usize,
  io::Error>`. Today's probe is lesson 054's chain *without* the
  `.expect(...)` call.

- Lessons 050, 044, 003, 011, 002, 005 (cited only): `std::io::stdin()`,
  `use std::io;`, the four-part diagnostic map (headline / location /
  source excerpt with caret / help / note), `println!`, `fn main`,
  `let name = value;`. Today's probe extends none of these — the
  cited lessons are scaffolding for the program's other lines.

- Lesson 070 (`rustc --explain`) and lesson 089 (`non_snake_case`)
  are referenced only as siblings — `--explain` is named in *What To
  Ignore For Now* as not applicable here; lesson 089 is mentioned in
  this appendix as the prior accepted lesson that named another
  warn-by-default lint.

## Contrast-probe omission justification

None — Probe 2 is the explicit contrast probe (warning trigger →
silenced by `let _ = ...`), and Probe 3 is a second negative contrast
(discarded `Option<T>` does not trigger the lint). The lesson's two
contrastive claims —

1. "with `read_line(&mut buf);` the warning fires; with
   `let _ = read_line(&mut buf);` it does not"
2. "the rule is for `Result` (and other `#[must_use]` types), not
   for any enum"

— are both witnessed empirically.

## Notes on deferred items

The lesson defers (and this appendix does not probe further):

- The full `#[must_use]` ecosystem: authoring the attribute on
  user-defined types/functions/traits, the `#[must_use = "msg"]`
  message form, the `MustUse`/`MustUseTrait` examples in the
  Reference (lines 431-507).
- Lint configuration: `#[allow(unused_must_use)]`,
  `#[deny(unused_must_use)]`, the `-A`/`-D`/`-W`/`-F` flags, the
  full lint level system (Reference, lints/levels). All deferred
  since lesson 069.
- Other consumers of a `Result` that also silence the lint —
  `match`, `if let Ok(_) = ...`, binding to a real name. Compose
  already-installed lessons; not centered today.
- The `?` operator. Future move.
- The interaction between the `unused_must_use` lint and the broader
  `unused` lint group (the parenthetical `(part of \`#[warn(unused)]\`)`
  in the `= note:` line names the group); the lint group taxonomy is
  deferred since lesson 069.
- The `Stdin::read_line`-specific `is_ok` chain — calling
  `.is_ok()` after `read_line(&mut buf)` *also* fires
  `unused_must_use`, because `Result::is_ok` itself returns a
  `bool` flagged as `#[must_use]`. The lesson does not name this
  to avoid widening from "`Result` is `#[must_use]`" to "many
  things in std are `#[must_use]`." Empirically observed on this
  host but not load-bearing for the centered move.

None of these are load-bearing for the centered claim "discarding a
`Result` fires `unused_must_use`; `let _ = ...` is the documented
escape hatch."
