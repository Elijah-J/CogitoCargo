---
id: 052-result-enum-and-is-ok
status: accepted
evidence: ../evidence/052-result-enum-and-is-ok.md
---

# `Result<i32, i32>` with `Ok` / `Err` constructors and `.is_ok()`

## The Move

Three closely-coupled facts install together. (1) Lesson 051's
`Ordering` had three variants that were each just a bare name. A
variant can also *carry a value* with it; in the declaration this is
written `Variant(T)`, and at use sites `Variant(value)` is called like
a function — the call wraps `value` and produces a value of the enum
type. Variants written this way are *constructors* of the enum.
(2) The standard library's `Result<T, E>` is the canonical example —
its declaration is `pub enum Result<T, E> { Ok(T), Err(E) }`. The
angle-bracketed `<T, E>` are *type parameters*: placeholders that get
filled in at each use site. Writing `Result<i32, i32>` says "Ok carries
an `i32` and Err also carries an `i32`." `Result` (along with `Ok` and
`Err`) is in the *prelude*, so no `use` line is needed.
(3) `.is_ok()` is a method (lesson 040) on `Result<T, E>` that returns
`true` when the value is an `Ok(_)` and `false` when it is an `Err(_)`
— the simplest inspection that does not require taking the payload
back out.

```rust
fn parity(n: i32) -> Result<i32, i32> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(n)
    }
}

fn main() {
    let a = parity(4);
    let b = parity(7);
    println!("a is ok: {}", a.is_ok());
    println!("b is ok: {}", b.is_ok());
}
```

`parity` returns `Result<i32, i32>` (lesson 021's return-type slot,
filled with a new typed name). The `if`-expression (lesson 026)
chooses which branch's value becomes the function's return value.
`Ok(n)` and `Err(n)` are *call expressions* that build a
`Result<i32, i32>` by wrapping `n`. `a.is_ok()` and `b.is_ok()` apply
the method to each binding; each returns a `bool`.

## Mental Model Delta

- Before: "Variants are bare names (lesson 051's `Ordering::Less`)."
- After: "A variant can carry a payload, written `Variant(T)` in the
  declaration; at use sites `Variant(value)` is a call expression that
  wraps `value` into a value of the enum type. `Result<T, E>` is the
  prelude's two-variant enum (`Ok(T)`, `Err(E)`); `<T, E>` are *type
  parameters* the user fills in (e.g. `Result<i32, i32>`). `.is_ok()`
  returns `true` for `Ok`, `false` for `Err`."

## Prerequisites

- Installed concepts:
  - Lesson 051 (load-bearing): the *enum*/*variant* nouns. Today
    extends "variant is a bare name" to "variant can carry a payload,
    written `Variant(T)`."
  - Lesson 040 (load-bearing): dot-form method call
    `value.method(args)`. `.is_ok()` is this shape with no arguments.
  - Lesson 021 (load-bearing): function return-type slot `-> RTYPE`.
    Today fills it with `Result<i32, i32>`.
  - Lesson 026: `if cond { a } else { b }` as an expression. The
    function body is exactly that.
  - Lesson 037: `%` remainder; lesson 013: `==`. Together: `n % 2 == 0`.
  - Lesson 020: parameter `name: TYPE`. Lesson 019: `let name: TYPE`.
  - Lessons 001, 002, 005, 008, 012: rustc + run, `fn main`, `let`,
    function calls, `bool`.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

In a fresh empty directory, create `demo.rs` with the source above.
Compile and run:

```console
$ rustc demo.rs
$ ./demo
a is ok: true
b is ok: false
```

Walk it. `parity(4)`: `4 % 2 == 0` is `true`, so the function
evaluates `Ok(4)` — a call to the `Ok` constructor that wraps `4` in
a `Result<i32, i32>`. The function returns that value, and
`let a = parity(4);` binds `a` to it. `parity(7)`: `7 % 2 == 0` is
`false`, so the `else` arm fires and the function returns `Err(7)`,
also a `Result<i32, i32>`. Then `a.is_ok()` returns `true` (because
`a` was built by `Ok`); `b.is_ok()` returns `false` (because `b` was
built by `Err`). Each `bool` is interpolated into a `println!` slot.

(Optional contrast.) Save `broken.rs`:

```rust
fn main() {
    let n: i32 = Ok(7);
    println!("{}", n);
}
```

Compile it. The headline reads
`error[E0308]: mismatched types`, and the source-excerpt label says
`expected \`i32\`, found \`Result<{integer}, _>\``. Same E-code as
lessons 024-034, 045-048. The contrast confirms that `Ok(7)` is *not*
an `i32` — it is a `Result<T, E>` value whose `T` rustc has inferred
to be an integer. The `_` for `E` is rustc's placeholder for "type
parameter could not be inferred from this context alone." (Full
transcript in `## Evidence`.)

## What Changed

- A variant declared as `Variant(T)` is a *constructor*: at use sites
  `Variant(value)` is a call expression that wraps `value` and
  produces a value of the enum type.
- `Result<T, E>` is the prelude's two-variant enum: `Ok(T)` for the
  success payload, `Err(E)` for the failure payload. No `use` line is
  needed.
- `<T, E>` in `Result<T, E>` are *type parameters*. At a use site you
  fill them in: `Result<i32, i32>` says both payloads are `i32`.
- `.is_ok()` is the simplest inspection method on `Result<T, E>`: it
  returns `true` for an `Ok` value and `false` for an `Err` value.
  Sibling `.is_err()` exists with the opposite truth values; not
  exercised today.
- A function can return `Result<T, E>` via the lesson-021 `-> RTYPE`
  slot. The function body decides per call which variant to build.

## Check Yourself

You write `tiny.rs`:

```rust
fn classify(n: i32) -> Result<i32, i32> {
    if n >= 0 {
        Ok(n)
    } else {
        Err(n)
    }
}

fn main() {
    let r = classify(-3);
    println!("{}", r.is_ok());
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) If you change `classify(-3)` to `classify(0)`, what does it print?

(c) Without recompiling, predict the value of `r.is_ok()` for
`classify(5)`.

(Answers: (a) Yes; prints `false`, since `-3 >= 0` is false so the
`Err(-3)` arm fires, and `.is_ok()` returns `false` for `Err`.
(b) Prints `true`, since `0 >= 0` is true so `Ok(0)` is returned.
(c) `true`, since `5 >= 0` is true so `Ok(5)` is built and
`.is_ok()` returns `true` for `Ok`.)

## What To Ignore For Now

- *Match on payload variants* `match r { Ok(v) => ..., Err(e) => ... }` — future move; today uses `.is_ok()` instead.
- *`.expect("msg")` and `.unwrap()`* — extract the `Ok` payload by panicking on `Err`. Direct successor.
- *The `Option<T>` enum* — sibling of `Result` with one type parameter.
- *Generics in general* — `<T>` on functions, where-clauses, trait bounds. Today surfaces only `Result<i32, i32>`.
- *The `?` operator* for error propagation.
- *Other `Result` methods* — `.is_err()` (mentioned only), `.unwrap_or`, `.map`, `.and_then`, `.is_ok_and`, `.ok()`, `.err()`.
- *Runtime panics* — `.is_ok()` returns `bool`; no panic path today.
- *Custom error types* (`enum MyError { ... }`, `Box<dyn Error>`).
- *`Display` / `Debug` formatting on `Result`* — the probe uses `.is_ok()` so only `bool` reaches `{}`.
- *`io::Result<T>` type alias* — used by `read_line`. Direct successor.
- *Trait machinery on `Result`* (`Eq`, `Debug`, etc.); *`&str`* as a typed name; all previously deferred items.

## Evidence

See `../evidence/052-result-enum-and-is-ok.md` for the corpus-quote
map, the rustc / system toolchain string, the working-probe transcript,
the broken-contrast E0308 transcript, and the prerequisite-claim
summary.
