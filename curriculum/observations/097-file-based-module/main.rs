// Probe for lesson 097 — file-based module declaration.
//
// `mod foo;` (semicolon, no braces) at the top of `main.rs` tells `rustc`
// to find the module body in `foo.rs` adjacent to this file. The body of
// `foo.rs` is the same kind of content that would have gone inside
// `mod foo { ... }` (lesson 096).
//
// Reproduce the working probe from this directory:
//
//     $ rustc main.rs
//     $ ./main
//     hi from foo
//
// Contrast probe (file missing): rename or delete foo.rs and rerun
// `rustc main.rs`. rustc fires
// `error[E0583]: file not found for module \`foo\`` with a help block
// proposing the candidate filenames `foo.rs` or `foo/mod.rs`. The exact
// captured transcript is in evidence/097-file-based-module.md.
//
// Privacy-rule carry-through probe: keep both files, but delete `pub`
// from `fn hi` in `foo.rs`. rustc fires E0603 (the lesson-096 diagnostic)
// with the use site in `main.rs` and the definition site in `foo.rs` —
// the same privacy rule, now witnessed across two files.

mod foo;

fn main() {
    foo::hi();
}
