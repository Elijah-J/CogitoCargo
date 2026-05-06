fn main() {
    // These bindings hold integer literals.
    let apples = 5;
    let oranges = 7;

    // The failed version used 0 in the else branch and produced E0308:
    // `if` and `else` have incompatible types.
    let message = if apples > oranges {
        "More apples"
    } else {
        "Not more apples"
    };

    println!("{message}");
}
