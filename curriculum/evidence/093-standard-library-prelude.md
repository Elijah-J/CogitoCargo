# Evidence — 093-standard-library-prelude

This appendix grounds the lesson's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -srvm` -> `Darwin 24.5.0 Darwin Kernel Version 24.5.0:
  Tue Apr 22 19:53:26 PDT 2025; root:xnu-11417.121.6~2/RELEASE_X86_64
  x86_64`
- Probes run in `/tmp/eduratchet093/` on this host. Same toolchain
  family as recent accepted lessons (082-092).

The committed observation file at
`experimental/eduratchet2/runs/rust-moves/observations/093-standard-library-prelude.rs`
is the working probe verbatim, with header comments naming the
expected output and the contrast probe.

## Sources

### `output/docs/rust/reference/names/preludes.md`

Reference's *Preludes* page. The load-bearing definition is line 8,
the load-bearing taxonomy is lines 14-20, and the load-bearing
edition table is lines 32-39:

> A *prelude* is a collection of names that are automatically brought
> into scope of every module in a crate.
>
> These prelude names are not part of the module itself: they are
> implicitly queried during name resolution. For example, even though
> something like `Box` is in scope in every module, you cannot refer
> to it as `self::Box` because it is not a member of the current
> module.

> There are several different preludes:
>
> - Standard library prelude
> - Extern prelude
> - Language prelude
> - `macro_use` prelude
> - Tool prelude

> Each crate has a standard library prelude, which consists of the
> names from a single standard library module.
>
> The module used depends on the crate's edition, and on whether the
> `no_std` attribute is applied to the crate:
>
> | Edition | `no_std` not applied | `no_std` applied |
> | --- | --- | --- |
> | 2015 | `std::prelude::rust_2015` | `core::prelude::rust_2015` |
> | 2018 | `std::prelude::rust_2018` | `core::prelude::rust_2018` |
> | 2021 | `std::prelude::rust_2021` | `core::prelude::rust_2021` |
> | 2024 | `std::prelude::rust_2024` | `core::prelude::rust_2024` |

Direct corpus warrant for the lesson's centered claims:

- *A prelude is a collection of names automatically in scope of every
  module*: line-8 quote verbatim.
- *There are several preludes; the standard library prelude is one
  of them*: lines 14-20. The lesson narrows to the standard library
  prelude and lists the other four under *What To Ignore For Now*.
- *Edition 2024 selects `std::prelude::rust_2024`*: row 4 of the
  edition table. The lesson cites this rule as the reason the
  default `cargo new` package's prelude module name is
  `std::prelude::rust_2024`.

### `experimental/eduratchet2/runs/rust-moves/lessons/052-result-enum-and-is-ok.md`

Lesson 052's body, lines 21-22 (also restated in *Mental Model
Delta* lines 58-59):

> `Result` (along with `Ok` and `Err`) is in the *prelude*, so no
> `use` line is needed.

Direct corpus warrant for the lesson's centered claim that lesson
052's `Result`/`Ok`/`Err` worked bare *because* of the prelude.
Lesson 052 used the word "prelude" inline but did not install it as
a centered concept; today does.

## Probes

### Probe 1 — working: prelude names used bare

Source (`/tmp/eduratchet093/demo.rs`):

```rust
fn main() {
    let s: String = String::new();
    let v: Vec<i32> = Vec::new();
    let r: Result<i32, String> = Ok(42);
    let opt: Option<i32> = Some(7);
    println!("s = {:?}", s);
    println!("v = {:?}", v);
    println!("r = {:?}", r);
    println!("opt = {:?}", opt);
}
```

Transcript:

```
$ rustc demo.rs
$ ./demo
s = ""
v = []
r = Ok(42)
opt = Some(7)
```

rustc compiled silently (no warnings, exit 0). Run produced four
lines of output exactly as predicted. Witnesses:

- *`String` is reachable with no `use` and no `std::` prefix*: line
  2's `String::new()` resolves; the `: String` annotation also
  resolves (the type name itself is in the prelude).
- *`Vec<T>` is reachable with no `use` and no `std::` prefix*: line
  3's `Vec::new()` and `: Vec<i32>` annotation both resolve.
- *`Result`/`Ok` are reachable bare*: line 4's `: Result<i32, String>`
  and `Ok(42)` both resolve. The `String` inside the `Result` type
  parameter is the same prelude `String` from line 2.
- *`Option`/`Some` are reachable bare*: line 5's `: Option<i32>` and
  `Some(7)` both resolve. (`Some`/`None` are the constructors of
  `Option<T>`; the lesson names them but does not exercise `None`.)
- *`{:?}` works as a `println!` formatter*: lines 6-9 use `{:?}`
  successfully against `String`, `Vec<i32>`, `Result<i32, String>`,
  and `Option<i32>`. The lesson glosses `{:?}` as the *Debug*
  formatter and defers the `Display`-vs-`Debug` distinction.

### Probe 2 — `cargo new` default edition

The lesson's body (lines 143-145) asserts that "the `cargo new`
default edition is 2024, so the prelude in scope is
`std::prelude::rust_2024`." The Reference's edition table (row 4,
above) warrants the *implication* "edition 2024 selects
`std::prelude::rust_2024`," but the antecedent — that `cargo new`
defaults to edition 2024 on this host — is an empirical, version-
dependent claim about cargo behavior. This probe captures it.

Source-of-truth: a fresh `cargo new` package on this host's cargo.

Transcript (`/tmp/eduratchet093-revision/`):

```
$ cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
$ cargo new --vcs none check_edition
    Creating binary (application) `check_edition` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cat check_edition/Cargo.toml
[package]
name = "check_edition"
version = "0.1.0"
edition = "2024"

[dependencies]
```

Witness: with `cargo 1.95.0` on this host, `cargo new --vcs none
check_edition` writes `edition = "2024"` to the package's
`Cargo.toml` with no flags, no template, and no override. Combined
with the Reference's edition table (row 4: edition 2024 selects
`std::prelude::rust_2024`), this empirically grounds the lesson's
claim that the default `cargo new` package's prelude module is
`std::prelude::rust_2024`. The claim is version-dependent (a future
cargo release could change the default), but it holds for the
toolchain pinned in *Toolchain* above.

### Probe 3 — contrast: a non-prelude name used bare

Source (`/tmp/eduratchet093/broken.rs`):

```rust
fn main() {
    let m = HashMap::new();
}
```

Transcript:

```
$ rustc broken.rs
error[E0433]: cannot find type `HashMap` in this scope
 --> broken.rs:2:13
  |
2 |     let m = HashMap::new();
  |             ^^^^^^^ use of undeclared type `HashMap`
  |
help: consider importing this struct
  |
1 + use std::collections::HashMap;
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0433`.
```

(rustc exit code 1.)

Witnesses:

- *`HashMap` is not in the prelude*: bare `HashMap::new()` fires
  E0433 "cannot find type `HashMap` in this scope". The compiler's
  resolver does not know the name without help.
- *The path of `HashMap` is `std::collections::HashMap`*: the
  `help:` block says `1 + use std::collections::HashMap;` — the
  compiler knows where the type lives and can suggest the `use`
  line that would put it in scope. (This `use` line is exactly
  lesson 044's mechanism.)
- *Either `use` or the full path resolves it*: the lesson cites
  `std::collections::HashMap::new()` (lesson 050's full-path
  mechanism) as the alternative the diagnostic does not show.

The lesson's body quotes the headline and the `help:` block
verbatim. The exact diagnostic is the lesson's primary witness for
the binary split: the same source shape that compiled silently in
Probe 1 (when the names were prelude members) here fails because
`HashMap` is not.

Note on the diagnostic's wording: the headline reads "cannot find
type" rather than the alternative "failed to resolve" form that
sometimes appears for path-prefixed lookups; the relevant E-code is
E0433 in either form. The wording difference does not change the
lesson's claim.

## Prerequisite-claim summary

- **Lesson 042 — `String::new()`** (load-bearing). Lesson 042's body
  established that `let s: String = String::new();` compiles in a
  bare `fn main` with no `use` declaration. Today's role: the
  prelude is *why* that works. Lesson 042's *What To Ignore For Now*
  explicitly named "*The standard prelude*" and deferred it; today
  installs it.

- **Lesson 044 — `use std::cmp::min;`** (load-bearing). Lesson 044
  installed the file-top-level `use Path::final;` form as the
  mechanism for bringing the final segment of a path into scope.
  Today's role: that is the mechanism a learner uses for any name
  *not* in the prelude. Lesson 044's *What To Ignore For Now* also
  named "*The standard prelude*" and deferred it.

- **Lesson 050 — `std::io::stdin()`** (load-bearing). Lesson 050
  used the full path `std::io::stdin()` rather than a `use`. Today's
  role: the explicit reason the full path is required is that
  `std::io::stdin` is not in the prelude. Lesson 050's *What To
  Ignore For Now* named "*The standard prelude*" and deferred it.

- **Lesson 052 — `Result<T, E>` with `Ok`/`Err` and `.is_ok()`**
  (load-bearing). Lesson 052's body and *Mental Model Delta* both
  noted in passing that "`Result` (along with `Ok` and `Err`) is in
  the *prelude*." Today's role: the lesson centers the prelude as a
  named language feature with its own move, rather than a
  parenthetical inside lesson 052.

- Lessons 002, 005, 011 (cited only): `fn main`, `let name: TYPE =
  value;`, `println!` with a positional `{}` slot. The probe extends
  lesson 011 by one formatter (`{:?}` instead of `{}`); the lesson's
  body glosses that in one sentence and defers the trait machinery.

## Contrast-probe omission justification

None — Probe 3 is a positive contrast probe. The contrast claim is
"with the prelude in effect, `String::new()` resolves; without
prelude membership, `HashMap::new()` does not." Probe 1 witnesses
the first half; Probe 3 witnesses the second half. No further
negative probe is needed.

## Notes on deferred items

The lesson defers the full prelude enumeration, edition-specific
prelude differences, the four other preludes, `#![no_std]`,
`#![no_implicit_prelude]`, glob imports, prelude re-exports, and
the `Display`/`Debug` trait distinction. None of these are
load-bearing for the centered claim "the prelude is the rule that
splits bare-usable names from `use`/full-path names."
