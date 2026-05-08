# Evidence — 112-trait-method-extra-parameter

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version --verbose`:

  ```
  rustc 1.95.0 (59807616e 2026-04-14)
  binary: rustc
  commit-hash: 59807616e1fa2540724bfbac14d7976d7e4a3860
  commit-date: 2026-04-14
  host: x86_64-apple-darwin
  release: 1.95.0
  LLVM version: 22.1.2
  ```

- `uname -a`:

  ```
  Darwin MacBookPro.lan 24.5.0 Darwin Kernel Version 24.5.0: Tue Apr 22 19:53:26 PDT 2025; root:xnu-11417.121.6~2/RELEASE_X86_64 x86_64
  ```

- Probes run in `/tmp/eduratchet-112/` and `/tmp/eduratchet-112-revision/`
  on this host. Same toolchain family as recently accepted lessons
  (107-111).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/112-trait-method-extra-parameter.rs`
is the working probe verbatim, with header comments naming the
centered E0053 contrast captured below.

## Sources

### `output/docs/rust/reference/items/traits.md`

The Reference's *Traits* page. Lesson 111 already cited lines 10-15
(grammar) and line 43 (semicolon-replaces-body rule). Today's
load-bearing addition is that the function-signature grammar
referenced from the trait body's `AssociatedItem*` slot allows the
ordinary parameter list — i.e., the trait body's signature is the
same `fn` shape as a free function, with the body slot filled by
either `;` (no default) or `{ ... }` (default).

#### Lines 43-58 — three-variety associated items, signature shape

> Trait functions may omit the function body by replacing it with a
> semicolon. This indicates that the implementation must define the
> function. If the trait function defines a body, this definition
> acts as a default for any implementation which does not override
> it. Similarly, associated constants may omit the equals sign and
> expression to indicate implementations must define the constant
> value. Associated types must never define the type, the type may
> only be specified in an implementation.
>
> ```rust
> // Examples of associated trait items with and without definitions.
> trait Example {
>     const CONST_NO_DEFAULT: i32;
>     const CONST_WITH_DEFAULT: i32 = 99;
>     type TypeNoDefault;
>     fn method_without_default(&self);
>     fn method_with_default(&self) {}
> }
> ```

Corpus warrant for the signature-with-body-replaced-by-`;` rule
(lesson 111, restated today). Today's `Scale` trait is the same
shape with a parameter beyond `&self`; the signature line
`fn scaled(&self, factor: u32) -> u32;` slots into the same
`AssociatedItem` position as `fn method_without_default(&self);` in
this example.

#### Lines 73-84 — generic trait example uses extra parameters

> Type parameters can be specified for a trait to make it generic.
> These appear after the trait name, using the same syntax used in
> generic functions.
>
> ```rust
> trait Seq<T> {
>     fn len(&self) -> u32;
>     fn elt_at(&self, n: u32) -> T;
>     fn iter<F>(&self, f: F) where F: Fn(T);
> }
> ```

Corpus warrant for trait-method signatures with parameters beyond
`&self`. The Reference's example `fn elt_at(&self, n: u32) -> T;`
has exactly today's shape `(&self, name: u32) -> Return` — `&self`
plus one ordinary parameter. The `Seq<T>` generic-trait header is
deferred today; the parameter-list shape inside the method
signature is the load-bearing piece. Two Reference example methods
on this page (`elt_at`, `iter`) take parameters beyond `&self`, so
the syntactic possibility is set by the corpus.

### `output/docs/rust/error_codes/E0053.md`

The error-code page for the centered contrast probe. Short page,
quoted in full:

> The parameters of any trait method must match between a trait
> implementation and the trait definition.
>
> Erroneous code example:
>
> ```rust
> trait Foo {
>     fn foo(x: u16);
>     fn bar(&self);
> }
>
> struct Bar;
>
> impl Foo for Bar {
>     // error, expected u16, found i16
>     fn foo(x: i16) { }
>
>     // error, types differ in mutability
>     fn bar(&mut self) { }
> }
> ```

Corpus warrant for today's centered contrast and centered teaching:
"the parameters of any trait method must match between a trait
implementation and the trait definition." The page's example shows
both the parameter-type-mismatch case and the receiver-mismatch
case. Today's centered contrast probe stays with the
parameter-type-mismatch form (changing `factor: u32` to `factor: u64`
in the *trait* declaration so the impl body still typechecks with
no cast) because receiver mismatches involve mechanics deferred
from 111 (`&mut self` and `self` by value).

### `output/docs/rust/reference/items/implementations.md`

Already cited in lesson 111 (lines 10-24, the
`Implementation → InherentImpl | TraitImpl` split). Today reuses
the `TraitImpl` grammar verbatim from 111; the only diff is that
today's impl method has a longer parameter list. Not re-quoted.

### `output/docs/rust/error_codes/E0061.md`

Cited in earlier lessons (008, 100). Today's Probe 4 (auxiliary,
optional) was captured to confirm: when the call site
`c.scaled()` omits the argument that the trait method declares,
rustc fires E0061 *before* trying to resolve the trait impl
(arity-check is upstream of signature-match). Page text:

> An invalid number of arguments was passed when calling a function.
> ...
> The number of arguments passed to a function must match the
> number of arguments specified in the function signature.

Today's framing uses E0053 as the centered diagnostic; the E0061
auxiliary probe is in this appendix only.

### `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`

The rmp source already cited in lesson 111. Today's deferral
section names this file as the canonical example of a trait method
with a *reference* parameter beyond `&self`:

```rust
impl PartialEq<BigUInt> for BigUInt {
    fn eq(&self, other: &BigUInt) -> bool {
        self.limbs == other.limbs
    }
}
```

(lines 4-8). The shape `(&self, other: &BigUInt) -> bool` has
`&self` plus one extra parameter — exactly today's general shape —
but the parameter type is `&BigUInt` (a reference), not a
primitive. Today partially-unlocks reading this method modulo the
deferred `&BigUInt` parameter type and the deferred
`<BigUInt>` generic trait parameter on the trait header.

### `output/docs/rust/book/ch10-02-traits.md`

Already heavily cited in 111. Today's lesson does not introduce
new material from this chapter; the multi-method trait shape it
shows (Listing 10-12 has `summarize(&self) -> String` only; later
listings show traits with multiple methods, all `&self`-only) does
not exhibit a parameter-beyond-`&self` example until later
chapters (e.g., the iterator chapter). The Reference's `Seq<T>`
example (above) is the corpus citation for parameter-beyond-`&self`
syntax on a trait method.

## Probes

Probes 1, 3, 4 were run in `/tmp/eduratchet-112/`. Probes 2 and 5
were re-run in `/tmp/eduratchet-112-revision/` to ground the revised
centered contrast and the new Check Yourself (c) item with
byte-accurate transcripts. The committed observation file is the
working probe verbatim.

### Probe 1 — working (`demo.rs`)

```rust
struct Counter {
    count: u32,
}

trait Scale {
    fn scaled(&self, factor: u32) -> u32;
}

impl Scale for Counter {
    fn scaled(&self, factor: u32) -> u32 {
        self.count * factor
    }
}

fn main() {
    let c = Counter { count: 7 };
    println!("scaled = {}", c.scaled(6));
}
```

```
$ /Users/eli/.cargo/bin/rustc demo.rs
$ echo $?
0
$ ./demo
scaled = 42
$ echo $?
0
```

Compiles silently and prints `scaled = 42`. Three composed pieces:
trait declaration `Scale { fn scaled(&self, factor: u32) -> u32; }`
with one extra parameter beyond `&self`; impl block
`impl Scale for Counter { fn scaled(&self, factor: u32) -> u32 { self.count * factor } }`
whose signature matches the trait's exactly; call site
`c.scaled(6)` passing one argument.

### Probe 2 — centered E0053 contrast (`mismatch.rs`)

Source: same as Probe 1 with one diff — the *trait*'s `factor`
parameter type is changed from `u32` to `u64`. The impl is
unchanged, so its body `self.count * factor` still typechecks as
`u32 * u32` — no cast, no auxiliary diagnostics. The
trait-vs-impl signature mismatch is the only diagnostic. Re-run
in `/tmp/eduratchet-112-revision/` for byte-accurate line numbers
matching the lesson body's source rendering.

```rust
struct Counter {
    count: u32,
}

trait Scale {
    fn scaled(&self, factor: u64) -> u32;
}

impl Scale for Counter {
    fn scaled(&self, factor: u32) -> u32 {
        self.count * factor
    }
}

fn main() {
    let c = Counter { count: 7 };
    println!("scaled = {}", c.scaled(6));
}
```

```
$ /Users/eli/.cargo/bin/rustc mismatch.rs
error[E0053]: method `scaled` has an incompatible type for trait
  --> mismatch.rs:10:30
   |
10 |     fn scaled(&self, factor: u32) -> u32 {
   |                              ^^^ expected `u64`, found `u32`
   |
note: type in trait
  --> mismatch.rs:6:30
   |
 6 |     fn scaled(&self, factor: u64) -> u32;
   |                              ^^^
   = note: expected signature `fn(&Counter, u64) -> _`
              found signature `fn(&Counter, u32) -> _`
help: change the parameter type to match the trait
   |
10 -     fn scaled(&self, factor: u32) -> u32 {
10 +     fn scaled(&self, factor: u64) -> u32 {
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0053`.
$ echo $?
1
```

Diagnostic shape mapped to lesson 003's four-part frame:

- *Headline*: `error[E0053]: method `scaled` has an incompatible
  type for trait`. New E-code, name names today's rule.
- *Caret*: `mismatch.rs:10:30`, three carets under the impl's
  parameter type `u32`. The diagnostic flags the *impl*, not the
  trait — the trait is the contract.
- *Inline label*: `expected `u64`, found `u32``. The "expected"
  type is from the trait; "found" is from the impl.
- *`note: type in trait`*: a second `-->` line at
  `mismatch.rs:6:30` with caret directly under the trait
  declaration's `u64`. This is the load-bearing diagnostic feature
  for today's teaching: the diagnostic always points back at the
  trait as the source of truth.
- *Summary `= note:`*: `expected signature `fn(&Counter, u64) -> _`
  / found signature `fn(&Counter, u32) -> _``. Both signatures laid
  out side by side — the `_` in the return slot indicates the
  return type matched (only the parameter differed).
- *`help:`*: proposes the exact diff that brings the impl back into
  conformance.
- *Trailer*: standard `error: aborting due to 1 previous error`
  and `--explain E0053`.

### Probe 3 — symmetric witness with cast (`mismatch_cast.rs`)

To confirm that the *direction* of the mismatch does not matter —
that the diagnostic always treats the trait as the contract and the
impl as the variable — Probe 3 swaps the diff: the trait says
`factor: u32`, the impl declares `factor: u64`. Because that
makes the body's `self.count * factor` mix `u32 * u64`, the body
needs `(factor as u32)` to typecheck so the trait-signature
mismatch is the only diagnostic. The cast itself uses an `as`
expression that is *not* a prerequisite of the centered lesson —
this probe is included only to demonstrate symmetry, not to
ground the lesson body.

```rust
struct Counter {
    count: u32,
}

trait Scale {
    fn scaled(&self, factor: u32) -> u32;
}

impl Scale for Counter {
    fn scaled(&self, factor: u64) -> u32 {
        self.count * (factor as u32)
    }
}

fn main() {
    let c = Counter { count: 7 };
    println!("scaled = {}", c.scaled(6));
}
```

```
$ /Users/eli/.cargo/bin/rustc mismatch_cast.rs
error[E0053]: method `scaled` has an incompatible type for trait
  --> mismatch_cast.rs:10:30
   |
10 |     fn scaled(&self, factor: u64) -> u32 {
   |                              ^^^ expected `u32`, found `u64`
   |
note: type in trait
  --> mismatch_cast.rs:6:30
   |
 6 |     fn scaled(&self, factor: u32) -> u32;
   |                              ^^^
   = note: expected signature `fn(&Counter, u32) -> _`
              found signature `fn(&Counter, u64) -> _`
help: change the parameter type to match the trait
   |
10 -     fn scaled(&self, factor: u64) -> u32 {
10 +     fn scaled(&self, factor: u32) -> u32 {
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0053`.
$ echo $?
1
```

Same E0053 with the same `note: type in trait` shape. The
"expected"/"found" labels swap (the impl now declares `u64`, so
"found" is `u64`), but the structural message is unchanged: the
caret on the *impl* line is the primary location, the *trait* line
is the `note: type in trait` reference. Empirical witness for the
lesson's framing that the trait declaration is binding regardless
of which side gets edited. The lesson body centers Probe 2 (no
cast); Probe 3 is footnote-style only.

### Probe 4 — auxiliary E0061 (`no_arg.rs`)

Captured for completeness; the lesson body uses E0053 as the
centered diagnostic. Source: same as Probe 1 with the call site
changed to `c.scaled()` (zero arguments).

```
$ /Users/eli/.cargo/bin/rustc no_arg.rs
error[E0061]: this method takes 1 argument but 0 arguments were supplied
  --> no_arg.rs:17:31
   |
17 |     println!("scaled = {}", c.scaled());
   |                               ^^^^^^-- argument #1 of type `u32` is missing
   |
note: method defined here
  --> no_arg.rs:6:8
   |
 6 |     fn scaled(&self, factor: u32) -> u32;
   |        ^^^^^^        ------
help: provide the argument
   |
17 |     println!("scaled = {}", c.scaled(/* u32 */));
   |                                      +++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0061`.
$ echo $?
1
```

Same E0061 shape as lesson 008 installed for free functions, now on
a trait method. The `note: method defined here` block points at the
trait declaration (line 6:8 — at the `fn scaled` token). The
`-------` underline in the note covers the parameter list `(...)`
under the `factor` slot. Witness that arity-checking on a trait
method is the same machinery lesson 008 already installed.

### Probe 5 — Check Yourself (c) ground (`check_yourself_c.rs`)

Ground for the Check Yourself (c) answer. Source: lesson body's
`tiny.rs` with one diff — the *trait declaration*'s parameter
type `addend: u32` is changed to `addend: i32`. The impl is
unchanged, so its body `self.n + addend` still typechecks as
`u32 + u32` (because the impl declares `addend: u32`). The only
diagnostic is the trait-vs-impl signature mismatch.

```rust
struct Tally { n: u32 }

trait Plus { fn plus(&self, addend: i32) -> u32; }

impl Plus for Tally {
    fn plus(&self, addend: u32) -> u32 {
        self.n + addend
    }
}

fn main() {
    let t = Tally { n: 10 };
    println!("plus = {}", t.plus(5));
}
```

```
$ /Users/eli/.cargo/bin/rustc check_yourself_c.rs
error[E0053]: method `plus` has an incompatible type for trait
 --> check_yourself_c.rs:6:28
  |
6 |     fn plus(&self, addend: u32) -> u32 {
  |                            ^^^ expected `i32`, found `u32`
  |
note: type in trait
 --> check_yourself_c.rs:3:37
  |
3 | trait Plus { fn plus(&self, addend: i32) -> u32; }
  |                                     ^^^
  = note: expected signature `fn(&Tally, i32) -> _`
             found signature `fn(&Tally, u32) -> _`
help: change the parameter type to match the trait
  |
6 -     fn plus(&self, addend: u32) -> u32 {
6 +     fn plus(&self, addend: i32) -> u32 {
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0053`.
$ echo $?
1
```

Single E0053. The caret sits on the impl's `addend: u32` (line 6),
and the `note: type in trait` block points at the trait
declaration's `addend: i32` (line 3). No additional diagnostics
fire on the body's `self.n + addend` because both operands are
`u32` (impl declares `u32`). This grounds the Check Yourself (c)
answer that the single E-code is E0053.

## Claim-to-evidence map

| Lesson claim                                                    | Source                                                                  |
|-----------------------------------------------------------------|-------------------------------------------------------------------------|
| Trait method body may be `;` (no default)                       | `traits.md:43` (cited in 111, restated today)                          |
| Trait method may have parameters beyond `&self`                 | `traits.md:79-80` (`fn elt_at(&self, n: u32) -> T;`)                   |
| Parameter list shape `(name: T, ...)` is the same as free fn    | Lesson 008 (load-bearing prerequisite)                                  |
| Impl signature must match trait signature exactly               | `error_codes/E0053.md` lines 4-5                                       |
| E0053 fires on trait-vs-impl parameter-type mismatch (no cast)  | Probe 2 (transcript verbatim, no `as` cast)                            |
| `note: type in trait` block points at trait declaration         | Probe 2 (`note:` block at `mismatch.rs:6:30`)                          |
| Diagnostic structure invariant under direction of mismatch      | Probe 3 (transcript verbatim, same shape, swapped labels; uses cast)   |
| Check Yourself (c): single E0053 when trait flips to `i32`      | Probe 5 (transcript verbatim, only E0053 fires)                        |
| Working probe prints `scaled = 42`                              | Probe 1 (transcript)                                                    |
| Body uses field access × parameter binding × `*`                | Lessons 095, 008, 009 (load-bearing)                                    |
| Call site `c.scaled(6)` is dot-call with one argument           | Lesson 040 (load-bearing); Probe 1 call site                           |
| `&self` receiver attaches the method to the dot call            | Lesson 100 (cited; restated in 111)                                     |
| Trait declaration + impl is two-block shape from 111            | Lesson 111 (load-bearing prerequisite)                                  |
| Auxiliary: missing-argument call fires E0061                    | Probe 4 (transcript verbatim); `error_codes/E0061.md` lines 4-5        |
| rmp's `cmp.rs:5` has a parameter-beyond-`&self` (deferred type) | `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs:4-8`       |

## Direct prerequisite summary

- **Lesson 111** (load-bearing): installed `trait Name { fn method(&self) -> T; }`,
  the `impl Trait for Type { ... }` block, and the rule that the
  impl's signature reproduces the trait's. Today extends the
  parameter list with one extra slot and centers the
  contract-matching rule that 111 only stated in passing.
- **Lesson 008** (load-bearing): installed `(p1: T1, p2: T2, ...)`
  as the parameter-list grammar for `fn` items. Today reuses the
  shape after `&self,` inside trait and impl method signatures.
- **Lesson 040** (load-bearing): installed `value.method(arg)`.
  Today exercises the with-argument case explicitly on a trait
  method; the call site shape is unchanged.
- **Lesson 095** (load-bearing): installed `struct Name { field: Type }`
  and `self.field`. Today reuses both inside the impl method body.

## Older supporting lessons

- 002 (`fn main`), 005 (`let`), 009 (`*`), 011 (`println!` `{}`),
  019 (type-annotation slot), 080 (`u32`), 003 (diagnostic
  four-part map), 001 (`rustc demo.rs && ./demo`), 100 (`&self`
  receiver and inherent-vs-trait split). Each used in lesson 111
  and unchanged today; cited only for the dependency record.

## Deferrals

Each item below was named in the lesson's *What To Ignore For Now*
and is not load-bearing for today's claims:

- Reference parameters in non-receiver slots (`&Type`).
- Multiple extra parameters.
- Other receivers in trait methods (`&mut self`, `self` by value).
- Default method bodies in traits.
- Multiple types implementing one trait.
- Generic trait parameters, associated types, trait bounds.
- Operator traits, derive macros, lifetimes, the orphan rule.
