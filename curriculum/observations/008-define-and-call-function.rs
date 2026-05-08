// EduRatchet-2 lesson 008: define a second function and call it from main.
//
// Working program. Defines `say_hi` below `main` and calls it once from
// inside `main` between two `println!` statements. Compile with
// `rustc 008-define-and-call-function.rs` and run the produced executable
// to print three lines in order: `from main`, `from say_hi`,
// `from main again`. Exits 0.
//
// The broken contrast (call without definition) is the same file with
// the `fn say_hi() { ... }` block deleted; that version is documented in
// the lesson's `## Evidence` section, not committed here.

fn main() {
    println!("from main");
    say_hi();
    println!("from main again");
}

fn say_hi() {
    println!("from say_hi");
}
