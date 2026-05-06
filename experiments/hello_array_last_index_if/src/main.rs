fn main() {
    let numbers: [i32; 0] = [];

    if numbers.len() > 0 {
        // This branch only runs when subtracting 1 from len will not underflow.
        let last_index: usize = numbers.len() - 1;
        println!("last index: {last_index}");
    } else {
        println!("empty array");
    }
}
