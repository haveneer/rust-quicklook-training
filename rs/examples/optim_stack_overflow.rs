// cargo run --example optim_stack_overflow            (debug: no tail call optimisation)
// cargo run --release --example optim_stack_overflow  (release: LLVM may turn it into a loop)
fn sum_rec(n: u64, acc: u64) -> u64 {
    if n == 0 {
        return acc;
    }
    sum_rec(n - 1, acc + n)
}

fn main() {
    let n = std::hint::black_box(10_000_000);
    println!("sum = {}", sum_rec(n, 0));
}
