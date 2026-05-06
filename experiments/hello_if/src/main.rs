fn main() {
    // These bindings hold integer literals.
    let apples = 7;
    let oranges = 5;

    // This condition is true, so the block runs.
    if apples > oranges {
        println!("More apples");
    }

    // This condition is false, so the block is skipped.
    if apples == oranges {
        println!("Same amount");
    }
}
