use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    let trimmed: &str = buf.trim();
    println!("buf has {} bytes; trimmed = [{trimmed}]", buf.len());
}
