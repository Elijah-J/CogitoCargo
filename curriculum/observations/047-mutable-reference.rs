fn main() {
    let mut n: i32 = 1;
    let r: &mut i32 = &mut n;
    *r = 99;
    println!("n = {n}");
}
