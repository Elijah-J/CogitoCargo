fn main() {
    let apples = 3;
    let oranges = 4;
    let total = add(apples, oranges);

    println!("Total: {total}");
}

fn add(left: i32, right: i32) -> i32 {
    left + right
}
