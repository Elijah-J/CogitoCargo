# Evidence — 059-continue-in-match-arm

Audit appendix for `lessons/059-continue-in-match-arm.md`. Holds the
corpus-quote map, toolchain string, working- and broken-contrast probe
transcripts, two calibration probes, and the prerequisite-claim
summary.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` -> `Darwin x86_64`
- Probes run in fresh `mktemp -d` directories, removed at the end of
  each run. Only the working `.rs` is committed (under
  `observations/059-continue-in-match-arm.rs`); the broken-contrast
  and calibration `.rs` files are not committed -- their transcripts
  below are the artifacts.

## Sources

### `output/docs/rust/reference/expressions/match-expr.md`

The Reference page for `match` expressions. Already cited in lessons
030, 031, 051, 058. Today's primary new corpus span is line 161 (the
divergence rule), which is the canonical license for the central new
mechanic: an arm whose body is a divergent expression is exempt from
the all-arms-share-type rule.

Line 138 (cycle 030's all-arms-share-type rule, re-cited):

> The type of the overall `match` expression is the [least upper
> bound] of the individual match arms.

Line 161 (today's load-bearing rule):

> If either the scrutinee expression or all of the match arms diverge,
> then the entire `match` expression also diverges.

Lesson 030 installed the half of this rule that says "arms agree on a
type." Today's lesson installs the *partial* form behind line 161:
when *some* (not all) arms diverge, those diverging arms join the
match without contributing to the type, and the match's type is the
least upper bound of the *remaining* (value-producing) arms. The
Reference's wording "all of the match arms diverge" is the *full*
divergence case (every arm escapes); today's lesson uses the more
common case where *one* arm escapes via `continue` and the rest
produce values. The mechanic underneath -- diverging expressions have
type `!` (the never type) and `!` coerces to any other type via the
"least upper bound" computation -- is the formal explanation; the
audience-level framing today is "control-flow keywords let an arm
escape without producing a value."

Calibration: line 142 ("If there are no match arms, then the `match`
expression is diverging and the type is `!`") and the empty-enum
example at lines 145-156 cover the *no-arms* divergence case, which
is unrelated to today's move and is not surfaced. The `!` typed name
is mentioned twice on the page but is explicitly deferred -- the
audience-level reading is sufficient for the move.

### `output/docs/rust/reference/expressions/loop-expr.md`

The Reference page for loop expressions. Already cited in lessons 017
(while), 022 (for), 027 (loop+break), 028 (break value), 035
(continue). Re-cited today for the divergence claim about
`continue`.

Lines 451-453 (the canonical formal definition of `continue`,
re-cited from cycle 035):

> When `continue` is encountered, the current iteration of the
> associated loop body is immediately terminated, returning control
> to the loop *head*.

Lines 455-457 (today's load-bearing claim that `continue` is
diverging):

> A `continue` expression is [diverging] and has a type of [`!`].

This is the formal license for the central new claim: `continue` is a
*diverging* expression. Combined with match-expr.md line 161 (above),
this gives the mechanism the lesson installs: putting `continue` as
an arm body lets the match's type be decided by the *other* arms.
The lesson's audience-level rephrase ("control-flow keywords let an
arm escape without producing a value") avoids both the `!` typed
name and the formal term "diverging" while keeping the substantive
content.

Line 473 (re-cited from cycle 035):

> A `continue` expression is only permitted in the body of a loop.

Today's working probe satisfies this -- the `match` is *inside* a
`for` loop, so the `continue` arm body is in the loop's body. The
lesson does not install a new bound on where `continue` is allowed;
cycle 035 already covered that.

Calibration: lines 469 ("`continue 'label`") cover labeled
`continue`, deferred since cycle 035. Lines 350 (`break` is
"diverging and has a type of `!`") and 524 (`loop` divergence) give
the parallel claim for `break`; the lesson names `break` and
`return` as siblings of `continue` in the divergent-arm exemption but
does not exercise them.

### `output/docs/rust/book/ch02-00-guessing-game-tutorial.md`

The Book guessing-game chapter. Already cited in lessons 042, 044,
050, 051, 052, 053, 054, 055, 056, 058. Reused today for the
canonical audience-level use of a `continue` arm body in a `match`
on a `Result` from `.parse()` -- the exact downstream composition
this lesson is grounding the type-rule for.

Lines 1171-1174 (re-cited from cycle 058):

> ```rust
> let guess: u32 = match guess.trim().parse() {
>     Ok(num) => num,
>     Err(_) => continue,
> };
> ```

Direct corpus precedent for a `match` arm whose body is `continue`.
The Book uses this shape inside a `loop`, with `let guess: u32 = ...`
binding the match's value to a `u32`. For this to type-check, the
`Err(_) => continue` arm must *not* be required to produce a `u32`;
the arm body `continue` is a diverging expression, so today's
divergent-arm rule grants the exemption. The Book chapter's audience
walk does not name the rule -- it simply uses it.

Lines 1212-1213 (the audience-level walkthrough):

> So, the program will execute the second arm's code, `continue`,
> which tells the program to go to the next iteration of the `loop`
> and ask for another guess.

The Book's plain-English statement of what the `continue` arm body
does at runtime (transfer control to the loop head). Pairs with the
Reference's "current iteration of the associated loop body is
immediately terminated" (loop-expr.md line 453, cited above) and is
the audience-level form the lesson reuses.

Calibration: the Book's example uses `Err(_) => continue` inside a
`loop` driving stdin reads. Today's lesson uses `_ => continue`
inside a `for` over an integer range, so the *pattern* shape is the
much simpler wildcard from cycle 031, the *scrutinee* is `n % 2`
rather than `guess.trim().parse()`, and the *binding's* type is
`i32` rather than `u32` -- but the load-bearing structural shape
(*`match` arm whose body is `continue`*) and the load-bearing rule
(*the diverging arm doesn't have to produce a value of the binding's
type*) are identical to the Book's shape. The full guessing-game
composition is explicitly deferred under *What To Ignore For Now*.

### `output/docs/rust/error_codes/E0308.md`

The error-code explainer for E0308 *mismatched types*. Already cited
in lessons 024, 025, 026, 028, 033, 045, 046, 047, 048, 052, 054,
055, 056, 057. Reused today for the broken-contrast probe.

Lines 4-8:

> Expected type did not match the received type. ... This error occurs
> when the compiler is unable to infer the concrete type of a
> variable. It can occur in several cases, the most common being a
> mismatch in the expected type that a function expects from its
> argument.

The lesson does not re-explain E0308; it cites the prior lessons by
number. The new structural fact for cycle 059 is that E0308 surfaces
in *two* shapes when an arm body is the wrong type:

1. *"`match` arms have incompatible types"* form -- when *two*
   value-producing arms disagree on type. The diagnostic spans both
   arms with bracketed context. This is the broken-contrast probe
   without a `continue` arm.
2. *"mismatched types"* form -- when there is *one* value-producing
   arm of the wrong type (the others diverge). The diagnostic points
   only at the offending arm; the diverging arms drop out of the
   type-check. This is the calibration probe at the end of *Check
   Yourself*.

Both shapes corroborate the central claim: the diverging arm is not
in the type-check. The broken-contrast probe captured below uses
form (1); the calibration probe uses form (2).

## Probes

### Working probe

Committed at
`experimental/eduratchet2/runs/rust-moves/observations/059-continue-in-match-arm.rs`.
Identical source to the *Try It* block.

Transcript, captured 2026-05-07 in a fresh `mktemp -d`:

```text
--- ls before ---
demo.rs
--- cat demo.rs ---
fn main() {
    let mut total: i32 = 0;
    for n in 1..=5 {
        let v: i32 = match n % 2 {
            0 => n,
            _ => continue,
        };
        total += v;
    }
    println!("total = {total}");
}
--- rustc demo.rs ---
rustc-exit=0
--- ls after ---
demo
demo.rs
--- ./demo ---
total = 6
demo-exit=0
--- temp dir removed ---
```

Notes (load-bearing observations):

- `rustc demo.rs` exits 0 silently. No warnings. The match with one
  value-producing arm (`0 => n`, an `i32`) and one diverging arm
  (`_ => continue`, no value) type-checks against `let v: i32 = ...;`
  cleanly. **This is the central load-bearing observation:** rustc
  does not require both arms to produce `i32`. If it did, the program
  would not compile (`continue` is not an `i32`).
- `./demo` prints exactly one line: `total = 6`. The empirical
  decomposition: `n` takes `1, 2, 3, 4, 5`. For odd `n` the `_`
  arm fires and `continue` diverges to the loop head, so `total +=
  v;` is not reached on those passes. For even `n` (`2`, `4`) the
  `0 =>` arm fires and `total += v;` runs, contributing `2 + 4 = 6`.
  The printed value confirms the decomposition.
- The annotation `: i32` on `v` is what selects the match's type as
  `i32` (cycle 030's all-arms-share-type rule, refined today). The
  `0 => n` arm produces `i32` (cycle 037 plus integer-literal
  default), and the `_ => continue` arm is exempt.
- No panic, no compile-time error. Exit 0.
- Only the working source is committed under `observations/`; the
  binary `demo` and the temp directory were removed.

### Broken-contrast probe (Shape A -- non-diverging non-`i32` arm)

Source (not committed -- the transcript below is the artifact). The
change from the working probe: the `_ => continue` arm is replaced
with `_ => "skip"` (a string literal, type `&str`). To make the
contrast surgical, the loop is dropped (so `continue` doesn't enter
the picture as a separate question) and `n` is set to a fixed value:

```rust
fn main() {
    let n = 3;
    let v: i32 = match n % 2 {
        0 => n,
        _ => "skip",
    };
    println!("{v}");
}
```

Captured 2026-05-07 in a fresh `mktemp -d` (filename `broken.rs`):

```text
--- cat broken.rs ---
fn main() {
    let n = 3;
    let v: i32 = match n % 2 {
        0 => n,
        _ => "skip",
    };
    println!("{v}");
}
--- rustc broken.rs (capturing stderr) ---
error[E0308]: `match` arms have incompatible types
 --> broken.rs:5:14
  |
3 |       let v: i32 = match n % 2 {
  |  __________________-
4 | |         0 => n,
  | |              - this is found to be of type `i32`
5 | |         _ => "skip",
  | |              ^^^^^^ expected `i32`, found `&str`
6 | |     };
  | |_____- `match` arms have incompatible types

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
rustc-exit=1
```

Notes (probe evidence -- not corpus quotation):

- The headline is `error[E0308]: \`match\` arms have incompatible
  types`. The same E-code as cycles 024-057, but a *new label*
  ("`match` arms have incompatible types") that names the problem
  *as* an arm-type disagreement. This is the load-bearing piece of
  probe evidence: rustc enforces cycle 030's all-arms-share-type rule
  for *value-producing* arms.
- The diagnostic spans both arms with bracketed context. The first
  arm `0 => n` is annotated `this is found to be of type \`i32\``;
  the second arm `_ => "skip"` is annotated `expected \`i32\`, found
  \`&str\``. Both arms produce values, so both arms are part of the
  type-check. *Neither* is exempt -- there is no diverging arm in
  this probe.
- This contrast establishes the *positive* version of the central
  rule: the all-arms-share-type rule still applies when arms produce
  values. Today's working probe is the *exemption* case (one arm
  diverges); this broken-contrast probe is the *no-exemption* case
  (both arms produce values). Together they corroborate "the
  exemption is specifically for diverging expressions."
- Exit code 1; no executable produced.

### Calibration probe (Shape B -- diverging arm + non-`i32` value arm)

Source (not committed -- transcript is the artifact). Used to verify
the *Check Yourself (c)* answer. The change from the working probe:
the value-producing arm `0 => n` is kept, the diverging arm `_ =>
continue` is kept, but the wildcard's body is now `"skip"` (a `&str`)
in a *different* arm position to compose with `continue`:

```rust
fn main() {
    let mut count: i32 = 0;
    for n in 1..=4 {
        let v: i32 = match n {
            1 => continue,
            _ => "skip",
        };
        count += v;
    }
    println!("count = {count}");
}
```

Captured 2026-05-07 in a fresh `mktemp -d`:

```text
--- cat broken.rs ---
fn main() {
    let mut count: i32 = 0;
    for n in 1..=4 {
        let v: i32 = match n {
            1 => continue,
            _ => "skip",
        };
        count += v;
    }
    println!("count = {count}");
}
--- rustc broken.rs ---
error[E0308]: mismatched types
 --> broken.rs:6:18
  |
6 |             _ => "skip",
  |                  ^^^^^^ expected `i32`, found `&str`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
rustc-exit=1
```

Notes (probe evidence -- not corpus quotation):

- The headline is `error[E0308]: mismatched types` -- the *simpler*
  form, *not* "`match` arms have incompatible types." Same E-code,
  different label.
- The diagnostic points at *only one* arm: `_ => "skip"`. The arm
  `1 => continue` is *not* mentioned in the diagnostic at all. This
  is *strong* empirical evidence for the central claim: the
  diverging arm has dropped out of the type-check entirely. There is
  only one value-producing arm (`_ => "skip"`), and that single arm
  is checked directly against the binding's `: i32` annotation --
  yielding the simpler "expected `i32`, found `&str`" form.
- Compare with the broken-contrast probe (Shape A) above:
  - Shape A: two value-producing arms that disagree -> *"match arms
    have incompatible types"* form, both arms in the diagnostic.
  - Shape B: one diverging arm + one value-producing arm of the
    wrong type -> *"mismatched types"* form, only the value arm in
    the diagnostic.
  Both confirm the rule, in different shapes of E0308.
- This probe is *load-bearing* for the *Check Yourself (c)* answer.
  The lesson body avoids overpromising the diagnostic shape; the
  appendix records the exact behavior.

### Calibration probe (Shape C -- `break` in arm body)

Source (not committed -- transcript is the artifact). Used to verify
the lesson's claim that `break` and `return` are siblings of
`continue` for the divergent-arm rule (named in *The Move* and
*Mental Model Delta* but not exercised in the working probe):

```rust
fn main() {
    let mut last: i32 = -1;
    for n in 1..=5 {
        let v: i32 = match n {
            10 => break,
            _ => n,
        };
        last = v;
    }
    println!("last = {last}");
}
```

The `10 => break` arm is unreachable for `n in 1..=5`, but its
*compilability* is what's load-bearing -- if `break` were not
exempt from the arms-share-type rule, the program would fail
type-check.

Captured 2026-05-07:

```text
--- cat breakvar.rs ---
fn main() {
    let mut last: i32 = -1;
    for n in 1..=5 {
        let v: i32 = match n {
            10 => break,
            _ => n,
        };
        last = v;
    }
    println!("last = {last}");
}
--- rustc breakvar.rs ---
rustc-exit=0
--- ./breakvar ---
last = 5
exit=0
```

Notes (probe evidence -- not corpus quotation):

- `rustc breakvar.rs` exits 0 silently. The match with one value
  arm (`_ => n`, `i32`) and one diverging `break` arm type-checks
  cleanly. This corroborates the lesson's claim that `break` is a
  sibling of `continue` for the divergent-arm exemption.
- `./breakvar` prints `last = 5`. The `10 => break` arm is never
  matched (because `n` never equals `10` in `1..=5`); the `_` arm
  always fires; `last` ends at `5`. The runtime behavior confirms
  the program is well-formed.
- This probe is calibration -- it does not change any lesson claim.
  It records that the divergent-arm rule does generalize to `break`
  as the lesson asserts. Loop-expr.md line 350 ("A `break`
  expression is diverging and has a type of `!`") is the
  corresponding corpus license; this probe is the empirical
  corroboration.

### Calibration probe (Shape D -- non-diverging unit-typed arm)

Source (not committed -- transcript is the artifact). Used to
sharpen the boundary of the divergent-arm rule. A `println!(...)`
call has unit type `()` (cycle 029's `()`). Unit is a *real* type, not
divergent, so it must agree with the binding's `i32` and should fail:

```rust
fn main() {
    let n = 3;
    let v: i32 = match n % 2 {
        0 => n,
        _ => println!("skip"),
    };
    println!("{v}");
}
```

Captured 2026-05-07:

```text
--- rustc unit.rs ---
error[E0308]: mismatched types
 --> unit.rs:5:14
  |
5 |         _ => println!("skip"),
  |              ^^^^^^^^^^^^^^^^ expected `i32`, found `()`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
rustc-exit=1
```

Notes (probe evidence):

- The headline is `error[E0308]: mismatched types` (same
  simpler-form variant as Shape B). The arm `_ => println!("skip")`
  produces a value of type `()`, not `i32`, and rustc reports the
  mismatch directly.
- Critically: replacing `continue` with a non-diverging body of a
  *different* type (here `()`) re-engages the share-type rule. This
  sharpens the lesson's boundary: the exemption is specifically for
  *diverging* expressions, not for "non-`i32` things" in general. A
  unit-typed arm body is *not* exempt; only `continue`/`break`/
  `return` (and other diverging expressions) are.
- This probe is calibration -- it backs the lesson body's framing
  ("the exemption is specifically for control-flow keywords that
  escape the match") and the *Check Yourself (c)* answer's parallel
  ("a non-diverging non-`i32` body would still fail").

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 059. Older supporting lessons are mentioned
above by number only.

- **Lesson 035 (load-bearing)** -- installed `continue;` as a
  loop-control statement that interrupts the current iteration and
  returns control to the loop head. Cycle 035's use was a top-level
  loop-body statement (`if cond { continue; }`). Today's cycle uses
  the *same* `continue` -- with the same runtime semantics -- in a
  *new place*: as the right-hand side of a `match` arm's `=>`. The
  `continue` is still inside the loop's body (the match is inside
  the loop), so loop-expr.md line 473's "only permitted in the body
  of a loop" constraint is satisfied. The new fact for 059 is that
  `continue` is a *diverging* expression (loop-expr.md line 457),
  which makes the type-system grant the arm-body exemption.
- **Lesson 030 (load-bearing)** -- installed the `match` machine
  (scrutinee, arms `pattern => arm_expression,`, matching arm wins,
  exhaustiveness, *all arms share a type*). Today *refines* the
  share-type rule: it applies only to *value-producing* arms;
  diverging arms (whose body is a control-flow expression like
  `continue`) are exempt. The Reference's match-expr.md line 138
  (the formal "least upper bound" formulation, re-cited above) plus
  line 161 (the divergence rule) together justify the refinement;
  the audience-level rephrase is "the type comes from the
  value-producing arms; the diverging arms join in for free."
- **Lesson 031 (load-bearing)** -- installed the `_` wildcard
  pattern at the top of a match arm. Used here in `_ => continue`.
  Cycle 031's exhaustiveness behavior (the wildcard makes the match
  exhaustive over an unbounded scrutinee like `n % 2`'s `i32`) is
  re-used unchanged.
- **Lesson 022 (load-bearing) + cycle 039** -- `for var in 0..=N {
  ... }` runs the body once per value, with `var` bound. The probe
  uses `for n in 1..=5` (cycle 022's `for` shape with cycle 039's
  inclusive `..=` form). This loop is what gives `continue`
  somewhere to return to.
- **Lesson 037** -- `n % 2` produces the integer remainder; for two
  positive operands, `n % 2` is `0` for even `n` and `1` for odd
  `n`. Used as the scrutinee in the working probe (`match n % 2`).
- **Lesson 023** -- `total += v;` is shorthand for `total = total +
  v;` (requires `mut` from cycle 006). Used in the loop body to
  accumulate.
- **Lessons 001, 002, 005, 006, 019** -- compile and run, `fn
  main`, `let`, `let mut`, `: i32` annotation. All used unchanged.

## Older supporting lessons

- Cycle 027 (`loop` and `break`) -- not a load-bearing prerequisite
  today (the probe uses `for`, not `loop`). Mentioned in *What To
  Ignore For Now* as the host for `break;` arm bodies (deferred).
- Cycle 028 (`break value;`) -- not used today; mentioned in *What
  To Ignore For Now* as the deferred shape `break value;` from a
  match arm.
- Cycle 058 (match payload variants `Ok(num)` / `Err(_)`) --
  *referenced* in the lesson body twice (in *What Changed* and in
  *Mental Model Delta* indirectly via the Book's guessing-game form),
  but **not** a load-bearing prerequisite. Today's match is on an
  integer scrutinee (`n % 2`), not on a `Result`. The divergent-arm
  rule applies to *any* match shape; cycle 058 is the natural
  composition target, not the load-bearing precondition. The
  orchestrator directive explicitly notes this.
- Cycle 029 (the unit type `()`) -- *referenced* in the calibration
  probe Shape D as the type of `println!(...)` arms. Not surfaced in
  the lesson body. The probe demonstrates that unit-typed arms are
  *not* exempt from the share-type rule; only diverging expressions
  are.
- Cycles 042-058 (Book guessing-game lift toward the canonical
  shape) -- the lesson's *What Changed* bullet about
  `Err(_) => continue` is the forward composition pointer. None of
  these are load-bearing for today's grounding.

## Calibration: minor surface choices not surfaced in the lesson body

- The probe uses `for n in 1..=5` rather than `0..N`. The inclusive
  range from cycle 039 makes the parity decomposition cleaner (`n in
  {1, 2, 3, 4, 5}` -> two evens contributing). With `0..N` the value
  `0` would be even and contribute, complicating the walk-through.
- The probe uses `n % 2` as the scrutinee rather than `n` directly.
  This compresses the parity check into the match's scrutinee
  position rather than spreading it across an `if`/`else` plus a
  match. The cycle-037 dependency is light (`n % 2` is a one-line
  use), and the `0 =>` / `_ =>` arms are the cleanest possible
  cycle-031 wildcard composition.
- The accumulator pattern (`let mut total: i32 = 0; ... total += v;
  ... println!("total = {total}");`) is identical to cycle 035's
  shape. This intentional parallel makes the "`continue` in this new
  place" framing legible -- only one piece of cycle 035's program is
  different.
- The probe binds the match's value to `let v: i32 = ...;` rather
  than using the match expression directly as a statement. The
  binding plus annotation makes the type-check the visible mechanic;
  using the match as a bare expression would obscure where the
  `i32` constraint comes from.
- The broken-contrast probe (Shape A) drops the loop and uses a
  fixed `let n = 3;` to keep the contrast surgical. With a loop in
  the picture, a non-diverging non-`i32` arm body would *still*
  fail to compile, but the diagnostic reads more clearly without
  the surrounding loop noise. The Shape B probe captures the
  loop-and-`continue` variant separately.
- The lesson does *not* install the never type `!`. The Reference
  uses `!` to formalize "diverging," and a complete account would
  install it; the audience-level framing "control-flow keywords let
  an arm escape without producing a value" is sufficient for the
  move and avoids opening type-theoretic machinery.
- The lesson does *not* install the `Err(_) => continue` shape
  itself; that is the natural next-cycle composition. Today
  installs only the *type-rule* that makes that shape legal.
