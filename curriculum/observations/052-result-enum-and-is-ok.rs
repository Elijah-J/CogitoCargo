fn parity(n: i32) -> Result<i32, i32> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(n)
    }
}

fn main() {
    let a = parity(4);
    let b = parity(7);
    println!("a is ok: {}", a.is_ok());
    println!("b is ok: {}", b.is_ok());
}
