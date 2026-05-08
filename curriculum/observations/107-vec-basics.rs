// Working probe for lesson 107-vec-basics.
//
// Three pieces composed: vec![] macro construction (empty and prefilled),
// the .len() method on Vec, and indexing v[i] with a literal usize.
//
// Compile and run:
//   $ rustc 107-vec-basics.rs -o demo
//   $ ./demo
// Expected output (exit 0):
//   empty.len() = 0
//   three.len() = 3
//   three[0] = 10
//   three[2] = 30
fn main() {
    let empty: Vec<u64> = vec![];
    let three: Vec<u64> = vec![10, 20, 30];
    println!("empty.len() = {}", empty.len());
    println!("three.len() = {}", three.len());
    println!("three[0] = {}", three[0]);
    println!("three[2] = {}", three[2]);
}
