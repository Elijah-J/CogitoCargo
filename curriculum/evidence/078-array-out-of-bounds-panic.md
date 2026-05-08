# Evidence — 078-array-out-of-bounds-panic

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end.
  Only the working `.rs` is committed, at
  `experimental/eduratchet2/runs/rust-moves/observations/078-array-out-of-bounds-panic.rs`.
  The in-bounds-contrast and constant-out-of-bounds `.rs` files
  are *not* committed; their transcripts below are the artifacts.

Same host and toolchain as recent accepted lessons (072-077).

## Sources

### `output/docs/rust/book/ch03-02-data-types.md`

Two load-bearing spans. The Book's *Invalid Array Element Access*
subsection runs from line 410 to line 473.

Lines 446-457 (the panic transcript shape — the spec the lesson
quotes verbatim):

> If you instead enter a number past the end of the array, such as
> `10`, you'll see output like this:
>
> ```console
> thread 'main' panicked at src/main.rs:19:19:
> index out of bounds: the len is 5 but the index is 10
> note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
> ```

Direct corpus warrant for the lesson's centered panic-transcript
display. The Book uses the same five-element array
`[1, 2, 3, 4, 5]` (lengths match) and the same out-of-bounds
index `10` the lesson uses, so the message line `index out of
bounds: the len is 5 but the index is 10` is *exactly* the Book's
text. The lesson's working probe is a smaller variant (no
`io::stdin`, no `.trim()`) but produces an identically shaped
panic — Probe 1 below corroborates this empirically.

The lesson body uses ellipsis `(...)` for the thread-id parens
because lesson 053 already deferred the thread-id (`.../* an OS
thread identifier; varies per run; not load-bearing */`). The
Book's transcript above does not have a thread id at all on this
release; rustc 1.95.0 does emit one (visible in Probe 1). The
lesson's `(...)` matches the Probe 1 shape; the missing-paren
shape in the Book transcript is older rustc output.

Lines 459-466 (the bounds-rule explanation — the rule the lesson
states in *The Move* and *What Changed*):

> The program resulted in a runtime error at the point of using
> an invalid value in the indexing operation. The program exited
> with an error message and didn't execute the final `println!`
> statement. When you attempt to access an element using indexing,
> Rust will check that the index you've specified is less than
> the array length. If the index is greater than or equal to the
> length, Rust will panic. This check has to happen at runtime,
> especially in this case, because the compiler can't possibly
> know what value a user will enter when they run the code later.

Direct corpus warrant for the lesson's central claim. Two
load-bearing pieces:

1. *"Rust will check that the index you've specified is less than
   the array length. If the index is greater than or equal to the
   length, Rust will panic."* — the bounds-rule statement. The
   lesson's *The Move* states this as "if `i >= a.len()` the
   program panics." Same condition, same consequence.
2. *"This check has to happen at runtime ... because the compiler
   can't possibly know what value a user will enter when they run
   the code later."* — the runtime-vs-compile-time distinction.
   The lesson's *What Changed* third bullet ("Compile succeeds
   when the index is a runtime value") restates this for the
   `.parse()` substitute the lesson uses instead of `read_line`.

The lesson also cites the Book's "the program exited with an
error message and didn't execute the final `println!` statement"
in *Try It* via the empirical observation that `./demo` produces
no `stdout`.

Lines 468-471 (memory-safety framing — cited but not centered):

> This is an example of Rust's memory safety principles in action.
> In many low-level languages, this kind of check is not done, and
> when you provide an incorrect index, invalid memory can be
> accessed. Rust protects you against this kind of error by
> immediately exiting instead of allowing the memory access and
> continuing.

Cited in the lesson's *Mental Model Delta* and the *What Changed*
bullet "Rust does not let invalid indexing read past the array's
memory; it terminates instead." The Book's framing is more
expansive ("In many low-level languages, this kind of check is
not done"); the lesson keeps the rule without expanding into
language-comparison territory.

The full Book example using `io::stdin().read_line(...)` and
`.trim().parse().expect(...)` (lines 418-441) is the canonical
runtime-index demo. The lesson uses a smaller substitute (a
hard-coded string fed straight to `.parse()`) because the
`io::stdin` machinery is not load-bearing for the bounds rule —
all that matters is that the index is *runtime-built*. The Book
example is named in *What Changed* as the natural full-stack
form, but the centered probe is the smaller substitute (Probe 1
below).

### `output/docs/rust/reference/expressions/array-expr.md`

Lines 120-122 (the compile-time-vs-runtime split — load-bearing
for the lesson's runtime-vs-compile-time framing):

> Array access is a constant expression, so bounds can be checked
> at compile-time with a constant index value. Otherwise a check
> will be performed at run-time that will put the thread in a
> *panicked state* if it fails.

Direct corpus warrant for the lesson's runtime-vs-compile-time
distinction. The Reference is more precise than the Book on this
specific split: it explicitly names *constant index value* as the
compile-time-checked case and *otherwise* as the runtime-checked
case. This is the load-bearing sentence behind the lesson's "The
trick on line 4 is reusing lesson 056 ... Because the value comes
through a `.parse()` call, rustc treats `bad_index` as a runtime
value — it does not constant-evaluate the index, so it does not
fire lesson 077's auxiliary `error: this operation will panic at
runtime`."

The phrase *panicked state* is the Reference's term. Lesson 053
already installed *panic* as the operational concept; today does
not need a separate *state* vocabulary.

The accompanying Reference example block at lines 124-143 includes
this fragment (load-bearing for the constant-vs-runtime
distinction within a single example):

> ```rust
> #![allow(unused)]
> fn main() {
> // lint is deny by default.
> #![warn(unconditional_panic)]
>
> ([1, 2, 3, 4])[2];        // Evaluates to 3
> // ...
> let x = (["a", "b"])[10]; // warning: index out of bounds
>
> let n = 10;
> let y = (["a", "b"])[n];  // panics
>
> let arr = ["a", "b"];
> arr[10];                  // warning: index out of bounds
> }
> ```

The split this example draws is exactly the lesson's split:
`(["a", "b"])[10]` (constant index) is the compile-time-rejected
case; `let n = 10; (["a", "b"])[n]` (variable index assigned a
constant) is the runtime-panic case. The lesson's *Try It* picks
the latter shape (runtime-built index → runtime panic) for its
working probe.

A subtlety the example shows but the lesson does not need: the
Reference uses `let n = 10;` to keep the index runtime — but on
rustc 1.95.0, that *also* gets folded by const-eval and triggers
the same `unconditional_panic` lint. The lesson uses
`bad_index_str.parse().expect(...)` instead, which rustc cannot
const-evaluate (parsing a string is too expensive for const-eval
on this release), so the working probe reliably reaches runtime.
Probe 1 below empirically confirms `rustc demo.rs` exits `0` for
the parse-built index.

The Reference's `// lint is deny by default.` comment plus
`#![warn(unconditional_panic)]` confirms what lesson 077's
auxiliary observed: the lint is deny-by-default in normal use,
and only flips to a warning when explicitly downgraded via the
`#![warn(...)]` attribute. The lesson's *Check Yourself* (c)
references this fact ("the `unconditional_panic` lint fired
then").

### Sources NOT cited as load-bearing

- `output/docs/rust/std/result/enum.Result.md` — the `.expect`
  source already cited by lessons 053 and 056. The lesson reuses
  `.expect` only as the lesson-056 chain step `.parse().expect(...)`;
  no new fact about `.expect` is installed.
- `output/docs/rust/std/primitive.array.md` — already used by
  lessons 076 and 077. Today does not lean on a fact this page
  uniquely supplies; the bounds-check rule is installed by the
  Book at lines 459-471 (corpus) plus the Reference at lines
  120-122 (corpus).
- `output/docs/rust/std/option/enum.Option.md` — `Option` is
  named only in *What To Ignore* under the `a.get(i)` deferral.
  Not a centered concept today; no quote needed.
- `output/docs/rust/error_codes/E0277.md` — no E-coded diagnostic
  is captured today. Probe 3 fires an uncoded `error: this
  operation will panic at runtime`, the same diagnostic shape
  lesson 077 captured in its Probe 3.

## Probes

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/078-array-out-of-bounds-panic.rs`)
is the *working* version (out-of-bounds runtime panic). Two
contrast probes (in-bounds runtime index, constant out-of-bounds)
are documented as separate runs below, not committed as separate
`.rs` files.

### Probe 1: working program (out-of-bounds runtime panic)

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
    let bad_index_str = "10";
    let bad_index: usize = bad_index_str.parse().expect("not a number");
    let element = nums[bad_index];
    println!("element = {}", element);
}
--- rustc demo.rs ---
rustc-exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo (stdout) ---
demo-exit=101
--- ./demo (stderr) ---

thread 'main' (132165324) panicked at demo.rs:5:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent. No `unconditional_panic`
  lint fires — rustc cannot const-evaluate the `.parse()` call,
  so the bounds check falls to runtime. This is the load-bearing
  fact the lesson hinges on: the index is runtime-built, so the
  program *compiles*, and the panic fires only when `nums[
  bad_index]` evaluates.
- `./demo` produces no `stdout`. The `println!` on line 6 never
  runs, witnessing the Book's "didn't execute the final
  `println!` statement" (lines 460-461).
- The `stderr` block has the exact three-line shape the Book
  describes at lines 453-457:
  - `thread 'main' (132165324) panicked at demo.rs:5:19:` —
    matches lesson 053's panic-trailer shape with location
    `<file>:<line>:<col>`. The location points at the *indexing
    expression* `nums[bad_index]` on line 5, not the binding line
    where `bad_index` was built. Column `19` is where the `[`
    opens.
  - `index out of bounds: the len is 5 but the index is 10` —
    matches the Book's spec at line 455 *verbatim* (modulo `the
    len is` vs no rephrasing). `5` is `nums.len()`, `10` is
    `bad_index`.
  - `note: run with \`RUST_BACKTRACE=1\` environment variable to
    display a backtrace` — matches the Book at line 456 *verbatim*
    and lesson 053's panic-trailer note line.
- Exit status `101` is the panic-in-`main` exit code lesson 053
  installed. No new exit-code fact today.
- The thread id `(132165324)` is the per-run OS thread identifier
  lesson 053 deferred. The lesson's transcript uses `(...)` to
  match.

This is the load-bearing positive probe for the lesson's two
co-installed claims:

1. *Bounds-checked-at-runtime witnessed:* `rustc demo.rs` succeeds
   and produces an executable; the failure happens at run time
   when `nums[bad_index]` evaluates with `bad_index = 10` and
   `nums.len() = 5`. Without this probe the lesson could not
   distinguish today's runtime case from lesson 077's
   compile-time auxiliary.
2. *Specific-message-shape witnessed:* the `index out of bounds:
   the len is 5 but the index is 10` line is exactly the Book's
   spec at line 455. The lesson teaches the *recognition* of this
   line — empirically grounded by Probe 1.

### Probe 2: in-bounds contrast — same shape, in-range index

Same temp dir family, separate file `demo.rs` with one character
changed:

```text
--- cat demo.rs ---
fn main() {
    let nums = [10, 20, 30, 40, 50];
    let bad_index_str = "2";
    let bad_index: usize = bad_index_str.parse().expect("not a number");
    let element = nums[bad_index];
    println!("element = {}", element);
}
--- rustc demo.rs ---
rustc-exit=0
--- ./demo (stdout) ---
element = 30
demo-exit=0
--- ./demo (stderr) ---
(empty)
```

Notes:

- Identical source structure to Probe 1; only `"10"` → `"2"` on
  line 3.
- `rustc demo.rs` exits 0 (silent), same as Probe 1.
- `./demo` prints `element = 30` on stdout and exits `0`. No
  panic. `nums[2]` is `30` per lesson 077's zero-based rule
  (index `2` is the third element of `[10, 20, 30, 40, 50]`).

This is the load-bearing positive contrast for the bounds-check
rule. The two probes share *every* structural feature except the
value of `bad_index`: same array, same parse chain, same indexing
expression, same `println!` line. The only difference is whether
`bad_index < nums.len()`. The bounds check is the only thing that
flipped — empirical witness that the rule is "compare `i` to
`a.len()`" and nothing else.

This satisfies the brief's "negative/contrast probe" requirement
for the lesson's contrastive claim ("with valid index runs cleanly,
with invalid index panics"). Probe 1 is the negative side (panic),
Probe 2 is the positive side (runs cleanly with same code shape).

### Probe 3: auxiliary — constant out-of-bounds (compile-time path)

Captured for evidence transparency only, to show the
runtime-vs-compile-time split the lesson discusses is empirically
grounded. Not committed.

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

- This is the *compile-time-rejected* case — the same code lesson
  077's auxiliary Probe 3 captured. `rustc` exits `1`, no
  executable produced.
- The diagnostic is uncoded (no `E####`). Lint name
  `unconditional_panic` named in the trailer, deny-by-default.
- The inline annotation `index out of bounds: the length is 5 but
  the index is 10` is *almost* — but not exactly — the runtime
  panic message. The runtime form (Probe 1) says "the **len** is
  5"; the compile-time form (this probe) says "the **length** is
  5". Same fact, slightly different wording. The lesson does not
  call attention to this micro-difference because today centers
  on the runtime case.
- Why this probe at all: the lesson's *What Changed* third bullet
  and *Check Yourself* (c) both reference the lesson-077-auxiliary
  contrast. Probe 3 here re-runs that auxiliary on the same host
  to confirm it still fires the same way on this rustc release,
  so the lesson's "lesson 077's `nums[10]` was rejected at
  compile time" claim is empirically corroborated *today* and
  not just inherited from 077's appendix.

### Negative / contrast probes summary

The brief calls for both a working probe and a contrast probe.
This lesson has two co-installed claims:

- *Bounds-checked-at-runtime + specific-message-shape:* witnessed
  by Probe 1 (out-of-bounds → panic with exact message) and
  Probe 2 (in-bounds → runs cleanly). The two probes form a
  matched pair on the same source structure, isolating the
  bounds-check as the only operationally relevant difference.
- *Runtime-vs-compile-time split:* witnessed by Probe 1 (runtime
  index → compile succeeds, runtime panic) versus Probe 3
  (constant index → compile rejected). The two probes share the
  array but differ in whether the index is rustc-const-evaluable.

### Reproducibility note

Probe 1 is deterministic on rustc 1.95.0 — no randomness, no
environment dependency. The thread id in parens varies per run;
the rest of the transcript is stable.

Probe 2 is deterministic.

Probe 3's headline (`error: this operation will panic at runtime`)
and inline annotation are deterministic on this release; both
match lesson 077's auxiliary Probe 3 verbatim.

The exact message lines are rustc-version-specific in wording but
stable in shape: the Book at line 455 specifies `index out of
bounds: the len is 5 but the index is 10` and Probe 1 produces
that exact line.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 078.

- **Lesson 053 (load-bearing for the panic concept)** — installed
  the *panic* vocabulary, the trailer shape (`thread 'main' ...
  panicked at file:line:col:`, `note: run with \`RUST_BACKTRACE=1\`
  ...`), the stdout-empty / stderr-diagnostic split, and the
  panic-in-`main` exit code `101`. Today reuses every one of those
  facts unchanged. The new piece today is *the cause* of the panic:
  not `.expect("msg")` on `Err`, but rustc's inserted bounds check
  on `a[i]`. The trailer wraps a different first line (`index out
  of bounds: ...` instead of `<msg>: <Err payload>`), but
  everything else — the location format, the note line, the exit
  code, the stderr routing — is exactly lesson 053. Lesson 053's
  *What To Ignore* explicitly named "the `RUST_BACKTRACE=1`
  environment variable" and "the thread id in parentheses" as
  deferred; today inherits both deferrals.

- **Lesson 056 (load-bearing for the runtime-built index)** —
  installed `&str.parse()` chained with `.expect(...)`, with the
  binding's `: TYPE` annotation driving inference. Today reuses
  this exactly: `bad_index_str.parse().expect("not a number")`
  produces a `usize` because `bad_index: usize`. No new fact about
  `.parse()` is installed; the chain is reused as a *means* to
  build a runtime index. The reason lesson 056 is the right pick
  (rather than, say, `io::stdin().read_line(...)` from lesson 054)
  is operational economy: today's claim is about the bounds rule,
  not about user input, so the smallest probe shape that produces
  a runtime `usize` is the right one. Lesson 056's *What To
  Ignore* deferred `.parse()` on other targets like `.parse::<u32>()`;
  today plugs `.parse()` into the `usize` slot, which is *also*
  one of the deferred targets — but the binding's `: usize`
  annotation handles the type-inference choice exactly the way
  lesson 056 set up the pattern.

- **Lesson 076 (load-bearing for the array literal and `.len()`)** —
  installed `[v1, v2, ...]` and the `.len()` method. Today reuses
  the literal `[10, 20, 30, 40, 50]` and references `nums.len()
  = 5` as the value the bounds check compares against. No new
  fact about array construction or `.len()` is installed.
  Lesson 076's *What To Ignore* explicitly named "*Out-of-bounds
  runtime panic* — `a[i]` with `i >= a.len()` panics with `index
  out of bounds: the length is N but the index is M`. Queue item
  E; needs D first." Today closes that deferral. (Slight wording
  drift in 076's prediction — "the length is N" — vs the actual
  runtime message "the len is N". The runtime form's wording
  matches the Book at line 455; 076's prose used the Reference's
  compile-time-form wording. Today's lesson uses the runtime
  form, which is the form witnessed by Probe 1 and the form the
  Book specifies.)

- **Lesson 077 (load-bearing for `a[i]` and `usize`)** —
  installed array element access and `usize` as the index type's
  centered name. Today extends 077 with the *bounds rule*. The
  compile-time-vs-runtime distinction the lesson centers comes
  directly from 077's auxiliary Probe 3: 077 saw the constant-
  index case fire `error: this operation will panic at runtime`;
  today shows the runtime-index case panics at run time with the
  matching message. Without 077's `a[i]` and `usize` slots
  installed, today's working probe could not be written.
  Lesson 077's *What To Ignore* explicitly named "*Out-of-bounds
  indexing* — queue item E ... With a constant index past the
  end, rustc on this release fires `error: this operation will
  panic at runtime`; with a runtime index past the end, the
  program panics with `index out of bounds: the length is N but
  the index is M`." Today closes the runtime half of that
  prediction. (Same wording-drift note as 076: 077's "the length
  is N" was the compile-time wording; the runtime wording is "the
  len is N". The lesson body uses the runtime form, witnessed by
  Probe 1.)

## Older supporting lessons

Mentioned by id only, not load-bearing for any individual claim
today:

- `001-rustc-compile-and-run` — `rustc file.rs` then `./name`;
  rustc silent on success. Used as the compile-and-run shape for
  all probes.
- `002-fn-main-entry-point` — body of `fn main` runs when the
  executable launches.
- `005-let-binding` — `let name = value;`. Today binds five names.
- `011-println-positional-args` — `println!("{}", expr)`. Reused
  unchanged; the line that *never runs* in Probe 1 and *runs* in
  Probe 2.
- `019-type-annotation-i32` — installed the `: TYPE` slot. Today
  plugs `usize` exactly as lesson 077 plugged it.
- `040-method-call-syntax`, `049-method-chaining` — receiver-and-
  method shape, chain receiver = call expression. Reused via the
  `.parse().expect(...)` chain inherited from lesson 056.
- `052-result-enum-and-is-ok` — `Result<T, E>` introduced.
  Reached transitively via `.parse()` returning a `Result`.
- `068-let-binding-scope`, `069-rustc-warnings`, `070-rustc-explain`,
  `071-macro-invocation-syntax`, `072-tuple-type-and-index`,
  `073-let-tuple-destructure`, `074-char-type`, `075-const-declaration`,
  `076-array-literal-and-type`, `077-array-indexing-and-usize` —
  recent lessons on the same host and toolchain. Mentioned only
  to confirm the host environment is unchanged.

No trait-related lesson is cited. Probe 3 mentions the
`unconditional_panic` *lint*, not a trait. Lesson 069 already
installed the warnings-vs-errors category but is not load-bearing
for today; the lesson body references the `unconditional_panic`
lint name only in *Check Yourself* (c) as a backreference to
lesson 077's auxiliary.

## Book Ch1-3 closure-pass effect

This lesson **closes item E** in
`experimental/eduratchet2/runs/rust-moves/book-ch1-3-coverage.md`.
Item E's listed prereqs were D (array indexing — lesson 077) and
053 (panic concept via expect). Today carries out exactly that
plan: the bounds rule + message shape co-install, with 077 + 053
+ 056 + 076 as the load-bearing prior lessons.

With out-of-bounds runtime panic installed, queue item **F**
(`for element in array` iteration) is unblocked from the
array side: 076 supplies the array, 077 the index, 022 the
for-range form, and today shows what would go wrong with a
manual `while index < a.len()` loop using the wrong bound (the
error 022 + iteration deliberately avoids). Queue item **G**
(full integer family) remains independently approachable. Queue
items **L** (`Cargo.lock`) and onward are unchanged by today.

The Book Ch1-3 array arc — C (literal/type/repeat-init) → D
(indexing + `usize`) → E (out-of-bounds runtime panic) — closes
with this lesson. Queue item F (for-iteration) is the natural
sibling-level next step on arrays, but no longer needs anything
from the array-arc itself.
