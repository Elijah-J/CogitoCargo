use std::cmp::Ordering;

fn main() {
    let a: u64 = 100;
    let b: u64 = 200;
    match a.cmp(&b) {
        Ordering::Less => println!("u64: a < b"),
        Ordering::Greater => println!("u64: a > b"),
        Ordering::Equal => println!("u64: a == b"),
    }

    let c: usize = 5;
    let d: usize = 5;
    match c.cmp(&d) {
        Ordering::Less => println!("usize: c < d"),
        Ordering::Greater => println!("usize: c > d"),
        Ordering::Equal => println!("usize: c == d"),
    }
}
