// Lesson 037 working probe: the `%` (remainder) operator on integers.
//
// Build:
//   rustc 037-remainder-operator.rs -o demo
//   ./demo
// Expected output:
//   10 / 3 = 3
//   10 % 3 = 1
// Expected exit code: 0.
//
// Both `let` lines use lesson 019's `name: i32 = value;` annotation.
// `10 / 3` is integer division from lesson 009: truncates toward zero,
// so `3 * 3 = 9` is the largest multiple of `3` not exceeding `10` and
// the result is `3`. `10 % 3` is the new operator: the remainder of
// dividing `10` by `3`, i.e. `10 - 3*3 = 10 - 9 = 1`. The arithmetic
// identity `a == (a / b) * b + (a % b)` holds (when `b` is non-zero):
// `3 * 3 + 1 == 10`.
//
// Calibration: `%` is a *remainder* operator, not a *modulo* operator.
// For positive operands the two coincide. For negative dividends, the
// result has the sign of the dividend (e.g. `-7 % 3` is `-1`, not `2`).
// Reference (output/docs/rust/reference/expressions/operator-expr.md
// line 452): "Rust uses a remainder defined with truncating division.
// Given `remainder = dividend % divisor`, the remainder will have the
// same sign as the dividend." This probe uses positive operands so the
// distinction does not surface.
//
// No broken-contrast probe is committed for this lesson. The natural
// broken-contrast `10 % 0` is a runtime panic ("attempt to calculate
// the remainder with a divisor of zero"), not a compile-time error,
// and runtime panics are outside lesson 003's diagnostic-walking scope.

fn main() {
    let q: i32 = 10 / 3;
    let r: i32 = 10 % 3;
    println!("10 / 3 = {q}");
    println!("10 % 3 = {r}");
}
