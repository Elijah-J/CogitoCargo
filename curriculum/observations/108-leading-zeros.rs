// Working probe for lesson 108 — `u64::leading_zeros()`.
//
// Build: `rustc 108-leading-zeros.rs -o demo`.
// Run:   `./demo`.
//
// Expected stdout:
//   0u64 leading zeros = 64
//   1u64 leading zeros = 63
//   0x100000000u64 leading zeros = 31
//   u64::MAX leading zeros = 0
//
// Each line invokes `n.leading_zeros()` on a different `u64` value
// and prints the returned `u32` count of leading zero bits in the
// 64-bit binary representation:
//   - `0u64`              -> all 64 bits are zero -> 64.
//   - `1u64`              -> low bit set, 63 zero bits above it -> 63.
//   - `0x100000000u64`    -> bit 32 set, 31 zero bits above it -> 31.
//   - `u64::MAX`          -> all 64 bits one, no leading zeros -> 0.
fn main() {
    println!("0u64 leading zeros = {}", 0u64.leading_zeros());
    println!("1u64 leading zeros = {}", 1u64.leading_zeros());
    println!("0x100000000u64 leading zeros = {}", 0x100000000u64.leading_zeros());
    println!("u64::MAX leading zeros = {}", u64::MAX.leading_zeros());
}
