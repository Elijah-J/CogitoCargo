---
id: 120-self-in-non-receiver
status: accepted
evidence: ../evidence/120-self-in-non-receiver.md
---

# Write `other: &Self` in a non-receiver parameter slot inside an impl

## The Move

Lesson 113 installed `&Counter` in a non-receiver parameter slot of a
trait method, with `&b` at the call site, and the witness that the
caller still owns `b` after the call. Lesson 100 installed `Self` as a
*type alias* inside an impl block: the Book ch05-03 says it verbatim,
"Within an `impl` block, the type `Self` is an alias for the type that
the `impl` block is for." Today composes the two: in the *impl method
signature*, write `&Self` instead of `&Counter` in the non-receiver
slot. Same mechanic, different spelling — only in the impl. The trait
declaration still uses the named type.

```rust
struct Counter {
    count: u32,
}

trait Combine {
    fn combine(&self, other: &Counter) -> u32;
}

impl Combine for Counter {
    fn combine(&self, other: &Self) -> u32 {
        self.count + other.count
    }
}

fn main() {
    let a = Counter { count: 7 };
    let b = Counter { count: 35 };
    let result = a.combine(&b);
    println!("result = {}", result);
    println!("b.count still = {}", b.count);
}
```

`rustc demo.rs` is silent (exit 0). `./demo` prints `result = 42` then
`b.count still = 35`. Because `Self` inside `impl Combine for Counter`
is an alias for `Counter`, `&Self` in the impl signature means
`&Counter` — which is exactly what the trait declaration says, so the
contract-matching rule from lesson 112 is satisfied after `Self`
resolution. The whole probe is byte-output-identical to the lesson-113
source (where the impl also wrote `&Counter`); the appendix records
both transcripts side-by-side.

This is the spelling the rmp source actually uses. In
`src/biguint/cmp.rs`, line 5 is `fn eq(&self, other: &BigUInt)` and
lines 13 and 19 are `fn partial_cmp(&self, other: &Self)` and
`fn cmp(&self, other: &Self)` — the *same file* uses both forms,
because inside `impl ... for BigUInt` the two name the same type.
Lesson 113's *What To Ignore For Now* explicitly named this stylistic
move as deferred. Today is exactly that move.

## Mental Model Delta

- *Before*: "I saw `Self` as a return type in an inherent impl (lesson
  100). `&self` was the receiver shorthand. Non-receiver reference
  parameters I have always written with the explicit type name, like
  `other: &Counter` (lesson 113)."
- *After*: "`Self` inside any impl block is a type alias for the
  impl-target type, so it can sit in any type position — including
  non-receiver parameters. `other: &Self` and `other: &Counter` inside
  `impl ... for Counter` name the same type; the choice is stylistic.
  Outside one of the Reference-allowed sites (an impl block, trait
  definition, or type definition), `Self` does not exist as a type —
  `E0411`."

## Prerequisites

- Installed concepts:
  - **Lesson 113** (load-bearing): `&Type` as a non-receiver parameter
    on a trait method, the matching `&value` call-site, and the
    "caller still owns" property. Today substitutes `Self` for the
    named type; the mechanic is unchanged.
  - **Lesson 100** (load-bearing): `Self` as a type alias inside an
    impl block, with the rule quoted above. 100 used the alias in a
    return-type slot inside an inherent impl; today extends it to a
    *parameter-type* slot inside a *trait* impl.
  - **Lesson 111** (load-bearing): the `impl Trait for Type { ... }`
    scaffold. The Self-alias rule applies inside it the same way.
  - **Lesson 112** (load-bearing): the rule that the impl method
    signature must match the trait declaration exactly. Today uses it
    *after* `Self` resolution: the impl's `&Self` resolves to
    `&Counter` and matches the trait declaration's `&Counter`.
  - **Lessons 095, 040, 003, 011, 001, 002, 005, 019, 080** (cited):
    struct with named fields; dot-call; rustc diagnostic map;
    `println!`; compile-and-run; `fn main`; `let`; type-annotation
    slot; `u32`.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
result = 42
b.count still = 35
```

The only change vs. lesson 113's working probe is the single `&Self`
token in the impl method signature; the trait declaration still says
`&Counter`. The call site is still `&b`; the body is still
`self.count + other.count`; the field reads still work. `Self` is
purely a spelling change in the impl.

*Now the contrast.* Save `outside.rs` and place `&Self` in a free
function — outside any impl block:

```rust
fn outside(a: &Self) -> u32 { 0 }

fn main() { println!("never reached"); }
```

Compile:

```text
error[E0411]: cannot find type `Self` in this scope
 --> outside.rs:1:16
  |
1 | fn outside(a: &Self) -> u32 {
  |    -------     ^^^^ `Self` is only available in impls, traits, and type definitions
  |    |
  |    `Self` not allowed in a function

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0411`.
```

Read with the lesson 003 map. Headline `error[E0411]: cannot find type
`Self` in this scope`. Caret under `Self` at column 16 of line 1. The
inline label states today's rule verbatim: ``Self` is only available
in impls, traits, and type definitions`. A free function is none of
those three sites, so the alias does not exist.

## What Changed

- Inside `impl ... for Counter`, `&Self` and `&Counter` name the same
  type. Swapping one for the other in the parameter slot leaves
  behavior unchanged — the appendix records byte-identical run output.
- `Self` is a *type alias*, not a special parameter. It can sit in any
  type position inside an impl block, not only in the receiver
  shorthand `&self`.
- Outside any impl, trait, or type definition, `Self` does not exist.
  rustc fires `E0411` with the inline label ``Self` is only available
  in impls, traits, and type definitions`.
- The rmp source uses both spellings inside the same file: line 5
  `&BigUInt` and lines 13 / 19 `&Self`. Both forms now read the same.

## Check Yourself

You write `tiny.rs`:

```rust
struct Tally { n: u32 }

trait Sum { fn sum(&self, other: &Tally) -> u32; }

impl Sum for Tally {
    fn sum(&self, other: &Self) -> u32 {
        self.n + other.n
    }
}

fn main() {
    let x = Tally { n: 10 };
    let y = Tally { n: 5 };
    let s = x.sum(&y);
    println!("s = {}, y.n = {}", s, y.n);
}
```

(a) Does `rustc tiny.rs` accept the program (silent, exit 0)?

(b) What single line does `./tiny` print?

(c) If you also change the *impl* signature from `&Self` to `&Tally`,
does the program still compile?

*(Answers: (a) Yes — inside `impl Sum for Tally`, `&Self` is `&Tally`,
so the impl signature matches the trait declaration's `&Tally` exactly
(lesson 112's contract-matching rule, applied after `Self` resolution).
(b) `s = 15, y.n = 5`. (c) Yes — both signatures are then literally
`&Tally`, the lesson-113 named-type form.)*

## What To Ignore For Now

Today installs only the spelling change. Deferred:

- *`&mut Self`* in a non-receiver slot — composes 101's `&mut self`.
- *`Self` as a return type in a trait method* — brings `Self: Sized`
  and dispatch questions.
- *`Self::method()` self-references inside an impl body* — calls an
  associated function via the alias; distinct mechanic.
- *`where Self: Bound` clauses* — composes today with trait bounds.
- *`Self` inside a trait declaration itself* (not the impl) — the
  Reference `paths.md:316-318` frames it from the declaration side as
  its own move.
- *Multiple `&Self` parameters in one signature*, `&'a Self`,
  `& &Self` — extensions; not centered today.
- All borrow-checker rules, lifetimes, shared-vs-mut exclusivity — as
  in 113, wholesale deferred.

## Evidence

See `../evidence/120-self-in-non-receiver.md`.
