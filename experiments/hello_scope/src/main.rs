fn main() {
    // This binding lives in the main function's block.
    let name = "outer";
    println!("Before block: {name}");

    {
        // This binding lives only inside this inner block.
        let name = "inner";
        println!("Inside block: {name}");
    }

    // The inner binding is gone, so `name` means the outer binding again.
    println!("After block: {name}");
}
