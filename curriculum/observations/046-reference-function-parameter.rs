fn show(r: &i32) {
    println!("via reference param: {r}");
}

fn main() {
    let n: i32 = 42;
    show(&n);
    println!("n is still: {n}");
}
