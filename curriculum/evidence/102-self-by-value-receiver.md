# Evidence — 102-self-by-value-receiver

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in `/tmp/lesson102-probes/` on this host. Same toolchain
  family as recent accepted lessons (082-101).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/102-self-by-value-receiver.rs`
is the working probe verbatim, with header comments naming the
centered E0382 contrast and the auxiliary long-form witness captured
below.

## Sources

### `output/docs/rust/book/ch05-03-method-syntax.md`

Reused chapter from lessons 100 and 101. Today centers the third
receiver shape; two passages from this chapter are load-bearing.

#### Lines 76-79 — `self` by value, the consuming receiver

> Having a method that takes ownership of the instance by using just
> `self` as the first parameter is rare; this technique is usually
> used when the method transforms `self` into something else and you
> want to prevent the caller from using the original instance after
> the transformation.

Corpus warrant for *The Move* piece (1) ("the *consuming* receiver"),
the closing paragraph of *The Move* (the rarity + transform framing),
and the closing paragraph of *What Changed* ("the `self` shape exists
when the method's job is to transform the value"). The lesson body
quotes the second sentence verbatim.

#### Lines 175-180 — receiver classification (the trio framing)

> This automatic referencing behavior works because methods have a
> clear receiver—the type of `self`. Given the receiver and name of
> a method, Rust can figure out definitively whether the method is
> reading (`&self`), mutating (`&mut self`), or consuming (`self`).
> The fact that Rust makes borrowing implicit for method receivers
> is a big part of making ownership ergonomic in practice.

Corpus warrant for *The Move*'s "methods are 'reading (`&self`),
mutating (`&mut self`), or consuming (`self`)'" verbatim quote and
for the trio table in *What Changed*. The Book's three verbs map
directly to lessons 100, 101, 102.

### `output/docs/rust/reference/items/associated-items.md`

Reused item from lessons 100 and 101. The shorthand-table passage now
licenses the *first* row.

#### Lines 153-159 — the `self` shorthand row

> Shorthand syntax can be used without specifying a type, which have
> the following equivalents:
>
> | Shorthand | Equivalent |
> | --- | --- |
> | `self` | `self: Self` |
> | `&'lifetime self` | `self: &'lifetime Self` |
> | `&'lifetime mut self` | `self: &'lifetime mut Self` |

Corpus warrant for *The Move* piece (1) — bare `self` is the
receiver-shorthand for `self: Self`. Lesson 100 used the second row,
lesson 101 used the third row, today uses the first. Today's
auxiliary Probe 3 (long-form `self: Self`) witnesses the equivalence
empirically.

#### Lines 165-167 — `mut self` (named in *What To Ignore For Now*)

> If the `self` parameter is prefixed with `mut`, it becomes a
> mutable variable, similar to regular parameters using a `mut`
> identifier pattern.

Cited only in *What To Ignore For Now* to defer the `mut self` (no
`&`) variant. Today's probe uses bare `self`; today's body never
mutates `self` in place (it returns `self.value` and moves on).

#### Lines 105-112 — methods (carried over)

> Associated functions whose first parameter is named `self` are
> called *methods* and may be invoked using the method call
> operator, for example, `x.foo()`, as well as the usual function
> call notation.

Already established by lesson 100; carried over because today's
`into_inner` is the same kind of associated item, just with a
different receiver row.

### `output/docs/rust/error_codes/E0382.md`

Corpus warrant for the centered contrast probe.

> A variable was used after its contents have been moved elsewhere.

The corpus's erroneous example uses field reassignment after a `let`
move (`let y = x; x.s = 6;`), not a method call, but the underlying
rule is the same: a value of a non-`Copy` type is moved when bound
to a new name *or passed by value as a non-reference argument* —
which is exactly what `self` (no `&`) is. The corpus page also
explicitly names the three escape hatches today defers (references,
`Clone`, `Copy`); see *What To Ignore For Now*. Probe 2 below
witnesses the method-call form of E0382 verbatim.

## Probes

### Probe 1 — Working program

The committed observation file. Run in `/tmp/lesson102-probes/`:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- cat demo.rs ---
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
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
inner = 42
exit=0
```

Witness for: `self` (no `&`) compiles silently in the receiver
position; the body `self.value` is a valid field-access expression on
the moved-in receiver; the dot call `w.into_inner()` runs the body
and returns the field value; `inner = 42` matches the field's
initializer; the program exits 0 with no warnings.

### Probe 2 — Centered E0382 contrast (use after move)

Source `use_after_move.rs`, identical to Probe 1 modulo one new line
in `main`: a second `let inner2 = w.into_inner();` after the first
call, and an updated `println!` that references both:

```text
--- cat use_after_move.rs ---
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
    let inner2 = w.into_inner();
    println!("inner = {}, inner2 = {}", inner, inner2);
}
--- rustc use_after_move.rs ---
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

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
exit=1
```

Witness for: trying to use `w` after the `self`-receiver method call
fires E0382 with headline `use of moved value: \`w\``. Three
annotations carry the full story — the binding line says "move
occurs because `Wrapper` ... does not implement the `Copy` trait",
the first call site says "`w` moved due to this method call", and
the second call site says "value used here after move". The
*separate* `note:` block points at the *method-definition* line and
states today's rule verbatim: `Wrapper::into_inner` *takes ownership
of the receiver* `self`, *which moves* `w`. The lesson body
reproduces the diagnostic up to the `error: aborting...` trailer,
which the appendix reads with lesson 069's category map.

The contrast's specific shape (a *second* call to the same `self`
method, rather than a bare `println!("{}", w.value)` after the
first call) was chosen because (a) it keeps the source diff to a
single added line, (b) it produces the cleanest annotation set —
the first-call line gets a green underline naming the cause and the
second-call line gets the caret, and (c) any subsequent use of `w`
triggers the same E0382 with an analogous `note:` block.

### Probe 2b — *Check Yourself* variant (field access after move)

Source `check_yourself.rs`, identical to Probe 1 plus one new line
after the existing `println!`: `println!("w.value = {}", w.value);`.
Captured to verify the *Check Yourself* answer:

```text
error[E0382]: borrow of moved value: `w`
  --> check_yourself.rs:15:30
   |
12 |     let w = Wrapper { value: 42 };
   |         - move occurs because `w` has type `Wrapper`, which does not implement the `Copy` trait
13 |     let inner = w.into_inner();
   |                   ------------ `w` moved due to this method call
14 |     println!("inner = {}", inner);
15 |     println!("w.value = {}", w.value);
   |                              ^^^^^^^ value borrowed here after move
   |
note: `Wrapper::into_inner` takes ownership of the receiver `self`, which moves `w`
  --> check_yourself.rs:6:19
   |
 6 |     fn into_inner(self) -> u32 {
   |                   ^^^^
```

Witness for: any subsequent use of `w` after the move fires E0382.
Note the headline-phrasing variance — the *Check Yourself* form
("`println!("{}", w.value)`") gives `error[E0382]: borrow of moved
value: \`w\``, while Probe 2's second-call form gives `error[E0382]:
use of moved value: \`w\``. The two phrasings come from the same
E-code (E0382) and carry the same `note:` block (today's rule
verbatim) and the same first-line + first-call annotations; the
caret-line label flips between "value used here after move" (call
form) and "value borrowed here after move" (field-access form). The
*Check Yourself* answer commits only to the E-code and the `note:`
text, both of which are stable across both forms. The headline-text
variance is documented here for red-team verification.

### Probe 3 — Auxiliary witness: long-form `self: Self`

Source `explicit_self.rs`, identical to Probe 1 modulo
`into_inner`'s signature: `self` rewritten as `self: Self` (the
unsweetened form):

```text
--- cat explicit_self.rs ---
struct Wrapper {
    value: u32,
}

impl Wrapper {
    fn into_inner(self: Self) -> u32 {
        self.value
    }
}

fn main() {
    let w = Wrapper { value: 42 };
    let inner = w.into_inner();
    println!("inner = {}", inner);
}
--- rustc explicit_self.rs ---
exit=0
--- ./explicit_self ---
inner = 42
exit=0
```

Witness for *The Move* piece (1) claim that "the bare `self` is the
receiver-shorthand for `self: Self`." The long-form spelling
compiles and runs byte-identically to Probe 1's shorthand. This
parallels lesson 100's auxiliary (`&self` ↔ `self: &Self`) and
lesson 101's auxiliary (`&mut self` ↔ `self: &mut Self`); today's
empirical evidence licenses the matching first row of the Reference
shorthand table.

## Prior lessons

### Direct prerequisites

- **100-inherent-impl-and-self** (accepted, *load-bearing*) —
  installs `impl Type { ... }`, `Self`, `&self` methods, and
  associated functions. Today's claims that reuse 100:
  - The `impl Wrapper { ... }` block is the same shape as lesson
    100's `impl Counter { ... }`; today's `into_inner` is one
    associated item inside it.
  - The dot-call form `value.method()` is unchanged from lesson 040
    via lesson 100.
  - The Reference shorthand table that licensed `&self ↔ self: &Self`
    in lesson 100 has a first row (`self ↔ self: Self`) that licenses
    today's bare `self`.

- **101-mut-self-receiver** (accepted, *load-bearing*) — installs
  `&mut self` and frames the *three-receiver-shapes* trio. Today's
  claims that reuse 101:
  - Lesson 101's *Mental Model Delta* explicitly named "three
    receiver shapes total: `&self` (read-only), `&mut self` (read +
    write), `self` by value (deferred); the first two differ only by
    the `mut` keyword." Today completes the deferred shape and adds
    the consume/transform framing that 101 could only point at.
  - Lesson 101's *What To Ignore For Now* listed "`self` by-value
    receiver — the consuming shape; lesson 102." Today is exactly
    that follow-on.
  - The Book's "reading (`&self`), mutating (`&mut self`), or
    consuming (`self`)" classification (Book ch05-03 lines 175-180)
    was cited in lesson 101's evidence and motivates today's third
    column of the trio table.

### Supporting prior lessons (cited only)

- **095-struct-with-named-fields** — `struct Wrapper { value: u32 }`
  declaration, `Wrapper { value: 42 }` construction, and `self.value`
  field access reused unchanged. No new claim about field expressions
  is introduced today — they sit on the right-hand side of an
  expression like in lesson 095, not on a left-hand assignment target
  (which would be the `&mut self` case from lesson 101).
- **040-method-call-syntax** — the dot-call form `value.method()`,
  unchanged. Today reuses lesson 040 verbatim through lesson 100's
  inheritance.
- **003-read-rustc-diagnostic** — the four-part diagnostic map
  applied to Probe 2's E0382 transcript. Today's diagnostic carries a
  *separate* `note:` block with its own `-->` line pointing at the
  method-definition site (line 6, column 19), the same multi-`-->`
  shape lesson 096 first installed for E0603.
- **069-rustc-warnings** — Probe 2's `error: aborting due to 1
  previous error` trailer is read with lesson 069's category map.
  The working probe produces no warnings.
- **001-rustc-compile-and-run** — `rustc demo.rs` then `./demo`,
  silent on success.
- **002-fn-main-entry-point**, **005-let-binding**,
  **011-println-positional-args**, **062-u32-unsigned-integer**,
  **020-function-with-parameter**, **021-function-return-value** —
  all reused unchanged from lesson 100/101's probe shape.

## Probe-shape note

Probe 1's `into_inner` body is `self.value` (a single field
expression returned implicitly as the function's tail expression).
The choice is deliberate: it lets today's body reuse lesson 095's
field-access form unchanged, with no new claim about move-out from
a field. The field-expression here happens to evaluate to a `u32`,
which is `Copy`, so the field's value is *copied* out of the
moved-in `self`; the `self` value is then dropped at the end of the
method body. This Copy-vs-move detail is *deferred* in the lesson
body (under *Copy types vs move types* in *What To Ignore For Now*),
but it is what makes Probe 1 compile silently — if `Wrapper.value`
were itself a non-`Copy` type, the same body shape would still work
(move-out from a field), but the rules for that are subtler and not
load-bearing today.

The function name `into_inner` follows a community convention
(`into_*` for `self`-consuming conversions); the lesson body does
not center this convention because the rule is purely cosmetic. Cited
silently in the working observation header.

## Mapping summary

| Lesson claim | Source / probe |
|---|---|
| Bare `self` is shorthand for `self: Self` | Reference items/associated-items.md lines 153-159 (first row of shorthand table); Probe 3 witnesses long-form equivalence |
| `self` (no `&`) is the *consuming* receiver | Book ch05-03 lines 175-180 (the reading/mutating/consuming split) |
| The shape is rare, used to transform `self` into something else | Book ch05-03 lines 76-79 verbatim |
| After `w.into_inner()`, the binding `w` cannot be used again | Probe 2 witnesses E0382 verbatim |
| The diagnostic's `note:` block names the receiver shape directly | Probe 2 transcript verbatim — `Wrapper::into_inner` takes ownership of the receiver `self`, which moves `w` |
| Three receiver shapes total: `&self`, `&mut self`, `self` | Book ch05-03 lines 67-70 and 175-180; lessons 100, 101, 102 install one each |
| `Wrapper` is non-`Copy`, hence the move | Probe 2's "does not implement the `Copy` trait" annotation; corpus `output/docs/rust/error_codes/E0382.md` lines 19-22 |
| Long-form `self: Self` compiles identically to bare `self` | Probe 3 transcript verbatim |

No lesson claim relies on a fact that does not appear in either a
listed corpus passage or a captured probe.
