fn main() {
    let word: &str = "café";
    let mut chars = word.chars();

    // Each call passes the next Option<char> from the iterator.
    describe(chars.next());
    describe(chars.next());
    describe(chars.next());
    describe(chars.next());
    describe(chars.next());
}

// This function uses match to extract the char from Some or handle None.
fn describe(item: Option<char>) {
    match item {
        Some(c) => println!("found: {c}"),
        None => println!("nothing left"),
    }
}
