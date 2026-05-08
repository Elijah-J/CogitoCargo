fn main() {
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 5 {
            break counter * 10;
        }
    };
    println!("result = {result}");
}
