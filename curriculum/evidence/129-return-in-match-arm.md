# Evidence — 129-return-in-match-arm

Audit appendix for `lessons/129-return-in-match-arm.md`. Holds
the corpus-quote map, the toolchain string, the working-probe
transcript, the centered (E0004) and secondary (unreachable
pattern) contrast transcripts, the corroborating transcript,
and the prerequisite-claim summary.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -srm` → `Darwin 24.5.0 x86_64`
- Probes captured 2026-05-08 in `/tmp/lesson129/` on this host.
- The working `.rs` source plus a single `.transcript.txt` are
  committed under
  `observations/129-return-in-match-arm.{rs,transcript.txt}`.
  The contrast and corroborating sources are not committed;
  their full transcripts live inside the `.transcript.txt`
  artifact.

## Sources

### `output/docs/rust/reference/patterns.md` — Identifier patterns

Section "Identifier patterns" (lines 196-244). Two
load-bearing spans.

Lines 204-206 (intro):

> [[patterns.ident.intro]]
>
> Identifier patterns bind the value they match to a variable
> in the [value namespace](names/namespaces.md#r-names.namespaces.kinds).

Lines 216-228 (the bare-identifier rule):

> [[patterns.ident.bare]]
>
> Patterns that consist of only an identifier, possibly with
> a `mut`, match any value and bind it to that identifier.
> This is the most commonly used pattern in variable
> declarations and parameters for functions and closures.

Plain reading: a bare identifier `ord` *as* a pattern matches
any value of the scrutinee's type and binds it to that name.
Lesson 058 already cited this section; today extends from
the inside-payload usage (`Ok(num)`) to top-level usage at the
top of a match arm. Same rule, broader application.

The Reference's `e @ 1..=5` example (lines 230-244) is the
`@`-binding form, named-deferred under *What To Ignore For
Now*.

### `output/docs/rust/reference/expressions/return-expr.md`

The Reference's return-expression page. Three load-bearing
spans (lines 14-22):

> [[expr.return.intro]]
>
> Return expressions are denoted with the keyword `return`.
>
> [[expr.return.behavior]]
>
> Evaluating a `return` expression moves its argument into
> the designated output location for the current function
> call, destroys the current function activation frame, and
> transfers control to the caller frame.
>
> [[expr.return.diverging]]
>
> A `return` expression is [diverging](../divergence.md#r-divergence)
> and has a type of [`!`](../types/never.md#r-type.never).

Plain reading: `return value` (the arrow-grammar at line 10
allows the optional argument) sends `value` to the caller,
exits the current function frame, and is itself a diverging
expression. The lesson body's "control leaves the enclosing
function with `value`; the post-match code is never reached"
maps directly onto this. The `!` type itself is named-deferred
in *What To Ignore For Now*.

The example in lines 26-36 (`fn max` with two early returns)
is the lesson-021 shape with two `return` statements, not in a
match arm body. Today's centered move is the same `return`
*statement* in a match arm body slot.

### `output/docs/rust/reference/expressions/match-expr.md` — match-type and diverging-match rules

Two load-bearing spans, both already cited by lesson 128.

Line 138 (the match-type rule):

> [[expr.match.type]]
>
> The type of the overall `match` expression is the [least
> upper bound](../type-coercions.md#r-coerce.least-upper-bound)
> of the individual match arms.

Line 161 (the diverging-arms rule):

> [[expr.match.diverging]]
>
> If either the scrutinee expression or all of the match arms
> diverge, then the entire `match` expression also diverges.

The first sentence is the all-arms-share-a-type rule (the
LUB). The second sentence handles the *all-arms-diverge*
case. Today's working probe sits in the *partial* case: one
arm has type `()` (empty block per lesson 128), one arm
diverges via `return value;` (per return-expr.md:22 above);
the diverging arm contributes type `!`, which coerces into any
other type per never.md:18 ("Expressions of type `!` can be
coerced into any other type"). The LUB of `()` and `!` is
`()`, so the whole match is `()`-typed and used as a
statement.

Lesson 059 already grounded this same rule for `continue`;
today's centered claim "diverging arms are exempt from the
all-arms-share-a-type rule" carries through unchanged. The
specific shift today is from `continue` (no value, jumps to
loop head) to `return value;` (carries a value, jumps out of
the function).

The neighboring `[expr.match.empty]` rule (line 142) — match
with no arms is `!`-typed — is named-deferred under *What To
Ignore For Now* in lesson 128 and remains so today.

### `output/docs/rust/reference/types/never.md`

Lines 12-18 (introduction and coercion):

> [[type.never.intro]]
>
> The never type `!` is a type with no values, representing
> the result of computations that never complete.
>
> [[type.never.coercion]]
>
> Expressions of type `!` can be coerced into any other type.

The audience-level framing today defers the *name* `!` and
just says "diverging arms are exempt from the
all-arms-share-a-type rule." The Reference span exists for
red-team audit: when one arm is `()` and another is `return
value;` (type `!`), the LUB rule doesn't reject because `!`
coerces into `()`. The exemption mechanic is name-deferred but
corpus-grounded.

### `output/docs/rust/error_codes/E0004.md`

The error-code explainer for E0004 *non-exhaustive patterns*.
Already cited from lesson 051 onward; the centered contrast
probe today fires E0004 with the missing variants `Less` and
`Greater` named in the diagnostic. The lesson body cites the
E-code by family ("the familiar E0004 from lesson 051").

The E0004 text on line 4-7:

> This error indicates that the compiler cannot guarantee a
> matching pattern for one or more possible inputs to a match
> expression. Guaranteed matches are required in order to
> assign values to match expressions, or alternatively,
> determine the flow of execution.

Lines 27-31 (the resolution):

> If you encounter this error you must alter your patterns so
> that every possible value of the input type is matched. For
> types with a small number of variants (like enums) you
> should probably cover all cases explicitly. Alternatively,
> the underscore `_` wildcard pattern can be added after all
> other patterns to match "anything else".

Today's centered contrast empirically witnesses this: stripping
the bare-name arm makes the match non-exhaustive, and rustc
names exactly the two variants the working probe's `ord =>`
arm catches.

### `output/repos/rmp/src/biguint/cmp.rs` (target use site)

The unlock target. Lines 23-26 read:

```rust
match left.cmp(right) {
    Ordering::Equal => {}
    ord => return ord,
}
```

Today's working probe transcribes this exactly (with `a`
standing in for `left.cmp(right)` and the function context
unwrapped from rmp's `for`-loop nesting). Lesson 128 opened
line 24 (`Ordering::Equal => {}`); today opens line 25
(`ord => return ord,`). Together with lesson 127's
`.cmp(&...)` mechanic, the two-arm match on lines 23-26 is
fully readable end-to-end at the audience level (the
surrounding `for`-loop and `iter().rev().zip(...)` chain were
opened by lessons 123-126, and the surrounding function
machinery and `Ord` impl by lessons 111-117).

## Probes

### Working probe

Source: `experimental/eduratchet2/runs/rust-moves/observations/129-return-in-match-arm.rs`.
Identical to *The Move* code block plus a `main` that
exercises both branches.

Transcript captured at
`experimental/eduratchet2/runs/rust-moves/observations/129-return-in-match-arm.transcript.txt`.

```text
--- rustc demo.rs ---
exit=0
(no output)

--- ./demo ---
less
greater
exit=0
```

Notes:

- `rustc demo.rs` is silent, exits 0 — the function and the
  match are perfectly legal Rust.
- `./demo` prints two lines. First line `less` is from the
  call `first_nonzero(Ordering::Less, Ordering::Greater)`:
  the bare-name arm fires, `ord` binds `Less`, `return ord;`
  sends `Less` back, the caller binds and prints it.
- Second line `greater` is from the call
  `first_nonzero(Ordering::Equal, Ordering::Greater)`: the
  empty arm fires, the match runs nothing, control falls
  through to the body's tail expression `b`, which is
  `Greater`; that becomes the function's return value (lesson
  025), the caller binds and prints it.
- Two facts in one transcript: (a) `return value;` in an arm
  body actually exits the function; (b) when an arm doesn't
  fire its return-arm sibling, control falls through past the
  match and the function continues normally.

### Centered contrast probe — drop the bare-name arm (E0004)

Source (not committed; in `transcript.txt`):

```rust
use std::cmp::Ordering;

fn first_nonzero(a: Ordering, b: Ordering) -> Ordering {
    match a {
        Ordering::Equal => {}
    }
    b
}

fn main() {
    let _ = first_nonzero(Ordering::Less, Ordering::Greater);
}
```

```text
error[E0004]: non-exhaustive patterns: `std::cmp::Ordering::Less` and `std::cmp::Ordering::Greater` not covered
 --> non_exhaustive.rs:4:11
  |
4 |     match a {
  |           ^ patterns `std::cmp::Ordering::Less` and `std::cmp::Ordering::Greater` not covered
  |
note: `std::cmp::Ordering` defined here
 --> /rustc/.../core/src/cmp.rs:404:0
 ::: /rustc/.../core/src/cmp.rs:407:4
  |
  = note: not covered
 ::: /rustc/.../core/src/cmp.rs:413:4
  |
  = note: not covered
  = note: the matched value is of type `std::cmp::Ordering`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
  |
5 ~         Ordering::Equal => {},
6 +         std::cmp::Ordering::Less | std::cmp::Ordering::Greater => todo!()
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
exit=1
```

Notes (probe evidence — not corpus quotation):

- E-code is E0004 (already established by lesson 051's
  `Ordering` matching). The headline names the missing
  variants explicitly: `Less` and `Greater`.
- Load-bearing point: rustc names exactly the two variants
  that the working probe's `ord =>` arm catches. The bare-name
  arm is *the* wildcard that completes exhaustiveness; remove
  it and the match is invalid.
- The `help:` suggestion shows rustc itself proposing
  `std::cmp::Ordering::Less | std::cmp::Ordering::Greater =>
  todo!()` — an or-pattern matching exactly the two variants
  the bare-name arm covers. The bare-name arm is shorter and
  also captures the value, which is what makes
  `ord => return ord,` work.
- Exit code: 1; no executable produced.

This contrast probe is the necessary witness for the lesson's
contrastive claim "the bare-name arm is the wildcard that
completes exhaustiveness when the specific arms don't."

### Secondary contrast probe — swap arm order (unreachable warning)

Source (not committed; in `transcript.txt`):

```rust
use std::cmp::Ordering;

fn first_nonzero(a: Ordering, b: Ordering) -> Ordering {
    match a {
        ord => return ord,
        Ordering::Equal => {}
    }
    b
}

fn main() {
    let _ = first_nonzero(Ordering::Less, Ordering::Greater);
}
```

```text
warning: unreachable pattern
 --> swapped.rs:6:9
  |
5 |         ord => return ord,
  |         --- matches any value
6 |         Ordering::Equal => {}
  |         ^^^^^^^^^^^^^^^ no value can reach this
  |
  = note: `#[warn(unreachable_patterns)]` (part of `#[warn(unused)]`) on by default

warning: 1 warning emitted

exit=0
```

Notes:

- This contrast supplies the *Check Yourself* answer to (c)
  in the lesson body. Source-order matching (lesson 031): if
  the bare-name arm appears first, it catches everything, and
  the `Ordering::Equal` arm afterwards is unreachable.
- The diagnostic is a *warning*, not a hard error
  (`#[warn(unreachable_patterns)]`). rustc still produces an
  executable. The warning is the empirical witness for the
  ordering rule.
- The dashes on line 5 underline `ord` with the inline label
  `matches any value` — rustc itself says the bare identifier
  is a catch-all pattern, restating the
  patterns.md:216-228 rule in source-spans.

### Corroborating probe — vary the inputs

Source (not committed; in `transcript.txt`):

```rust
use std::cmp::Ordering;

fn first_nonzero(a: Ordering, b: Ordering) -> Ordering {
    match a {
        Ordering::Equal => {}
        ord => return ord,
    }
    b
}

fn main() {
    let r1 = first_nonzero(Ordering::Greater, Ordering::Less);
    let label1 = match r1 { Ordering::Less => "less",
                            Ordering::Equal => "equal",
                            Ordering::Greater => "greater" };
    println!("r1 = {}", label1);

    let r2 = first_nonzero(Ordering::Equal, Ordering::Less);
    let label2 = match r2 { Ordering::Less => "less",
                            Ordering::Equal => "equal",
                            Ordering::Greater => "greater" };
    println!("r2 = {}", label2);
}
```

```text
--- rustc corroborator.rs ---
exit=0

--- ./corroborator ---
r1 = greater
r2 = less
exit=0
```

Notes:

- First call `first_nonzero(Greater, Less)`: bare-name arm
  binds `ord = Greater`, `return Greater` exits — caller gets
  `Greater`, prints `r1 = greater`. Witness: a *different*
  variant value (Greater, not Less from the working probe)
  rides through the bare-name arm to the caller.
- Second call `first_nonzero(Equal, Less)`: empty arm fires,
  body falls through to tail `b`, function returns `Less` —
  caller gets `Less`, prints `r2 = less`. Witness: a
  *different* fallback value (Less, not Greater from the
  working probe) rides through the tail expression.
- Together with the working probe, three of three
  `Ordering`-variant values have been observed riding through
  the bare-name arm (Less, Greater) and through the tail
  fallback (Less, Greater) — the move composes for every
  variant value, not just the working probe's particular
  pair.

## Direct prerequisite claims

Only summarizing the *specific claim* each direct prerequisite
contributes to lesson 129.

- **Lesson 058 (load-bearing for the bare-identifier binding
  pattern)** — installed binding patterns inside payload
  constructors: `Ok(num)` matches `Ok(_)` and binds the
  payload to `num`. The Reference's *Identifier patterns*
  section (patterns.md:196-218, "Patterns that consist of
  only an identifier ... match any value and bind it to that
  identifier") is the same rule applied at the top level of
  an arm. Today's `ord => return ord,` is `ord` as a
  bare-identifier pattern *not* nested inside a constructor —
  the same rule, less surrounding structure.
- **Lesson 021 (load-bearing for `return value;`)** —
  installed `return value;` as a function-exit statement that
  sends `value` to the caller and exits the function frame.
  Today places that same statement inside a match arm body
  slot. The function-exit semantics are unchanged; what
  changes is the syntactic position.
- **Lesson 059 (load-bearing for diverging arms)** —
  installed that a match arm body can be a diverging
  control-flow expression (specifically `continue`), and that
  such arms are exempt from the all-arms-share-a-type rule.
  The Reference's `[expr.match.diverging]` and
  `[type.never.coercion]` together state the rule today
  reuses for `return value;`. The audience-level framing
  ("diverging arms are exempt") carries through unchanged.
- **Lesson 128 (load-bearing for empty arm body + match-as-statement)**
  — installed `Pattern => {}` as an empty arm body, type
  `()`. Today's working probe pairs an empty arm with a
  return-arm; the rmp `cmp.rs:23-26` shape requires both. The
  whole match is `()`-typed because one arm is `()` (lesson
  128) and one arm is `!` which coerces to `()` (today's
  diverging rule).

## Older supporting lessons

- Lesson 051 (cited only): `Ordering` enum and three-variant
  match. Today's contrast probe trips E0004 with the missing
  variants named — same E-code, same enum, same diagnostic
  shape as lesson 051's original contrast.
- Lesson 025 (cited only): a function's tail expression is the
  return value. Today's working probe relies on this for the
  `b` tail — when the empty arm fires and control falls
  through, `b` becomes the function's return value.
- Lesson 020 (cited only): a function with two parameters
  (`a: Ordering, b: Ordering`).
- Lesson 044 (cited only): `use std::cmp::Ordering;`.
- Lesson 011 (cited only): `println!("{}", x);` — six
  invocations across the working probe and corroborator.
- Lessons 005, 002, 001 (cited only): `let name = value;`,
  `fn main`, compile/run shape.
- Lesson 003 (cited only): four-part diagnostic map applied
  to the E0004 contrast walk in *Try It* and the
  unreachable-pattern walk in *Check Yourself*.
- Lesson 031 (cited only): the source-order matching rule
  used by *Check Yourself* (c) and the secondary contrast
  probe.

## Why these contrast probes (and not "remove the empty arm")

The lesson's centered claim is composite: (1) bare-name
binding pattern at the top level, (2) `return value;` in an
arm body, (3) diverging arm exempt from the share-a-type
rule. The contrastive claim is "the `ord =>` arm is necessary
to make the function work." Two complementary contrasts land
this:

- **E0004 contrast (centered):** drop the `ord =>` arm
  entirely. rustc names the two missing variants and refuses
  to compile. This witnesses the *exhaustiveness role* of the
  bare-name arm — without it, the match is invalid.
- **Unreachable-pattern contrast (secondary):** swap the
  order. rustc still compiles but warns that the second arm
  cannot be reached. This witnesses the *catch-all role* of
  the bare-name arm — `ord` matches every value, so any arm
  after it is unreachable.

The two contrasts together pin the bare-name arm in place:
needed (without it, E0004), but only at the end (with it
first, unreachable warning). This matches the rmp `cmp.rs`
ordering exactly: line 24 is the empty `Equal` arm, line 25
is the bare-name `ord => return ord,` arm — the wildcard
*last*.

A "remove the empty `Equal` arm" probe would also typecheck
(the bare-name arm alone covers every variant including
`Equal`), but it would change the *behavior* of the function
— `Equal` would also early-return through the bare-name arm
rather than falling through to `b`. That is a semantic
question, not the centered claim about exhaustiveness and
ordering. The corroborating probe witnesses the
fall-through behavior with `Equal` directly.
