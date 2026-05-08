# Evidence — 041-qualified-method-call

Audit appendix for `lessons/041-qualified-method-call.md`. Holds the
corpus-quote map, the toolchain string, the full working and broken-
contrast probe transcripts, and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/041-qualified-method-call.rs`); the broken-contrast
  `.rs` is not committed — its transcript below is the artifact.

## Sources

### `output/docs/rust/reference/expressions/call-expr.md`

The Reference page for call expressions, *Disambiguating function
calls*. Primary corpus source for this lesson. Three load-bearing
spans.

Line 51:

> All function calls are sugar for a more explicit [fully-qualified
> syntax](../paths.md#qualified-paths).

This is the canonical statement that the dot-form (lesson 040) and
the qualified form (this lesson) both desugar to a common shape. The
lesson re-states this less formally as "two method-call shapes that
reach the same method."

Lines 75-114 (the *Disambiguating function calls* example):

> ```rust
> trait Pretty {
>     fn print(&self);
> }
> ...
> fn main() {
>     let f = Foo;
>     ...
>     // we can do this because we only have one item called `print` for `Foo`s
>     f.print();
>     // more explicit, and, in the case of `Foo`, not necessary
>     Foo::print(&f);
>     // if you're not into the whole brevity thing
>     <Foo as Pretty>::print(&f);
>     ...
> }
> ```

The `f.print();` / `Foo::print(&f);` pair is the exact pattern this
lesson installs (with `i32::abs(n)` instead of `Foo::print(&f)`). The
Reference labels the qualified form as "more explicit, and, in the
case of `Foo`, not necessary" — i.e. the dot-form is sugar for the
qualified form, which directly grounds the lesson's *equivalence*
claim. The `<Foo as Pretty>::print(&f);` form on the third line is
the trait-disambiguation form that this lesson explicitly defers in
*What To Ignore For Now*.

Calibration: the Reference's example uses `&f` rather than `f` because
`Pretty::print` takes `&self`. `i32::abs` takes `self` by value (per
`output/docs/rust/std/primitive.i32.md` line 2511, quoted below), so
the lesson uses the bare receiver `n` instead of `&n`. The lesson
explicitly defers the receiver-by-reference case to "Method-resolution
autoref/autoderef."

### `output/docs/rust/reference/paths.md`

The Reference page for paths. Cited for the grammar of the qualified
call form. Two load-bearing spans.

Line 8:

> A *path* is a sequence of one or more path segments separated by
> `::` tokens.

Grounds the lesson's "`::` is the path separator" label.

Lines 56-62 (*Paths in expressions* grammar):

> [PathInExpression](paths.md#railroad-PathInExpression) →
>     ::? [PathExprSegment](paths.md#grammar-PathExprSegment) ( ::
>     [PathExprSegment](paths.md#grammar-PathExprSegment) )*
>
> [PathExprSegment](paths.md#railroad-PathExprSegment) →
>     [PathIdentSegment](paths.md#grammar-PathIdentSegment) ( ::
>     [GenericArgs](paths.md#grammar-GenericArgs) )?

This is the formal grammar that licenses `i32::abs` as a path-in-
expression with two segments. The lesson does not reproduce the
grammar — only the spoken-English shape "type, then `::`, then method
name."

### `output/docs/rust/book/ch05-03-method-syntax.md`

The Book chapter on method syntax. Already cited in lesson 040 for
the dot-form; reused here for the qualified-form-on-a-primitive
pattern. Two load-bearing spans.

Lines 161-166 (the `Point::distance` example body):

> ```rust
> impl Point {
>    fn distance(&self, other: &Point) -> f64 {
>        let x_squared = f64::powi(other.x - self.x, 2);
>        let y_squared = f64::powi(other.y - self.y, 2);
>
>        f64::sqrt(x_squared + y_squared)
>    }
> }
> ```

Direct corpus precedent for the qualified-form-on-a-primitive pattern:
`f64::powi(other.x - self.x, 2)` and `f64::sqrt(...)` are exactly the
shape `Type::method(receiver, args)` on a primitive type. The lesson
mirrors this pattern with `i32::abs(n)`. The Book uses the qualified
form here without commentary — confirming that the form is in everyday
use on primitive types, not just on user-defined structs.

Lines 327-329 (*Associated Functions* section):

> To call this associated function, we use the `::` syntax with the
> struct name; `let sq = Rectangle::square(3);` is an example. This
> function is namespaced by the struct: The `::` syntax is used for
> both associated functions and namespaces created by modules.

Cited for the prose statement that the `::` syntax names a function
attached to a type. The lesson uses "the type the method is associated
with" / "names the type explicitly" framing, which matches the Book's
"namespaced by the struct" framing. The lesson explicitly defers the
no-`self` *associated function* sub-case (`Rectangle::square(3)` here,
or `String::new()` later) to *What To Ignore For Now*; today's lesson
installs only the receiver-bearing case.

Calibration: the Book chapter never uses the term *qualified* —
"qualified" is the Reference's terminology
(`output/docs/rust/reference/paths.md` lines 144-181, *Qualified
paths*). The lesson uses the Reference's term. The Book's own term in
the disambiguation context is "fully-qualified syntax" (linked from
`call-expr.md` line 51). For audience-level prose the lesson uses just
"qualified form."

### `output/docs/rust/std/primitive.i32.md`

The std-library page for the `i32` primitive type. Cited only for the
canonical `abs` signature (already used in lesson 040).

Line 2511:

> #### pub const fn abs(self) -> i32

This is the canonical signature for `i32::abs`: takes `self` (the
receiver) as its single parameter, returns `i32`. Two claims in the
lesson rest on this:

1. The qualified form with one argument is correct: `i32::abs(n)`
   supplies the single `self` parameter as a positional argument.
2. The qualified form with zero arguments is incorrect: `i32::abs()`
   supplies zero arguments to a function that takes one, which is
   exactly the trigger condition for E0061 per the explainer page.

### `output/docs/rust/error_codes/E0061.md`

The error-code explainer for E0061, "an invalid number of arguments
was passed when calling a function." Already cited in lesson 036.
Reused here for the broken-contrast probe. Three load-bearing spans.

Lines 4-15:

> An invalid number of arguments was passed when calling a function.
>
> Erroneous code example:
>
> ```rust
> fn f(u: i32) {}
>
> f(); // error!
> ```

The corpus's own E0061 example — `fn f(u: i32) {}` then `f();` — is
*structurally identical* to this lesson's broken-contrast probe: a
function with one parameter, called with zero arguments. The lesson's
probe just substitutes the qualified form `i32::abs()` for the free-
function form `f()` and exercises the same one-arg/zero-supplied
condition.

Lines 17-19:

> The number of arguments passed to a function must match the number
> of arguments specified in the function signature.

Plain-English statement of the rule the broken-contrast probe
violates. The lesson does not re-explain this — lesson 036 already
installed it.

Lines 31-33:

> Note that Rust does not have a notion of optional function arguments
> or variadic functions (except for its C-FFI).

This is the corpus statement that `i32::abs()` cannot be parsed as
"call with the receiver defaulted/elided." The receiver is mandatory
in the qualified form because Rust has no defaulting mechanism that
would let it be omitted. The lesson states this as "the receiver is
mandatory; the dot-form just hides that fact."

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/041-qualified-method-call.rs`.
Identical source to the Try It block.

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
    let n: i32 = -7;
    let dot: i32 = n.abs();
    let qual: i32 = i32::abs(n);
    println!("dot = {dot}, qualified = {qual}");
}
--- rustc demo.rs ---
exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
dot = 7, qualified = 7
exit=0
```

Notes:

- `rustc demo.rs` exits 0 and is silent on success (lesson 001).
- `./demo` prints exactly one line: `dot = 7, qualified = 7`. Both
  values are `7`, confirming the *equivalence* claim of the lesson:
  the dot-form `n.abs()` and the qualified form `i32::abs(n)` produce
  the same value for the same receiver. This is the working-side
  corroboration of the Reference's "all function calls are sugar for
  a more explicit fully-qualified syntax" statement.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe

Source: same shape as the working probe with line 4 changed from
`let qual: i32 = i32::abs(n);` to `let qual: i32 = i32::abs();`
(qualified form with zero arguments). Not committed; the transcript
below is the artifact. Captured 2026-05-07 in a fresh `mktemp -d`
(filename `broken.rs`):

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- ls before ---
broken.rs
--- cat broken.rs ---
fn main() {
    let n: i32 = -7;
    let qual: i32 = i32::abs();
    println!("qual = {qual}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0061]: this function takes 1 argument but 0 arguments were supplied
 --> broken.rs:3:21
  |
3 |     let qual: i32 = i32::abs();
  |                     ^^^^^^^^-- argument #1 of type `i32` is missing
  |
note: method defined here
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/num/int_macros.rs:3604:21
 --> /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/num/mod.rs:394:4
 ::: /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/num/mod.rs:413:5
  |
  = note: in this macro invocation
  = note: this error originates in the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: provide the argument
  |
3 |     let qual: i32 = i32::abs(/* i32 */);
  |                              +++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0061`.
exit=1
```

Notes (probe evidence — not corpus quotation):

- The headline reads `error[E0061]: this function takes 1 argument
  but 0 arguments were supplied`. Same E-code lesson 036 installed,
  same template "this function takes N arguments but M arguments were
  supplied" — the headline shape carries over from lesson 036 with N=1
  and M=0.
- The diagnostic has the four lesson-003 parts: headline + `-->`
  location (`broken.rs:3:21`) + source excerpt with caret + `help:`
  block, plus the `--explain E0061` trailer that lesson 003 named.
- The caret `^^^^^^^^--` is two-part: `^^^^^^^^` underlines
  `i32::abs`, then `--` underlines the empty `()` and labels it
  `argument #1 of type `i32` is missing`. This is rustc's pinpoint of
  the missing receiver slot. Probe evidence for the lesson's "the
  caret highlights the missing slot inside the parens" claim.
- The `note: method defined here` line is the lesson-036 dual-`-->`
  pattern (the secondary `-->` block points at the std-library
  definition site rather than user code). One small fidelity note:
  lesson 036's broken probe showed `note: function defined here` for
  a free function; here rustc emits `note: method defined here`
  because `i32::abs` is a method. The structural pattern is the same;
  the noun differs to match the kind of item being defined. The
  lesson body uses "method defined here" verbatim from this probe.
- The std-library definition lives behind a `int_impl!` macro
  expansion, which is why the secondary `-->` block carries
  `:::` cross-reference notes naming the macro. The lesson does not
  expose this detail; macros are deferred. The note "this error
  originates in the macro `int_impl`" is captured in this transcript
  for completeness but the lesson body elides it (rendered as `...`
  in the truncated quote in *Try It*).
- The `help: provide the argument` block suggests
  `i32::abs(/* i32 */)` with a `+++++++++` source-diff insertion. The
  literal placeholder `/* i32 */` is rustc's rendering of "an
  expression of type `i32` goes here" — this is a comment-style
  marker, not legal source code, and the learner is expected to
  replace it with a real expression like `n`.
- Exit code: 1. No executable was produced. The `ls after` shows only
  `broken.rs`, no `broken` binary.

The broken-contrast probe is necessary because the lesson makes a
contrastive claim — equivalence on the working side
(`n.abs()` == `i32::abs(n)`), receiver-mandatory on the broken side
(`i32::abs()` fails). The captured `note: method defined here` and
`help: provide the argument` text are rustc's own runtime statement of
the rule; they ground the lesson's body claims to a captured
transcript rather than only to the abstract E0061 explainer page. The
corpus-level grounding for E0061 itself is
`output/docs/rust/error_codes/E0061.md` (already cited above and in
lesson 036); this probe is the live transcript that ties that
explainer to this specific qualified-form-on-a-method instance.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 041. Older supporting lessons are mentioned
above by number only.

- **Lesson 040 (load-bearing)** — installed (a) the dot-form
  `value.method(args)`, (b) the concrete method `i32::abs` returning
  the absolute value as an `i32`, and (c) the framing that methods
  are functions associated with a type. Lesson 041 is built directly
  on this: the qualified form is the *other* call shape for the same
  method. The working probe re-runs lesson 040's dot-form on the same
  line as the new qualified form to make the equivalence visible.
- **Lesson 036 (load-bearing)** — installed E0061 ("this function
  takes N arguments but M arguments were supplied") with the dual-
  `-->` pattern (`note: function defined here` cross-referencing the
  definition). Lesson 041's broken-contrast probe reuses that
  diagnostic shape unchanged, with N=1 and M=0, and the `defined
  here` noun shifts from `function` to `method` because `i32::abs` is
  a method. No new diagnostic mechanism is introduced.
- **Lesson 003 (load-bearing)** — diagnostics have headline + `-->`
  location + source excerpt with caret + optional `note:`/`help:`
  lines. The broken-contrast walk uses that map without re-teaching
  it.
- **Lessons 020, 021** — typed parameters and return types. The
  qualified form is just a function call against a parameter list, so
  the same rules apply: `i32::abs(n)` supplies one `i32` argument and
  produces one `i32` return value, which fits on the right of `let m:
  i32 = ...;`.
- **Lessons 001, 002, 005, 019** — `rustc file.rs` then `./name`;
  `fn main` is the entry point; `let name: TYPE = value;` annotated
  bindings; `i32` is the default integer type. Used unchanged.

## Older supporting lessons

Lesson 008 (free-function call form `name(args)` — invoked indirectly
via lesson 040's contrast and lesson 036's installation; not re-stated
here). All other supporting lessons are reachable through the direct
prerequisites listed above.
