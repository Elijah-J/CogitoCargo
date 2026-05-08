# Evidence — Lesson 122: `self.method(args)` inside another method body

Lesson: `experimental/eduratchet2/runs/rust-moves/lessons/122-self-method-delegation.md`
Observation source: `experimental/eduratchet2/runs/rust-moves/observations/122-self-method-delegation.rs`
Observation transcript: `experimental/eduratchet2/runs/rust-moves/observations/122-self-method-delegation.transcript.txt`

## Toolchain

Captured on host:

```text
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ uname -srm
Darwin 24.5.0 x86_64
```

The probes below were typed into a fresh scratch directory
(`/tmp/eduratchet122/`) and compiled with `rustc <file>`; the
resulting executables were run from the same directory.

## Direct prerequisite — lesson 040

Lesson 040 installed:

- The dot-call shape `value.method(args)`: receiver expression, dot,
  method name, parenthesized argument list. The whole expression has
  whatever type the method returns and slots into expression
  positions.
- Method-call lookup: rustc finds the method by looking at the *type*
  of the receiver. (The auto-deref/auto-ref step is deferred; today
  reuses the lookup framing only.)

Today fills the *receiver* slot with `self` instead of a `let`-bound
name. The dot-call shape is unchanged.

## Direct prerequisite — lesson 100

Lesson 100 installed:

- Inherent `impl Type { ... }` as the block where methods are
  authored. Inside the block, `&self` is shorthand for `self: &Self`,
  and `Self` is an alias for the impl-target type (Book ch05-03 lines
  62-63 verbatim: "The `&self` is actually short for `self: &Self`.
  Within an `impl` block, the type `Self` is an alias for the type
  that the `impl` block is for").
- The diagnostic shape for a missing `&self` method: `error[E0599]:
  no method named \`X\` found for ...`. Lesson 100's contrast probe
  removed `&self` from a method's signature; today's contrast probe
  removes the method definition entirely. Both fire E0599.

## Direct prerequisite — lesson 095

Lesson 095 installed `struct Counter { n: u32 }`, the struct
expression `Counter { n: 7 }`, and field access `instance.field`.
Today reuses all three — `self.n` is field access on the implicit
receiver. The lesson body of `doubled` is exactly `self.n * 2`.

## Older supporting lessons

- **Lessons 002, 001** — `fn main` entry; `rustc demo.rs` then
  `./demo`, silent on success.
- **Lesson 005** — `let c = Counter { n: 7 };` and the equivalents in
  the contrast and Check-Yourself probes.
- **Lesson 011** — `println!("{}: {}", label, expr)` with positional
  `{}` slots formatting `u32` values via `Display`. Two calls in the
  working probe.
- **Lesson 003** — the diagnostic four-part map (headline, location,
  source excerpt with caret, optional help). Used to read Probe 2's
  E0599 transcript.
- **Lesson 009** — `*` on integers, used as `self.n * 2` and
  `self.doubled() * 2`.
- **Lesson 080** — `u32` for the field type and method return types.
- **Lesson 019** — the `: TYPE` annotation slot; today reuses the
  return-type form `-> u32` (the slot was extended in lesson 021).

## Probe 1 — working probe (`self.doubled()` inside `quadrupled`)

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/122-self-method-delegation.rs`
is the probe.

```rust
struct Counter { n: u32 }

impl Counter {
    fn doubled(&self) -> u32 {
        self.n * 2
    }
    fn quadrupled(&self) -> u32 {
        self.doubled() * 2
    }
}

fn main() {
    let c = Counter { n: 7 };
    println!("doubled    = {}", c.doubled());
    println!("quadrupled = {}", c.quadrupled());
}
```

Compile and run on host (full transcript at
`observations/122-self-method-delegation.transcript.txt`):

```text
$ rustc demo.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./demo
doubled    = 14
quadrupled = 28
$ echo "run-exit=$?"
run-exit=0
```

The centered claim — "`self.method(args)` inside a method body is
exactly the dot-call shape with `self` as the receiver" — is carried
by line 8: `self.doubled() * 2`. The output `quadrupled = 28` is the
end-to-end witness: `quadrupled`'s body called `doubled` on the same
`self`, got `14`, multiplied by `2`.

## Probe 2 — centered contrast (E0599 for missing sibling method)

`broken.rs` drops the `fn doubled` definition but leaves the body of
`quadrupled` referencing it:

```rust
struct Counter { n: u32 }

impl Counter {
    fn quadrupled(&self) -> u32 {
        self.doubled() * 2
    }
}

fn main() {
    let c = Counter { n: 7 };
    println!("quadrupled = {}", c.quadrupled());
}
```

Compile result, captured verbatim:

```text
error[E0599]: no method named `doubled` found for reference `&Counter` in the current scope
 --> broken.rs:5:14
  |
5 |         self.doubled() * 2
  |              ^^^^^^^ method not found in `&Counter`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
exit=1
```

The diagnostic states today's rule verbatim. The headline is E0599
(same E-code lesson 100 installed for the missing-`&self`-method
case; `error_codes/E0599.md` line 4: "This error occurs when a method
is used on a type which doesn't implement it"). The caret at column
14 of line 5 lands inside `quadrupled`'s body — *not* at a `main`-
side call — and the inline label "method not found in \`&Counter\`"
names the type `self` has at that site.

The receiver type `&Counter` (rather than `Counter`) is the visible
trace of the `&self` shorthand: lesson 100 cited Book ch05-03:62-63
that "`&self` is actually short for `self: &Self`", so inside an
`&self` body, `self` has type `&Self` = `&Counter`. Today does not
re-derive this — the diagnostic surface is named, the rule is cited
to lesson 100.

## Probe 3 — corroborating (path form `Counter::doubled(self)`)

Same source as Probe 1 with `quadrupled`'s body switched from the
dot form to the named-type path form:

```rust
struct Counter { n: u32 }

impl Counter {
    fn doubled(&self) -> u32 { self.n * 2 }
    fn quadrupled(&self) -> u32 { Counter::doubled(self) * 2 }
}

fn main() {
    let c = Counter { n: 7 };
    println!("quadrupled = {}", c.quadrupled());
}
```

Compile and run:

```text
$ rustc path.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./path
quadrupled = 28
$ echo "run-exit=$?"
run-exit=0
```

`diff <(./demo | tail -1) <(./path)` exits 0 — `quadrupled = 28` is
byte-identical between the dot form and the named-type path form.
This corroborates lesson 100's auxiliary Probe 4 fact ("`Counter::current(&c)`
compiles equivalently to `c.current()`") composed with today's
"the receiver can be `self`": the dot form `self.doubled()` produces
the same value as the path form `Counter::doubled(self)` when the
method takes `&self`.

Note: the path form passes `self` (not `&self`) as the explicit
receiver argument. rustc accepts this because of the auto-ref step in
method-call resolution (Reference `expressions/method-call-expr.md:39`
"for each candidate `T`, add `&T` and `&mut T` to the list"). Inside
`quadrupled`'s body, `self` already has type `&Counter`, which
matches the `&self` shorthand expansion `self: &Self = self: &Counter`.
The auto-ref machinery is deferred from lesson 100; today notes it
only as the structural reason the path-form variant compiles.

The lesson body centers the dot form and only mentions the path form
in `## What To Ignore For Now`. The named-type path form is corroboration,
not a centered claim.

## Probe 4 — Check Yourself (a)/(b) ground

`tiny.rs` (the Check-Yourself working program):

```rust
struct Tally { n: u32 }

impl Tally {
    fn add_one(&self) -> u32 {
        self.n + 1
    }
    fn add_three(&self) -> u32 {
        self.add_one() + 2
    }
}

fn main() {
    let t = Tally { n: 10 };
    println!("add_three = {}", t.add_three());
}
```

```text
$ rustc tiny.rs
$ echo "compile-exit=$?"
compile-exit=0
$ ./tiny
add_three = 13
$ echo "run-exit=$?"
run-exit=0
```

Confirms (a) yes, silent compile; (b) `add_three = 13`.

## Probe 5 — Check Yourself (c) ground

`tiny_broken.rs` drops `fn add_one` but leaves `self.add_one() + 2`:

```rust
struct Tally { n: u32 }

impl Tally {
    fn add_three(&self) -> u32 {
        self.add_one() + 2
    }
}

fn main() {
    let t = Tally { n: 10 };
    println!("add_three = {}", t.add_three());
}
```

```text
$ rustc tiny_broken.rs
error[E0599]: no method named `add_one` found for reference `&Tally` in the current scope
 --> tiny_broken.rs:5:14
  |
5 |         self.add_one() + 2
  |              ^^^^^^^ method not found in `&Tally`

error: aborting due to 1 previous error
```

Confirms (c) E0599; the inline label says "method not found in
`&Tally`".

## Why this works — Reference & Book grounding

### Book `output/docs/rust/book/ch05-03-method-syntax.md` lines 9-11

Verbatim:

> Unlike functions, methods are defined within the context of a struct
> (or an enum or a trait object…), and their first parameter is always
> `self`, which represents the instance of the struct the method is
> being called on.

This grounds the framing that `self` *names a value* — the instance —
inside the method body. A value can sit in the receiver slot of a
dot-call (lesson 040), so `self.method(args)` is well-formed by the
same grammar as `c.method(args)`.

### Book `output/docs/rust/book/ch05-03-method-syntax.md` lines 61-67

Verbatim (the same span lesson 100 cited):

> In the signature for `area`, we use `&self` instead of `rectangle:
> &Rectangle`. The `&self` is actually short for `self: &Self`. Within
> an `impl` block, the type `Self` is an alias for the type that the
> `impl` block is for.

Inside `impl Counter { ... }`, `Self` = `Counter`, so `&self` =
`self: &Counter`. The contrast probe's diagnostic surfaces this
expansion verbatim by saying `&Counter` (not just `Counter`).

### Book `output/docs/rust/book/ch05-03-method-syntax.md` lines 138-148

Verbatim:

> Rust doesn't have an equivalent to the `->` operator; instead, Rust
> has a feature called *automatic referencing and dereferencing*.
> Calling methods is one of the few places in Rust with this behavior.
>
> Here's how it works: When you call a method with `object.something()`,
> Rust automatically adds in `&`, `&mut`, or `*` so that `object`
> matches the signature of the method.

This is the auto-ref/deref machinery deferred from lesson 100. Probe
3's path-form variant (passing `self` of type `&Counter` to
`Counter::doubled(self)` whose first parameter is `&self` =
`&Counter`) needs no auto-ref: `self`'s type already matches. Probe 1's
dot-form variant `self.doubled()` likewise needs no extra auto-ref —
`self` is already `&Counter`. Today does not exercise the
auto-ref step centrally; the lesson body cites lesson 100 for the
deferral, not this Book span.

### Reference `output/docs/rust/reference/expressions/method-call-expr.md` lines 13-14

Verbatim:

> A *method call* consists of an expression (the *receiver*) followed
> by a single dot, an expression path segment, and a parenthesized
> expression-list.

This is the formal grammar lesson 040 already cited. The receiver is
"an expression" — any expression of an appropriate type. `self`
inside a method body is a parameter binding, which is an expression
of type `&Self` / `&mut Self` / `Self` depending on the receiver
shape. Today fills "the expression" slot with the `self` parameter.

### Reference `output/docs/rust/reference/items/associated-items.md` lines 109-111

Verbatim:

> Associated functions whose first parameter is named `self` are
> called *methods* and may be invoked using the [method call
> operator](../expressions/method-call-expr.md), for example, `x.foo()`,
> as well as the usual function call notation.

Confirms two call shapes for the same method: the dot form `x.foo()`
and the path form. Probes 1 and 3 witness both shapes producing
identical output for the same sibling-call.

### Error code `output/docs/rust/error_codes/E0599.md` line 4

Verbatim:

> This error occurs when a method is used on a type which doesn't
> implement it.

Probe 2's headline `error[E0599]: no method named \`doubled\` found
for reference \`&Counter\`` is exactly this error: the dot-call
references `doubled`, the type of the receiver is `&Counter`, and
`Counter` does not implement `doubled` (because we deleted its
definition). The diagnostic's choice of `&Counter` rather than
`Counter` is the visible trace of the `&self` shorthand expansion.

## rmp unlock — `cmp.rs:14` `Some(self.cmp(other))`

Source `output/repos/rmp/src/biguint/cmp.rs` lines 18-20 verbatim:

```rust
impl Ord for BigUInt {
    fn cmp(&self, other: &Self) -> Ordering {
```

And `cmp.rs:13-15`:

```rust
impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
```

`self.cmp(other)` on line 14 is the rmp-source instance of today's
shape. Inside `partial_cmp`'s body, `self` has type `&BigUInt`
(the `&self` shorthand expansion); `cmp` is a sibling method declared
on the same type via the `impl Ord for BigUInt` block (lesson 111
installed `impl Trait for Type`); the receiver of the dot-call is the
`self` parameter; the method name is `cmp`; the argument is `other`,
itself a parameter binding of type `&Self = &BigUInt`. The whole
expression `self.cmp(other)` evaluates to an `Ordering`, which is
then wrapped in `Some(...)`.

Today reads only the call expression `self.cmp(other)`. The `Some(...)`
wrapper composes lesson 119's `Option::Some`. The full body is
readable end-to-end after lesson 119 + today.

A second rmp instance — `cmp.rs:20` `self.limbs.len().cmp(...)` —
chains: `self.limbs` is field access (lesson 095), `.len()` is a
method call on the field, and `.cmp(...)` is a method call on `len`'s
return value (lesson 049's chaining mechanic). The first link
`self.limbs` is *field access*, not method delegation; today does
not center it. The chain is readable after today + 049 + 107 (Vec
basics for `len`).

## Verbatim corpus quotes

### Book `output/docs/rust/book/ch05-03-method-syntax.md`

Line 9-11 (quoted in full above) — `self` represents the instance.

Lines 27-34 — Listing 5-13's `area` method on `Rectangle`:

> ```
> impl Rectangle {
>     fn area(&self) -> u32 {
>         self.width * self.height
>     }
> }
> ```

The body uses `self.width` and `self.height` (field access on the
implicit receiver). It does not exercise method delegation, but it
witnesses that `self` is a *value* — the same `self` whose
field-access form we read here would equally support a dot-call.
Today's working probe extends from `self.field` (Listing 5-13) to
`self.method()`.

Lines 61-67 (quoted in full above) — `&self` shorthand and `Self`
alias.

### Reference `output/docs/rust/reference/expressions/method-call-expr.md`

Lines 13-14 — the grammar. Same span lesson 040 cited.

Line 39 — auto-ref step:

> Then, for each candidate `T`, add `&T` and `&mut T` to the list
> immediately after `T`.

Cited only for Probe 3's path-form variant; not centered today.

### Reference `output/docs/rust/reference/items/associated-items.md`

Lines 109-111 (quoted in full above) — methods invokable via dot or
the usual function call notation.

### Error codes `output/docs/rust/error_codes/E0599.md`

Line 4 (quoted in full above) — the error description.

### rmp `output/repos/rmp/src/biguint/cmp.rs`

Lines 13-15 (quoted in full above) — `partial_cmp` body containing
`self.cmp(other)`. Lines 18-19 — the sibling `impl Ord for BigUInt`
declaring `cmp(&self, other: &Self) -> Ordering`.

## Claim-to-evidence map

- "`self.method(args)` is exactly lesson 040's dot-call shape with
  `self` filling the receiver slot" — Reference
  `method-call-expr.md:13-14` (the grammar requires "an expression"
  in the receiver slot); Probe 1 transcript (compiles silently, runs
  to expected output).
- "Inside an `&self` method body, `self` has type `&Self`" — Book
  ch05-03 lines 61-67 verbatim. Probe 2's diagnostic surfaces
  `&Counter` (not bare `Counter`) at the receiver.
- "Method name is looked up on the type of the receiver" — lesson
  040; restated structurally on the contrast probe whose diagnostic
  says "method not found in \`&Counter\`".
- "`self.doubled()` produces a `u32` from `Counter`'s declared return
  type" — Probe 1 transcript output (`14` printed).
- "If the named method is not on the type, E0599 fires" — Probe 2
  transcript verbatim; `error_codes/E0599.md:4` verbatim; lesson 100
  installed E0599 for the parallel "missing `&self`" case.
- "The diagnostic location lands inside the method body, not in
  `main`" — Probe 2 transcript shows `--> broken.rs:5:14` pointing
  inside `quadrupled`'s body.
- "rmp `cmp.rs:14` `Some(self.cmp(other))` is exactly today's
  mechanic" — `output/repos/rmp/src/biguint/cmp.rs:14` verbatim;
  Probe 1 mirrors the shape.
- "Path-form `Counter::doubled(self)` produces equivalent output to
  the dot form" — Probe 3 transcript; sanity diff exits 0;
  `associated-items.md:109-111` verbatim ("may be invoked using the
  method call operator … as well as the usual function call notation").

## Negative / contrast probe coverage

The lesson's centered contrastive claim is "`self.method()` requires
`method` to exist on the type of `self`; if it does not, the dot-call
fails to resolve." Probe 2 (E0599 with `fn doubled` deleted) is the
centered contrast and is captured verbatim.

A second contrast — keeping `fn doubled` but renaming it `fn doubld`
— was *not* attempted. It would produce structurally the same
diagnostic (E0599, "method not found in \`&Counter\`") with a
different surface name; nothing new is witnessed beyond Probe 2.

The path-form corroboration (Probe 3) is *not* a contrast; it is a
positive witness that the dot-form and the named-type path form
produce identical output. Today centers the dot form; the path form
is in `## What To Ignore For Now`.

The Check-Yourself (c) ground (Probe 5) is a second instance of the
centered contrast on a different struct/method-name pair, included
to ground the answer key.
