---
id: 059-continue-in-match-arm
status: accepted
evidence: ../evidence/059-continue-in-match-arm.md
---

# A `match` arm whose body is `continue;`

## The Move

A `match` arm's body is allowed to be a *control-flow* expression
instead of a value expression. The smallest useful case: write
`_ => continue` as one arm of a `match` inside a loop. When that arm
fires, control jumps straight to the loop head (cycle 035) without
producing a value for the match. The other arms still produce values
of the binding's type, and the program type-checks:

```rust
let v: i32 = match n % 2 {
    0 => n,
    _ => continue,
};
```

For odd `n`, the wildcard arm fires; `continue` skips the rest of the
loop pass, so `let v: i32 = ...;` and any code after it never run for
that pass. For even `n`, the `0 =>` arm fires and the match's value is
`n`, which gets bound to `v`.

## Mental Model Delta

- *Before:* "Cycle 030 said all arms of a `match` must produce values
  of the same type. So if the binding is `: i32`, every arm body has
  to evaluate to an `i32`."
- *After:* "There is one exception. An arm whose body is a *diverging*
  control-flow expression — `continue`, `break`, `return` — never
  reaches the point where its value would be used; control leaves the
  match before the value question matters. Such arms are exempt from
  the all-arms-share-type rule. The match's type is decided by the
  *value-producing* arms; the diverging arms join in for free. The
  audience-level reading: control-flow keywords let a match arm
  *escape* without producing a value."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 005, 006, 019, 023: compile and run, `fn main`,
    `let`, `let mut`, `: i32` annotation, compound `+=`. Used as the
    `let mut total: i32 = 0;` plus `total += v;` scaffold.
  - Lesson 022 + cycle 039 (load-bearing): `for var in 1..=N { ... }`
    runs the body once per value, with `var` bound. The probe uses
    `for n in 1..=5`.
  - Lesson 030 (load-bearing): the `match` machine — scrutinee plus
    arms `pattern => arm_expression,`, matching arm's value is the
    match's value. **The rule today refines: all arms share a type.**
  - Lesson 031 (load-bearing): the `_` wildcard pattern at the top
    of an arm. Used here to catch all non-zero remainders.
  - Lesson 035 (load-bearing): `continue;` inside a loop body skips
    the rest of the current pass and returns to the loop head.
    **This lesson installs that the same `continue` works as a
    `match` arm's body.**
  - Lesson 037: `n % 2` is the parity check.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
containing exactly:

```rust
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
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
total = 6
```

Walk it. `n` takes each value in `1..=5`. On odd `n` (`1`, `3`, `5`)
the `n % 2` scrutinee is `1`; the `0 =>` arm doesn't match, `_`
does, and its body is `continue` — control leaves the match and
returns to the loop head *without producing a value*. The `let v:
i32 = ...;` and `total += v;` lines below never run on those passes.
On even `n` (`2`, `4`) the scrutinee is `0`, the first arm matches,
the arm's value is `n`, the match evaluates to `n`, `v` is bound,
and `total += v;` runs. After the range exhausts, `total` is
`2 + 4 = 6` and `println!` prints `total = 6`.

The load-bearing observation. The binding is `let v: i32 = ...;`,
and cycle 030's all-arms-share-type rule says every arm body must
produce an `i32`. The `_ => continue` arm doesn't produce *any*
value: it transfers control out of the match. rustc accepts the
program anyway. The Reference states the rule (match-expr.md line
161): *if either the scrutinee expression or all of the match arms
diverge, then the entire `match` expression also diverges.* Today
uses the *partial* form: some arms diverge, others produce values,
and the match's type comes from the value-producing arms — `i32`
here, from `0 => n`.

`continue` is one of three control-flow keywords with this
escape-without-a-value behavior; `break` and `return` are the other
two. The same exemption applies to all three.

## What Changed

- A `match` arm's body can be `continue`, jumping straight to the
  loop head when that arm fires. Cycle 035's `continue;` works in a
  new place: as the right-hand side of a `match` arm's `=>`.
- Cycle 030's all-arms-share-type rule is refined: arms whose body
  is a *diverging* control-flow expression (`continue`, `break`,
  `return`) are exempt — control leaves the match before any value
  is needed. The match's type comes from the value-producing arms.
- The natural shape is `_ => continue` (or `Variant => continue`) —
  an arm that says "this case has nothing to contribute; skip it."
- The Book's guessing-game uses exactly this shape:
  `Err(_) => continue` (cycle 058's payload pattern, today's
  divergent-arm rule, cycle 035's `continue`). The full composition
  is a future cycle.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let mut count: i32 = 0;
    for n in 1..=4 {
        let v: i32 = match n {
            1 => continue,
            _ => n,
        };
        count += v;
    }
    println!("count = {count}");
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) On which value of `n` is `count += v;` *not* executed, and why?

(c) If you change the `_ => n,` arm to `_ => "skip",` (a string,
keeping `1 => continue,` the same), what does rustc do? Why doesn't
the `1 => continue,` arm have to be a string too?

*(Answers: (a) Yes; prints `count = 9`. The `_` arm fires for `n` in
`{2, 3, 4}`, contributing `2 + 3 + 4 = 9`. The `1 =>` arm fires once
and diverges via `continue`, contributing nothing. (b) On `n = 1`.
The arm jumps to the loop head before the match produces a value, so
neither `let v: i32 = ...;` nor `count += v;` runs that pass. (c)
rustc refuses with `error[E0308]: mismatched types`, pointing at
`"skip"` and reporting `expected \`i32\`, found \`&str\``. Note rustc
flags only `"skip"`, not the `1 => continue` arm — `continue`
diverges and is exempt, so the binding's `: i32` is the only `i32`
constraint left, and `"skip"` clearly violates it. A non-diverging
non-`i32` body would still fail; the exemption is specifically for
control-flow keywords that escape the match.)*

## What To Ignore For Now

- *The never type `!`*. The Reference's formal name for the type of
  a diverging expression. The audience-level framing here is
  "control-flow keywords let an arm escape without producing a
  value"; `!` as a typed name is deferred.
- *`break;` and `return;` in match arm bodies*. Same divergent-arm
  rule applies; named here, not exercised.
- *`break value;` from a match arm*. Cycle 028's shape can also
  appear as an arm body. Deferred.
- *`panic!()` and other diverging library calls*. Also diverge, also
  exempt; deferred.
- *Labeled loops `'outer: loop { ... continue 'outer; }`*. Deferred
  along with cycle 035's same-named deferral.
- *The full guessing-game form `let guess: u32 = match
  guess.trim().parse() { Ok(num) => num, Err(_) => continue, };`*.
  Natural composition cycle once `read_line + parse + match + loop`
  are all together. Forward pointer.
- *`let _ = match ... { ... continue }` without an annotation*.
  Inference of the match's type via `continue`'s divergence; edge
  case, deferred.
- All previously deferred items.

## Evidence

See `../evidence/059-continue-in-match-arm.md`.
