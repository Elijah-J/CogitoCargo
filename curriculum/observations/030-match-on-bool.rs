fn main() {
    let cond: bool = true;
    let result: i32 = match cond {
        true => 100,
        false => -100,
    };
    println!("result = {result}");
}
