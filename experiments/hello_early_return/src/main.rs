fn main() {
    let zero_description = describe_count(0);
    let three_description = describe_count(3);

    println!("Zero: {zero_description}");
    println!("Three: {three_description}");
}

fn describe_count(count: i32) -> &'static str {
    if count == 0 {
        return "none";
    }

    "some"
}
