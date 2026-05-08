---
id: 109-module-scope-const
status: accepted
evidence: ../evidence/109-module-scope-const.md
---

# Declare a `const` at module scope and gate it with `pub(...)`

## The Move

Lesson 075 wrote `const NAME: TYPE = value;` *inside* `fn main`. The
same declaration shape works *outside* every function, at the crate
root or inside a `mod` block. Once the declaration sits at module
scope, it is a *module-level item* — same item position `fn` and
`mod` live in — and may carry a visibility modifier in front, exactly
as a function may. Two pieces, one composed move:

```rust
const MAX_BYTES: u32 = 1024;

mod inner {
    const LOCAL: u64 = 100;
    pub(crate) const SHARED: u64 = 64;

    pub fn read_local() -> u64 { LOCAL }
}

fn main() {
    println!("MAX_BYTES = {}", MAX_BYTES);
    println!("inner::SHARED = {}", inner::SHARED);
    println!("inner::LOCAL via fn = {}", inner::read_local());
}
```

`MAX_BYTES` sits at the *crate root* — outside every `fn`, outside
every `mod`. `LOCAL` and `SHARED` sit inside `mod inner` — module
scope, inside a child module. `LOCAL` has no visibility marker, so it
is private to `inner`; `SHARED` carries `pub(crate)`, opening it to
anywhere in the crate.

(This unlocks reading rmp's
`pub(crate) const LIMB_SIZE_BITS: u64 = 8 * (std::mem::size_of::<u64>() as u64);`
in `biguint/basic.rs`. The right-hand-side `std::mem::size_of::<u64>()`
is its own move and stays deferred today.)

## Mental Model Delta

- *Before*: "Lesson 075 said `const NAME: TYPE = value;` declares a
  compile-time constant. The Book also said a const may sit at global
  scope, but every probe so far put it inside `fn main`."
- *After*: "The same `const NAME: TYPE = value;` shape is also a
  *module-level item* — sibling to `fn` and `mod` — when it sits
  outside any function. Module-scope const items have a
  visibility, defaulting to private. `pub`, `pub(crate)`, and
  `pub(super)` go in the same syntactic position they did on
  functions in lessons 096 and 103. Other modules read the
  constant via the same `module::ITEM` path form they use for
  functions."

## Prerequisites

- Installed concepts:
  - Lesson 075 (*load-bearing*): `const NAME: TYPE = value;` syntax,
    the SCREAMING_SNAKE_CASE convention, the required `: TYPE`
    annotation, the constant-expression rule. Lesson 075 already
    quoted the Book's "Constants can be declared in any scope,
    including the global scope" but every committed probe placed
    the const inside `fn main`. Today exercises the global / module
    half of that rule.
  - Lesson 096 (*load-bearing*): `mod foo { ... }` items inline, `pub`
    on a function item, items private by default, the `module::item`
    access form, and the E0603 diagnostic. Today applies the *same*
    `pub`-position rule to a `const` item instead of a `fn` item;
    Probe 2's E0603 transcript carries the same E-code with the
    inline label `private constant` rather than `private function`.
  - Lesson 103 (*load-bearing*): `pub(super)` and `pub(crate)` as
    shapes from the `Visibility` grammar, going in the same
    syntactic position as plain `pub`. Today reuses the same two
    modifiers verbatim, on a `const` item.
  - Lesson 104 (cited): named for completeness as the lesson that
    installed `super::` and `crate::` path tokens. Today's probes do
    *not* exercise `super::name` token syntax — they reach module-scope
    consts through bare names (when same-module), the `module::ITEM`
    path form from lesson 096, or absolute paths like
    `outer::inner::PARENT_ONLY`. The `pub(super)` *modifier* used in
    Probe 4 is from lesson 103, not 104.
  - Lessons 002, 005, 008, 011, 040, 043 (cited): unchanged
    supporting machinery (`fn main`, `let`, `fn name() -> T`,
    `println!`, `module::name(args)`). 062 (cited): `u32`/`u64` in
    the `: TYPE` slot.
  - Lesson 003 (cited): the four-part diagnostic map applied to E0603.
- Ordinary computer-use assumptions: same as lesson 001.

## Try It

Save the program above as `demo.rs`. Compile and run:

```console
$ rustc demo.rs
$ ./demo
MAX_BYTES = 1024
inner::SHARED = 64
inner::LOCAL via fn = 100
```

Three lines. `MAX_BYTES` is reached as a bare name from `fn main`
because both live in the crate root. `inner::SHARED` is reached
through the `module::item` path because it sits inside `inner` and is
exposed via `pub(crate)`. `inner::LOCAL` is *not* reached directly —
`inner::read_local()` is called instead, because `LOCAL` is private
to `inner` and the helper `fn` is the only public way out.

*Now the contrast — the centered teaching moment.* Drop the
`pub(crate)` from `SHARED` and recompile. Save as `broken.rs`:

```rust
mod inner {
    const LOCAL: u64 = 100;
    const SHARED: u64 = 64;
    pub fn read_local() -> u64 { LOCAL }
}

fn main() {
    println!("inner::SHARED = {}", inner::SHARED);
    println!("inner::LOCAL via fn = {}", inner::read_local());
}
```

Compile:

```text
error[E0603]: constant `SHARED` is private
  --> broken.rs:8:43
   |
 8 |     println!("inner::SHARED = {}", inner::SHARED);
   |                                           ^^^^^^ private constant
   |
note: the constant `SHARED` is defined here
  --> broken.rs:3:5
   |
 3 |     const SHARED: u64 = 64;
   |     ^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error
```

Read it with the lesson 003 map. Headline E0603 — the same E-code
lesson 096 first installed when `pub` was missing from `fn hi`. Caret
under `SHARED` at the *use site*; a `note:` block points back at the
definition. The inline label is `private constant` (instead of
`private function` from 096). Same rule, different item kind: a
module-scope `const` is private by default, and `pub(...)` is what
opens it.

## What Changed

- A `const NAME: TYPE = value;` declaration may sit at *module scope*
  — outside any `fn`, in the crate root or inside a `mod` block —
  where it becomes a module-level item like a `fn`.
- Module-scope const items carry a visibility, just like functions.
  No marker means private to the declaring module; `pub`,
  `pub(crate)`, and `pub(super)` open it on the same radii lessons
  096 and 103 installed.
- Other modules reach the constant via the same `module::ITEM` path
  form already used for functions.
- A failed visibility check on a `const` fires the same E0603 lesson
  096 installed for functions, with `private constant` in the inline
  label slot in place of `private function`.

## Check Yourself

You write `quiz.rs`:

```rust
mod cfg {
    pub(crate) const TIMEOUT_MS: u32 = 250;
    const RETRY_LIMIT: u32 = 3;
    pub fn retries() -> u32 { RETRY_LIMIT }
}

fn main() {
    println!("timeout = {}", cfg::TIMEOUT_MS);
    println!("retries = {}", cfg::retries());
}
```

(a) Does `rustc quiz.rs` accept the program? What does `./quiz`
print?

(b) If you change line 9 to `println!("retries = {}", cfg::RETRY_LIMIT);`,
does it still compile? Which E-code fires?

(c) Why does `cfg::TIMEOUT_MS` work from `main` but `cfg::RETRY_LIMIT`
does not?

*(Answers: (a) Yes. Prints `timeout = 250` then `retries = 3`. (b) No.
E0603, with the caret under `RETRY_LIMIT` at the use site and a
`note:` pointing at `const RETRY_LIMIT: u32 = 3;`, inline label
`private constant`. (c) `TIMEOUT_MS` is `pub(crate)`, so it is
visible everywhere in the crate, including `fn main`. `RETRY_LIMIT`
has no visibility marker, so it is private to `mod cfg`; the public
helper `cfg::retries()` is the only way out.)*

## What To Ignore For Now

Today installs only module-scope position and the three visibility
modifiers `pub` / `pub(crate)` / `pub(super)` on a `const` item.
Real and deferred:

- *`static` items* — `static NAME: TYPE = value;` shares today's
  shape but has different semantics (one fixed memory location vs
  the const's "essentially inlined wherever used" rule per
  Reference items/constant-items.md). Future move.
- *`const` items in trait or impl blocks* — `impl Type { const N:
  ... = ...; }` is an *associated constant*, not a free constant.
  Different rules; deferred. Reference items/constant-items.md
  line 91 names the distinction.
- *Generic const items / const generics* — `fn f<const N: usize>()`
  uses the `const` keyword in a different role.
- *Right-hand-side function calls* — `const N: u64 = compute();`
  requires `compute` to be a `const fn`. Lesson 075 saw E0015 on
  this; today's right-hand side stays at literals.
- *`pub(in path)` and `pub(self)`* — already deferred in lesson 103.
- *Underscore-named consts* — `const _: () = assert!(...);`
  (Reference items/constant-items.md lines 87-101). Future move
  for top-level compile-time assertions.
- *`std::mem::size_of::<T>()`* — the rmp `LIMB_SIZE_BITS`
  expression's right-hand side. Generic associated function; its
  own future move.
- *Cross-crate `pub`* — what plain `pub` actually means when
  another crate links against this one (already deferred in 103).

## Evidence

See `../evidence/109-module-scope-const.md`.
