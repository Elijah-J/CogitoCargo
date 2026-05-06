fn main() {
    // This integer is divisible by both 3 and 2.
    let number = 6;

    // Rust runs the first true branch and skips the rest.
    if number % 5 == 0 {
        println!("Divisible by 5");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else if number % 2 == 0 {
        println!("Divisible by 2");
    } else {
        println!("No small divisor");
    }
}
