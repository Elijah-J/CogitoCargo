// EduRatchet-2 lesson 071: macro invocation syntax `name!(...)` is
// syntactically distinct from function-call syntax `name(...)`.
//
// Working program. Defines `greet` (a function, lesson 008's shape) below
// `main` and calls it from `main` two ways the lesson contrasts:
//   1. `greet();`           — function-call form (lesson 008).
//   2. `println!("...");`   — macro-invocation form (already in operational
//                              use since lesson 001; today named).
// Compile with `rustc 071-macro-invocation-syntax.rs` and run the
// produced executable to print two lines: `hi` then `from a macro`.
// Exits 0.
//
// The two broken contrasts (`greet!();` with the bang on a function, and
// `println("...")` without the bang on a macro) are documented as separate
// runs inside the lesson's evidence appendix, not committed here.

fn greet() {
    println!("hi");
}

fn main() {
    greet();
    println!("from a macro");
}
