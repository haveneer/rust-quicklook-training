mod ffi;

use ffi::manual::Point;
use ffi::safe::{self, Counter};

fn main() {
    println!("== Case 1: struct by pointer (repr(C)) ==");
    let p = Point { x: 3.0, y: 4.0 };
    println!("norm({:?}) = {}", p, safe::norm(p));

    println!("\n== Case 2: pointer + length ==");
    let values = [1.0, 2.0, 3.0, 4.5];
    println!("sum({:?}) = {}", values, safe::sum(&values));

    println!("\n== Case 3: opaque handle + Drop (RAII) ==");
    {
        let mut counter = Counter::new(10);
        counter.inc();
        counter.inc();
        println!("counter after two inc() = {}", counter.get());
    } // Counter::drop() runs here: counter_free() is called automatically.
    println!("counter dropped, C-side memory freed automatically");

    println!("\n== Case 4 (bonus): C calling back into Rust ==");
    let mut squares = Vec::new();
    safe::for_each(&[1, 2, 3, 4, 5], |v| squares.push(v * v));
    println!("squares collected via C callback: {squares:?}");

    #[cfg(feature = "bindgen-compare")]
    run_bindgen_comparison();
}

#[cfg(feature = "bindgen-compare")]
fn run_bindgen_comparison() {
    println!("\n== Bonus: bindgen-generated bindings vs. hand-written ones ==");
    let manual_result = safe::norm(Point { x: 3.0, y: 4.0 });
    let generated_result = ffi::generated::compare_point_norm(3.0, 4.0);
    println!("manual::point_norm  = {manual_result}");
    println!("bindgen::point_norm = {generated_result}");
    assert_eq!(
        manual_result, generated_result,
        "manual and generated bindings disagree!"
    );
    println!("bindings agree - same C symbol, only the binding-generation method differs");
}
