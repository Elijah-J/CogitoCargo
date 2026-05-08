// Working probe for lesson 122: method delegation `self.method(args)` in
// the body of another method on the same impl. `self.doubled()` inside
// `quadrupled` is the centered new shape.

struct Counter { n: u32 }

impl Counter {
    fn doubled(&self) -> u32 {
        self.n * 2
    }
    fn quadrupled(&self) -> u32 {
        self.doubled() * 2
    }
}

fn main() {
    let c = Counter { n: 7 };
    println!("doubled    = {}", c.doubled());
    println!("quadrupled = {}", c.quadrupled());
}
