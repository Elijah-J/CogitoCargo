# Evidence — 109-module-scope-const

This appendix grounds lesson 109's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` -> `Darwin x86_64`
- Probes run from `mktemp -d` directories on this host. Same
  toolchain as recent accepted lessons (107, 108).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/109-module-scope-const.rs`
is the working probe verbatim.

## Sources

### `output/docs/rust/reference/items.md`

The Reference's top-level `Item` grammar (lines 10-29) gives the
formal warrant for today's centered move. A reproduction:

> [Item] →
>     [OuterAttribute]\* ( [VisItem] | [MacroItem] )
>
> [VisItem] →
>     [Visibility]?
>     (
>         [Module]
>       | [ExternCrate]
>       | [UseDeclaration]
>       | [Function]
>       | [TypeAlias]
>       | [Struct]
>       | [Enumeration]
>       | [Union]
>       | [ConstantItem]
>       | [StaticItem]
>       | [Trait]
>       | [Implementation]
>       | [ExternBlock]
>     )

Three load-bearing facts:

1. `ConstantItem` is one of the alternatives inside `VisItem` —
   meaning a `const` declaration is one kind of *visible item*.
2. `VisItem` carries an optional `Visibility?` prefix. Any of the
   five `Visibility` shapes from `visibility-and-privacy.md` may
   sit in front of any `VisItem`, including `ConstantItem`.
3. The wrapper alternative names `Function` next to `ConstantItem`
   in the same production — load-bearing for the lesson's framing
   that "`pub`-on-`const` is the same rule as `pub`-on-`fn`".

### `output/docs/rust/reference/items/constant-items.md`

The dedicated reference page for `const` items. Two load-bearing
quotes:

#### Lines 9-11 — the `ConstantItem` grammar

> [ConstantItem] →
>     const ( [IDENTIFIER] | _ ) : [Type] ( = [Expression] )? ;

The `Visibility?` slot is *not* part of the `ConstantItem` rule
itself — it lives one level up in `VisItem`. This is the formal
reason a function-scope const cannot be `pub`-marked in any
interesting way: a function body is not a `VisItem`-bearing
position. (Auxiliary observation: `pub` on a function-scope const
compiles silently with no diagnostic — see Probe 5 below — but the
modifier has no effect.)

#### Line 23 — the namespace rule

> The constant declaration defines the constant value in the
> [value namespace] of the module or block where it is located.

Direct corpus warrant for the lesson's claim that const items live
in *either* a module *or* a block. Lesson 075 covered the block
case (function bodies are blocks); today covers the module case.

### `output/docs/rust/book/ch03-01-variables-and-mutability.md`

Lesson 075's primary corpus source.

#### Lines 122-123 — module / global scope claim

> Constants can be declared in any scope, including the global
> scope, which makes them useful for values that many parts of code
> need to know about.

Lesson 075 quoted this line and named it as the "fifth difference
from `let`": a `const` can sit at global scope where a `let`
cannot. Lesson 075's working probe placed
`THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;` at global scope to
exercise this — but lesson 075's centered teaching content was the
five-fact difference list against `let`, not visibility on the
result. Today centers visibility.

### `output/docs/rust/reference/visibility-and-privacy.md`

Lesson 103's primary corpus source. Three load-bearing claims
carry forward unchanged today:

#### Lines 8-15 — the `Visibility` grammar (already installed in 103)

> [Visibility] →
>       pub
>     | pub ( crate )
>     | pub ( self )
>     | pub ( super )
>     | pub ( in [SimplePath] )

Today reuses three of the five rows: `pub`, `pub(crate)`,
`pub(super)`. Lesson 103 already centered `pub(crate)` and
`pub(super)` on a function item; today applies the same rows in
the same syntactic position to a `ConstantItem`.

#### Line 31 — the privacy-by-default rule (already installed in 096/103)

> By default, everything is *private*, with two exceptions:
> Associated items in a `pub` Trait are public by default; Enum
> variants in a `pub` enum are also public by default.

Today's probe witnesses this on a const item: `LOCAL` (no marker)
is private to `inner`; `inner::LOCAL` from `main` would fire
E0603. Probe 2 below witnesses the same outcome on `SHARED` after
the `pub(crate)` is dropped.

### `output/docs/rust/error_codes/E0603.md`

The error-code reference page already installed by lesson 096 and
reused by 103 and 105. Today's contrast (Probe 2) reports E0603
with the inline label `private constant` instead of `private
function` — same E-code, item-kind-specialized phrasing.

## rmp target line

The lesson's parenthetical in *The Move* names rmp's
`pub(crate) const LIMB_SIZE_BITS: u64 = 8 * (std::mem::size_of::<u64>() as u64);`
in `biguint/basic.rs`. The `pub(crate) const NAME: TYPE = value;`
left-hand side is exactly today's shape; the right-hand side
expression `8 * (std::mem::size_of::<u64>() as u64)` involves
`std::mem::size_of::<u64>()` (a generic associated function) which
is named-but-deferred today and listed in *What To Ignore For Now*.
Source for the line: `experimental/eduratchet2/runs/rust-moves/rmp-target-audit.md`
lines 39 and 394-396 (the audit explicitly names "const items at
module scope with pub(crate) visibility — lesson 075 covered const
at function scope only" as a direct unblock for today).

## Direct prerequisite summaries

### Lesson 075 (load-bearing — `const NAME: TYPE = value;` form)

- Lesson 075 installed the syntax `const NAME: TYPE = value;`,
  the SCREAMING_SNAKE_CASE convention, the required `: TYPE`
  annotation, and the constant-expression rule for the right-hand
  side.
- Lesson 075's working probe placed `THREE_HOURS_IN_SECONDS: u32 =
  60 * 60 * 3;` at global scope and `MAX_POINTS: u32 = 100;`
  inside `fn main`, but the *centered teaching content* was the
  five-fact difference list against `let` — not the rule "the
  global-scope position turns the const into a visible item with
  visibility semantics." Today centers exactly that follow-on.

### Lesson 096 (load-bearing — `pub` on a function item, E0603)

- Lesson 096 installed `mod foo { ... }`, items inside a module
  are private by default, `pub` opens an item, and the E0603
  diagnostic with `--> use_site` and a `note:` block at the
  definition.
- Today applies the *same* rule to a different item kind: a
  `ConstantItem` instead of a `Function`. The `Visibility?` slot
  in the Reference's `VisItem` grammar applies to both. Probe 2's
  E0603 transcript matches lesson 096's E0603 transcript shape
  exactly — same caret-at-use-site, same `note:` block at the
  definition, same `For more information...` trailer — only the
  inline label changes from `private function` to `private
  constant`.

### Lesson 103 (load-bearing — `pub(crate)` and `pub(super)`)

- Lesson 103 installed `pub(crate)` and `pub(super)` as two more
  shapes from the `Visibility` grammar, both in the same
  syntactic position as plain `pub`, both diagnosed by the same
  E0603 when access fails.
- Today reuses both shapes verbatim, on a `const` item. The
  working probe uses `pub(crate)` on `SHARED`; auxiliary Probe 4
  exercises `pub(super)` on a const inside a deeper-nested
  module. Both are admitted by the formal `VisItem → Visibility?
  ... ConstantItem ...` grammar.

### Lesson 104 (cited — `super::` / `crate::` path tokens)

- Lesson 104 installed the relative path prefixes `super::` and
  `crate::` for navigating the module tree. Cited here for
  completeness as the lesson naming `super::name` token syntax.
- Today's probes do *not* exercise `super::name` token syntax.
  Module-scope consts in the probes are reached through bare
  names (when same-module), the `module::ITEM` path form from
  lesson 096, or absolute paths like `outer::inner::PARENT_ONLY`.
- The `pub(super)` *modifier* used inside Probe 4 is from lesson
  103's `Visibility` grammar, not from lesson 104's path-token
  grammar; the two grammars share the `super` keyword but appear
  in different syntactic positions (visibility marker vs path
  prefix).

### Lessons 002, 005, 008, 011, 040, 043, 062 (cited)

- Lesson 002: `fn main` body runs when the executable launches;
  the host of all probes today.
- Lesson 005: `let` is not used directly in today's probes
  (constants are read directly into `println!` placeholders); the
  cite is for the contrast with `const` already drawn in 075.
- Lesson 008: `fn name() -> T { ... }` definition shape; the
  helper `fn read_local` in the working probe is unchanged from
  008's shape.
- Lesson 011: `println!` with `{}` placeholders, three calls in
  the working probe.
- Lesson 040: dot-call shape on values; not used today, cited
  only because the constant is read by direct path access
  `inner::SHARED`, not by method call.
- Lesson 043: `module::name(args)` path call form; the working
  probe's `inner::SHARED`, `inner::read_local()` are this shape
  with `inner` as a user-declared module.
- Lesson 062: `u32` and `u64` in the `: TYPE` slot of the const
  declarations.

## Probes

### Probe 1 — working probe

Source (committed at
`observations/109-module-scope-const.rs`, header comments elided):

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

Transcript:

```text
$ rustc demo.rs
$ ./demo
MAX_BYTES = 1024
inner::SHARED = 64
inner::LOCAL via fn = 100
$ echo $?
0
```

`rustc demo.rs` exits `0` and is silent. `./demo` prints three
lines and exits `0`. Three positions and two visibilities are
witnessed in one program:

- `MAX_BYTES` at the crate root, accessed by bare name from `main`
  (same module — privacy passes by being inside the declaring
  module).
- `SHARED` at module scope inside `inner`, with `pub(crate)`,
  accessed from `main` via the path `inner::SHARED` (different
  module — privacy passes through the `pub(crate)` opener).
- `LOCAL` at module scope inside `inner`, no visibility marker,
  *not* accessed from `main` directly. The helper `pub fn
  read_local` is in the same module as `LOCAL` (privacy passes
  inside `inner`) and is itself `pub` (so `main` can call it).

### Probe 2 — centered contrast: drop `pub(crate)`, fire E0603

Source (`broken.rs`):

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

(The `MAX_BYTES` line is removed for compactness; the contrast
centers on `SHARED` losing its `pub(crate)`.)

Compile transcript:

```text
$ rustc broken.rs
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

For more information about this error, try `rustc --explain E0603`.
$ echo $?
1
```

Same E0603 lesson 096 first installed. Caret at the *use site*
under `SHARED` (column 43, 6 characters); `note:` block points at
the definition with a second `-->` and underlines the entire
`const SHARED: u64 = 64;` line. Inline label `private constant` —
the new piece today, replacing the `private function` lesson 096
saw on `fn hi`. Same E-code, item-kind-specialized phrasing.
Reproduced in the lesson body verbatim.

### Probe 3 — auxiliary: same E0603 with the dropped marker on a small witness

Smaller witness using the lesson's own auxiliary form (the
appendix's earlier validation step before the lesson body's
contrast was finalized):

```rust
mod inner {
    const SHARED: u64 = 64;
    pub fn read() -> u64 { SHARED }
}

fn main() {
    println!("from outside: SHARED = {}", inner::SHARED);
}
```

Compile transcript:

```text
$ rustc tiny.rs
error[E0603]: constant `SHARED` is private
 --> tiny.rs:7:50
  |
7 |     println!("from outside: SHARED = {}", inner::SHARED);
  |                                                  ^^^^^^ private constant
  |
note: the constant `SHARED` is defined here
 --> tiny.rs:2:5
  |
2 |     const SHARED: u64 = 64;
  |     ^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
$ echo $?
1
```

Same E0603 shape on a smaller program — confirms Probe 2's
contrast is not an artifact of the surrounding `LOCAL`+`SHARED`
two-const layout. Inline label `private constant` carries through.

### Probe 4 — auxiliary: `pub(super)` on a const, reach exhaustion

Source (`aux_super.rs`):

```rust
mod outer {
    pub mod inner {
        pub(super) const PARENT_ONLY: u32 = 42;
    }
    pub fn read() -> u32 { inner::PARENT_ONLY }
}

fn main() {
    println!("via outer::read = {}", outer::read());
    println!("direct = {}", outer::inner::PARENT_ONLY);
}
```

Compile transcript:

```text
$ rustc aux_super.rs
error[E0603]: constant `PARENT_ONLY` is private
  --> aux_super.rs:10:43
   |
10 |     println!("direct = {}", outer::inner::PARENT_ONLY);
   |                                           ^^^^^^^^^^^ private constant
   |
note: the constant `PARENT_ONLY` is defined here
  --> aux_super.rs:3:9
   |
 3 |         pub(super) const PARENT_ONLY: u32 = 42;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
$ echo $?
1
```

Operational distinguisher: `pub(super)` on a const item reaches
*only* the parent of the declaring module. `outer::read()` succeeds
because `outer` is the parent of `inner`; `outer::inner::PARENT_ONLY`
from `main` (the crate root, outside `outer`) fires E0603 — same
exhaustion pattern lesson 103 installed for fn items, now witnessed
on a const. The `note:` block underlines the whole `pub(super) const
PARENT_ONLY: u32 = 42;` line including the visibility modifier —
visual confirmation that the `Visibility?` slot is part of the item
declaration.

### Probe 5 — auxiliary observation: `pub` on a function-scope const

Source (`aux_fn_pub.rs`):

```rust
fn main() {
    pub const X: u32 = 5;
    println!("X = {}", X);
}
```

Compile transcript:

```text
$ rustc aux_fn_pub.rs
$ ./aux_fn_pub
X = 5
$ echo $?
0
```

Compiles silently (no warnings, no errors), runs to exit 0,
prints `X = 5`. The `pub` is *accepted* but has no effect at
function scope: a function body is a `Block`, not a `VisItem`-bearing
position, so the visibility slot has nothing to bind to. This is
not centered in the lesson body; the lesson presents module-scope
position and visibility as the centered move, and function-scope
remains lesson 075's territory. Recorded here for completeness.

### Probe 6 — Check Yourself

Source for Check Yourself (a) (`quiz.rs`):

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

Transcript:

```text
$ rustc quiz.rs
$ ./quiz
timeout = 250
retries = 3
$ echo $?
0
```

Witnesses Check Yourself (a). Source for Check Yourself (b) is
the same with line 9 changed to
`println!("retries = {}", cfg::RETRY_LIMIT);`:

```text
$ rustc quiz_broken.rs
error[E0603]: constant `RETRY_LIMIT` is private
 --> quiz_broken.rs:9:35
  |
9 |     println!("retries = {}", cfg::RETRY_LIMIT);
  |                                   ^^^^^^^^^^^ private constant
  |
note: the constant `RETRY_LIMIT` is defined here
 --> quiz_broken.rs:3:5
  |
3 |     const RETRY_LIMIT: u32 = 3;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
$ echo $?
1
```

Witnesses Check Yourself (b)'s answer: E0603 fires with caret
under `RETRY_LIMIT` and inline label `private constant`. Confirms
the answer to (c): the `pub(crate)` opener is what makes
`TIMEOUT_MS` reachable from outside `cfg`; `RETRY_LIMIT` has no
opener so the bare-name access from `main` is rejected.

## Claim-to-evidence map

| Lesson claim | Source |
|---|---|
| A `const` declaration may sit at module scope (outside any `fn`) | Reference `items/constant-items.md` line 23 ("module or block where it is located"); Reference `items.md` lines 13-29 (`ConstantItem` is a `VisItem` alternative); Book Ch3-1 lines 122-123 ("any scope, including the global scope") |
| Module-scope const items are *visible items* and may carry a visibility prefix | Reference `items.md` lines 13-29 (`VisItem → Visibility? ( ... \| ConstantItem \| ... )`) |
| The visibility shapes are `pub`, `pub(crate)`, `pub(super)` (plus deferred `pub(self)`, `pub(in path)`) | Reference `visibility-and-privacy.md` lines 8-15 (already installed by lesson 103) |
| Default is private to the declaring module | Reference `visibility-and-privacy.md` line 31 ("By default, everything is *private*"; already installed by lessons 096/103) |
| A failed access on a private const fires E0603 with inline label `private constant` | Probes 2, 3, 4, 6 above |
| The diagnostic shape (caret at use site, `note:` at definition) matches lesson 096's E0603 transcript | Probe 2 transcript above; lesson 096 evidence appendix Probe 2 |
| `pub(super)` on a const reaches the parent module only | Probe 4 above; same exhaustion pattern lesson 103 installed for fn items |
| The path form `module::ITEM` reads the constant from outside the declaring module | Probe 1 (`inner::SHARED`); same form lesson 043 installed for `module::name(args)` |
| The right-hand side stays a *constant expression* (no new rule today) | Lesson 075 (load-bearing); today's probes use literals only |
| `pub` on a function-scope const compiles silently with no effect | Probe 5 above |
| rmp's `pub(crate) const LIMB_SIZE_BITS: u64 = ...;` is exactly today's shape on the left-hand side | rmp-target-audit lines 39, 394-396 |

## Notes

- The probe directories `/tmp/lesson109.*/` etc. are ephemeral and
  not committed; only `observations/109-module-scope-const.rs` is.
- Why `MAX_BYTES` at the crate root *and* `SHARED`/`LOCAL` inside
  `inner`: the working probe needs to exhibit both the *position*
  rule (a const can sit outside any `fn`) and the *visibility*
  rule (the same const can carry `pub(crate)`/`pub(super)`/no
  marker). The crate-root `MAX_BYTES` shows position with no
  visibility content (it is in the same module as its only
  consumer); the `inner::SHARED`/`inner::LOCAL` pair shows
  visibility content (the centered contrast in Probe 2 swaps the
  marker on `SHARED`).
- Why the contrast probe drops `pub(crate)` from `SHARED` rather
  than from a crate-root const: a const at the crate root is
  visible to every descendant module (including `inner`) regardless
  of visibility marker because the privacy-by-default rule passes
  inside the declaring module *and all its descendants* (Reference
  `visibility-and-privacy.md` access rules). Dropping `pub(crate)`
  from the crate-root `MAX_BYTES` does not fire E0603 from `main`
  for the same reason. To witness E0603, the contrast must put the
  const inside a non-ancestor module, which is what
  `mod inner { const SHARED: u64 = 64; }` does.
- The auxiliary E0015 (lesson 075's "constant-expression required"
  rule) is not re-witnessed today — that rule is unchanged by the
  module-scope position. Same for the "no `mut`" rule.
