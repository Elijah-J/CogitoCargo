// Probe for lesson 007-shadowing.
//
// Move: rebind an existing name with a second `let name = ...;` to
// shadow the old binding.
//
// This is the working program. It compiles cleanly and prints:
//   first: x = 5
//   second: x = 10
//
// The first `let x = 5;` binds x to 5. The first println! sees that
// binding. The second `let x = 10;` shadows the first: it creates a
// new binding under the same name. From that statement onward, the
// name x refers to the new value, so the second println! prints 10.
//
// Contrast with lesson 006: writing `x = 10;` (no second `let`)
// instead would attempt to reassign the immutable `x` and fail with
// E0384. That diagnostic is captured in lesson 006's probe and is
// referenced by the lesson, not re-captured here.

fn main() {
    let x = 5;
    println!("first: x = {x}");
    let x = 10;
    println!("second: x = {x}");
}
