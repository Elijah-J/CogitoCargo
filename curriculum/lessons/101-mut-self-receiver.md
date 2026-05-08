---
id: 101-mut-self-receiver
status: accepted
evidence: ../evidence/101-mut-self-receiver.md
---

# Author a method that mutates the receiver with `&mut self`

## The Move

Lesson 100 installed two receiver shapes inside `impl Type { ... }`: no
receiver (associated function, e.g. `fn new() -> Self`) and `&self`
(read-only method). Today adds the third shape that lets a method
*write* to the value it was called on:

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
    fn current(&self) -> u32 {
        self.count
    }
    fn bump(&mut self) {
        self.count = self.count + 1;
    }
}

fn main() {
    let mut c = Counter::new();
    c.bump();
    c.bump();
    println!("count = {}", c.current());
}
```

`./demo` prints `count = 2`. Three coupled pieces:

1. **Authoring**: `fn bump(&mut self) { ... }` inside the impl. The
   `&mut self` is the receiver-shorthand for `self: &mut Self` — the
   third row of the same Reference shorthand table that gave us
   `&self` as `self: &Self` last lesson.
2. **Mutation through the receiver**: `self.count = self.count + 1;`
   *writes* to the field. Lesson 095's field-access expression now
   sits on the left of `=` — that works only because `self` is `&mut
   Self` rather than `&Self`.
3. **Calling**: the dot site `c.bump()` is unchanged from lesson 040.
   The new rule lives on the *binding*: `c` must be `let mut c`.
   Without `mut`, the borrow check rejects the dot call.

## Mental Model Delta

- *Before*: "Lesson 100 gave me `&self` methods that read fields. The
  receiver is borrowed immutably; assigning to `self.field` would
  fail."
- *After*: "`&mut self` is the mutation receiver. Same impl block,
  same dot-call site. Inside the body, `self.field = value;` is now
  legal because `self` is a mutable borrow rather than a shared one.
  The caller's binding must be `let mut`. Three receiver shapes
  total: `&self` (read-only), `&mut self` (read + write), `self` by
  value (deferred); the first two differ only by the `mut` keyword."

## Prerequisites

- Installed concepts:
  - Lesson 100 (*load-bearing*): `impl Type { ... }`, `&self`
    methods, `Self`, associated functions. The impl block,
    associated function `new`, and `&self` reader `current` are
    reused untouched; the only new piece is `bump`.
  - Lesson 006 (*load-bearing*): `let mut name = value;`. Today's
    caller side requires `let mut c`; without it, E0596 fires.
  - Lesson 047 (*load-bearing*): `&mut T` as the mutable sibling of
    `&T`. Today's `&mut self` is `&mut T` in the receiver-shorthand
    position; not re-explained here.
  - Lesson 095 (cited): `instance.field`. Today extends it: under
    `&mut self`, `self.field` is also valid on the *left* of `=`.
  - Lesson 040 (cited): the dot-call form, unchanged.
  - Lesson 003 (cited): the four-part diagnostic map.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
count = 2
```

Each `c.bump()` runs the body of `bump`, incrementing `count` by 1;
`c.current()` reads it back.

*Now the contrast.* Drop `mut` from line 18 — change `let mut c =
Counter::new();` to `let c = Counter::new();`:

```
error[E0596]: cannot borrow `c` as mutable, as it is not declared as mutable
  --> no_mut.rs:18:9
   |
18 |     let c = Counter::new();
   |         ^ not mutable
19 |     c.bump();
   |     - cannot borrow as mutable
20 |     c.bump();
   |     - cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
18 |     let mut c = Counter::new();
   |         +++
```

Read it with the lesson 003 map. Headline E0596; caret under `c` at
the *binding line*, not the method. Annotations under each call site
say `cannot borrow as mutable`. The `help:` proposes inserting `mut`.
(A secondary contrast — assigning to `self.field` inside a `&self`
body — fires E0594 `cannot assign to \`self.count\`, which is behind
a \`&\` reference`. Witness in the appendix.)

## What Changed

- `&mut self` is the third receiver shape: the receiver-shorthand for
  `self: &mut Self`. Methods declared this way may write to fields.
- `self.field = value;` inside a `&mut self` body is legal; the same
  expression in a `&self` body fires E0594.
- The call site `c.method()` is unchanged, but the binding must be
  `let mut c`. Without `mut`, E0596 fires with caret under the
  *binding*, not the call.
- Three receiver shapes total: `&self` (lesson 100), `&mut self`
  (today), `self` by value (deferred).

## Check Yourself

Take today's probe verbatim and add a third `c.bump();` line in
`main` (so three increments now run). Then:

(a) Does `rustc demo.rs` still compile silently?

(b) What single line does `./demo` print?

(c) If you also delete `mut` from `let mut c`, what E-code appears,
and which line does the `-->` location point at?

(*Answers: (a) Yes. (b) `count = 3`. (c) E0596; the `-->` points at
the binding line `let c = Counter::new();`. The `help:` proposes
inserting `mut`.*)

## What To Ignore For Now

Real and deferred:

- *`self` by-value receiver* — the consuming shape; lesson 102.
- *The full borrow-check aliasing rule* — "one `&mut` xor any number
  of `&`." Today surfaces a piece (E0596); the full rule is its own
  move.
- *Reborrows inside `&mut self`* — e.g. `self.vec.push(x)` takes a
  fresh `&mut` of `self.vec`. Subtle.
- *Methods returning a `&mut` into `self`* — `iter_mut`-shaped APIs.
- *Multiple-mutable-borrow errors* (E0499).
- *Different uses of `mut`* — pattern bindings vs references vs
  function parameters. Today is only the binding form.
- *Interior mutability* (`Cell`, `RefCell`, `Mutex`).
- *`*self = expr` deref-write-through-pointer* — used in rmp's
  `Sign::flip`; not centered today.
- All previously deferred items.

(Today's move unlocks reading rmp's many `&mut self` methods on
`BigUInt` — e.g. `pub fn flip(&mut self)` in `bigint.rs`.)

## Evidence

See `../evidence/101-mut-self-receiver.md` for the corpus-quote map,
toolchain string, working probe transcript, centered E0596 contrast,
secondary E0594 contrast, long-form `self: &mut Self` auxiliary, and
prerequisite-claim summary.
