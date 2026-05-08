---
id: 076-array-literal-and-type
status: accepted
evidence: ../evidence/076-array-literal-and-type.md
---

# Build an array with `[v1, v2, ...]`, type it `[T; N]`, repeat-init it `[v; N]`

## The Move

A Rust *array* is a fixed-length, *homogeneous* bundle of values —
the count of elements is part of the type, every element shares the
same type. Three syntactic shapes co-install today, because none is
observable on its own:

- *Array literal*: square brackets, comma-separated values:
  `[1, 2, 3, 4, 5]`. All elements must share one type.
- *Array type*: square brackets, the element type, semicolon,
  length: `[i32; 5]` reads "five-element array of `i32`." Plugs
  into the lesson-019 `: TYPE` slot.
- *Repeat-init*: square brackets, one element value, semicolon,
  length: `[0; 4]` builds `[0, 0, 0, 0]`. The Book calls this "the
  same as writing `let a = [3, 3, 3, 3, 3];` but in a more concise
  way."

To witness an array's length without other new machinery, call
`.len()` on it (lesson 040's `value.method()` shape). The Book at
line 322 draws the contrast that frames today: "Unlike a tuple,
every element of an array must have the same type." Today is the
homogeneous sibling of lesson 072's tuple.

## Mental Model Delta

- *Before:* "Lesson 072 gave me one *compound* type — the tuple,
  written with parens-and-commas, element types allowed to differ.
  I have no syntax for the matched-type case."
- *After:* "Rust has a second primitive compound type — the
  *array*. The Book pairs them: tuples are *heterogeneous* and use
  parens-and-commas; arrays are *homogeneous* and use
  square-brackets-and-commas. The array's length is part of its
  *type* — `[i32; 5]` and `[i32; 6]` are different types. Three
  syntactic shapes co-install: literal `[v1, v2, ...]`, type
  `[T; N]`, repeat-init `[v; N]`."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs` then `./name`; rustc silent
    on success.
  - Lesson 003 (load-bearing): the four-part diagnostic map. The
    contrast probe is read with that map.
  - Lesson 005 (load-bearing): `let name = value;`. Today puts an
    array value on the right.
  - Lesson 019 (load-bearing): `name: TYPE`. Today plugs
    `[i32; 5]` into the `TYPE` slot — same shape lessons 033,
    062, 074 extended with simpler type names.
  - Lesson 040 (load-bearing): `value.method()`. Today calls
    `.len()` on each array binding.
  - Lesson 072 (load-bearing): tuples — the heterogeneous
    compound type with parens-and-commas. Today is the
    homogeneous sibling, exactly the Book's line-322 contrast.
  - Lessons 029, 011 (cited): `()` as the 0-arity tuple priming
    the compound-types family; positional `{}` printing reused.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`:

```rust
fn main() {
    let nums = [1, 2, 3, 4, 5];
    let typed: [i32; 5] = [10, 20, 30, 40, 50];
    let zeros = [0; 4];
    println!("nums.len() = {}", nums.len());
    println!("typed.len() = {}", typed.len());
    println!("zeros.len() = {}", zeros.len());
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
nums.len() = 5
typed.len() = 5
zeros.len() = 4
```

Three array forms in three `let` lines. Line 1 is the *literal*
form with no annotation; rustc infers `[i32; 5]` because there
are five integer-literal elements (integer literals default to
`i32`, lesson 019). Line 2 is the *type* form: the annotation
`[i32; 5]` plugs into the lesson-019 `: TYPE` slot — element type
before the semicolon, length after. Line 3 is the *repeat-init*
form, equivalent to `[0, 0, 0, 0]`. Punctuation matters: `[0, 4]`
(*comma*) is a two-element literal, not a repeat-init.

Each `.len()` returns the length the type names — `5`, `5`, `4`.
The printed number matches the count baked into the type, which is
the operational witness that arrays know their length.

Now the contrast. Save `broken.rs`:

```rust
fn main() {
    let a = [1, 2.5];
    println!("{}", a.len());
}
```

The only change is one element type. `[1, 2.5]` mixes an integer
literal `1` with a floating-point literal `2.5`. Compile it. Read
the headline with the lesson 003 map; full transcript in
`## Evidence`:

```text
error[E0308]: mismatched types
 --> broken.rs:2:17
  |
2 |     let a = [1, 2.5];
  |                 ^^^ expected integer, found floating-point number
```

The caret underlines `2.5` with the annotation `expected integer,
found floating-point number`. rustc fixed the element type from the
*first* element (an integer) and then rejected the second because
it does not match. That is the *homogeneous* claim, witnessed by
rustc at compile time.

## What Changed

- A second primitive compound type, alongside the tuple: the
  *array*. The Book pairs them under "Compound Types" (line 249-251):
  tuple = parens, mixed types; array = square brackets, one type.
- Three co-installed shapes: literal `[v1, v2, ...]`, type `[T; N]`
  (in the lesson-019 `: TYPE` slot), repeat-init `[v; N]`.
- The length is part of the type. `[i32; 5]` and `[i32; 6]` are
  different types; `.len()` returns the length the type names.
- Mixed element types fail at compile time. `[1, 2.5]` fires
  `error[E0308]: mismatched types` with `expected integer, found
  floating-point number` — homogeneity enforced from the first
  element on.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let a = [10, 20, 30];
    let b: [i32; 4] = [7; 4];
    println!("a.len() = {}", a.len());
    println!("b.len() = {}", b.len());
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) What is the type of `a`?

(c) What three-line literal would `[7; 4]` be equal to if you spelled
it out element by element?

(d) If you changed line 2 to `let a = [10, 20, 3.5];`, would
`rustc tiny.rs` still accept the program? Why or why not?

(Answers: (a) Yes; prints `a.len() = 3` then `b.len() = 4`. (b)
`[i32; 3]` — three integer literals, default `i32`. (c)
`[7, 7, 7, 7]`, per the Book line 384. (d) No; rustc fires
`error[E0308]: mismatched types` with `expected integer, found
floating-point number`, same shape as the contrast probe.)

## What To Ignore For Now

Today installs only the three array shapes and the
homogeneous-vs-heterogeneous contrast with tuples. Real and
deferred:

- *Array element access* `a[i]` — Book Ch1-3 closure queue item
  D, the explicit next move.
- *`usize` as a centered typed name* — used here only in passing
  (return type of `.len()`, type of the `N` slot). A typed-name
  install is queue D's sibling, or queue G's integer family.
- *Out-of-bounds runtime panic* — `a[i]` with `i >= a.len()`
  panics with `index out of bounds: the len is N but the index is
  M`. Queue item E; needs D first.
- *`for element in array` iteration* — queue item F.
- *Slices* `&a[..]` and the type `[T]` / `&[T]`. `.len()` is
  actually defined on slices (`primitive.slice.md`); arrays
  *coerce* to slices, which is why the method works.
- *`Vec<T>`* — the heap-allocated growable cousin the Book
  contrasts at lines 340-344. Different type, different mechanics.
- *`Debug` formatting `{:?}`* — needed to print an array's
  elements directly. Today uses `.len()` because arrays do not
  implement `Display` for `{}`.
- *Stack vs heap* as a typed concept — Book line 336-338 forwards
  to chapter 4.
- *Multidimensional arrays* `[[T; N]; M]`; *array patterns in
  `match`*; *the `.iter()` method*.
- *Constant-expression rule for `N`* — Reference `types/array.md`
  line 18 says `N` must be a `usize` constant expression. Today's
  lengths are plain integer literals, so the rule does not bite.

## Evidence

See `../evidence/076-array-literal-and-type.md`.
