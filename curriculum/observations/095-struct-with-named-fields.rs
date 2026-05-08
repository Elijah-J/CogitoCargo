// Probe for lesson 095 — define a struct with named fields, construct
// an instance, and read a field.
//
// Three composed pieces in one program:
//
//   1. Declaration:   `struct Point { x: i32, y: i32 }`
//   2. Construction:  `Point { x: 3, y: 7 }`
//   3. Field access:  `p.x` and `p.y`
//
// Compile (silent, exit 0):
//
//     $ rustc demo.rs
//     $ ls
//     demo  demo.rs
//
// Run:
//
//     $ ./demo
//     p.x = 3, p.y = 7
//     (exit 0)
//
// The contrast probes in evidence/095-struct-with-named-fields.md drop a
// required field at construction (E0063 "missing field `y`") and read a
// nonexistent field on the instance (E0609 "no field `z` on type
// `Point`"). Both witness rules already implicit in this probe: every
// field must be supplied at construction time, and the field names are
// part of the type.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 7 };
    println!("p.x = {}, p.y = {}", p.x, p.y);
}
