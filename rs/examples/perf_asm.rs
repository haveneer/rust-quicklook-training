//! cargo asm --example perf_asm perf_asm::sum_u32
//! cargo asm --example perf_asm perf_asm::sum_f64

/// Integer addition is associative: LLVM may reorder it and vectorize the loop
#[inline(never)]
pub fn sum_u32(xs: &[u32]) -> u32 {
    xs.iter().fold(0, |acc, &x| acc.wrapping_add(x))
}

/// Floating-point addition is not: the loop stays scalar, in source order
#[inline(never)]
pub fn sum_f64(xs: &[f64]) -> f64 {
    xs.iter().sum()
}

fn main() {
    let ints: Vec<u32> = (0..1000).collect();
    let floats: Vec<f64> = (0..1000).map(f64::from).collect();
    println!("{} {}", sum_u32(&ints), sum_f64(&floats));
}
