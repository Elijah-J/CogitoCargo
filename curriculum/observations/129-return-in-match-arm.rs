use std::cmp::Ordering;

fn first_nonzero(a: Ordering, b: Ordering) -> Ordering {
    match a {
        Ordering::Equal => {}
        ord => return ord,
    }
    b
}

fn main() {
    let x = first_nonzero(Ordering::Less, Ordering::Greater);
    let label = match x {
        Ordering::Less => "less",
        Ordering::Equal => "equal",
        Ordering::Greater => "greater",
    };
    println!("{}", label);

    let y = first_nonzero(Ordering::Equal, Ordering::Greater);
    let label_y = match y {
        Ordering::Less => "less",
        Ordering::Equal => "equal",
        Ordering::Greater => "greater",
    };
    println!("{}", label_y);
}
