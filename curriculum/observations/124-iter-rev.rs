fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    for x in v.iter().rev() {
        println!("{}", x);
    }
}
