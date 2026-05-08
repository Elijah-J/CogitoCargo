// Self-contained mirror of rmp's src/biguint/cmp.rs:12-33.
// Mirrors the structure 1:1 so each token matches the rmp source.
// The field is plain `pub` here (no parent module to use pub(super)).

use std::cmp::{self, Ord, Ordering};

#[derive(Clone)]
pub struct BigUInt {
    pub limbs: Vec<u64>,
}

impl PartialEq<BigUInt> for BigUInt {
    fn eq(&self, other: &BigUInt) -> bool {
        self.limbs == other.limbs
    }
}

impl Eq for BigUInt {}

impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigUInt {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.limbs.len().cmp(&other.limbs.len());
        if ord == cmp::Ordering::Equal {
            for (left, right) in self.limbs.iter().rev().zip(other.limbs.iter().rev()) {
                match left.cmp(right) {
                    Ordering::Equal => {}
                    ord => return ord,
                }
            }
            return Ordering::Equal;
        } else {
            ord
        }
    }
}

fn label(o: Ordering) -> &'static str {
    match o {
        Ordering::Less => "Less",
        Ordering::Equal => "Equal",
        Ordering::Greater => "Greater",
    }
}

fn main() {
    let a = BigUInt { limbs: vec![100] };
    let b = BigUInt { limbs: vec![200] };
    let c = BigUInt { limbs: vec![100] };

    // PartialOrd: partial_cmp returns Option<Ordering>
    let pc = a.partial_cmp(&b);
    let _: Option<Ordering> = pc;
    println!("a.partial_cmp(&b) is Some(Less)? {}", pc == Some(Ordering::Less));

    // Ord: cmp returns Ordering directly
    let o = a.cmp(&b);
    println!("a.cmp(&b) = {}", label(o));

    let o2 = a.cmp(&c);
    println!("a.cmp(&c) = {}", label(o2));

    // Length first: shorter limbs is Less than longer limbs.
    let short = BigUInt { limbs: vec![] };       // canonical zero
    let long = BigUInt { limbs: vec![0] };       // non-canonical zero
    println!("zero.cmp(&from0) = {}", label(short.cmp(&long)));

    // Big-endian comparison: equal-length, MSL determines.
    let p = BigUInt { limbs: vec![0xFFFF, 0x1] }; // value = 1*2^64 + 0xFFFF
    let q = BigUInt { limbs: vec![0x0001, 0x2] }; // value = 2*2^64 + 0x0001
    println!("p.cmp(&q) (MSL 1 vs 2)  = {}", label(p.cmp(&q)));
}
