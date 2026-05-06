fn main() {
    let word: &str = "café";

    // The for loop runs the body once for each char in the iterator.
    for c in word.chars() {
        println!("char: {c}");
    }

    // Execution continues after the loop finishes.
    println!("done");
}
