fn main() {
    let n = "42";
    let n: i32 = n.parse().expect("not a number");
    let doubled: i32 = n * 2;
    println!("n = {n}, doubled = {doubled}");
}
