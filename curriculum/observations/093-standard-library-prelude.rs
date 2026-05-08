// Probe for lesson 093 — the standard library prelude.
//
// Witnesses that `String`, `Vec`, `Result`/`Ok`, and `Option`/`Some` are
// all reachable as bare names with no `use` declaration and no full path.
// The collection of names automatically in scope of every Rust module is
// called the *standard library prelude* (Reference, names/preludes.md).
//
// Compile and run:
//
//     $ rustc 093-standard-library-prelude.rs
//     $ ./093-standard-library-prelude
//     s = ""
//     v = []
//     r = Ok(42)
//     opt = Some(7)
//
// The contrast probe is `let m = HashMap::new();` in a separate file.
// `HashMap` is *not* in the prelude, so the same shape fires
// `error[E0433]: cannot find type \`HashMap\` in this scope` with a
// `help:` line literally suggesting `use std::collections::HashMap;`.
// Full transcript in evidence/093-standard-library-prelude.md.

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
