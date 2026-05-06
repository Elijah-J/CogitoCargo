fn main() {
    // These bindings hold integer literals.
    let apples = 5;
    let oranges = 7;

    // This if expression chooses one string literal value.
    let message = if apples > oranges {
        "More apples"
    } else {
        "Not more apples"
    };

    println!("{message}");
}
