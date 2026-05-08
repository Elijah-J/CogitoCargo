# Evidence — 100-inherent-impl-and-self

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` -> `Darwin x86_64`
- Probes run in `/tmp/lesson100-probes/` on this host. Same toolchain
  family as recent accepted lessons (082-099).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/100-inherent-impl-and-self.rs`
is the working four-piece probe verbatim, with header comments naming
the centered E0599 contrast and the auxiliary E0061 probe captured
below.

## Sources

### `output/docs/rust/book/ch05-03-method-syntax.md`

The Book's *Method Syntax* chapter — the canonical lesson-anchor
chapter for today's move. Five load-bearing passages:

#### Lines 4-11 — what a method is

> Methods are similar to functions: We declare them with the `fn`
> keyword and a name, they can have parameters and a return value,
> and they contain some code that's run when the method is called
> from somewhere else. Unlike functions, methods are defined within
> the context of a struct (or an enum or a trait object, which we
> cover in Chapter 6 and Chapter 18, respectively), and their first
> parameter is always `self`, which represents the instance of the
> struct the method is being called on.

Corpus warrant for the *method-vs-free-function* framing the lesson
uses. Today's `current(&self)` is exactly the Book's method shape;
today's `new()` (no `self`) is the *associated-function* shape
introduced in the same chapter (lines 283-321).

#### Lines 51-60 — the `impl` block

> To define the function within the context of `Rectangle`, we start
> an `impl` (implementation) block for `Rectangle`. Everything within
> this `impl` block will be associated with the `Rectangle` type.
> Then, we move the `area` function within the `impl` curly brackets
> and change the first (and in this case, only) parameter to be
> `self` in the signature and everywhere within the body.

Corpus warrant for piece (1) of *The Move* — the `impl Type { ... }`
block syntax and the framing "everything within this `impl` block
will be associated with the `Rectangle` type." Today's
`impl Counter { ... }` is the same shape with `Counter` as the
implementing type.

#### Lines 61-66 — `&self` is shorthand for `self: &Self`

> In the signature for `area`, we use `&self` instead of `rectangle:
> &Rectangle`. The `&self` is actually short for `self: &Self`.
> Within an `impl` block, the type `Self` is an alias for the type
> that the `impl` block is for.

Corpus warrant for piece (3) and piece (4) of *The Move* — `&self` is
the shorthand for `self: &Self`, and `Self` inside an impl block is
an alias for the implementing type. The lesson's *The Move* and
*What Changed* both quote / paraphrase this passage.

#### Lines 283-298 — associated functions

> All functions defined within an `impl` block are called *associated
> functions* because they're associated with the type named after the
> `impl`. We can define associated functions that don't have `self`
> as their first parameter (and thus are not methods) because they
> don't need an instance of the type to work with. We've already used
> one function like this: the `String::from` function that's defined
> on the `String` type.
>
> Associated functions that aren't methods are often used for
> constructors that will return a new instance of the struct. These
> are often called `new`, but `new` isn't a special name and isn't
> built into the language.

Corpus warrant for piece (2) of *The Move* — the *associated function*
terminology, the rule "no `self` first parameter ⇒ not a method", and
the canonical `new` constructor shape. Today's `Counter::new()` is
the Book's exact pattern with a different type name.

#### Lines 322-331 — `Self` in body and return; calling via `Type::name()`

> The `Self` keywords in the return type and in the body of the
> function are aliases for the type that appears after the `impl`
> keyword, which in this case is `Rectangle`.
>
> To call this associated function, we use the `::` syntax with the
> struct name; `let sq = Rectangle::square(3);` is an example. This
> function is namespaced by the struct: The `::` syntax is used for
> both associated functions and namespaces created by modules.

Corpus warrant for the call-form `Type::name()` and for the
*Prerequisites* claim that today's `Counter::new()` is the same
shape lesson 043 installed but with a *type* on the left of `::`
rather than a module ("the `::` syntax is used for both associated
functions and namespaces created by modules"). The Book's
`Rectangle::square(3)` is the canonical pattern; today's
`Counter::new()` is identical modulo names and arity.

### `output/docs/rust/reference/items/implementations.md`

The Reference's *Implementations* item. Two load-bearing passages:

#### Lines 12-16 — the `InherentImpl` grammar

> InherentImpl →
>     impl GenericParams? Type WhereClause? {
>         InnerAttribute*
>         AssociatedItem*
>     }

Corpus warrant for the formal shape of an inherent impl block —
`impl` keyword, type, brace-enclosed body. Today's lesson uses the
ungeneric, where-clause-free, attribute-free form.

#### Lines 41-67 — definition and what may live inside

> An inherent implementation is defined as the sequence of the
> `impl` keyword, generic type declarations, a path to a nominal
> type, a where clause, and a bracketed set of associable items.
>
> The nominal type is called the *implementing type* and the
> associable items are the *associated items* to the implementing
> type.
>
> Inherent implementations associate the contained items to the
> implementing type.
>
> Inherent implementations can contain associated functions
> (including methods) and associated constants.
>
> A type can also have multiple inherent implementations.

Corpus warrant for the lesson's *What Changed* terminology
("inherent implementation", "associated items"), and for the
deferred items in *What To Ignore For Now* (associated constants;
multiple impl blocks). The Reference also confirms the lesson's
narrowing: today exercises only the `fn`-item case.

### `output/docs/rust/reference/items/associated-items.md`

The Reference's *Associated items* item. Three load-bearing passages:

#### Lines 105-112 — methods are associated functions whose first parameter is named `self`

> Associated functions whose first parameter is named `self` are
> called *methods* and may be invoked using the method call operator,
> for example, `x.foo()`, as well as the usual function call notation.

Corpus warrant for the lesson's *What Changed* claim that an
associated function with `&self` first is a method, and for the
deferred *What To Ignore For Now* claim that "`Counter::current(&c)`
works" — the Reference licenses both call shapes for methods.

#### Lines 153-159 — the `&self` shorthand table

> Shorthand syntax can be used without specifying a type, which have
> the following equivalents:
>
> | Shorthand | Equivalent |
> | --- | --- |
> | `self` | `self: Self` |
> | `&'lifetime self` | `self: &'lifetime Self` |
> | `&'lifetime mut self` | `self: &'lifetime mut Self` |

Corpus warrant for the lesson's *The Move* claim "`&self` is
shorthand for `self: &Self`." Today's lesson uses the shorthand
spelling exclusively.

#### Lines 64-79 — the `Struct::new() -> Struct` example

> ```rust
> struct Struct {
>     field: i32
> }
>
> impl Struct {
>     fn new() -> Struct {
>         Struct {
>             field: 0i32
>         }
>     }
> }
>
> fn main () {
>     let _struct = Struct::new();
> }
> ```

Corpus warrant for the canonical associated-function-as-constructor
shape today's probe instantiates. The Reference's `new()` returns
`Struct` rather than `Self`; the Book chapter ch05-03 lines 309-315
uses `-> Self` in the `square` example. Today's probe uses `-> Self`
to install both pieces (associated function + `Self` alias) at once.

### `output/docs/rust/error_codes/E0599.md`

Corpus warrant for the centered contrast probe's diagnostic. The
error code documentation is brief — its erroneous example is "no
method named ... found for type ..." and the documented fix is "you
need to implement the ... method to fix the error." Today's contrast
runs the inverse direction: a method authored with `&self` works
under the dot, removing `&self` makes it an associated function and
breaks the dot call with E0599.

## Probes

### Probe 1 — Working program

The committed observation file. Run in `/tmp/lesson100-probes/`:

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
}

fn main() {
    let c = Counter::new();
    println!("count = {}", c.current());
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
count = 0
exit=0
```

Witness for: `impl Type { ... }` block compiles silently; an
associated function `new() -> Self` returning a struct expression
works; a method `current(&self) -> u32` returning `self.count` works;
both are reachable via the path call `Counter::new()` and the dot
call `c.current()` respectively.

### Probe 2 — Centered E0599 contrast (no `&self`, dot call)

Source `no_self.rs`, identical to the working probe modulo
`fn current(&self) -> u32 { self.count }` replaced with
`fn current() -> u32 { 0 }` (the body trivialized to keep `self`
out of scope for the contrast):

```text
--- cat no_self.rs ---
struct Counter { count: u32 }

impl Counter {
    fn new() -> Self { Counter { count: 0 } }
    fn current() -> u32 { 0 }
}

fn main() {
    let c = Counter::new();
    println!("count = {}", c.current());
}
--- rustc no_self.rs ---
error[E0599]: no method named `current` found for struct `Counter` in the current scope
  --> no_self.rs:10:30
   |
 1 | struct Counter { count: u32 }
   | -------------- method `current` not found for this struct
...
10 |     println!("count = {}", c.current());
   |                              ^^^^^^^ this is an associated function, not a method
   |
   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: the candidate is defined in an impl for the type `Counter`
  --> no_self.rs:5:5
   |
 5 |     fn current() -> u32 { 0 }
   |     ^^^^^^^^^^^^^^^^^^^
help: use associated function syntax instead
   |
10 -     println!("count = {}", c.current());
10 +     println!("count = {}", Counter::current());
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
exit=1
```

Witness for: removing `&self` from a function inside `impl Counter`
turns it into an associated function. Calling it via the dot form
fires E0599. The diagnostic literally states the rule today installs:
the inline label says `this is an associated function, not a method`,
and the `= note:` line says `to be used as methods, functions must
have a \`self\` parameter`. The `help:` block proposes the path form
`Counter::current()` as the fix. The transcript reproduced verbatim
in the lesson body matches this exactly modulo the truncated initial
`note:` ellipsis (the lesson body is the first 17 lines of this
output; the `error: aborting...` and `--explain` trailers are read
with lesson 069's category map).

### Probe 3 — Auxiliary E0061 contrast (method-via-path, no receiver)

Source `method_via_path.rs`, the working probe modulo the call site
in `main` switched from `c.current()` to `Counter::current()` (no
argument):

```text
--- cat method_via_path.rs ---
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
}

fn main() {
    let c = Counter::new();
    println!("count = {}", Counter::current());
    let _ = c;
}
--- rustc method_via_path.rs ---
error[E0061]: this function takes 1 argument but 0 arguments were supplied
  --> method_via_path.rs:16:28
   |
16 |     println!("count = {}", Counter::current());
   |                            ^^^^^^^^^^^^^^^^-- argument #1 of type `&Counter` is missing
   |
note: method defined here
  --> method_via_path.rs:9:8
   |
 9 |     fn current(&self) -> u32 {
   |        ^^^^^^^ -----
   |
help: provide the argument
   |
16 |     println!("count = {}", Counter::current(/* &Counter */));
   |                                             ++++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0061`.
exit=1
```

Witness for: a method authored with `&self` is *also* reachable via
the path form `Counter::current(&c)` — but in that form `&self`
becomes positional argument #1 of type `&Counter`, and omitting it
fires E0061 (the same E-code lesson 036 used for arity mismatch).
This corroborates the *What To Ignore For Now* deferral "calling
`&self` methods via the path form — `Counter::current(&c)` works".

### Probe 4 — Auxiliary verification: `Counter::current(&c)` works

Source `method_via_path_with_arg.rs`, identical to Probe 3 modulo
the call site `Counter::current(&c)` (passing the receiver as the
explicit positional argument):

```text
--- cat method_via_path_with_arg.rs ---
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
}

fn main() {
    let c = Counter::new();
    println!("count = {}", Counter::current(&c));
}
--- rustc method_via_path_with_arg.rs ---
exit=0
--- ./method_via_path_with_arg ---
count = 0
exit=0
```

Witness for the deferred *What To Ignore For Now* claim that
`Counter::current(&c)` is a valid call form — empirical evidence
matching the Reference's "may be invoked using the method call
operator ... as well as the usual function call notation."

### Probe 5 — Auxiliary verification: `Self { ... }` construction

Source `self_construction.rs`, identical to Probe 1 modulo the body
of `new` rewritten as `Self { count: 0 }` (using `Self` instead of
`Counter` in the struct expression):

```text
--- cat self_construction.rs ---
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
    fn current(&self) -> u32 {
        self.count
    }
}

fn main() {
    let c = Counter::new();
    println!("count = {}", c.current());
}
--- rustc self_construction.rs ---
exit=0
--- ./self_construction ---
count = 0
exit=0
```

Witness for the lesson's *The Move* claim "`Self` inside the impl
body and signature is an alias for the type the impl block is
attached to." Today's committed working probe uses the explicit
`Counter` spelling in the body of `new`; this auxiliary probe shows
that `Self { count: 0 }` is also accepted (Book ch05-03 lines
311-313's `Self { width: size, height: size }` form). The lesson's
*What To Ignore For Now* notes the Book mixes both spellings
stylistically; today's probe uses `Counter { count: 0 }` for
clarity.

### Probe 6 — Auxiliary verification: associated-item order is free

Source `reordered.rs`, identical to Probe 1 modulo `current` and
`new` swapped in the impl block (so the method appears textually
before the associated function):

```text
--- cat reordered.rs ---
struct Counter {
    count: u32,
}

impl Counter {
    fn current(&self) -> u32 {
        self.count
    }
    fn new() -> Self {
        Counter { count: 0 }
    }
}

fn main() {
    let c = Counter::new();
    println!("count = {}", c.current());
}
--- rustc reordered.rs ---
exit=0
--- ./reordered ---
count = 0
exit=0
```

Witness that the order of associated items inside an impl block is
free, matching the Reference's grammar `AssociatedItem*` (zero or
more, no ordering rule). The lesson body does not center this
auxiliary fact; it is captured here for completeness in case the
red-team asks whether item order is load-bearing.

## Prior lessons

### Direct prerequisites

- **095-struct-with-named-fields** (accepted, *load-bearing*) —
  installs `struct Name { field: Type }` declaration,
  `Name { field: value }` construction, and `instance.field` access.
  Today's three claims that reuse 095:
  - The struct declaration `struct Counter { count: u32 }` is the
    impl target, unchanged from 095.
  - The struct expression `Counter { count: 0 }` inside `new`'s body
    is the same expression form 095 installed.
  - `self.count` is a field-access expression on the receiver, the
    same shape 095's `p.x` installed; the only new piece is that the
    receiver name is `self` (the implicit shorthand parameter)
    rather than a name bound by `let`.

- **040-method-call-syntax** (accepted, *load-bearing*) — installs
  the call-site form `value.method(args)`. Today's claim that reuses
  040: at the call site `c.current()`, the dot, the method name, and
  the parenthesized argument list are exactly 040's shape; today's
  contribution is the *authoring* side — `&self` in the signature is
  what makes the dot call resolve.

- **020-function-with-parameter** and **021-function-return-value**
  (accepted, *load-bearing*) — install `fn name(p: T) -> R { body }`.
  Today's claim that reuses 020/021: every `fn` inside an impl is
  exactly the same `fn` shape (parameter list, return type, brace
  body). The one new rule is that the first parameter slot may be
  `&self`, which the Reference's "Self pattern shorthands" table
  expands to `self: &Self`.

- **043-nested-module-paths** (accepted, *load-bearing*) — installs
  the path call `path::name(args)`. Today's claim that reuses 043:
  `Counter::new()` is the same `::`-separated path call form, with a
  *type* segment on the left rather than a module segment. The Book
  ch05-03 lines 327-330 names this directly: "the `::` syntax is
  used for both associated functions and namespaces created by
  modules."

### Supporting prior lessons (cited only)

- **062-u32-unsigned-integer** — `u32`, the field type and method
  return type. No new claim about `u32` is introduced today; the
  type is reused unchanged from 062.
- **002-fn-main-entry-point** — `fn main` is the entry point; the
  probe uses one `fn main` block, unchanged.
- **005-let-binding** — `let p = expression;` reused once in `main`
  to bind the constructed `Counter` to `c`.
- **011-println-positional-args** — `println!` with one `{}` slot
  consumed by `c.current()`; lesson 011 explicitly licensed any
  expression in the argument slot.
- **003-read-rustc-diagnostic** — the four-part diagnostic map
  applied to Probe 2's E0599 transcript and Probe 3's E0061
  transcript. Probe 2 carries an additional `note:` block with a
  second `-->` line at the *definition site*, the same shape lesson
  096 installed for E0603.
- **001-rustc-compile-and-run** — `rustc demo.rs` then `./demo`,
  silent on success.
- **069-rustc-warnings** — Probes 2 and 3's `error: aborting due to 1
  previous error` trailer is read with lesson 069's category map,
  distinguishing from the `warning:` category. The working probe
  produces no warnings.
- **098-enum-with-unit-variants** — the cross-reference for E0599's
  E-code; lesson 098 installed E0599 in the "no variant ... found"
  shape, and today extends it to the "no method ... found" shape on
  the same struct. The Reference catalogs E0599 broadly as "method,
  associated function, or item not found."
- **036-multiple-parameters** — cited only in the auxiliary E0061
  probe and the *What To Ignore For Now* deferral; lesson 036
  installed E0061 for arity mismatch on free functions, and Probe 3
  shows the same E-code on a path-form method call where `&self`
  becomes positional argument #1.
- **096-inline-module-and-pub** — cited only in the *What To Ignore
  For Now* deferral noting that the rmp target's
  `pub fn zero() -> Self` composes lesson 096's `pub` with today's
  associated-function shape.

## Probe-shape note

The working probe (Probe 1) constructs the `Counter` instance and
calls `.current()` on it, so neither `dead_code` nor the
construction-without-use lints fire. The contrast probe (Probe 2)
produces a hard error and never reaches lint-checking. The
auxiliary path-form probe (Probe 3) deliberately writes `let _ = c;`
after the broken call so that `c` is "used" enough not to trip
`unused_variables` if the compiler were to keep going past E0061
(it doesn't, but the precaution keeps the source minimal). Probe 4
constructs and consumes `c` in the call site; Probes 5 and 6 are
identical to Probe 1 modulo the targeted change.

## Mapping summary

| Lesson claim | Source / probe |
|---|---|
| `impl Type { ... }` is an *inherent implementation* | Reference items/implementations.md lines 12-16 (grammar) and 41-67 (semantics); Probe 1 |
| Functions inside the impl are *associated items*; with `&self` first they are *methods*, otherwise *associated functions* | Reference items/associated-items.md lines 36-79 and 105-112; Book ch05-03 lines 283-298 |
| `&self` is shorthand for `self: &Self` | Reference items/associated-items.md lines 153-159; Book ch05-03 lines 61-66 |
| `Self` inside the impl is an alias for the implementing type | Book ch05-03 lines 61-66 and 322-325 |
| Method called via `value.method()`; associated function called via `Type::name()` | Book ch05-03 lines 51-60 (method) and 326-331 (associated function); Probe 1 |
| Without `&self`, the dot form fires E0599 with the inline label `this is an associated function, not a method` | Probe 2 transcript verbatim |
| The `help:` block proposes the path form `Counter::current()` | Probe 2 transcript verbatim |
| Calling a `&self` method via the path form requires the receiver as positional argument #1 | Probe 3 (E0061); Probe 4 (working `Counter::current(&c)`) |
| `Self { count: 0 }` works as the construction shape inside the impl body | Probe 5; Book ch05-03 lines 311-313 |
| Order of items inside the impl block is free | Probe 6; Reference items/implementations.md grammar `AssociatedItem*` |

No lesson claim relies on a fact that does not appear in either a
listed corpus passage or a captured probe.
