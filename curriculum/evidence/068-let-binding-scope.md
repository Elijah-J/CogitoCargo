# Evidence — 068-let-binding-scope

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Sources

### `output/docs/rust/reference/names/scopes.md`

Two load-bearing direct quotes from the Rust Reference:

- Lines 6-8 (introduction): "A *scope* is the region of source text
  where a named entity may be referenced with that name." Grounds
  the lesson's definition of *scope* as "the region of code where
  a name has meaning."
- Line 54 (`names.scopes.pattern-bindings.let`): "`let` statement
  bindings range from just after the `let` statement until the end
  of the block where it is declared." Grounds the lesson's specific
  claim about a `let` binding's region — quoted verbatim in the
  lesson body.

The same chapter explicitly enumerates separate scope rules for
function parameters (line 58), closure parameters (line 62),
`for` loop bindings (line 66), `if let`/`while let` (line 70),
and `match` arm bindings (line 74). The lesson's "What To Ignore
For Now" defers each of these by name; the Reference's separate
treatment is the corpus warrant for treating them as distinct
later moves.

### `output/docs/rust/book/ch03-01-variables-and-mutability.md`

Lines 179-193 contain the Book's example of an inner `{ ... }`
block introducing a fresh `let x = x * 2;` and the explicit
sentence on line 193: "When that scope is over, the inner shadowing
ends and `x` returns to being `6`." This is the corpus existence
proof, in The Book, that `{ ... }` blocks bound a binding's scope.
The Book uses this example to teach shadowing; the lesson uses the
same scope-bounding rule without unpacking shadowing (deferred to
lesson 007 / 057).

### `output/docs/rust/error_codes/E0425.md`

Direct quote (line 4): "An unresolved name was used." The page's
fix example (lines 56-60) is a `let unknown_variable = 12u32;`
followed by `let x = unknown_variable;`. The contrast probe in
this lesson reproduces the *same* error code on a name whose `let`
exists but is in a different block — a calibration on top of
lesson 005, which exercised the simpler "no `let` at all" case.

### `output/docs/rust/rust-by-example/variable_bindings/scope.md`

Direct quote (lines 4-5): "Variable bindings have a scope, and are
constrained to live in a *block*. A block is a collection of
statements enclosed by braces `{}`." This is a third corpus source
(rust-by-example) for the same rule as the Reference and the Book,
in plainer language matching the lesson's audience. The page's
example (lines 8-26) shows a `short_lived_binding` declared inside
an inner `{ ... }` block and used after the block ends, with the
comment "Error! `short_lived_binding` doesn't exist in this scope"
— the same structural pattern this lesson's contrast probe uses.

## Probe

The committed observation file
(`experimental/eduratchet2/runs/rust-moves/observations/068-let-binding-scope.rs`)
is the *working* version. The contrast version is documented as a
second run in this transcript only; no broken `.rs` is committed.

Both runs were executed in the same temp directory created with
`mktemp -d` and removed at the end.

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64

=== RUN 1: working program — let inside if, used inside if ===
--- ls before compile ---
demo.rs
--- cat demo.rs ---
fn main() {
    let n = 7;
    if n > 5 {
        let label = "big";
        println!("n = {n}, label = {label}");
    }
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
n = 7, label = big
exit=0

=== RUN 2: contrast — println! moved after the if block ===
--- cat demo.rs ---
fn main() {
    let n = 7;
    if n > 5 {
        let label = "big";
    }
    println!("n = {n}, label = {label}");
}
--- rustc demo.rs (capturing stderr) ---
error[E0425]: cannot find value `label` in this scope
 --> demo.rs:6:33
  |
6 |     println!("n = {n}, label = {label}");
  |                                 ^^^^^
  |
help: the binding `label` is available in a different scope in the same function
 --> demo.rs:4:13
  |
4 |         let label = "big";
  |             ^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
exit=1
--- ls after broken compile ---
demo
demo.rs
```

### Notes from the transcript

- Run 1 (working): `rustc` exits 0 silently; `./demo` prints
  `n = 7, label = big`. Both `n` (bound in the outer block) and
  `label` (bound in the inner block) are in scope at the use site
  inside the inner block.
- Run 2 (contrast): `rustc` exits 1 with `error[E0425]`. The
  load-bearing observations are:
  - The error fires *only* on `label`, not on `n`. Both names
    appear in the same broken `println!` format string, but the
    caret sits under the five characters of `label` inside
    `{label}` at column 33, while `n` inside `{n}` (column 19)
    triggers no error. This is the lesson's asymmetry claim,
    captured empirically.
  - The headline contains the exact phrase "in this scope". The
    word "scope" in this run's E0425 message names the same
    region the Reference defines on line 8.
  - rustc's `help:` block adds a *secondary* `-->` pointing back
    at line 4, the original `let label = "big";`, with the label
    "available in a different scope in the same function". This
    is rustc's own framing — it confirms that the binding exists
    *and* that scope is the variable: the binding is in one scope,
    the use site is in another. (Lesson 005's contrast — no `let`
    at all — got a simpler diagnostic with no such `help:` block;
    this one is calibration evidence that rustc distinguishes the
    two cases.)
- The `demo` shown in `ls after broken compile` is the executable
  produced by Run 1; Run 2 produced no new executable (consistent
  with lesson 001's two-step picture and lesson 005's same
  observation under E0425).
- Only the working `.rs` is committed under `observations/`; the
  broken variant exists only inside this transcript. The temp dir
  was removed; `git status` confirms only the working `.rs` was
  added.

### Compile-time vs runtime calibration (no separate probe)

The lesson asserts that scope is decided at compile time, not by
which blocks happen to run. This is implicit in the Run 2
transcript: `rustc` rejected the program before any execution.
A separate probe with `if false` would also fail to compile for the
same E0425 reason; the empirical claim is already covered by Run 2,
because at compile time `rustc` does not yet know whether the
condition will be `true` or `false`. No additional probe captured.

## Prior lessons

Direct prerequisites (load-bearing claims):

- `005-let-binding` (accepted, load-bearing) — installs `let name =
  value;` and the E0425 contrast for "no binding at all". This
  lesson narrows lesson 005's "later statements in the same body
  can use the bound name" claim to "later statements inside the
  same enclosing `{ ... }` block." Lesson 005 explicitly deferred
  *scope* under What To Ignore For Now; this cycle resolves that
  deferral.
- `014-if-else` (accepted) — installs `if condition { ... }` and
  the inner `{ ... }` block. Used here as the smallest available
  inner block inside `fn main`. Whether the block actually runs
  at runtime is irrelevant to scope; the lesson states this and
  the appendix's Run 2 calibration carries it.
- `003-read-rustc-diagnostic` (accepted) — installs the
  headline / location / source-excerpt-with-caret / help map. The
  lesson reads Run 2's E0425 with that map and does not re-teach.

Older supporting lessons (mentioned by id only):

- `001-rustc-compile-and-run` — `rustc file.rs` workflow.
- `002-fn-main-entry-point` — `fn main() { ... }` is the outer
  block.
- `007-shadowing` — referenced in What To Ignore For Now.
- `020-function-with-parameter` — referenced in What To Ignore
  For Now (parameter scope).
- `024-statement-vs-expression`, `026-if-as-expression` —
  referenced in What To Ignore For Now (block-as-value).
- `030-match-on-bool`, `031-match-integer-and-wildcard`,
  `057-type-changing-shadowing`, `058-match-result-payload-variants`
  — referenced in What To Ignore For Now.
