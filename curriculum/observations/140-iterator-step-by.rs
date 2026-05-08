// Probe 1 (working): for-loop and chained .count() on a step_by(2) wrapper.
fn main() {
    let v: Vec<u64> = vec![10, 20, 30, 40, 50];

    // step_by(2) starts at index 0, then 2, then 4
    for x in v.iter().step_by(2) {
        println!("{}", x);
    }

    println!("---");

    // chain composition: step_by then count
    let n = v.iter().step_by(2).count();
    println!("{}", n);
}
