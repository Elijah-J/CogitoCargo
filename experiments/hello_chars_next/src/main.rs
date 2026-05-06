fn main() {
    let word: &str = "café";

    let mut chars = word.chars();

    println!("first: {:?}", chars.next());
    println!("second: {:?}", chars.next());
    println!("third: {:?}", chars.next());
    println!("fourth: {:?}", chars.next());
    println!("done: {:?}", chars.next());
}
