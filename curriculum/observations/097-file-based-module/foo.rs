// Companion file for the working probe of lesson 097.
//
// `mod foo;` in main.rs (this directory's neighbour) makes `rustc` look
// for the module body here. The contents below are exactly what would
// have lived inside `mod foo { ... }` in lesson 096's inline form.

pub fn hi() {
    println!("hi from foo");
}
