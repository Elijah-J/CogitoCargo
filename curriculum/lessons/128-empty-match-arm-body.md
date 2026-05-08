---
id: 128-empty-match-arm-body
status: accepted
evidence: ../evidence/128-empty-match-arm-body.md
---

# An empty match arm body `Pattern => {}`

## The Move

A match arm body can be any expression. The smallest one is the
empty block `{}` — by lesson 029 its type is `()` and its value is
`()`. Writing `Pattern => {}` makes the arm match and *do nothing*.
When every arm has type `()`, the whole `match` has type `()` and
works as a statement.

```rust
use std::cmp::Ordering;

fn main() {
    let o: Ordering = Ordering::Equal;
    match o {
        Ordering::Equal => {}
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("greater"),
    }
    println!("done");
}
```

`rustc demo.rs` is silent (exit 0). `./demo` prints:

```text
done
```

The scrutinee is `Ordering::Equal`, so the first arm matches. Its
body `{}` runs nothing. Control falls through to `println!("done")`.
The other two arms are `println!(...)` calls, which also have type
`()`, so all three arms agree. The whole `match` is a `()`-typed
expression, used here as a statement.

## Mental Model Delta

- Before: "Every match arm I have written so far carried a useful
  value or a side-effecting `println!`. I have not written an
  arm that does nothing."
- After: "An arm body is an expression, and `{}` is a perfectly good
  expression — it has type `()` and runs no code. `Pattern => {}`
  is the way to say *match this case but do nothing here*. If every
  arm has type `()`, the whole match has type `()` too, and I can
  use it as a statement (no `let x = ...`)."

## Prerequisites

- Installed concepts:
  - Lesson 029 (load-bearing): `{}` is an empty block; type and
    value are both `()`. Today's move is that block in a match arm
    body.
  - Lesson 030 (load-bearing): the match form, with arms
    `pattern => arm_expression`, and the rule *all arms must share
    a type*. Today's contrast probe trips that rule on purpose.
  - Lesson 051 (load-bearing): match on `Ordering`'s three
    variants. Today's probe is a tiny variation — same three arms,
    one now empty.
  - Lesson 031 (cited): exhaustiveness; reused via 051.
  - Lesson 011 (cited): `println!(...)` in arm bodies; each call
    has type `()`.
  - Lesson 044 (cited): `use std::cmp::Ordering;`.
  - Lesson 019 (cited): `let o: Ordering = Ordering::Equal;`.
  - Lessons 001, 002, 005, 003: `rustc demo.rs` then `./demo`,
    `fn main`, `let name = value;`, the four-part diagnostic map
    used in the contrast walk.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, Linux/macOS shell.

## Try It

Save the snippet above as `demo.rs` in a fresh directory. Compile
and run:

```console
$ rustc demo.rs
$ ./demo
done
```

Now the contrast. Try to bind the match to a name and *change one
arm's type* so the arms disagree. Save `broken.rs`:

```rust
use std::cmp::Ordering;

fn main() {
    let o: Ordering = Ordering::Equal;
    let x = match o {
        Ordering::Equal => {},
        Ordering::Less => 5,
        Ordering::Greater => 10,
    };
    println!("{}", x);
}
```

`rustc broken.rs` rejects it:

```text
error[E0308]: `match` arms have incompatible types
 --> broken.rs:7:27
  |
5 |       let x = match o {
  |  _____________-
6 | |         Ordering::Equal => {},
  | |                            -- this is found to be of type `()`
7 | |         Ordering::Less => 5,
  | |                           ^ expected `()`, found integer
...
```

Read it with lesson 003's map. The E0308 headline carries a new
sub-form `\`match\` arms have incompatible types`. Dashes underline
`{}` with `this is found to be of type \`()\``; the caret points at
`5` with `expected \`()\`, found integer`. Two facts in one
diagnostic: (1) rustc names `{}`'s type as `()`, in the span; (2)
once the first arm fixes the match's type at `()`, every other arm
is checked against `()`. The `i32` literal `5` fails the check —
lesson 030's all-arms-share-a-type rule, on a fresh type pair.
(Full transcript in `../evidence/128-empty-match-arm-body.md`.)

## What Changed

- You can write `Pattern => {}` as a match arm. The body `{}` is an
  empty block expression, type `()` (lesson 029), value `()`. The
  arm matches and runs nothing.
- A match expression whose arms all have type `()` is itself
  `()`-typed. Such a match works as a *statement*: no `let x = ...`,
  no value to bind. Control falls through to the next statement
  after the `match`.
- The arms-must-share-type rule from lesson 030 still applies. The
  empty `{}` body fixes the arm's type at `()`. Mixing `{}` with
  arms that produce integers fires E0308 with the sub-form
  `\`match\` arms have incompatible types`, and rustc explicitly
  spells `()` in the inline labels.

## Check Yourself

You write `tiny.rs`:

```rust
use std::cmp::Ordering;

fn main() {
    let o: Ordering = Ordering::Greater;
    match o {
        Ordering::Equal => {}
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("greater"),
    }
    println!("done");
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) The `Ordering::Equal` arm is empty. If `o` were `Ordering::Equal`
instead, what would `./tiny` print?

(c) If you changed `Ordering::Less => println!("less"),` to
`Ordering::Less => 7,` (and left the empty-body `Equal` arm alone),
what would the rustc headline say?

*(Answers: (a) Yes; prints `greater` then `done` (two lines).
(b) Just `done`. The empty `Equal` arm matches, runs nothing,
and control falls through to the unconditional `println!`. (c)
`error[E0308]: \`match\` arms have incompatible types`. The empty
`{}` arm fixes the match's type at `()`; the literal `7` violates
that — same shape as today's contrast probe.)*

## What To Ignore For Now

- *Diverging arm bodies* — `continue` (lesson 059), `break`,
  `return`, `panic!`. These also "do nothing meaningful with the
  match's value", but the rule is different (the never type `!`
  coerces to anything). Only `continue` is installed; the rest are
  named-deferred.
- *The never type `!` and its arm-body merge rules.* Wholesale
  deferred. (The Reference says a match with no arms has type `!`,
  match-expr.md:142.)
- *Block expressions with statements inside* — `{ side_effect();
  42 }`. Real Rust uses these constantly. Today's empty block has
  no statements at all. Future move.
- *The `_` wildcard with an empty body* — `_ => {}`. Composes
  today's move with lesson 031's `_` pattern; not centered.
- *`if let Pattern = expr { } else { ... }`* — another way to
  "match one variant and do nothing." Future move.
- All previously deferred items.

## Evidence

See `../evidence/128-empty-match-arm-body.md` for the corpus-quote
map, the toolchain string, the working probe transcript, the
arm-type-mismatch contrast E0308 transcript, the corroborating
`Ordering::Less` transcript, and the prerequisite-claim summary.
