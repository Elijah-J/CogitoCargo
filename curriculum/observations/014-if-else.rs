// Probe for lesson 014-if-else.
//
// The committed source below is the working `n = 7` version, which prints
// `big` when compiled and run. The lesson and the Evidence section also
// describe a second run with `n = 4` that prints `small`; that variant is
// only kept inside the lesson's transcript, not as a separate `.rs` file.
//
// Probe transcript (reproduced verbatim in the lesson's Evidence section)
// was captured in a fresh temp directory created with `mktemp -d`, with
// rustc 1.95.0 on Darwin x86_64.

fn main() {
    let n = 7;
    if n > 5 {
        println!("big");
    } else {
        println!("small");
    }
}
