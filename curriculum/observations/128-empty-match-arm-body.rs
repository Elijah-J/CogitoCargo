use std::cmp::Ordering;

fn main() {
    let o: Ordering = Ordering::Equal;
    match o {
        Ordering::Equal => {}
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("greater"),
    }
    println!("done");
}
