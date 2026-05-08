---
id: 129-return-in-match-arm
status: accepted
evidence: ../evidence/129-return-in-match-arm.md
---

# A diverging match arm `name => return name,`

## The Move

A match arm body can be `return value;`. When that arm fires,
control leaves the enclosing function carrying `value` — the
post-match code is never reached for that call.

The arm pattern can also be a bare name. A bare identifier at
the top of an arm is a *binding pattern*: it matches any value
of the scrutinee's type and binds it to that local name. Pair
the two and you get the shape `name => return name,`: catch
whatever's left, hand it back to the caller. Combined with
lesson 128's empty arm, the canonical "skip one variant, exit
early on every other" function is one match wide:

```rust
use std::cmp::Ordering;

fn first_nonzero(a: Ordering, b: Ordering) -> Ordering {
    match a {
        Ordering::Equal => {}
        ord => return ord,
    }
    b
}
```

`rustc demo.rs` is silent (exit 0). With `a = Ordering::Less`,
the bare-name arm fires (`ord` binds `Less`), `return ord;`
exits the function with `Less`. With `a = Ordering::Equal`,
the empty arm fires; control falls through past the match to
the tail expression `b` (lesson 025), which the function
returns instead.

## Mental Model Delta

- Before: "I know `return value;` exits a function (021). I
  know an arm body can be `continue` (059). I have not put
  `return value;` in an arm body, and I have not used a bare
  name as an arm pattern."
- After: "Both pieces work. (1) A bare identifier `ord` at
  the top of an arm matches any value and binds it — the same
  rule lesson 058 used inside `Ok(num)`, now at the top
  level. (2) `return value;` works in any expression
  position, including a match arm body. The arm diverges,
  just like `continue` did in lesson 059, and gets the same
  exemption from the all-arms-share-a-type rule. So a match
  can mix `Pattern => {}` (type `()`) with `name => return
  value;` (diverging) and still typecheck."

## Prerequisites

- Installed concepts:
  - Lesson 058 (load-bearing): a bare identifier inside
    `Ok(num)` is a binding pattern that matches any value and
    binds it. Today extends from inside-payload to top-level:
    `ord => ...` at the top of an arm.
  - Lesson 021 (load-bearing): `return value;` exits the
    enclosing function with `value`.
  - Lesson 059 (load-bearing): a match arm body can be a
    diverging control-flow expression (`continue`); such arms
    are exempt from the all-arms-share-a-type rule. Today is
    `return value;` instead of `continue`.
  - Lesson 128 (load-bearing): `Pattern => {}` arm body, type
    `()`. Today's working probe pairs an empty arm with a
    return-arm; the rmp `cmp.rs:23-26` shape requires both.
  - Lesson 051 (cited): `Ordering` enum and three-variant
    match.
  - Lesson 025 (cited): a function's tail expression is the
    return value (here, `b`).
  - Lesson 020 (cited): a function with two parameters.
  - Lessons 044, 011, 005, 002, 001, 003 (cited): unchanged.
- Ordinary computer-use assumptions: terminal, plain-text
  editor, `rustc` on `PATH`, Linux/macOS shell.

## Try It

Save the snippet as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
less
greater
```

Walk it. First call: `a = Less`, `b = Greater`. The
`Ordering::Equal` arm doesn't match. The bare-name arm `ord`
matches any value, binding `ord` to `Less`; its body
`return ord;` exits the function with `Less` — `b` is never
read. The caller binds `x` to `Less` and prints `less`.

Second call: `a = Equal`, `b = Greater`. The first arm now
matches; `{}` runs nothing; control falls through past the
match to the tail `b`. Lesson 025: a function's tail
expression *is* the return value, so the function returns
`Greater` instead. The caller prints `greater`.

Now the contrast. *Drop* the bare-name arm entirely. Save
`broken.rs`:

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

`rustc broken.rs` rejects it:

```text
error[E0004]: non-exhaustive patterns: `std::cmp::Ordering::Less` and `std::cmp::Ordering::Greater` not covered
 --> broken.rs:4:11
  |
4 |     match a {
  |           ^ patterns `std::cmp::Ordering::Less` and `std::cmp::Ordering::Greater` not covered
...
```

This is the familiar E0004 from lesson 051 — but read it as a
*structural* witness: rustc names exactly the two variants the
working probe's `ord =>` arm catches. The bare-name arm is the
wildcard that completes exhaustiveness when the specific arms
don't. Strip it and the match is invalid.

## What Changed

- A bare identifier `name` at the top of a match arm pattern
  is a binding pattern: it matches any value of the scrutinee's
  type and binds it to `name`, usable in the arm's body — the
  same rule lesson 058 used inside `Ok(num)`.
- A match arm body can be `return value;`. When that arm
  fires, control leaves the enclosing function with `value`;
  the post-match code is never reached.
- Diverging arms — `continue` (lesson 059), `return value;`
  (today) — are exempt from the all-arms-share-a-type rule.
  A match can mix `Pattern => {}` (type `()`) with
  `name => return value;` (diverging) and still typecheck.
- The composed shape — `Variant => {}` for one case,
  `name => return name;` for everything else — says "skip
  one variant, hand every other back to the caller early."
  rmp's `BigUInt` ordering uses exactly this shape.

## Check Yourself

You write `tiny.rs`:

```rust
use std::cmp::Ordering;

fn skip_less(a: Ordering, fallback: Ordering) -> Ordering {
    match a {
        Ordering::Less => {}
        ord => return ord,
    }
    fallback
}

fn main() {
    let r1 = skip_less(Ordering::Greater, Ordering::Equal);
    let r2 = skip_less(Ordering::Less, Ordering::Equal);
    println!("{:?} {:?}", r1, r2);
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) On which call does the body's tail `fallback` run, and
why?

(c) If you swap the two arms so `ord => return ord,` appears
*first*, what does rustc say? Does it still produce an
executable?

*(Answers: (a) Yes; prints `Greater Equal`. First call hits
the bare-name arm; `ord` binds `Greater`, `return Greater`
exits. Second call hits the empty arm and falls through to
`fallback`, which is `Equal`. (b) The second call
(`a = Less`); the empty arm fires and the body's tail runs.
(c) `warning: unreachable pattern`. The bare-name arm
catches everything, so the more specific `Ordering::Less`
arm afterwards is unreachable. It's a warning, not an error,
so rustc still produces the executable — but the wildcard-shaped
arm must come last.)*

## What To Ignore For Now

- *The never type `!`.* The Reference's formal name for the
  type of `return value;` and other diverging expressions.
  Audience-level framing today is "diverging arms are exempt";
  `!` is name-deferred.
- *`break value;`, `panic!()` in arm bodies.* Same exemption
  applies; named, not exercised.
- *Match guards* `pattern if cond => arm`. Deferred.
- *`@`-bindings* `name @ pattern`. Composes today's bare-name
  pattern with a subpattern. Deferred.
- *Returning from inside a `for`-loop body.* rmp's actual use
  site is `for { match { return } }` nested. Today returns
  from a function's plain body; loop composition is a future
  move.
- All previously deferred items.

## Evidence

See `../evidence/129-return-in-match-arm.md`.
