# Evidence — 105-pub-use-re-export

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` → `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` → `Darwin x86_64`
- Probes run in `/tmp/lesson105-probes/` on this host. Same toolchain
  family as recent accepted lessons (082-104).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/105-pub-use-re-export.rs`
is the working probe verbatim, with header comments naming the expected
output and the centered E0603 contrast captured below.

## Sources

### `output/docs/rust/reference/items/use-declarations.md`

The primary corpus for today.

#### Lines 10-15 — `UseDeclaration` and `UseTree` grammar

> UseDeclaration → use UseTree ;
>
> UseTree →
>       ( SimplePath? :: )? * <br>
>     | ( SimplePath? :: )? { ( UseTree ( , UseTree )* ,? )? } <br>
>     | SimplePath ( as ( IDENTIFIER | _ ) )?

Corpus warrant for the lesson's "*A new item shape*: `pub use
Path::Item;`. Same syntactic position as a plain `use`, with `pub` in
front." `UseDeclaration` is itself an *item* (Reference's items chapter
lists it under modules); items can take a `Visibility?` slot in front,
which is where `pub` and the `pub(...)` family from lesson 103 sit.

#### Lines 19 — re-export framing

> A *use declaration* creates one or more local name bindings
> synonymous with some other path. [...] A `use` declaration is also
> sometimes called an *import*, or, if it is public, a *re-export*.

Corpus warrant for the lesson's "The Reference calls this
*re-exporting*." Names the term verbatim and ties it to the `pub`
keyword.

#### Lines 68-78 — `use` Visibility section (load-bearing)

> ## `use` Visibility
>
> Like items, `use` declarations are private to the containing module,
> by default. Also like items, a `use` declaration can be public, if
> qualified by the `pub` keyword. Such a `use` declaration serves to
> *re-export* a name. A public `use` declaration can therefore
> *redirect* some public name to a different target definition: even a
> definition with a private canonical path, inside a different module.
>
> If a sequence of such redirections form a cycle or cannot be
> resolved unambiguously, they represent a compile-time error.

Corpus warrant — load-bearing — for *The Move*'s "qualified by the
`pub` keyword" / "re-export a name" quote in the lesson body. Also the
warrant for *What To Ignore For Now*'s "Cycle detection in re-export
chains" deferral.

#### Lines 80-97 — re-export example (load-bearing)

> An example of re-exporting:
>
> ```rust
> mod quux {
>     pub use self::foo::{bar, baz};
>     pub mod foo {
>         pub fn bar() {}
>         pub fn baz() {}
>     }
> }
>
> fn main() {
>     quux::bar();
>     quux::baz();
> }
> ```
>
> In this example, the module `quux` re-exports two public names
> defined in `foo`.

Corpus warrant for today's working probe shape. The Reference's
example uses *exactly* the same pattern: a `mod` with a `pub mod`
inside, a `pub use self::inner::name;` re-export at the outer level,
and the call site `outer::name()` from `fn main`. Today's probe is the
Reference's example with `quux` → `inner`, `foo` → `hidden`, `{bar,
baz}` → `value`. The use of `self::` in the path is the Reference's
own choice, not invented for the lesson.

#### Lines 163-179 — `as` renames (deferred)

> ## `as` renames
>
> The `as` keyword can be used to change the name of an imported
> entity. For example:
>
> ```rust
> use inner::foo as bar;
> ```

Corpus warrant for *What To Ignore For Now*'s "*`pub use Path::Item as
Alias;`* — re-export under a *different* name." Named, deferred.

### `output/docs/rust/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.md`

#### Lines 214-258 — Re-exporting Names with `pub use` section (load-bearing)

> ### Re-exporting Names with `pub use`
>
> When we bring a name into scope with the `use` keyword, the name is
> private to the scope into which we imported it. To enable code
> outside that scope to refer to that name as if it had been defined
> in that scope, we can combine `pub` and `use`. This technique is
> called *re-exporting* because we're bringing an item into scope but
> also making that item available for others to bring into their
> scope.

Corpus warrant — load-bearing — for *Mental Model Delta*'s before/after
contrast. The Book's framing "the name is private to the scope into
which we imported it" / "to enable code outside that scope to refer to
that name as if it had been defined in that scope" is exactly today's
mental-model delta.

The Book's Listing 7-17 (lines 228-240) shows the same pattern: a
`pub use crate::front_of_house::hosting;` re-export at the crate root
that lets external callers reach `hosting` directly. The lesson body
adapts this to a single-file `rustc` probe.

The Book's lines 244-248 — "Before this change, external code would
have to call the `add_to_waitlist` function by using the path
`restaurant::front_of_house::hosting::add_to_waitlist()` [...]. Now
that this `pub use` has re-exported the `hosting` module from the root
module, external code can use the path
`restaurant::hosting::add_to_waitlist()` instead." — is the
shorter-path framing the lesson uses for the rmp note ("callers say
`bignum::biguint::BigUInt` instead of the longer
`bignum::biguint::basic::BigUInt`").

### `output/docs/rust/error_codes/E0603.md`

> A private item was used outside its scope.

Corpus warrant for the contrast probe's E-code. E0603 is the same
E-code lesson 096 first installed for "function `hi` is private";
today's diagnostic carries the same E0603 with the slightly
specialized phrase "private function *import*" because the gating item
is a `use` declaration rather than the function definition itself.
This is the same family — the lesson body explicitly says "Same E0603
lesson 096 installed."

## Probes

All probes run from `/tmp/lesson105-probes/` on the host described in
*Toolchain*.

### Probe 1 — working `pub use` re-export

The committed observation file. Reproduced for grounding:

```rust
mod inner {
    pub mod hidden {
        pub fn value() -> u32 {
            42
        }
    }
    pub use self::hidden::value;
}

fn main() {
    println!("via re-export: {}", inner::value());
    println!("via original:  {}", inner::hidden::value());
}
```

```
$ rustc demo.rs
$ ./demo
via re-export: 42
via original:  42
$ echo $?
0
```

Witnesses today's centered claim: *both* `inner::value()` (the
re-export) and `inner::hidden::value()` (the original path) reach the
same function. Same return value, same printout, no errors, no
warnings. The `pub use self::hidden::value;` line *adds* the
`inner::value` path; it does not remove `inner::hidden::value`.

### Probe 2 — centered contrast: drop `pub` from the `use`

```rust
mod inner {
    pub mod hidden {
        pub fn value() -> u32 {
            42
        }
    }
    use self::hidden::value;
}

fn main() {
    println!("via re-export: {}", inner::value());
}
```

```
$ rustc no_pub.rs
error[E0603]: function import `value` is private
  --> no_pub.rs:11:42
   |
11 |     println!("via re-export: {}", inner::value());
   |                                          ^^^^^ private function import
   |
note: the function import `value` is defined here...
  --> no_pub.rs:7:9
   |
 7 |     use self::hidden::value;
   |         ^^^^^^^^^^^^^^^^^^^
note: ...and refers to the function `value` which is defined here
  --> no_pub.rs:3:9
   |
 3 |         pub fn value() -> u32 {
   |         ^^^^^^^^^^^^^^^^^^^^^ you could import this directly
help: consider importing this function instead
   |
11 |     println!("via re-export: {}", inner::hidden::value());
   |                                          ++++++++
help: import `value` through the re-export
   |
11 -     println!("via re-export: {}", inner::value());
11 +     println!("via re-export: {}", hidden::value());
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
$ echo $?
1
```

Witnesses today's contrast claim. The single-character difference from
Probe 1 — the missing `pub` keyword — produces:

- E0603 ("private item used outside its scope"). Same E-code lesson
  096 installed; the slightly specialized phrase "private function
  *import*" matches because the gating item is now the `use`
  declaration rather than the underlying function.
- A *chain* of `note:` blocks: the first points at the private `use`
  line (the immediate cause); the second chases the redirect to the
  underlying `pub fn value()`. The Reference's "redirect [...] to a
  different target definition" framing is visible in this two-step
  diagnostic chain.
- Two `help:` blocks suggesting the workaround (write the original
  path `inner::hidden::value()`, or re-import locally).

The diagnostic's four parts (lesson 003 map):

- Headline: `error[E0603]: function import \`value\` is private`.
- Location: `--> no_pub.rs:11:42`.
- Source excerpt with carets at the call site (line 11) plus two
  `note:` re-anchors, each with its own `-->` location.
- Help/note: two `help:` suggestion blocks plus the standard
  `error: aborting due to 1 previous error` and `--explain E0603`
  trailer.

This is the negative/contrast probe required by the README for
"with X works, without X fails" claims: it proves the `pub` is what
makes the re-export reachable from outside.

### Probe 3 — auxiliary witness: plain `use` *does* work *inside* the module

```rust
mod inner {
    pub mod hidden {
        pub fn value() -> u32 { 42 }
    }
    use self::hidden::value;
    pub fn local_caller() -> u32 {
        value()
    }
}

fn main() {
    println!("{}", inner::local_caller());
}
```

```
$ rustc local_use_works.rs
$ ./local_use_works
42
$ echo $?
0
```

Witnesses the *symmetric* half of the rule. The same plain
`use self::hidden::value;` that fails to expose `inner::value` to
outside callers (Probe 2) *does* let `inner::local_caller` call
`value()` bare *inside* `inner` — that is exactly the lesson-044
behaviour. So the bare `use` *does* bind `value` in `inner`'s local
scope; what it does *not* do is re-export. `pub use` adds the
re-export half on top of the local-scope binding.

This probe is *not* mentioned in the lesson body but is what licenses
the lesson body's "the bare name works *inside* the module but is
invisible from outside" sentence in *What Changed*.

### Note — `self::`-prefix observation

Probe 1's working `pub use` line reads `pub use self::hidden::value;`
not `pub use hidden::value;`. The bare form fires E0432
("unresolved import `hidden`") under `rustc demo.rs`:

```
$ rustc demo_no_self.rs
error[E0432]: unresolved import `hidden`
 --> demo_no_self.rs:7:13
  |
7 |     pub use hidden::value;
  |             ^^^^^^
  |
help: a similar path exists
  |
7 |     pub use self::hidden::value;
  |             ++++++
```

The diagnostic itself proposes the `self::` fix. This is an
edition/path-resolution detail orthogonal to today's `pub` move. The
Reference's own re-export example (use-declarations.md lines 80-97)
uses `pub use self::foo::{bar, baz};` — same `self::` prefix — so the
lesson body uses the Reference's shape. *What To Ignore For Now*
explicitly defers the rule for *why* the bare form fails.

The rmp target's `pub use basic::BigUInt;` works without `self::`
because Cargo's library-build differs from `rustc`'s single-file
compilation; not relevant to today's `pub`-on-`use` mental model.

## Prerequisite-claim summary

Direct prerequisites (load-bearing claims today depends on):

- **Lesson 044 (load-bearing)** — installs `use Path::Item;`: the
  `use` keyword as a top-level item, the path syntax in the argument
  position, the trailing `;`, and the effect of bringing the final
  segment into the current scope. Today's `pub use self::hidden::value;`
  is *exactly* this shape with `pub` added. Without lesson 044, the
  audience would not know what a bare `use` means, what scope it
  binds into, or what the path-shape rules are.
- **Lesson 096 (load-bearing)** — installs `mod foo { ... }`, `pub` on
  a function item, the privacy-by-default rule for items inside a
  module, the access form `module::item`, and the E0603 diagnostic.
  Today applies the *same* `pub` keyword to a *different* item kind
  (a `use` declaration). The Reference's "Like items, `use`
  declarations are private to the containing module, by default. Also
  like items, a `use` declaration can be public" makes this analogy
  explicit. Probe 2's E0603 transcript carries the same E-code lesson
  096 first installed; the only difference is the slightly
  specialized "private function *import*" phrasing.

Older supporting lessons (named only, not load-bearing):

- Lesson 043 (`module::name(args)`) — both call sites in Probe 1
  (`inner::value()` and `inner::hidden::value()`) are this shape,
  unchanged.
- Lesson 003 (rustc diagnostic four-part map) — applied to Probe 2's
  E0603 transcript; the diagnostic structure is unchanged from
  lesson 003.
- Lesson 103 (restricted visibility) — named in *What To Ignore For
  Now* as the natural composition for `pub(crate) use ...` and
  `pub(super) use ...`. Not used in any committed probe today.
- Lesson 104 (`super::` and `crate::`) — accepted at commit
  `b8a05aa33` immediately before this lesson; named in the rmp note
  about callers writing `bignum::biguint::BigUInt`. Not load-bearing
  for today's centered move.
- Lesson 002 (`fn main`), lesson 011 (`println!` positional args),
  lesson 008 (define and call function), lesson 062 (`u32` integer
  type), lesson 069 (rustc warning category map), lesson 001 (`rustc`
  and `./executable`) — unchanged supporting machinery.

## Probe-shape notes

- The working probe nests `pub mod hidden` inside `mod inner` so that
  `pub use self::hidden::value;` *re-exports a name from a deeper
  module*, exactly the rmp `biguint/mod.rs` pattern. Both call sites
  reach the same function so the re-export effect is observable from
  outside.
- The contrast probe is byte-identical to the working probe modulo the
  single deleted `pub` keyword on the `use` line. This is the cleanest
  shape for a "with X works, without X differs" probe.
- `pub mod hidden` is itself `pub` so that the *original path*
  `inner::hidden::value` remains reachable; that lets the working
  probe demonstrate "both paths reach the same function." If
  `pub mod hidden` were dropped to plain `mod hidden`, only the
  re-export path would work — a different (also valid) shape, but not
  the one the lesson body claims.
- All probes use only `u32` (lesson 062), `mod` and `pub mod` (lesson
  096), `use` declarations (lesson 044), `fn` (lesson 008), `println!`
  with one or two `{}` slots (lesson 011), `let`-free.
