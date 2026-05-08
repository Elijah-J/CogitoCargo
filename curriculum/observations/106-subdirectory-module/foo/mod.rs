// Companion file for the working probe of lesson 106.
//
// `mod foo;` in `../main.rs` (one directory up) makes `rustc` look for
// the module body here, in `foo/mod.rs`. The body below is the same
// kind of content that would have lived in `foo.rs` (lesson 097's
// adjacent-file form) or inside `mod foo { ... }` (lesson 096's
// inline form). The Reference's items/modules.md states the rule
// directly: "Module filenames may also be the name of the module as
// a directory with the contents in a file named `mod.rs` within that
// directory."

pub fn hi() {
    println!("hi from foo/mod.rs");
}
