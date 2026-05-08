# Evidence — 106-subdirectory-module

This appendix grounds lesson 106's substantive claims. The
learner-facing lesson keeps only a pointer here.

## Toolchain

- `rustc --version` -> `rustc 1.95.0 (59807616e 2026-04-14)`
- `uname -sm` -> `Darwin x86_64`
- Probes run from a `mktemp -d` directory on this host. Same toolchain
  as recent accepted lessons.

The committed observation files at
`experimental/eduratchet2/runs/rust-moves/observations/106-subdirectory-module/`
(directory containing `main.rs` and `foo/mod.rs`) are the working
two-file probe verbatim. The directory layout is itself part of the
probe — the file-name rule today's lesson installs depends on the
*subdirectory* `foo/` containing a file named exactly `mod.rs`.

## Sources

### `output/docs/rust/reference/items/modules.md`

The Reference's formal grammar and file-resolution rules.

#### Lines 9-15 — the two `Module` shapes

> [Module] -> <br>
>       unsafe? mod IDENTIFIER ; <br>
>     | unsafe? mod IDENTIFIER { <br>
>         InnerAttribute\* <br>
>         Item\* <br>
>       }

Lesson 097 is the semicolon form's first admitted file path; today
adds the second. The grammar rule itself is unchanged from 096/097.

#### Lines 70-81 — the `foo.rs` rule (lesson 097's load-bearing claim)

> A module without a body is loaded from an external file. When the
> module does not have a `path` attribute, the path to the file mirrors
> the logical [module path](../paths.md).
>
> Ancestor module path components are directories, and the module's
> contents are in a file with the name of the module plus the `.rs`
> extension.

This is lesson 097's installed rule. Today builds on it.

#### Lines 82-88 — the `foo/mod.rs` alternative (today's centered claim)

> Module filenames may also be the name of the module as a directory
> with the contents in a file named `mod.rs` within that directory.
> The above example can alternately be expressed with `crate::util`'s
> contents in a file named `util/mod.rs`. **It is not allowed to have
> both `util.rs` and `util/mod.rs`.**

Corpus warrant for both load-bearing claims today:

1. The `foo/mod.rs` form is the second admitted file path for
   `mod foo;`. ("Module filenames may also be the name of the module
   as a directory with the contents in a file named `mod.rs`.")
2. The two forms are mutually exclusive. ("It is not allowed to have
   both.")

#### Lines 86-88 — Note on legacy convention

> Note: Prior to `rustc` 1.30, using `mod.rs` files was the way to load
> a module with nested children. It is encouraged to use the new
> naming convention as it is more consistent, and avoids having many
> files named `mod.rs` within a project.

Corpus warrant for the deferral list bullet noting the 2018+
parent-without-`mod.rs` form. The Reference notes this is the
encouraged modern convention; today centers the older form because the
rmp target uses it.

### `output/docs/rust/book/ch07-05-separating-modules-into-different-files.md`

Friendly version of the same rule.

#### Lines 87-105 — alternate file paths

> So far we've covered the most idiomatic file paths the Rust compiler
> uses, but Rust also supports an older style of file path. For a
> module named `front_of_house` declared in the crate root, the
> compiler will look for the module's code in:
>
> - *src/front_of_house.rs* (what we covered)
> - *src/front_of_house/mod.rs* (older style, still supported path)
>
> [...]
>
> If you use both styles for the same module, you'll get a compiler
> error. Using a mix of both styles for different modules in the same
> project is allowed but might be confusing for people navigating
> your project.

Corpus warrant — second source — for the same two claims: both forms
are admitted, and using both at once is a compile error. The Book's
"compiler error" is exactly E0761 captured in Probe 2 below.

### `output/docs/rust/error_codes/E0761.md`

The verbatim corpus warrant for today's centered contrast probe.

> Multiple candidate files were found for an out-of-line module.
>
> Erroneous code example:
>
> ```rust
> // file: ambiguous_module/mod.rs
>
> fn foo() {}
>
> // file: ambiguous_module.rs
>
> fn foo() {}
>
> // file: lib.rs
>
> mod ambiguous_module; // error: file for module `ambiguous_module`
>                       // found at both ambiguous_module.rs and
>                       // ambiguous_module/mod.rs
> ```
>
> Please remove this ambiguity by deleting/renaming one of the
> candidate files.

The corpus example shape (`mod NAME;` with both `NAME.rs` and
`NAME/mod.rs` populated) matches today's contrast probe exactly. The
corpus's "deleting/renaming one of the candidate files" matches the
rustc 1.95.0 `help:` block "delete or rename one of them to remove the
ambiguity" verbatim modulo phrasing.

### `output/docs/rust/error_codes/E0583.md`

Cited only — the E0583 case (neither file present) was lesson 097's
contrast and is referenced in today's lesson body and Check Yourself
question (c). Today does not re-witness it; lesson 097 already did.

The relevant passage from lesson 097's evidence appendix:

> If you want to use a module named `file_that_doesnt_exist`, you need
> to have a file named `file_that_doesnt_exist.rs` or
> `file_that_doesnt_exist/mod.rs` in the same directory.

This *or* is the same rule today centers — E0583's `help:` block names
*both* candidate filenames precisely because the file-resolution rule
admits two forms.

## Probes

### Probe 1 (working) — subdirectory form

Captured 2026-05-08 from a `mktemp -d` directory.

`main.rs`:

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

Transcript:

```
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ ls
foo  main.rs
$ ls foo
mod.rs
$ rustc main.rs
$ echo "exit: $?"
exit: 0
$ ls
foo  main  main.rs
$ ./main
hi from foo/mod.rs
$ echo "exit: $?"
exit: 0
```

Result: `rustc` builds silently. The executable is named `main` (the
stem of `main.rs`, the file passed to `rustc`). The body of `foo`
resolves to `foo/mod.rs` rather than `foo.rs` because no `foo.rs`
exists.

(The committed observation files at
`observations/106-subdirectory-module/main.rs` and
`observations/106-subdirectory-module/foo/mod.rs` are this probe's
source. The build artefact `main` is removed before commit.)

### Probe 2 (centered contrast) — both forms populated

Captured 2026-05-08 from a fresh `mktemp -d` directory. Same
`main.rs` as Probe 1; same `foo/mod.rs` as Probe 1; *plus* a sibling
`foo.rs` next to `main.rs`:

`foo.rs` (added):

```rust
pub fn hi() {
    println!("hi from foo.rs");
}
```

Transcript:

```
$ ls
foo  foo.rs  main.rs
$ ls foo
mod.rs
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
$ echo "exit: $?"
exit: 1
```

This is the lesson's centered contrast. Diagnostic shape, applied
through the lesson 003 four-part map:

- *Headline*: `error[E0761]: file for module \`foo\` found at both
  "foo.rs" and "foo/mod.rs"`. New E-code in the run's collection. The
  message itself names the two admitted candidate paths.
- *Location*: `--> main.rs:1:1`. Same shape as lesson 097's E0583 —
  the location is the declaring line, not a use site.
- *Source excerpt with caret*: line 1 (`mod foo;`), caret under the
  whole declaration (eight `^` characters).
- *Help/note lines*: the `help:` block names the operational fix —
  "delete or rename one of them to remove the ambiguity". The exact
  phrasing matches the corpus's "deleting/renaming one of the
  candidate files".
- *Trailer*: `error: aborting due to 1 previous error` and the
  `--explain E0761` pointer (lesson 069's category map applies).

The diagnostic statement *is* the rule statement: rustc has two places
it might look, and finding something at both is ambiguous.

### Probe 3 (cited only) — neither file present

Lesson 097's Probe 2 captured this case verbatim — `mod foo;` with no
`foo.rs` and no `foo/mod.rs` fires E0583 with `help:` block "to create
the module `foo`, create file `foo.rs` or `foo/mod.rs`". The *or* in
the corpus and rustc help blocks is the same rule today centers.

Today does not re-capture this case. Lesson 097's evidence appendix
already records it.

## Claim → evidence mapping

| Lesson claim | Source |
|---|---|
| `mod foo;` resolves to `foo/mod.rs` when no `foo.rs` exists (subdirectory form is admitted) | Reference items/modules.md lines 82-88; Book ch07-05 lines 87-95; Probe 1 (verbatim) |
| The two forms (`foo.rs` and `foo/mod.rs`) are alternatives — same module, same `mod` keyword | Reference items/modules.md lines 9-15 (one grammar rule); Reference items/modules.md lines 82-88 ("alternately be expressed"); Book ch07-05 lines 87-95 (single bulleted list of two paths) |
| The two forms cannot both exist; rustc emits E0761 | Reference items/modules.md line 84 ("It is not allowed to have both"); Book ch07-05 lines 103-105 ("If you use both styles for the same module, you'll get a compiler error"); Probe 2 (verbatim); error_codes/E0761.md |
| The privacy rule, `pub`, and `foo::item` access form carry through unchanged | Lesson 097's evidence appendix (load-bearing prerequisite); the body of `foo/mod.rs` in Probe 1 is `pub fn hi()` unchanged from lesson 097's `foo.rs`; the call site `foo::hi()` is unchanged |
| The diagnostic locates at the declaring line, not a use site | Probe 2 (verbatim `--> main.rs:1:1`); same shape as lesson 097's E0583 |
| The E0761 headline message names both candidate paths verbatim | Probe 2 (verbatim) |
| The `help:` block proposes deleting or renaming one of the two files | Probe 2 (verbatim); error_codes/E0761.md ("Please remove this ambiguity by deleting/renaming one of the candidate files") |
| The neither-present case fires E0583 with `help:` naming both candidate paths | Lesson 097's evidence Probe 2 (the *or* in the help block witnesses today's rule retroactively); error_codes/E0583.md |
| The subdirectory form is the way to spread one conceptual module across many files | Reference items/modules.md lines 86-88 (note on legacy convention); rmp target uses `biguint/mod.rs` plus siblings `add.rs`, `basic.rs`, etc. |
| The committed observation directory layout reproduces the published transcript | Observation files at `observations/106-subdirectory-module/` rebuilt 2026-05-08; transcript matches Probe 1 |

## Direct prerequisite — lesson 097 (load-bearing)

Lesson 097 installed:

- `mod foo;` (semicolon, no body) as the file-based form of the
  `Module` grammar rule. Today extends this by *only* naming the
  second admitted file path.
- The semicolon form loads the body from a separate file. Today's
  `foo/mod.rs` is "a separate file" — just one not adjacent to the
  declaring source.
- The privacy rule, `pub`'s effect, and the `foo::item` access form
  carry through across the file boundary; lesson 097's Probe 3
  witnessed E0603 across two files. Today's body in `foo/mod.rs` uses
  `pub fn hi` unchanged; no new privacy claim is made.

Today's lesson does not re-install any of these. The body of
`foo/mod.rs` is the same kind of content that would have gone in
`foo.rs` in lesson 097.

## Older supporting lessons (cited, not re-installed)

- Lesson 096 (inline modules + `pub`): named in the Mental Model
  Delta and Prerequisites; the `pub fn hi` body comes from 096/097
  unchanged.
- Lesson 002 (`fn main`): the host block in `main.rs`.
- Lesson 008 (define + call a function): `pub fn hi() { ... }` and
  the call.
- Lesson 011 (`println!` positional `{}`): the body of `hi`.
- Lesson 043 (nested module paths): the call form `foo::hi()`.
- Lesson 003 (rustc diagnostic four-part map): used to read E0761 in
  Probe 2.
- Lesson 069 (rustc warnings): named only — Probe 2's
  `error: aborting due to 1 previous error` trailer is read with
  lesson 069's category map.
- Lesson 001 (`rustc` compile and run): the build command.

No claim on this list is load-bearing in a way that lesson 097 has
not already restated.

## Toolchain notes

- `rustc main.rs` is the right tool for today's probe — bare `rustc`
  is sufficient because the file-resolution rule is `rustc`'s own,
  not Cargo's. Same rationale as lesson 097.
- The probe was rerun from
  `experimental/eduratchet2/runs/rust-moves/observations/106-subdirectory-module/`
  before commit; the published `main.rs` and `foo/mod.rs` reproduce
  Probe 1's output. The build artefact `main` is removed before
  commit (no `target/` directory exists — `rustc` produces the
  executable in the working directory).

## Observation directory convention

Same convention as lesson 097, extended by one level:
`observations/106-subdirectory-module/main.rs` and
`observations/106-subdirectory-module/foo/mod.rs`. The directory
layout is part of the rule. Renaming or flattening the layout would
lie about what `rustc`'s rule depends on.
