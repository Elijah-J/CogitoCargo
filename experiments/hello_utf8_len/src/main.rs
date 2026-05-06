fn main() {
    let ascii_word: &str = "cafe";
    let utf8_word: &str = "café";

    println!("ASCII bytes: {}", ascii_word.len());
    println!("UTF-8 bytes: {}", utf8_word.len());
}
