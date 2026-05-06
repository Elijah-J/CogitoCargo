fn main() {
    let numbers = [3, 4, 5];

    // The array stores several integer values in one fixed sequence.
    for number in numbers {
        // The loop variable receives one array element on each pass.
        println!("number: {number}");
    }

    // Execution continues after every array element has been used.
    println!("done");
}
