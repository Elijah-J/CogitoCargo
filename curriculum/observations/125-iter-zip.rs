fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    let w: Vec<u64> = vec![100, 200, 300];
    for pair in v.iter().zip(w.iter()) {
        println!("{} / {}", pair.0, pair.1);
    }
}
