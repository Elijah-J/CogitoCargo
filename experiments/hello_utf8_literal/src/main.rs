fn main() {
    // "cafe" uses only ASCII letters.
    let ascii_word: &str = "cafe";

    // "café" is not ASCII because of "é", but it is valid UTF-8.
    let utf8_word: &str = "café";

    println!("ASCII: {ascii_word}");
    println!("UTF-8: {utf8_word}");
}
