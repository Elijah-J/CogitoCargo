---
id: 049-method-chaining
status: accepted
evidence: ../evidence/049-method-chaining.md
---

# Chain two method calls with `String::new().is_empty()`

## The Move

Lesson 040 introduced the method-call form `receiver.method(args)`
and showed the *receiver* — the thing on the left of the dot — as a
plain binding name like `n`. The Reference's grammar is more general
than that: the receiver is *any expression*. The result of one call
can be the receiver of the next call, written directly:

```rust
let chained: bool = String::new().is_empty();
```

Read it left to right. `String::new()` is lesson 042's no-receiver
associated function and produces a fresh empty `String` value. That
whole expression is the *receiver* of the next method call:
`.is_empty()` is invoked on the `String` that `String::new()` just
produced. `is_empty` returns `true` when a `String` has length zero,
so `chained` ends up bound to `true`.

This is what *method chaining* means: two `.method()` calls written
end-to-end, with the value of each call becoming the receiver of the
next. The chain parses left-associatively — `String::new().is_empty()`
groups as `(String::new()).is_empty()`. No new mechanism beyond
"receiver is an expression"; pure composition of lessons 040 + 042.

## Mental Model Delta

- Before: "The receiver in `value.method(args)` is a binding — I have
  to write `let s = String::new();` first, *then* `s.is_empty()`."
- After: "The receiver is an *expression*. Any expression that
  produces a value of the right type can sit on the left of the dot,
  including another method call. The chain `a().b()` runs `a()`
  first, then calls `.b()` on the value `a()` returned. The two-step
  `let temp = a(); temp.b();` form is equivalent in value — chaining
  just skips the intermediate binding."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 005: compile/run, `fn main`, `let name = value;`
    plus the `{name}` placeholder.
  - Lesson 012: the boolean values `true`/`false` print as the literal
    words.
  - Lesson 019 (load-bearing for the *shape*): `let name: TYPE =
    value;`. Today fills `TYPE` with `bool` and `String` — same slot.
  - Lesson 040 (load-bearing): the dot-form `receiver.method(args)`.
    The Reference grammar names the receiver an *Expression* —
    today's lesson exercises that.
  - Lesson 042 (load-bearing): `String::new()` returns a fresh empty
    `String`. The chain uses it as the inner expression.
- Ordinary computer-use assumptions: terminal, plain-text editor,
  `rustc` on `PATH`, Linux/macOS shell.

## Try It

In a fresh empty directory, create `demo.rs`:

```rust
fn main() {
    let chained: bool = String::new().is_empty();
    let s: String = String::new();
    let stepped: bool = s.is_empty();
    println!("chained = {chained}, stepped = {stepped}");
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
chained = true, stepped = true
```

Line 2 is the chain: `String::new()` is evaluated first, producing an
empty `String`; `.is_empty()` is then called on that fresh `String`,
producing `true`. Lines 3-4 are the two-step form: bind `s` to a
fresh empty `String`, then call `s.is_empty()`. The output line shows
both `true` — same value, same boolean fact about an empty `String`,
just a different way of arranging the source.

Grouping check: editing line 2 to `let chained: bool =
(String::new()).is_empty();` (explicit parens around the inner call)
still compiles and still prints the same line. Probe-level evidence
that the chain parses as `(String::new()).is_empty()` — the parens
are redundant.

## What Changed

- The receiver in `receiver.method(args)` is *any expression*. A
  prior call can sit directly on the left of the dot, with no
  intermediate `let`.
- `String::new().is_empty()` is the smallest concrete chain: lesson
  042's `String::new()` produces the receiver, then `.is_empty()`
  runs on it.
- A chain is equivalent in value to a `let temp = ...; temp.method();`
  rewrite. Today's `chained` and `stepped` both end up `true`.
- `a().b()` parses as `(a()).b()` — left-associative; the implicit
  grouping lines up with reading order.
- `String::is_empty` is a small collateral fact: it returns `true`
  when the `String` has zero length.

## Check Yourself

You write `pred.rs`:

```rust
fn main() {
    let a: bool = String::new().is_empty();
    let s: String = String::new();
    let b: bool = s.is_empty();
    println!("a = {a}, b = {b}");
}
```

(a) Does rustc accept the program?

(b) What single line does `./pred` print?

(c) In the chain `String::new().is_empty()`, name the *receiver* of
the `.is_empty()` call.

(Answers: (a) Yes — same shape as the lesson, with `a`/`b` as the
binding names. (b) `a = true, b = true`. (c) The expression
`String::new()` — i.e. the value it produces, a fresh empty
`String`. Today's main concept: receivers are expressions, not just
names.)

## What To Ignore For Now

This lesson installs only one idea: a method-call's receiver can be
any expression, including another method or function call, so two
`.method()` calls can be written end-to-end. Deferred:

- *Trait machinery* — methods (`is_empty`, `new`, any other) are
  defined in `impl` blocks (often via traits). Carrying over from
  lessons 040, 041, 042.
- *`is_empty` on other types* — `Vec::is_empty`, `&str::is_empty`,
  many more. Today's example is on `String`.
- *`is_empty` is `const fn`* — the std signature is `pub const fn
  is_empty(&self) -> bool`. Treat it as a regular method.
- *Iterator chains* `it.filter(...).map(...).collect()`, *the
  builder pattern* `Foo::new().a(...).build()`, and other multi-link
  chains. Today's chain is two methods.
- *Method-resolution autoref/autoderef* — the magic that lets
  `s.is_empty()` call a `&self` method without an explicit `&`.
  Carrying over from lessons 040, 041.
- *Operator precedence in chains* — how `.method()` interacts with
  `?`, `as`, etc. Today's chain has no other operators.
- *Chains where an intermediate method returns `()`* — e.g.
  `s.push_str("x").push_str("y")` fails to type-check because
  `push_str` returns `()`. Future move.
- All previously deferred items.

## Evidence

See `../evidence/049-method-chaining.md` for the corpus-quote map,
the rustc / system toolchain string, the working probe transcript,
the parens-grouping bonus probe, and the prerequisite-claim summary.
