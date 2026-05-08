fn main() {
    let nums = [10, 20, 30, 40, 50];
    let bad_index_str = "10";
    let bad_index: usize = bad_index_str.parse().expect("not a number");
    let element = nums[bad_index];
    println!("element = {}", element);
}
