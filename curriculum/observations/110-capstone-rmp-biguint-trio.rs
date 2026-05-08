// Self-contained probe for lesson 110 — rmp BigUInt trio capstone.
//
// Replicates the rmp slice from `src/biguint/basic.rs`: the
// `LIMB_SIZE_BITS` constant, the `BigUInt` struct, and the three
// methods `zero`, `is_zero`, `num_bits`. The `pub(super)` field
// modifier on `limbs` is replaced with `pub` here because this probe
// has no parent module (the `pub(super)` reach is empty at the crate
// root). The cross-crate driver in the evidence appendix exercises
// `pub(super)` on the real rmp source.
//
// Compile and run:
//   $ rustc 110-capstone-rmp-biguint-trio.rs -o probe
//   $ ./probe
//
// Expected stdout (exit 0):
//   LIMB_SIZE_BITS = 64
//   zero.limbs.len() = 0
//   zero.is_zero() = true
//   zero.num_bits() = 0
//   one.is_zero() = false
//   one.num_bits() = 1
//   v256.num_bits() = 9
//   max1.num_bits() = 64
//   two_64.num_bits() = 65
//   max2.num_bits() = 128
//   bad.is_zero() = false  (length-only check)
//   bad.num_bits() = 0

pub const LIMB_SIZE_BITS: u64 = 8 * (std::mem::size_of::<u64>() as u64);

pub struct BigUInt {
    pub limbs: Vec<u64>,
}

impl BigUInt {
    pub fn zero() -> Self {
        BigUInt { limbs: vec![] }
    }

    pub fn is_zero(&self) -> bool {
        return self.limbs.len() == 0;
    }

    pub fn num_bits(&self) -> u64 {
        match self.limbs.len() {
            0 => 0,
            n => {
                (n as u64 - 1) * LIMB_SIZE_BITS
                    + (LIMB_SIZE_BITS - (self.limbs[n - 1].leading_zeros() as u64))
            }
        }
    }
}

fn main() {
    // 1. Witness LIMB_SIZE_BITS evaluates to 64 at compile time.
    println!("LIMB_SIZE_BITS = {}", LIMB_SIZE_BITS);

    // 2. Canonical zero: empty Vec.
    let z = BigUInt::zero();
    println!("zero.limbs.len() = {}", z.limbs.len());
    println!("zero.is_zero() = {}", z.is_zero());
    println!("zero.num_bits() = {}", z.num_bits());

    // 3. Hand-built one-limb values. The canonical-form invariant is
    //    "no trailing-zero limbs," so the most-significant limb of each
    //    well-formed example is non-zero.
    let one = BigUInt { limbs: vec![1] };           // value 1
    println!("one.is_zero() = {}", one.is_zero());
    println!("one.num_bits() = {}", one.num_bits());

    let v256 = BigUInt { limbs: vec![0x100] };      // value 256
    println!("v256.num_bits() = {}", v256.num_bits());

    let max1 = BigUInt { limbs: vec![u64::MAX] };   // value 2^64 - 1
    println!("max1.num_bits() = {}", max1.num_bits());

    // 4. Two-limb values. Limb order is little-endian: limbs[0] is the
    //    least-significant 64-bit limb, limbs[1] the most-significant.
    //    Value here is limbs[0] + limbs[1] * 2^64 = 0 + 1 * 2^64 = 2^64.
    let two_64 = BigUInt { limbs: vec![0, 1] };
    println!("two_64.num_bits() = {}", two_64.num_bits());

    // 2^128 - 1, which uses 128 bits.
    let max2 = BigUInt { limbs: vec![u64::MAX, u64::MAX] };
    println!("max2.num_bits() = {}", max2.num_bits());

    // 5. is_zero is a length-only check. A single-zero-limb BigUInt
    //    violates the canonical-form rule but the runtime check just
    //    looks at the Vec's length — operational witness that the
    //    invariant is constructor-side discipline, not a runtime check.
    //    num_bits, by contrast, returns 0 here because
    //    leading_zeros(0) == 64 makes the MSL term zero.
    let bad = BigUInt { limbs: vec![0] };
    println!("bad.is_zero() = {}  (length-only check)", bad.is_zero());
    println!("bad.num_bits() = {}", bad.num_bits());
}
