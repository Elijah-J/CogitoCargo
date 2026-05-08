// Working probe for lesson 123: `v.iter()` on a Vec<T>, consumed by a
// for-loop. The for-loop walks the iterator front-to-back, binding x to
// each yielded reference; println!("{}", x) prints the underlying u64.
//
// Compile and run:
//   $ rustc 123-vec-iter.rs -o demo
//   $ ./demo
// Expected output (exit 0):
//   10
//   20
//   30
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];
    for x in v.iter() {
        println!("{}", x);
    }
}
