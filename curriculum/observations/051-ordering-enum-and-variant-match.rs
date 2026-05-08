use std::cmp::Ordering;

fn main() {
    let direction: Ordering = Ordering::Less;
    let label = match direction {
        Ordering::Less => "less",
        Ordering::Greater => "greater",
        Ordering::Equal => "equal",
    };
    println!("direction = {label}");
}
