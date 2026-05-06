fn main() {
    // `mut` lets this binding be assigned a new value later.
    let mut name = "Eli";

    println!("Before: {name}");

    // This changes the value bound to `name`.
    name = "Rust";

    println!("After: {name}");
}
