---
id: 102-self-by-value-receiver
status: accepted
evidence: ../evidence/102-self-by-value-receiver.md
---

# Author a method that consumes the receiver with `self`

## The Move

Lessons 100 and 101 installed two of the three receiver shapes inside
`impl Type { ... }`: `&self` (read) and `&mut self` (read + write).
Today adds the third and final shape — `self` with no `&` and no `mut`
— the *consuming* receiver:

```rust
struct Wrapper {
    value: u32,
}

impl Wrapper {
    fn into_inner(self) -> u32 {
        self.value
    }
}

fn main() {
    let w = Wrapper { value: 42 };
    let inner = w.into_inner();
    println!("inner = {}", inner);
}
```

`./demo` prints `inner = 42`. Two coupled pieces:

1. **Authoring**: `fn into_inner(self) -> u32 { ... }`. The bare
   `self` is the receiver-shorthand for `self: Self` — the first row
   of the same Reference shorthand table that gave us `&self` (lesson
   100) and `&mut self` (lesson 101).
2. **Calling**: `w.into_inner()` is unchanged from lesson 040. The
   new rule is what happens to `w` *after* the call. The dot call
   *consumes* (moves) the value into the method; the binding `w`
   cannot be used again.

The Book frames the trio explicitly: methods are "reading (`&self`),
mutating (`&mut self`), or consuming (`self`)." Today is the third
verb. The Book also notes the shape is rare — "usually used when the
method transforms `self` into something else and you want to prevent
the caller from using the original instance after the transformation."

## Mental Model Delta

- *Before*: "Lessons 100 and 101 gave me `&self` (the receiver borrows
  the value, caller keeps it) and `&mut self` (the receiver borrows
  mutably, caller still keeps it). Both leave the binding usable after
  the call."
- *After*: "`self` (no `&`) is the third shape: the method *consumes*
  the value. After `w.into_inner()`, `w` has been moved into the
  method; using it again fires E0382 with a `note:` block naming the
  receiver shape. Three receiver shapes now installed; the first two
  leave the binding usable, the third does not."

## Prerequisites

- Installed concepts:
  - Lesson 100 (*load-bearing*): `impl Type { ... }`, `Self`, `&self`
    methods. The only new piece today is the receiver `self` (no `&`).
  - Lesson 101 (*load-bearing*): `&mut self`. Today completes the
    receiver-shape trio that lesson 101 framed.
  - Lesson 095 (cited): `instance.field`; today's `self.value` reuses
    this unchanged.
  - Lesson 040 (cited): the dot-call form, unchanged.
  - Lesson 003 (cited): the four-part diagnostic map applied to E0382.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
inner = 42
```

*Now the contrast.* Same source, but add a second
`let inner2 = w.into_inner();` *after* the first call. Save as
`use_after_move.rs`:

```rust
fn main() {
    let w = Wrapper { value: 42 };
    let inner = w.into_inner();
    let inner2 = w.into_inner();
    println!("inner = {}, inner2 = {}", inner, inner2);
}
```

Compile:

```
error[E0382]: use of moved value: `w`
  --> use_after_move.rs:14:18
   |
12 |     let w = Wrapper { value: 42 };
   |         - move occurs because `w` has type `Wrapper`, which does not implement the `Copy` trait
13 |     let inner = w.into_inner();
   |                   ------------ `w` moved due to this method call
14 |     let inner2 = w.into_inner();
   |                  ^ value used here after move
   |
note: `Wrapper::into_inner` takes ownership of the receiver `self`, which moves `w`
  --> use_after_move.rs:6:19
   |
 6 |     fn into_inner(self) -> u32 {
   |                   ^^^^
```

Read it with the lesson 003 map. Headline E0382; caret on the second
call site. Three annotations carry the story: the binding line says
"move occurs because `Wrapper` ... does not implement the `Copy`
trait"; the first call says "`w` moved due to this method call"; the
second says "value used here after move." A separate `note:` block
points at the method's signature and names today's rule verbatim:
`Wrapper::into_inner` *takes ownership of the receiver* `self`, *which
moves* `w`.

## What Changed

- `self` (no `&`, no `mut`) is the third receiver shape: shorthand
  for `self: Self`. Methods declared this way consume the value they
  were called on.
- After `w.into_inner()`, `w` cannot be used again. E0382 fires, and
  the `note:` block names the receiver shape directly.
- Three receiver shapes total. The trio:

| Shape       | Reads | Mutates              | Consumes | Lesson |
|-------------|:-----:|:--------------------:|:--------:|:------:|
| `&self`     |  yes  |                      |          |  100   |
| `&mut self` |  yes  |         yes          |          |  101   |
| `self`      |  yes  | no (needs `mut self`)|   yes    |  102   |

Most real-world methods use `&self` or `&mut self`. The `self` shape
exists when the method's job is to *transform* the value into
something else.

## Check Yourself

Take today's probe verbatim. Add `println!("w.value = {}", w.value);`
*after* the existing `println!`, leaving everything else unchanged.

(a) What E-code fires?

(b) What does the `note:` block at the method-definition site say
about who took ownership?

(*Answers: (a) E0382. (b) `Wrapper::into_inner` takes ownership of
the receiver `self`, which moves `w`; the `-->` points at the `fn
into_inner(self) -> u32` line.*)

## What To Ignore For Now

Real and deferred:

- *Move semantics in depth* — the rule "non-`Copy` types are moved
  when bound to a new name or passed as a non-reference argument."
  Today shows only the operational surface; the full framework is
  Book Ch4's arc.
- *`Copy` types vs move types* — primitive integers like `u32` are
  `Copy`, so `let a = 5; let b = a; let c = a;` works. Today's
  `Wrapper` is *not* `Copy`, so it moves (visible in the contrast's
  "does not implement the `Copy` trait" line). Deferred.
- *`Drop`* — what happens to a moved-then-dropped value. Trait
  machinery, deferred.
- *Builder patterns* — `value.method1().method2()` chains where each
  step takes `self` and returns `Self`. Future move.
- *`mut self` (without `&`)* — `fn method(mut self)` makes the
  consumed `self` a mutable variable inside the body (Reference
  items/associated-items.md lines 165-167). Future move.
- *Cloning to avoid the move* — `value.clone().method(...)` keeps the
  original. Trait machinery, deferred.
- All previously deferred items.

(Today's move unlocks reading `impl Neg for BigInt { fn neg(self) ->
Self::Output { ... } }` in the rmp target's `bigint.rs` — though
`Neg` is a *trait* impl, the *receiver shape* is today's. Trait impls
remain deferred.)

## Evidence

See `../evidence/102-self-by-value-receiver.md` for the corpus-quote
map, toolchain string, working probe transcript, centered E0382
contrast, long-form `self: Self` auxiliary, and prerequisite-claim
summary.
