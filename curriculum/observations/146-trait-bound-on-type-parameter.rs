// Lesson 146 working probe: a generic function may carry a *trait bound*
// on its type parameter. `<T: std::fmt::Display>` restricts the types
// that can be substituted for `T` to types that implement `Display`,
// and inside the body the `{}` placeholder works on `t: T` because of
// the bound.
//
// Compile silently; run prints two lines: `5` then `7`.
fn say<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}

fn main() {
    say(5_u32);
    say(7_i32);
}
