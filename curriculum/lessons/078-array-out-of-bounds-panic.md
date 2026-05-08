---
id: 078-array-out-of-bounds-panic
status: accepted
evidence: ../evidence/078-array-out-of-bounds-panic.md
---

# An out-of-bounds `a[i]` panics at runtime with `index out of bounds: the len is N but the index is M`

## The Move

Lesson 077 installed `a[i]` and flagged the queue-E follow-up:
what happens when `i` is a valid `usize` but too big for the
array — `i >= a.len()`. Today closes that follow-up.

When `a` has length `N`, the only legal values for `i` in `a[i]`
are `0, 1, ..., N - 1`. Any other `usize` is out of bounds. Rust
does not silently read past the end. Each time `a[i]` evaluates,
rustc has inserted a *bounds check* that compares `i` against
`a.len()`, and *if `i >= a.len()` the program panics* — same panic
mechanism lesson 053 installed.

A wrinkle: lesson 077's auxiliary fed rustc a constant index
`nums[10]`, which rustc evaluated at compile time and rejected.
To see the *runtime* panic, the index must come from somewhere
rustc cannot constant-evaluate. Lesson 056 gives us a tool:
`"10".parse().expect("...")` produces a `usize` from a string at
runtime. With that in the index slot, the program compiles
silently and the panic only fires when `a[bad_index]` evaluates.

## Mental Model Delta

- *Before:* "I can read `a[i]` and I know `i` has to be a `usize`
  (lesson 077). I don't know what happens if `i` is too big."
- *After:* "Indexing is *bounds-checked at runtime*. Each `a[i]`
  compares `i` to `a.len()`; if `i >= a.len()` the program panics
  with `index out of bounds: the len is N but the index is M`.
  Compile still succeeds for a runtime-built index. Rust does not
  let invalid indexing read past the array's memory; it terminates
  instead."

## Prerequisites

- Installed concepts:
  - Lessons 001, 002, 005 (load-bearing): `rustc` + `./name`;
    `let name = value;`.
  - Lesson 019 (cited): the `: TYPE` slot.
  - Lesson 053 (load-bearing): the *panic* concept and the panic
    trailer (`thread 'main' ... panicked at file:line:col:`,
    `note: ... RUST_BACKTRACE=1 ...`, exit code `101`, output on
    stderr). Reused unchanged.
  - Lesson 056 (load-bearing): `"...".parse().expect(...)` driven
    by a typed binding's annotation. Reused to build a runtime
    `usize`.
  - Lesson 076 (load-bearing): the array literal and `.len()`.
  - Lesson 077 (load-bearing): `a[i]` and `usize` as the index
    type. Today extends 077 with the *bounds rule*.
  - Lesson 011 (cited): `println!("{}", expr)`.
- Ordinary computer-use assumptions: same as lesson 001, plus the
  stdout-vs-stderr and `echo $?` knowledge already used by lesson
  053.

## Try It

Make a fresh empty directory, `cd` into it, and create `demo.rs`:

```rust
fn main() {
    let nums = [10, 20, 30, 40, 50];
    let bad_index_str = "10";
    let bad_index: usize = bad_index_str.parse().expect("not a number");
    let element = nums[bad_index];
    println!("element = {}", element);
}
```

Compile and run:

```console
$ rustc demo.rs
$ ./demo
$ echo $?
101
```

`rustc demo.rs` exits `0` and is silent: the program compiled and
`demo` was produced. `./demo` then produces no `stdout` — the
`println!` on line 6 never runs. On `stderr`:

```text
thread 'main' (...) panicked at demo.rs:5:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The message line: `index out of bounds: the len is 5 but the index
is 10`. `5` is `nums.len()`. `10` is `bad_index`. The condition the
bounds check failed was `10 >= 5`. The location `demo.rs:5:19`
points at `nums[bad_index]` — the *moment* the bad indexing
expression evaluated, not the line where `bad_index` was built.
Wrapper lines and the `101` exit code are exactly lesson 053's
panic trailer.

Now the in-bounds contrast. Same source, change one character on
line 3: `"10"` → `"2"`. Recompile and run:

```console
$ rustc demo.rs
$ ./demo
element = 30
$ echo $?
0
```

Same `.parse().expect(...)` machinery — but now `2 < 5`, the
bounds check passes, `nums[2]` reads `30`, and `println!` runs.

## What Changed

- *`a[i]` is bounds-checked at runtime.* If `i >= a.len()`, the
  thread panics.
- *The panic message names the failure precisely.* `index out of
  bounds: the len is N but the index is M` — `N` is `a.len()`,
  `M` is the index that failed.
- *Compile succeeds when the index is a runtime value.* Lesson
  077's `nums[10]` was rejected at compile time because the index
  was a literal rustc could evaluate. Today's `.parse()`-built
  index reaches `bad_index` only at runtime.
- *The panic stops the program.* Stdout is empty; the diagnostic
  is on stderr; exit status is `101`.

## Check Yourself

You have the working `demo.rs` from *Try It*.

(a) Predict: change `"10"` on line 3 to `"5"`. Does this run
cleanly or panic? What would the message say?

(b) Predict: change `"10"` to `"4"`. What does `./demo` print, and
what is the exit code?

(c) Why does *today's* probe compile while lesson 077's auxiliary
`let x = nums[10];` did not?

*(Answers: (a) Panics. `5 >= 5` fails the bounds check; valid
indices for a 5-element array are `0..=4`. The line reads `index
out of bounds: the len is 5 but the index is 5`. (b) Prints
`element = 50` and exits `0`. Index `4` is the last valid slot.
(c) `nums[10]` had a literal `10` rustc could evaluate at compile
time; the `unconditional_panic` lint fired then. Today's index
goes through `.parse()`, which rustc does not constant-evaluate,
so the bounds check moves to runtime.)*

## What To Ignore For Now

Today installs only the runtime bounds-check rule and the
`index out of bounds: the len is N but the index is M` message
shape. Real and deferred:

- *`for element in array` iteration* — queue item F.
- *`a.get(i)` returning `Option<&T>`* — the *non-panicking*
  sibling: `Some(&v)` when `i < a.len()`, `None` otherwise. Needs
  `Option` and `&T` first.
- *Backtrace mechanics* — `RUST_BACKTRACE=1` / `=full`. Lesson 053
  already deferred this.
- *`std::panic::catch_unwind`* — recover from a panic on the same
  thread.
- *`get_unchecked(i)`* — an `unsafe` form that skips the bounds
  check.
- *Multi-thread panic semantics* — today has only the `main`
  thread.
- *Non-array indexing* — `Vec<T>[i]`, `HashMap<K, V>[k]`, string
  indexing. Different types.
- *Custom panic messages* — the indexing message is fixed by
  rustc, not by `.expect(msg)`.
- *Slice indexing with a range* `&a[1..3]` — separate move.
- *Negative literal indexing* `a[-1]` — compile-time rejection
  with a different diagnostic (lesson 077 Probe 4).

## Evidence

See `../evidence/078-array-out-of-bounds-panic.md`.
