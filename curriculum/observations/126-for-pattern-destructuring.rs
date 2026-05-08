fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    for (a, b) in v.iter().zip(w.iter()) {
        println!("{} / {}", a, b);
    }
}
