fn main() {
    let numbers = [3, 4, 5];
    let mut total = 0;

    // This loop uses the same accumulator shape as hello_array_sum.
    for number in numbers {
        // `+=` adds the current element into the mutable total binding.
        total += number;
        println!("running total: {total}");
    }

    println!("final total: {total}");
}
