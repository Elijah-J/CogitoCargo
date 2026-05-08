fn main() {
    let mut total: i32 = 0;
    for n in 1..=5 {
        let v: i32 = match n % 2 {
            0 => n,
            _ => continue,
        };
        total += v;
    }
    println!("total = {total}");
}
