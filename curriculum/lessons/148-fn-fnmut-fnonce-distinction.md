---
id: 148-fn-fnmut-fnonce-distinction
status: accepted
evidence: ../evidence/148-fn-fnmut-fnonce-distinction.md
---

# `Fn`, `FnMut`, `FnOnce`: three traits for callables, picked by what the closure body does

## The Move

Lesson 147 used *one* trait from the parenthesized-bound family:
`<F: Fn(u32) -> u32>`. Today: there are *three* such traits — `Fn`,
`FnMut`, `FnOnce` — and any closure auto-implements one or more of
them based on what its body does with captured values. The bound on
the receiving function decides which closures it accepts.

```rust
fn call_fn<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
    f(x)
}

fn call_fnmut<F: FnMut(u32) -> u32>(mut f: F, x: u32) -> u32 {
    f(x)
}

fn call_fnonce<F: FnOnce(u32) -> u32>(f: F, x: u32) -> u32 {
    f(x)
}

fn main() {
    let pure = |n: u32| n + 1;
    println!("Fn:     {}", call_fn(pure, 5));

    let mut counter: u32 = 0;
    let tick = |n: u32| { counter += n; counter };
    println!("FnMut:  {}", call_fnmut(tick, 5));

    let s = String::from("hello");
    let consume = move |_: u32| { drop(s); 0_u32 };
    println!("FnOnce: {}", call_fnonce(consume, 0));
}
```

`rustc demo.rs` is silent; `./demo` prints three lines:

```text
Fn:     6
FnMut:  5
FnOnce: 0
```

Three closures, three stories about captures:

- **`pure`** captures nothing — body uses only its own parameter
  `n`. Implements `Fn`.
- **`tick`** captures `counter` (lesson 144) and *mutates* it via
  `counter += n`. Implements `FnMut`.
- **`consume`** *moves* the captured `s` out via `drop(s)` — the
  leading `move` keyword forces ownership transfer at capture time;
  the body then consumes it. Implements `FnOnce`.

The Book at `ch13-01-closures.md:437-447` packages the rule: a
closure that *moves* captured values out implements `FnOnce` only;
one that *mutates* captures (without moving them out) implements
`FnMut`; one that only *reads* captures by shared reference, or
captures nothing, implements `Fn`. The three trait pages
(`std/ops/trait.{Fn,FnMut,FnOnce}.md` line 27 each) carry the same
rule from the trait side.

## The supertrait shape

The trait headers (`Fn.md:7`, `FnMut.md:7`, `FnOnce.md:7`) read:

```text
pub trait Fn<Args>:    FnMut<Args>     { ... }
pub trait FnMut<Args>: FnOnce<Args>    { ... }
pub trait FnOnce<Args>                 { type Output; ... }
```

The `:` after the trait name names a *supertrait*: every type that
implements `Fn` also implements `FnMut`, and every `FnMut` also
implements `FnOnce`. The family is layered: `Fn` strictest, `FnOnce`
most permissive. So an `<F: FnOnce(...)>` bound accepts any closure;
`<F: FnMut(...)>` accepts FnMut and Fn closures; `<F: Fn(...)>`
accepts only Fn closures.

Probe it: `pure` (Fn) passes to all three call helpers. `tick`
(FnMut) passes to `call_fnmut` and `call_fnonce` but not `call_fn`:

```text
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
  --> wrong_trait.rs:10:16
   |
10 |     let tick = |n: u32| { counter += n; counter };
   |                ^^^^^^^^   ------- closure is `FnMut` because it mutates the variable `counter` here
```

The lesson 003 map applies. New code `E0525`. rustc names `counter`
inline as the variable whose mutation forced `tick` into FnMut.

## The `mut f: F` side-fact for FnMut bounds

Notice that `call_fnmut`'s parameter is `mut f: F` rather than
`f: F`. Drop the `mut` and rustc rejects the body:

```text
error[E0596]: cannot borrow `f` as mutable, as it is not declared as mutable
 --> no_mut.rs:6:5
  |
6 |     f(x)
  |     ^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
5 | fn call_fnmut<F: FnMut(u32) -> u32>(mut f: F, x: u32) -> u32 {
  |                                     +++
```

Why? Each trait's required method has a different receiver shape
(`Fn.md:14`, `FnMut.md:14`, `FnOnce.md:16`):

```text
Fn:     fn call(&self, args: Args)         // shared borrow
FnMut:  fn call_mut(&mut self, args: Args) // mutable borrow
FnOnce: fn call_once(self, args: Args)     // by-value, consuming
```

Calling `f(x)` invokes whichever method the trait declares. For
`FnMut`, that is `call_mut(&mut self, ...)` — the call takes a
mutable borrow of the closure value `f`, so the parameter binding
must be `mut`. Same lesson 006 keyword, slotted into lesson 020's
parameter position. `Fn` needs only a shared borrow, so plain
`f: F` is enough. `FnOnce` consumes `f` on the call, which is why
"once": try calling `f(x)` twice in `call_fnonce`'s body and rustc
fires `error[E0382]: use of moved value: \`f\`` with `note: \`FnOnce\`
closures can only be called once`.

## Mental Model Delta

- *Before:* "There is one `Fn` trait used in the parenthesized-bound
  form (lesson 147). Closures capture outer locals (lesson 144).
  Two facts, one trait."
- *After:* "There are three Fn-family traits, layered as supertraits
  (`Fn: FnMut: FnOnce`). A closure auto-implements one or more based
  on what its body does with captures: no capture or read-only →
  `Fn` (and so all three); mutate → `FnMut` + `FnOnce`; move out →
  `FnOnce` only. The bound on the receiving function constrains
  which closures it accepts. `FnMut` bounds need `mut f: F` so the
  body can call `f(x)` — which internally takes `&mut self`. E0525
  is the trait-mismatch diagnostic."

## Prerequisites

- Installed concepts:
  - **Lesson 147** (load-bearing): the `<F: Fn(T) -> R>`
    parenthesized trait bound and closure-as-argument. Today reuses
    that grammar three times — `Fn`, `FnMut`, `FnOnce` — plus the
    supertrait relationship and the auto-implementation rule.
  - **Lesson 144** (load-bearing): closures capture outer locals.
    Today's centered fact is which of the three traits a closure
    auto-implements based on how the body uses what it captured.
  - **Lesson 006** (load-bearing): `let mut x = ...` makes a binding
    reassignable. Today extends `mut` to a function parameter slot
    (`mut f: F`) — same keyword, different host. The
    `let mut counter: u32 = 0;` in the FnMut probe is exactly
    lesson 006's shape.
  - **Lesson 003** (load-bearing): rustc diagnostic map. E0525 is
    new today; E0596 reappears (lesson 131 first witnessed it on
    `&mut self` for `next()`) with payload now at function-parameter
    position; E0382 reappears in the FnOnce-twice case.
  - Cited: lessons 146 (inline `<T: TRAIT>`); 145 (generic `<T>`);
    142 (closure literal); 094 (the `_` in `move |_: u32|` reuses
    lesson 094's `_` wildcard meaning at a closure-parameter host:
    the parameter slot exists but the body doesn't use it); 042
    (`String::new` shape — today uses `String::from("hello")` to get
    a non-`Copy` value); 081 (`0_u32` literal suffix form); 020
    (`f: F`); 008 (`f(x)`); 080 (`u32`); 011, 005, 002, 001.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the working probe as `demo.rs`, compile, run; output is three
lines, `Fn: 6` / `FnMut: 5` / `FnOnce: 0`. Then save `wrong_trait.rs`
— pass the `tick` closure to `call_fn` instead of `call_fnmut`. The
E0525 transcript above is what you should see, with rustc naming
`counter` as the captured variable that makes `tick` only `FnMut`.
Then save `no_mut.rs` — drop the `mut` from `call_fnmut`'s parameter
slot. The E0596 transcript above is what you should see, with the
`+++` markers under the proposed `mut` insertion.

## What Changed

- The Fn-family has *three* traits. A closure auto-implements one
  or more based on what its body does with captures: no capture or
  shared-ref read → all three; mutate → `FnMut` + `FnOnce`; move
  out → `FnOnce` only.
- The traits are layered (`Fn: FnMut`, `FnMut: FnOnce`). An
  `<F: FnOnce(...)>` bound accepts any closure; `<F: FnMut(...)>`
  accepts FnMut and Fn; `<F: Fn(...)>` accepts only Fn.
- A `<F: FnMut(...)>` bound forces `mut f: F` because calling `f(x)`
  invokes `FnMut::call_mut(&mut self, ...)`. Without `mut`, rustc
  fires E0596 with `+++` markers proposing the missing keyword.
- A trait mismatch fires E0525, with rustc naming the captured
  variable that forced the closure into the more restrictive trait.
- The leading `move` keyword on a closure literal forces ownership
  transfer of captures at definition time; combined with a body that
  consumes them, it produces a `FnOnce`-only closure. The body's
  actions are what ultimately decide which traits the closure
  implements.

## Check Yourself

You write `q.rs`:

```rust
fn call_fn<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 { f(x) }

fn main() {
    let mut total: u32 = 100;
    let bump = |n: u32| { total += n; total };
    println!("{}", call_fn(bump, 7));
}
```

(a) Does `rustc q.rs` compile? Which E-code fires, and which captured
binding does rustc name in the diagnostic? (b) Change the bound on
`call_fn` from `Fn` to `FnMut` and add `mut` to the parameter slot,
giving `fn call_fn<F: FnMut(u32) -> u32>(mut f: F, x: u32) -> u32`.
Now does it compile, and what does it print?

(Answers: (a) does not compile; E0525, "expected a closure that
implements the `Fn` trait, but this closure only implements `FnMut`";
rustc names `total` as the captured binding the body mutates. (b)
Compiles silently; prints `107` — `total += 7` updates `100` to
`107`, and the closure returns the updated `total`.)

## What To Ignore For Now

This lesson installs *only* the three-trait distinction, the
supertrait shape, the auto-implementation rule, and the `mut f: F`
side-fact for FnMut bounds. With this, the closure sub-arc closes;
the 27 closure-driven Iterator methods (audit §4.4.1) become
teachable. Deferred:

- **The `move` keyword as its own mechanic** — used lightly today.
  The Book covers it at `ch13-01-closures.md:286+`. Its own move.
- **The desugaring** — `Fn(T) -> R` ≡ `Fn<(T,), Output = R>`.
  Implementor-side; deferred.
- **`impl Fn(...)` parameter or return position** — separate sugar.
- **`Box<dyn Fn(...)>` / `&dyn Fn(...)`** — dynamic dispatch.
- **Function pointers `fn(u32) -> u32`** — lowercase `fn`, a *type*
  (not a trait). Non-capturing closures coerce to it; capturing
  closures do not.
- **Higher-ranked trait bounds** — `for<'a> Fn(&'a T) -> R`.
- **`AsyncFn` / `AsyncFnMut` / `AsyncFnOnce`** — async-closure family.
- **Multiple parameters `Fn(T, U) -> R`, no-return form `Fn(T)`** —
  same mechanic extended.
- **The std-library implementations on `&F` / `&mut F` / `Box<F>`** —
  each Fn-family trait page lists implementors.

## Evidence

See `../evidence/148-fn-fnmut-fnonce-distinction.md`.
