fn set_to_99(r: &mut i32) {
    *r = 99;
}

fn main() {
    let mut n: i32 = 1;
    set_to_99(&mut n);
    println!("n = {n}");
}
