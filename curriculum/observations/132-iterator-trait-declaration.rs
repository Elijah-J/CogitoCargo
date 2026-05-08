// Probe 1 — minimal user-defined Iterator impl exercising the
// required surface only. Witnesses that filling in `type Item` and
// `fn next` is sufficient to satisfy the trait.
//
// $ rustc 132-iterator-trait-declaration.rs -o demo
// $ ./demo
// Some(0)
// Some(1)
// Some(2)
// None

struct Counter {
    value: u32,
    limit: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value < self.limit {
            let v = self.value;
            self.value += 1;
            Some(v)
        } else {
            None
        }
    }
}

fn main() {
    let mut c = Counter { value: 0, limit: 3 };
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());
}
