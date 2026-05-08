---
id: 077-array-indexing-and-usize
status: accepted
evidence: ../evidence/077-array-indexing-and-usize.md
---

# Read an array element with `a[i]`, with `i` typed `usize`

## The Move

Lesson 076 built arrays but never read one back out. Today installs
the read: write the array-typed value, then square brackets
containing one index expression. `nums[0]` returns the first element,
`nums[1]` the second. Indices are *zero-based* — counting starts at
`0`, so the last slot of an N-element array is `a[N - 1]`.

The index has to be of type `usize`. The Reference states it
directly: "Array and slice-typed values can be indexed by writing a
square-bracket-enclosed expression of type `usize` (the index) after
them." For literal indexes like `nums[0]`, rustc infers `usize`. As
soon as you use a *named binding* as the index, that binding's type
matters.

`usize` is the third typed integer this run installs as a centered
name, joining `i32` (lesson 019) and `u32` (lesson 062). It is
*unsigned* (the `u`-prefix family) so it starts at `0`, and
*architecture-dependent*: 64 bits on a 64-bit machine, 32 bits on a
32-bit machine. The Book: "the primary situation in which you'd use
`isize` or `usize` is when indexing some sort of collection." That
is exactly today's situation.

`a[i]` produces a value of the array's *element type* `T` (from
`[T; N]`), fit for the right of `let` and for `{}` printing.

## Mental Model Delta

- *Before:* "I can build an array (lesson 076) and ask its length
  with `.len()`, but the elements are opaque — I have no syntax to
  read them back out."
- *After:* "I read element `i` of array `a` with `a[i]`. The brackets
  follow the array value, like the dot follows a receiver in lesson
  040. Indices count from `0`. The index has to be of type `usize` —
  the unsigned, architecture-sized integer type whose explicit role
  is *indexing collections*. For a named index, write
  `let i: usize = ...;`."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002: `rustc file.rs` then `./name`; rustc silent
    on success.
  - Lesson 003 (load-bearing): the four-part diagnostic map. The
    contrast probe is read with that map.
  - Lesson 005 (load-bearing): `let name = value;`. Today binds five
    names: an array, two literal-indexed reads, an index, and a
    variable-indexed read.
  - Lesson 019 (load-bearing): `let name: TYPE = value;`. Today
    plugs `usize` into the `TYPE` slot, the same shape lessons 033,
    062, 074 used with `f64`, `u32`, `char`.
  - Lesson 062 (load-bearing): `u32` as the unsigned counterpart to
    `i32`. `usize` is the same family — the Book's *Integer Types*
    table puts `i32`, `u32`, and `usize` in the same column
    structure, with `usize` on the *Architecture-dependent* row.
  - Lesson 076 (load-bearing): the array literal and `[T; N]` type
    today indexes. The probe builds `let nums = [10, 20, 30, 40,
    50];` exactly as 076 taught.
  - Lesson 011 (cited): `println!("{}", expr)` for the three output
    lines.
- Ordinary computer-use assumptions: same as lesson 001 (terminal,
  plain-text editor, `rustc` on `PATH`, Linux/macOS shell).

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`:

```rust
fn main() {
    let nums = [10, 20, 30, 40, 50];
    let first = nums[0];
    let second = nums[1];
    let i: usize = 2;
    let middle = nums[i];
    println!("first = {}", first);
    println!("second = {}", second);
    println!("middle = {}", middle);
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
first = 10
second = 20
middle = 30
```

Three reads. Lines 2-3 use *literal* indexes: rustc infers `usize`
for `0` and `1`. Line 4 declares `let i: usize = 2;` — the `: usize`
annotation in the lesson-019 slot. Line 5 reads `nums[i]`. The
output matches zero-based counting: index `0` → `10`, index `1` →
`20`, index `2` → `30`.

Now the contrast. Save `broken.rs`:

```rust
fn main() {
    let nums = [10, 20, 30, 40, 50];
    let i: i32 = 2;
    let x = nums[i];
    println!("x = {}", x);
}
```

Only the index variable's type changed: `usize` → `i32`. Read the
headline with the lesson-003 map; full transcript in `## Evidence`:

```text
error[E0277]: the type `[{integer}]` cannot be indexed by `i32`
 --> broken.rs:4:18
  |
4 |     let x = nums[i];
  |                  ^ slice indices are of type `usize` or ranges of `usize`
```

The caret underlines `i`. rustc rejects it because its type is
`i32`; the inline gloss restates the rule the Reference quotes:
indices are of type `usize`. The `E0277` code and the trait talk
below it (`SliceIndex`, `Index<i32>`) are the structural reason —
leave that machinery for later. The sibling `: u32` would also
fail. Only `usize` fits.

## What Changed

- New expression form: `array_value[index_expr]`. Square brackets
  follow the array value, like the dot follows a receiver in lesson
  040.
- *Indices are zero-based.* For `[10, 20, 30, 40, 50]`, `a[0]` is
  `10`. The last index of an N-element array is `N - 1`.
- *The index must be `usize`.* rustc infers it for literals; for a
  named index, write `let i: usize = ...;`. `i32` does not fit —
  the contrast probe fires `error[E0277]: the type `[{integer}]`
  cannot be indexed by `i32``.
- *`usize` is your third typed integer*, after `i32` and `u32`.
  Unsigned, architecture-dependent (64 bits on 64-bit, 32 bits on
  32-bit), explicitly named by the Book as the indexing type.

## Check Yourself

You write `tiny.rs`:

```rust
fn main() {
    let xs = [100, 200, 300];
    let last = xs[2];
    let k: usize = 0;
    let head = xs[k];
    println!("head = {}, last = {}", head, last);
}
```

You run `rustc tiny.rs && ./tiny`.

(a) Does it compile? What does it print?

(b) What index would you put in the brackets to read `200`?

(c) If you replaced line 4 with `let k: u32 = 0;`, what
diagnostic family would rustc fire? (Hint: same family as the
contrast probe, just with a different type name in the headline.)

(d) `xs.len()` returns `3`. Why is `xs[3]` not a valid read?

(Answers: (a) Yes; prints `head = 100, last = 300`. (b) `1` —
zero-based, so `200` is at index `1`. (c) `error[E0277]: the type
`[{integer}]` cannot be indexed by `u32``, same *slice indices are
of type `usize`* gloss. Sibling unsigned type, still not `usize`.
(d) Valid indices for a 3-element array are `0`, `1`, `2`. Index
`3` is one past the end; what happens then is queue item E —
deferred today.)

## What To Ignore For Now

Today installs only `a[i]`, the zero-based rule, and `usize` as the
index type's centered name. Real and deferred:

- *Out-of-bounds indexing* — queue item E; needs today's `a[i]`
  first. With a constant index past the end, rustc on this release
  fires `error: this operation will panic at runtime`; with a
  runtime index past the end, the program panics with `index out
  of bounds: the length is N but the index is M`.
- *Negative literal indexes* — `a[-1]` fires "negative integers
  cannot be used to index". Auxiliary, not centered today.
- *`for element in array` iteration* — queue item F.
- *Slice indexing with a range* — `&a[1..3]` returns a slice, not
  one element. The index slot also accepts *ranges of `usize`*
  per the contrast probe's gloss.
- *The `Index` and `IndexMut` traits* — the structural reason
  `a[i]` works. Trait machinery is deferred since lesson 040.
- *Multidimensional indexing* — `b[1][2]`. Composes today's move
  with itself; deferred with multidimensional arrays.
- *Mutable element assignment* — `a[i] = v;`. Needs `let mut` on
  the array; separate move.
- *`isize`* — the signed sibling. Indexing wants `usize`.
- *The full integer family* — Table 3-1's remaining variants.
  Queue item G.
- *The `usize` literal suffix `0usize`* — not used here; the probe
  uses the `: usize` annotation instead.

## Evidence

See `../evidence/077-array-indexing-and-usize.md`.
