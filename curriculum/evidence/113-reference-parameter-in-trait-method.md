# Evidence — 113-reference-parameter-in-trait-method

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

- `which rustc`: `/Users/eli/.cargo/bin/rustc`.
- Probes run in `/tmp/eduratchet-113/`. Same toolchain family as
  recently accepted lessons (107-112).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/113-reference-parameter-in-trait-method.rs`
is the working probe verbatim with header comments naming the
centered E0308 contrast captured below.

## Sources

### `output/docs/rust/book/ch04-02-references-and-borrowing.md`

The Book's chapter 4.2, the canonical reference-and-borrowing
introduction. This is the primary corpus warrant for what `&Type`
*means*.

#### Lines 7-11 — references defined

> A reference is like a pointer in that it's an address we can
> follow to access the data stored at that address; that data is
> owned by some other variable. Unlike a pointer, a reference is
> guaranteed to point to a valid value of a particular type for
> the life of that reference.

Corpus warrant for the framing "a reference is a pointer to a value
owned by something else." Today's lesson does not introduce
"pointer" as a vocabulary term to the learner; it uses the simpler
phrase "a reference *to* a value."

#### Lines 32-36 — `&` at type and expression positions

> note that we pass `&s1` into `calculate_length` and, in its
> definition, we take `&String` rather than `String`. These
> ampersands represent references, and they allow you to refer to
> some value without taking ownership of it.

Direct corpus warrant for the lesson's two coupled mechanics:
`&Type` in the parameter list, and `&value` at the call site, both
as ampersands. The Book uses `&String` and `&s1`; today's lesson
uses `&Counter` and `&b` — same shape, learner's struct in the
type position.

#### Lines 66-68 — `&value` does not own

> The `&s1` syntax lets us create a reference that *refers* to the
> value of `s1` but does not own it. Because the reference does not
> own it, the value it points to will not be dropped when the
> reference stops being used.

This is the sentence the lesson body quotes inline. Direct corpus
warrant for the "still owns" property the working probe witnesses
empirically: after `a.combine(&b)`, `b` is still owned by `main`.

#### Lines 95-97 — borrowing as the named action

> We call the action of creating a reference *borrowing*. As in
> real life, if a person owns something, you can borrow it from
> them. When you're done, you have to give it back. You don't
> own it.

Corpus warrant for the term "borrow" used in the lesson body
("`&value` argument *borrows*") and in rustc's diagnostic
(`help: consider borrowing here`).

### `output/docs/rust/reference/types/pointer.md`

The Reference's pointer-types page. Provides the formal grammar.

#### Line 18 — reference type grammar

> ReferenceType → & Lifetime? mut? TypeNoBounds

Corpus warrant for the `&Type` shape as a *type* — the grammar
production places `&` before an optional lifetime, an optional
`mut`, and a `TypeNoBounds`. Today's `&Counter` instantiates this
production with no `Lifetime`, no `mut`, and `TypeNoBounds = Counter`.
Lifetimes and `mut` are deferred.

#### Lines 22-30 — shared references

> Shared references point to memory which is owned by some other
> value.
> ...
> When a shared reference to a value is created, it prevents
> direct mutation of the value. Interior mutability provides an
> exception for this in certain circumstances. As the name
> suggests, any number of shared references to a value may exist.
> A shared reference type is written `&type`, or `&'a type` when
> you need to specify an explicit lifetime.

Corpus warrant for `&Type` (no `mut`) being the *shared-reference*
form. The "any number may exist" clause is consistent with the
working probe creating two separate `&b` references (one per call)
without conflict. The exclusivity-with-`&mut` rule and the
lifetime form `&'a Counter` are deferred.

### `output/docs/rust/error_codes/E0308.md`

The error-code page for the centered contrast probe.

> Expected type did not match the received type.
> ...
> This error occurs when an expression was used in a place where
> the compiler expected an expression of a different type. It can
> occur in several cases, the most common being when calling a
> function and passing an argument which has a different type
> than the matching type in the function declaration.

Corpus warrant for the headline `error[E0308]: mismatched types`
and the fact that the most common case is "calling a function and
passing an argument which has a different type than the matching
type in the function declaration" — exactly today's contrast.

### `/Users/eli/InfoScraper/output/repos/rmp/src/biguint/cmp.rs`

The rmp source target.

#### Lines 4-8 — `eq` with `&BigUInt` parameter

```rust
impl PartialEq<BigUInt> for BigUInt {
    fn eq(&self, other: &BigUInt) -> bool {
        self.limbs == other.limbs
    }
}
```

Corpus warrant for the named-type `&Counter` (here: `&BigUInt`)
form being the directly readable shape used in rmp. Today's
`fn combine(&self, other: &Counter) -> u32` has exactly the
same parameter pattern modulo the type name and return type.
Once today is installed, the parameter list `(&self, other: &BigUInt)`
in `cmp.rs:5` is fully readable. The `<BigUInt>` generic trait
parameter on the trait header and the `Vec` equality on `self.limbs`
remain deferred.

#### Lines 12-19 — `&Self` form coexists in the same file

```rust
impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigUInt {
    fn cmp(&self, other: &Self) -> Ordering {
```

Corpus warrant for the lesson's note that `&Self` and `&Counter`
(here: `&BigUInt`) name the same type inside the impl: the *same
file* uses `other: &BigUInt` on line 5 and `other: &Self` on lines
13 and 19. Today's lesson centers the named-type spelling because
it is directly readable; the `&Self` alternative is named in
*What To Ignore For Now* as a stylistic deferral.

## Probes

All probes run in `/tmp/eduratchet-113/`. The committed observation
file is the working probe verbatim.

### Probe 1 — working (`demo.rs`)

```rust
struct Counter {
    count: u32,
}

trait Combine {
    fn combine(&self, other: &Counter) -> u32;
}

impl Combine for Counter {
    fn combine(&self, other: &Counter) -> u32 {
        self.count + other.count
    }
}

fn main() {
    let a = Counter { count: 7 };
    let b = Counter { count: 35 };
    let first = a.combine(&b);
    let second = a.combine(&b);
    println!("first  = {}", first);
    println!("second = {}", second);
    println!("b.count still = {}", b.count);
}
```

```
$ /Users/eli/.cargo/bin/rustc demo.rs
$ echo $?
0
$ ./demo
first  = 42
second = 42
b.count still = 35
$ echo $?
0
```

Compiles silently. Prints `first  = 42` (7 + 35), `second = 42`
(same result on the second call — `b` was not consumed by the
first call), and `b.count still = 35` (`b` is alive after both
calls and its field is readable directly). The two-call pattern
plus the final field read is the empirical witness for the
"caller still owns" property of `&value` arguments.

### Probe 2 — centered E0308 contrast (`no_amp.rs`)

Source: same as Probe 1 with one diff — drop the `&` from the
first argument: `a.combine(b)` instead of `a.combine(&b)`. Trait
and impl signatures unchanged. The `let second = ...` line and
the `b.count still = ...` print are removed so that the only
diagnostic is the type mismatch on the bare `b` argument (rather
than a downstream "use of moved value" cascade).

```rust
struct Counter {
    count: u32,
}

trait Combine {
    fn combine(&self, other: &Counter) -> u32;
}

impl Combine for Counter {
    fn combine(&self, other: &Counter) -> u32 {
        self.count + other.count
    }
}

fn main() {
    let a = Counter { count: 7 };
    let b = Counter { count: 35 };
    let first = a.combine(b);
    println!("first  = {}", first);
}
```

```
$ /Users/eli/.cargo/bin/rustc no_amp.rs
error[E0308]: mismatched types
  --> no_amp.rs:18:27
   |
18 |     let first = a.combine(b);
   |                   ------- ^ expected `&Counter`, found `Counter`
   |                   |
   |                   arguments to this method are incorrect
   |
note: method defined here
  --> no_amp.rs:6:8
   |
 6 |     fn combine(&self, other: &Counter) -> u32;
   |        ^^^^^^^        -----
help: consider borrowing here
   |
18 |     let first = a.combine(&b);
   |                           +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
$ echo $?
1
```

Diagnostic shape mapped to lesson 003's four-part frame:

- *Headline*: `error[E0308]: mismatched types`. The generic
  type-mismatch code; the corpus page (`E0308.md`) names the most
  common case as "calling a function and passing an argument
  which has a different type than the matching type in the
  function declaration" — exactly today's case.
- *Caret*: `no_amp.rs:18:27`, single caret directly under the
  bare `b` at the call site. Inline label
  `expected `&Counter`, found `Counter`` — verbatim the rule
  today installs (the trait declares the parameter type as
  `&Counter`; the call site passed a `Counter`).
- *Secondary annotation* on the call line: dashes under
  `a.combine` followed by `arguments to this method are
  incorrect`. This points at the dispatch target; the carets
  point at the offending argument.
- *`note: method defined here`*: a second `-->` line at
  `no_amp.rs:6:8`, carets under `fn combine` and dashes under the
  parameter list. Points back at the trait declaration as the
  contract — the same diagnostic shape lessons 100, 109, 111, 112
  installed for E0599 / E0603 / E0053.
- *`help: consider borrowing here`*: proposes inserting a single
  `+` (the `&` character) before `b` at the call site. The diff
  from `a.combine(b)` to `a.combine(&b)` is exactly the lesson's
  centered teaching: the call-site `&` is what makes the bare
  value `b` (type `Counter`) into a reference (type `&Counter`).
- *Trailer*: standard `error: aborting due to 1 previous error`
  and `--explain E0308`.

Note on what this rustc 1.95.0 transcript does *not* contain: the
`note: expected reference `&Counter` / found struct `Counter``
expansion that the orchestrator brief mentioned does not appear in
this version's output for this case. The inline label
`expected `&Counter`, found `Counter`` carries the same information
in compressed form, so the lesson body grounds to the actual
transcript verbatim.

### Probe 3 — auxiliary E0382 (`consumed.rs`)

Captured to demonstrate why the `&Type` parameter shape is
load-bearing for the working probe's two-call pattern. Source:
the trait/impl signatures change `other: &Counter` to
`other: Counter` (ownership-passing form), the impl body's
`other.count` is unchanged (Counter's `count: u32` is still
readable through the moved value), and the call sites are
`a.combine(b)` and `a.combine(b)` (no `&`).

```rust
struct Counter {
    count: u32,
}

trait Combine {
    fn combine(&self, other: Counter) -> u32;
}

impl Combine for Counter {
    fn combine(&self, other: Counter) -> u32 {
        self.count + other.count
    }
}

fn main() {
    let a = Counter { count: 7 };
    let b = Counter { count: 35 };
    let first = a.combine(b);
    let second = a.combine(b);
    println!("first  = {}", first);
    println!("second = {}", second);
}
```

```
$ /Users/eli/.cargo/bin/rustc consumed.rs
error[E0382]: use of moved value: `b`
  --> consumed.rs:19:28
   |
17 |     let b = Counter { count: 35 };
   |         - move occurs because `b` has type `Counter`, which does not implement the `Copy` trait
18 |     let first = a.combine(b);
   |                           - value moved here
19 |     let second = a.combine(b);
   |                            ^ value used here after move
   |
note: consider changing this parameter type in method `combine` to borrow instead if owning the value isn't necessary
  --> consumed.rs:6:30
   |
 6 |     fn combine(&self, other: Counter) -> u32;
   |        -------               ^^^^^^^ this parameter takes ownership of the value
   |        |
   |        in this method
note: if `Counter` implemented `Clone`, you could clone the value
  --> consumed.rs:1:1
   |
 1 | struct Counter {
   | ^^^^^^^^^^^^^^ consider implementing `Clone` for this type
...
18 |     let first = a.combine(b);
   |                           - you could clone this value
   |
error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
$ echo $?
1
```

Empirical witness for *why* today's `&Counter` parameter shape
matters: with `other: Counter` (no `&`), the first call consumes
`b`, and the second call fires E0382 *use of moved value*. The
`note: consider changing this parameter type in method `combine`
to borrow instead if owning the value isn't necessary` points
directly at the trait declaration's `Counter` parameter type and
labels it `this parameter takes ownership of the value` — rustc
itself frames the choice as "ownership-passing parameter vs.
borrowing parameter." The lesson body does not center this
diagnostic (E0308 on the call site is the centered contrast); this
appendix probe is footnote-style only, grounding the deferral
about consuming-receiver alternatives.

### Probe 4 — Check Yourself (b) ground (`tiny.rs`)

```rust
struct Tally { n: u32 }

trait Sum { fn sum(&self, other: &Tally) -> u32; }

impl Sum for Tally {
    fn sum(&self, other: &Tally) -> u32 {
        self.n + other.n
    }
}

fn main() {
    let x = Tally { n: 10 };
    let y = Tally { n: 5 };
    let s1 = x.sum(&y);
    let s2 = x.sum(&y);
    println!("s1 = {}, s2 = {}, y.n = {}", s1, s2, y.n);
}
```

```
$ /Users/eli/.cargo/bin/rustc tiny.rs
$ echo $?
0
$ ./tiny
s1 = 15, s2 = 15, y.n = 5
$ echo $?
0
```

Grounds Check Yourself (a) "yes, silent compile" and (b)
`s1 = 15, s2 = 15, y.n = 5`.

### Probe 5 — Check Yourself (c) ground (`tiny_c.rs`)

Source: Probe 4 with the second call site changed from
`x.sum(&y)` to `x.sum(y)` (drop the `&`).

```rust
struct Tally { n: u32 }

trait Sum { fn sum(&self, other: &Tally) -> u32; }

impl Sum for Tally {
    fn sum(&self, other: &Tally) -> u32 {
        self.n + other.n
    }
}

fn main() {
    let x = Tally { n: 10 };
    let y = Tally { n: 5 };
    let s1 = x.sum(&y);
    let s2 = x.sum(y);
    println!("s1 = {}, s2 = {}, y.n = {}", s1, s2, y.n);
}
```

```
$ /Users/eli/.cargo/bin/rustc tiny_c.rs
error[E0308]: mismatched types
  --> tiny_c.rs:15:20
   |
15 |     let s2 = x.sum(y);
   |                --- ^ expected `&Tally`, found `Tally`
   |                |
   |                arguments to this method are incorrect
   |
note: method defined here
  --> tiny_c.rs:3:16
   |
 3 | trait Sum { fn sum(&self, other: &Tally) -> u32; }
   |                ^^^        -----
help: consider borrowing here
   |
15 |     let s2 = x.sum(&y);
   |                    +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
$ echo $?
1
```

Single E0308 with caret on the bare `y`, label
`expected `&Tally`, found `Tally``, and `help: consider borrowing
here` proposing the `&` insertion. Grounds Check Yourself (c).

## Claim-to-evidence map

| Lesson claim                                                    | Source                                                                  |
|-----------------------------------------------------------------|-------------------------------------------------------------------------|
| `&Type` is a *reference type*                                   | `book/ch04-02-references-and-borrowing.md:7-11, 32-36`; `reference/types/pointer.md:18, 22-30` |
| `&value` creates a reference and does not own the value         | `book/ch04-02-references-and-borrowing.md:66-68` (lesson body quote)    |
| Caller still owns `value` after passing `&value`                | `book/ch04-02-references-and-borrowing.md:66-68`; Probe 1 (two calls + final field read) |
| Borrowing is the named action of creating a reference           | `book/ch04-02-references-and-borrowing.md:95-97`                        |
| `&Type` can sit in any parameter slot (not just receiver)       | `reference/types/pointer.md:18` (grammar); `book/ch04-02-references-and-borrowing.md:32-36` (`fn calculate_length(s: &String)`); rmp `cmp.rs:5` (`other: &BigUInt` non-receiver) |
| Reading `other.field` mirrors reading `self.field` through `&self` | Lesson 100 (load-bearing — installed `self.field` on `&self`)         |
| Working probe prints `first  = 42 / second = 42 / b.count still = 35` | Probe 1 (transcript verbatim)                                     |
| Without `&` at the call site, rustc fires E0308                 | Probe 2 (transcript verbatim); `error_codes/E0308.md:29-32`             |
| Inline label `expected `&Counter`, found `Counter``             | Probe 2 (verbatim)                                                       |
| `help: consider borrowing here` proposes the `&` insertion       | Probe 2 (verbatim, single `+` marker)                                   |
| `note: method defined here` block points at the trait declaration | Probe 2 (verbatim, second `-->` at line 6:8)                          |
| Inside the impl, `Self` and the type name (`Counter`) are aliased | Lesson 100 (cited; restated today); rmp `cmp.rs:13, 19` (uses `&Self`); rmp `cmp.rs:5` (uses `&BigUInt`) |
| Trait/impl method signature shape with parameter beyond `&self` | Lesson 112 (load-bearing)                                                |
| Impl signature reproduces trait signature exactly               | Lesson 112 (load-bearing)                                                |
| Dot-call shape `value.method(arg)` is unchanged                 | Lesson 040 (load-bearing); Probe 1 call sites                            |
| rmp `cmp.rs:5` `(&self, other: &BigUInt) -> bool` becomes readable | rmp `cmp.rs:4-8` (corpus); today's outer signature shape unlocks this |
| Auxiliary: passing a value (no `&`) to a consuming parameter type fires E0382 on the second call | Probe 3 (transcript verbatim)              |
| Check Yourself (a)/(b): `tiny.rs` compiles silently and prints `s1 = 15, s2 = 15, y.n = 5` | Probe 4 (transcript verbatim)                  |
| Check Yourself (c): single E0308 with the same shape           | Probe 5 (transcript verbatim)                                            |

## Direct prerequisite summary

- **Lesson 112** (load-bearing): installed the trait method shape
  `fn name(&self, p: T) -> R;`, the matching impl method body, and
  the contract-matching rule. Today's parameter type is a reference
  rather than a primitive; the rule that the impl signature
  reproduces the trait signature exactly is unchanged and load-bearing
  for the diagnostic in Probe 2 (the `note: method defined here`
  points at the trait declaration as the contract).
- **Lesson 100** (load-bearing): installed `&self` as the
  borrowing receiver, `self.field` as field access through the
  reference, and `Self` as a type alias for the impl-target type.
  Today extends `&Type` from the receiver to a non-receiver parameter
  slot, and `other.field` reuses the same auto-deref-on-field-access
  machinery the lesson installed for `self.field`. The lesson body's
  note that `&Self` and `&Counter` are interchangeable is a direct
  consequence of lesson 100's `Self`-as-alias rule.
- **Lesson 095** (load-bearing): `struct Name { field: Type }`,
  struct expression, field access. Two `Counter` values built and
  the `b.count` field read directly after the calls.
- **Lesson 008** (load-bearing): `(p1: T1, p2: T2)` parameter-list
  grammar. Today's `(&self, other: &Counter)` slots `&Counter` into
  the type position.
- **Lesson 040** (load-bearing): the dot-call shape
  `value.method(arg)`. The argument expression now begins with `&`.

## Older supporting lessons

- 002 (`fn main`), 005 (`let`), 009 (`+`), 011 (`println!` `{}`),
  019 (type-annotation slot), 080 (`u32`), 003 (diagnostic
  four-part map), 001 (`rustc demo.rs && ./demo`), 102 (the
  consuming-receiver `self` named in deferral as the contrast of
  contrasts). Each used in lesson 112 and unchanged today; cited
  only for the dependency record.

## Deferrals

Each item below was named in the lesson's *What To Ignore For Now*
and is not load-bearing for today's claims:

- `&Self` as an alias for `&Counter` inside an impl (stylistic
  choice; the rmp source uses both spellings).
- `&mut Type` as a non-receiver parameter (mutable borrows in a
  parameter slot).
- The full borrowing rules — borrow checker, shared-vs-mutable
  exclusivity, lifetime parameters (`&'a Counter`, `'static`).
- Multiple reference parameters in one signature.
- Reference-of-reference (`& &Counter`).
- `&Type` as a return type (`fn first(&self) -> &Counter`).
- Other receivers in trait methods (`&mut self`, `self` by value).
- Multi-type dispatch, default method bodies, generic trait
  parameters, associated types, operator traits.
