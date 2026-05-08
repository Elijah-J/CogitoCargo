# Evidence — 089-ch3-3-function-conventions

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

The lesson installs *two* Ch3-3 conventions as one coupled move:

1. *snake_case is Rust's conventional naming style for function and
   variable names* — Book Ch3-3 lines 9-10.
2. *Function definition order is free* — a function defined later
   in the source file is callable from one defined earlier — Book
   Ch3-3 lines 33-36.

The Book pairs these in one passage with one example, and one
working probe witnesses both. The bundling is principled because
the corpus itself bundles them. This lesson closes items R and S
of the Book Ch1-3 closure queue together.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes ran on `2026-05-07` from fresh `mktemp -d` directories
  removed at the end. Only the working `.rs` is committed at
  `experimental/eduratchet2/runs/rust-moves/observations/089-ch3-3-function-conventions.rs`.
  The contrast and auxiliary `.rs` files are *not* committed; the
  transcripts below are the artifacts.

Same host and toolchain as recent accepted lessons (082-088).

## Sources

### `output/docs/rust/book/ch03-03-how-functions-work.md`

Three load-bearing spans from the *Functions* section.

Lines 9-11 (the snake_case convention — verbatim):

> Rust code uses *snake case* as the conventional style for function
> and variable names, in which all letters are lowercase and
> underscores separate words. Here's a program that contains an
> example function definition:

Direct corpus warrant for:

- *snake_case as the conventional style*: lesson's *The Move*,
  *What Changed* bullet 1, *Mental Model Delta* "After" framing 1.
  The Book sentence covers *function and variable names* in one
  noun phrase, which is the corpus warrant for the lesson's claim
  that the same style applies to lesson-005 `let` bindings, not
  just function names.
- *all lowercase, underscores separate words*: lesson's *What
  Changed* bullet 1 (`my_count`, `read_line`) and *Check Yourself*
  question (a) (sorting `read_line` from `readLine`/`ReadLine`/
  `READ_LINE`).

Lines 15-25 (the canonical Book example — verbatim shape used by
the working probe):

> ```rust
> fn main() {
>     println!("Hello, world!");
>
>     another_function();
> }
>
> fn another_function() {
>     println!("Another function.");
> }
> ```

Direct corpus warrant for the lesson's *Try It* working probe.
The committed `.rs` file is this example *bit-for-bit* (with the
same blank line between the `println!` and the `another_function();`
call inside `main`, and the same blank line between the two `fn`
blocks). Only the filename differs — the Book uses `src/main.rs`
under `cargo run`, the lesson uses `demo.rs` under `rustc demo.rs`
per lesson 001 (calibration: behavior is the same).

Lines 31-36 (the free definition-order rule — verbatim):

> We can call any function we've defined by entering its name
> followed by a set of parentheses. Because `another_function` is
> defined in the program, it can be called from inside the `main`
> function. Note that we defined `another_function` *after* the
> `main` function in the source code; we could have defined it
> before as well. Rust doesn't care where you define your
> functions, only that they're defined somewhere in a scope that
> can be seen by the caller.

Direct corpus warrant for:

- *Definition order is free*: lesson's *The Move*, *Mental Model
  Delta* "After" framing 2, *What Changed* bullet 2 (the Book
  sentence quoted verbatim), *Check Yourself* question (b).
- *Lesson 008's deferral is closed*: lesson 008's *What To Ignore
  For Now* named "*Where the definition can sit.* The Book: 'Rust
  doesn't care where you define your functions, only that they're
  defined somewhere in a scope that can be seen by the caller.' We
  use one placement: definition *below* `main`. The general rule
  is not taught yet." Today closes that line by installing the
  general rule with the same Book quote.
- *"Scope that can be seen by the caller"*: lesson's *What To
  Ignore For Now* "`pub` and visibility" bullet — the rule is
  named, the unpacking is deferred.

Line 51-53 (a corroborating sentence on call-order, not centered
today):

> The lines execute in the order in which they appear in the `main`
> function. First the "Hello, world!" message prints, and then
> `another_function` is called and its message is printed.

Lesson 008 already installed source-order execution. Today reuses
the rule unchanged: the printed output ("Hello, world!" then
"Another function.") follows *call order in `main`*, not
*definition order in the file*. Cited only as the corroborating
walk in the lesson's *Try It*; not load-bearing for any new claim.

### `output/docs/rust/rustc/lints/listing/warn-by-default.md`

Lines 3470-3497 (the `non_snake_case` lint — load-bearing for the
contrast probe and for the *convention vs requirement* claim):

> ## non-snake-case
>
> The `non_snake_case` lint detects variables, methods, functions,
> lifetime parameters and modules that don't have snake case names.
>
> ### Example
>
> ```rust
> let MY_VALUE = 5;
> ```
>
> This will produce:
>
> ```text
> warning: variable `MY_VALUE` should have a snake case name
>  --> lint_example.rs:2:5
>   |
> 2 | let MY_VALUE = 5;
>   |     ^^^^^^^^ help: convert the identifier to snake case: `my_value`
>   |
>   = note: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default
> ```
>
> ### Explanation
>
> The preferred style for these identifiers is to use "snake case",
> where all the characters are in lowercase, with words separated
> with a single underscore, such as `my_value`.

Direct corpus warrant for:

- *snake_case is enforced as a warning, not an error*: lesson's
  *What Changed* bullet 3, *Mental Model Delta* "After"
  ("convention vs requirement"), *Check Yourself* answer (c). The
  contrast probe (Probe 2 below) reproduces this lint diagnostic
  in shape; the lint listing's headline (`warning: variable ...
  should have a snake case name`) and the `= note:`
  (``#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`)
  on by default``) match Probe 2's transcript bit-for-bit modulo
  the noun (`function` vs `variable`).
- *The same lint covers functions and variables*: lesson's *What
  Changed* bullet 4. The lint listing line 3472-3473 verbatim says
  "The `non_snake_case` lint detects *variables, methods,
  functions, lifetime parameters and modules*." The lesson's claim
  ("The same lint covers both") is this listing sentence narrowed
  to the two kinds of names installed today.
- *`help: convert the identifier to snake case`*: lesson's *Try
  It* contrast block, *Check Yourself* answer (c).

### Sources NOT cited as load-bearing

- `output/docs/rust/rustc/lints/groups.md` — names the
  `nonstandard_style` group containing `non-camel-case-types`,
  `non-snake-case`, `non-upper-case-globals`. Probe 2's `= note:`
  cites this group. The lesson does not unpack lint groups; cited
  in passing only.
- `output/docs/rust/book/ch03-01-variables-and-mutability.md` —
  earlier Book chapter where lesson 005 `let` was installed.
  Today's claim that variables also use snake_case is grounded by
  Ch3-3 line 9 directly ("function and variable names"); this
  earlier chapter is not separately quoted.
- `output/docs/rust/style/index.md` (Rust style guide) — corroborates
  the snake_case convention in finer detail. Today follows the Book's
  audience-level shape; the style guide's full rule list is not
  load-bearing.
- `output/docs/rust/reference/identifiers.md` — Reference's
  formal rule that identifiers may use any combination of
  ASCII/Unicode characters within the lexical grammar. snake_case
  is *style*, not lexical-grammar requirement; the corpus warrant
  for the latter is the Reference, but not load-bearing today
  (Probe 2 itself witnesses that CamelCase compiles).

## Probes

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/089-ch3-3-function-conventions.rs`)
is the *working* version. Two contrast probes are documented as
separate runs below, not committed as separate `.rs` files.

### Probe 1: working program (snake_case + after-`main` definition)

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
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
Hello, world!
Another function.
exit=0
--- temp dir removed ---
```

Notes:

- `rustc demo.rs` exits 0 and is silent (no warnings, no errors),
  consistent with lesson 001. Both Ch3-3 conventions are
  satisfied: `another_function` is snake_case, and the rule
  "callee must be defined before caller" is *not* a Rust rule.
- `./demo` prints exactly two lines, in the call order from inside
  `main`:
  1. `Hello, world!` (from the first `println!`).
  2. `Another function.` (from the body of `another_function`,
     which `main` calls *after* the first `println!`).
- Two load-bearing witnesses in this one transcript:
  - *snake_case witness*: `another_function` (a snake_case name)
    works as both definition (line 7 `fn another_function()`) and
    call site (line 4 `another_function();`). No diagnostic fires.
  - *Definition-order witness*: `another_function` is *defined on
    line 7-9 of the source*, AFTER `main` (lines 1-5). `main`
    calls it on line 4. The program compiles and runs; the printed
    output is in *call order*, not definition order. This is the
    operational shape of Book lines 33-36 verbatim.
- The committed `.rs` file's source matches the *Try It* code
  block exactly; the file also carries source-comment lines for
  the lesson's evidence pointer per the run convention.

### Probe 2: contrast — CamelCase function name (`non_snake_case` warning, NOT error)

Same temp-dir family, separate file `camel.rs`. The only
difference from Probe 1 is the function name: `AnotherFunction`
(CamelCase) instead of `another_function` (snake_case), at both
the definition site and the call site.

```text
--- cat camel.rs ---
fn main() {
    println!("Hello, world!");

    AnotherFunction();
}

fn AnotherFunction() {
    println!("Another function.");
}
--- rustc camel.rs ---
warning: function `AnotherFunction` should have a snake case name
 --> camel.rs:7:4
  |
7 | fn AnotherFunction() {
  |    ^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `another_function`
  |
  = note: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default

warning: 1 warning emitted

exit=0
--- ls after ---
camel
camel.rs
--- ./camel ---
Hello, world!
Another function.
exit=0
```

Read with lesson 069's category map:

- **Headline**: `warning: function \`AnotherFunction\` should have
  a snake case name`. The first word is `warning:`, not `error:`.
  This is the load-bearing diagnostic shape for the *convention
  vs requirement* claim — same shape lesson 069 installed for
  `unused_variables`, here applied to `non_snake_case`.
- **Location**: `camel.rs:7:4` — line 7, column 4, the function
  name on its `fn` line.
- **Source excerpt with caret**: `^^^^^^^^^^^^^^^` underlines
  `AnotherFunction` itself; the inline `help:` text reads
  `help: convert the identifier to snake case: \`another_function\``.
  rustc itself encodes the conventional fix.
- **`= note:` block**: ``#[warn(non_snake_case)]` (part of
  `#[warn(nonstandard_style)]`) on by default``. Names the lint
  (`non_snake_case`), its containing group (`nonstandard_style`),
  and the on-by-default level. The lesson does not unpack lint
  configuration; lesson 069's category installation is enough to
  read the line.
- **Trailer**: `warning: 1 warning emitted`. *Not* the
  `error: aborting due to N previous error(s)` form. rustc told
  you "no abort."
- **Exit code**: 0. Executable `camel` *is* produced. `./camel`
  runs and prints the same two lines as Probe 1's `./demo`.

This is the load-bearing negative probe for two centered claims:

1. *Convention vs requirement.* Probe 2's `warning:` headline
   together with `exit=0` and the produced executable witness that
   snake_case is enforced as a *warning*, not an error. The code
   *runs* despite violating the convention.
2. *The same `non_snake_case` lint covers function names.* The
   rustc lint listing's example uses a *variable* (`MY_VALUE`);
   Probe 2 reproduces the same lint shape on a *function name*,
   confirming the listing's claim that the lint detects "variables,
   methods, functions, lifetime parameters and modules."

### Probe 3: auxiliary — CamelCase variable name (`non_snake_case` covers `let` too)

Captured for evidence transparency. Not centered in the lesson
body. Documented to ground the lesson's claim that `let` bindings
follow the same naming style as `fn` names.

```text
--- cat camel_var.rs ---
fn main() {
    let MyCount = 5;
    println!("MyCount = {}", MyCount);
}
--- rustc camel_var.rs ---
warning: variable `MyCount` should have a snake case name
 --> camel_var.rs:2:9
  |
2 |     let MyCount = 5;
  |         ^^^^^^^ help: convert the identifier to snake case: `my_count`
  |
  = note: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default

warning: 1 warning emitted

exit=0
```

Notes:

- Same `non_snake_case` lint as Probe 2; only the noun in the
  headline changes (`function` → `variable`) and the caret moves
  from the `fn`-name slot to the `let`-name slot.
- The `= note:` line is *bit-for-bit identical* to Probe 2's:
  ``#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`)
  on by default``. This is the operational evidence that one lint
  covers both kinds of names — the rustc lint listing claim
  ("variables, methods, functions, lifetime parameters and
  modules") narrowed to the two kinds of names installed today.
- This probe corroborates the lesson's *What Changed* bullet 4
  ("The `let` bindings from lesson 005 follow the same naming
  style as `fn` names") and the Book's wording at line 9-10
  ("function and variable names").

### Negative / contrast probes — rationale

The lesson makes two contrastive claims:

1. *snake_case is a convention, not a hard rule.* The negative
   witness needed is "code that violates snake_case still
   compiles" — Probe 2 supplies this with `exit=0` and a produced
   executable, and the auxiliary Probe 3 corroborates on
   variables.
2. *Definition order is free.* The negative witness needed would
   be "rejected if and only if the function is missing." But that
   is exactly lesson 008's contrast probe (delete the `fn` block
   → `E0425`). Lesson 008 already supplied this; today's working
   probe (function defined *after* `main`, not *before*) is
   itself the positive witness for the order claim. No new
   negative probe is needed for the order rule on this host:
   reversing definition order yields a working program either way.

The lesson does not run a probe for the *cross-module* case
("definition order across `mod` boundaries"). The rule is named
in *What To Ignore For Now* and is its own future move. The
lesson's order-claim is restricted to the one-file case.

### Reproducibility note

Probe 1's printed output (`Hello, world!` then `Another function.`)
is deterministic on rustc 1.95.0 and on every recent release; the
program has no randomness or environment dependency.

Probe 2's headline (`warning: function \`AnotherFunction\` should
have a snake case name`), the inline `help:` text, and the
`= note:` line are deterministic on this rustc release. The
*shape* of the diagnostic — `warning:` category, on-by-default
`non_snake_case` lint pointer, source-diff `help:` — is grounded
in lesson 069's category map and matches the rustc lint listing
example bit-for-bit modulo the kind-of-name noun. Stable across
recent releases; the exact wording is rustc-version-specific.

Probe 3's transcript shape is identical to Probe 2's modulo
`function` → `variable` and the source-line content. Same
release-stability caveat.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 089.

- **Lesson 008 (load-bearing for `fn name() { ... }` + `name();`
  and the deferral of definition order)** — installs the
  define-and-call shape. Lesson 008's *What To Ignore For Now*
  named "Where the definition can sit ... We use one placement:
  definition *below* `main`. The general rule is not taught yet."
  with the same Book quote ("Rust doesn't care where you define
  your functions, only that they're defined somewhere in a scope
  that can be seen by the caller"). Today closes that deferral
  with the verbatim same Book span as source. Today *also*
  extends 008's `name` slot with the snake_case rule (008 used
  `say_hi`, which is already snake_case, but did not name the
  *convention*). The Probe 1 source is structurally similar to
  008's working source — two `fn` blocks, one calling the other,
  definition *below* `main` — confirming continuity with 008's
  setup.

## Older supporting lessons

Mentioned by id only, not load-bearing for any individual claim
today.

- `001-rustc-compile-and-run` — `rustc file.rs` then `./name`;
  rustc silent on success on Probe 1.
- `002-fn-main-entry-point` — body of `fn main` runs when the
  executable launches. Cited; not extended.
- `003-read-rustc-diagnostic` — four-part diagnostic map. Probes
  2 and 3 are read with this map; no new diagnostic vocabulary.
- `005-let-binding` — `let name = value;`. Cited for the claim
  that variables follow snake_case too; corroborated by Probe 3.
- `011-println-positional-args` — positional `{}` printing.
  Reused twice in Probe 1, once in Probe 3.
- `069-rustc-warnings` (cited for the warning category) —
  installs warning-vs-error as separate categories with separate
  abort behavior. Probe 2's `warning:` headline + `exit=0` +
  produced executable is read with 069's category map.
- `075-const-declaration` (cited for the SCREAMING_SNAKE_CASE
  contrast) — installed a different naming convention for `const`
  names. Today's *Check Yourself* answer (a) sorts `READ_LINE`
  out as belonging to 075's territory.
- `082-cargo-build-release`, `083-integer-overflow`,
  `084-cargo-check`, `085-toolchain-housekeeping`,
  `086-rustup-doc`, `087-rustfmt`, `088-f32-floating-point` —
  most recent accepted lessons on the same host and toolchain.
  Mentioned only to confirm the host environment is unchanged.

No trait-related lesson is cited.

## Book Ch1-3 closure-pass effect

This lesson **closes items R and S together** in the Book Ch1-3
closure queue:

- *Item R*: snake_case as the conventional naming style for
  function and variable names. The queue note for R explicitly
  said: "minor; could fold into another lesson's prose if too
  small to justify its own cycle. Worker discretion." Today folds
  it into a coupled Ch3-3 conventions lesson.
- *Item S*: function definition-order independence. Lesson 008's
  deferral is closed by the same lesson; both are taught from the
  same Book passage.

The bundling is principled because the Book itself bundles them
(lines 9-36 are one continuous passage with one example), and the
natural working probe (`another_function` defined after `main` and
called from inside `main`) demonstrates both at once. The single
centered concept "Ch3-3 function conventions" is the Book's own
unit of instruction here.

With items R and S installed together, future Ch3-3 moves
(parameters, multiple parameters — already at lesson 020 / 036 —
return values at lesson 021, statement vs expression at lesson
024) all build on the convention that defined functions can be
called in any source-file position from inside other defined
functions. The `non_snake_case` lint and `non_upper_case_globals`
sibling are both installed as *named* infrastructure for future
lint-configuration moves; the centered claim today is the
audience-level convention, not the lint internals.
