use std::cmp::Ordering;

fn main() {
    let a: i32 = 3;
    let b: i32 = 5;
    match a.cmp(&b) {
        Ordering::Less => println!("a < b"),
        Ordering::Greater => println!("a > b"),
        Ordering::Equal => println!("a == b"),
    }
}
