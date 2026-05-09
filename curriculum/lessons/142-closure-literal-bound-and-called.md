---
id: 142-closure-literal-bound-and-called
status: accepted
evidence: ../evidence/142-closure-literal-bound-and-called.md
---

# Bind a closure literal to a `let` and call it with parens

## The Move

Functions so far lived at the top level of the file as `fn`-blocks
(lessons 008/020/021), and you called them from inside `main` with parens
(`name(value)`). Today: keep the *call-with-parens* shape, but build the
callable thing as a *value* on the right of a `let`, using a new syntax
— pipes around the parameter list, then the body.

```rust
fn main() {
    let add_one = |x: u32| x + 1;
    let a = add_one(5);
    let b = add_one(10);
    println!("{}", a);
    println!("{}", b);
}
```

`rustc demo.rs` is silent; `./demo` prints two lines:

```text
6
11
```

The right side of `let add_one = ...` is the *closure literal*
`|x: u32| x + 1`. The pipes `|...|` enclose a parameter list — `x: u32`
is a typed parameter, same `name: TYPE` shape lesson 020 used between
function parens. After the closing pipe comes the body, here the single
expression `x + 1` with no braces and no trailing `;`. The call sites
`add_one(5)` and `add_one(10)` reuse the lesson-008 parens-call shape on
the `let`-bound name.

## The new fact: closure literal syntax

The Book at `output/docs/rust/book/ch13-01-closures.md:208-213` lines up
four equivalent forms side by side:

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

Today's `add_one = |x: u32| x + 1;` keeps `v2`'s parameter annotation
and `v4`'s expression body:

- The pipe-bracket-pipe list `|x: u32|` mirrors the function-parameter
  list `(x: u32)` from lesson 020. Same `name: TYPE` slot.
- The body is a single expression `x + 1`, no braces. Lesson 025's
  implicit-return rule applies: the body expression *is* the closure's
  return value, no `return` keyword needed. The Book: "we remove the
  brackets, which are optional because the closure body has only one
  expression."
- The return type is inferred from the body — `v3`/`v4` skip `-> u32`
  because rustc can read it off `x + 1` once `x` is `u32`.

The Book introduces closures as "anonymous functions you can save in a
variable or pass as arguments to other functions." Today only covers the
"save in a variable" half; passing closures to functions is deferred.

## The closure is a *value with a type*

Drop the call parens and try to assign the closure itself to a `u32`:

```rust
fn main() {
    let add_one = |x: u32| x + 1;
    let a: u32 = add_one;
    println!("{}", a);
}
```

`rustc` rejects this with E0308:

```text
error[E0308]: mismatched types
 --> noparens.rs:3:18
3 |     let a: u32 = add_one;
  |            ---   ^^^^^^^ expected `u32`, found closure
  |
  = note: expected type `u32`
          found closure `{closure@noparens.rs:2:19: 2:27}`
help: use parentheses to call this closure
```

Two facts from rustc's mouth:

- `add_one` *is* a value, but its type is *not* `u32`. The `note:`
  names the kind: `closure`. The full name
  `{closure@noparens.rs:2:19: 2:27}` is rustc's anonymous closure-type
  spelling — opaque, tied to source location.
- The `help:` line says "use parentheses to call this closure" — the
  missing step. `add_one(5)` is what yields the `u32`.

## Mental Model Delta

- *Before:* "Functions live at the top level of a file as `fn name(p: T) -> R { ... }`
  blocks (lessons 008/020/021). Calls go `name(value)` with parens. The
  right side of `let` is values like literals or arithmetic results."
- *After:* "The same call-with-parens shape works on a value bound to
  `let`, when that value is a *closure literal* `|param: T| body`.
  Pipes around the parameter list, single-expression body after. The
  closure has its own type — rustc names it `closure {closure@<loc>}`
  — and you call it with parens to get the body's value out."

## Prerequisites

- Installed concepts:
  - **Lesson 008** (load-bearing): call-with-parens `name(value)`.
    Reused on a `let`-bound name.
  - **Lesson 020** (load-bearing): typed parameter `p: TYPE`. Same
    shape inside `|...|` brackets.
  - **Lesson 021** (load-bearing): functions return a value of a
    declared type. Closures return one too; here the type is inferred.
  - **Lesson 025** (load-bearing): a body's last expression *is* the
    return value when there is no `;`. Today's `x + 1` is that form.
  - **Lesson 005** (load-bearing): `let name = value;` binds. Here
    `value` is a closure literal.
  - **Lesson 080** (cited): `u32`. **Lesson 009** (cited): `+` on
    integers. **Lessons 011, 003, 002, 001** (cited):
    `println!("{}", name)`; diagnostic map; `fn main`; rustc compile+run.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is `6` and
`11`. Now write the contrast file `noparens.rs` from the second snippet
above (drop the call: `let a: u32 = add_one;`) and recompile. The
diagnostic above is what you should see, with the `note:` line spelling
`closure` as the kind of `add_one`.

## What Changed

- You can write a *closure literal* `|x: u32| x + 1` on the right of a
  `let` and bind it to a name.
- Pipes `|...|` enclose the parameter list (same `name: TYPE` shape as
  function parens). After the closing pipe comes the body. A
  single-expression body with no braces is the Book-v4 form; lesson
  025's implicit-return rule makes that expression the closure's
  return value.
- The `let`-bound name is callable with the lesson-008 parens shape:
  `add_one(5)` yields the value the body produces.
- The closure is a *value* with its own type. Rustc spells that type
  `closure {closure@<file>:<line>:<col>: <line>:<col>}`.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let triple = |n: u32| n * 3;
    let r = triple(7);
    println!("{}", r);
}
```

Run `rustc tiny.rs && ./tiny`. (a) Does it compile, and what does it
print? (b) In `triple(7)`, which token is the *argument*? In the
closure literal `|n: u32| n * 3`, which token is the *parameter name*,
and what is the closure's body?

(Answers: (a) compiles silently; prints `21`. (b) Argument is `7`;
parameter name is `n`; body is `n * 3` — a single expression with no
`;`, so its value is the closure's return value.)

## What To Ignore For Now

This lesson installs *only* the closure literal syntax and the
bind-and-call shape. Deferred:

- **Closure type inference and "first call fixes the type"** — what
  happens when the parameter type is *not* annotated. Closure sub-arc
  step 2 (audit §6).
- **Capturing outer bindings** — closures can read and modify locals
  from the enclosing scope, which is what makes them more than unnamed
  functions. Closure sub-arc step 3.
- **The `Fn` / `FnMut` / `FnOnce` traits** — three traits classifying
  closures by how they capture. Steps 4-5.
- **Passing closures to functions** as parameters; `impl Fn` /
  `Box<dyn Fn>` return forms. Deferred.
- **The opaque type name `{closure@<loc>}`** itself — what kind of
  type that is, why it cannot be written by hand, whether two closures
  with identical bodies share a type. Deferred.
- **Closure body as a block** with `{ ... }` (Book v2/v3 forms). Today
  uses only the brace-free single-expression body.

## Evidence

See `../evidence/142-closure-literal-bound-and-called.md`.
