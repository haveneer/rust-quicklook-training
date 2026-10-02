// cargo run --release --example perf_dhat
// Writes dhat-heap.json: open it in https://nnethercote.github.io/dh_view/dh_view.html
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc; // counts every allocation

fn naive(n: usize) -> Vec<Vec<f64>> {
    let mut rows = Vec::new();
    for i in 0..n {
        let mut row = Vec::new(); // grows step by step: several reallocations
        for j in 0..n {
            row.push((i * j) as f64);
        }
        rows.push(row);
    }
    rows
}

fn preallocated(n: usize) -> Vec<Vec<f64>> {
    let mut rows = Vec::with_capacity(n);
    for i in 0..n {
        rows.push((0..n).map(|j| (i * j) as f64).collect()); // exact size known
    }
    rows
}

fn main() {
    let version = std::env::args().nth(1).unwrap_or_default();
    let _profiler = dhat::Profiler::new_heap(); // prints a summary when dropped
    let m = if version == "pre" {
        preallocated(1000)
    } else {
        naive(1000)
    };
    std::hint::black_box(m);
}
