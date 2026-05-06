fn main() {
    // These bindings hold integer literals.
    let apples = 5;
    let oranges = 7;

    // The first condition is false, so Rust checks the else if condition.
    if apples > oranges {
        println!("More apples");
    } else if apples < oranges {
        println!("Fewer apples");
    } else {
        println!("Same amount");
    }
}
