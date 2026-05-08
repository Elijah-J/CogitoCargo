// Lesson 105 — `pub use Path::Item;` re-exports.
//
// Working probe: `pub use self::hidden::value;` re-exports
// `inner::hidden::value` under `inner::value`. Both call paths reach
// the same function and print 42.
//
// $ rustc 105-pub-use-re-export.rs
// $ ./105-pub-use-re-export
// via re-export: 42
// via original:  42
//
// Centered contrast: drop the `pub` and the bare `use self::hidden::value;`
// brings the name into `inner`'s local scope only. Calling `inner::value()`
// from outside fails with E0603 ("private function import"). See the
// evidence appendix for that transcript.

mod inner {
    pub mod hidden {
        pub fn value() -> u32 {
            42
        }
    }
    pub use self::hidden::value;
}

fn main() {
    println!("via re-export: {}", inner::value());
    println!("via original:  {}", inner::hidden::value());
}
