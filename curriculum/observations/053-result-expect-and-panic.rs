fn parity(n: i32) -> Result<i32, i32> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(n)
    }
}

fn main() {
    let v: i32 = parity(4).expect("expected even");
    println!("v = {v}");
}
