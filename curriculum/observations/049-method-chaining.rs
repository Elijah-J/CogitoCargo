// EduRatchet-2 cycle 049 — method chaining.
// Working probe: the chain `String::new().is_empty()` produces the
// same value as the two-step form `let s = String::new(); s.is_empty();`.
// Both bind to `true`. The chain composes lesson 042's `String::new()`
// (no-receiver associated function returning a fresh empty `String`)
// with lesson 040's method-call grammar `receiver.method(args)` where
// the receiver is itself an expression — namely the call `String::new()`.

fn main() {
    let chained: bool = String::new().is_empty();
    let s: String = String::new();
    let stepped: bool = s.is_empty();
    println!("chained = {chained}, stepped = {stepped}");
}
