# Evidence — 103-restricted-visibility

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in `/tmp/lesson103-probes/` on this host. Same toolchain
  family as recent accepted lessons (082-102).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/103-restricted-visibility.rs`
is the working probe verbatim, with header comments naming the
centered E0603 contrast and the auxiliary witnesses captured below.

## Sources

### `output/docs/rust/reference/visibility-and-privacy.md`

The primary corpus for today. Book Ch7-2 / Ch7-3 cover plain `pub`
(load-bearing in lesson 096) but do not document the restricted forms;
the Reference is the only corpus source for `pub(super)` and
`pub(crate)`.

#### Lines 6-15 — the formal `Visibility` grammar

> **Syntax**
> Visibility →
>       pub
>     | pub ( crate )
>     | pub ( self )
>     | pub ( super )
>     | pub ( in SimplePath )

Corpus warrant for *The Move*'s "the Reference's `Visibility` grammar
admits five shapes" claim and for *What To Ignore For Now*'s naming
of `pub(self)` and `pub(in path)`. Lesson 096 already cited lines 9-15
as the formal grammar; today centers two more rows of the same five-row
production.

#### Lines 29-31 — privacy-by-default rule (carried over from lesson 096)

> By default, everything is *private*, with two exceptions: Associated
> items in a `pub` Trait are public by default; Enum variants in a
> `pub` enum are also public by default.

Carried over from lesson 096; cited because today's contrast probe
(Probe 2) relies on the unmarked-private case still firing E0603.

#### Lines 125-145 — the four restricted-visibility shapes

> ## `pub(in path)`, `pub(crate)`, `pub(super)`, and `pub(self)`
>
> In addition to public and private, Rust allows users to declare an
> item as visible only within a given scope. The rules for `pub`
> restrictions are as follows:
>
> - `pub(in path)` makes an item visible within the provided `path`.
>   `path` must be a simple path which resolves to an ancestor module
>   of the item whose visibility is being declared. Each identifier in
>   `path` must refer directly to a module (not to a name introduced by
>   a `use` statement).
> - `pub(crate)` makes an item visible within the current crate.
> - `pub(super)` makes an item visible to the parent module. This is
>   equivalent to `pub(in super)`.
> - `pub(self)` makes an item visible to the current module. This is
>   equivalent to `pub(in self)` or not using `pub` at all.

Corpus warrant for *The Move* piece (1) ("`pub(super)` — visible only
to the *parent* module"), piece (2) ("`pub(crate)` — visible *anywhere
within this crate*"), the *What Changed* radius table, and the *What
To Ignore For Now* defer of `pub(self)` ("equivalent to ... not using
`pub` at all" — verbatim line 145) and `pub(in path)` ("makes an item
visible within the provided `path`" — line 133, plus the
"ancestor module" constraint).

The Reference's wording is precise about `pub(super)`: it grants
visibility *to the parent module* (line 141). This is what licenses
the working probe — `inner`'s parent is the crate root, and `fn main`
is in the crate root, so the call resolves.

#### Lines 153-200 — the multi-modifier worked example

The Reference's nested `outer_mod / inner_mod` example exercises all
four restricted shapes in one program, with explicit comments on which
calls are visible from which module. Today's probes are simpler
(top-level `mod inner` only, `pub(super)` and `pub(crate)` only) but
the rule statements come directly from this section. The Reference
example explicitly shows that `outer_mod::inner_mod::super_mod_visible_fn()`
fails *outside* `outer_mod` — a structural parallel to today's Probe 3.

#### Line 206-207 — additional restriction note

> This syntax only adds another restriction to the visibility of an
> item. It does not guarantee that the item is visible within all
> parts of the specified scope. To access an item, all of its parent
> items up to the current scope must still be visible as well.

Cited only in *What To Ignore For Now* under "Visibility on the `mod`
declaration itself" — the rule that ancestor modules must also be
visible motivates that follow-on move. Today's probe avoids the issue
by keeping the parent (`mod inner`) at the crate root with default
privacy; the call goes through fine because the crate root's items
are accessible from anywhere in the crate.

### `output/docs/rust/error_codes/E0603.md`

Reused warrant from lesson 096. The corpus's erroneous example is the
same `mod foo { ... } foo::PRIVATE` shape lesson 096 used; today's
Probe 2 reuses that shape with a `private` function instead of a
constant.

> A private item was used outside its scope.

Today's contrast probe witnesses E0603 firing on the unmarked-private
function `inner::private()`, demonstrating that today's `pub(super)`
and `pub(crate)` opt the items into access where the unmarked one
remains private.

### `output/docs/rust/book/ch07-02-defining-modules-to-control-scope-and-privacy.md` and `ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md`

Cited only as the parent context (already lesson 096's primary corpus).
Neither chapter documents `pub(super)` or `pub(crate)` in body text; a
quick `grep "pub(" ch07-02 ch07-03` returns no matches. The Reference
is the unique corpus for the restricted shapes.

## Probes

### Probe 1 — Working program

The committed observation file. Run in `/tmp/lesson103-probes/`:

```text
--- rustc --version ---
rustc 1.95.0 (59807616e 2026-04-14)
--- uname -sm ---
Darwin x86_64
--- cat demo.rs ---
mod inner {
    pub(super) fn for_super() -> u32 { 1 }
    pub(crate) fn for_crate() -> u32 { 2 }
}

fn main() {
    println!("super = {}, crate = {}", inner::for_super(), inner::for_crate());
}
--- rustc demo.rs ---
exit=0
--- ls after compile ---
demo
demo.rs
--- ./demo ---
super = 1, crate = 2
exit=0
```

Witness for: both restricted-visibility forms compile silently in the
function-item position; the call sites `inner::for_super()` and
`inner::for_crate()` from `fn main` resolve to the bodies; the program
prints both return values and exits 0 with no warnings.

### Probe 2 — Centered E0603 contrast (unmarked-private call)

Source `private_call.rs`, identical to Probe 1 plus a third item `fn
private() -> u32 { 3 }` (no `pub`-anything) and a third call from
`main`:

```text
--- cat private_call.rs ---
mod inner {
    pub(super) fn for_super() -> u32 { 1 }
    pub(crate) fn for_crate() -> u32 { 2 }
    fn private() -> u32 { 3 }
}

fn main() {
    println!("super = {}, crate = {}, private = {}",
        inner::for_super(),
        inner::for_crate(),
        inner::private());
}
--- rustc private_call.rs ---
error[E0603]: function `private` is private
  --> private_call.rs:11:16
   |
11 |         inner::private());
   |                ^^^^^^^ private function
   |
note: the function `private` is defined here
  --> private_call.rs:4:5
   |
 4 |     fn private() -> u32 { 3 }
   |     ^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
exit=1
```

Witness for: a function inside `mod inner` *without* any `pub(...)` form
is still subject to lesson 096's privacy rule and fires the same E0603
diagnostic. The same `mod inner` block hosts both `pub(super)` and
`pub(crate)` items that resolve fine and one unmarked item that does
not. This grounds the lesson's centered teaching point: today's two
modifiers are *opt-ins* to the access rule, just like plain `pub`.

The source-diff to Probe 1 is one new item (`fn private`) plus a third
call argument; the modifiers on the other two items are unchanged. The
selectivity of the diagnostic — only `private` is flagged — empirically
confirms that `pub(super)` and `pub(crate)` are recognized as forms of
`pub` for the access check.

### Probe 3 — Auxiliary contrast: `pub(super)` out of reach

Source `super_out_of_reach.rs` — the deeper-nesting case where
`pub(super)` matters. Two levels of module nesting; the inner module
is itself `pub mod` so the path resolves; `pub(super)` only reaches the
inner module's *parent* (here `outer`), and `fn main` is outside
`outer`:

```text
--- cat super_out_of_reach.rs ---
mod outer {
    pub mod inner {
        pub(super) fn for_super() -> u32 { 1 }
    }
}

fn main() {
    println!("{}", outer::inner::for_super());
}
--- rustc super_out_of_reach.rs ---
error[E0603]: function `for_super` is private
 --> super_out_of_reach.rs:8:34
  |
8 |     println!("{}", outer::inner::for_super());
  |                                  ^^^^^^^^^ private function
  |
note: the function `for_super` is defined here
 --> super_out_of_reach.rs:3:9
  |
3 |         pub(super) fn for_super() -> u32 { 1 }
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
exit=1
```

Witness for: `pub(super)` is genuinely *restricted* — it doesn't open
the item to the whole crate, only to the parent of the declaring
module. From `fn main` (outside `outer`), the call to a `pub(super)`
function inside `outer::inner` fails with the same E0603 the
unmarked-private case fires.

This is the transcript referenced by *Check Yourself*'s answer (b). The
diagnostic underlines the entire `pub(super) fn for_super()` line at
the definition site (column 9 through the function-name span), in
contrast to Probe 2's underline of the bare `fn private()` line — a
structural difference the lesson body does not center but the appendix
records for red-team verification.

### Probe 4 — Auxiliary witness: `pub(crate)` reaches through nesting

Source `crate_reaches_through.rs` — same nested setup as Probe 3, but
with `pub(crate)` in place of `pub(super)`:

```text
--- cat crate_reaches_through.rs ---
mod outer {
    pub mod inner {
        pub(crate) fn for_crate() -> u32 { 2 }
    }
}

fn main() {
    println!("{}", outer::inner::for_crate());
}
--- rustc crate_reaches_through.rs ---
exit=0
--- ./crate_reaches_through ---
2
exit=0
```

Witness for: `pub(crate)` *does* reach `fn main` even from two levels
deep, because `fn main` is in the same crate. This is the
operational-difference probe between today's two modifiers — the
identical nesting that breaks `pub(super)` (Probe 3) does not break
`pub(crate)` (Probe 4). The lesson body's *Mental Model Delta* claim
"`pub(crate)` reaches everywhere *inside this crate*" is grounded
empirically here.

(The intermediate `pub mod inner` is needed because the privacy chain
rule from Reference visibility-and-privacy.md lines 206-207 still
applies — every ancestor must be visible. Without `pub mod inner`,
`outer::inner::for_crate` would fail because `inner` itself is private
to `outer`, blocking the path before the call's own visibility check
runs. This subtlety is intentionally deferred from the lesson body
under *What To Ignore For Now*'s "Visibility on the `mod` declaration
itself" bullet.)

## Prior lessons

### Direct prerequisites

- **096-inline-module-and-pub** (accepted, *load-bearing*) — installs:
  - inline submodule declaration `mod foo { ... }` at module scope;
  - the privacy-by-default rule for items inside a module;
  - plain `pub` as the keyword that opens an item to outside callers
    via `module::item`;
  - the E0603 diagnostic for failed visibility checks at the module
    boundary, with caret at the use site and `note:` block at the
    definition.

  Today extends 096 by exactly one rule family: the same `pub`
  position now admits `pub(super)` and `pub(crate)` as alternative
  modifier shapes, with narrower reach than plain `pub`. Probe 1
  (working program) reuses 096's `mod foo { ... } fn ... fn main { ...
  }` skeleton verbatim with the only diff being two `pub(...)`
  modifiers in place of `pub`. Probe 2 reuses 096's E0603 contrast
  exactly (a function with no `pub`-anything fires E0603 when called
  from outside the module), now embedded in the same probe alongside
  the two new modifiers. Lesson 096's *unlocks* list explicitly named
  "future restricted visibility `pub(super)` ... and `pub(crate)`
  moves" — today is exactly that pair.

### Supporting prior lessons (cited only)

- **003-read-rustc-diagnostic** — the four-part diagnostic map applied
  to Probe 2's E0603 transcript. Today's diagnostic is the same shape
  lesson 096 first installed for E0603 (caret at use site, separate
  `note:` block with `-->` at definition site).
- **002-fn-main-entry-point** — `fn main` as the entry point; the
  probe uses one `fn main` block, unchanged.
- **005-let-binding** — not used in today's probe (the call results
  flow directly into `println!`); cited for completeness.
- **008-define-and-call-function** — the `fn name() -> Type { ... }`
  shape and the `name()` call shape; both unchanged.
- **011-println-positional-args** — `println!` with two `{}` slots
  consumed by two function-call arguments. Probe 2 extends to three
  `{}` slots.
- **043-nested-module-paths** — the path call form `module::name(args)`
  with `::` between segments; today's `inner::for_super()`,
  `inner::for_crate()`, `inner::private()`, `outer::inner::for_super()`,
  and `outer::inner::for_crate()` are all this form, unchanged.
- **069-rustc-warnings** — Probes 2 and 3's `error: aborting due to 1
  previous error` trailer is read with lesson 069's category map; the
  working probe (Probe 1) emits no warnings (deliberate — early
  iteration of the working probe included an unmarked `fn private`,
  which fired the `dead_code` warn-by-default lint; the committed
  working probe omits it, deferring the unmarked case to Probe 2).
- **001-rustc-compile-and-run** — `rustc demo.rs` then `./demo`,
  silent on success.

## Probe-shape note

The working probe (Probe 1) uses two items, one with each modifier, in
a single inline module at the crate root. This shape was chosen because:

1. *Both* modifiers must be exercised in one program for the lesson's
   "two coupled syntactic shapes" framing to hold. A single-modifier
   probe would have required two separate working programs.
2. *Top-level* `mod inner` (rather than nested `mod outer { mod inner
   { ... } }`) is the simplest shape where `pub(super)` and
   `pub(crate)` are *both* operationally distinguishable from each
   other and from plain `pub`. (At top level, `pub(super)` ↔ "visible
   to the crate root" and `pub(crate)` ↔ "visible to any module in
   the crate" both happen to license `fn main` calls — *because* the
   parent of `inner` *is* the crate root. The auxiliary Probe 3
   distinguishes them at depth.)
3. The unmarked `fn private` was *removed* from Probe 1 to keep it
   warning-free, then added back in Probe 2 as the centered contrast.
   This split is documented in the observation file's header comment.

The lesson body chooses Probe 2 (unmarked-private contrast, top-level
nesting) as the centered contrast because it reuses lesson 096's
identical E0603 shape — minimal cognitive load, maximum reuse of
already-installed machinery. Probe 3 (`pub(super)` out-of-reach,
deeper nesting) is the *operational distinguisher* between the two
modifiers and is referenced by *Check Yourself*'s answer (b), but the
deeper-nesting subtleties (privacy chain on the parent module) make
it a less clean centered probe. This trade-off mirrors the lesson 096
choice (E0603 centered, E0425 auxiliary).

## Mapping summary

| Lesson claim | Source / probe |
|---|---|
| The `Visibility` grammar admits five shapes | Reference visibility-and-privacy.md lines 6-15 (formal grammar) |
| `pub(super)` is visible only to the parent module | Reference visibility-and-privacy.md line 141 verbatim; Probe 1 (works from parent), Probe 3 (fails outside parent) |
| `pub(crate)` is visible anywhere within the current crate | Reference visibility-and-privacy.md line 137 verbatim; Probe 1 (works from crate root), Probe 4 (works from non-ancestor module) |
| Plain `pub` reaches library consumers; `pub(crate)` does not | Reference visibility-and-privacy.md line 137 ("within the current crate") and lines 137-141 contrast |
| The same E0603 fires on a failed visibility check | Probe 2 transcript verbatim; Probe 3 transcript verbatim; lesson 096 already installed E0603 for the unmarked case |
| Both modifiers go in the same syntactic position as plain `pub` | Reference visibility-and-privacy.md lines 6-15 (`Visibility` grammar admits all five as a single non-terminal); Probes 1, 3, 4 all place the modifier directly before `fn` |
| Without any `pub` form, the item is private to its module | Reference visibility-and-privacy.md lines 29-31 (privacy by default); lesson 096 (carried over); Probe 2 transcript |
| `pub(self)` is equivalent to no `pub` at all | Reference visibility-and-privacy.md line 145 verbatim |
| `pub(in path)` takes a module path argument | Reference visibility-and-privacy.md line 133 verbatim |

No lesson claim relies on a fact that does not appear in either a
listed corpus passage or a captured probe.
