fn main() {
    let mut count: i32 = 0;
    for n in 0..5 {
        if n == 2 {
            continue;
        }
        count += 1;
    }
    println!("count = {count}");
}
