// Probe for lesson 012-bool-literals.
//
// Working program: bind a name to each of the two boolean literals
// `true` and `false`, then print each bound value with a named
// placeholder. Expected output is two lines:
//
//   yes = true
//   no = false
//
// Compile and run as in lesson 001:
//
//   $ rustc 012-bool-literals.rs
//   $ ./012-bool-literals
//
// (In the captured transcript the file is named `bools.rs`; the source
// is identical.)

fn main() {
    let yes = true;
    let no = false;
    println!("yes = {yes}");
    println!("no = {no}");
}
