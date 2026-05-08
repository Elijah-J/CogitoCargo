// Probe source for EduRatchet-2 lesson 004-statements-in-order.
//
// This file is the ORIGINAL ordering used by the lesson: three
// `println!` statements inside `fn main`, each terminated by `;`,
// labelled "first line", "second line", "third line" in source order.
//
// To reproduce the lesson's first transcript, copy this file into an
// empty directory and compile it:
//   rustc 004-statements-in-order.rs
// Then run the produced executable:
//   ./004-statements-in-order
// Expected output (one line per println, in source order):
//   first line
//   second line
//   third line
//
// The lesson's contrast probe is performed by editing this file
// in place to swap the second and third statements:
//   fn main() {
//       println!("first line");
//       println!("third line");
//       println!("second line");
//   }
// Recompiling and rerunning then prints:
//   first line
//   third line
//   second line
// i.e. the executable's output reorders to match the new source order.
fn main() {
    println!("first line");
    println!("second line");
    println!("third line");
}
