---
id: 140-iterator-step-by
status: accepted
evidence: ../evidence/140-iterator-step-by.md
---

# Yield every `step`th element with `iter.step_by(step)`

## The Move

Lessons 136-139 installed four lazy adapters: `take`, `skip`,
`enumerate`, `fuse`. Today's `step_by(step)` is the next sibling.
Structurally it matches `take(n)` / `skip(n)` exactly: consuming
`self`, one `step: usize` argument, returns a wrapper struct
(`StepBy<Self>`) that itself implements `Iterator`. The new fact today
is `step_by` is the **first adapter in the run with a runtime panic
precondition**: it panics if `step` is `0`.

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];

    for x in v.iter().step_by(2) {
        println!("{}", x);
    }

    println!("---");

    let n = v.iter().step_by(2).count();
    println!("{}", n);
}
```

`rustc demo.rs` is silent; `./demo` prints:

```text
10
30
50
---
3
```

Walk: `v.iter()` produces `&10, &20, &30, &40, &50`. `.step_by(2)`
yields the element at index 0 (`&10`), then index 2 (`&30`), then
index 4 (`&50`). Index 6 is past the end, the wrapper produces
`None`, and the `for` loop stops. The chained `.count()` independently
confirms `3`.

The trait declaration spells `fn step_by(self, step: usize) ->
StepBy<Self> where Self: Sized,`
(`output/docs/rust/std/iter/trait.Iterator.md:551`). All three
structural slots reuse 136/137: consuming `self`, `step: usize`, and
the wrapper-struct return `StepBy<Self>` documented at
`struct.StepBy.md:7` (`pub struct StepBy<I> { /* private fields
*/ }`). Forced-error probe (`let _x: u32 = v.iter().step_by(2);`)
makes rustc spell the type `StepBy<Iter<'_, u64>>`. The lazy framing
also carries from 136 — but the `step != 0` check is *not* lazy. See
below.

## The new fact: `step == 0` panics at construction

The doc at `trait.Iterator.md:582-584` says only "The method will
panic if the given step is `0`." It does not specify *when*. Captured
empirically (appendix Probe 2): the panic fires inside the `step_by`
body itself, *before* any iterator state is built and *before* any
element is pulled. The same panic also fires on `let mut it = ...;
it.next();` and on the bare-statement form `v.iter().step_by(0);` —
the construction check is unconditional.

```text
thread 'main' (...) panicked at .../core/src/iter/adapters/step_by.rs:35:9:
assertion failed: step != 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
exit=101
```

Same panic shape lesson 053 captured for `Result::expect("msg")`:
exit status 101, output to stderr. The message comes from a bare
`assert!` inside std, so there is no human-written tail.

So `step_by` is *not* lazy about the `step == 0` precondition; it
*is* lazy about pulling elements. That asymmetry is the new fact.

## Mental Model Delta

- *Before:* "Adapters are lazy: building the wrapper does no work,
  and adapter calls themselves succeed silently regardless of
  argument values."
- *After:* "Adapter laziness applies to *iteration*, not to argument
  validation. `step_by(step)` is lazy about pulling elements — same
  as 136-139 — but its `step != 0` precondition is checked
  *immediately* at construction, inside the `step_by` body. This is
  the first iterator adapter in the run with a documented panic
  precondition."

## Prerequisites

- Installed concepts:
  - **Lessons 136, 137** (load-bearing): the adapter shape `(self, n:
    usize) -> Wrapper<Self>`. Substitution today: `step` for `n` and
    `StepBy` for `Take` / `Skip`.
  - **Lesson 132** (load-bearing): `Iterator` trait with 75 provided
    methods. `step_by` is one; synopsis line at `:25-26` ends in
    `{ ... }` (lesson 116's default-body marker).
  - **Lesson 131** (load-bearing): `.next()` on a slice iterator
    returns `Option<&T>` and stops at `None`.
  - **Lesson 053** (load-bearing): the runtime-panic shape — `thread
    'main' ... panicked at <file>:<line>: <message>`, stderr, exit
    101. Today's `step != 0` panic reuses that template; no new
    panic mechanic.
  - **Lessons 133, 102, 080, 049, 022, 040, 011, 005, 003, 002, 001**
    (cited): `.count()`; consuming `self`; `usize`; method chaining;
    `for x in iter`; dot-call; `println!`; `let`; diagnostic map;
    `fn main`; rustc compile + run.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is the five
lines above. Then witness the panic — save as `panic.rs`:

```rust
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let _ = v.iter().step_by(0);
    println!("never reached");
}
```

`rustc panic.rs` is silent. `./panic` prints (to stderr) a `thread
'main' ... panicked at ...step_by.rs:35:9: assertion failed: step != 0`
block; stdout is empty (`never reached` never prints); `echo $?` is
`101`. The panic fires *before* any iteration — there is no `.next()`
call in this source.

For sanity, also try `step_by(1)`: it yields every element (identity).
Try `step_by(100)` on a 3-element vec: it yields only the first.

## What Changed

- Signature `fn step_by(self, step: usize) -> StepBy<Self> where Self:
  Sized,` (`trait.Iterator.md:551`). Same shape as `take`/`skip`.
- Yields the element at index `0`, then `step`, then `2*step`, ...,
  until past the end. The doc at `:556-557`: "The first element of
  the iterator will always be returned, regardless of the step
  given." `step_by(100)` on a 3-element source yields just the first.
- `step == 0` panics at construction (not at first `.next()`). Exit
  status 101; lesson 053's panic shape applies.
- `step_by(1)` is the identity adapter — every element yielded.
- Type-pin probe names the wrapper `StepBy<Iter<'_, u64>>`.

## Check Yourself

```rust
fn main() {
    let v: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let n = v.iter().step_by(3).count();
    println!("{}", n);

    for x in v.iter().step_by(3) {
        println!("{}", x);
    }
}
```

(a) Does it compile silently? What does it print?

(b) Predict what would happen if `step_by(3)` were changed to
`step_by(0)`. Where in the source does the panic fire — at the
`step_by(0)` call expression, or at the first `.next()` driven by
the `for` loop?

*(Answers: (a) Yes. Prints `4`, then `1, 4, 7, 10`. Indices `0, 3, 6,
9` reach four elements; index 12 is past the end. (b) The panic fires
at the `step_by(0)` call itself — the std assertion lives in the
`step_by` body, not in `StepBy::next`. The `let n = ...` line on its
own would panic; the `for` loop never starts.)*

## What To Ignore For Now

Deferred: the `StepBy<I>` struct's private fields and its `next` body
(the doc at `:559-565` describes two equivalent pull strategies,
unobservable to the caller); the `Self: Sized` bound (still); the
`unused_must_use` warning rustc emits when a `StepBy` is built and
discarded as a statement (separate lint mechanic); the std source
line `step_by.rs:35` named in the panic (internal path, not user-
facing). Next move per audit §5: `size_hint` (step 11).

## Evidence

See `../evidence/140-iterator-step-by.md`.
