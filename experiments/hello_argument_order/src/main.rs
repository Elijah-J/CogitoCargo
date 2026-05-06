fn main() {
    let starting = 10;
    let removed = 3;

    let remaining = subtract(starting, removed);
    let reversed = subtract(removed, starting);

    println!("Remaining: {remaining}");
    println!("Reversed: {reversed}");
}

fn subtract(left: i32, right: i32) -> i32 {
    left - right
}
