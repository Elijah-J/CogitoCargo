// Probe 1 (working): size_hint on a slice iter, before & after .next(),
// plus the empty case.
fn main() {
    let v: Vec<u64> = vec![10, 20, 30];

    // Fresh slice iter on a 3-element vec
    let iter = v.iter();
    println!("{:?}", iter.size_hint());

    // After one .next() advances the cursor
    let mut iter = v.iter();
    let _ = iter.next();
    println!("{:?}", iter.size_hint());

    // Empty source
    let empty: Vec<u64> = vec![];
    println!("{:?}", empty.iter().size_hint());
}
