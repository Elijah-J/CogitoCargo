fn main() {
    let numbers = [3, 4, 5];
    let mut total = 0;

    // The loop visits one array element at a time.
    for number in numbers {
        // The total binding keeps the running sum across loop passes.
        total = total + number;
        println!("running total: {total}");
    }

    // After the loop, total contains the sum of all array elements.
    println!("final total: {total}");
}
