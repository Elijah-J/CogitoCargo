fn main() {
    let count = 3;
    let next_count = add_one(count);

    println!("Next count: {next_count}");
}

fn add_one(number: i32) -> i32 {
    return number + 1;
}
