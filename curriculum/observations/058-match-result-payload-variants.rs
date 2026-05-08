fn main() {
    let good: i32 = match "42".parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    let bad: i32 = match "abc".parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    println!("good = {good}, bad = {bad}");
}
