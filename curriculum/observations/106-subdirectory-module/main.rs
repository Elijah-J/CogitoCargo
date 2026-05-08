// Probe for lesson 106 — subdirectory module declaration.
//
// `mod foo;` (semicolon, no braces) at the top of `main.rs` tells `rustc`
// to find the module body in EITHER `foo.rs` (lesson 097) OR
// `foo/mod.rs` (today). This probe uses the second form: there is no
// `foo.rs` next to `main.rs`; instead a sibling subdirectory `foo/`
// contains `mod.rs` with the body. From rustc's point of view the two
// resolutions are alternatives of the same rule.
//
// Reproduce the working probe from this directory:
//
//     $ rustc main.rs
//     $ ./main
//     hi from foo/mod.rs
//
// Contrast probe (both forms present): create a sibling `foo.rs` next
// to `main.rs` while keeping `foo/mod.rs` in place, then rerun
// `rustc main.rs`. rustc fires
// `error[E0761]: file for module \`foo\` found at both "foo.rs" and "foo/mod.rs"`
// with a help block reading `delete or rename one of them to remove the
// ambiguity`. The exact captured transcript is in
// evidence/106-subdirectory-module.md.
//
// (The neither-present case is lesson 097's E0583 contrast and is not
// re-witnessed here.)

mod foo;

fn main() {
    foo::hi();
}
