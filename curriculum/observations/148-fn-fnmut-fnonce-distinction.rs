// Lesson 148 working probe: three call helpers, one for each trait in the
// Fn-family, and three closures matching one trait each.
//
// - `call_fn`     accepts `F: Fn(u32) -> u32`     (`&self` receiver on call)
// - `call_fnmut`  accepts `F: FnMut(u32) -> u32`  (`&mut self`; needs `mut f: F`)
// - `call_fnonce` accepts `F: FnOnce(u32) -> u32` (`self`-by-value)
//
// The closure auto-implements one or more of the three based on what the
// body does with captures: `pure` captures nothing → Fn; `tick` mutates
// captured `counter` → FnMut; `consume` moves captured `s` out → FnOnce.
//
// Compile silently; run prints three lines: `Fn:     6`, `FnMut:  5`,
// `FnOnce: 0`.
fn call_fn<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
    f(x)
}

fn call_fnmut<F: FnMut(u32) -> u32>(mut f: F, x: u32) -> u32 {
    f(x)
}

fn call_fnonce<F: FnOnce(u32) -> u32>(f: F, x: u32) -> u32 {
    f(x)
}

fn main() {
    // (1) Captures nothing — implements Fn (and so also FnMut and FnOnce).
    let pure = |n: u32| n + 1;
    println!("Fn:     {}", call_fn(pure, 5));

    // (2) Mutates a captured `let mut counter` — implements FnMut + FnOnce,
    // not Fn.
    let mut counter: u32 = 0;
    let tick = |n: u32| { counter += n; counter };
    println!("FnMut:  {}", call_fnmut(tick, 5));

    // (3) Moves a captured String out of the closure — implements FnOnce only.
    let s = String::from("hello");
    let consume = move |_: u32| { drop(s); 0_u32 };
    println!("FnOnce: {}", call_fnonce(consume, 0));
}
