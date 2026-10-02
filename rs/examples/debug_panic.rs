// cargo run --example debug_panic
// RUST_BACKTRACE=1 cargo run --example debug_panic
fn read_value(values: &[f64], index: usize) -> f64 {
    values[index] // panics when index is out of bounds
}

fn average_of_window(values: &[f64], start: usize, len: usize) -> f64 {
    (start..start + len)
        .map(|i| read_value(values, i))
        .sum::<f64>()
        / len as f64
}

fn main() {
    let values = vec![1.0, 2.0, 3.0];
    println!("{}", average_of_window(&values, 1, 3)); // window overflows by one element
}
