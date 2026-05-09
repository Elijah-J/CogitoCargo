---
id: 144-closure-captures-outer-let
status: accepted
evidence: ../evidence/144-closure-captures-outer-let.md
---

# A closure body can reference an outer `let`

## The Move

Lessons 142 and 143 wrote closures whose bodies only used their own
parameter: `|x: u32| x + 1`, `|x| x`. Today the body reads a name *from
the enclosing scope* — a name the closure literal never declared.

```rust
fn main() {
    let n: u32 = 10;
    let add_n = |x: u32| x + n;
    let a = add_n(5);
    let b = add_n(7);
    println!("{}", a);
    println!("{}", b);
}
```

`rustc demo.rs` is silent; `./demo` prints two lines:

```text
15
17
```

The closure literal is `|x: u32| x + n`. The parameter list `|x: u32|`
declares one slot, lesson-142 style. The body `x + n` mentions two
names. `x` is the parameter — same as before. `n` is *not* declared
inside the closure; it is the `let n: u32 = 10;` binding from the
enclosing `fn main`, one line earlier. The closure picks it up
silently.

When you call `add_n(5)`, rustc substitutes `5` for `x` and reads `n`
from the outer scope, producing `5 + 10 = 15`. The second call
substitutes `7`, reads the same `n`, produces `17`.

Rust's word for this is *capture*: the closure has *captured* the
binding `n` from its enclosing environment. The Book opens chapter 13
with this exact distinction (`output/docs/rust/book/ch13-01-closures.md:6-9`):
"Unlike functions, closures can capture values from the scope in
which they're defined."

## The new fact: a `fn` item cannot do this

Replace the closure with a top-level-shaped `fn` item declared inside
`main`. Save `fnitem.rs`:

```rust
fn main() {
    let n: u32 = 10;
    fn add_n(x: u32) -> u32 { x + n }
    println!("{}", add_n(5));
}
```

`rustc fnitem.rs` rejects this:

```text
error[E0434]: can't capture dynamic environment in a fn item
 --> fnitem.rs:3:35
  |
3 |     fn add_n(x: u32) -> u32 { x + n }
  |                                   ^
  |
  = help: use the `|| { ... }` closure form instead
```

Read it with the lesson 003 map. Headline: a *new* error code, `E0434`,
with the one-line description "can't capture dynamic environment in a
fn item." The caret points precisely at the `n` token inside the `fn`
body — that name is what rustc refuses to resolve. The `help:` line
proposes the fix: switch the `fn` into a closure (`|| { ... }`). The
explainer page at `output/docs/rust/error_codes/E0434.md` puts it
plainly: "Inner functions do not have access to their containing
environment. To fix this error, you can replace the function with a
closure."

The two probes differ in one line — `let add_n = |x: u32| x + n;`
versus `fn add_n(x: u32) -> u32 { x + n }` — same body `x + n`. The
closure compiles and runs; the `fn` fires E0434. "Closures can read
outer locals" is the property that makes a closure genuinely different
from a `fn`.

## Mental Model Delta

- *Before:* "A closure literal is a value bound to a `let` and called
  with parens (142-143). I have been treating closures as a tidier
  way to write a `fn`."
- *After:* "Closures and `fn` items are not the same. A closure body
  may mention names from the surrounding scope; a `fn` body may not.
  Rust calls this *capture*. It is the property that separates the
  two forms — using a captured name from a `fn` fires E0434."

## Prerequisites

- Installed concepts:
  - **Lesson 142** (load-bearing): closure literal `|param: T| body`
    bound with `let` and called with parens. Today's new fact is
    what the body is allowed to mention.
  - **Lesson 005** (load-bearing): `let name = value;` binds a name
    in scope for later statements. The outer `let n: u32 = 10;` is
    the binding the closure captures.
  - **Lesson 003** (load-bearing): rustc diagnostic map. E0434 is a
    new error code today; the four-part shape carries unchanged.
  - **Lesson 008** / **lesson 020** (load-bearing): `fn name(p: T) -> R { ... }`
    defined and called. Probe 2 uses exactly that shape, declared
    inside `main` instead of at file top level.
  - Cited: lesson 062 (`let n: u32 = ...;` annotation form), lesson
    080 (`u32` in the integer family), lesson 009 (`+` on integers),
    lesson 011 (`println!`), lesson 002 (`fn main`), lesson 001
    (rustc compile + run).
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

In an empty directory, save `demo.rs` from the working probe above.
Run `rustc demo.rs && ./demo`. The output is two lines, `15` and
`17`. Then save `fnitem.rs` (the contrast). Run `rustc fnitem.rs`;
the diagnostic above is what you should see, with the caret on the
`n` inside the `fn` body and the `help:` line suggesting the closure
form.

## What Changed

- A closure body can refer to a name from the enclosing scope. Rust
  calls this *capturing* the binding.
- The captured name need not be declared inside the closure literal —
  it just has to be in scope where the literal is written.
- A `fn` item declared in the same place cannot do this. The attempt
  fires `error[E0434]: can't capture dynamic environment in a fn
  item`; the `help:` line proposes the closure form.
- This asymmetry — closures capture, `fn` items do not — is what
  makes a closure genuinely different from a `fn`.

## Check Yourself

You write `q.rs`:

```rust
fn main() {
    let g: u32 = 9;
    let times_g = |x: u32| x * g;
    let r = times_g(4);
    println!("{}", r);
}
```

(a) Does `rustc q.rs` compile, and what does `./q` print?

(b) Suppose you replace `let times_g = |x: u32| x * g;` with the
nested `fn`-item form `fn times_g(x: u32) -> u32 { x * g }`. Which
error code does rustc fire, and which token does the caret point at?

(Answers: (a) compiles silently; prints `36`. (b) E0434, "can't
capture dynamic environment in a fn item." The caret points at the
`g` inside the `fn` body — that's the captured name rustc refuses
to resolve. The `help:` line suggests the closure form `|| { ... }`.)

## What To Ignore For Now

This lesson installs *only* immutable capture — the closure body
*reads* the captured binding, nothing more. Deferred:

- **Mutable capture** — `let mut count = 0; let mut bump = || count += 1;`.
  Future move.
- **The `move` keyword** — `move |...| ...` forcing the closure to
  take ownership of captured values rather than borrow them.
- **Capture mode classification** — by shared reference vs mutable
  reference vs move. The Book treats this at
  `ch13-01-closures.md:286+` "Capturing References or Moving
  Ownership." Closure sub-arc.
- **The `Fn` / `FnMut` / `FnOnce` traits** — three traits that
  classify closures by capture mode. Closure sub-arc steps 4-5.
- **Borrow-checker interactions** — what happens if you try to mutate
  the captured binding through the original name while the closure
  is still alive.
- **Passing a capturing closure to a function** — generic
  `Fn`-bounded parameters, `impl Fn` returns. Closure sub-arc step 4.
- **The `static`/`const` work-around** — E0434's explainer notes that
  a `fn` body *can* reach a `static` or `const` declared at the same
  level. Today stays with the closure form.

## Evidence

See `../evidence/144-closure-captures-outer-let.md`.
