# Evidence — 104-super-and-crate-paths

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in `/tmp/lesson104-probes/` on this host. Same toolchain
  family as recent accepted lessons (082-103).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/104-super-and-crate-paths.rs`
is the working probe verbatim, with header comments naming the
centered E0433 contrast and the auxiliary witnesses captured below.

## Sources

### `output/docs/rust/reference/paths.md`

The primary corpus for today. Contains the path grammar and the formal
descriptions of all four path qualifiers (`::`, `self`, `Self`,
`super`, `crate`, `$crate`).

#### Lines 32-33 — `SimplePathSegment` grammar

> SimplePathSegment →
>     IDENTIFIER | super | self | crate | $crate

Corpus warrant for *The Move*'s "the Reference's path grammar lists
both as path qualifiers." Lesson 043 already cited paths.md as the
formal grammar; today centers two specific qualifier keywords from the
same production.

#### Lines 64-65 — `PathIdentSegment` grammar

> PathIdentSegment →
>     IDENTIFIER | super | self | Self | crate | $crate

Same listing, used in `PathInExpression` and `TypePath`. Cited because
today's call sites are expression paths (`super::at_outer()`,
`crate::at_root()`).

#### Lines 381-403 — the formal `super` qualifier

> #### `super`
>
> `super` in a path resolves to the parent module.
>
> It may only be used in leading segments of the path, possibly after
> an initial `self` segment.
>
> ```rust
> mod a {
>     pub fn foo() {}
> }
> mod b {
>     pub fn foo() {
>         super::a::foo(); // call a's foo function
>     }
> }
> fn main() {}
> ```

Corpus warrant for *The Move*'s "`super::name` walks up one level, to
the *parent* module, and resolves `name` from there." The Reference's
example at lines 393-403 is structurally the same shape today's working
probe uses (a function defined at one module level, called from a
sibling/child module via `super::`). Cited as load-bearing.

#### Lines 405-423 — `super` repetition

> `super` may be repeated several times after the first `super` or
> `self` to refer to ancestor modules.
>
> ```rust
> mod a {
>     fn foo() {}
>
>     mod b {
>         mod c {
>             fn foo() {
>                 super::super::foo(); // call a's foo function
>                 self::super::super::foo(); // call a's foo function
>             }
>         }
>     }
> }
> fn main() {}
> ```

Corpus warrant for *What To Ignore For Now*'s "Multi-level
`super::super::...` — chains walk up multiple levels." Today exercises
exactly one level of `super::`; the multi-level form is named but
deferred. The Reference's "may be repeated" sentence is the rule
behind why Probe 4 fires E0433 instead of resolving (the chain
exhausted the available ancestors).

#### Lines 425-445 — the formal `crate` qualifier

> #### `crate`
>
> `crate` resolves the path relative to the current crate.
>
> `crate` can only be used as the first segment, without a preceding
> `::`.
>
> ```rust
> fn foo() {}
> mod a {
>     fn bar() {
>         crate::foo();
>     }
> }
> fn main() {}
> ```

Corpus warrant for *The Move*'s "`crate::name` jumps to the *crate
root* (the file `rustc` was given), regardless of how deeply nested the
call site is." The Reference's example at lines 437-445 is
structurally the same shape today's working probe uses for `crate::`.
Cited as load-bearing.

The Reference says "first segment, without a preceding `::`." The
position table in *What Changed* names "leftmost prefix" — the same
rule, learner-facing wording.

#### Lines 278-306 — `self` qualifier (named only)

> ### `self`
>
> `self` resolves the path relative to the current module.
>
> `self` can only be used as the first segment, without a preceding
> `::`.

Corpus warrant for *Mental Model Delta*'s "siblings of `self::`" claim
and *What To Ignore For Now*'s "*`self::name`* — the third relative
qualifier; means 'the current module,' equivalent to bare `name` in
most positions." Named, not centered.

#### Lines 226-248 — overview of path qualifiers and edition note

> ## Path qualifiers
>
> Paths can be denoted with various leading qualifiers to change the
> meaning of how it is resolved.
>
> [...]
>
> > 2018 Edition differences
> >
> > In the 2015 Edition, identifiers resolve from the "crate root"
> > (`crate::` in the 2018 edition), which contains a variety of
> > different items, including external crates, default crates such as
> > `std` or `core`, and items in the top level of the crate
> > (including `use` imports).
> >
> > Beginning with the 2018 Edition, paths starting with `::` resolve
> > from crates in the [extern prelude]. That is, they must be
> > followed by the name of a crate.

Corpus warrant for *What To Ignore For Now*'s "*Edition differences* —
Rust 2015 used a leading `::` for the crate root; Rust 2018 introduced
`crate::`. This run uses edition 2024." (Edition 2024 inherits the
2018 behaviour for these qualifiers; no breaking change in the path
grammar itself.)

### `output/docs/rust/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md`

Background corpus for absolute-vs-relative path framing.

#### Lines 8-14 — absolute vs relative

> A path can take two forms:
>
> - An *absolute path* is the full path starting from a crate root;
>   for code from an external crate, the absolute path begins with the
>   crate name, and for code from the current crate, it starts with
>   the literal `crate`.
> - A *relative path* starts from the current module and uses `self`,
>   `super`, or an identifier in the current module.

Corpus warrant for *The Move*'s "Those paths are *absolute* — they
start with a crate name and walk down. Today installs *relative* paths
that start where the calling code lives." Also corpus warrant for the
*Mental Model Delta* "second style: *relative* paths anchored by a
keyword."

#### Lines 56-59 — filesystem analogy for `crate`

> You can imagine a filesystem with the same structure: We'd specify
> the path `/front_of_house/hosting/add_to_waitlist` to run the
> `add_to_waitlist` program; using the `crate` name to start from the
> crate root is like using `/` to start from the filesystem root in
> your shell.

Background framing. The lesson body uses the parallel filesystem
analogy for `super::` (line below); the Book provides the analogy for
`crate::`. Not directly quoted in the lesson, but the framing is the
Book's.

#### Lines 293-301 — the `super` introduction

> ### Starting Relative Paths with `super`
>
> We can construct relative paths that begin in the parent module,
> rather than the current module or the crate root, by using `super`
> at the start of the path. This is like starting a filesystem path
> with the `..` syntax that means to go to the parent directory.

Corpus warrant for *Mental Model Delta*'s "(one level up, like `..` in
a filesystem path)." The Book explicitly introduces the `..` analogy
for `super::`.

### `output/docs/rust/error_codes/E0433.md`

> An undeclared crate, module, or type was used.

E0433 is the broader "failed to resolve" family; rustc reuses the same
E-code for the specific *too many leading `super` keywords* phrasing
captured in Probe 2 and Probe 4 below. The corpus example of E0433 is
about an unimported `HashMap`, but the E-code is the same. Cited as
the "same E-code family as lesson 043's unresolved-path diagnostics"
in *What Changed*.

The corpus page also reads:

> To use a module from your current crate, add the `crate::` prefix
> to the path.

Corpus warrant — the official error-code page itself instructs the
reader to add `crate::` as the fix; today's lesson installs that
prefix.

## Probes

All probes run from `/tmp/lesson104-probes/` on the host described in
*Toolchain*.

### Probe 1 — working `super::` and `crate::` together

The committed observation file. Reproduced for grounding:

```rust
fn at_root() -> u32 { 1 }

mod outer {
    pub fn at_outer() -> u32 { 2 }
    pub mod inner {
        pub fn use_super() -> u32 {
            super::at_outer()
        }
        pub fn use_crate() -> u32 {
            crate::at_root()
        }
    }
}

fn main() {
    println!("super = {}, crate = {}",
        outer::inner::use_super(),
        outer::inner::use_crate());
}
```

```
$ rustc demo.rs
$ ./demo
super = 2, crate = 1
$ echo $?
0
```

Witnesses both prefixes resolving in one program: `super::` walks up
one level (from `inner` to `outer`, where `at_outer` is defined),
`crate::` jumps to the crate root (where `at_root` is defined). Both
calls happen from inside `inner`, the inner-most module.

`rustc demo.rs` emits no warnings: every item is `pub` and is reached
by a call from `fn main`, so `dead_code` (warn-by-default) does not
fire.

### Probe 2 — centered contrast: `super::` from the crate root

```rust
fn main() {
    super::missing();
}
```

```
$ rustc too_many_supers.rs
error[E0433]: too many leading `super` keywords
 --> too_many_supers.rs:2:5
  |
2 |     super::missing();
  |     ^^^^^ there are too many leading `super` keywords

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0433`.
$ echo $?
1
```

Witnesses the contrast claim: `super::` from a position that has no
parent module (the crate root) fires E0433 with the verbatim phrase
"there are too many leading `super` keywords." The diagnostic states
the rule today installs.

The diagnostic's four parts (lesson 003 map):

- Headline: `error[E0433]: too many leading \`super\` keywords`.
- Location: `--> too_many_supers.rs:2:5`.
- Source excerpt with caret: lines 2-3 underlining `super` (5 chars).
- Help/note: the trailer `error: aborting due to 1 previous error` and
  `For more information about this error, try \`rustc --explain
  E0433\`.`

The function name `missing` is irrelevant — `rustc` rejects the path
during resolution before checking whether the named function exists,
because there is no parent module to resolve `super` against.

### Probe 3 — operational distinguisher: swap `super::` for `crate::`

```rust
fn at_root() -> u32 { 1 }

mod outer {
    pub fn at_outer() -> u32 { 2 }
    pub mod inner {
        pub fn use_super_target() -> u32 {
            // `super::` walks up to `outer`, where `at_outer` is defined.
            // `crate::` would walk up to the crate root, where there is
            // no `at_outer` — only `at_root`.
            crate::at_outer()
        }
    }
}

fn main() {
    println!("{}", outer::inner::use_super_target());
}
```

```
$ rustc swap_super_for_crate.rs
error[E0425]: cannot find function `at_outer` in the crate root
  --> swap_super_for_crate.rs:10:20
   |
10 |             crate::at_outer()
   |                    ^^^^^^^^ not found in the crate root
   |
help: consider importing this function
   |
 6 +         use outer::at_outer;
   |
help: if you import `at_outer`, refer to it directly
   |
10 -             crate::at_outer()
10 +             at_outer()
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
$ echo $?
1
```

Witnesses the *operational difference* between `super::` and
`crate::`. From inside `inner`, `super::at_outer` resolves (Probe 1)
because `super` from `inner` is `outer`, which contains `at_outer`.
Swapping the prefix to `crate::at_outer` instead jumps to the crate
root, where `at_outer` is *not* — the diagnostic literally says "not
found in the crate root." E0425 is the same E-code lessons 005, 008,
040, 042, 043, 044 installed for the broader "cannot find" family;
today's diagnostic message names the search location ("in the crate
root") because the path explicitly anchored there.

This is the negative/contrast probe required by the README for
"with X works, without X differs" claims: it proves the two prefixes
genuinely route to *different* destinations.

### Probe 4 — multi-level `super::super::` exhausts ancestors

```rust
fn at_root() -> u32 { 1 }

mod outer {
    pub fn try_super_super() -> u32 {
        super::super::at_root()
    }
}

fn main() {
    println!("{}", outer::try_super_super());
}
```

```
$ rustc super_super_too_far.rs
error[E0433]: too many leading `super` keywords
 --> super_super_too_far.rs:5:16
  |
5 |         super::super::at_root()
  |                ^^^^^ there are too many leading `super` keywords

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0433`.
$ echo $?
1
```

Witnesses the rule by exhaustion: from inside `outer` (depth 1), the
first `super::` walks up one level to the crate root; the second
`super::` has nowhere to go. Same E0433, caret on the second `super`
(column 16), same diagnostic phrase. Each `super::` walks up exactly
one level, and the chain bottoms out at the crate root.

The general rule (verified by an extra probe at
`/tmp/lesson104-probes/quiz_b3.rs` not preserved in observations): from
depth N (counted from the crate root), you can stack up to N `super::`
keywords legally — the N-th one lands at the crate root. Stacking N+1
fires E0433 with the caret on the (N+1)-th `super`. The *Check
Yourself* answer (b) uses depth 2 (`mod a { mod b { ... } }`), so
`super::super::` is legal (reaches the crate root, then E0425 if the
named item is not at the root) and `super::super::super::` fires
E0433. The lesson's worked answer states this correctly: "From `b` you
can walk up at most *two* levels."

## Prerequisite-claim summary

Direct prerequisites (load-bearing claims today depends on):

- **Lesson 096 (load-bearing)** — installs `mod foo { ... }` and the
  rule that items inside the braces live in a new namespace called by
  the module name. Today's probe nests `mod inner` inside `mod outer`
  to make `super::` observable; without lesson 096, the audience would
  not know how to declare a module at all.
- **Lesson 043 (load-bearing)** — installs the call form
  `module::name(args)`. Today's `super::at_outer()`,
  `crate::at_root()`, and `outer::inner::use_super()` are all this
  shape; the prefixes go in the *leftmost* segment, the rest is
  unchanged. Lesson 043's *unlocks* explicitly named "the `crate::` /
  `self::` / `super::` path roots" as a future move — today is exactly
  the first two.
- **Lesson 003 (load-bearing)** — the four-part diagnostic map applied
  to Probe 2's E0433 transcript. The headline / location / source-with-
  caret / help-note structure is unchanged from lesson 003.

Older supporting lessons (named only, not load-bearing):

- Lesson 097 (file-based modules) — `super::` and `crate::` work
  unchanged across file boundaries. Today's working probe is
  single-file for compactness; the rmp target uses both prefixes
  across many files.
- Lesson 044 (`use` declaration) — the natural follow-on composition
  is `use super::Item;` and `use crate::module::Item;`; deferred.
- Lesson 100 (`Self` in `impl` bodies) — capital-S `Self` is named in
  *What To Ignore For Now* as a different keyword in a different
  namespace.
- Lesson 002 (`fn main`), lesson 011 (`println!` positional args),
  lesson 008 (define and call function), lesson 062 (`u32` integer
  type), lesson 069 (rustc warning category map), lesson 001
  (`rustc` and `./executable`) — unchanged supporting machinery.

## Probe-shape notes

- The working probe nests `mod inner` inside `mod outer` so that
  `super::` from `inner` resolves to `outer` (a non-trivial parent),
  not to the crate root. This makes `super::at_outer` and
  `crate::at_root` reach *different* items, which is the point.
- `at_root` is intentionally placed at the crate root (outside any
  `mod`) so `crate::at_root` resolves there.
- All four probes use only `u32` (lesson 062), `pub` and `pub mod`
  (lesson 096), file-based-module knowledge optional (lesson 097
  named only), call expressions (lesson 008/043), `let`-free,
  `println!` with two `{}` slots in Probe 1.
- The contrast Probe 2 puts `super::` directly inside `fn main` at the
  crate root rather than in any submodule, so the failure mode is the
  most direct possible witness of the rule.
