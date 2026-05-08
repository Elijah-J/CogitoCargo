# Evidence — 069-rustc-warnings

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Sources

### `output/docs/rust/rustc/lints/index.md`

The canonical rustc lint-system page. Two load-bearing claims for
this cycle:

- Lines 5-7: "The Rust compiler contains a number of lints, and
  when it compiles your code, it will also run the lints. These
  lints may produce a warning, an error, or nothing at all,
  depending on how you've configured things." — corpus statement
  that warning and error are *categories* a single lint can be
  configured into. The lesson's category claim is one direction of
  this: with default configuration, an `unused_variables`
  violation produces a warning, not an error.

- Lines 9-29: the exact program

  ```
  fn main() {
      let x = 5;
  }
  ```

  followed by the verbatim transcript

  ```
  warning: unused variable: `x`
   --> main.rs:2:9
    |
  2 |     let x = 5;
    |         ^
    |
    = note: `#[warn(unused_variables)]` on by default
    = note: to avoid this warning, consider using `_x` instead
  ```

  This is the corpus *existence proof* for the exact probe shape
  used in this lesson. The page's framing line on line 27-29 —
  "this is the `unused_variables` lint, and it tells you that
  you've introduced a variable that you don't use ... that's not
  *wrong*, so it's not an error, but it might be a bug, so you
  get a warning" — is the corpus articulation of the
  warning-vs-error category distinction.

  **Calibration**: the captured 1.95.0 transcript differs slightly
  from the corpus snippet. On 1.95.0, the `help:` line is folded
  onto the caret line ("`^ help: if this is intentional, prefix
  it with an underscore: `_x``") rather than appearing as a
  second `= note:` line, and the `= note:` lint pointer reads
  ``#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by
  default`` rather than just ``#[warn(unused_variables)]` on by
  default``. These are surface differences; the categorical claim
  (warning headline, no abort, exit 0, executable produced) is
  unchanged.

### `output/docs/rust/rustc/lints/levels.md`

Lines 4-11 enumerate the six lint *levels* (allow / expect /
warn / force-warn / deny / forbid). Lines 105-131 describe the
`deny` level: "A 'deny' lint produces an error if you violate
it." This is the corpus warrant for the lesson's claim that error
vs warning is a *category* a lint can land in, not a property of
the diagnostic shape; rustc 1.95.0's defaults put
`unused_variables` in `warn`, but the level system lets the same
lint produce an `error:` headline (and abort) under `deny`.

The lesson explicitly defers the level taxonomy and the `-A` /
`-W` / `-D` / `-F` flags. This page is cited only for the
categorical "warnings vs errors are the two output flavors of the
lint system" claim. No quote from the level enumeration is in
the lesson body.

### `output/docs/rust/rustc/lints/listing/warn-by-default.md`

Lines 5871-5891. The canonical listing entry for the
`unused_variables` lint, with its `Example` snippet (the same
`let x = 5;` shape as this lesson's probe) and its expected
warning text:

```
warning: unused variable: `x`
 --> lint_example.rs:2:5
  |
2 | let x = 5;
  |     ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default
```

This matches the captured 1.95.0 transcript byte-for-byte except
for the source path / line / column (which differ because the
captured probe wraps the binding in `fn main`, shifting `let` from
top-level line 2 to indented line 2 column 9). This is the corpus
warrant that the captured `(part of `#[warn(unused)]`)` parenthetical
is the *current* canonical form of the note, not an artifact.

### `output/docs/rust/error_codes/index.md` and `error_codes/`

Lines 1-3 of `index.md`: "This page lists all the error codes
emitted by the Rust compiler." The page enumerates `E####` codes
only. A directory listing of `output/docs/rust/error_codes/`
returns 520 files, of which exactly two are non-`E####` files
(`index.md`, `error-index.md`); zero `W####` files exist. This
is the corpus warrant for the lesson's framing that there is no
`W####` enumeration parallel to the `E####` system: warnings
are emitted by *lint name* (`unused_variables`, etc.), not by an
enumerated warning code. The lesson does not state this directly
in the body but relies on it implicitly when it tells the reader
to read the `= note:` lint name as the warning's identifier and
defers the lint system as a whole.

### `experimental/eduratchet2/runs/rust-moves/lessons/003-read-rustc-diagnostic.md`

Cross-referenced for the diagnostic-shape vocabulary (headline,
`-->` location, source excerpt with caret, help / note,
`aborting due to N previous error(s)` trailer). The lesson 069
body uses lesson 003's terms verbatim and does not re-teach. The
load-bearing claim from lesson 003 reused here is structural:
the four-part skeleton is shared between error and warning
diagnostics; only the headline word and the trailer wording
change. Lesson 003's What-To-Ignore explicitly deferred warnings
("`warning:` instead of `error:` ... and the `rustc` lint system.
Same skeleton, different category. Deferred."). This cycle
resolves that deferral for the *category* claim only; the lint
system itself remains deferred.

## Probe

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/069-rustc-warnings.rs`)
is the program shown in the lesson body. Captured in a temp dir
created with `mktemp -d` and removed at the end:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let x = 5;
}
--- rustc demo.rs (capturing both streams) ---
warning: unused variable: `x`
 --> demo.rs:2:9
  |
2 |     let x = 5;
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: 1 warning emitted

exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
exit=0
--- file demo demo.rs ---
demo:    Mach-O 64-bit executable x86_64
demo.rs: c program text, ASCII text
```

### Notes from the transcript

The four load-bearing observations:

1. **Headline word**: `warning:` (not `error:`). First word of
   the diagnostic block. This is the category marker the lesson
   teaches.
2. **Trailer**: `warning: 1 warning emitted`. *Not*
   `error: aborting due to N previous error(s)`. The word
   "aborting" is absent. (Cross-check: lesson 003's appendix
   captures `error: aborting due to 1 previous error` as the
   error trailer; lesson 068's appendix Run 2 captures the same
   form. Same shape, different sentence — this is the trailer
   contrast.)
3. **Exit code**: 0. The shell sees `rustc demo.rs` as a
   successful invocation. Lesson 003's appendix and lesson 068's
   Run 2 both captured `exit=1` for `error:`-class diagnostics.
4. **Executable**: `demo` appears in `ls after compile` and runs
   silently with exit 0. Lessons 002, 003, 005, 068 all captured
   the contrast: with an `error:` diagnostic, no executable is
   produced. This is the structural payoff of the category claim.

Diagnostic-shape parts inside the warning, mapped with lesson 003
vocabulary:

- **Headline**: `warning: unused variable: \`x\``. No `[W####]`
  bracketed code; the `= note:` line below names the lint instead.
- **Location**: ` --> demo.rs:2:9`. Same `file:line:column` shape
  as every error case so far.
- **Source excerpt with caret**: line 2 reprinted with one `^`
  under column 9 (the `x` of `let x = 5;`). Same bordered-block
  shape as error case.
- **Help (folded onto caret line)**: ` ^ help: if this is
  intentional, prefix it with an underscore: `_x``. Lesson 003
  already captured the same "help folded onto the caret line"
  layout in lesson 002's E0601 transcript ("consider adding a
  `main` function to `hello.rs`"); the layout is shared between
  categories, no new diagnostic-structure concept.
- **`= note:`**: ``= note: `#[warn(unused_variables)]` (part of
  `#[warn(unused)]`) on by default``. Same `= note:` shape as
  lesson 003's `= note: similarly named macro` line. The content
  inside (lint name, attribute syntax, group membership) is what
  the lesson defers as "the lint system."

### Contrast with prior error transcripts (no new probe captured)

The lesson's category claim has the form "with `warning:`,
executable is produced and exit is 0; with `error:`, no
executable and exit 1." The error half of this contrast is
already empirically witnessed in committed evidence on the same
host and same `rustc 1.95.0`:

- Lesson 003's appendix transcript: `error: cannot find macro
  \`prntln\`` → `exit=1`, `ls after` shows only `prntln.rs` (no
  executable produced).
- Lesson 005's appendix: E0425 → no executable.
- Lesson 068's appendix Run 2: E0425 → `exit=1`, only the
  *previous* run's executable persists, no new executable from
  the broken run.

No additional negative probe is captured for this cycle: the
contrast is structural and already on file three times.

### Calibration probe for the Check-Yourself (d) prediction

The lesson's `Check Yourself` (d) asks the learner to predict
that

```rust
fn main() {
    let n = m;
}
```

produces an `error:`-headlined diagnostic and no executable. Run
on the same host:

```text
--- rustc tmp.rs ---
error[E0425]: cannot find value `m` in this scope
 --> tmp.rs:2:13
  |
2 |     let n = m;
  |             ^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
exit=1
--- ls after ---
tmp.rs
```

This is the same lesson-005 / lesson-068 E0425 shape; the
prediction reduces to the contrast already on file. No `warning:`
block is emitted alongside (rustc suppresses the
`unused_variables` warning on `n` when the assignment expression
itself fails to resolve), so the "first word is `error:`"
framing in the lesson's answer (d) is empirically correct. No
new committed probe; this transcript is documented inline only.

## Prior lessons

Direct prerequisites (load-bearing claims):

- `001-rustc-compile-and-run` (accepted) — `rustc file.rs`
  silent on success, produces executable next to source. The
  lesson's "exit 0 and an executable appears" observation is
  framed against this baseline.
- `002-fn-main-entry-point` (accepted) — `fn main() { ... }` is
  the program shape used here.
- `003-read-rustc-diagnostic` (accepted, load-bearing) — the
  diagnostic-shape map (headline, `-->` location, source excerpt
  with caret, help/note, `aborting due to N previous error(s)`
  trailer). Lesson 069 reads the warning with this map and adds
  exactly one delta: the headline word and trailer sentence
  change between categories. Lesson 003's What-To-Ignore line
  about deferred warnings is the explicit pointer this cycle
  resolves.
- `005-let-binding` (accepted) — `let name = value;` form, used
  here as the smallest reachable unused-variable trigger.

Older supporting lessons (mentioned by id only in the lesson's
contrast paragraph):

- `068-let-binding-scope` — third recent E0425 contrast on the
  same `rustc 1.95.0`; cited only to extend the "errors abort,
  no executable" baseline.
