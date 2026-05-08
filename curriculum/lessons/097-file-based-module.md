---
id: 097-file-based-module
status: accepted
evidence: ../evidence/097-file-based-module.md
---

# Move a module's body into a separate file with `mod foo;`

## The Move

Lesson 096 declared a module *inline*: `mod foo { ... }`, body in
braces, in the same `.rs` file as `fn main`. Today, write the same
module across two adjacent files. In `main.rs`, replace the brace
block with one line, `mod foo;` (a semicolon, *no* braces). In a
sibling file `foo.rs` next to `main.rs`, write the body.

`main.rs`:

```rust
mod foo;

fn main() {
    foo::hi();
}
```

`foo.rs`:

```rust
pub fn hi() {
    println!("hi from foo");
}
```

From the directory holding both files, `rustc main.rs` compiles
silently and the executable prints `hi from foo` — byte-identical to
lesson 096's inline output.

The single new piece is the source-layout rule: when `rustc` sees
`mod foo;` (semicolon, no body), it looks for the body in `foo.rs`
next to the declaring file. Everything else from lesson 096 carries
through unchanged — the privacy-by-default rule still applies, `pub`
still controls access, and the call site `foo::hi()` is still the
lesson-043 `module::name(args)` shape.

## Mental Model Delta

- *Before*: "I author a module with `mod foo { ... }`. Body in braces,
  same `.rs` file as `fn main`."
- *After*: "`mod foo;` (semicolon, no braces) is the *file-based* form
  of the same declaration. Body lives in `foo.rs` next to the declaring
  file. Same `mod` keyword, same module, same privacy rule, same
  `foo::item` access — only the location of the body changes."

## Prerequisites

- Installed concepts:
  - Lesson 096 (*load-bearing*): `mod foo { ... }`, `pub` on the inner
    function, the privacy-by-default rule, and the access form
    `foo::item`. Today extends 096 by *only* the source-layout step.
  - Lesson 002 (`fn main`), lesson 008 (define + call a function),
    lesson 011 (`println!` positional `{}`), lesson 043 (`module::name`
    call), and lesson 003 (diagnostic four-part map for reading E0583).
- Ordinary computer-use assumptions: same as lesson 001, plus the
  ability to create *two* files in one directory and run `rustc` from
  that directory.

## Try It

Save the two files in a fresh empty directory. The filename `foo.rs`
is what `rustc` matches against the `mod foo;` line.

```console
$ ls
foo.rs  main.rs
$ rustc main.rs
$ ./main
hi from foo
```

*Now the contrast — the centered teaching moment.* Rename `foo.rs` so
the file `rustc` is told to look for is no longer there:

```console
$ mv foo.rs foo.rs.bak
$ rustc main.rs
error[E0583]: file not found for module `foo`
 --> main.rs:1:1
  |
1 | mod foo;
  | ^^^^^^^^
  |
  = help: to create the module `foo`, create file "foo.rs" or "foo/mod.rs"
  = note: if there is a `mod foo` elsewhere in the crate already, import it with `use crate::...` instead

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0583`.
```

Read with the lesson 003 map. Headline `E0583`; location at the
declaring line (`main.rs:1:1`); caret under the whole `mod foo;`. The
`help:` block names exactly the rule today installs — candidate
filenames `foo.rs` (today) and `foo/mod.rs` (deferred). Rename back
and the program prints `hi from foo` again.

(Today unlocks reading the rmp target's `pub mod bigint;` line in
`lib.rs`, which loads `bigint.rs`. The same target's `pub mod biguint;`
loads `biguint/mod.rs` — the *subdirectory* form, deferred below.)

## What Changed

- `mod foo;` (semicolon, no body) tells `rustc` to load the module
  body from `foo.rs` adjacent to the declaring file. The Reference's
  `Module` grammar admits two shapes — lesson 096's brace form and
  today's semicolon form.
- Lesson 096's privacy rule, `pub` semantics, and `foo::item` access
  form carry through unchanged. The body of `foo.rs` is the same kind
  of content that would have gone inside the braces.
- New E-code in your collection: `E0583` ("file not found for module")
  when the expected file is missing. The diagnostic names the two
  candidate filenames — the source-layout rule, stated by the compiler.
- `rustc` does not "include" every file it sees. Only files named by
  some `mod` line are part of the build.

## Check Yourself

You set up two files. `app.rs` contains:

```rust
mod greet;

fn main() {
    greet::wave();
}
```

(a) What name must the second file have for `rustc app.rs` to succeed?

(b) What declaration, with which keyword in front, must that file
contain so the call compiles?

(c) If the file exists and is named correctly but `pub` is missing
from `wave`, which E-code fires — the same one as the file-missing
case, or a different one?

(*Answers: (a) `greet.rs`, in the same directory as `app.rs`. (b)
`pub fn wave() { ... }`; `pub` from lesson 096 makes `wave` reachable
from outside the module. (c) E0603 (lesson 096), not E0583. E0583
fires only when the file is missing; once the file is present, the
privacy check is the next gate.*)

## What To Ignore For Now

- *Subdirectory modules* — `mod foo;` also resolves to `foo/mod.rs` if
  no `foo.rs` is present. The rmp target's `pub mod biguint;` uses this
  form to spread one module across many files. Separate move.
- *Nested file-based modules* — a file-based module body containing
  its own `mod bar;` lines. The resolution rule for nested cases has
  its own subtleties.
- *`pub mod foo;`* — making the module declaration itself public,
  already deferred in 096.
- *The `#[path = "..."]` attribute* — overrides the default file-name
  rule. Exotic.
- *Edition-specific path-resolution differences* — Rust 2018 changed
  the rules from Rust 2015. This run uses edition 2024.
- *Cargo's package layout* — Cargo conventions (`src/main.rs`,
  `src/foo.rs`) wrap today's rule, but the file-name rule is `rustc`'s,
  not Cargo's. The probe uses bare `rustc main.rs` to keep the focus
  on the language rule.
- All previously deferred items from lesson 096.

## Evidence

See `../evidence/097-file-based-module.md` for the corpus-quote map,
the toolchain string, the working two-file probe transcript, the
captured E0583 contrast probe with the file removed, the auxiliary
E0603 probe witnessing that the lesson-096 privacy rule carries through
unchanged across the file boundary, and the prerequisite-claim summary.
