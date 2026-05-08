# Evidence — 101-mut-self-receiver

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in `/tmp/lesson101-probes/` on this host. Same toolchain
  family as recent accepted lessons (082-100).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/101-mut-self-receiver.rs`
is the working probe verbatim, with header comments naming the
centered E0596 contrast and the secondary E0594 contrast captured
below.

## Sources

### `output/docs/rust/book/ch05-03-method-syntax.md`

Reused chapter from lesson 100. Two new load-bearing passages today:

#### Lines 67-70 — receiver options including `&mut self`

> Methods can take ownership of `self`, borrow `self` immutably, as
> we've done here, or borrow `self` mutably, just as they can any
> other parameter.

Corpus warrant for the *three-receiver-shapes* framing in *The Move*
piece (1) and *Mental Model Delta*. The Book lists exactly three
options today's lesson lists: ownership-taking (deferred to lesson
102), `&self` (lesson 100), `&mut self` (today).

#### Lines 73-75 — when to use `&mut self`

> If we wanted to change the instance that we've called the method
> on as part of what the method does, we'd use `&mut self` as the
> first parameter.

Corpus warrant for the *Mental Model Delta* claim that `&mut self`
is "the mutation receiver." Today's `bump(&mut self)` does exactly
what the Book describes: changes the instance via `self.count =
self.count + 1`.

#### Lines 175-180 — receiver classification

> This automatic referencing behavior works because methods have a
> clear receiver—the type of `self`. Given the receiver and name of
> a method, Rust can figure out definitively whether the method is
> reading (`&self`), mutating (`&mut self`), or consuming (`self`).
> The fact that Rust makes borrowing implicit for method receivers
> is a big part of making ownership ergonomic in practice.

Corpus warrant for the lesson's *What Changed* "three receiver shapes
total" line and for the *Mental Model Delta* labels (read-only,
read+write, deferred consume). The Book uses the verbs *reading*,
*mutating*, *consuming* exactly parallel to today's split.

### `output/docs/rust/reference/items/associated-items.md`

Reused item from lesson 100. The shorthand-table passage now licenses
the third row.

#### Lines 153-159 — the `&mut self` shorthand row

> Shorthand syntax can be used without specifying a type, which have
> the following equivalents:
>
> | Shorthand | Equivalent |
> | --- | --- |
> | `self` | `self: Self` |
> | `&'lifetime self` | `self: &'lifetime Self` |
> | `&'lifetime mut self` | `self: &'lifetime mut Self` |

Corpus warrant for *The Move* piece (1) — `&mut self` is the
shorthand for `self: &mut Self`. Lesson 100 used the second row
(`&self` ↔ `self: &Self`); today uses the third row (`&mut self` ↔
`self: &mut Self`). Today's auxiliary Probe 3 (long-form
`self: &mut Self`) witnesses the equivalence empirically.

#### Lines 105-112 — methods (carried over)

> Associated functions whose first parameter is named `self` are
> called *methods* and may be invoked using the method call
> operator, for example, `x.foo()`, as well as the usual function
> call notation.

Already established by lesson 100; carried over because today's
`bump` is the same kind of associated item, just with a different
receiver row.

### `output/docs/rust/error_codes/E0596.md`

Corpus warrant for the centered contrast probe. The error-code page
is brief but exact:

> This error occurs because you tried to mutably borrow a non-mutable
> variable.

Erroneous example: `let x = 1; let y = &mut x;`. Fix: change to `let
mut x = 1;`. Today's contrast is the dot-method form of the same
underlying rule — calling `&mut self` method on a `let` (not `let
mut`) binding silently inserts a `&mut` borrow that triggers E0596
exactly as the standalone `&mut x` form does.

### `output/docs/rust/error_codes/E0594.md`

Corpus warrant for the secondary contrast probe.

> A non-mutable value was assigned a value.

Erroneous example: `let ss = SolarSystem { earth: 3 }; ss.earth = 2;`.
The corpus example is closer to a future lesson (assigning a struct
field directly through the binding) than today's centered move, but
the same E-code fires when the assignment target sits inside a
`&self` method body (`self` is then a `&` reference). Probe 2 below
witnesses this.

## Probes

### Probe 1 — Working program

The committed observation file. Run in `/tmp/lesson101-probes/`:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- cat demo.rs ---
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
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
count = 2
exit=0
```

Witness for: `&mut self` compiles silently in the receiver position;
`self.count = self.count + 1` inside the body is a valid assignment;
two consecutive `c.bump()` dot calls each run the body and increment
the field; the `&self` reader `current` co-exists with `&mut self`
`bump` in the same impl block; final state read-out matches the two
increments (`count = 2`).

### Probe 2 — Centered E0596 contrast (no `mut` on the binding)

Source `no_mut.rs`, identical to Probe 1 modulo `let mut c = ...`
changed to `let c = ...`:

```text
--- cat no_mut.rs ---
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
    let c = Counter::new();
    c.bump();
    c.bump();
    println!("count = {}", c.current());
}
--- rustc no_mut.rs ---
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

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0596`.
exit=1
```

Witness for: dropping `mut` from the binding causes the `c.bump()`
dot call to fail with E0596. The diagnostic carries headline
`cannot borrow \`c\` as mutable, as it is not declared as mutable`,
caret on the *binding line* (line 18), not the method definition;
*two* secondary annotations under the call sites both saying
`cannot borrow as mutable`. The `help:` block proposes inserting
`mut` after `let`, with `+++` markers under exactly the position
where it should be added. The transcript reproduced verbatim in the
lesson body matches this exactly modulo the `error: aborting...`
trailer (which the lesson reads with lesson 069's category map).

### Probe 3 — Secondary E0594 contrast (assignment in `&self` body)

Source `bad_assign.rs`, identical to Probe 1 modulo `bump`'s
signature: `&mut self` replaced with `&self` (body unchanged):

```text
--- cat bad_assign.rs ---
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
    fn bump(&self) {
        self.count = self.count + 1;
    }
}

fn main() {
    let mut c = Counter::new();
    c.bump();
    println!("count = {}", c.current());
}
--- rustc bad_assign.rs ---
error[E0594]: cannot assign to `self.count`, which is behind a `&` reference
  --> bad_assign.rs:13:9
   |
13 |         self.count = self.count + 1;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so it cannot be written to
   |
help: consider changing this to be a mutable reference
   |
12 |     fn bump(&mut self) {
   |              +++

warning: variable does not need to be mutable
  --> bad_assign.rs:18:9
   |
18 |     let mut c = Counter::new();
   |         ----^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

error: aborting due to 1 previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0594`.
exit=1
```

Witness for: assigning to `self.field` inside a `&self` method body
fires E0594 with caret under the assignment expression and the inline
label `\`self\` is a \`&\` reference, so it cannot be written to`.
The `help:` proposes flipping the receiver to `&mut self` — symmetric
to Probe 2's `help:` (which proposes flipping the binding to `let
mut`). The two contrasts together pin down the rule: assignment
through `self.field` requires both `&mut self` *and* `let mut` on
the caller side. (The auxiliary `unused_mut` warning fires because
the body never reaches the `c.bump()` call site under the `&self`
shape — rustc can prove `c` is read-only despite the binding's `mut`.
Lesson 069 covers the warning category; the lesson body does not
center this auxiliary line.)

### Probe 4 — Auxiliary witness: long-form `self: &mut Self`

Source `explicit_self.rs`, identical to Probe 1 modulo `bump`'s
signature: `&mut self` rewritten as `self: &mut Self` (the unsweetened
form):

```text
--- cat explicit_self.rs ---
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
    fn bump(self: &mut Self) {
        self.count = self.count + 1;
    }
}

fn main() {
    let mut c = Counter::new();
    c.bump();
    c.bump();
    println!("count = {}", c.current());
}
--- rustc explicit_self.rs ---
exit=0
--- ./explicit_self ---
count = 2
exit=0
```

Witness for *The Move* piece (1) claim that "`&mut self` is the
receiver-shorthand for `self: &mut Self`." The long-form spelling
compiles and runs byte-identically to Probe 1's shorthand. This
parallels lesson 100's auxiliary Probe 5 (which witnessed the
shorthand for `&self`); today's empirical evidence licenses the
matching `&mut self` row of the Reference shorthand table.

## Prior lessons

### Direct prerequisites

- **100-inherent-impl-and-self** (accepted, *load-bearing*) — installs
  inherent impl, `&self` methods, `Self`, and associated functions.
  Today's claims that reuse 100:
  - The `impl Counter { ... }` block is unchanged in shape; today's
    `bump` is one more associated item inside it.
  - The associated function `new() -> Self` and the `&self` reader
    `current` are *literally identical* to Probe 1 of lesson 100;
    today's working probe is lesson 100's Probe 1 plus one new
    method (`bump`) plus the binding-side `mut`.
  - The dot-call form `c.method()` is the same lesson 040 form
    lesson 100 inherited.

- **006-mut-binding** (accepted, *load-bearing*) — installs `let mut
  name = value;` for a reassignable binding. Today's claim that
  reuses 006: the caller-side `let mut c = Counter::new();` requires
  `mut` between `let` and `c`. Probe 2 witnesses what fails when the
  `mut` is dropped (E0596). The `mut` keyword's position (between
  `let` and the name) is unchanged from 006; only the *consequence*
  is new — without `mut`, the `&mut self` dot call fails, not just a
  later `c = new_value;` reassignment.

- **047-mutable-reference** (accepted, *load-bearing*) — installs
  `&mut T` as the mutable sibling of `&T`. Today's claim that reuses
  047: `&mut self` is `&mut Self` in the receiver-shorthand position;
  the associated-items table at the Reference's lines 153-159 makes
  this explicit. Lesson 047's E0596 was named in *What To Ignore For
  Now* as a future move; today centers it. The deref-assign form
  `*r = newval;` from 047 is *not* used today (today's writes go
  through `self.field = value`, not `*self = ...`); the `*self =
  expr` form is explicitly deferred in *What To Ignore For Now*.

### Supporting prior lessons (cited only)

- **095-struct-with-named-fields** — `struct Counter { count: u32 }`
  declaration and `Counter { count: 0 }` construction reused
  unchanged. The new claim today is that `instance.field` from 095
  is also valid on the *left* of `=` when the receiver is a `&mut`
  borrow. This is a quiet extension of 095, not a new field-access
  rule.
- **040-method-call-syntax** — the dot-call form `value.method()`,
  unchanged.
- **003-read-rustc-diagnostic** — the four-part diagnostic map
  applied to Probe 2's E0596 transcript and Probe 3's E0594
  transcript.
- **069-rustc-warnings** — Probe 3's `error: aborting due to 1
  previous error; 1 warning emitted` trailer is read with lesson
  069's category map. The auxiliary `unused_mut` warning belongs to
  the warning category, not the error category.
- **001-rustc-compile-and-run** — `rustc demo.rs` then `./demo`,
  silent on success.
- **020-function-with-parameter** and **021-function-return-value**
  — every `fn` inside today's impl is the same `fn` shape lesson
  100 already inherited from 020/021.
- **002-fn-main-entry-point**, **005-let-binding**,
  **011-println-positional-args**, **062-u32-unsigned-integer** —
  reused unchanged from lesson 100's probe.

## Probe-shape note

Probe 1's `bump` body uses `self.count = self.count + 1` rather than
the more idiomatic `self.count += 1`. Compound-assign operators
(`+=`, `-=`, etc.) are not in the graph yet; today's body uses only
the plain assignment from lesson 095 generalized + the `+` operator
from earlier lessons. The choice is deliberate to keep today's
mental-model delta narrow: write-access through `&mut self`, not
compound assignment.

## Mapping summary

| Lesson claim | Source / probe |
|---|---|
| `&mut self` is shorthand for `self: &mut Self` | Reference items/associated-items.md lines 153-159 (third row of shorthand table); Probe 4 witnesses the long-form equivalence |
| Methods declared with `&mut self` may write to `self.field` | Book ch05-03 lines 73-75; Probe 1 witnesses; Probe 3 witnesses what fails without it (E0594) |
| Three receiver shapes total: `&self`, `&mut self`, `self` | Book ch05-03 lines 67-70 and 175-180 (the *reading*/*mutating*/*consuming* split) |
| Calling a `&mut self` method requires the binding to be `let mut` | E0596 corpus warrant; Probe 2 witnesses verbatim |
| Without `let mut`, the dot-call fails with E0596 caret on the binding line | Probe 2 transcript verbatim |
| `help:` proposes inserting `mut` after `let` | Probe 2 transcript verbatim |
| Assignment to `self.field` in a `&self` body fires E0594 | Probe 3 transcript verbatim |
| Long-form `self: &mut Self` compiles identically to `&mut self` | Probe 4 transcript verbatim |

No lesson claim relies on a fact that does not appear in either a
listed corpus passage or a captured probe.
