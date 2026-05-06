fn main() {
    let ascii_word: &str = "cafe";

    let method_call_length = ascii_word.len();
    let path_call_length = str::len(ascii_word);

    println!("method call: {method_call_length}");
    println!("path call: {path_call_length}");
}
