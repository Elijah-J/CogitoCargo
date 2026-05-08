// Lesson 118 — Capstone: read rmp's `src/biguint/cmp.rs:4-10`
// (the PartialEq + Eq pair on BigUInt) end-to-end, using only the
// 117 installed concepts plus three named deferrals: `?Sized`, the
// `= Self` default-type-parameter, and the `Eq: PartialEq` supertrait.
//
// Self-contained probe. A small `BigUInt`-shaped struct mirrors
// rmp's slice closely enough that the two `impl` blocks below match
// rmp's `cmp.rs:4-10` token-for-token (modulo the field visibility,
// which is `pub(super)` in rmp's basic.rs and elided here because
// the probe has no module structure to ride).
//
// Compile and run with:
//   rustc 118-capstone-rmp-cmp-equality.rs -o probe
//   ./probe
//
// Expected: silent compile, exit 0, four lines on stdout —
//   a == b -> true
//   a == c -> false
//   a != b -> false
//   a != c -> true
//
// Walk: `a == b` desugars to
// `<BigUInt as PartialEq<BigUInt>>::eq(&a, &b)` (Reference
// `expressions/operator-expr.md:512-516`). The dispatched `eq` is
// the body on line 14 below: `self.limbs == other.limbs`. That
// inner `==` is `Vec<u64> == Vec<u64>`, lesson 117's mechanic, which
// returns `true` exactly when the two Vecs have the same length and
// the same element values pairwise. `!=` rides the default `ne` body
// in std's `PartialEq` declaration, which is `!(self == other)`.

struct BigUInt {
    limbs: Vec<u64>,
}

// Lesson 111 (impl Trait for Type) + lesson 114 (generic argument
// `<BigUInt>`) + lesson 113 (`other: &BigUInt` non-receiver reference
// parameter) + lesson 112 (extra parameter) + lesson 100 (`&self`)
// + lesson 012 (`bool`).
impl PartialEq<BigUInt> for BigUInt {
    fn eq(&self, other: &BigUInt) -> bool {
        // Lesson 095 (field access on `&self`) + lesson 117
        // (`==` on `Vec<u64>`).
        self.limbs == other.limbs
    }
}

// Lesson 116 (empty-impl-accepts-default). `Eq` has no required
// methods (its one implicit method is feature-gated and uses a
// default body); an empty `{}` body is a complete impl.
impl Eq for BigUInt {}

fn main() {
    let a = BigUInt { limbs: vec![42] };
    let b = BigUInt { limbs: vec![42] };
    let c = BigUInt { limbs: vec![7] };

    println!("a == b -> {}", a == b);
    println!("a == c -> {}", a == c);
    println!("a != b -> {}", a != b);
    println!("a != c -> {}", a != c);
}
