fn main() {
    let word: &str = "café";

    let byte_count = word.len();
    let char_count = word.chars().count();

    println!("bytes: {byte_count}");
    println!("chars: {char_count}");
}
