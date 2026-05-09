// Lesson 143 working probe — unannotated closure parameter, first call
// fixes the type. Build with `rustc 143-unannotated-closure-first-use.rs`
// and run `./143-unannotated-closure-first-use` (silent compile, two
// printed lines `5` and `10`).

fn main() {
    let id = |x| x;
    let a = id(5_u32);
    let b = id(10_u32);
    println!("{}", a);
    println!("{}", b);
}
