fn main() {
    let numbers: [i32; 0] = [];

    // Empty arrays have length 0, so subtracting 1 from len underflows.
    let last_index: usize = numbers.len() - 1;

    println!("last index: {last_index}");
}
