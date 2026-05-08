---
id: 106-subdirectory-module
status: accepted
evidence: ../evidence/106-subdirectory-module.md
---

# Put a module's body in a subdirectory's `mod.rs`

## The Move

Lesson 097 wrote `mod foo;` and put the body in a sibling file
`foo.rs`. Today, the same `mod foo;` line gets the *other* file path
the Reference admits: a sibling *directory* `foo/` containing `mod.rs`.

Layout for today's working probe:

```
demo_dir/
├── main.rs
└── foo/
    └── mod.rs
```

`main.rs` (unchanged from lesson 097's shape):

```rust
mod foo;

fn main() {
    foo::hi();
}
```

`foo/mod.rs`:

```rust
pub fn hi() {
    println!("hi from foo/mod.rs");
}
```

From `demo_dir/`, `rustc main.rs` compiles silently and `./main` prints
`hi from foo/mod.rs`. The single new piece: when `rustc` reads
`mod foo;`, it looks for the body at *either* `foo.rs` *or*
`foo/mod.rs` — two candidate file paths for one declaration. Lesson
097 covered the first; today covers the second.

The Reference at `items/modules.md` states the rule: "Module filenames
may also be the name of the module as a directory with the contents in
a file named `mod.rs` within that directory. ... It is not allowed to
have both."

## Mental Model Delta

- *Before*: "`mod foo;` looks for the body in `foo.rs` next to the
  declaring file." (Lesson 097.)
- *After*: "`mod foo;` looks for the body in *either* `foo.rs` *next to*
  the declaring file *or* `foo/mod.rs` *inside* a sibling directory
  `foo/`. The two are alternatives — same module, same `mod` keyword,
  same privacy rule, same `foo::item` access form, only the chosen file
  path differs. The two forms cannot both exist; rustc rejects that
  with E0761."

## Prerequisites

- Installed concepts:
  - Lesson 097 (*load-bearing*): `mod foo;` (semicolon, no body) loads
    the module body from a separate file. Today extends 097 by *only*
    naming the second admitted file path.
  - Lesson 096: inline modules and `pub` on items. Today's `pub fn hi`
    is unchanged from 096/097.
  - Lesson 003: the diagnostic four-part map applied to the new
    E-code E0761.
- Ordinary computer-use assumptions: same as lesson 097, plus the
  ability to make a *subdirectory* and place a file inside it.

## Try It

Make an empty directory. Inside it, create `main.rs` with the contents
above. Then make a subdirectory `foo/` and place `mod.rs` inside `foo/`
with the `pub fn hi` body.

```console
$ ls
foo  main.rs
$ ls foo
mod.rs
$ rustc main.rs
$ ./main
hi from foo/mod.rs
```

*Now the contrast — the centered teaching moment.* Add a sibling
`foo.rs` next to `main.rs` (same content, different message), so that
both candidate locations are populated at once:

```console
$ ls
foo  foo.rs  main.rs
$ rustc main.rs
error[E0761]: file for module `foo` found at both "foo.rs" and "foo/mod.rs"
 --> main.rs:1:1
  |
1 | mod foo;
  | ^^^^^^^^
  |
  = help: delete or rename one of them to remove the ambiguity

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0761`.
```

Read with the lesson 003 map. The headline message literally names the
two admitted candidate paths (`"foo.rs"` and `"foo/mod.rs"`). Location
at the declaring line (`main.rs:1:1`); caret under the whole
`mod foo;`. The `help:` block names the operational fix — pick one,
delete or rename the other. The diagnostic itself is the rule
statement: rustc has two places it might look, and finding something
at both is ambiguous. Delete `foo.rs` and the program prints
`hi from foo/mod.rs` again.

(The other contrast — *neither* file present — is lesson 097's E0583,
already installed. The `help:` block on E0583 reads "create file
`foo.rs` or `foo/mod.rs`": that *or* is exactly the rule today centers.
Today unlocks reading the rmp target's `pub mod biguint;` line, which
loads `biguint/mod.rs`.)

## What Changed

- A second admitted file path. `mod foo;` resolves to `foo.rs` *or*
  `foo/mod.rs`. Either form is a valid body for the same module.
- The two forms are mutually exclusive. With both populated, `rustc`
  fires E0761 ("file for module `foo` found at both ...") and refuses
  to build.
- Everything else carries through unchanged from lesson 097: the `mod`
  keyword, the privacy rule, `pub`'s effect, and the `foo::item` access
  form. Only the location of the body changes.
- The subdirectory form is the standard way to spread one conceptual
  module across many files. `mod.rs` holds the parent, and `foo/`
  becomes a folder for siblings (`foo/add.rs`, `foo/cmp.rs`, ...) that
  the parent declares with further `mod NAME;` lines.

## Check Yourself

You set up a directory with `app.rs` containing
`mod widget; fn main() { widget::draw(); }` and a sibling
`widget/mod.rs` containing `pub fn draw() { println!("drew a widget"); }`.

(a) What does `rustc app.rs && ./app` print?

(b) You also add `widget.rs` (next to `app.rs`) with the same
    `pub fn draw()` body, leaving `widget/mod.rs` in place. Which
    E-code fires now?

(c) You instead delete `widget/mod.rs` (and there is no sibling
    `widget.rs`). Which E-code fires? *(Hint: lesson 097.)*

(*Answers: (a) `drew a widget` — `mod widget;` resolves to
`widget/mod.rs` because no `widget.rs` is present. (b) E0761, "file for
module `widget` found at both `widget.rs` and `widget/mod.rs`" — both
candidate paths are populated, the ambiguity fires. (c) E0583,
file-not-found from lesson 097; the `help:` block names both candidate
filenames because rustc tried and failed at both.*)

## What To Ignore For Now

- *Multi-level subdirectory module trees* — `foo/bar/mod.rs` requires
  a `mod bar;` line *inside* `foo/mod.rs`. The same rule applied
  recursively; future move.
- *The 2018+ `foo.rs` plus sibling `foo/` form* — `mod foo;` may
  resolve to `foo.rs` while submodules live under `foo/` (no `mod.rs`
  on the parent), the modern Cargo convention. Book Ch7-5 walks it.
  The rmp target uses today's older `foo/mod.rs` style.
- *The `#[path = "..."]` attribute* — already deferred in lesson 097.
- *Edition-specific differences* — already deferred in lesson 097.
- *Cargo's `src/lib.rs` and `src/main.rs`* — special crate-root files,
  not `mod.rs` files. The `mod.rs` rule is for *non-root* modules.
- All previously deferred items from lessons 096 and 097.

## Evidence

See `../evidence/106-subdirectory-module.md`.
