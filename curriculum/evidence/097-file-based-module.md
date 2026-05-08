# Evidence — 097-file-based-module

This appendix grounds lesson 097's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` -> `Darwin x86_64`
- Probes run from a `mktemp -d` directory on this host. Same toolchain
  family as recent accepted lessons (082-096).

The committed observation files at
`experimental/eduratchet2/runs/rust-moves/observations/097-file-based-module/`
(directory containing `main.rs` and `foo.rs`) are the working two-file
probe verbatim. The directory layout is itself part of the probe — the
file-name rule today's lesson installs depends on `foo.rs` being
adjacent to `main.rs`.

## Sources

### `output/docs/rust/book/ch07-05-separating-modules-into-different-files.md`

The Book's *Separating Modules into Different Files* chapter. Two
load-bearing passages.

#### Lines 14-30 — the file-based form replaces the brace block

> First, we'll extract the `front_of_house` module to its own file.
> Remove the code inside the curly brackets for the `front_of_house`
> module, leaving only the `mod front_of_house;` declaration, so that
> *src/lib.rs* contains the code shown in Listing 7-21. Note that this
> won't compile until we create the *src/front_of_house.rs* file in
> Listing 7-22.
>
> Filename: src/lib.rs
>
> ```rust
> mod front_of_house;
>
> pub use crate::front_of_house::hosting;
>
> pub fn eat_at_restaurant() {
>     hosting::add_to_waitlist();
> }
> ```

Corpus warrant for the lesson's centered claim: `mod NAME;` (semicolon,
no body) is the form that loads the module body from a separate file
named `NAME.rs`. The Book's example uses `mod front_of_house;` /
`front_of_house.rs`; today's probe uses `mod foo;` / `foo.rs` for
audience clarity.

#### Lines 34-37 — the file-name rule

> Next, place the code that was in the curly brackets into a new file
> named *src/front_of_house.rs*, as shown in Listing 7-22. The compiler
> knows to look in this file because it came across the module
> declaration in the crate root with the name `front_of_house`.

Corpus warrant for the lesson's "the body lives in `foo.rs` adjacent to
the declaring file" framing. The Book makes the rule explicit: the
file's name matches the name in the `mod` declaration.

#### Lines 49-56 — `mod` is not "include"

> Note that you only need to load a file using a `mod` declaration
> *once* in your module tree. Once the compiler knows the file is part
> of the project (and knows where in the module tree the code resides
> because of where you've put the `mod` statement), other files in
> your project should refer to the loaded file's code using a path to
> where it was declared, as covered in the ["Paths for Referring to an
> Item in the Module Tree"]... section. In other words, `mod` is *not*
> an "include" operation that you may have seen in other programming
> languages.

Corpus warrant for the lesson's "files no `mod` line ever names are
not part of the build" claim in *What Changed*.

#### Lines 87-105 — alternate file paths

> So far we've covered the most idiomatic file paths the Rust compiler
> uses, but Rust also supports an older style of file path. For a
> module named `front_of_house` declared in the crate root, the
> compiler will look for the module's code in:
>
> - *src/front_of_house.rs* (what we covered)
> - *src/front_of_house/mod.rs* (older style, still supported path)

Corpus warrant for the lesson's deferral of the subdirectory form. The
Book confirms both `foo.rs` and `foo/mod.rs` are admitted candidates.
Today centers only the `foo.rs` form; the `foo/mod.rs` form (used by
the rmp target's `biguint/`) is deferred.

### `output/docs/rust/reference/items/modules.md`

The Reference's formal grammar.

#### Lines 9-15 — the two `Module` shapes

> [Module] -> <br>
>       unsafe? mod IDENTIFIER ; <br>
>     | unsafe? mod IDENTIFIER { <br>
>         InnerAttribute\* <br>
>         Item\* <br>
>       }

Corpus warrant for the lesson's framing "the Reference's `Module`
grammar admits two shapes" in *What Changed*. Lesson 096 installed the
brace form; today installs the semicolon form. They are the same
grammar rule's two alternatives.

#### Lines 70-81 — the file-resolution rule, formally

> A module without a body is loaded from an external file. When the
> module does not have a `path` attribute, the path to the file mirrors
> the logical [module path](../paths.md).
>
> Ancestor module path components are directories, and the module's
> contents are in a file with the name of the module plus the `.rs`
> extension.

Reference-level statement of the rule. The Book chapter quoted above
is the friendlier form; this is the formal one.

#### Lines 82-88 — the `mod.rs` alternative

> Module filenames may also be the name of the module as a directory
> with the contents in a file named `mod.rs` within that directory.
> The above example can alternately be expressed with `crate::util`'s
> contents in a file named `util/mod.rs`. It is not allowed to have
> both `util.rs` and `util/mod.rs`.

Reference-level confirmation that the subdirectory `foo/mod.rs` form
is an admitted alternative — explicitly deferred today.

### `output/docs/rust/error_codes/E0583.md`

The verbatim corpus warrant for the contrast probe.

> A file wasn't found for an out-of-line module.
>
> Erroneous code example:
>
> ```rust
> mod file_that_doesnt_exist; // error: file not found for module
>
> fn main() {}
> ```
>
> Please be sure that a file corresponding to the module exists. If
> you want to use a module named `file_that_doesnt_exist`, you need to
> have a file named `file_that_doesnt_exist.rs` or
> `file_that_doesnt_exist/mod.rs` in the same directory.

The corpus example shape (`mod NAME;` with no `NAME.rs` file present)
matches the lesson's contrast probe exactly. The corpus's prose
matches the rustc 1.95.0 `help:` block: "create file `NAME.rs` or
`NAME/mod.rs`".

## Probes

### Probe 1 (working) — both files present

Captured 2026-05-08 from a `mktemp -d` directory.

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

Transcript:

```
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ ls
foo.rs  main.rs
$ rustc main.rs
$ echo "exit: $?"
exit: 0
$ ls
foo  foo.rs  main  main.rs
$ ./main
hi from foo
$ echo "exit: $?"
exit: 0
```

Result: `rustc` builds silently. The executable is named `main` (the
stem of `main.rs`, the file passed to `rustc`). Output is byte-
identical to lesson 096's inline-form probe.

(The committed observation files at
`observations/097-file-based-module/` are this probe's source. The
build artefact `main` is not committed.)

### Probe 2 (contrast) — `foo.rs` missing

Captured 2026-05-08 from the same directory after `mv foo.rs
foo.rs.bak`.

```
$ ls
foo.rs.bak  main.rs
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
$ echo "exit: $?"
exit: 1
```

This is the lesson's centered contrast. Diagnostic shape, applied
through the lesson 003 four-part map:

- *Headline*: `error[E0583]: file not found for module \`foo\``. New
  E-code in the run's collection.
- *Location*: `--> main.rs:1:1`. The location is the declaring line,
  not a use site (lesson 096's E0603 located at the use site).
- *Source excerpt with caret*: line 1 (`mod foo;`), caret under the
  whole declaration (eight `^` characters).
- *Help/note lines*: the `help:` block names the two candidate
  filenames `foo.rs` (today's form) and `foo/mod.rs` (deferred). The
  `note:` block names a sibling case (`mod foo` elsewhere in the
  crate) that the lesson does not exercise.
- *Trailer*: `error: aborting due to 1 previous error` and the
  `--explain` pointer (lesson 069's category map applies).

### Probe 3 (auxiliary) — privacy rule carries through

Captured 2026-05-08 from the same directory with `foo.rs` restored but
`pub` deleted from `fn hi`.

`foo.rs` (modified):

```rust
fn hi() {
    println!("hi from foo");
}
```

Transcript:

```
$ rustc main.rs
error[E0603]: function `hi` is private
 --> main.rs:4:10
  |
4 |     foo::hi();
  |          ^^ private function
  |
note: the function `hi` is defined here
 --> foo.rs:1:1
  |
1 | fn hi() {
  | ^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
$ echo "exit: $?"
exit: 1
```

This is the lesson 096 privacy diagnostic, now witnessed across two
files. The `note:` block's second `-->` line points at `foo.rs:1:1`
rather than back into the same source file — concrete evidence for
the lesson's *What Changed* claim "lesson 096's privacy rule, `pub`
semantics, and `foo::item` access form carry through unchanged". The
file boundary changes nothing about the rule.

This probe is not the lesson's centered contrast (E0583 is). It is
included here to ground the carry-through claim, not to install a new
rule. Lesson 096 already installed E0603.

## Claim → evidence mapping

| Lesson claim | Source |
|---|---|
| `mod foo;` (semicolon, no body) loads the body from `foo.rs` adjacent to the declaring file | Book ch07-05 lines 14-30, 34-37; Reference items/modules.md lines 70-81 |
| The Reference's `Module` grammar admits two shapes (semicolon and brace) | Reference items/modules.md lines 9-15 |
| Privacy rule, `pub`, and `foo::item` access form from lesson 096 carry through unchanged | Probe 3 (E0603 across two files); lesson 096 evidence appendix; the file-resolution rule does not interact with the visibility rule per Book ch07-05's silence on the matter |
| E0583 fires when the expected file is missing | Probe 2 (verbatim); error_codes/E0583.md |
| The `help:` block names two candidate filenames `foo.rs` and `foo/mod.rs` | Probe 2 (verbatim); error_codes/E0583.md |
| The diagnostic locates at the declaring line, not a use site | Probe 2 (verbatim `--> main.rs:1:1`) |
| `rustc` does not "include" every file it sees; only files named by `mod` lines are part of the build | Book ch07-05 lines 49-56 ("`mod` is *not* an `include` operation") |
| The committed observation directory layout reproduces the published transcript | Observation files at `observations/097-file-based-module/` rebuilt 2026-05-08; transcript matches Probe 1 |

## Direct prerequisite — lesson 096 (load-bearing)

Lesson 096 installed:

- `mod foo { ... }` as the inline (brace) form of the `Module` grammar
  rule. Today extends this by introducing the semicolon form of the
  *same* rule.
- `pub` before an item declaration as the keyword that exposes the
  item across the module boundary. Today's probe re-uses `pub fn hi`
  unchanged.
- The privacy-by-default rule and the access form `module::item`.
  Probe 3 above witnesses both, unchanged across the file boundary.

Today's lesson does not re-install any of these. The body of `foo.rs`
is the same kind of content that would have gone inside the braces of
`mod foo { ... }` in lesson 096.

## Older supporting lessons (cited, not re-installed)

- Lesson 002 (`fn main`): the host block in `main.rs`.
- Lesson 008 (define + call a function): `pub fn hi() { ... }` and the
  call.
- Lesson 011 (`println!` positional `{}`): the body of `hi`.
- Lesson 043 (nested module paths): the call form `foo::hi()` is
  `module::name(args)` from 043 with the user's own module on the left
  of `::`. Lesson 096 already used this; today's call site is
  unchanged.
- Lesson 003 (rustc diagnostic four-part map): used to read E0583
  in Probe 2 and E0603 in Probe 3.
- Lesson 069 (rustc warnings): named only — Probe 2's
  `error: aborting due to 1 previous error` trailer is read with
  lesson 069's category map (error vs warning).

No claim on this list is load-bearing in a way that lesson 096 has not
already restated.

## Toolchain notes

- `rustc main.rs` is the right tool for today's probe — bare `rustc`
  is sufficient because the file-resolution rule is `rustc`'s own,
  not Cargo's. Using a Cargo package would conflate today's rule with
  the package layout convention.
- The probe was rerun from
  `experimental/eduratchet2/runs/rust-moves/observations/097-file-based-module/`
  before commit; the published `main.rs` and `foo.rs` reproduce
  Probe 1's output. The build artefact `main` is removed before
  commit (no `target/` directory exists — `rustc` produces the
  executable in the working directory).

## Observation directory convention

This is the run's first multi-file probe outside Cargo packages.
Decision: `observations/097-file-based-module/` directory containing
`main.rs` and `foo.rs`. Rationale:

1. The directory layout is part of the rule. Two files at the
   observations root with disambiguating prefixes (e.g.,
   `097-...-main.rs` / `097-...-foo.rs`) would lie about the layout —
   `rustc`'s rule depends on the *exact* filename `foo.rs` and the
   adjacency.
2. The directory pattern is already used for the Cargo-package probes
   (064/065/066/067/082/083/084/087); reusing it keeps the convention
   uniform.
3. A learner replaying the probe by `cd`ing into the directory and
   running `rustc main.rs` reproduces Probe 1's transcript exactly.
