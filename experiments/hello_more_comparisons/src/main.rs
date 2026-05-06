fn main() {
    // These bindings hold integer literals.
    let apples = 7;
    let oranges = 5;

    // These comparison expressions complete the first-pass comparison set.
    let fewer_apples = apples < oranges;
    let different_amount = apples != oranges;
    let at_least_as_many = apples >= oranges;
    let at_most_as_many = apples <= oranges;

    println!("Fewer apples: {fewer_apples}");
    println!("Different amount: {different_amount}");
    println!("At least as many: {at_least_as_many}");
    println!("At most as many: {at_most_as_many}");
}
