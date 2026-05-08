---
id: 107-vec-basics
status: accepted
evidence: ../evidence/107-vec-basics.md
---

# Build a `Vec<T>` with `vec![]`, ask `.len()`, read `v[i]`

## The Move

Lesson 093 named `Vec<T>` as a member of the standard library prelude
but never built one. Today installs three coupled pieces — *build*
with the `vec![]` macro, *measure* with `.len()`, *read* with `v[i]`.

```rust
fn main() {
    let empty: Vec<u64> = vec![];
    let three: Vec<u64> = vec![10, 20, 30];
    println!("empty.len() = {}", empty.len());
    println!("three.len() = {}", three.len());
    println!("three[0] = {}", three[0]);
    println!("three[2] = {}", three[2]);
}
```

`rustc demo.rs` compiles silently. `./demo` prints `empty.len() = 0`,
`three.len() = 3`, `three[0] = 10`, `three[2] = 30`.

The std `macro.vec` docs name two working forms — `vec![1, 2, 3]` for
a list of values and `vec![]` for the empty case (the macro's `() =>
{ ... }` rule). The `!` is lesson 071's macro mark. The `Vec<u64>`
annotation pins the element type explicitly; without it `vec![]`
cannot infer `T`, and `vec![10, 20, 30]` would default to `Vec<i32>`
(lesson 080). For today's probe the annotation keeps the syntax
visible.

The std docs give `pub const fn len(&self) -> usize` "Returns the
number of elements in the vector." Same method-call shape as lesson
040, same `usize` return as the array `.len()` from lesson 076.

For indexing, the std `vec/struct.Vec` page leads with "The `Vec`
type allows access to values by index" and the example `let v =
vec![0, 2, 4, 6]; println!("{}", v[1]);` — exactly today's shape,
same `usize` index requirement as lesson 077's `a[i]`.

## Mental Model Delta

- *Before:* "`Vec<T>` is a prelude name (lesson 093). I have not
  constructed one or read an element."
- *After:* "I build a `Vec<T>` with the `vec![]` macro — `vec![1, 2,
  3]` prefilled, `vec![]` empty. `v.len()` returns the element count
  as `usize`. `v[i]` with `i: usize` reads element `i`.
  Syntactically these are the same shapes lessons 076 and 077 used
  for arrays — only the type underneath changed."

## Prerequisites

- Installed concepts:
  - Lesson 093 (load-bearing): `Vec<T>` is a prelude member, written
    bare with no `use` line.
  - Lesson 077 (load-bearing): `a[i]` with `i: usize`. Today reuses
    that shape — same brackets, same `usize` requirement, same E0277
    on a non-`usize` named index.
  - Lesson 071 (load-bearing): macro invocation `name!(...)`. `vec!`
    is one; today uses the bracket-flavored `vec![...]` form.
  - Lesson 040 (load-bearing): `value.method()`. `.len()` is that
    shape on a `Vec<T>` receiver.
  - Lesson 019 (load-bearing): the `: TYPE` annotation slot. Today
    plugs in `Vec<u64>`.
  - Lesson 076 (cited): `.len()` was first installed on arrays.
  - Lesson 080 (cited): `u64` as the element type in the probe — one
    of the twelve integer types that lesson named.
  - Lesson 078 (cited): the panic shape `index out of bounds: the
    len is N but the index is M` for the contrast probe.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`
with the source above. Compile and run:

```console
$ rustc demo.rs
$ ./demo
empty.len() = 0
three.len() = 3
three[0] = 10
three[2] = 30
```

Now the contrast — `Vec<T>` vs array on out-of-bounds. Save
`broken.rs`:

```rust
fn main() {
    let three: Vec<u64> = vec![10, 20, 30];
    let bad = three[5];
    println!("bad = {}", bad);
}
```

Compile and run:

```console
$ rustc broken.rs
$ ./broken
$ echo $?
101
```

`rustc broken.rs` exits `0` and is silent — *the program compiled*.
`./broken` produces no `stdout`. On `stderr`:

```text
thread 'main' (...) panicked at broken.rs:3:20:
index out of bounds: the len is 3 but the index is 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Read with the lesson 003 map. The message `index out of bounds: the
len is 3 but the index is 5` is exactly the shape lesson 078
installed for arrays — `N` is `three.len()`, `M` is the failed
index. Wrapper lines and exit `101` are lesson 053's panic trailer.

One pedagogical surprise. Lesson 077's auxiliary `nums[10]` on a
`[i32; 5]` fired *at compile time* via the `unconditional_panic`
lint, because the array's length lives in the type. `Vec<u64>` does
not say "three" — its length is not part of the type. So rustc
cannot constant-evaluate the bounds, and the same shape rejects only
at runtime. Same message, different timing.

## What Changed

- *Construction:* `vec![]` builds an empty `Vec<T>`; `vec![v1, v2,
  ...]` builds one prefilled. The `Vec<T>` annotation declares the
  element type for the empty form.
- *Length:* `v.len()` returns the element count as a `usize` — same
  shape and return type as the array `.len()` from lesson 076.
- *Read:* `v[i]` with `i: usize` returns element `i`. Same syntax
  and same panic message as lesson 077/078 for arrays.
- *Bounds-check timing differs from arrays.* Arrays carry length in
  the type, so constant-out-of-bounds is rejected at compile time.
  `Vec<T>` does not — the same shape rejects only at runtime.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let xs: Vec<i32> = vec![100, 200, 300];
    let ys: Vec<i32> = vec![];
    println!("xs.len() = {}", xs.len());
    println!("ys.len() = {}", ys.len());
    println!("xs[1] = {}", xs[1]);
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) What index would you put in the brackets to read `300`?

(c) You replace `xs[1]` with `ys[0]` and rerun. What happens at
compile time? At runtime?

*(Answers: (a) Yes; prints `xs.len() = 3`, `ys.len() = 0`, `xs[1] =
200`. (b) `2` — zero-based. (c) Compiles silently. At runtime
panics with `index out of bounds: the len is 0 but the index is 0`
— an empty vector has no legal indices.)*

## What To Ignore For Now

Today installs only `vec![]`/`vec![v1, ...]` for construction,
`.len()`, and `v[i]`. Real and deferred:

- *`Vec::new()`* — associated-function constructor; equivalent to
  `vec![]` for the empty case via the lesson-041 qualified shape.
- *`v.push(x)`* — Vec's most common mutating method; needs `let
  mut`. Future move.
- *`Vec::with_capacity(n)`* — preallocate; capacity vs length is
  its own concept.
- *Iteration* — `for x in &v`, `v.iter()`, `v.iter_mut()`,
  `.enumerate()`, `.zip()`, `.map`/`.filter`/`.collect`. Lesson 079
  installed `for x in array`; the Vec analogue is its own move.
- *Slicing* `&v[a..b]`, *`v.as_slice()`*, range indexing — separate
  move from single-element indexing.
- *Other mutating methods* — `pop`, `resize`, `reserve`, `set_len`
  (`unsafe`), `clear`, `truncate`, `fill`.
- *`v.get(i)`* — the non-panicking sibling returning `Option<&T>`;
  lesson 078 already named the array version.
- *Type-parameter inference for `vec![v1, ...]`* — usually rustc
  infers `T` from the elements; today's probe annotates explicitly.
- *Equality* `v1 == v2`, *`Vec<T>` as a parameter or return type*,
  *heap allocation and growth strategy*, *the `vec![elem; n]`
  repeating form* — all out of today's three pieces.

(Today's three pieces unlock reading rmp's `biguint/basic.rs` trio —
`BigUInt::zero` returns a value built with `vec![]`,
`BigUInt::is_zero` checks `self.limbs.len() == 0`, and
`BigUInt::num_bits` reads `self.limbs[n - 1]`.)

## Evidence

See `../evidence/107-vec-basics.md`.
