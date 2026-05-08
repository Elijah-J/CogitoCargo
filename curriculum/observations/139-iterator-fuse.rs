// Resumes after None: yields Some, None, Some, None, Some, None, ...
struct Stutter { n: u32 }

impl Iterator for Stutter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n % 2 == 0 {
            let v = self.n / 2;
            self.n += 1;
            Some(v)
        } else {
            self.n += 1;
            None
        }
    }
}

fn main() {
    // Without fuse: resumes after None.
    let mut a = Stutter { n: 0 };
    for _ in 0..6 {
        println!("{:?}", a.next());
    }

    println!("---");

    // With fuse: sticks at first None.
    let mut b = Stutter { n: 0 }.fuse();
    for _ in 0..6 {
        println!("{:?}", b.next());
    }
}
